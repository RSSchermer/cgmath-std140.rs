use cgmath::{Vector1, Vector2, Vector3, Vector4, Matrix2, Matrix3, Matrix4};

/// Implemented for types that can be represented as [st140] values.
pub trait AsStd140 {
    /// The `std140` that this type converts to.
    type Std140: std140::ReprStd140;

    /// Converts this value into an [st140] value.
    fn as_std140(&self) -> Self::Std140;
}

impl AsStd140 for Vector1<f32> {
    type Std140 = std140::float;

    fn as_std140(&self) -> Self::Std140 {
        std140::float(self.x)
    }
}

impl AsStd140 for Vector2<f32> {
    type Std140 = std140::vec2;

    fn as_std140(&self) -> Self::Std140 {
        std140::vec2(self.x, self.y)
    }
}

impl AsStd140 for Vector3<f32> {
    type Std140 = std140::vec3;

    fn as_std140(&self) -> Self::Std140 {
        std140::vec3(self.x, self.y, self.z)
    }
}

impl AsStd140 for Vector4<f32> {
    type Std140 = std140::vec4;

    fn as_std140(&self) -> Self::Std140 {
        std140::vec4(self.x, self.y, self.z, self.w)
    }
}

impl AsStd140 for Vector1<i32> {
    type Std140 = std140::int;

    fn as_std140(&self) -> Self::Std140 {
        std140::int(self.x)
    }
}

impl AsStd140 for Vector2<i32> {
    type Std140 = std140::ivec2;

    fn as_std140(&self) -> Self::Std140 {
        std140::ivec2(self.x, self.y)
    }
}

impl AsStd140 for Vector3<i32> {
    type Std140 = std140::ivec3;

    fn as_std140(&self) -> Self::Std140 {
        std140::ivec3(self.x, self.y, self.z)
    }
}

impl AsStd140 for Vector4<i32> {
    type Std140 = std140::ivec4;

    fn as_std140(&self) -> Self::Std140 {
        std140::ivec4(self.x, self.y, self.z, self.w)
    }
}

impl AsStd140 for Vector1<u32> {
    type Std140 = std140::uint;

    fn as_std140(&self) -> Self::Std140 {
        std140::uint(self.x)
    }
}

impl AsStd140 for Vector2<u32> {
    type Std140 = std140::uvec2;

    fn as_std140(&self) -> Self::Std140 {
        std140::uvec2(self.x, self.y)
    }
}

impl AsStd140 for Vector3<u32> {
    type Std140 = std140::uvec3;

    fn as_std140(&self) -> Self::Std140 {
        std140::uvec3(self.x, self.y, self.z)
    }
}

impl AsStd140 for Vector4<u32> {
    type Std140 = std140::uvec4;

    fn as_std140(&self) -> Self::Std140 {
        std140::uvec4(self.x, self.y, self.z, self.w)
    }
}

impl AsStd140 for Vector1<bool> {
    type Std140 = std140::boolean;

    fn as_std140(&self) -> Self::Std140 {
        self.x.into()
    }
}

impl AsStd140 for Vector2<bool> {
    type Std140 = std140::bvec2;

    fn as_std140(&self) -> Self::Std140 {
        std140::bvec2(self.x.into(), self.y.into())
    }
}

impl AsStd140 for Vector3<bool> {
    type Std140 = std140::bvec3;

    fn as_std140(&self) -> Self::Std140 {
        std140::bvec3(self.x.into(), self.y.into(), self.z.into())
    }
}

impl AsStd140 for Vector4<bool> {
    type Std140 = std140::bvec4;

    fn as_std140(&self) -> Self::Std140 {
        std140::bvec4(self.x.into(), self.y.into(), self.z.into(), self.w.into())
    }
}

impl AsStd140 for Matrix2<f32> {
    type Std140 = std140::mat2x2;

    fn as_std140(&self) -> Self::Std140 {
        std140::mat2x2(
            self.x.as_std140(),
            self.y.as_std140(),
        )
    }
}

impl AsStd140 for Matrix3<f32> {
    type Std140 = std140::mat3x3;

    fn as_std140(&self) -> Self::Std140 {
        std140::mat3x3(
            self.x.as_std140(),
            self.y.as_std140(),
            self.z.as_std140(),
        )
    }
}

impl AsStd140 for Matrix4<f32> {
    type Std140 = std140::mat4x4;

    fn as_std140(&self) -> Self::Std140 {
        std140::mat4x4(
            self.x.as_std140(),
            self.y.as_std140(),
            self.z.as_std140(),
            self.w.as_std140(),
        )
    }
}
