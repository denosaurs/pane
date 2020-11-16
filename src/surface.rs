use deno_core::error::anyhow;
use deno_core::error::AnyError;

use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::platform::desktop::EventLoopExtDesktop;

use winit::window::Icon;
use winit::window::Window;

use crate::event::SurfaceEvent;
use crate::helpers::hash;

pub struct Surface {
  pub event_loop: EventLoop<()>,
  pub window: Window,
}

impl Surface {
  pub fn new() -> Result<Surface, AnyError> {
    let event_loop = EventLoop::new();

    match Window::new(&event_loop) {
      Ok(window) => Ok(Surface { event_loop, window }),
      Err(err) => Err(anyhow!(err)),
    }
  }

  pub fn id(&self) -> u64 {
    hash(self.window.id())
  }

  pub fn run(&mut self) -> Vec<SurfaceEvent> {
    let mut events = Vec::new();

    self.event_loop.run_return(|event, _, control_flow| {
      *control_flow = ControlFlow::Exit;

      events.push(SurfaceEvent::from(event));
    });

    events
  }

  pub fn set_title(&self, title: &str) {
    self.window.set_title(title);
  }

  pub fn set_icon(
    &self,
    rgba: Vec<u8>,
    width: u64,
    height: u64,
  ) -> Result<(), AnyError> {
    match Icon::from_rgba(rgba, width as u32, height as u32) {
      Ok(icon) => {
        self.window.set_window_icon(Some(icon));
        Ok(())
      }
      Err(err) => Err(anyhow!(err)),
    }
  }
}
