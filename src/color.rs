use std::marker::PhantomData;

use bitflags::bitflags;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColorSpace {
    Srgb,
    Hsl,
    Hwb,
    Lab,
    Lch,
    Oklab,
    Oklch,
    SrgbLinear,
    DisplayP3,
    A98Rgb,
    ProphotoRgb,
    Rec2020,
    XyzD50,
    XyzD65,
}

impl ColorSpace {
    pub fn is_rgb_like(&self) -> bool {
        matches!(
            self,
            Self::Srgb | Self::DisplayP3 | Self::A98Rgb | Self::ProphotoRgb | Self::Rec2020
        )
    }

    pub fn is_rectangular_orthogonal(&self) -> bool {
        matches!(self, Self::Lab | Self::Oklab)
    }

    pub fn is_cylindrical_polar(&self) -> bool {
        matches!(self, Self::Lch | Self::Oklch)
    }

    pub fn is_xyz_like(&self) -> bool {
        matches!(self, Self::XyzD50 | Self::XyzD65)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct ColorFlags : u8 {
        const C0_IS_NONE = 1 << 0;
        const C1_IS_NONE = 1 << 1;
        const C2_IS_NONE = 1 << 2;
        const ALPHA_IS_NONE = 1 << 3;
    }
}

pub type Components = [f32; 3];

#[derive(Clone, PartialEq, Debug)]
#[repr(C)]
pub struct Color {
    pub components: Components,
    pub flags: ColorFlags,
    pub color_space: ColorSpace,
    pub alpha: f32,
}

/// Implement a From<*> for this struct to allow components of that type to be
/// used to construct a new color.
pub struct ComponentDetails {
    value: f32,
    is_none: bool,
}

impl From<f32> for ComponentDetails {
    fn from(value: f32) -> Self {
        Self {
            value,
            is_none: false,
        }
    }
}

impl From<u8> for ComponentDetails {
    fn from(value: u8) -> Self {
        Self {
            value: value as f32,
            is_none: false,
        }
    }
}

impl From<Option<f32>> for ComponentDetails {
    fn from(value: Option<f32>) -> Self {
        if let Some(value) = value {
            Self {
                value,
                is_none: false,
            }
        } else {
            Self {
                value: 0.0,
                is_none: true,
            }
        }
    }
}

impl Color {
    pub fn new(
        color_space: ColorSpace,
        c0: impl Into<ComponentDetails>,
        c1: impl Into<ComponentDetails>,
        c2: impl Into<ComponentDetails>,
        alpha: impl Into<ComponentDetails>,
    ) -> Self {
        let mut flags = ColorFlags::empty();

        macro_rules! component_details {
            ($c:expr, $flag:expr) => {{
                let details = $c.into();
                if details.is_none {
                    flags |= $flag;
                }
                details.value
            }};
        }

        let c0 = component_details!(c0, ColorFlags::C0_IS_NONE);
        let c1 = component_details!(c1, ColorFlags::C1_IS_NONE);
        let c2 = component_details!(c2, ColorFlags::C2_IS_NONE);
        let alpha = component_details!(alpha, ColorFlags::ALPHA_IS_NONE);

        Self {
            components: [c0, c1, c2],
            flags,
            color_space,
            alpha,
        }
    }

    pub fn as_model<C: ColorSpaceModel>(&self) -> &C {
        if self.color_space != C::COLOR_SPACE {
            panic!(
                "Color is not in the requested color space ({:?})",
                C::COLOR_SPACE
            );
        }
        unsafe { std::mem::transmute(self) }
    }
}

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

#[repr(C)]
pub struct Rgb<C: tag::RgbColorSpace, E: tag::RgbEncoding> {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub flags: ColorFlags,

    pub color_space_tag: PhantomData<C>,
    pub encoding_tag: PhantomData<E>,
}

pub type Srgb = Rgb<tag::Srgb, tag::GammaEncoded>;

impl ColorSpaceModel for Srgb {
    const COLOR_SPACE: ColorSpace = ColorSpace::Srgb;

    fn into_color(self, alpha: f32) -> Color {
        Color {
            components: [self.red, self.green, self.blue],
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
            components: [self.red, self.green, self.blue],
            flags: self.flags,
            color_space: Self::COLOR_SPACE,
            alpha,
        }
    }
}

#[repr(C)]
pub struct Hsl {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
    pub flags: ColorFlags,
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

pub trait WhitePointTag {
    const WHITE_POINT: Components;
}

pub struct D50Tag;

impl WhitePointTag for D50Tag {
    const WHITE_POINT: Components = [0.9642956764295677, 1.0, 0.8251046025104602];
}

pub struct D65Tag;

impl WhitePointTag for D65Tag {
    const WHITE_POINT: Components = [0.9504559270516716, 1.0, 1.0890577507598784];
}

#[repr(C)]
pub struct Xyz<W: WhitePointTag> {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub flags: ColorFlags,

    pub white_point: PhantomData<W>,
}

pub type XyzD50 = Xyz<D50Tag>;

impl ColorSpaceModel for XyzD50 {
    const COLOR_SPACE: ColorSpace = ColorSpace::XyzD50;

    fn into_color(self, alpha: f32) -> Color {
        Color {
            components: [self.x, self.y, self.z],
            flags: self.flags,
            color_space: Self::COLOR_SPACE,
            alpha,
        }
    }
}

pub type XyzD65 = Xyz<D65Tag>;

impl ColorSpaceModel for XyzD65 {
    const COLOR_SPACE: ColorSpace = ColorSpace::XyzD65;

    fn into_color(self, alpha: f32) -> Color {
        Color {
            components: [self.x, self.y, self.z],
            flags: self.flags,
            color_space: Self::COLOR_SPACE,
            alpha,
        }
    }
}

pub mod tag {
    pub trait RgbColorSpace {}

    pub trait RgbEncoding {}

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

    pub struct GammaEncoded;

    impl RgbEncoding for GammaEncoded {}

    pub struct LinearLight;

    impl RgbEncoding for LinearLight {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_for_missing_components_should_be_set_correctly() {
        let no_none = Color::new(ColorSpace::Srgb, 0.8235, 0.4117, 0.1176, 0.75);
        assert_eq!(no_none.flags, ColorFlags::empty());

        let no_red = Color::new(ColorSpace::Srgb, None, 0.4117, 0.1176, 0.75);
        assert_eq!(no_red.flags, ColorFlags::C0_IS_NONE);

        let no_green = Color::new(ColorSpace::Srgb, 0.8235, None, 0.1176, 0.75);
        assert_eq!(no_green.flags, ColorFlags::C1_IS_NONE);

        let no_blue = Color::new(ColorSpace::Srgb, 0.8235, 0.4117, None, 0.75);
        assert_eq!(no_blue.flags, ColorFlags::C2_IS_NONE);

        let no_alpha = Color::new(ColorSpace::Srgb, 0.8235, 0.4117, 0.1176, None);
        assert_eq!(no_alpha.flags, ColorFlags::ALPHA_IS_NONE);

        let all_none = Color::new(ColorSpace::Srgb, None, None, None, None);
        assert_eq!(
            all_none.flags,
            ColorFlags::C0_IS_NONE
                | ColorFlags::C1_IS_NONE
                | ColorFlags::C2_IS_NONE
                | ColorFlags::ALPHA_IS_NONE
        );
    }
}
