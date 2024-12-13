use std::io::{self, Write};

use image::DynamicImage;
use rustemon::client::{CACacheManager, CacheMode, Environment, RustemonClientBuilder};

mod pokemon;
mod sprite;

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

async fn draw_image(sprite: &DynamicImage) -> Result<(), Box<dyn std::error::Error>> {
    let config = viuer::Config {
        transparent: true,
        ..Default::default()
    };

    viuer::print(&sprite, &config)?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rustemon_client = RustemonClientBuilder::default()
        .with_environment(Environment::Production)
        .with_mode(CacheMode::ForceCache)
        .with_manager(CACacheManager::default())
        .try_build()?;

    let species = pokemon::get_random_pokemon_species(&rustemon_client).await?;

    let sprite_url = pokemon::get_pokemon_species_sprite_url(&rustemon_client, &species)
        .await?
        .unwrap();
    let sprite = sprite::crop_to_content(&sprite::download_from_url(&sprite_url).await?);

    clear_screen();
    draw_image(&sprite).await?;

    println!(
        "You rolled {}!",
        pokemon::get_pokemon_species_name(&species)
    );

    Ok(())
}
