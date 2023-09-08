use super::ColorSpaceModel;
use crate::{Color, ColorFlags, ColorSpace, Components};

#[repr(C)]
pub struct Lab {
    pub lightness: f32,
    pub a: f32,
    pub b: f32,
    pub flags: ColorFlags,
}

impl Lab {
    pub fn new(lightness: f32, a: f32, b: f32, flags: ColorFlags) -> Self {
        Self {
            lightness,
            a,
            b,
            flags,
        }
    }
}

impl ColorSpaceModel for Lab {
    const COLOR_SPACE: ColorSpace = ColorSpace::Lab;

    fn into_color(self, alpha: f32) -> Color {
        Color {
            components: Components(self.lightness, self.a, self.b),
            flags: self.flags,
            color_space: Self::COLOR_SPACE,
            alpha,
        }
    }
}

#[repr(C)]
pub struct Lch {
    pub lightness: f32,
    pub chroma: f32,
    pub hue: f32,
    pub flags: ColorFlags,
}

impl Lch {
    pub fn new(lightness: f32, chroma: f32, hue: f32, flags: ColorFlags) -> Self {
        Self {
            lightness,
            chroma,
            hue,
            flags,
        }
    }
}

impl ColorSpaceModel for Lch {
    const COLOR_SPACE: ColorSpace = ColorSpace::Lch;

    fn into_color(self, alpha: f32) -> Color {
        Color {
            components: Components(self.lightness, self.chroma, self.hue),
            flags: self.flags,
            color_space: Self::COLOR_SPACE,
            alpha,
        }
    }
}
