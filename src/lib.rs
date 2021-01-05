use std::cell::RefCell;
use std::collections::HashMap;

use winit::dpi::LogicalSize;
use winit::dpi::PhysicalSize;

use deno_core::error::anyhow;
use deno_core::error::AnyError;

use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::ZeroCopyBuf;

use deno_core::serde::Deserialize;

use deno_core::serde_json;
use deno_core::serde_json::json;
use deno_core::serde_json::Value;

use deno_json_op::json_op;

use winit::dpi::LogicalPosition;
use winit::dpi::PhysicalPosition;
use winit::dpi::Position;
use winit::dpi::Size;

use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::CursorIcon;
use winit::window::UserAttentionType;

mod event;
mod helpers;
mod window;

use event::Event;
use window::Window;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", remote = "UserAttentionType")]
pub enum UserAttentionTypeDef {
  Critical,
  Informational,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", remote = "Position")]
pub enum PositionDef {
  Physical(PhysicalPosition<i32>),
  Logical(LogicalPosition<f64>),
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", remote = "Size")]
pub enum SizeDef {
  Physical(PhysicalSize<u32>),
  Logical(LogicalSize<f64>),
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", remote = "CursorIcon")]
pub enum CursorIconDef {
  Default,
  Crosshair,
  Hand,
  Arrow,
  Move,
  Text,
  Wait,
  Help,
  Progress,
  NotAllowed,
  ContextMenu,
  Cell,
  VerticalText,
  Alias,
  Copy,
  NoDrop,
  Grab,
  Grabbing,
  AllScroll,
  ZoomIn,
  ZoomOut,
  EResize,
  NResize,
  NeResize,
  NwResize,
  SResize,
  SeResize,
  SwResize,
  WResize,
  EwResize,
  NsResize,
  NeswResize,
  NwseResize,
  ColResize,
  RowResize,
}

thread_local! {
  static EVENT_LOOP: RefCell<EventLoop<()>> = RefCell::new(EventLoop::new());
  static WINDOW_MAP: RefCell<HashMap<u64, Window>> = RefCell::new(HashMap::new());
}

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
  interface.register_op("window_new", window_new);
  interface.register_op("window_scale_factor", window_scale_factor);
  interface.register_op("window_request_redraw", window_request_redraw);
  interface.register_op("window_inner_position", window_inner_position);
  interface.register_op("window_outer_position", window_outer_position);
  interface.register_op("window_set_outer_position", window_set_outer_position);
  interface.register_op("window_inner_size", window_inner_size);
  interface.register_op("window_set_inner_size", window_set_inner_size);
  interface.register_op("window_outer_size", window_outer_size);
  interface.register_op("window_set_min_inner_size", window_set_min_inner_size);
  interface.register_op("window_set_max_inner_size", window_set_max_inner_size);
  interface.register_op("window_set_title", window_set_title);
  interface.register_op("window_set_visible", window_set_visible);
  interface.register_op("window_set_resizable", window_set_resizable);
  interface.register_op("window_set_minimized", window_set_minimized);
  interface.register_op("window_set_maximized", window_set_maximized);
  interface.register_op("window_set_decorations", window_set_decorations);
  interface.register_op("window_set_always_on_top", window_set_always_on_top);
  interface.register_op("window_set_window_icon", window_set_window_icon);
  interface.register_op("window_set_ime_position", window_set_ime_position);
  interface.register_op("window_request_user_attention", window_request_user_attention);
  interface.register_op("window_set_cursor_icon", window_set_cursor_icon);
  interface
    .register_op("window_set_cursor_position", window_set_cursor_position);
  interface.register_op("window_set_cursor_grab", window_set_cursor_grab);
  interface.register_op("window_set_cursor_visible", window_set_cursor_visible);
  interface.register_op("window_render_frame", window_render_frame);
  interface.register_op("window_draw_frame", window_draw_frame);
  interface.register_op("window_resize_frame", window_resize_frame);
  interface.register_op("window_view_frame", window_view_frame);

  interface.register_op("event_loop_step", event_loop_step);
}

#[json_op]
fn window_new(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let width = json["width"].as_u64().unwrap() as u32;
  let height = json["height"].as_u64().unwrap() as u32;

  WINDOW_MAP.with(|cell| {
    let mut window_map = cell.borrow_mut();
    EVENT_LOOP.with(|cell| {
      let event_loop = cell.borrow();
      let window = Window::new(&event_loop, width, height)?;
      let id = window.id();
      window_map.insert(id, window);
      Ok(json!(id))
    })
  })
}

#[json_op]
fn window_scale_factor(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let id = json["id"].as_u64().unwrap();

  WINDOW_MAP.with(|cell| {
    let window_map = cell.borrow();

    if let Some(window) = window_map.get(&id) {
      Ok(json!(window.scale_factor()))
    } else {
      Err(anyhow!("Could not find window with id: {}", id))
    }
  })
}

#[json_op]
fn window_request_redraw(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let id = json["id"].as_u64().unwrap();

  WINDOW_MAP.with(|cell| {
    let window_map = cell.borrow();

    if let Some(window) = window_map.get(&id) {
      Ok(json!(window.request_redraw()))
    } else {
      Err(anyhow!("Could not find window with id: {}", id))
    }
  })
}

#[json_op]
fn window_inner_position(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let id = json["id"].as_u64().unwrap();

  WINDOW_MAP.with(|cell| {
    let window_map = cell.borrow();

    if let Some(window) = window_map.get(&id) {
      let position = window.inner_position()?;
      Ok(json!(position))
    } else {
      Err(anyhow!("Could not find window with id: {}", id))
    }
  })
}

#[json_op]
fn window_outer_position(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let id = json["id"].as_u64().unwrap();

  WINDOW_MAP.with(|cell| {
    let window_map = cell.borrow();

    if let Some(window) = window_map.get(&id) {
      let position = window.outer_position()?;
      Ok(json!(position))
    } else {
      Err(anyhow!("Could not find window with id: {}", id))
    }
  })
}

#[json_op]
fn window_set_outer_position(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let id = json["id"].as_u64().unwrap();
  let position: Position =
    PositionDef::deserialize(json["position"].to_owned()).unwrap();

  WINDOW_MAP.with(|cell| {
    let window_map = cell.borrow();

    if let Some(window) = window_map.get(&id) {
      window.set_outer_position(position);
      Ok(json!(()))
    } else {
      Err(anyhow!("Could not find window with id: {}", id))
    }
  })
}

#[json_op]
fn window_inner_size(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let id = json["id"].as_u64().unwrap();

  WINDOW_MAP.with(|cell| {
    let window_map = cell.borrow();

    if let Some(window) = window_map.get(&id) {
      Ok(json!(window.inner_size()))
    } else {
      Err(anyhow!("Could not find window with id: {}", id))
    }
  })
}

#[json_op]
fn window_set_inner_size(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let id = json["id"].as_u64().unwrap();
  let size: Size = SizeDef::deserialize(json["size"].to_owned()).unwrap();

  WINDOW_MAP.with(|cell| {
    let window_map = cell.borrow();

    if let Some(window) = window_map.get(&id) {
      window.set_inner_size(size);
      Ok(json!(()))
    } else {
      Err(anyhow!("Could not find window with id: {}", id))
    }
  })
}

#[json_op]
fn window_outer_size(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let id = json["id"].as_u64().unwrap();

  WINDOW_MAP.with(|cell| {
    let window_map = cell.borrow();

    if let Some(window) = window_map.get(&id) {
      Ok(json!(window.outer_size()))
    } else {
      Err(anyhow!("Could not find window with id: {}", id))
    }
  })
}

#[json_op]
fn window_set_min_inner_size(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let id = json["id"].as_u64().unwrap();
  let min_size: Option<Size> =
    if let Ok(size) = SizeDef::deserialize(json["minSize"].to_owned()) {
      Some(size)
    } else {
      None
    };

  WINDOW_MAP.with(|cell| {
    let window_map = cell.borrow();

    if let Some(window) = window_map.get(&id) {
      window.set_min_inner_size(min_size);
      Ok(json!(()))
    } else {
      Err(anyhow!("Could not find window with id: {}", id))
    }
  })
}

#[json_op]
fn window_set_max_inner_size(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let id = json["id"].as_u64().unwrap();
  let max_size: Option<Size> =
    if let Ok(size) = SizeDef::deserialize(json["maxSize"].to_owned()) {
      Some(size)
    } else {
      None
    };

  WINDOW_MAP.with(|cell| {
    let window_map = cell.borrow();

    if let Some(window) = window_map.get(&id) {
      window.set_max_inner_size(max_size);
      Ok(json!(()))
    } else {
      Err(anyhow!("Could not find window with id: {}", id))
    }
  })
}

#[json_op]
fn window_set_title(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let id = json["id"].as_u64().unwrap();
  let title = json["title"].as_str().unwrap();

  WINDOW_MAP.with(|cell| {
    let window_map = cell.borrow();

    if let Some(window) = window_map.get(&id) {
      window.set_title(title);
      Ok(json!(()))
    } else {
      Err(anyhow!("Could not find window with id: {}", id))
    }
  })
}

#[json_op]
fn window_set_visible(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let id = json["id"].as_u64().unwrap();
  let visible = json["visible"].as_bool().unwrap();

  WINDOW_MAP.with(|cell| {
    let window_map = cell.borrow();

    if let Some(window) = window_map.get(&id) {
      window.set_visible(visible);
      Ok(json!(()))
    } else {
      Err(anyhow!("Could not find window with id: {}", id))
    }
  })
}

#[json_op]
fn window_set_resizable(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let id = json["id"].as_u64().unwrap();
  let resizable = json["resizable"].as_bool().unwrap();

  WINDOW_MAP.with(|cell| {
    let window_map = cell.borrow();

    if let Some(window) = window_map.get(&id) {
      window.set_resizable(resizable);
      Ok(json!(()))
    } else {
      Err(anyhow!("Could not find window with id: {}", id))
    }
  })
}

#[json_op]
fn window_set_minimized(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let id = json["id"].as_u64().unwrap();
  let minimized = json["minimized"].as_bool().unwrap();

  WINDOW_MAP.with(|cell| {
    let window_map = cell.borrow();

    if let Some(window) = window_map.get(&id) {
      window.set_minimized(minimized);
      Ok(json!(()))
    } else {
      Err(anyhow!("Could not find window with id: {}", id))
    }
  })
}

#[json_op]
fn window_set_maximized(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let id = json["id"].as_u64().unwrap();
  let maximized = json["maximized"].as_bool().unwrap();

  WINDOW_MAP.with(|cell| {
    let window_map = cell.borrow();

    if let Some(window) = window_map.get(&id) {
      window.set_maximized(maximized);
      Ok(json!(()))
    } else {
      Err(anyhow!("Could not find window with id: {}", id))
    }
  })
}

#[json_op]
fn window_set_decorations(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let id = json["id"].as_u64().unwrap();
  let decorations = json["decorations"].as_bool().unwrap();

  WINDOW_MAP.with(|cell| {
    let window_map = cell.borrow();

    if let Some(window) = window_map.get(&id) {
      window.set_decorations(decorations);
      Ok(json!(()))
    } else {
      Err(anyhow!("Could not find window with id: {}", id))
    }
  })
}

#[json_op]
fn window_set_always_on_top(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let id = json["id"].as_u64().unwrap();
  let always_on_top = json["alwaysOnTop"].as_bool().unwrap();

  WINDOW_MAP.with(|cell| {
    let window_map = cell.borrow();

    if let Some(window) = window_map.get(&id) {
      window.set_always_on_top(always_on_top);
      Ok(json!(()))
    } else {
      Err(anyhow!("Could not find window with id: {}", id))
    }
  })
}

#[json_op]
fn window_set_window_icon(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let id = json["id"].as_u64().unwrap();
  let rgba: Vec<u8> = json["rgba"]
    .as_array()
    .unwrap()
    .iter()
    .map(|v| v.as_u64().unwrap() as u8)
    .collect();
  let width = json["width"].as_u64().unwrap();
  let height = json["height"].as_u64().unwrap();

  WINDOW_MAP.with(|cell| {
    let window_map = cell.borrow();

    if let Some(window) = window_map.get(&id) {
      window.set_window_icon(rgba, width, height)?;
      Ok(json!(()))
    } else {
      Err(anyhow!("Could not find window with id: {}", id))
    }
  })
}

#[json_op]
fn window_set_ime_position(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let id = json["id"].as_u64().unwrap();
  let position: Position =
    PositionDef::deserialize(json["position"].to_owned()).unwrap();

  WINDOW_MAP.with(|cell| {
    let window_map = cell.borrow();

    if let Some(window) = window_map.get(&id) {
      window.set_ime_position(position);
      Ok(json!(()))
    } else {
      Err(anyhow!("Could not find window with id: {}", id))
    }
  })
}

#[json_op]
fn window_request_user_attention(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let id = json["id"].as_u64().unwrap();
  let request_type: Option<UserAttentionType> =
    if json["requestType"].is_null() {
      None
    } else {
      Some(
        UserAttentionTypeDef::deserialize(json["requestType"].to_owned())
          .unwrap(),
      )
    };

  WINDOW_MAP.with(|cell| {
    let window_map = cell.borrow();

    if let Some(window) = window_map.get(&id) {
      window.request_user_attention(request_type);
      Ok(json!(()))
    } else {
      Err(anyhow!("Could not find window with id: {}", id))
    }
  })
}

#[json_op]
fn window_set_cursor_icon(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let id = json["id"].as_u64().unwrap();
  let cursor: CursorIcon =
    CursorIconDef::deserialize(json["cursor"].to_owned()).unwrap();

  WINDOW_MAP.with(|cell| {
    let window_map = cell.borrow();

    if let Some(window) = window_map.get(&id) {
      window.set_cursor_icon(cursor);
      Ok(json!(()))
    } else {
      Err(anyhow!("Could not find window with id: {}", id))
    }
  })
}

#[json_op]
fn window_set_cursor_position(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let id = json["id"].as_u64().unwrap();
  let position: Position =
    PositionDef::deserialize(json["position"].to_owned()).unwrap();

  WINDOW_MAP.with(|cell| {
    let window_map = cell.borrow();

    if let Some(window) = window_map.get(&id) {
      window.set_cursor_position(position)?;
      Ok(json!(()))
    } else {
      Err(anyhow!("Could not find window with id: {}", id))
    }
  })
}

#[json_op]
fn window_set_cursor_grab(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let id = json["id"].as_u64().unwrap();
  let grab = json["grab"].as_bool().unwrap();

  WINDOW_MAP.with(|cell| {
    let window_map = cell.borrow();

    if let Some(window) = window_map.get(&id) {
      window.set_cursor_grab(grab)?;
      Ok(json!(()))
    } else {
      Err(anyhow!("Could not find window with id: {}", id))
    }
  })
}

#[json_op]
fn window_set_cursor_visible(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let id = json["id"].as_u64().unwrap();
  let visible = json["visible"].as_bool().unwrap();

  WINDOW_MAP.with(|cell| {
    let window_map = cell.borrow();

    if let Some(window) = window_map.get(&id) {
      window.set_cursor_visible(visible);
      Ok(json!(()))
    } else {
      Err(anyhow!("Could not find window with id: {}", id))
    }
  })
}

#[json_op]
fn window_render_frame(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let id = json["id"].as_u64().unwrap();

  WINDOW_MAP.with(|cell| {
    let mut window_map = cell.borrow_mut();

    if let Some(window) = window_map.get_mut(&id) {
      window.render_frame()?;
      Ok(json!(()))
    } else {
      Err(anyhow!("Could not find window with id: {}", id))
    }
  })
}

#[json_op]
fn window_draw_frame(
  json: Value,
  zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let id = json["id"].as_u64().unwrap();

  WINDOW_MAP.with(|cell| {
    let mut window_map = cell.borrow_mut();

    if let Some(window) = window_map.get_mut(&id) {
      window.draw_frame(&mut zero_copy[0]);
      Ok(json!(()))
    } else {
      Err(anyhow!("Could not find window with id: {}", id))
    }
  })
}

#[json_op]
fn window_resize_frame(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  let id = json["id"].as_u64().unwrap();
  let width = json["width"].as_u64().unwrap() as u32;
  let height = json["height"].as_u64().unwrap() as u32;

  WINDOW_MAP.with(|cell| {
    let mut window_map = cell.borrow_mut();

    if let Some(window) = window_map.get_mut(&id) {
      window.resize_frame(width, height);
      Ok(json!(()))
    } else {
      Err(anyhow!("Could not find window with id: {}", id))
    }
  })
}

fn window_view_frame(
  _interface: &mut dyn Interface,
  zero_copy: &mut [ZeroCopyBuf],
) -> Op {
  let json: Value = serde_json::from_slice(&zero_copy[0]).unwrap();
  let id = json["id"].as_u64().unwrap();

  WINDOW_MAP.with(|cell| {
    let mut window_map = cell.borrow_mut();

    if let Some(window) = window_map.get_mut(&id) {
      let frame = Box::from(window.view_frame());
      Op::Sync(frame)
    } else {
      panic!("Could not get frame buffer");
    }
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
