//! Blaze Continuum Design Language (BCDL: Kinetic Glass)
//! Perceptual Design Tokens and Optical Physics for BlazeOS

use serde::{Deserialize, Serialize};

/// OKLCH Algoritmik Renk Gösterimi
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ColorOklch {
    pub l: f32, // Lightness (0.0 .. 1.0)
    pub c: f32, // Chroma (0.0 .. 0.4)
    pub h: f32, // Hue (0.0 .. 360.0 derece)
    pub a: f32, // Alpha (0.0 .. 1.0)
}

impl ColorOklch {
    pub const fn new(l: f32, c: f32, h: f32, a: f32) -> Self {
        Self { l, c, h, a }
    }

    pub fn to_css_string(&self) -> String {
        if (self.a - 1.0).abs() < f32::EPSILON {
            format!("oklch({:.2} {:.3} {:.1})", self.l, self.c, self.h)
        } else {
            format!("oklch({:.2} {:.3} {:.1} / {:.2})", self.l, self.c, self.h, self.a)
        }
    }
}

/// Standart RGBA Renk Tipi (Doğrudan Wayland/Cairo Buffer render için)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColorRgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ColorRgba {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
    }

    pub fn to_cairo_rgba(&self) -> (f64, f64, f64, f64) {
        (
            self.r as f64 / 255.0,
            self.g as f64 / 255.0,
            self.b as f64 / 255.0,
            self.a as f64 / 255.0,
        )
    }
}

/// BCDL Kinetic Glass Renk Sabitleri
pub struct BcdlColors;

impl BcdlColors {
    // Surfaces
    pub const SURFACE_CANVAS: ColorRgba = ColorRgba::new(13, 15, 18, 255);       // #0d0f12
    pub const SURFACE_GLASS_BASE: ColorRgba = ColorRgba::new(18, 21, 25, 184);   // #121519b8 (72%)
    pub const SURFACE_GLASS_ELEVATED: ColorRgba = ColorRgba::new(32, 37, 43, 209); // #20252bd1 (82%)
    pub const SURFACE_SCRIM: ColorRgba = ColorRgba::new(8, 9, 11, 115);          // #08090b73 (45%)

    // 1px Asimetrik Speküler Kenar Işığı
    pub const BORDER_SPECULAR_TOP: ColorRgba = ColorRgba::new(255, 255, 255, 41);   // #ffffff29 (16%)
    pub const BORDER_SPECULAR_BOTTOM: ColorRgba = ColorRgba::new(255, 255, 255, 13); // #ffffff0d (5%)

    // Düşük Kroma Doğal Mineral Vurgular (C <= 0.035 - Anti-Neon)
    pub const ACCENT_MINERAL: ColorRgba = ColorRgba::new(169, 199, 255, 255);   // #a9c7ff (Cornflower Slate)
    pub const STATUS_SYNCING: ColorRgba = ColorRgba::new(229, 190, 107, 255);   // #e5be6b (Amber Mineral)
    pub const STATUS_STABLE: ColorRgba = ColorRgba::new(133, 215, 158, 255);    // #85d79e (Celadon Green)
    pub const STATUS_ERROR: ColorRgba = ColorRgba::new(224, 108, 117, 255);     // #e06c75 (Brick Crimson)

    // Tipografi
    pub const TEXT_PRIMARY: ColorRgba = ColorRgba::new(229, 233, 240, 255);     // #e5e9f0
    pub const TEXT_SECONDARY: ColorRgba = ColorRgba::new(155, 163, 175, 255);   // #9ba3af
    pub const TEXT_MUTED: ColorRgba = ColorRgba::new(101, 108, 120, 255);       // #656c78
}

/// BCDL Optik ve Hareket Kinematiği Sabitleri
pub struct BcdlKinematics;

impl BcdlKinematics {
    /// Kritik sönümlü yay direnci
    pub const SPRING_STIFFNESS: f32 = 420.0;
    /// Kritik sönümlü yay katsayısı
    pub const SPRING_DAMPING: f32 = 30.0;
    /// Yay kütlesi
    pub const SPRING_MASS: f32 = 1.0;

    /// Ghost Overlay devir tamamlandığında yok olma süresi (ms)
    pub const FADE_OUT_DURATION_MS: u64 = 120;

    /// Telemetri Hapı bulanıklık yarıçapı (Kawase / Dual-Kawase)
    pub const BACKDROP_BLUR_RADIUS_PX: u32 = 10;
    /// Doygunluk katsayısı (%92)
    pub const BACKDROP_SATURATION_PERCENT: u32 = 92;

    /// Pencere köşe kavis yarıçapı
    pub const CORNER_RADIUS_PX: f64 = 12.0;
    /// Telemetri Hapı köşe kavis yarıçapı
    pub const PILL_RADIUS_PX: f64 = 16.0;
}
