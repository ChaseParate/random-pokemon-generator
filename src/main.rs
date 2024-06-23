use std::io::{self, Write};

use image::{DynamicImage, GenericImageView};
use rustemon::client::{CacheMode, Environment, RustemonClientBuilder};

mod sprite;
mod pokemon;

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

async fn draw_image(sprite: &DynamicImage) -> Result<(), Box<dyn std::error::Error>> {
    let (sprite_width, sprite_height) = sprite.dimensions();

    let config = viuer::Config {
        transparent: true,
        width: Some(sprite_width),
        height: Some(sprite_height / 2),
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
        .try_build()?;

    let species = pokemon::get_random_pokemon_species(&rustemon_client).await?;

    let sprite_url =
        pokemon::get_pokemon_species_sprite_url(&rustemon_client, &species)
            .await?
            .unwrap();
    let sprite = sprite::crop_to_content(&sprite::download_from_url(&sprite_url).await?);

    clear_screen();
    draw_image(&sprite).await?;

    let name = pokemon::get_pokemon_species_name(&species);
    println!("You rolled {name}!");

    Ok(())
}
