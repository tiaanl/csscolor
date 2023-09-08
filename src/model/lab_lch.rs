use super::ColorSpaceModel;
use crate::{Color, ColorFlags, ColorSpace};

#[repr(C)]
pub struct Lab {
    pub lightness: f32,
    pub a: f32,
    pub b: f32,
    pub flags: ColorFlags,
}

impl ColorSpaceModel for Lab {
    const COLOR_SPACE: ColorSpace = ColorSpace::Lab;

    fn into_color(self, alpha: f32) -> Color {
        Color {
            components: [self.lightness, self.a, self.b],
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

impl ColorSpaceModel for Lch {
    const COLOR_SPACE: ColorSpace = ColorSpace::Lch;

    fn into_color(self, alpha: f32) -> Color {
        Color {
            components: [self.lightness, self.chroma, self.hue],
            flags: self.flags,
            color_space: Self::COLOR_SPACE,
            alpha,
        }
    }
}
