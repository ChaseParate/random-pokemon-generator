use std::io::{self, Write};

use ::image::GenericImageView;
use rustemon::client::{CacheMode, Environment, RustemonClientBuilder};

mod image;
mod pokemon;

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

async fn draw_image(image_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let image = image::download_from_url(image_url).await?;
    let image = image::crop_to_content(&image);
    let (sprite_width, sprite_height) = image.dimensions();

    let config = viuer::Config {
        transparent: true,
        width: Some(sprite_width),
        height: Some(sprite_height / 2),
        ..Default::default()
    };

    viuer::print(&image, &config)?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rustemon_client = RustemonClientBuilder::default()
        .with_environment(Environment::Production)
        .with_mode(CacheMode::ForceCache)
        .try_build()?;

    let species = pokemon::get_random_pokemon_species(&rustemon_client).await?;

    clear_screen();

    let default_sprite_url =
        pokemon::get_pokemon_species_default_sprite_url(&rustemon_client, &species)
            .await?
            .unwrap();

    draw_image(&default_sprite_url).await?;

    let name = pokemon::get_pokemon_species_name(&species).await;
    println!("You rolled {}!", name);

    Ok(())
}
