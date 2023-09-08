use crate::{Color, ColorSpace, Components};

mod hsl;
mod hwb;
mod lab_lch;
mod rgb;
mod xyz;

pub use hsl::Hsl;
pub use hwb::Hwb;
pub use lab_lch::{Lab, Lch};
pub use rgb::{Rgb, Srgb, SrgbLinear};
pub use xyz::{WhitePoint, XyzD50, XyzD65, D50, D65};

pub trait ColorSpaceModel {
    const COLOR_SPACE: ColorSpace;

    fn components(&self) -> &Components
    where
        Self: Sized,
    {
        unsafe { std::mem::transmute(self) }
    }

    fn into_color(self, alpha: f32) -> Color;
}
