use crate::{Color, ColorFlags, ColorSpace};

use super::ColorSpaceModel;

#[repr(C)]
pub struct Hwb {
    pub hue: f32,
    pub whiteness: f32,
    pub blackness: f32,
    pub flags: ColorFlags,
}

impl ColorSpaceModel for Hwb {
    const COLOR_SPACE: ColorSpace = ColorSpace::Hwb;

    fn into_color(self, alpha: f32) -> Color {
        Color {
            components: [self.hue, self.whiteness, self.blackness],
            flags: self.flags,
            color_space: Self::COLOR_SPACE,
            alpha,
        }
    }
}
