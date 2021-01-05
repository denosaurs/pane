# pane

[![Tags](https://img.shields.io/github/release/denosaurs/pane)](https://github.com/denosaurs/pane/releases)
[![Rust](https://img.shields.io/github/workflow/status/denosaurs/pane/rust)](https://github.com/denosaurs/pane/actions)
[![Deno](https://img.shields.io/github/workflow/status/denosaurs/pane/deno)](https://github.com/denosaurs/pane/actions)
[![Release](https://img.shields.io/github/workflow/status/denosaurs/pane/release)](https://github.com/denosaurs/pane/actions)
[![License](https://img.shields.io/github/license/denosaurs/pane)](https://github.com/denosaurs/pane/blob/master/LICENSE)

---
> ⚠️ Work in progress. Expect breaking changes.
---

Pane provides bindings for rust crate [winit](https://github.com/rust-windowing/winit)
in preparation for [webgpu](https://github.com/denoland/deno/pull/7977)
integration in deno. This module will provide a way of getting a [`raw_window_handle`
resource](https://github.com/denoland/deno/issues/7863#issuecomment-706897139) to
provide to deno and interaction with the window. Currently pane also provides bindings
to [pixels](https://github.com/parasyte/pixels) as a way of drawing framebuffers onto
the window.

## Example

### Draw random pixels to window

```typescript
import { Pane } from "https://deno.land/x/pane/mod.ts";

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
```

### Multiple windows

```typescript
import { Pane } from "https://deno.land/x/pane/mod.ts";
import { serialize } from "https://deno.land/x/pane/helpers.ts";

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
```

## Maintainers

- Elias Sjögreen ([@eliassjogreen](https://github.com/eliassjogreen))

## Permission Table

| Permission Needed | Required | Reason                                |
| ----------------- | -------- | ------------------------------------- |
| `--allow-env`     | yes      | For development variables.            |
| `--allow-net`     | yes      | For getting the prebuild binaries.    |
| `--allow-read`    | yes      | For reading the library.              |
| `--allow-plugin`  | yes      | It's a plugin, what do you expect.    |
| `--unstable`      | yes      | It's unstable because it is a plugin. |

## Other

### Related

- [pixels](https://github.com/parasyte/pixels)
- [winit](https://github.com/rust-windowing/winit)
- [deno_json_op](https://github.com/denosaurs/deno_json_op)

### Contribution

Pull request, issues and feedback are very welcome. Code style is formatted with `deno fmt` and commit messages are done following Conventional Commits spec.

### Licence

Copyright 2020-present, the denosaurs team. All rights reserved. MIT license.