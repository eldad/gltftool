use std::{fs::OpenOptions, io::Write};

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

        #[arg(short, long)]
        output_filename: Option<String>,

        #[arg(long)]
        overwrite: bool,
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

fn extract_basecolor_by_index(gltf: &Gltf, material_index: usize) -> Result<Option<&[u8]>, RuntimeError> {
    let material = gltf
        .materials()
        .nth(material_index)
        .ok_or(RuntimeError::MaterialIndexNotFound { material_index })?;

    let texture = material
        .pbr_metallic_roughness()
        .base_color_texture()
        .ok_or(RuntimeError::PbrMetallicRougnessBaseColorTextureNotFound { material_index })?
        .texture();

    let mut bytes: Option<&[u8]> = None;

    match texture.source().source() {
        gltf::image::Source::View { view, mime_type } => {
            let blob = gltf.blob.as_ref().ok_or(RuntimeError::NoGltfBlob)?;

            let begin = view.offset();
            let end = begin + view.length();

            bytes = Some(&blob[begin..end]);

            let length = end - begin;
            println!("[{mime_type}] {length} bytes");
        }
        gltf::image::Source::Uri { uri, mime_type } => match mime_type {
            Some(mime_type) => println!("[{mime_type}] {uri}"),
            None => println!("{uri}"),
        },
    };

    Ok(bytes)
}

fn extract_basecolor(gltf: &Gltf, material_name: Option<String>) -> Result<Option<&[u8]>, RuntimeError> {
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
        Action::Basecolor {
            material_name,
            output_filename,
            overwrite,
        } => {
            let maybe_bytes = extract_basecolor(&gltf, material_name)?;

            if let (Some(bytes), Some(filename)) = (maybe_bytes, output_filename) {
                let mut open_options = OpenOptions::new();
                open_options.write(true);
                if overwrite {
                    open_options.create(true).truncate(true)
                } else {
                    open_options.create_new(true)
                };

                let mut f = open_options.open(filename)?;
                f.write_all(bytes)?;
            };
        }
    }

    Ok(())
}
