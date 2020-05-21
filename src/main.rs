use std::fs::File;

use console::style;
use modpackr_lib::{manifest::Manifest, r#mod::Mod};
use serde_yaml::from_reader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("./examples/testpak.yaml")?;
    let manifest: Manifest = from_reader(file)?;

    for m in manifest.mods.unwrap().curse_forge {
        match m.get_latest_for_version(manifest.minecraft.version) {
            Ok(Some(version)) => {
                println!("{:<35}: {:?}", m.name, style(version).green());
            }
            Ok(None) => {
                println!(
                    "{:<35}: {}",
                    m.name,
                    style(format!(
                        "Not available for version {}",
                        manifest.minecraft.version
                    ))
                    .red()
                );
            }
            Err(err) => println!(
                "{:<35}: {:?}",
                m.name,
                style(format!("Could not determine status: {}", err)).red()
            ),
        }
    }

    Ok(())
}
