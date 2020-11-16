import { serialize } from "../helpers.ts";
import { EventLoop, Window } from "../mod.ts";

const window1 = new Window();
const window2 = new Window();

const id = setInterval(() => {
  for (const event of EventLoop.Step()) {
    switch (event.type) {
      case "windowEvent":
        console.log(serialize(event, 2));
        if (event.value.event.type === "closeRequested") {
          clearInterval(id);
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
