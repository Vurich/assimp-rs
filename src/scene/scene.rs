use ffi::*;

use std::ptr::NonNull;

// Import all types
use super::animation::*;
use super::camera::*;
use super::light::*;
use super::material::*;
use super::mesh::*;
use super::node::*;
use super::texture::*;

/// The top-level scene type. This contains all the data in the imported file, such as
/// individual meshes, bones for skeletal animation, cameras, lights, and a node
/// heirarchy to organize all of these elements.
pub struct Scene<'a>(&'a aiScene);

impl Scene<'_> {
    /// Create a scene from a raw pointer to an original `aiScene` struct from the
    /// source library.
    pub unsafe fn from_raw(inner: NonNull<aiScene>) -> Self {
        Self(&*inner.as_ptr())
    }
}

impl std::ops::Deref for Scene<'_> {
    type Target = aiScene;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl Scene<'_> {
    /// Returns true if the imported scene is not complete.
    pub fn is_incomplete(&self) -> bool {
        self.mFlags & AI_SCENE_FLAGS_INCOMPLETE != 0
    }

    /// Returns true if the imported scene was successfully validated by the
    /// `validate_data_structure` post-process step.
    pub fn is_validated(&self) -> bool {
        self.mFlags & AI_SCENE_FLAGS_VALIDATED != 0
    }

    /// Returns true if any warnings were generated by the `validate_data_structure`
    /// post-process step. The details of the warnings are written to the output log.
    pub fn has_validation_warning(&self) -> bool {
        self.mFlags & AI_SCENE_FLAGS_VALIDATION_WARNING != 0
    }

    /// Returns true if the `join_identical_vertices` post-process step was run.
    pub fn is_non_verbose_format(&self) -> bool {
        self.mFlags & AI_SCENE_FLAGS_NON_VERBOSE_FORMAT != 0
    }

    /// Returns true if the imported mesh contained height-map terrain data.
    pub fn is_terrain(&self) -> bool {
        self.mFlags & AI_SCENE_FLAGS_TERRAIN != 0
    }

    /// Returns the root node of the scene hierarchy
    pub fn root_node(&self) -> Option<&Node> {
        unsafe { Some(Node::from_raw(NonNull::new(self.mRootNode)?)) }
    }

    /// Returns the number of meshes in the scene.
    pub fn num_meshes(&self) -> u32 {
        self.mNumMeshes
    }

    /// Returns an iterator over all the meshes in the scene.
    pub fn mesh_iter(&self) -> MeshIter {
        MeshIter::new(
            NonNull::new(self.mMeshes as *mut *const aiMesh),
            self.mNumMeshes as usize,
        )
    }

    /// Return an individual mesh from the scene.
    pub fn mesh(&self, id: usize) -> Option<&Mesh> {
        if id < self.mNumMeshes as usize {
            unsafe {
                Some(Mesh::from_raw(NonNull::new(
                    *(NonNull::new(self.mMeshes)?.as_ptr().offset(id as isize)),
                )?))
            }
        } else {
            None
        }
    }

    /// Returns the number of materials in the scene.
    pub fn num_materials(&self) -> u32 {
        self.mNumMaterials
    }

    /// Returns an iterator over all the materials in the scene.
    pub fn material_iter(&self) -> MaterialIter {
        MaterialIter::new(
            NonNull::new(self.mMaterials as *mut *const aiMaterial),
            self.mNumMaterials as usize,
        )
    }

    /// Returns the number of animations in the scene.
    pub fn num_animations(&self) -> u32 {
        self.mNumAnimations
    }

    /// Returns an iterator over all the animations in the scene.
    pub fn animation_iter(&self) -> AnimationIter {
        AnimationIter::new(
            NonNull::new(self.mAnimations as *mut *const aiAnimation),
            self.mNumAnimations as usize,
        )
    }

    /// Return an individual animation from the scene.
    pub fn animation(&self, id: usize) -> Option<&Animation> {
        if id < self.mNumAnimations as usize {
            unsafe {
                Some(Animation::from_raw(NonNull::new(
                    *(NonNull::new(self.mAnimations)?.as_ptr().offset(id as isize)),
                )?))
            }
        } else {
            None
        }
    }

    /// Returns the number of animations in the scene.
    pub fn num_textures(&self) -> u32 {
        self.mNumTextures
    }

    /// Returns an iterator over all the textures in the scene, if any.
    pub fn texture_iter(&self) -> TextureIter {
        TextureIter::new(
            NonNull::new(self.mTextures as *mut *const aiTexture),
            self.mNumTextures as usize,
        )
    }

    /// Returns the number of lights in the scene.
    pub fn num_lights(&self) -> u32 {
        self.mNumLights
    }

    /// Returns an iterator over all the lights in the scene.
    pub fn light_iter(&self) -> LightIter {
        LightIter::new(
            NonNull::new(self.mLights as *mut *const aiLight),
            self.mNumLights as usize,
        )
    }

    /// Returns the number of cameras in the scene.
    pub fn num_cameras(&self) -> u32 {
        self.mNumCameras
    }

    /// Returns an iterator over all the cameras in the scene.
    pub fn camera_iter(&self) -> CameraIter {
        CameraIter::new(
            NonNull::new(self.mCameras as *mut *const aiCamera),
            self.mNumCameras as usize,
        )
    }
}

// Drop implementation for a scene owned by Assimp.
// Scenes returned by aiImportFile* methods must be freed with aiReleaseImport.
impl Drop for Scene<'_> {
    fn drop(&mut self) {
        unsafe {
            aiReleaseImport(self.0);
        }
    }
}
