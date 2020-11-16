export type Result<T> = { err: string } | { ok: T };

export type Id = bigint;

export type Position = { physical: PhysicalPosition } | {
  logical: LogicalPosition;
};
export type PhysicalPosition = { x: number; y: number };
export type LogicalPosition = { x: number; y: number };

export type Size = { physical: PhysicalSize } | { logical: LogicalSize };
export type PhysicalSize = { width: number; height: number };
export type LogicalSize = { width: number; height: number };

export type CursorIcon = unknown;

export type Event =
  | { type: "newEvents"; value: StartCause }
  | { type: "windowEvent"; value: { windowId: Id; event: WindowEvent } }
  | { type: "deviceEvent"; value: { deviceId: Id; event: DeviceEvent } }
  | { type: "userEvent" }
  | { type: "suspended" }
  | { type: "resumed" }
  | { type: "mainEventsCleared" }
  | { type: "redrawRequested" }
  | { type: "redrawEventsCleared" }
  | { type: "loopDestroyed" };

export type StartCause =
  | {
    type: "resumeTimeReached";
    value: { start: number; requestedResume: number };
  }
  | {
    type: "waitCancelled";
    value: { start: number; requestedResume?: number };
  }
  | { type: "poll" }
  | { type: "init" };

export type WindowEvent =
  | { type: "resized"; value: PhysicalSize }
  | { type: "moved"; value: PhysicalPosition }
  | { type: "closeRequested" }
  | { type: "destroyed" }
  | { type: "droppedFile"; value: string }
  | { type: "hoveredFile"; value: string }
  | { type: "hoveredFileCancelled" }
  | { type: "receivedCharacter"; value: string }
  | { type: "focused"; value: boolean }
  | {
    type: "keyboardInput";
    value: { deviceId: Id; input: KeyboardInput; isSynthetic: boolean };
  }
  | { type: "modifiersChanged"; value: ModifiersState }
  | {
    type: "cursorMoved";
    value: { deviceId: Id; position: PhysicalPosition };
  }
  | { type: "cursorEntered"; value: { deviceId: Id } }
  | { type: "cursorLeft"; value: { deviceId: Id } }
  | {
    type: "mouseWheel";
    value: { deviceId: Id; delta: MouseScrollDelta; phase: TouchPhase };
  }
  | {
    type: "mouseInput";
    value: { deviceId: Id; state: ElementState; button: MouseButton };
  }
  | {
    type: "touchpadPressure";
    value: { deviceId: Id; pressure: number; stage: number };
  }
  | {
    type: "axisMotion";
    value: { deviceId: Id; axis: AxisId; value: number };
  }
  | { type: "touch"; value: Touch }
  | {
    type: "scaleFactorChanged";
    value: { scaleFactor: number; newInnerSize: PhysicalSize };
  }
  | { type: "ThemeChanged"; value: Theme };

export type ScanCode = number;
export type AxisId = number;
export type ButtonId = number;

export type Theme = "light" | "dark";

export type KeyboardInput = {
  scancode: ScanCode;
  state: ElementState;
  virtualKeycode?: VirtualKeyCode;
};

export type VirtualKeyCode =
  | "Key1"
  | "Key2"
  | "Key3"
  | "Key4"
  | "Key5"
  | "Key6"
  | "Key7"
  | "Key8"
  | "Key9"
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
  | "Snapshot"
  | "Scroll"
  | "Pause"
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
  | "Back"
  | "Return"
  | "Space"
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

export type ElementState = "pressed" | "released";

export type MouseButton = "left" | "right" | "middle" | { other: number };

export type MouseScrollDelta =
  | { type: "lineDelta"; value: [number, number] }
  | { type: "pixelDelta"; value: PhysicalPosition };

export type TouchPhase = "started" | "moved" | "ended" | "cancelled";

export type Touch = {
  deviceId: Id;
  phase: TouchPhase;
  location: PhysicalPosition;
  force?: Force;
  id: number;
};

export type Force =
  | {
    type: "calibrated";
    value: {
      force: number;
      maxPossibleForce: number;
      altitudeAngle?: number;
    };
  }
  | { type: "normalized"; value: number };

export type ModifiersState = {
  shift: boolean;
  ctrl: boolean;
  alt: boolean;
  logo: boolean;
};

export type DeviceEvent =
  | { type: "added" }
  | { type: "removed" }
  | { type: "mouseMotion"; value: { delta: [number, number] } }
  | { type: "mouseWheel"; value: { delta: MouseScrollDelta } }
  | { type: "motion"; value: { axis: AxisId; value: number } }
  | { type: "button"; value: { button: ButtonId; state: ElementState } }
  | { type: "key"; value: KeyboardInput }
  | { type: "text"; value: { codepoint: string } };
