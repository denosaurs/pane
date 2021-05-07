import { PaneEventLoop, PaneWindow } from "../mod.ts";

const eventLoop = new PaneEventLoop();
const _pane1 = new PaneWindow(eventLoop);
const _pane2 = new PaneWindow(eventLoop);

setInterval(() => {
  for (const event of eventLoop.step()) {
    if (
      event.type === "windowEvent" &&
      event.value.event.type === "closeRequested"
    ) {
      Deno.exit();
    }
  }
}, 0);
