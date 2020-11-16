use std::path::PathBuf;
use std::time::Instant;

use deno_core::serde::Serialize;

use winit::dpi::PhysicalPosition;
use winit::dpi::PhysicalSize;

use winit::event::AxisId;
use winit::event::ButtonId;
use winit::event::ElementState;
use winit::event::KeyboardInput;
use winit::event::ModifiersState;
use winit::event::MouseButton;
use winit::event::MouseScrollDelta;
use winit::event::TouchPhase;

use crate::helpers::hash;

#[derive(Debug, Clone, Serialize)]
pub enum Event {
  NewEvents(StartCause),
  WindowEvent { window_id: u64, event: WindowEvent },
  DeviceEvent { device_id: u64, event: DeviceEvent },
  UserEvent,
  Suspended,
  Resumed,
  MainEventsCleared,
  RedrawRequested,
  RedrawEventsCleared,
  LoopDestroyed,
}

impl From<winit::event::Event<'_, ()>> for Event {
  fn from(event: winit::event::Event<()>) -> Self {
    match event {
      winit::event::Event::NewEvents(start_cause) => {
        Event::NewEvents(StartCause::from(start_cause))
      }
      winit::event::Event::WindowEvent { window_id, event } => {
        Event::WindowEvent {
          window_id: hash(window_id),
          event: WindowEvent::from(event),
        }
      }
      winit::event::Event::DeviceEvent { device_id, event } => {
        Event::DeviceEvent {
          device_id: hash(device_id),
          event: DeviceEvent::from(event),
        }
      }
      winit::event::Event::UserEvent(_) => Event::UserEvent,
      winit::event::Event::Suspended => Event::Suspended,
      winit::event::Event::Resumed => Event::Resumed,
      winit::event::Event::MainEventsCleared => Event::MainEventsCleared,
      winit::event::Event::RedrawRequested(_) => Event::RedrawRequested,
      winit::event::Event::RedrawEventsCleared => Event::RedrawEventsCleared,
      winit::event::Event::LoopDestroyed => Event::LoopDestroyed,
    }
  }
}

#[derive(Debug, Clone, Serialize)]
pub enum StartCause {
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

impl From<winit::event::StartCause> for StartCause {
  fn from(start_cause: winit::event::StartCause) -> Self {
    match start_cause {
      winit::event::StartCause::ResumeTimeReached {
        start,
        requested_resume,
      } => StartCause::ResumeTimeReached {
        start: start,
        requested_resume,
      },
      winit::event::StartCause::WaitCancelled {
        start,
        requested_resume,
      } => StartCause::WaitCancelled {
        start: start,
        requested_resume,
      },
      winit::event::StartCause::Poll => StartCause::Poll,
      winit::event::StartCause::Init => StartCause::Init,
    }
  }
}

#[derive(Debug, Clone, Serialize)]
pub enum WindowEvent {
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

impl From<winit::event::WindowEvent<'_>> for WindowEvent {
  fn from(window_event: winit::event::WindowEvent) -> Self {
    match window_event {
      winit::event::WindowEvent::Resized(size) => WindowEvent::Resized(size),
      winit::event::WindowEvent::Moved(pos) => WindowEvent::Moved(pos),
      winit::event::WindowEvent::CloseRequested => WindowEvent::CloseRequested,
      winit::event::WindowEvent::Destroyed => WindowEvent::Destroyed,
      winit::event::WindowEvent::DroppedFile(file) => {
        WindowEvent::DroppedFile(file)
      }
      winit::event::WindowEvent::HoveredFile(file) => {
        WindowEvent::HoveredFile(file)
      }
      winit::event::WindowEvent::HoveredFileCancelled => {
        WindowEvent::HoveredFileCancelled
      }
      winit::event::WindowEvent::ReceivedCharacter(ch) => {
        WindowEvent::ReceivedCharacter(ch)
      }
      winit::event::WindowEvent::Focused(foc) => WindowEvent::Focused(foc),
      winit::event::WindowEvent::KeyboardInput {
        device_id,
        input,
        is_synthetic,
      } => WindowEvent::KeyboardInput {
        device_id: hash(device_id),
        input,
        is_synthetic,
      },
      winit::event::WindowEvent::ModifiersChanged(modifiers) => {
        WindowEvent::ModifiersChanged(modifiers)
      }
      winit::event::WindowEvent::CursorMoved {
        device_id,
        position,
        modifiers: _,
      } => WindowEvent::CursorMoved {
        device_id: hash(device_id),
        position,
      },
      winit::event::WindowEvent::CursorEntered { device_id } => {
        WindowEvent::CursorEntered {
          device_id: hash(device_id),
        }
      }
      winit::event::WindowEvent::CursorLeft { device_id } => {
        WindowEvent::CursorLeft {
          device_id: hash(device_id),
        }
      }
      winit::event::WindowEvent::MouseWheel {
        device_id,
        delta,
        phase,
        modifiers: _,
      } => WindowEvent::MouseWheel {
        device_id: hash(device_id),
        delta,
        phase,
      },
      winit::event::WindowEvent::MouseInput {
        device_id,
        state,
        button,
        modifiers: _,
      } => WindowEvent::MouseInput {
        device_id: hash(device_id),
        state,
        button,
      },
      winit::event::WindowEvent::TouchpadPressure {
        device_id,
        pressure,
        stage,
      } => WindowEvent::TouchpadPressure {
        device_id: hash(device_id),
        pressure,
        stage,
      },
      winit::event::WindowEvent::AxisMotion {
        device_id,
        axis,
        value,
      } => WindowEvent::AxisMotion {
        device_id: hash(device_id),
        axis,
        value,
      },
      winit::event::WindowEvent::Touch(touch) => {
        WindowEvent::Touch(Touch::from(touch))
      }
      winit::event::WindowEvent::ScaleFactorChanged {
        scale_factor,
        new_inner_size,
      } => WindowEvent::ScaleFactorChanged {
        scale_factor,
        new_inner_size: *new_inner_size,
      },
      winit::event::WindowEvent::ThemeChanged(theme) => {
        WindowEvent::ThemeChanged(Theme::from(theme))
      }
    }
  }
}

#[derive(Debug, Clone, Serialize)]
pub enum DeviceEvent {
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

impl From<winit::event::DeviceEvent> for DeviceEvent {
  fn from(device_event: winit::event::DeviceEvent) -> Self {
    match device_event {
      winit::event::DeviceEvent::Added => DeviceEvent::Added,
      winit::event::DeviceEvent::Removed => DeviceEvent::Removed,
      winit::event::DeviceEvent::MouseMotion { delta } => {
        DeviceEvent::MouseMotion { delta }
      }
      winit::event::DeviceEvent::MouseWheel { delta } => {
        DeviceEvent::MouseWheel { delta }
      }
      winit::event::DeviceEvent::Motion { axis, value } => {
        DeviceEvent::Motion { axis, value }
      }
      winit::event::DeviceEvent::Button { button, state } => {
        DeviceEvent::Button { button, state }
      }
      winit::event::DeviceEvent::Key(keyboard_input) => {
        DeviceEvent::Key(keyboard_input)
      }
      winit::event::DeviceEvent::Text { codepoint } => {
        DeviceEvent::Text { codepoint }
      }
    }
  }
}
