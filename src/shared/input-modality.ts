// Radix focuses tabs and select triggers by script on mousedown. Chromium gives script focus `:focus-visible` until the
// first native mouse focus, so after the window is focused from the tray a clicked tab shows a ring. `focus-ring` reads this.
const root = document.documentElement;
addEventListener("pointerdown", () => (root.dataset.pointer = ""), true);
addEventListener("keydown", () => delete root.dataset.pointer, true);
