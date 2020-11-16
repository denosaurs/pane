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

mod event;
mod helpers;
mod surface;

use surface::Surface;

thread_local! {
  static SURFACE_MAP: RefCell<HashMap<u64, Surface>> = RefCell::new(HashMap::new());
}

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
  interface.register_op("surface_new", surface_new);
  interface.register_op("surface_step", surface_step);
}

#[json_op]
fn surface_new(
  _json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  SURFACE_MAP.with(|cell| {
    let surface = Surface::new()?;
    let id = surface.id();
    cell.borrow_mut().insert(id, surface);
    Ok(json!(id))
  })
}

#[json_op]
fn surface_step(
  json: Value,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Result<Value, AnyError> {
  if let Some(id) = json.as_u64() {
    SURFACE_MAP.with(|cell| {
      let mut surface_map = cell.borrow_mut();

      if let Some(surface) = surface_map.get_mut(&id) {
        Ok(json!(surface.run()))
      } else {
        Err(anyhow!("could not find surface {}", id))
      }
    })
  } else {
    Err(anyhow!("id is none"))
  }
}
