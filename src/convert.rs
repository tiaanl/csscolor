use crate::color::{Color, ColorSpace};

impl Color {
    pub fn to_color_space(&self, color_space: ColorSpace) -> Color {
        if self.color_space == color_space {
            return self.clone();
        }

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_conversion_happens_if_color_spaces_are_equal() {
        let from = Color::new(ColorSpace::Srgb, 0.8235, 0.4117, 0.1176, 1.0);
        let to = from.to_color_space(ColorSpace::Srgb);
        assert_eq!(to, from);
    }

    #[test]
    fn conversion_srgb_to_hsl() {}
}
