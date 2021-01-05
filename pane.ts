import { sync, syncRaw, unwrap } from "./plugin.ts";
import {
  CursorIcon,
  Id,
  PaneEvent,
  PhysicalPosition,
  PhysicalSize,
  Position,
  Size,
  UserAttentionType,
} from "./types.ts";

/**
 * Represents a window.
 */
export class Pane {
  /** Takes a step in the global event loop, returning an array of `PaneEvent`s. */
  static Step(): PaneEvent[] {
    return unwrap(sync("event_loop_step"));
  }

  /** This pane windows unique id. */
  readonly id: Id;

  constructor(width: number = 320, height: number = 240) {
    this.id = unwrap(sync("window_new", { width, height }));
  }

  /**
   * Returns the scale factor that can be used to map logical pixels to physical
   * pixels, and vice versa.
   */
  scaleFactor(): number {
    return unwrap(sync("window_scale_factor", { id: this.id }));
  }

  /**
   * Emits a `redrawRequested` event in the event loop after all OS events have
   * been processed by the event loop.
   * 
   * This is the strongly encouraged method of redrawing windows, as it can integrate
   * with OS-requested redraws (e.g. when a window gets resized).
   */
  requestRedraw(): void {
    unwrap(sync("window_request_redraw", { id: this.id }));
  }

  /**
   * Returns the position of the top-left hand corner of the window's client area
   * relative to the top-left hand corner of the desktop.
   * 
   * Note that the top-left hand corner of the desktop is not necessarily the same
   * as the screen. If the user uses a desktop with multiple monitors, the top-left
   * hand corner of the desktop is the top-left hand corner of the monitor at the
   * top-left of the desktop. The coordinates can be negative if the top-left hand
   * corner of the window is outside of the visible screen region.
   */
  innerPosition(): PhysicalPosition {
    return unwrap(sync("window_inner_position", { id: this.id }));
  }

  /**
   * Returns the position of the top-left hand corner of the window relative to
   * the top-left hand corner of the desktop.
   * 
   * Note that the top-left hand corner of the desktop is not necessarily the same
   * as the screen. If the user uses a desktop with multiple monitors, the top-left
   * hand corner of the desktop is the top-left hand corner of the monitor at the
   * top-left of the desktop. The coordinates can be negative if the top-left hand
   * corner of the window is outside of the visible screen region.
   */
  outerPosition(): PhysicalPosition {
    return unwrap(sync("window_outer_position", { id: this.id }));
  }

  /** Modifies the position of the window. */
  setOuterPosition(position: Position): void {
    unwrap(sync("window_set_outer_position", { id: this.id, position }));
  }

  /**
   * Returns the physical size of the window's client area.
   * 
   * The client area is the content of the window, excluding the title bar and borders.
   */
  innerSize(): PhysicalSize {
    return unwrap(sync("window_inner_size", { id: this.id }));
  }

  /**
   * Modifies the inner size of the window.
   * 
   * See `innerSize` for more information about the values. This automatically
   * un-maximizes the window if it's maximized.
   */
  setInnerSize(size: Size): void {
    unwrap(sync("window_set_inner_size", { id: this.id, size }));
  }

  /**
   * Returns the physical size of the entire window.
   * 
   * These dimensions include the title bar and borders. If you don't want that
   * (and you usually don't), use `innerSize` instead.
   */
  outerSize(): PhysicalSize {
    return unwrap(sync("window_outer_size", { id: this.id }));
  }

  /** Sets a minimum dimension size for the window. */
  setMinInnerSize(minSize?: Size): void {
    unwrap(
      sync("window_set_min_inner_size", { id: this.id, minSize }),
    );
  }

  /** Sets a maximum dimension size for the window. */
  setMaxInnerSize(maxSize?: Size): void {
    unwrap(
      sync("window_set_max_inner_size", { id: this.id, maxSize }),
    );
  }

  /** Modifies the title of the window. */
  setTitle(title: string): void {
    unwrap(sync("window_set_title", { id: this.id, title }));
  }

  /**
   * Modifies the window's visibility.
   * 
   * If `false`, this will hide the window. If `true`, this will show the window.
   */
  setVisible(visible: boolean): void {
    unwrap(sync("window_set_visible", { id: this.id, visible }));
  }

  /**
   * Sets whether the window is resizable or not.
   * 
   * Note that making the window unresizable doesn't exempt you from handling
   * `resized`, as that event can still be triggered by DPI scaling, entering
   * fullscreen mode, etc.
   */
  setResizable(resizable: boolean): void {
    unwrap(sync("window_set_resizable", { id: this.id, resizable }));
  }

  /** Sets the window to minimized or back. */
  setMinimized(minimized: boolean): void {
    unwrap(sync("window_set_minimized", { id: this.id, minimized }));
  }

  /** Sets the window to maximized or back. */
  setMaximized(maximized: boolean): void {
    unwrap(sync("window_set_maximized", { id: this.id, maximized }));
  }

  /** Turn window decorations on or off. */
  setDecorations(decorations: boolean): void {
    unwrap(sync("window_set_decorations", { id: this.id, decorations }));
  }

  /** Change whether or not the window will always be on top of other windows. */
  setAlwaysOnTop(alwaysOnTop: boolean): void {
    unwrap(
      sync(
        "window_set_always_on_top",
        { id: this.id, alwaysOnTop },
      ),
    );
  }

  /**
   * Sets the window icon. On Windows and X11, this is typically the small icon
   * in the top-left corner of the titlebar.
   */
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

  /**
   * Sets location of IME candidate box in client area coordinates relative to
   * the top left.
   */
  setImePosition(position: Position): void {
    unwrap(sync("window_set_ime_position", { id: this.id, position }));
  }

  /**
   * Requests user attention to the window, this has no effect if the application
   * is already focused. How requesting for user attention manifests is platform
   * dependent, see `UserAttentionType` for details.
   * 
   * Providing no type will unset the request for user attention. Unsetting the
   * request for user attention might not be done automatically by the WM when
   * the window receives input.
   */
  requestUserAttention(requestType?: UserAttentionType) {
    unwrap(sync("window_request_user_attention", { id: this.id, requestType }));
  }

  /** Modifies the cursor icon of the window. */
  setCursorIcon(cursor: CursorIcon): void {
    unwrap(sync("window_set_cursor_icon", { id: this.id, cursor }));
  }

  /** Changes the position of the cursor in window coordinates. */
  setCursorPosition(position: Position): void {
    unwrap(sync("window_set_cursor_position", { id: this.id, position }));
  }

  /**
   * Grabs the cursor, preventing it from leaving the window.
   * 
   * There's no guarantee that the cursor will be hidden. You should hide it by
   * yourself if you want so.
   */
  setCursorGrab(grab: boolean): void {
    unwrap(sync("window_set_cursor_grab", { id: this.id, grab }));
  }

  /**
   * Modifies the cursor's visibility.
   * 
   * If `false`, this will hide the cursor. If `true`, this will show the cursor.
   */
  setCursorVisible(visible: boolean): void {
    unwrap(sync("window_set_cursor_visible", { id: this.id, visible }));
  }

  /**
   * Renders the current pixels buffer as set by `drawFrame`.
   */
  renderFrame(): void {
    unwrap(sync("window_render_frame", { id: this.id }));
  }

  /**
   * Sets the current pixels buffer to whatever is provided.
   * 
   * Buf needs to be of size `width * height * 4` and in raw
   * RGBA value.
   */
  drawFrame(buf: Uint8Array): void {
    unwrap(sync("window_draw_frame", { id: this.id }, buf));
  }

  /** 
   * Resize the surface upon which the pixel buffer is rendered.
   * 
   * This does not resize the pixel buffer. The pixel buffer will be fit onto the
   * surface as best as possible by scaling to the nearest integer, e.g. 2x, 3x,
   * 4x, etc.
   * 
   * Call this method in response to a `resize`. The size expected is in physical
   * pixel units.
   */
  resizeFrame(width: number, height: number): void {
    unwrap(sync("window_resize_frame", { id: this.id, width, height }));
  }

  /** Returns the current pixels buffer stored in this pane window */
  viewFrame(): Uint8Array {
    return syncRaw("window_view_frame", { id: this.id });
  }
}
