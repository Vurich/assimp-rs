use crate::math::Color3D;
#[cfg(feature = "cgmath")]
use cgmath::Vector4;
use ffi::aiColor4D;

define_type_and_iterator! {
    /// Color4D docs
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Color4D(aiColor4D)
    /// Color4DIter docs
    struct Color4DIter
}

impl Color4D {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color4D {
        Color4D(aiColor4D {
            r: r,
            g: g,
            b: b,
            a: a,
        })
    }
}

impl From<[f32; 4]> for Color4D {
    fn from(v: [f32; 4]) -> Color4D {
        Color4D::new(v[0], v[1], v[2], v[3])
    }
}

impl From<Color4D> for [f32; 4] {
    fn from(c: Color4D) -> [f32; 4] {
        [c.r, c.g, c.b, c.a]
    }
}

#[cfg(feature = "cgmath")]
impl From<Vector4<f32>> for Color4D {
    fn from(v: Vector4<f32>) -> Color4D {
        Color4D::new(v[0], v[1], v[2], v[3])
    }
}

#[cfg(feature = "cgmath")]
impl From<Color4D> for Vector4<f32> {
    fn from(c: Color4D) -> Vector4<f32> {
        Vector4::new(c.r, c.g, c.b, c.a)
    }
}

impl From<Color3D> for Color4D {
    fn from(c: Color3D) -> Color4D {
        Color4D::new(c.r, c.g, c.b, 1.0)
    }
}
