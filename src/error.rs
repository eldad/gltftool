use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("No material name specified and more than one material found")]
    NoMaterialNameMoreThanOneMaterial,
    #[error("No materials found")]
    NoMaterials,
    #[error("Material '{material_name}' not found")]
    MaterialNotFound { material_name: String },
    #[error("Texture at index {texture_index} was not found")]
    TextureIndexNotFound { texture_index: usize },
    #[error("No gltf blob")]
    NoGltfBlob,
    #[error("io {0}")]
    StdIo(#[from] std::io::Error),
}
