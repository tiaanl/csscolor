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
