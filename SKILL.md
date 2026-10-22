---
name: blaze-kinetic-glass
description: Enforces the Blaze Continuum Design Language (BCDL: Kinetic Glass) across all Wayland overlays, GTK4/Slint interfaces, and web HUDs. Prevents AI-slop, bans emojis and saturated neon glows, enforces OKLCH color math, 1px asymmetric specular refraction, and tabular Swiss telemetry typography.
---

# Blaze Kinetic Glass (BCDL) Skill

This skill governs the visual and interaction architecture of **BlazeOS**, specifically designed for high-performance Wayland surfaces, `wlr-layer-shell` HUDs, and the Blaze Continuum Ghost Overlay.

## When to Activate
Activate this skill whenever:
1. Designing, styling, or implementing UI components for BlazeOS (CSS, GTK4, Slint, egui, or native Wayland clients).
2. Authoring shaders, blurs, or visual overlays for desktop session mobility.
3. Reviewing UI code for accessibility, perceptual contrast, or aesthetic integrity.

---

## The Inviolable Anti-AI-Slop Laws

Any code violating these rules must be rejected and corrected immediately:

1. **NO EMOJIS IN CHROME:** System UI must never contain emojis (`⚡`, `🚀`, `🛡️`, `✨`). Use typographic geometric dots (`●`, `○`), unicode arrows (`->`, `::`), or standard text.
2. **NO NEON / CYBERPUNK BLOOM:** Pure cyan (`#00FFFF`), magenta (`#FF00FF`), or neon purple glowing drop shadows are strictly prohibited.
3. **NEVER BLUR WINDOW CONTENTS:** When freezing or displaying a window replica (Ghost Surface), do not apply Gaussian blur to the window frame. Dim it by 10-15% using `--blz-surface-scrim`. Blur is quarantined strictly to floating HUD pills and controls.
4. **MANDATORY TABULAR NUMBERS:** All dynamic figures (PID, latency ms, epoch counter, queued inputs) MUST use `font-variant-numeric: tabular-nums` or a monospace font (JetBrains Mono) to prevent layout jitter (`layout-shift = 0`).
5. **DETERMINISTIC STATE OPACITY:** Do not introduce new colors on hover/active states. Add pure white overlay alpha steps (`+4%` hover, `+8%` active, `+12%` selected).

---

## Token Reference

### CSS / Web / GTK4
```css
:root {
  /* Surfaces */
  --blz-surface-canvas: oklch(0.12 0.010 255);          /* #0d0f12 */
  --blz-surface-glass-base: oklch(0.16 0.012 255 / 0.72); /* #121519b8 */
  --blz-surface-glass-elevated: oklch(0.22 0.015 255 / 0.82); /* #20252bd1 */
  --blz-surface-scrim: oklch(0.10 0.008 255 / 0.45);    /* #08090b73 */

  /* Specular Refractive Borders (1px Asymmetric) */
  --blz-border-specular-top: oklch(0.95 0.005 255 / 0.16);
  --blz-border-specular-bottom: oklch(0.95 0.005 255 / 0.05);

  /* Accents & Status (Low-Chroma Minerals) */
  --blz-accent-mineral: oklch(0.82 0.035 255);          /* Cornflower Slate */
  --blz-status-syncing: oklch(0.78 0.050 75);           /* Mineral Amber */
  --blz-status-stable: oklch(0.80 0.045 145);           /* Celadon Green */
  --blz-status-error: oklch(0.68 0.065 25);             /* Brick Crimson */

  /* Typography */
  --blz-text-primary: oklch(0.96 0.004 255);            /* Crisp Light */
  --blz-text-secondary: oklch(0.74 0.010 255);          /* Muted Silver */
  --blz-text-muted: oklch(0.54 0.012 255);              /* Slate Gray */

  --blz-font-sans: "Inter Display", system-ui, sans-serif;
  --blz-font-mono: "JetBrains Mono", monospace;
}
```

---

## Ghost Overlay Implementation Blueprint

When coding the Wayland Ghost Surface:
1. Render `/tmp/bctp_ghost_surface.jpg` onto the target window geometry with 12px border radius.
2. Render a 1px border using `linear-gradient(180deg, var(--blz-border-specular-top), var(--blz-border-specular-bottom))`.
3. In the bottom-right corner (16px margin), anchor the Telemetry Pill:
   ```html
   <div class="blz-telemetry-pill">
     <span class="blz-dot syncing">●</span>
     <span class="blz-status">SYNCING</span>
     <span class="blz-sep">·</span>
     <span class="blz-epoch">EPOCH #02</span>
     <span class="blz-sep">·</span>
     <span class="blz-metric">1.2ms</span>
     <span class="blz-sep">·</span>
     <span class="blz-queue">+3 INP</span>
   </div>
   ```
4. Listen for pointer clicks and key strokes; forward them to `InputReplayQueue`.
5. Upon process rehydrate signal, fade out the overlay over 120ms and unmap.
