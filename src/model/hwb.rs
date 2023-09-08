use crate::{Color, ColorFlags, ColorSpace, Components};

use super::ColorSpaceModel;

#[repr(C)]
pub struct Hwb {
    pub hue: f32,
    pub whiteness: f32,
    pub blackness: f32,
    pub flags: ColorFlags,
}

impl Hwb {
    pub fn new(hue: f32, whiteness: f32, blackness: f32, flags: ColorFlags) -> Self {
        Self {
            hue,
            whiteness,
            blackness,
            flags,
        }
    }
}

impl ColorSpaceModel for Hwb {
    const COLOR_SPACE: ColorSpace = ColorSpace::Hwb;

    fn into_color(self, alpha: f32) -> Color {
        Color {
            components: Components(self.hue, self.whiteness, self.blackness),
            flags: self.flags,
            color_space: Self::COLOR_SPACE,
            alpha,
        }
    }
}
