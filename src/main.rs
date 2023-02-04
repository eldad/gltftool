use clap::{Parser, Subcommand};
use error::RuntimeError;
use gltf::Gltf;
use serde::Serialize;

mod error;

/// gltf tool
///
/// GLTF CLI
#[derive(Parser)]
#[clap(author = "Eldad Zack <eldad@fogrefinery.com>", version, about)]
struct Args {
    /// Command
    #[command(subcommand)]
    action: Action,

    /// GLTF/GLB filename
    #[arg()]
    gltf_filename: String,
}

#[derive(Subcommand)]
enum Action {
    /// Show info
    Info,

    /// Extract basecolor texture from a Metallic-Roughness Material
    Basecolor {
        #[arg()]
        material_name: Option<String>,
    },
}

#[derive(Serialize, Debug)]
struct GltfInfo {
    material_names: Vec<String>,
    images_names: Vec<String>,
    meshes_names: Vec<String>,
    texture_names: Vec<String>,
}

impl From<Gltf> for GltfInfo {
    fn from(gltf: Gltf) -> Self {
        let material_names: Vec<String> = gltf.materials().flat_map(|t| t.name().map(str::to_owned)).collect();
        let images_names: Vec<String> = gltf.images().flat_map(|t| t.name().map(str::to_owned)).collect();
        let meshes_names: Vec<String> = gltf.meshes().flat_map(|t| t.name().map(str::to_owned)).collect();
        let texture_names: Vec<String> = gltf.textures().flat_map(|t| t.name().map(str::to_owned)).collect();

        Self {
            material_names,
            images_names,
            meshes_names,
            texture_names,
        }
    }
}

fn show_info(gltf: Gltf) -> anyhow::Result<()> {
    let info: GltfInfo = gltf.into();
    let yaml = serde_yaml::to_string(&info)?;
    println!("{yaml}");
    Ok(())
}

fn extract_basecolor_by_index(gltf: Gltf, texture_index: usize) -> Result<(), RuntimeError> {
    let image_index = gltf.textures().nth(texture_index).ok_or( RuntimeError::TextureIndexNotFound { texture_index })?.index();

    println!("image index {image_index}");

    Ok(())
}

fn extract_basecolor(gltf: Gltf, material_name: Option<String>) -> Result<(), RuntimeError> {
    match (material_name, gltf.materials().len()) {
        (Some(material_name), _) => gltf
            .materials()
            .find(|m| m.name().map(|name| name == material_name).unwrap_or(false))
            .and_then(|m| m.index())
            .map(|index| extract_basecolor_by_index(gltf, index))
            .unwrap_or_else(|| Err(RuntimeError::MaterialNotFound { material_name })),
        (None, 1) => extract_basecolor_by_index(gltf, 0),
        (None, 2..) => Err(RuntimeError::NoMaterialNameMoreThanOneMaterial),
        (None, _) => Err(RuntimeError::NoMaterials),
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let gltf = Gltf::open(args.gltf_filename)?;

    match args.action {
        Action::Info => show_info(gltf)?,
        Action::Basecolor { material_name } => extract_basecolor(gltf, material_name)?,
    }

    Ok(())
}
