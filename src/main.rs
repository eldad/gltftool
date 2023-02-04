use clap::{Parser, Subcommand};
use gltf::Gltf;
use serde::Serialize;

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

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let gltf = Gltf::open(args.gltf_filename)?;

    match args.action {
        Action::Info => show_info(gltf)?,
    }

    Ok(())
}
