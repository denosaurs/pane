export * from "./window.ts";
export * from "./event_loop.ts";

import { load, unload } from "./plugin.ts";

await load();

// deno-lint-ignore ban-ts-comment
// @ts-ignore
if (typeof window !== "undefined") window.addEventListener("unload", unload);
