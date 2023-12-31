use std::marker::PhantomData;

use crate::{Color, ColorFlags, ColorSpace, Components};

use super::ColorSpaceModel;

pub mod tag {
    pub trait RgbColorSpace {}

    pub struct Srgb;
    impl RgbColorSpace for Srgb {}

    pub struct DisplayP3;
    impl RgbColorSpace for DisplayP3 {}

    pub struct A98Rgb;
    impl RgbColorSpace for A98Rgb {}

    pub struct ProphotoRgb;
    impl RgbColorSpace for ProphotoRgb {}

    pub struct Rec2020;
    impl RgbColorSpace for Rec2020 {}

    pub trait RgbEncoding {}

    pub struct GammaEncoded;
    impl RgbEncoding for GammaEncoded {}

    pub struct LinearLight;
    impl RgbEncoding for LinearLight {}
}

#[repr(C)]
pub struct Rgb<C: tag::RgbColorSpace, E: tag::RgbEncoding> {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub flags: ColorFlags,

    color_space_tag: PhantomData<C>,
    encoding_tag: PhantomData<E>,
}

impl<C: tag::RgbColorSpace, E: tag::RgbEncoding> Rgb<C, E> {
    pub fn new(red: f32, green: f32, blue: f32, flags: ColorFlags) -> Self {
        Self {
            red,
            green,
            blue,
            flags,

            color_space_tag: PhantomData,
            encoding_tag: PhantomData,
        }
    }
}

pub type Srgb = Rgb<tag::Srgb, tag::GammaEncoded>;

impl ColorSpaceModel for Srgb {
    const COLOR_SPACE: ColorSpace = ColorSpace::Srgb;

    fn into_color(self, alpha: f32) -> Color {
        Color {
            components: Components(self.red, self.green, self.blue),
            flags: self.flags,
            color_space: Self::COLOR_SPACE,
            alpha,
        }
    }
}

pub type SrgbLinear = Rgb<tag::Srgb, tag::LinearLight>;

impl ColorSpaceModel for SrgbLinear {
    const COLOR_SPACE: ColorSpace = ColorSpace::SrgbLinear;

    fn into_color(self, alpha: f32) -> Color {
        Color {
            components: Components(self.red, self.green, self.blue),
            flags: self.flags,
            color_space: Self::COLOR_SPACE,
            alpha,
        }
    }
}
