export * from "./window.ts";
export * from "./event_loop.ts";

import { load, unload } from "./plugin.ts";

await load();

// deno-fmt-ignore
// deno-lint-ignore
// @ts-ignore
if (typeof window !== "undefined") window.addEventListener("unload", unload);
