import { Pane } from "../mod.ts";

const pane = new Pane();

console.log(pane.scaleFactor());
console.log(pane.requestRedraw());
console.log(pane.innerPosition());
console.log(pane.outerPosition());
console.log(pane.setOuterPosition({ physical: { x: 10, y: 10 } }));
console.log(pane.setOuterPosition({ logical: { x: 10, y: 10 } }));
console.log(pane.innerSize());
console.log(pane.setInnerSize({ physical: { width: 10, height: 10 } }));
console.log(pane.setInnerSize({ logical: { width: 10, height: 10 } }));
console.log(pane.outerSize());
console.log(pane.setMinInnerSize({ physical: { width: 10, height: 10 } }));
console.log(pane.setMinInnerSize({ logical: { width: 10, height: 10 } }));
console.log(pane.setMaxInnerSize({ physical: { width: 10, height: 10 } }));
console.log(pane.setMaxInnerSize({ logical: { width: 10, height: 10 } }));
console.log(pane.setTitle("Hello from deno!"));
console.log(pane.setVisible(false));
console.log(pane.setVisible(true));
console.log(pane.setResizable(false));
console.log(pane.setResizable(true));
console.log(pane.setMaximized(false));
console.log(pane.setMaximized(true));
console.log(pane.setMinimized(false));
console.log(pane.setMinimized(true));
console.log(pane.setDecorations(false));
console.log(pane.setDecorations(true));
console.log(pane.setAlwaysOnTop(false));
console.log(pane.setAlwaysOnTop(true));
console.log(
  pane.setWindowIcon(
    new Uint8Array(32 * 32 * 4).map((_) => Math.floor(Math.random() * 255)),
    32,
    32,
  ),
);
console.log(pane.setImePosition({ physical: { x: 10, y: 10 } }));
console.log(pane.setImePosition({ logical: { x: 10, y: 10 } }));
console.log(pane.requestUserAttention("critical"));
console.log(pane.requestUserAttention("informational"));
console.log(pane.setCursorIcon("hand"));
console.log(pane.setCursorPosition({ physical: { x: 10, y: 10 } }));
console.log(pane.setCursorPosition({ logical: { x: 10, y: 10 } }));
console.log(pane.setCursorGrab(false));
console.log(pane.setCursorGrab(true));
console.log(pane.setCursorVisible(false));
console.log(pane.setCursorVisible(true));

Deno.exit();
