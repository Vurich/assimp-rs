//! # assimp - Open Asset Import Library
//!
//! Bindings for the [Assimp](http://assimp.org/) library.

#![warn(missing_docs)]

extern crate assimp_sys as ffi;

pub use import::Importer;
pub use log::LogStream;
pub use math::{Color3D, Color4D, Matrix3x3, Matrix4x4, Quaternion, Vector2D, Vector3D};
pub use scene::{
    Animation, Camera, Face, Light, Material, Mesh, Metadata, MetadataEntry, Node, NodeAnim,
    QuatKey, Scene, Texture, TextureData, Value, VectorKey,
};

#[macro_use]
mod internal_macros;

pub mod export;
pub mod import;
pub mod log;
pub mod math;
pub mod scene;

unsafe fn aistring_to_cstr(aistring: &ffi::aiString) -> &std::ffi::CStr {
    std::ffi::CStr::from_bytes_with_nul_unchecked(std::mem::transmute(
        &aistring.data[..aistring.length as usize],
    ))
}

fn str_to_aistring(val: &str) -> ffi::aiString {
    let bytes = val.as_bytes();

    assert!(bytes.len() < 1024);

    let mut data = [0u8; 1024];

    data.copy_from_slice(bytes);

    let data = unsafe { std::mem::transmute(data) };

    ffi::aiString {
        length: bytes.len() as _,
        data,
    }
}
