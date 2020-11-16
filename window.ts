import { sync, unwrap } from "./plugin.ts";
import {
  CursorIcon,
  Id,
  PhysicalPosition,
  PhysicalSize,
  Position,
  Size,
} from "./types.ts";

export class Window {
  public readonly id: Id;

  constructor() {
    this.id = unwrap(sync("window_new"));
  }

  public scaleFactor(): number {
    return unwrap(sync("window_scale_factor", { id: this.id }));
  }

  public requestRedraw(): void {
    unwrap(sync("window_request_redraw", { id: this.id }));
  }

  public innerPosition(): PhysicalPosition {
    return unwrap(sync("window_inner_position", { id: this.id }));
  }

  public outerPosition(): PhysicalPosition {
    return unwrap(sync("window_outer_position", { id: this.id }));
  }

  public setOuterPosition(position: Position): void {
    unwrap(sync("window_set_outer_position", { id: this.id, position }));
  }

  public innerSize(): PhysicalSize {
    return unwrap(sync("window_inner_size", { id: this.id }));
  }

  public setInnerSize(size: Size): void {
    unwrap(sync("window_set_inner_size", { id: this.id, size }));
  }

  public outerSize(): PhysicalSize {
    return unwrap(sync("window_outer_size", { id: this.id }));
  }

  public setMinInnerSize(minSize?: Size): void {
    unwrap(
      sync("window_set_min_inner_size", { id: this.id, minSize }),
    );
  }

  public setMaxInnerSize(maxSize?: Size): void {
    unwrap(
      sync("window_set_max_inner_size", { id: this.id, maxSize }),
    );
  }

  public setTitle(title: string): void {
    unwrap(sync("window_set_title", { id: this.id, title }));
  }

  public setResizable(resizable: boolean): void {
    unwrap(sync("window_set_resizable", { id: this.id, resizable }));
  }

  public setMinimized(minimized: boolean): void {
    unwrap(sync("window_set_minimized", { id: this.id, minimized }));
  }

  public setMaximized(maximized: boolean): void {
    unwrap(sync("window_set_maximized", { id: this.id, maximized }));
  }

  public setDecorations(decorations: boolean): void {
    unwrap(sync("window_set_decorations", { id: this.id, decorations }));
  }

  public setAlwaysOnTop(alwaysOnTop: boolean): void {
    unwrap(
      sync(
        "window_set_always_on_top",
        { id: this.id, alwaysOnTop },
      ),
    );
  }

  public setWindowIcon(
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

  public setImePosition(position: Position): void {
    unwrap(sync("window_set_ime_position", { id: this.id, position }));
  }

  public setCursorIcon(cursor: CursorIcon): void {
    unwrap(sync("window_set_cursor_icon", { id: this.id, cursor }));
  }

  public setCursorPosition(position: Position): void {
    unwrap(sync("window_set_cursor_position", { id: this.id, position }));
  }

  public setCursorGrab(grab: boolean): void {
    unwrap(sync("window_set_cursor_grab", { id: this.id, grab }));
  }

  public setCursorVisible(visible: boolean): void {
    unwrap(sync("window_set_cursor_visible", { id: this.id, visible }));
  }
}
