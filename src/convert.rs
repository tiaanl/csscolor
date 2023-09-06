use crate::color::{Color, ColorSpace};

impl Color {
    pub fn to_color_space(&self, color_space: ColorSpace) -> Color {
        use ColorSpace as C;

        if self.color_space == color_space {
            return self.clone();
        }

        // Handle conversions that can be done directly.
        match (self.color_space, color_space) {
            (C::Srgb, C::Hsl) => {
                let [hue, saturation, lightness] = util::rgb_to_hsl(&self.components);
                return Self::new(color_space, hue, saturation, lightness, self.alpha);
            }
            (C::Hsl, C::Srgb) => {
                let [red, green, blue] = util::hsl_to_rgb(&self.components);
                return Self::new(color_space, red, green, blue, self.alpha);
            }

            (C::Srgb, C::Hwb) => {
                let [hue, whiteness, blackness] = util::rgb_to_hwb(&self.components);
                return Self::new(color_space, hue, whiteness, blackness, self.alpha);
            }
            (C::Hwb, C::Srgb) => {
                let [red, green, blue] = util::hwb_to_rgb(&self.components);
                return Self::new(color_space, red, green, blue, self.alpha);
            }

            (C::Lch, C::Lab) | (C::Oklch, C::Oklab) => {
                let [lightness, chroma, hue] = util::polar_to_orthogonal(&self.components);
                return Self::new(color_space, lightness, chroma, hue, self.alpha);
            }
            (C::Lab, C::Lch) | (C::Oklab, C::Oklch) => {
                let [lightness, a, b] = util::orthogonal_to_polar(&self.components);
                return Self::new(color_space, lightness, a, b, self.alpha);
            }

            _ => {
                // Not a direct conversion.
            }
        }

        todo!()
    }
}

mod util {
    use super::super::color::Components;

    /// Normalize hue into [0, 360).
    fn normalize_hue(hue: f32) -> f32 {
        hue.rem_euclid(360.0)
    }

    /// Calculate the hue from RGB components and return it along with the min and
    /// max RGB values.
    fn rgb_to_hue_min_max(red: f32, green: f32, blue: f32) -> (f32, f32, f32) {
        let max = red.max(green).max(blue);
        let min = red.min(green).min(blue);

        let delta = max - min;

        let hue = if delta != 0.0 {
            60.0 * if max == red {
                (green - blue) / delta + if green < blue { 6.0 } else { 0.0 }
            } else if max == green {
                (blue - red) / delta + 2.0
            } else {
                (red - green) / delta + 4.0
            }
        } else {
            f32::NAN
        };

        (hue, min, max)
    }

    /// Convert from RGB notation to HSL notation.
    /// <https://drafts.csswg.org/css-color-4/#rgb-to-hsl>
    pub fn rgb_to_hsl(from: &Components) -> Components {
        let [red, green, blue] = *from;

        let (hue, min, max) = rgb_to_hue_min_max(red, green, blue);

        let lightness = (min + max) / 2.0;
        let delta = max - min;

        let saturation = if delta != 0.0 {
            if lightness == 0.0 || lightness == 1.0 {
                0.0
            } else {
                (max - lightness) / lightness.min(1.0 - lightness)
            }
        } else {
            0.0
        };

        [hue, saturation, lightness]
    }

    /// Convert from HSL notation to RGB notation.
    /// https://drafts.csswg.org/css-color-4/#hsl-to-rgb
    pub fn hsl_to_rgb(from: &Components) -> Components {
        fn hue_to_rgb(t1: f32, t2: f32, hue: f32) -> f32 {
            let hue = normalize_hue(hue);

            if hue * 6.0 < 360.0 {
                t1 + (t2 - t1) * hue / 60.0
            } else if hue * 2.0 < 360.0 {
                t2
            } else if hue * 3.0 < 720.0 {
                t1 + (t2 - t1) * (240.0 - hue) / 60.0
            } else {
                t1
            }
        }

        let [hue, saturation, lightness] = *from;

        let t2 = if lightness <= 0.5 {
            lightness * (saturation + 1.0)
        } else {
            lightness + saturation - lightness * saturation
        };
        let t1 = lightness * 2.0 - t2;

        [
            hue_to_rgb(t1, t2, hue + 120.0),
            hue_to_rgb(t1, t2, hue),
            hue_to_rgb(t1, t2, hue - 120.0),
        ]
    }

    /// Convert from RGB notation to HWB notation.
    /// https://drafts.csswg.org/css-color-4/#rgb-to-hwb
    pub fn rgb_to_hwb(from: &Components) -> Components {
        let [red, green, blue] = *from;

        let (hue, min, max) = rgb_to_hue_min_max(red, green, blue);

        let whiteness = min;
        let blackness = 1.0 - max;

        [hue, whiteness, blackness]
    }

    /// Convert from HWB notation to RGB notation.
    /// https://drafts.csswg.org/css-color-4/#hwb-to-rgb
    pub fn hwb_to_rgb(from: &Components) -> Components {
        let [hue, whiteness, blackness] = *from;

        if whiteness + blackness > 1.0 {
            let gray = whiteness / (whiteness + blackness);
            return [gray, gray, gray];
        }

        let x = 1.0 - whiteness - blackness;
        hsl_to_rgb(&[hue, 1.0, 0.5]).map(|v| v * x + whiteness)
    }

    /// Convert from a cylindrical polar coordinate to the rectangular orthogonal
    /// form. This is used to convert (ok)lch to (ok)lab.
    /// <https://drafts.csswg.org/css-color-4/#color-conversion-code>
    pub fn polar_to_orthogonal(from: &Components) -> Components {
        let [lightness, chroma, hue] = *from;

        let hue = hue.to_radians();
        let a = chroma * hue.cos();
        let b = chroma * hue.sin();

        [lightness, a, b]
    }

    /// Convert from the rectangular orthogonal form to a cylindrical polar
    /// coordinate. This is used to convert (ok)lab to (ok)lch.
    /// <https://drafts.csswg.org/css-color-4/#color-conversion-code>
    pub fn orthogonal_to_polar(from: &Components) -> Components {
        let [lightness, a, b] = *from;

        let hue = b.atan2(a).to_degrees().rem_euclid(360.0);
        let chroma = (a * a + b * b).sqrt();

        [lightness, chroma, hue]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! almost_equal {
        ($c1:expr, $c2:expr) => {{
            // Equality within 4 decimal places.
            ($c2 - $c1).abs() < 1.0e-4
        }};
    }

    #[test]
    fn no_conversion_happens_if_color_spaces_are_equal() {
        let from = Color::new(ColorSpace::Srgb, 0.8235, 0.4117, 0.1176, 1.0);
        let to = from.to_color_space(ColorSpace::Srgb);
        assert_eq!(to, from);
    }

    #[test]
    fn conversions() {
        #[rustfmt::skip]
        let conversions = [
            (ColorSpace::Srgb, 0.8235, 0.4118, 0.1176, 1.0, ColorSpace::Hsl, 25.0064, 0.75, 0.4706, 1.0),

            (ColorSpace::Lab, 56.6293, 39.2371, 57.5538, 1.0, ColorSpace::Lch, 56.6293, 69.6562, 55.7159, 1.0),
            (ColorSpace::Lch, 56.6293, 69.6562, 55.7159, 1.0, ColorSpace::Lab, 56.6293, 39.2371, 57.5538, 1.0),
        ];

        for (
            from_color_space,
            from_c0,
            from_c1,
            from_c2,
            from_alpha,
            to_color_space,
            to_c0,
            to_c1,
            to_c2,
            to_alpha,
        ) in conversions
        {
            let from = Color::new(from_color_space, from_c0, from_c1, from_c2, from_alpha);
            let to = Color::new(to_color_space, to_c0, to_c1, to_c2, to_alpha);

            let result = from.to_color_space(to_color_space);

            assert_eq!(result.color_space, to.color_space);
            assert!(
                almost_equal!(result.components[0], to.components[0]),
                "c0 {} is not equal to {}",
                result.components[0],
                to.components[0]
            );
            assert!(
                almost_equal!(result.components[1], to.components[1]),
                "c1 {} is not equal to {}",
                result.components[1],
                to.components[1]
            );
            assert!(
                almost_equal!(result.components[2], to.components[2]),
                "c2 {} is not equal to {}",
                result.components[2],
                to.components[2]
            );
            assert!(
                almost_equal!(result.alpha, to.alpha),
                "alpha {} is not equal to {}",
                result.alpha,
                to.alpha
            );
        }
    }
}
