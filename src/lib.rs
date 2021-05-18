use std::borrow::Cow;
use std::cell::RefCell;

use deno_core::error::bad_resource_id;
use deno_core::error::AnyError;
use deno_core::op_sync;
use deno_core::serde::Deserialize;
use deno_core::Extension;
use deno_core::OpState;
use deno_core::Resource;
use deno_core::ResourceId;
use deno_core::ZeroCopyBuf;

use helpers::hash;
use winit::dpi::PhysicalPosition;
use winit::dpi::PhysicalSize;
use winit::dpi::Position;
use winit::dpi::Size;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::CursorIcon;
use winit::window::Icon;
use winit::window::Window;

mod event;
mod helpers;

use event::Event;

pub mod resources {
  pub use super::WindowResource;
  pub use super::EventLoopResource;
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
enum UserAttentionType {
  Critical,
  Informational,
}

impl From<UserAttentionType> for winit::window::UserAttentionType {
  fn from(request_type: UserAttentionType) -> Self {
    match request_type {
      UserAttentionType::Critical => winit::window::UserAttentionType::Critical,
      UserAttentionType::Informational => {
        winit::window::UserAttentionType::Informational
      }
    }
  }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WindowPositionArgs {
  rid: ResourceId,
  position: Position,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WindowSizeArgs {
  rid: ResourceId,
  size: Size,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WindowOptionSizeArgs {
  rid: ResourceId,
  size: Option<Size>,
}

#[derive(Deserialize)]
struct WindowTitleArgs {
  rid: ResourceId,
  title: String,
}

#[derive(Deserialize)]
struct WindowVisibleArgs {
  rid: ResourceId,
  visible: bool,
}

#[derive(Deserialize)]
struct WindowResizableArgs {
  rid: ResourceId,
  resizable: bool,
}

#[derive(Deserialize)]
struct WindowMinimizedArgs {
  rid: ResourceId,
  minimized: bool,
}

#[derive(Deserialize)]
struct WindowMaximizedArgs {
  rid: ResourceId,
  maximized: bool,
}

#[derive(Deserialize)]
struct WindowDecorationsArgs {
  rid: ResourceId,
  decorations: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WindowAlwaysOnTopArgs {
  rid: ResourceId,
  always_on_top: bool,
}

#[derive(Deserialize)]
struct WindowIconArgs {
  rid: ResourceId,
  rgba: Vec<u8>,
  width: u32,
  height: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WindowUserAttentionArgs {
  rid: ResourceId,
  request_type: Option<UserAttentionType>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WindowCursorIconArgs {
  rid: ResourceId,
  cursor: CursorIcon,
}

#[derive(Deserialize)]
struct WindowCursorGrabArgs {
  rid: ResourceId,
  grab: bool,
}

pub struct EventLoopResource(RefCell<EventLoop<()>>);

impl Resource for EventLoopResource {
  fn name(&self) -> Cow<str> {
    "eventLoop".into()
  }
}

pub struct WindowResource(Window);

impl WindowResource {
  pub fn new(event_loop: &EventLoop<()>) -> Result<Self, AnyError> {
    Ok(Self(winit::window::Window::new(event_loop)?))
  }

  pub fn id(&self) -> u32 {
    hash(self.0.id())
  }
}

impl Resource for WindowResource {
  fn name(&self) -> Cow<str> {
    "window".into()
  }
}

#[no_mangle]
pub fn init() -> Extension {
  Extension::builder()
    .ops(vec![
      ("pane_event_loop_new", op_sync(event_loop_new)),
      ("pane_event_loop_step", op_sync(event_loop_step)),
      ("pane_window_new", op_sync(window_new)),
      ("pane_window_id", op_sync(window_id)),
      ("pane_window_scale_factor", op_sync(window_scale_factor)),
      ("pane_window_request_redraw", op_sync(window_request_redraw)),
      ("pane_window_inner_position", op_sync(window_inner_position)),
      ("pane_window_outer_position", op_sync(window_outer_position)),
      (
        "pane_window_set_outer_position",
        op_sync(window_set_outer_position),
      ),
      ("pane_window_inner_size", op_sync(window_inner_size)),
      ("pane_window_set_inner_size", op_sync(window_set_inner_size)),
      ("pane_window_outer_size", op_sync(window_outer_size)),
      (
        "pane_window_set_min_inner_size",
        op_sync(window_set_min_inner_size),
      ),
      (
        "pane_window_set_max_inner_size",
        op_sync(window_set_max_inner_size),
      ),
      ("pane_window_set_title", op_sync(window_set_title)),
      ("pane_window_set_visible", op_sync(window_set_visible)),
      ("pane_window_set_resizable", op_sync(window_set_resizable)),
      ("pane_window_set_minimized", op_sync(window_set_minimized)),
      ("pane_window_set_maximized", op_sync(window_set_maximized)),
      ("pane_window_is_maximized", op_sync(window_is_maximized)),
      (
        "pane_window_set_decorations",
        op_sync(window_set_decorations),
      ),
      (
        "pane_window_set_always_on_top",
        op_sync(window_set_always_on_top),
      ),
      (
        "pane_window_set_window_icon",
        op_sync(window_set_window_icon),
      ),
      (
        "pane_window_set_ime_position",
        op_sync(window_set_ime_position),
      ),
      (
        "pane_window_request_user_attention",
        op_sync(window_request_user_attention),
      ),
      (
        "pane_window_set_cursor_icon",
        op_sync(window_set_cursor_icon),
      ),
      (
        "pane_window_set_cursor_position",
        op_sync(window_set_cursor_position),
      ),
      (
        "pane_window_set_cursor_grab",
        op_sync(window_set_cursor_grab),
      ),
      (
        "pane_window_set_cursor_visible",
        op_sync(window_set_cursor_visible),
      ),
      ("pane_window_drag_window", op_sync(window_drag_window)),
    ])
    .build()
}

fn event_loop_new(
  state: &mut OpState,
  _args: (),
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<ResourceId, AnyError> {
  Ok(
    state
      .resource_table
      .add(EventLoopResource(RefCell::new(EventLoop::new()))),
  )
}

fn event_loop_step(
  state: &mut OpState,
  rid: ResourceId,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<Vec<Event>, AnyError> {
  let event_loop = state
    .resource_table
    .get::<EventLoopResource>(rid)
    .ok_or_else(bad_resource_id)?;
  let mut events = Vec::new();

  event_loop
    .0
    .borrow_mut()
    .run_return(|event, _, control_flow| {
      *control_flow = ControlFlow::Exit;
      events.push(Event::from(event));
    });

  Ok(events)
}

fn window_new(
  state: &mut OpState,
  rid: ResourceId,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<ResourceId, AnyError> {
  let event_loop = state
    .resource_table
    .get::<EventLoopResource>(rid)
    .ok_or_else(bad_resource_id)?;

  let event_loop = event_loop.0.borrow_mut();

  Ok(state.resource_table.add(WindowResource::new(&event_loop)?))
}

fn window_id(
  state: &mut OpState,
  rid: ResourceId,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<u32, AnyError> {
  let window = state
    .resource_table
    .get::<WindowResource>(rid)
    .ok_or_else(bad_resource_id)?;

  Ok(window.id())
}

fn window_scale_factor(
  state: &mut OpState,
  rid: ResourceId,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<f64, AnyError> {
  let window = state
    .resource_table
    .get::<WindowResource>(rid)
    .ok_or_else(bad_resource_id)?;

  Ok(window.0.scale_factor())
}

fn window_request_redraw(
  state: &mut OpState,
  rid: ResourceId,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<(), AnyError> {
  let window = state
    .resource_table
    .get::<WindowResource>(rid)
    .ok_or_else(bad_resource_id)?;

  window.0.request_redraw();

  Ok(())
}

fn window_inner_position(
  state: &mut OpState,
  rid: ResourceId,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<PhysicalPosition<i32>, AnyError> {
  let window = state
    .resource_table
    .get::<WindowResource>(rid)
    .ok_or_else(bad_resource_id)?;

  Ok(window.0.inner_position()?)
}

fn window_outer_position(
  state: &mut OpState,
  rid: ResourceId,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<PhysicalPosition<i32>, AnyError> {
  let window = state
    .resource_table
    .get::<WindowResource>(rid)
    .ok_or_else(bad_resource_id)?;

  Ok(window.0.outer_position()?)
}

fn window_set_outer_position(
  state: &mut OpState,
  args: WindowPositionArgs,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<(), AnyError> {
  let window = state
    .resource_table
    .get::<WindowResource>(args.rid)
    .ok_or_else(bad_resource_id)?;

  window.0.set_outer_position(args.position);

  Ok(())
}

fn window_inner_size(
  state: &mut OpState,
  rid: ResourceId,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<PhysicalSize<u32>, AnyError> {
  let window = state
    .resource_table
    .get::<WindowResource>(rid)
    .ok_or_else(bad_resource_id)?;

  Ok(window.0.inner_size())
}

fn window_set_inner_size(
  state: &mut OpState,
  args: WindowSizeArgs,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<(), AnyError> {
  let window = state
    .resource_table
    .get::<WindowResource>(args.rid)
    .ok_or_else(bad_resource_id)?;

  window.0.set_inner_size(args.size);

  Ok(())
}

fn window_outer_size(
  state: &mut OpState,
  rid: ResourceId,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<PhysicalSize<u32>, AnyError> {
  let window = state
    .resource_table
    .get::<WindowResource>(rid)
    .ok_or_else(bad_resource_id)?;

  Ok(window.0.outer_size())
}

fn window_set_min_inner_size(
  state: &mut OpState,
  args: WindowOptionSizeArgs,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<(), AnyError> {
  let window = state
    .resource_table
    .get::<WindowResource>(args.rid)
    .ok_or_else(bad_resource_id)?;

  window.0.set_min_inner_size(args.size);

  Ok(())
}

fn window_set_max_inner_size(
  state: &mut OpState,
  args: WindowOptionSizeArgs,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<(), AnyError> {
  let window = state
    .resource_table
    .get::<WindowResource>(args.rid)
    .ok_or_else(bad_resource_id)?;

  window.0.set_max_inner_size(args.size);

  Ok(())
}

fn window_set_title(
  state: &mut OpState,
  args: WindowTitleArgs,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<(), AnyError> {
  let window = state
    .resource_table
    .get::<WindowResource>(args.rid)
    .ok_or_else(bad_resource_id)?;

  window.0.set_title(&args.title);

  Ok(())
}

fn window_set_visible(
  state: &mut OpState,
  args: WindowVisibleArgs,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<(), AnyError> {
  let window = state
    .resource_table
    .get::<WindowResource>(args.rid)
    .ok_or_else(bad_resource_id)?;

  window.0.set_visible(args.visible);

  Ok(())
}

fn window_set_resizable(
  state: &mut OpState,
  args: WindowResizableArgs,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<(), AnyError> {
  let window = state
    .resource_table
    .get::<WindowResource>(args.rid)
    .ok_or_else(bad_resource_id)?;

  window.0.set_resizable(args.resizable);

  Ok(())
}

fn window_set_minimized(
  state: &mut OpState,
  args: WindowMinimizedArgs,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<(), AnyError> {
  let window = state
    .resource_table
    .get::<WindowResource>(args.rid)
    .ok_or_else(bad_resource_id)?;

  window.0.set_minimized(args.minimized);

  Ok(())
}

fn window_set_maximized(
  state: &mut OpState,
  args: WindowMaximizedArgs,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<(), AnyError> {
  let window = state
    .resource_table
    .get::<WindowResource>(args.rid)
    .ok_or_else(bad_resource_id)?;

  window.0.set_maximized(args.maximized);

  Ok(())
}

fn window_is_maximized(
  state: &mut OpState,
  rid: ResourceId,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<bool, AnyError> {
  let window = state
    .resource_table
    .get::<WindowResource>(rid)
    .ok_or_else(bad_resource_id)?;

  Ok(window.0.is_maximized())
}

fn window_set_decorations(
  state: &mut OpState,
  args: WindowDecorationsArgs,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<(), AnyError> {
  let window = state
    .resource_table
    .get::<WindowResource>(args.rid)
    .ok_or_else(bad_resource_id)?;

  window.0.set_decorations(args.decorations);

  Ok(())
}

fn window_set_always_on_top(
  state: &mut OpState,
  args: WindowAlwaysOnTopArgs,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<(), AnyError> {
  let window = state
    .resource_table
    .get::<WindowResource>(args.rid)
    .ok_or_else(bad_resource_id)?;

  window.0.set_always_on_top(args.always_on_top);

  Ok(())
}

fn window_set_window_icon(
  state: &mut OpState,
  args: WindowIconArgs,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<(), AnyError> {
  let window = state
    .resource_table
    .get::<WindowResource>(args.rid)
    .ok_or_else(bad_resource_id)?;

  window
    .0
    .set_window_icon(Icon::from_rgba(args.rgba, args.width, args.height).ok());
  Ok(())
}

fn window_set_ime_position(
  state: &mut OpState,
  args: WindowPositionArgs,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<(), AnyError> {
  let window = state
    .resource_table
    .get::<WindowResource>(args.rid)
    .ok_or_else(bad_resource_id)?;

  window.0.set_ime_position(args.position);

  Ok(())
}

fn window_request_user_attention(
  state: &mut OpState,
  args: WindowUserAttentionArgs,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<(), AnyError> {
  let window = state
    .resource_table
    .get::<WindowResource>(args.rid)
    .ok_or_else(bad_resource_id)?;

  window.0.request_user_attention(
    args
      .request_type
      .map(winit::window::UserAttentionType::from),
  );

  Ok(())
}

fn window_set_cursor_icon(
  state: &mut OpState,
  args: WindowCursorIconArgs,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<(), AnyError> {
  let window = state
    .resource_table
    .get::<WindowResource>(args.rid)
    .ok_or_else(bad_resource_id)?;

  window.0.set_cursor_icon(args.cursor);

  Ok(())
}

fn window_set_cursor_position(
  state: &mut OpState,
  args: WindowPositionArgs,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<(), AnyError> {
  let window = state
    .resource_table
    .get::<WindowResource>(args.rid)
    .ok_or_else(bad_resource_id)?;

  Ok(window.0.set_cursor_position(args.position)?)
}

fn window_set_cursor_grab(
  state: &mut OpState,
  args: WindowCursorGrabArgs,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<(), AnyError> {
  let window = state
    .resource_table
    .get::<WindowResource>(args.rid)
    .ok_or_else(bad_resource_id)?;

  Ok(window.0.set_cursor_grab(args.grab)?)
}

fn window_set_cursor_visible(
  state: &mut OpState,
  args: WindowVisibleArgs,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<(), AnyError> {
  let window = state
    .resource_table
    .get::<WindowResource>(args.rid)
    .ok_or_else(bad_resource_id)?;

  window.0.set_cursor_visible(args.visible);

  Ok(())
}

fn window_drag_window(
  state: &mut OpState,
  rid: ResourceId,
  _zero_copy: Option<ZeroCopyBuf>,
) -> Result<(), AnyError> {
  let window = state
    .resource_table
    .get::<WindowResource>(rid)
    .ok_or_else(bad_resource_id)?;

  window.0.drag_window()?;

  Ok(())
}
