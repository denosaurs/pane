import { sync, syncRaw, unwrap } from "./plugin.ts";
import {
  CursorIcon,
  Id,
  PhysicalPosition,
  PhysicalSize,
  Position,
  Size,
} from "./types.ts";

export class Window {
  readonly id: Id;

  constructor(width: number = 320, height: number = 240) {
    this.id = unwrap(sync("window_new", { width, height }));
  }

  scaleFactor(): number {
    return unwrap(sync("window_scale_factor", { id: this.id }));
  }

  requestRedraw(): void {
    unwrap(sync("window_request_redraw", { id: this.id }));
  }

  innerPosition(): PhysicalPosition {
    return unwrap(sync("window_inner_position", { id: this.id }));
  }

  outerPosition(): PhysicalPosition {
    return unwrap(sync("window_outer_position", { id: this.id }));
  }

  setOuterPosition(position: Position): void {
    unwrap(sync("window_set_outer_position", { id: this.id, position }));
  }

  innerSize(): PhysicalSize {
    return unwrap(sync("window_inner_size", { id: this.id }));
  }

  setInnerSize(size: Size): void {
    unwrap(sync("window_set_inner_size", { id: this.id, size }));
  }

  outerSize(): PhysicalSize {
    return unwrap(sync("window_outer_size", { id: this.id }));
  }

  setMinInnerSize(minSize?: Size): void {
    unwrap(
      sync("window_set_min_inner_size", { id: this.id, minSize }),
    );
  }

  setMaxInnerSize(maxSize?: Size): void {
    unwrap(
      sync("window_set_max_inner_size", { id: this.id, maxSize }),
    );
  }

  setTitle(title: string): void {
    unwrap(sync("window_set_title", { id: this.id, title }));
  }

  setResizable(resizable: boolean): void {
    unwrap(sync("window_set_resizable", { id: this.id, resizable }));
  }

  setMinimized(minimized: boolean): void {
    unwrap(sync("window_set_minimized", { id: this.id, minimized }));
  }

  setMaximized(maximized: boolean): void {
    unwrap(sync("window_set_maximized", { id: this.id, maximized }));
  }

  setDecorations(decorations: boolean): void {
    unwrap(sync("window_set_decorations", { id: this.id, decorations }));
  }

  setAlwaysOnTop(alwaysOnTop: boolean): void {
    unwrap(
      sync(
        "window_set_always_on_top",
        { id: this.id, alwaysOnTop },
      ),
    );
  }

  setWindowIcon(
    rgba: Uint8Array,
    width: number,
    height: number,
  ): void {
    unwrap(
      sync(
        "window_set_window_icon",
        { id: this.id, rgba: Array.from(rgba), width, height },
      ),
    );
  }

  setImePosition(position: Position): void {
    unwrap(sync("window_set_ime_position", { id: this.id, position }));
  }

  setCursorIcon(cursor: CursorIcon): void {
    unwrap(sync("window_set_cursor_icon", { id: this.id, cursor }));
  }

  setCursorPosition(position: Position): void {
    unwrap(sync("window_set_cursor_position", { id: this.id, position }));
  }

  setCursorGrab(grab: boolean): void {
    unwrap(sync("window_set_cursor_grab", { id: this.id, grab }));
  }

  setCursorVisible(visible: boolean): void {
    unwrap(sync("window_set_cursor_visible", { id: this.id, visible }));
  }

  renderFrame(): void {
    unwrap(sync("window_render_frame", { id: this.id }));
  }

  drawFrame(buf: Uint8Array): void {
    unwrap(sync("window_draw_frame", { id: this.id }, buf));
  }

  resizeFrame(width: number, height: number): void {
    unwrap(sync("window_resize_frame", { id: this.id, width, height }));
  }

  viewFrame(): Uint8Array {
    return syncRaw("window_view_frame", { id: this.id });
  }
}
