use crate::{
    color::{Color, ColorSpace, Components},
    model::{ColorSpaceModel, WhitePoint},
    Hsl, Hwb,
};
use crate::{Lab, Lch, Srgb, SrgbLinear, XyzD50, XyzD65, D50};
use std::marker::PhantomData;

type Transform = euclid::default::Transform3D<f32>;
type Vector = euclid::default::Vector3D<f32>;

fn transform(from: &Components, mat: &Transform) -> Components {
    let result = mat.transform_vector3d(Vector::new(from[0], from[1], from[2]));
    [result.x, result.y, result.z]
}

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

        // We have to go all the way to XYZ space to convert.
        let xyz = match self.color_space {
            C::Srgb => self
                .as_model::<Srgb>()
                .to_linear_light()
                .to_xyz_d65()
                .to_xyz_d50(),
            C::Hsl => self
                .as_model::<Hsl>()
                .to_srgb()
                .to_linear_light()
                .to_xyz_d65()
                .to_xyz_d50(),
            C::Hwb => self
                .as_model::<Hwb>()
                .to_srgb()
                .to_linear_light()
                .to_xyz_d65()
                .to_xyz_d50(),
            C::Lab => self.as_model::<Lab>().to_xyz_d50(),
            C::Lch => self.as_model::<Lch>().to_lab().to_xyz_d50(),
            C::Oklab => todo!(),
            C::Oklch => todo!(),
            C::SrgbLinear => self.as_model::<SrgbLinear>().to_xyz_d65().to_xyz_d50(),
            C::DisplayP3 => todo!(),
            C::A98Rgb => todo!(),
            C::ProphotoRgb => todo!(),
            C::Rec2020 => todo!(),
            C::XyzD50 => XyzD50 {
                x: self.components[0],
                y: self.components[1],
                z: self.components[2],
                flags: self.flags,
                white_point: PhantomData,
            },
            C::XyzD65 => self.as_model::<XyzD65>().to_xyz_d50(),
        };

        let _result: Color = match color_space {
            C::Srgb => xyz
                .to_xyz_d65()
                .to_srgb()
                .to_gamma_encoded()
                .into_color(self.alpha),
            C::Hsl => xyz
                .to_xyz_d65()
                .to_srgb()
                .to_gamma_encoded()
                .to_hsl()
                .into_color(self.alpha),
            C::Hwb => xyz
                .to_xyz_d65()
                .to_srgb()
                .to_gamma_encoded()
                .to_hwb()
                .into_color(self.alpha),
            C::Lab => xyz.to_lab().into_color(self.alpha),
            C::Lch => xyz.to_lab().to_lch().into_color(self.alpha),
            C::Oklab => todo!(),
            C::Oklch => todo!(),
            C::SrgbLinear => xyz.to_xyz_d65().to_srgb().into_color(self.alpha),
            C::DisplayP3 => todo!(),
            C::A98Rgb => todo!(),
            C::ProphotoRgb => todo!(),
            C::Rec2020 => todo!(),
            C::XyzD50 => xyz.into_color(self.alpha),
            C::XyzD65 => xyz.to_xyz_d65().into_color(self.alpha),
        };

        todo!()
    }
}

impl Srgb {
    fn to_linear_light(&self) -> SrgbLinear {
        let [red, green, blue] = [self.red, self.green, self.blue].map(|c| {
            let abs = c.abs();

            if abs < 0.04045 {
                c / 12.92
            } else {
                c.signum() * ((abs + 0.055) / 1.055).powf(2.4)
            }
        });

        SrgbLinear {
            red,
            green,
            blue,
            flags: self.flags,

            color_space_tag: PhantomData,
            encoding_tag: PhantomData,
        }
    }

    fn to_hsl(&self) -> Hsl {
        let [hue, saturation, lightness] = util::rgb_to_hsl(self.components());
        Hsl {
            hue,
            saturation,
            lightness,
            flags: self.flags,
        }
    }

    fn to_hwb(&self) -> Hwb {
        let [hue, whiteness, blackness] = util::rgb_to_hwb(self.components());
        Hwb {
            hue,
            whiteness,
            blackness,
            flags: self.flags,
        }
    }
}

impl SrgbLinear {
    pub fn to_gamma_encoded(&self) -> Srgb {
        let [red, green, blue] = self.components().map(|c| {
            let abs = c.abs();

            if abs > 0.0031308 {
                c.signum() * (1.055 * abs.powf(1.0 / 2.4) - 0.055)
            } else {
                12.92 * c
            }
        });

        Srgb {
            red,
            green,
            blue,
            flags: self.flags,

            color_space_tag: PhantomData,
            encoding_tag: PhantomData,
        }
    }

    pub fn to_xyz_d65(&self) -> XyzD65 {
        #[rustfmt::skip]
        const TO_XYZ: Transform = Transform::new(
            0.4123907992659595,  0.21263900587151036, 0.01933081871559185, 0.0,
            0.35758433938387796, 0.7151686787677559,  0.11919477979462599, 0.0,
            0.1804807884018343,  0.07219231536073371, 0.9505321522496606,  0.0,
            0.0,                 0.0,                 0.0,                 1.0,
        );

        let [x, y, z] = transform(self.components(), &TO_XYZ);

        XyzD65 {
            x,
            y,
            z,
            flags: self.flags,

            white_point: PhantomData,
        }
    }
}

impl Hsl {
    pub fn to_srgb(&self) -> Srgb {
        let [red, green, blue] = util::hsl_to_rgb(self.components());
        Srgb {
            red,
            green,
            blue,
            flags: self.flags,

            color_space_tag: PhantomData,
            encoding_tag: PhantomData,
        }
    }
}

impl Hwb {
    pub fn to_srgb(&self) -> Srgb {
        let [red, green, blue] = util::hwb_to_rgb(self.components());
        Srgb {
            red,
            green,
            blue,
            flags: self.flags,

            color_space_tag: PhantomData,
            encoding_tag: PhantomData,
        }
    }
}

impl Lab {
    const KAPPA: f32 = 24389.0 / 27.0;
    const EPSILON: f32 = 216.0 / 24389.0;

    pub fn to_xyz_d50(&self) -> XyzD50 {
        let f1 = (self.lightness + 16.0) / 116.0;
        let f0 = f1 + self.a / 500.0;
        let f2 = f1 - self.b / 200.0;

        let f0_cubed = f0 * f0 * f0;
        let x = if f0_cubed > Self::EPSILON {
            f0_cubed
        } else {
            (116.0 * f0 - 16.0) / Self::KAPPA
        };

        let y = if self.lightness > Self::KAPPA * Self::EPSILON {
            let v = (self.lightness + 16.0) / 116.0;
            v * v * v
        } else {
            self.lightness / Self::KAPPA
        };

        let f2_cubed = f2 * f2 * f2;
        let z = if f2_cubed > Self::EPSILON {
            f2_cubed
        } else {
            (116.0 * f2 - 16.0) / Self::KAPPA
        };

        XyzD50 {
            x: x * D50::WHITE_POINT[0],
            y: y * D50::WHITE_POINT[1],
            z: z * D50::WHITE_POINT[2],
            flags: self.flags,

            white_point: PhantomData,
        }
    }

    pub fn to_lch(&self) -> Lch {
        let [lightness, chroma, hue] = util::orthogonal_to_polar(self.components());
        Lch {
            lightness,
            chroma,
            hue,
            flags: self.flags,
        }
    }
}

impl Lch {
    pub fn to_lab(&self) -> Lab {
        let [lightness, a, b] = util::polar_to_orthogonal(self.components());

        Lab {
            lightness,
            a,
            b,
            flags: self.flags,
        }
    }
}

impl XyzD50 {
    pub fn to_xyz_d65(&self) -> XyzD65 {
        #[rustfmt::skip]
        const MAT: Transform = Transform::new(
             0.9554734527042182,   -0.028369706963208136,  0.012314001688319899, 0.0,
            -0.023098536874261423,  1.0099954580058226,   -0.020507696433477912, 0.0,
             0.0632593086610217,    0.021041398966943008,  1.3303659366080753,   0.0,
             0.0,                   0.0,                   0.0,                  1.0,
        );

        let [x, y, z] = transform(self.components(), &MAT);

        XyzD65 {
            x,
            y,
            z,
            flags: self.flags,

            white_point: PhantomData,
        }
    }

    fn to_lab(&self) -> Lab {
        const KAPPA: f32 = 24389.0 / 27.0;
        const EPSILON: f32 = 216.0 / 24389.0;

        let adapted = [
            self.x / D50::WHITE_POINT[0],
            self.y / D50::WHITE_POINT[1],
            self.z / D50::WHITE_POINT[2],
        ];

        // 4. Convert D50-adapted XYZ to Lab.
        let [f0, f1, f2] = adapted.map(|v| {
            if v > EPSILON {
                v.cbrt()
            } else {
                (KAPPA * v + 16.0) / 116.0
            }
        });

        let lightness = 116.0 * f1 - 16.0;
        let a = 500.0 * (f0 - f1);
        let b = 200.0 * (f1 - f2);

        Lab {
            lightness,
            a,
            b,
            flags: self.flags,
        }
    }
}

impl XyzD65 {
    pub fn to_srgb(&self) -> SrgbLinear {
        #[rustfmt::skip]
        const FROM_XYZ: Transform = Transform::new(
             3.2409699419045213, -0.9692436362808798,  0.05563007969699361, 0.0,
            -1.5373831775700935,  1.8759675015077206, -0.20397695888897657, 0.0,
            -0.4986107602930033,  0.04155505740717561, 1.0569715142428786,  0.0,
             0.0,                 0.0,                 0.0,                 1.0,
        );

        let [red, green, blue] = transform(self.components(), &FROM_XYZ);

        SrgbLinear {
            red,
            green,
            blue,
            flags: self.flags,

            color_space_tag: PhantomData,
            encoding_tag: PhantomData,
        }
    }

    pub fn to_xyz_d50(&self) -> XyzD50 {
        #[rustfmt::skip]
        const MAT: Transform = Transform::new(
             1.0479298208405488,    0.029627815688159344, -0.009243058152591178, 0.0,
             0.022946793341019088,  0.990434484573249,     0.015055144896577895, 0.0,
            -0.05019222954313557,  -0.01707382502938514,   0.7518742899580008,   0.0,
             0.0,                   0.0,                   0.0,                  1.0,
        );

        let [x, y, z] = transform(self.components(), &MAT);

        XyzD50 {
            x,
            y,
            z,
            flags: self.flags,

            white_point: PhantomData,
        }
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
