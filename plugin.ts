import { Plug } from "./deps.ts";
import { deserialize, serialize } from "./helpers.ts";

const encoder = new TextEncoder();
const decoder = new TextDecoder();

let rid: number | undefined;

function decode(data: Uint8Array): unknown {
  const text = decoder.decode(data);
  return deserialize(text);
}

function encode(data: unknown): Uint8Array {
  const text = serialize(data);
  return encoder.encode(text);
}

interface Result<T> {
  err?: string;
  ok?: T;
}

function sync<R extends Result<any>>(op: string, data: unknown): R {
  if (rid === undefined) {
    throw "The plugin must be initialized before use";
  }

  const opId = Plug.getOpId(op);
  const response = Plug.core.dispatch(opId, encode(data))!;

  return decode(response) as R;
}

function unwrap<T, R extends Result<T>>(response: R): T {
  if (response.err !== undefined) {
    throw response.err;
  }

  if (response.ok !== undefined) {
    return response.ok;
  }

  throw `Invalid response`;
}

/**
 * Loads the plugin
 */
export async function load() {
  unload();
  rid = await Plug.prepare({
    name: "pane",
    url: "file://./target/debug/",
    policy: Plug.CachePolicy.NONE,
  });
}

/**
 * Frees the plugin
 */
export function unload() {
  if (rid !== undefined) Deno.close(rid);
  rid = undefined;
}

export function SurfaceNew(): bigint {
  return unwrap(sync("surface_new", {}));
}

export function SurfaceStep(id: bigint) {
  return unwrap(sync("surface_step", id));
}
