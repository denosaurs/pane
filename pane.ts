import { Plug } from "./deps.ts";
import {
  CursorIcon,
  PaneEvent,
  PhysicalPosition,
  PhysicalSize,
  Position,
  Size,
  UserAttentionType,
} from "./types.ts";

/**
 * Represents a winit event loop.
 */
export class PaneEventLoop {
  readonly rid: number;

  constructor() {
    this.rid = Plug.core.opSync("pane_event_loop_new");
  }

  /** Takes a step in this event loop, returning an array of `PaneEvent`s. */
  step(): PaneEvent[] {
    return Plug.core.opSync("pane_event_loop_step", this.rid);
  }
}

/**
 * Represents a window.
 */
export class PaneWindow {
  readonly rid: number;
  /**
   * This pane windows unique raw window handle.
   *
   * The main use of this is to be passed to Deno's WebGPU instance when creating
   * a surface.
   */
  readonly rawWindowHandleRid: number;

  /** This pane windows unique id. */
  get id(): number {
    return Plug.core.opSync("pane_window_id", this.rid);
  }

  constructor(eventLoop: PaneEventLoop) {
    [this.rawWindowHandleRid, this.rid] = Plug.core.opSync(
      "pane_window_new",
      eventLoop.rid,
    );
  }

  /**
   * Returns the scale factor that can be used to map logical pixels to physical
   * pixels, and vice versa.
   */
  scaleFactor(): number {
    return Plug.core.opSync("pane_window_scale_factor", this.rid);
  }

  /**
   * Emits a `redrawRequested` event in the event loop after all OS events have
   * been processed by the event loop.
   *
   * This is the strongly encouraged method of redrawing windows, as it can integrate
   * with OS-requested redraws (e.g. when a window gets resized).
   */
  requestRedraw(): void {
    Plug.core.opSync("pane_window_request_redraw", this.rid);
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
    return Plug.core.opSync("pane_window_inner_position", this.rid);
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
    return Plug.core.opSync("pane_window_outer_position", this.rid);
  }

  /** Modifies the position of the window. */
  setOuterPosition(position: Position): void {
    Plug.core.opSync("pane_window_set_outer_position", {
      rid: this.rid,
      position,
    });
  }

  /**
   * Returns the physical size of the window's client area.
   *
   * The client area is the content of the window, excluding the title bar and borders.
   */
  innerSize(): PhysicalSize {
    return Plug.core.opSync("pane_window_inner_size", this.rid);
  }

  /**
   * Modifies the inner size of the window.
   *
   * See `innerSize` for more information about the values. This automatically
   * un-maximizes the window if it's maximized.
   */
  setInnerSize(size: Size): void {
    Plug.core.opSync("pane_window_set_inner_size", { rid: this.rid, size });
  }

  /**
   * Returns the physical size of the entire window.
   *
   * These dimensions include the title bar and borders. If you don't want that
   * (and you usually don't), use `innerSize` instead.
   */
  outerSize(): PhysicalSize {
    return Plug.core.opSync("pane_window_outer_size", this.rid);
  }

  /** Sets a minimum dimension size for the window. */
  setMinInnerSize(size?: Size): void {
    Plug.core.opSync("pane_window_set_min_inner_size", { rid: this.rid, size });
  }

  /** Sets a maximum dimension size for the window. */
  setMaxInnerSize(size?: Size): void {
    Plug.core.opSync("pane_window_set_max_inner_size", { rid: this.rid, size });
  }

  /** Modifies the title of the window. */
  setTitle(title: string): void {
    Plug.core.opSync("pane_window_set_title", { rid: this.rid, title });
  }

  /**
   * Modifies the window's visibility.
   *
   * If `false`, this will hide the window. If `true`, this will show the window.
   */
  setVisible(visible: boolean): void {
    Plug.core.opSync("pane_window_set_visible", { rid: this.rid, visible });
  }

  /**
   * Sets whether the window is resizable or not.
   *
   * Note that making the window unresizable doesn't exempt you from handling
   * `resized`, as that event can still be triggered by DPI scaling, entering
   * fullscreen mode, etc.
   */
  setResizable(resizable: boolean): void {
    Plug.core.opSync("pane_window_set_resizable", { rid: this.rid, resizable });
  }

  /** Sets the window to minimized or back. */
  setMinimized(minimized: boolean): void {
    Plug.core.opSync("pane_window_set_minimized", { rid: this.rid, minimized });
  }

  /** Sets the window to maximized or back. */
  setMaximized(maximized: boolean): void {
    Plug.core.opSync("pane_window_set_maximized", { rid: this.rid, maximized });
  }

  /** Gets the window’s current maximized state. */
  isMaximized(): boolean {
    return Plug.core.opSync("pane_window_is_maximized", this.rid);
  }

  /** Turn window decorations on or off. */
  setDecorations(decorations: boolean): void {
    Plug.core.opSync("pane_window_set_decorations", {
      rid: this.rid,
      decorations,
    });
  }

  /** Change whether or not the window will always be on top of other windows. */
  setAlwaysOnTop(alwaysOnTop: boolean): void {
    Plug.core.opSync("pane_window_set_always_on_top", {
      rid: this.rid,
      alwaysOnTop,
    });
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
    Plug.core.opSync("pane_window_set_window_icon", {
      rid: this.rid,
      rgba,
      width,
      height,
    });
  }

  /**
   * Sets location of IME candidate box in client area coordinates relative to
   * the top left.
   */
  setImePosition(position: Position): void {
    Plug.core.opSync("pane_window_set_ime_position", {
      rid: this.rid,
      position,
    });
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
    Plug.core.opSync("pane_window_request_user_attention", {
      rid: this.rid,
      requestType,
    });
  }

  /** Modifies the cursor icon of the window. */
  setCursorIcon(cursor: CursorIcon): void {
    Plug.core.opSync("window_set_cursor_icon", { rid: this.rid, cursor });
  }

  /** Changes the position of the cursor in window coordinates. */
  setCursorPosition(position: Position): void {
    Plug.core.opSync("pane_window_set_cursor_position", {
      rid: this.rid,
      position,
    });
  }

  /**
   * Grabs the cursor, preventing it from leaving the window.
   *
   * There's no guarantee that the cursor will be hidden. You should hide it by
   * yourself if you want so.
   */
  setCursorGrab(grab: boolean): void {
    Plug.core.opSync("pane_window_set_cursor_grab", { rid: this.rid, grab });
  }

  /**
   * Modifies the cursor's visibility.
   *
   * If `false`, this will hide the cursor. If `true`, this will show the cursor.
   */
  setCursorVisible(visible: boolean): void {
    Plug.core.opSync("pane_window_set_cursor_visible", {
      rid: this.rid,
      visible,
    });
  }

  /**
   * Moves the window with the left mouse button until the button is released.
   *
   * There’s no guarantee that this will work unless the left mouse button was
   * pressed immediately before this function is called.
   */
  dragWindow(): void {
    Plug.core.opSync("pane_window_set_cursor_visible", this.rid);
  }
}
