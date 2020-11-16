use std::path::PathBuf;
use std::time::Instant;

use deno_core::serde::Serialize;

use winit::dpi::PhysicalPosition;
use winit::dpi::PhysicalSize;

use winit::event::AxisId;
use winit::event::ButtonId;
use winit::event::DeviceEvent;
use winit::event::ElementState;
use winit::event::Event;
use winit::event::KeyboardInput;
use winit::event::ModifiersState;
use winit::event::MouseButton;
use winit::event::MouseScrollDelta;
use winit::event::StartCause;
use winit::event::TouchPhase;
use winit::event::WindowEvent;

use crate::helpers::hash;

#[derive(Debug, Clone, Serialize)]
pub enum SurfaceEvent {
  NewEvents(SurfaceStartCause),
  WindowEvent {
    window_id: u64,
    event: SurfaceWindowEvent,
  },
  DeviceEvent {
    device_id: u64,
    event: SurfaceDeviceEvent,
  },
  UserEvent,
  Suspended,
  Resumed,
  MainEventsCleared,
  RedrawRequested,
  RedrawEventsCleared,
  LoopDestroyed,
}

impl From<Event<'_, ()>> for SurfaceEvent {
  fn from(event: Event<()>) -> Self {
    match event {
      Event::NewEvents(start_cause) => {
        SurfaceEvent::NewEvents(SurfaceStartCause::from(start_cause))
      }
      Event::WindowEvent { window_id, event } => SurfaceEvent::WindowEvent {
        window_id: hash(window_id),
        event: SurfaceWindowEvent::from(event),
      },
      Event::DeviceEvent { device_id, event } => SurfaceEvent::DeviceEvent {
        device_id: hash(device_id),
        event: SurfaceDeviceEvent::from(event),
      },
      Event::UserEvent(_) => SurfaceEvent::UserEvent,
      Event::Suspended => SurfaceEvent::Suspended,
      Event::Resumed => SurfaceEvent::Resumed,
      Event::MainEventsCleared => SurfaceEvent::MainEventsCleared,
      Event::RedrawRequested(_) => SurfaceEvent::RedrawRequested,
      Event::RedrawEventsCleared => SurfaceEvent::RedrawEventsCleared,
      Event::LoopDestroyed => SurfaceEvent::LoopDestroyed,
    }
  }
}

#[derive(Debug, Clone, Serialize)]
pub enum SurfaceStartCause {
  ResumeTimeReached {
    #[serde(with = "serde_millis")]
    start: Instant,
    #[serde(with = "serde_millis")]
    requested_resume: Instant,
  },
  WaitCancelled {
    #[serde(with = "serde_millis")]
    start: Instant,
    #[serde(with = "serde_millis")]
    requested_resume: Option<Instant>,
  },
  Poll,
  Init,
}

impl From<StartCause> for SurfaceStartCause {
  fn from(start_cause: StartCause) -> Self {
    match start_cause {
      StartCause::ResumeTimeReached {
        start,
        requested_resume,
      } => SurfaceStartCause::ResumeTimeReached {
        start: start,
        requested_resume,
      },
      StartCause::WaitCancelled {
        start,
        requested_resume,
      } => SurfaceStartCause::WaitCancelled {
        start: start,
        requested_resume,
      },
      StartCause::Poll => SurfaceStartCause::Poll,
      StartCause::Init => SurfaceStartCause::Init,
    }
  }
}

#[derive(Debug, Clone, Serialize)]
pub enum SurfaceWindowEvent {
  Resized(PhysicalSize<u32>),
  Moved(PhysicalPosition<i32>),
  CloseRequested,
  Destroyed,
  DroppedFile(PathBuf),
  HoveredFile(PathBuf),
  HoveredFileCancelled,
  ReceivedCharacter(char),
  Focused(bool),
  KeyboardInput {
    device_id: u64,
    input: KeyboardInput,
    is_synthetic: bool,
  },
  ModifiersChanged(ModifiersState),
  CursorMoved {
    device_id: u64,
    position: PhysicalPosition<f64>,
  },
  CursorEntered {
    device_id: u64,
  },
  CursorLeft {
    device_id: u64,
  },
  MouseWheel {
    device_id: u64,
    delta: MouseScrollDelta,
    phase: TouchPhase,
  },
  MouseInput {
    device_id: u64,
    state: ElementState,
    button: MouseButton,
  },
  TouchpadPressure {
    device_id: u64,
    pressure: f32,
    stage: i64,
  },
  AxisMotion {
    device_id: u64,
    axis: AxisId,
    value: f64,
  },
  Touch(Touch),
  ScaleFactorChanged {
    scale_factor: f64,
    new_inner_size: PhysicalSize<u32>,
  },
  ThemeChanged(Theme),
}

#[derive(Debug, Clone, Serialize)]
pub struct Touch {
  device_id: u64,
  phase: TouchPhase,
  location: PhysicalPosition<f64>,
  force: Option<Force>,
  id: u64,
}

impl From<winit::event::Touch> for Touch {
  fn from(touch: winit::event::Touch) -> Self {
    let force = if let Some(force) = touch.force {
      Some(Force::from(force))
    } else {
      None
    };

    Touch {
      device_id: hash(touch.device_id),
      phase: touch.phase,
      location: touch.location,
      force,
      id: touch.id,
    }
  }
}

#[derive(Debug, Clone, Serialize)]
pub enum Force {
  Calibrated {
    force: f64,
    max_possible_force: f64,
    altitude_angle: Option<f64>,
  },
  Normalized(f64),
}

impl From<winit::event::Force> for Force {
  fn from(force: winit::event::Force) -> Self {
    match force {
      winit::event::Force::Calibrated {
        force,
        max_possible_force,
        altitude_angle,
      } => Force::Calibrated {
        force,
        max_possible_force,
        altitude_angle,
      },
      winit::event::Force::Normalized(force) => Force::Normalized(force),
    }
  }
}

#[derive(Debug, Clone, Serialize)]
pub enum Theme {
  Dark,
  Light,
}

impl From<winit::window::Theme> for Theme {
  fn from(theme: winit::window::Theme) -> Self {
    match theme {
      winit::window::Theme::Dark => Theme::Dark,
      winit::window::Theme::Light => Theme::Light,
    }
  }
}

impl From<WindowEvent<'_>> for SurfaceWindowEvent {
  fn from(window_event: WindowEvent) -> Self {
    match window_event {
      WindowEvent::Resized(size) => SurfaceWindowEvent::Resized(size),
      WindowEvent::Moved(pos) => SurfaceWindowEvent::Moved(pos),
      WindowEvent::CloseRequested => SurfaceWindowEvent::CloseRequested,
      WindowEvent::Destroyed => SurfaceWindowEvent::Destroyed,
      WindowEvent::DroppedFile(file) => SurfaceWindowEvent::DroppedFile(file),
      WindowEvent::HoveredFile(file) => SurfaceWindowEvent::HoveredFile(file),
      WindowEvent::HoveredFileCancelled => {
        SurfaceWindowEvent::HoveredFileCancelled
      }
      WindowEvent::ReceivedCharacter(ch) => {
        SurfaceWindowEvent::ReceivedCharacter(ch)
      }
      WindowEvent::Focused(foc) => SurfaceWindowEvent::Focused(foc),
      WindowEvent::KeyboardInput {
        device_id,
        input,
        is_synthetic,
      } => SurfaceWindowEvent::KeyboardInput {
        device_id: hash(device_id),
        input,
        is_synthetic,
      },
      WindowEvent::ModifiersChanged(modifiers) => {
        SurfaceWindowEvent::ModifiersChanged(modifiers)
      }
      WindowEvent::CursorMoved {
        device_id,
        position,
        modifiers: _,
      } => SurfaceWindowEvent::CursorMoved {
        device_id: hash(device_id),
        position,
      },
      WindowEvent::CursorEntered { device_id } => {
        SurfaceWindowEvent::CursorEntered {
          device_id: hash(device_id),
        }
      }
      WindowEvent::CursorLeft { device_id } => SurfaceWindowEvent::CursorLeft {
        device_id: hash(device_id),
      },
      WindowEvent::MouseWheel {
        device_id,
        delta,
        phase,
        modifiers: _,
      } => SurfaceWindowEvent::MouseWheel {
        device_id: hash(device_id),
        delta,
        phase,
      },
      WindowEvent::MouseInput {
        device_id,
        state,
        button,
        modifiers: _,
      } => SurfaceWindowEvent::MouseInput {
        device_id: hash(device_id),
        state,
        button,
      },
      WindowEvent::TouchpadPressure {
        device_id,
        pressure,
        stage,
      } => SurfaceWindowEvent::TouchpadPressure {
        device_id: hash(device_id),
        pressure,
        stage,
      },
      WindowEvent::AxisMotion {
        device_id,
        axis,
        value,
      } => SurfaceWindowEvent::AxisMotion {
        device_id: hash(device_id),
        axis,
        value,
      },
      WindowEvent::Touch(touch) => {
        SurfaceWindowEvent::Touch(Touch::from(touch))
      }
      WindowEvent::ScaleFactorChanged {
        scale_factor,
        new_inner_size,
      } => SurfaceWindowEvent::ScaleFactorChanged {
        scale_factor,
        new_inner_size: *new_inner_size,
      },
      WindowEvent::ThemeChanged(theme) => {
        SurfaceWindowEvent::ThemeChanged(Theme::from(theme))
      }
    }
  }
}

#[derive(Debug, Clone, Serialize)]
pub enum SurfaceDeviceEvent {
  Added,
  Removed,
  MouseMotion {
    delta: (f64, f64),
  },
  MouseWheel {
    delta: MouseScrollDelta,
  },
  Motion {
    axis: AxisId,
    value: f64,
  },
  Button {
    button: ButtonId,
    state: ElementState,
  },
  Key(KeyboardInput),
  Text {
    codepoint: char,
  },
}

impl From<DeviceEvent> for SurfaceDeviceEvent {
  fn from(device_event: DeviceEvent) -> Self {
    match device_event {
      DeviceEvent::Added => SurfaceDeviceEvent::Added,
      DeviceEvent::Removed => SurfaceDeviceEvent::Removed,
      DeviceEvent::MouseMotion { delta } => {
        SurfaceDeviceEvent::MouseMotion { delta }
      }
      DeviceEvent::MouseWheel { delta } => {
        SurfaceDeviceEvent::MouseWheel { delta }
      }
      DeviceEvent::Motion { axis, value } => {
        SurfaceDeviceEvent::Motion { axis, value }
      }
      DeviceEvent::Button { button, state } => {
        SurfaceDeviceEvent::Button { button, state }
      }
      DeviceEvent::Key(keyboard_input) => {
        SurfaceDeviceEvent::Key(keyboard_input)
      }
      DeviceEvent::Text { codepoint } => SurfaceDeviceEvent::Text { codepoint },
    }
  }
}
