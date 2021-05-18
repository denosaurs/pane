# pane

[![Tags](https://img.shields.io/github/release/denosaurs/pane)](https://github.com/denosaurs/pane/releases)
[![deno doc](https://doc.deno.land/badge.svg)](https://doc.deno.land/https/deno.land/x/pane/mod.ts)
[![deno](https://github.com/denosaurs/pane/actions/workflows/deno.yml/badge.svg)](https://github.com/denosaurs/pane/actions/workflows/deno.yml)
[![release](https://github.com/denosaurs/pane/actions/workflows/release.yml/badge.svg)](https://github.com/denosaurs/pane/actions/workflows/release.yml)
[![rust](https://github.com/denosaurs/pane/actions/workflows/rust.yml/badge.svg)](https://github.com/denosaurs/pane/actions/workflows/rust.yml)
[![License](https://img.shields.io/github/license/denosaurs/pane)](https://github.com/denosaurs/pane/blob/master/LICENSE)
[![Dependencies](https://img.shields.io/endpoint?url=https%3A%2F%2Fdeno-visualizer.danopia.net%2Fshields%2Fdep-count%2Fhttps%2Fdeno.land%2Fx%2Fpane%2Fmod.ts)](https://deno-visualizer.danopia.net/dependencies-of/https/deno.land/x/pane/mod.ts)
[![Dependency freshness](https://img.shields.io/endpoint?url=https%3A%2F%2Fdeno-visualizer.danopia.net%2Fshields%2Fupdates%2Fhttps%2Fdeno.land%2Fx%2Fpane%2Fmod.ts)](https://deno-visualizer.danopia.net/dependencies-of/https/deno.land/x/pane/mod.ts)

---

> ⚠️ Work in progress. Expect breaking changes.

---

Pane provides bindings for rust crate
[winit](https://github.com/rust-windowing/winit) in preparation for
[webgpu](https://github.com/denoland/deno/pull/7977) integration in deno. This
module will provide a way of getting a
[`raw_window_handle` resource](https://github.com/denoland/deno/issues/7863#issuecomment-706897139)
to provide to deno and interaction with the window. Pane no longer provides
bindings to [pixels](https://github.com/parasyte/pixels) as a way of drawing
framebuffers onto the window, instead use WebGPU.

## Example

### Singe window

```typescript
import { PaneEventLoop, PaneWindow } from "https://deno.land/x/pane/mod.ts";

const eventLoop = new PaneEventLoop();
const _pane = new PaneWindow(eventLoop);

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
```

### Multiple windows

```typescript
import { PaneEventLoop, PaneWindow } from "https://deno.land/x/pane/mod.ts";

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

- [winit](https://github.com/rust-windowing/winit)

### Contribution

Pull request, issues and feedback are very welcome. Code style is formatted with
`deno fmt` and commit messages are done following Conventional Commits spec.

### Licence

Copyright 2020-2021, the denosaurs team. All rights reserved. MIT license.
