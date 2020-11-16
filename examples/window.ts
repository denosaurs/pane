import { Window } from "../mod.ts";

const window = new Window();

console.log(window.scaleFactor());
console.log(window.requestRedraw());
console.log(window.innerPosition());
console.log(window.outerPosition());
console.log(window.setOuterPosition({ physical: { x: 10, y: 10 } }));
console.log(window.setOuterPosition({ logical: { x: 10, y: 10 } }));
console.log(window.innerSize());
console.log(window.setInnerSize({ physical: { width: 10, height: 10 } }));
console.log(window.setInnerSize({ logical: { width: 10, height: 10 } }));
console.log(window.outerSize());
console.log(window.setMinInnerSize({ physical: { width: 10, height: 10 } }));
console.log(window.setMinInnerSize({ logical: { width: 10, height: 10 } }));
console.log(window.setMaxInnerSize({ physical: { width: 10, height: 10 } }));
console.log(window.setMaxInnerSize({ logical: { width: 10, height: 10 } }));
console.log(window.setTitle("Hello from deno!"));
console.log(window.setResizable(false));
console.log(window.setResizable(true));
console.log(window.setMaximized(false));
console.log(window.setMaximized(true));
console.log(window.setMinimized(false));
console.log(window.setMinimized(true));
console.log(window.setDecorations(false));
console.log(window.setDecorations(true));
console.log(window.setAlwaysOnTop(false));
console.log(window.setAlwaysOnTop(true));
console.log(
  window.setWindowIcon(
    new Uint8Array(32 * 32 * 4).map((_) => Math.floor(Math.random() * 255)),
    32,
    32,
  ),
);
console.log(window.setImePosition({ physical: { x: 10, y: 10 } }));
console.log(window.setImePosition({ logical: { x: 10, y: 10 } }));
console.log(window.setCursorIcon("hand"));
console.log(window.setCursorPosition({ physical: { x: 10, y: 10 } }));
console.log(window.setCursorPosition({ logical: { x: 10, y: 10 } }));
console.log(window.setCursorGrab(false));
console.log(window.setCursorGrab(true));
console.log(window.setCursorVisible(false));
console.log(window.setCursorVisible(true));
