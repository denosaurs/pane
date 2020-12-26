import { EventLoop, Window } from "../mod.ts";

const width = 320;
const height = 240;

const window = new Window(width, height);

window.setInnerSize({ logical: { width: width * 2, height: height * 2 } });
window.setMinInnerSize({ logical: { width: width * 2, height: height * 2 } });
window.setMaxInnerSize({ logical: { width: width * 2, height: height * 2 } });

setInterval(() => {
  for (const event of EventLoop.Step()) {
    switch (event.type) {
      case "windowEvent":
        switch (event.value.event.type) {
          case "closeRequested":
            Deno.exit();
            break;
          case "resized":
            window.resizeFrame(
              event.value.event.value.width,
              event.value.event.value.height,
            );
            break;
        }
        break;

      case "redrawRequested":
        window.drawFrame(
          new Uint8Array(width * height * 4).fill(0).map((_) =>
            Math.floor(Math.random() * 255)
          ),
        );
        window.renderFrame();
        window.requestRedraw();
        break;
    }
  }
}, 1000 / 30);
