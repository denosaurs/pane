use deno_core::error::anyhow;
use deno_core::error::AnyError;

use winit::event_loop::EventLoop;

use winit::window::Icon;

use crate::helpers::hash;

pub struct Window {
  pub window: winit::window::Window,
}

impl Window {
  pub fn new(event_loop: &EventLoop<()>) -> Result<Window, AnyError> {
    match winit::window::Window::new(event_loop) {
      Ok(window) => Ok(Window { window }),
      Err(err) => Err(anyhow!(err)),
    }
  }

  pub fn id(&self) -> u64 {
    hash(self.window.id())
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
