import { PaneEventLoop } from "../mod.ts";

const eventLoop = new PaneEventLoop();

console.log(eventLoop.step());
