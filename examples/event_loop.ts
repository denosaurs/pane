import { serialize } from "../helpers.ts";
import { EventLoop, Window } from "../mod.ts";

const window = new Window();

const id = setInterval(() => {
  for (const event of EventLoop.Step()) {
    switch (event.type) {
      case "windowEvent":
        window.setCursorIcon("hand");
        console.log(serialize(event, 2));
        if (event.value.event.type === "closeRequested") {
          clearInterval(id);
        }
        break;
    }
  }
}, 1000 / 30);
