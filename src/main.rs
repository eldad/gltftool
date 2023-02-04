use clap::{Parser, Subcommand};
use gltf::Gltf;

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

fn show_info(gltf: Gltf) {
    println!("Materials:");
    gltf.materials()
        .flat_map(|t| t.name().map(str::to_owned))
        .for_each(|name| println!("- {name}"));
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let gltf = Gltf::open(args.gltf_filename)?;

    match args.action {
        Action::Info => show_info(gltf),
    }

    Ok(())
}
