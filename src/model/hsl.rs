use crate::{Color, ColorFlags, ColorSpace};

use super::ColorSpaceModel;

#[repr(C)]
pub struct Hsl {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
    pub flags: ColorFlags,
}

impl Hsl {
    pub fn new(hue: f32, saturation: f32, lightness: f32, flags: ColorFlags) -> Self {
        Self {
            hue,
            saturation,
            lightness,
            flags,
        }
    }
}

impl ColorSpaceModel for Hsl {
    const COLOR_SPACE: ColorSpace = ColorSpace::Hsl;

    fn into_color(self, alpha: f32) -> Color {
        Color {
            components: [self.hue, self.saturation, self.lightness],
            flags: self.flags,
            color_space: Self::COLOR_SPACE,
            alpha,
        }
    }
}
