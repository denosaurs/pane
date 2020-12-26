use deno_core::error::anyhow;
use deno_core::error::AnyError;

use pixels::Pixels;
use pixels::SurfaceTexture;

use winit::event_loop::EventLoop;

use winit::dpi::PhysicalPosition;
use winit::dpi::PhysicalSize;
use winit::dpi::Position;
use winit::dpi::Size;

use winit::window::CursorIcon;
use winit::window::Icon;

use crate::helpers::hash;

pub struct Window {
  window: winit::window::Window,
  pixels: Pixels<winit::window::Window>,
}

impl Window {
  pub fn new(
    event_loop: &EventLoop<()>,
    width: u32,
    height: u32,
  ) -> Result<Self, AnyError> {
    let window = winit::window::Window::new(event_loop)?;
    let pixels = {
      let window_size = window.inner_size();
      let surface_texture =
        SurfaceTexture::new(window_size.width, window_size.height, &window);
      Pixels::new(width, height, surface_texture)?
    };
    Ok(Self { window, pixels })
  }

  pub fn id(&self) -> u64 {
    hash(self.window.id())
  }

  pub fn scale_factor(&self) -> f64 {
    self.window.scale_factor()
  }

  pub fn request_redraw(&self) {
    self.window.request_redraw()
  }

  pub fn inner_position(&self) -> Result<PhysicalPosition<i32>, AnyError> {
    match self.window.inner_position() {
      Ok(position) => Ok(position),
      Err(err) => Err(anyhow!(err)),
    }
  }

  pub fn outer_position(&self) -> Result<PhysicalPosition<i32>, AnyError> {
    match self.window.outer_position() {
      Ok(position) => Ok(position),
      Err(err) => Err(anyhow!(err)),
    }
  }

  pub fn set_outer_position(&self, position: Position) {
    self.window.set_outer_position(position)
  }

  pub fn inner_size(&self) -> PhysicalSize<u32> {
    self.window.inner_size()
  }

  pub fn set_inner_size(&self, size: Size) {
    self.window.set_inner_size(size)
  }

  pub fn outer_size(&self) -> PhysicalSize<u32> {
    self.window.outer_size()
  }

  pub fn set_min_inner_size(&self, min_size: Option<Size>) {
    self.window.set_min_inner_size(min_size)
  }

  pub fn set_max_inner_size(&self, max_size: Option<Size>) {
    self.window.set_max_inner_size(max_size)
  }

  pub fn set_title(&self, title: &str) {
    self.window.set_title(title)
  }

  pub fn set_resizable(&self, resizable: bool) {
    self.window.set_resizable(resizable)
  }

  pub fn set_minimized(&self, minimized: bool) {
    self.window.set_minimized(minimized)
  }

  pub fn set_maximized(&self, maximized: bool) {
    self.window.set_maximized(maximized)
  }

  pub fn set_decorations(&self, decorations: bool) {
    self.window.set_decorations(decorations)
  }

  pub fn set_always_on_top(&self, always_on_top: bool) {
    self.window.set_always_on_top(always_on_top)
  }

  pub fn set_window_icon(
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

  pub fn set_ime_position(&self, position: Position) {
    self.window.set_ime_position(position)
  }

  pub fn set_cursor_icon(&self, cursor: CursorIcon) {
    self.window.set_cursor_icon(cursor)
  }

  pub fn set_cursor_position(
    &self,
    position: Position,
  ) -> Result<(), AnyError> {
    match self.window.set_cursor_position(position) {
      Ok(()) => Ok(()),
      Err(err) => Err(anyhow!(err)),
    }
  }

  pub fn set_cursor_grab(&self, grab: bool) -> Result<(), AnyError> {
    match self.window.set_cursor_grab(grab) {
      Ok(()) => Ok(()),
      Err(err) => Err(anyhow!(err)),
    }
  }

  pub fn set_cursor_visible(&self, visible: bool) {
    self.window.set_cursor_visible(visible)
  }

  pub fn render_frame(&mut self) -> Result<(), AnyError> {
    match self.pixels.render() {
      Ok(()) => Ok(()),
      Err(err) => Err(anyhow!(err)),
    }
  }

  pub fn draw_frame(&mut self, buf: &mut [u8]) {
    self.pixels.get_frame().copy_from_slice(buf);
  }

  pub fn resize_frame(&mut self, width: u32, height: u32) {
    self.pixels.resize(width, height)
  }

  pub fn view_frame(&mut self) -> &[u8] {
    self.pixels.get_frame()
  }
}
