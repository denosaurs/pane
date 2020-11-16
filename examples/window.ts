import { EventLoopStep, load, unload, WindowNew } from "../plugin.ts";

await load();

let id = WindowNew();
setInterval(() => console.log(EventLoopStep()), 1000 / 30);
