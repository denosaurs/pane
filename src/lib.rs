use std::cell::RefCell;
use std::collections::HashMap;

use deno_core::error::anyhow;
use deno_core::error::AnyError;

use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::ZeroCopyBuf;

use deno_core::serde_json::json;
use deno_core::serde_json::Value;

use deno_json_op::json_op;

use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::platform::desktop::EventLoopExtDesktop;

mod event;
mod helpers;
mod window;

use event::Event;
use window::Window;

thread_local! {
  static EVENT_LOOP: RefCell<EventLoop<()>> = RefCell::new(EventLoop::new());
  static WINDOW_MAP: RefCell<HashMap<u64, Window>> = RefCell::new(HashMap::new());
}

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
  interface.register_op("window_new", window_new);
  interface.register_op("event_loop_step", event_loop_step);
}

#[json_op]
fn window_new(
  _json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  WINDOW_MAP.with(|cell| {
    let mut window_map = cell.borrow_mut();
    EVENT_LOOP.with(|cell| {
      let event_loop = cell.borrow();
      let window = Window::new(&event_loop)?;
      let id = window.id();
      window_map.insert(id, window);
      Ok(json!(id))
    })
  })
}

#[json_op]
fn event_loop_step(
  _json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let mut events = Vec::new();

  EVENT_LOOP.with(|cell| {
    let event_loop = &mut *cell.borrow_mut();
    event_loop.run_return(|event, _, control_flow| {
      *control_flow = ControlFlow::Exit;

      events.push(Event::from(event));
    });
  });

  Ok(json!(events))
}
