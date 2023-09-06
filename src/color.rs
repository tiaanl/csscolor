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
    pub fn is_rgb_line(&self) -> bool {
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
pub struct Color {
    pub components: Components,
    pub alpha: f32,
    pub color_space: ColorSpace,
    pub flags: ColorFlags,
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
            alpha,
            color_space,
            flags,
        }
    }

    pub fn as_srgb(&self) -> &Rgb<Srgb, GammaEncoded> {
        if self.color_space != ColorSpace::Srgb {
            panic!("Color not in requested color space.");
        }
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_srgb_linear(&self) -> &Rgb<Srgb, LinearLight> {
        if self.color_space != ColorSpace::SrgbLinear {
            panic!("Color not in requested color space.");
        }
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_hsl(&self) -> &Hsl {
        if self.color_space != ColorSpace::Hsl {
            panic!("Color not in requested color space.");
        }
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_hwb(&self) -> &Hwb {
        if self.color_space != ColorSpace::Hwb {
            panic!("Color not in requested color space.");
        }
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_lab(&self) -> &Lab {
        if self.color_space != ColorSpace::Lab {
            panic!("Color not in requested color space.");
        }
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_lch(&self) -> &Lch {
        if self.color_space != ColorSpace::Lch {
            panic!("Color not in requested color space.");
        }
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_xyz_d50(&self) -> &Xyz<D50> {
        if self.color_space != ColorSpace::XyzD50 {
            panic!("Color not in requested color space.");
        }
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_xyz_d65(&self) -> &Xyz<D65> {
        if self.color_space != ColorSpace::XyzD65 {
            panic!("Color not in requested color space.");
        }
        unsafe { std::mem::transmute(self) }
    }
}

macro_rules! impl_color_space_struct {
    ($space:tt) => {
        pub fn components(&self) -> &Components {
            if self.color_space != ColorSpace::$space {
                panic!("Color not in requested color space.");
            }
            unsafe { std::mem::transmute(self) }
        }

        pub fn into_color(self) -> Color {
            unsafe { std::mem::transmute(self) }
        }
    };
}

pub trait RgbColorSpaceTag {}

pub trait RgbEncodingTag {}

pub struct Srgb;

impl RgbColorSpaceTag for Srgb {}

pub struct DisplayP3;

impl RgbColorSpaceTag for DisplayP3 {}

pub struct A98Rgb;

impl RgbColorSpaceTag for A98Rgb {}

pub struct ProphotoRgb;

impl RgbColorSpaceTag for ProphotoRgb {}

pub struct Rec2020;

impl RgbColorSpaceTag for Rec2020 {}

pub struct GammaEncoded;

impl RgbEncodingTag for GammaEncoded {}

pub struct LinearLight;

impl RgbEncodingTag for LinearLight {}

pub struct Rgb<C: RgbColorSpaceTag, E: RgbEncodingTag> {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
    pub color_space: ColorSpace,
    pub flags: ColorFlags,

    pub color_space_tag: PhantomData<C>,
    pub encoding_tag: PhantomData<E>,
}

impl Rgb<Srgb, GammaEncoded> {
    impl_color_space_struct!(Srgb);
}

impl Rgb<Srgb, LinearLight> {
    impl_color_space_struct!(SrgbLinear);
}

pub struct Hsl {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
    pub alpha: f32,
    pub color_space: ColorSpace,
    pub flags: ColorFlags,
}

impl Hsl {
    impl_color_space_struct!(Hsl);
}

pub struct Hwb {
    pub hue: f32,
    pub whiteness: f32,
    pub blackness: f32,
    pub alpha: f32,
    pub color_space: ColorSpace,
    pub flags: ColorFlags,
}

impl Hwb {
    impl_color_space_struct!(Hwb);
}

pub struct Lab {
    pub lightness: f32,
    pub a: f32,
    pub b: f32,
    pub alpha: f32,
    pub color_space: ColorSpace,
    pub flags: ColorFlags,
}

impl Lab {
    impl_color_space_struct!(Lab);
}

pub struct Lch {
    pub lightness: f32,
    pub chroma: f32,
    pub hue: f32,
    pub alpha: f32,
    pub color_space: ColorSpace,
    pub flags: ColorFlags,
}

impl Lch {
    impl_color_space_struct!(Lch);
}

pub trait WhitePointTag {
    const WHITE_POINT: Components;
}

pub struct D50;

impl WhitePointTag for D50 {
    const WHITE_POINT: Components = [0.9642956764295677, 1.0, 0.8251046025104602];
}

pub struct D65;

impl WhitePointTag for D65 {
    const WHITE_POINT: Components = [0.9504559270516716, 1.0, 1.0890577507598784];
}

pub struct Xyz<W: WhitePointTag> {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub alpha: f32,
    pub color_space: ColorSpace,
    pub flags: ColorFlags,

    pub white_point: PhantomData<W>,
}

impl<W: WhitePointTag> Xyz<W> {
    pub fn components(&self) -> &Components {
        unsafe { std::mem::transmute(self) }
    }
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
