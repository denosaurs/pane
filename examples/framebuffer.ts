import { Pane } from "../mod.ts";

const width = 320;
const height = 240;

const pane = new Pane(width, height);

pane.setInnerSize({ logical: { width: width * 2, height: height * 2 } });
pane.setMinInnerSize({ logical: { width: width * 2, height: height * 2 } });
pane.setMaxInnerSize({ logical: { width: width * 2, height: height * 2 } });

setInterval(() => {
  for (const event of Pane.Step()) {
    switch (event.type) {
      case "windowEvent":
        switch (event.value.event.type) {
          case "closeRequested":
            Deno.exit();
            break;
          case "resized":
            pane.resizeFrame(
              event.value.event.value.width,
              event.value.event.value.height,
            );
            break;
        }
        break;

      case "redrawRequested":
        pane.drawFrame(
          new Uint8Array(width * height * 4).fill(0).map((_) =>
            Math.floor(Math.random() * 255)
          ),
        );
        pane.renderFrame();
        pane.requestRedraw();
        break;
    }
  }
}, 1000 / 30);
