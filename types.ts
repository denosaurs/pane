/** A position that's either physical or logical. */
export type Position = { physical: PhysicalPosition } | {
  logical: LogicalPosition;
};
/**	A position represented in physical pixels. */
export type PhysicalPosition = { x: number; y: number };
/** A position represented in logical pixels. */
export type LogicalPosition = { x: number; y: number };

/** A size that's either physical or logical. */
export type Size = { physical: PhysicalSize } | { logical: LogicalSize };
/**	A size represented in physical pixels. */
export type PhysicalSize = { width: number; height: number };
/** A size represented in logical pixels. */
export type LogicalSize = { width: number; height: number };

/** Describes the appearance of the mouse cursor. */
export type CursorIcon =
  /** The platform-dependent default cursor. */
  | "default"
  /** A simple crosshair. */
  | "crosshair"
  /** A hand (often used to indicate links in web browsers). */
  | "hand"
  /** Self explanatory. */
  | "arrow"
  /** Indicates something is to be moved. */
  | "move"
  /** Indicates text that may be selected or edited. */
  | "text"
  /** Program busy indicator. */
  | "wait"
  /** Help indicator (often rendered as a "?") */
  | "help"
  /**
   * Progress indicator. Shows that processing is being done. But in contrast
   * with "Wait" the user may still interact with the program. Often rendered
   * as a spinning beach ball, or an arrow with a watch or hourglass.
   */
  | "progress"
  /** Cursor showing that something cannot be done. */
  | "notAllowed"
  | "contextMenu"
  | "cell"
  | "verticalText"
  | "alias"
  | "copy"
  | "noDrop"
  /** Indicates something can be grabbed. */
  | "grab"
  /** Indicates something is grabbed. */
  | "grabbing"
  | "allScroll"
  | "zoomIn"
  | "zoomOut"
  | "eResize"
  | "nResize"
  | "neResize"
  | "nwResize"
  | "sResize"
  | "seResize"
  | "swResize"
  | "wResize"
  | "ewResize"
  | "nsResize"
  | "neswResize"
  | "nwseResize"
  | "colResize"
  | "rowResize";

/** Describes a generic event. */
export type PaneEvent =
  | {
    /** Emitted when new events arrive from the OS to be processed. */
    type: "newEvents";
    value: StartCause;
  }
  | {
    /** Emitted when the OS sends an event to a winit window. */
    type: "windowEvent";
    value: { windowId: number; event: WindowEvent };
  }
  | {
    /** Emitted when the OS sends an event to a device. */
    type: "deviceEvent";
    value: { deviceId: number; event: DeviceEvent };
  }
  | {
    /** Unused in pane. */
    type: "userEvent";
  }
  | {
    /** Emitted when the application has been suspended. */
    type: "suspended";
  }
  | {
    /** Emitted when the application has been resumed. */
    type: "resumed";
  }
  | {
    /**
     * Emitted when all of the event loop's input events have been processed and
     * redraw processing is about to begin.
     */
    type: "mainEventsCleared";
  }
  | {
    /** Emitted after `mainEventsCleared` when a window should be redrawn. */
    type: "redrawRequested";
  }
  | {
    /**
     * Emitted after all `redrawRequested` events have been processed and control
     * flow is about to be taken away from the program. If there are no `redrawRequested`
     * events, it is emitted immediately after `mainEventsCleared`.
     */
    type: "redrawEventsCleared";
  }
  | {
    /**
     * Emitted when the event loop is being shut down. Beware! This event is emitted
     * every step in the event loop as the last thing that happens each step.
     */
    type: "loopDestroyed";
  };

/** Describes the reason the event loop is resuming. */
export type StartCause =
  /** Unused in pane. */
  | {
    type: "resumeTimeReached";
    value: { start: number; requestedResume: number };
  }
  /**
   * Sent if the OS has new events to send to the window, after a wait was requested.
   * Contains the moment the wait was requested and the resume time, if requested.
   */
  | {
    type: "waitCancelled";
    value: { start: number; requestedResume?: number };
  }
  /** Unused in pane. */
  | {
    type: "poll";
  }
  /** Emitted every step in the event loop as the first event. */
  | {
    type: "init";
  };

/** Describes an event from a `Pane` window. */
export type WindowEvent =
  | {
    /** The size of the window has changed. Contains the client area's new dimensions. */
    type: "resized";
    value: PhysicalSize;
  }
  | {
    /** The position of the window has changed. Contains the window's new position. */
    type: "moved";
    value: PhysicalPosition;
  }
  | {
    /** The window has been requested to close. */
    type: "closeRequested";
  }
  | {
    /** The window has been destroyed. */
    type: "destroyed";
  }
  | {
    /** A file has been dropped into the window. */
    type: "droppedFile";
    value: string;
  }
  | {
    /** A file is being hovered over the window. */
    type: "hoveredFile";
    value: string;
  }
  | {
    /**  A file was hovered, but has exited the window. */
    type: "hoveredFileCancelled";
  }
  | {
    /** The window received a unicode character. */
    type: "receivedCharacter";
    value: string;
  }
  | {
    /**
     * The window gained or lost focus. The parameter is true if the window has gained
     * focus, and false if it has lost focus.
     */
    type: "focused";
    value: boolean;
  }
  | {
    /**
     * An event from the keyboard has been received.
     */
    type: "keyboardInput";
    value: {
      deviceId: number;
      input: KeyboardInput;
      /**
      * If `true`, the event was generated synthetically by winit
      * in one of the following circumstances:
      *
      * * Synthetic key press events are generated for all keys pressed
      *   when a window gains focus. Likewise, synthetic key release events
      *   are generated for all keys pressed when a window goes out of focus.
      *   ***Currently, this is only functional on X11 and Windows***
      *
      * Otherwise, this value is always `false`.
      */
      isSynthetic: boolean;
    };
  }
  | {
    /** The keyboard modifiers have changed. */
    type: "modifiersChanged";
    value: ModifiersState;
  }
  | {
    /** The cursor has moved on the window. */
    type: "cursorMoved";
    value: { deviceId: number; position: PhysicalPosition };
  }
  | {
    /**  The cursor has entered the window. */
    type: "cursorEntered";
    value: { deviceId: number };
  }
  | {
    /** The cursor has left the window. */
    type: "cursorLeft";
    value: { deviceId: number };
  }
  | {
    /** A mouse wheel movement or touchpad scroll occurred. */
    type: "mouseWheel";
    value: { deviceId: number; delta: MouseScrollDelta; phase: TouchPhase };
  }
  | {
    /** An mouse button press has been received. */
    type: "mouseInput";
    value: { deviceId: number; state: ElementState; button: MouseButton };
  }
  | {
    /** Touchpad pressure event.
     *
     * At the moment, only supported on Apple forcetouch-capable macbooks.
     */
    type: "touchpadPressure";
    value: {
      deviceId: number;
      /** A value between 0 and 1 representing how hard the touchpad is being pressed. */
      pressure: number;
      /** An integer representing the click level */
      stage: number;
    };
  }
  | {
    /**
     * Motion on some analog axis. May report data redundant to other, more specific
     * events.
     */
    type: "axisMotion";
    value: { deviceId: number; axis: AxisId; value: number };
  }
  | {
    /** Touch event has been received */
    type: "touch";
    value: Touch;
  }
  | {
    /**
     * The window's scale factor has changed.
     *
     * The following user actions can cause DPI changes:
     * * Changing the display's resolution.
     * * Changing the display's scale factor (e.g. in Control Panel on Windows).
     * * Moving the window to a display with a different scale factor.
    */
    type: "scaleFactorChanged";
    value: { scaleFactor: number; newInnerSize: PhysicalSize };
  }
  | {
    /**
     * The system window theme has changed.
     *
     * Applications might wish to react to this to change the theme of the content of the window
     * when the system changes the window theme.
     *
     * At the moment this is only supported on Windows.
    */
    type: "themeChanged";
    value: Theme;
  };

/** Hardware-dependent keyboard scan code. */
export type ScanCode = number;
/** Identifier for a specific analog axis on some device. */
export type AxisId = number;
/** Identifier for a specific button on some device. */
export type ButtonId = number;

/** The os theme. */
export type Theme = "light" | "dark";

/** Describes a keyboard input event. */
export type KeyboardInput = {
  /** Identifies the physical key pressed. */
  scancode: ScanCode;
  state: ElementState;
  /** Identifies the semantic meaning of the key. */
  virtualKeycode?: VirtualKeyCode;
};

export type VirtualKeyCode =
  /** The "1" key over the letters. */
  | "Key1"
  /** The "2" key over the letters. */
  | "Key2"
  /** The "3" key over the letters. */
  | "Key3"
  /** The "4" key over the letters. */
  | "Key4"
  /** The "5" key over the letters. */
  | "Key5"
  /** The "6" key over the letters. */
  | "Key6"
  /** The "7" key over the letters. */
  | "Key7"
  /** The "8" key over the letters. */
  | "Key8"
  /** The "9" key over the letters. */
  | "Key9"
  /** The "0" key over the "O" and "P" keys. */
  | "Key0"
  | "A"
  | "B"
  | "C"
  | "D"
  | "E"
  | "F"
  | "G"
  | "H"
  | "I"
  | "J"
  | "K"
  | "L"
  | "M"
  | "N"
  | "O"
  | "P"
  | "Q"
  | "R"
  | "S"
  | "T"
  | "U"
  | "V"
  | "W"
  | "X"
  | "Y"
  | "Z"
  /** The Escape key, next to F1. */
  | "Escape"
  | "F1"
  | "F2"
  | "F3"
  | "F4"
  | "F5"
  | "F6"
  | "F7"
  | "F8"
  | "F9"
  | "F10"
  | "F11"
  | "F12"
  | "F13"
  | "F14"
  | "F15"
  | "F16"
  | "F17"
  | "F18"
  | "F19"
  | "F20"
  | "F21"
  | "F22"
  | "F23"
  | "F24"
  /** Print Screen/SysRq. */
  | "Snapshot"
  /** Scroll Lock. */
  | "Scroll"
  /** Pause/Break key, next to Scroll lock. */
  | "Pause"
  /** Insert, next to Backspace. */
  | "Insert"
  | "Home"
  | "Delete"
  | "End"
  | "PageDown"
  | "PageUp"
  | "Left"
  | "Up"
  | "Right"
  | "Down"
  /** The Backspace key, right over Enter. */
  | "Back"
  /** The Enter key. */
  | "Return"
  /** The space bar. */
  | "Space"
  /** The "Compose" key on Linux. */
  | "Compose"
  | "Caret"
  | "Numlock"
  | "Numpad0"
  | "Numpad1"
  | "Numpad2"
  | "Numpad3"
  | "Numpad4"
  | "Numpad5"
  | "Numpad6"
  | "Numpad7"
  | "Numpad8"
  | "Numpad9"
  | "NumpadAdd"
  | "NumpadDivide"
  | "NumpadDecimal"
  | "NumpadComma"
  | "NumpadEnter"
  | "NumpadEquals"
  | "NumpadMultiply"
  | "NumpadSubtract"
  | "AbntC1"
  | "AbntC2"
  | "Apostrophe"
  | "Apps"
  | "Asterisk"
  | "At"
  | "Ax"
  | "Backslash"
  | "Calculator"
  | "Capital"
  | "Colon"
  | "Comma"
  | "Convert"
  | "Equals"
  | "Grave"
  | "Kana"
  | "Kanji"
  | "LAlt"
  | "LBracket"
  | "LControl"
  | "LShift"
  | "LWin"
  | "Mail"
  | "MediaSelect"
  | "MediaStop"
  | "Minus"
  | "Mute"
  | "MyComputer"
  | "NavigateForward"
  | "NavigateBackward"
  | "NextTrack"
  | "NoConvert"
  | "OEM102"
  | "Period"
  | "PlayPause"
  | "Plus"
  | "Power"
  | "PrevTrack"
  | "RAlt"
  | "RBracket"
  | "RControl"
  | "RShift"
  | "RWin"
  | "Semicolon"
  | "Slash"
  | "Sleep"
  | "Stop"
  | "Sysrq"
  | "Tab"
  | "Underline"
  | "Unlabeled"
  | "VolumeDown"
  | "VolumeUp"
  | "Wake"
  | "WebBack"
  | "WebFavorites"
  | "WebForward"
  | "WebHome"
  | "WebRefresh"
  | "WebSearch"
  | "WebStop"
  | "Yen"
  | "Copy"
  | "Paste"
  | "Cut";

/** Describes the input state of a key. */
export type ElementState = "pressed" | "released";

/** Describes a button of a mouse controller. */
export type MouseButton = "left" | "right" | "middle" | { other: number };

/** Describes a difference in the mouse scroll wheel state. */
export type MouseScrollDelta =
  | {
    type: "lineDelta";
    /**
     * Amount in lines or rows to scroll in the horizontal and vertical directions.
     *
     * Positive values indicate movement forward (away from the user) or rightwards.
     */
    value: [number, number];
  }
  | {
    type: "pixelDelta";
    /**
     * Amount in pixels to scroll in the horizontal and vertical direction.
     *
     * Scroll events are expressed as a PixelDelta if supported by the device
     * (eg. a touchpad) and platform.
     */
    value: PhysicalPosition;
  };

/** Describes touch-screen input state. */
export type TouchPhase = "started" | "moved" | "ended" | "cancelled";

export type UserAttentionType =
  /**
   * * macOS: Bounces the dock icon until the application is in focus.
   * * Windows: Flashes both the window and the taskbar button until the application is in focus.
   */
  | "critical"
  /**
   * * macOS: Bounces the dock icon once.
   * * Windows: Flashes the taskbar button until the application is in focus.
   */
  | "informational";

/**
 * Represents a touch event.
 *
 * Every time the user touches the screen, a new `started` event with an unique
 * identifier for the finger is generated. When the finger is lifted, an
 * `ended` event is generated with the same finger id.
 *
 * After a `started` event has been emitted, there may be zero or more `moved`
 * events when the finger is moved or the touch pressure changes.
 *
 * The finger id may be reused by the system after an `ended` event. The user should
 * assume that a new `started` event received with the same id has nothing to do
 * with the old finger and is a new finger.
 *
 * A `cancelled` event is emitted when the system has canceled tracking this touch,
 * such as when the window loses focus.
 */
export type Touch = {
  deviceId: number;
  phase: TouchPhase;
  location: PhysicalPosition;
  /**
   * Describes how hard the screen was pressed. May be `unknown` if the platform
   * does not support pressure sensitivity.
   */
  force?: Force;
  id: bigint;
};

/** Describes the force of a touch event. */
export type Force =
  | {
    /** Currently unused in pane as this is an ios specific event. */
    type: "calibrated";
    value: {
      force: number;
      maxPossibleForce: number;
      altitudeAngle?: number;
    };
  }
  | {
    /**
     * If the platform reports the force as normalized, we have no way of
     * knowing how much pressure 1.0 corresponds to – we know it's the maximum
     * amount of force, but as to how much force, you might either have to
     * press really really hard, or not hard at all, depending on the device.
     */
    type: "normalized";
    value: number;
  };

/** Represents the current state of the keyboard modifiers. */
export type ModifiersState = {
  shift: boolean;
  ctrl: boolean;
  alt: boolean;
  logo: boolean;
};

/**
 * Represents raw hardware events that are not associated with any particular window.
 *
 * Useful for interactions that diverge significantly from a conventional 2D GUI,
 * such as 3D camera or first-person game controls. Many physical actions, such
 * as mouse movement, can produce both device and window events. Because window
 * events typically arise from virtual devices (corresponding to GUI cursors and
 * keyboard focus) the device IDs may not match.
 *
 * Note that these events are delivered regardless of input focus.
 */
export type DeviceEvent =
  | { type: "added" }
  | { type: "removed" }
  | {
    /**
     * Change in physical position of a pointing device.
     *
     * This represents raw, unfiltered physical motion. Not to be confused with
     * the `WindowEvent`.
     */
    type: "mouseMotion";
    value: {
      /**
       * [x, y] change in position in unspecified units.
       *
       * Different devices may use different units.
       */
      delta: [number, number];
    };
  }
  | {
    /** Physical scroll event. */
    type: "mouseWheel";
    value: { delta: MouseScrollDelta };
  }
  | {
    /**
     * Motion on some analog axis. This event will be reported for all arbitrary
     * input devices that winit supports on this platform, including mouse devices.
     * If the device is a mouse device then this will be reported alongside the
     * `mouseMotion` event.
     */
    type: "motion";
    value: { axis: AxisId; value: number };
  }
  | { type: "button"; value: { button: ButtonId; state: ElementState } }
  | { type: "key"; value: KeyboardInput }
  | { type: "text"; value: { codepoint: string } };
