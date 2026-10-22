# Blaze Kinetic Glass (BCDL)
### The Anti-AI-Slop Design Language for Sovereign Operating Systems

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Color Space: OKLCH](https://img.shields.io/badge/Color_Space-OKLCH_Perceptual-85d79e.svg)](#perceptual-color-tokens)
[![Platform: Wayland / Niri](https://img.shields.io/badge/Platform-Wayland%20%2F%20Layer--Shell-a9c7ff.svg)](#wayland-layer-shell-integration)
[![Zero AI Slop](https://img.shields.io/badge/Aesthetic-Anti--AI--Slop-e5be6b.svg)](#the-anti-ai-slop-manifesto)

**Blaze Kinetic Glass (BCDL)** is a disciplined, high-performance design language engineered specifically for **BlazeOS**, Wayland compositors (such as [Niri](https://github.com/YaLTeR/niri)), and low-latency system HUDs.

It bridges the gap between the structural rigor of Google's **Material 3 Expressive**, the optical elegance of Apple's **Liquid Glass**, and the mathematical determinism of the **OKLCH** color space — while strictly outlawing the visual clichés, glowing neon halos, and fake animations common to "AI-generated" UI concepts.

---

## 🎯 The Anti-AI-Slop Manifesto

Most modern Linux desktop rice concepts fall into one of two traps:
1. **The RGB Gamer Trap:** Oversaturated cyan/magenta neons, pulsating drop shadows, and gaudy glowing borders that ruin dark-room ergonomics.
2. **The Superficial macOS Clone Trap:** Heavy Gaussian blur applied across the entire screen, rendering code editors and terminal text illegible while consuming 40% of the GPU compositing budget.

**BCDL rejects both.**

* **No Emojis in System Chrome:** System telemetry must communicate authority and precision. Emojis (`⚡`, `🚀`, `🛡️`) are forbidden. Use geometric mineral dots (`●`, `○`) and monospaced typography.
* **Content is Sacred:** The captured or frozen window content (`/tmp/bctp_ghost_surface.jpg`) is **never** blurred. It is dimmed by only 10–15% using an optical scrim.
* **Restricted Glass:** Glass refraction and dual-Kawase backdrop blur are quarantined strictly to floating HUD pills and window borders.
* **Zero Layout-Shift:** Numerical readouts (Epoch, Latencies, Input Queues) strictly use `tabular-nums` (JetBrains Mono). Rakamlar akarken hiçbir arayüz kutusu titremez.
* **Deterministic Opacity Layers:** State changes (hover, active, selected) alter luminance and alpha steps (`+4%`, `+8%`, `+12%`), never introducing rainbow hues.

---

## 🏛️ The Four Architectural Pillars

```
┌────────────────────────────────────────────────────────────────────────┐
│                        BCDL: KINETIC GLASS                             │
├────────────────────────────────────────────────────────────────────────┤
│  [1] STRUCTURAL SKELETON      -->  Material 3 Expressive (8dp grid)    │
│  [2] OPTICAL RIGOR            -->  Apple Liquid Glass (1px Specular)   │
│  [3] COLOR MATHEMATICS        -->  Perceptual OKLCH (Chroma <= 0.015)  │
│  [4] TELEMETRY DISCIPLINE     -->  Swiss Typography (tabular-nums)     │
└────────────────────────────────────────────────────────────────────────┘
```

1. **Structural Skeleton (Material 3 Expressive):** Built on an 8dp spatial rhythm with deterministic state layers. Clear visual hierarchy without cartoonish roundness.
2. **Optical Rigor (Liquid Glass):** 1px directional specular refraction rim (top/left at 16% opacity, bottom/right at 5% opacity) simulating natural ambient top-down lighting.
3. **Color Mathematics (OKLCH):** Replaces legacy sRGB with the perceptually uniform OKLCH color space. Low-chroma slate neutrals guarantee WCAG AAA contrast across arbitrary OLED/IPS displays.
4. **Telemetry Discipline (Swiss Modernism):** High-frequency counters display instant system metrics with zero visual jitter.

---

## 🎨 Perceptual Color Tokens

All tokens are defined in **`oklch(Lightness Chroma Hue)`**:

```css
:root {
  /* Surfaces */
  --blz-surface-canvas:          oklch(0.12 0.010 255);          /* #0d0f12 */
  --blz-surface-glass-base:      oklch(0.16 0.012 255 / 0.72);   /* #121519b8 */
  --blz-surface-glass-elevated:  oklch(0.22 0.015 255 / 0.82);   /* #20252bd1 */
  --blz-surface-scrim:           oklch(0.10 0.008 255 / 0.45);   /* #08090b73 */

  /* 1px Asymmetric Specular Refractive Edges */
  --blz-border-specular-top:     oklch(0.95 0.005 255 / 0.16);
  --blz-border-specular-bottom:  oklch(0.95 0.005 255 / 0.05);

  /* Low-Chroma Mineral Accents */
  --blz-accent-mineral:          oklch(0.82 0.035 255);          /* Cornflower Slate */
  --blz-status-syncing:          oklch(0.78 0.050 75);           /* Mineral Amber */
  --blz-status-stable:           oklch(0.80 0.045 145);          /* Celadon Green */
  --blz-status-error:            oklch(0.68 0.065 25);           /* Brick Crimson */

  /* Typography */
  --blz-text-primary:            oklch(0.96 0.004 255);          /* Pure Off-White */
  --blz-text-secondary:          oklch(0.74 0.010 255);          /* Muted Silver */
  --blz-text-muted:              oklch(0.54 0.012 255);          /* Deep Slate */

  /* Typography Stacks */
  --blz-font-sans:               "Inter Display", -apple-system, system-ui, sans-serif;
  --blz-font-mono:               "JetBrains Mono", "Fira Code", monospace;
}
```

---

## 🪟 Component Spotlight: The Ghost Surface Overlay

During session teleportation via **Blaze Continuum (BCTP/1)**, the destination machine renders a transient Ghost Surface:

```text
┌────────────────────────────────────────────────────────────────────────┐
│ [Window Frame - Specular 1px Hairline Rim]                             │
│                                                                        │
│   Active application buffer (/tmp/bctp_ghost_surface.jpg)              │
│   Crisp & readable · 15% Dimmed Scrim · Zero Gaussian Mush             │
│                                                                        │
│                                                                        │
│                                       ┌──────────────────────────────┐ │
│                                       │ [ ● SYNCING ] EPOCH #02 · 1.2ms · +3 INP │ │
│                                       └──────────────────────────────┘ │
└────────────────────────────────────────────────────────────────────────┘
```

### The Floating Telemetry Pill HUD
```css
.blz-telemetry-pill {
  position: absolute;
  right: 16px;
  bottom: 16px;
  height: 32px;
  padding: 0 12px;
  display: inline-flex;
  align-items: center;
  gap: 8px;

  background: var(--blz-surface-glass-elevated);
  backdrop-filter: blur(10px) saturate(92%);
  -webkit-backdrop-filter: blur(10px) saturate(92%);

  border-radius: 9999px;
  border: 1px solid var(--blz-border-specular-top);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.35);

  font-family: var(--blz-font-mono);
  font-size: 11px;
  font-weight: 500;
  font-variant-numeric: tabular-nums;
  color: var(--blz-text-primary);
  user-select: none;
}

.blz-dot.syncing {
  color: var(--blz-status-syncing);
  animation: blz-pulse 1.8s ease-in-out infinite;
}

.blz-dot.stable {
  color: var(--blz-status-stable);
}
```

---

## 🦀 Rust Integration

The design tokens are directly embeddable in Rust using `ColorOklch` / `ColorRgba` structures:

```rust
use blaze_continuum::ui::tokens::*;

let border = BCDL_TOKENS.border_specular_top;
let pill_bg = BCDL_TOKENS.surface_glass_elevated;
println!("Rendering HUD with background: rgba({}, {}, {}, {})", 
    pill_bg.r, pill_bg.g, pill_bg.b, pill_bg.a);
```

---

## 📦 Repository Structure

```
blaze-kinetic-glass/
├── README.md             # This document
├── DESIGN_RULES.md       # Rigorous engineering specifications & physics
├── SKILL.md              # Reusable agent skill for AI pair programmers
├── tokens.css            # Standard CSS custom properties
├── tokens.json           # Design Tokens format (W3C standard)
└── src/
    └── tokens.rs         # Native Rust bindings & color math
```

---

## 📜 License

Licensed under the [MIT License](LICENSE).  
Engineered with sovereign pride by the **BlazeOS Core Team**.
