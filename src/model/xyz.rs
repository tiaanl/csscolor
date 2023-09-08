use std::marker::PhantomData;

use super::ColorSpaceModel;
use crate::{Color, ColorFlags, ColorSpace, Components};

pub trait WhitePoint {
    const WHITE_POINT: Components;
}

pub struct D50;
impl WhitePoint for D50 {
    const WHITE_POINT: Components = [0.9642956764295677, 1.0, 0.8251046025104602];
}

pub struct D65;
impl WhitePoint for D65 {
    const WHITE_POINT: Components = [0.9504559270516716, 1.0, 1.0890577507598784];
}

#[repr(C)]
pub struct Xyz<W: WhitePoint> {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub flags: ColorFlags,

    pub white_point: PhantomData<W>,
}

pub type XyzD50 = Xyz<D50>;

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

pub type XyzD65 = Xyz<D65>;

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
