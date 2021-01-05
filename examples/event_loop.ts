import { serialize } from "../helpers.ts";
import { Pane } from "../mod.ts";

const window1 = new Pane();
const window2 = new Pane();

setInterval(() => {
  for (const event of Pane.Step()) {
    switch (event.type) {
      case "windowEvent":
        console.log(serialize(event, 2));
        if (event.value.event.type === "closeRequested") {
          Deno.exit();
        }

        if (event.value.event.type === "cursorEntered") {
          if (event.value.windowId === window1.id) {
            window1.setCursorIcon("hand");
          } else {
            window2.setCursorIcon("crosshair");
          }
        }
        break;
    }
  }
}, 1000 / 30);
