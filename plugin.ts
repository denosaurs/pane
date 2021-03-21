import { Plug } from "./deps.ts";
import { deserialize, serialize } from "./helpers.ts";
import { Result } from "./types.ts";

const VERSION = "0.1.2";
const POLICY = Deno.env.get("PLUGIN_URL") === undefined
  ? Plug.CachePolicy.STORE
  : Plug.CachePolicy.NONE;
const PLUGIN_URL = Deno.env.get("PLUGIN_URL") ??
  `https://github.com/denosaurs/pane/releases/download/${VERSION}/`;

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

export function syncRaw(
  op: string,
  data: unknown = {},
  buf?: Uint8Array,
): Uint8Array {
  if (rid === undefined) {
    throw "The plugin must be initialized before use";
  }

  const opId = Plug.getOpId(op);
  const response = buf
    ? Plug.core.dispatch(opId, encode(data), buf)!
    : Plug.core.dispatch(opId, encode(data))!;

  return response;
}

export function sync<T>(op: string, data: unknown = {}, buf?: Uint8Array): T {
  return decode(syncRaw(op, data, buf)) as T;
}

export function unwrap<T>(result: Result<T>): T {
  if ("err" in result) {
    throw (result as { err: string }).err;
  }

  if ("ok" in result) {
    return (result as { ok: T }).ok;
  }

  throw `Invalid result (${JSON.stringify(result)})`;
}

/**
 * Loads the plugin
 */
export async function load() {
  unload();
  rid = await Plug.prepare({
    name: "pane",
    url: PLUGIN_URL,
    policy: POLICY,
  });
}

/**
 * Frees the plugin
 */
export function unload() {
  if (rid !== undefined) Deno.close(rid);
  rid = undefined;
}
