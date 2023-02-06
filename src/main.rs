use std::io::{self, Write};

use ::image::GenericImageView;
use rustemon::client::{CacheMode, Environment, RustemonClient, RustemonClientBuilder};
use rustemon::model::pokemon::PokemonSpecies;

mod image;
mod pokemon;

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

async fn draw_pokemon_sprite(
    rustemon_client: &RustemonClient,
    pokemon_species: &PokemonSpecies,
) -> Result<(), Box<dyn std::error::Error>> {
    let default_sprite_url =
        pokemon::get_pokemon_species_default_sprite_url(rustemon_client, pokemon_species)
            .await?
            .unwrap();

    let sprite = image::download_from_url(&default_sprite_url).await?;
    let sprite = image::crop_to_content(&sprite);
    let (sprite_width, sprite_height) = sprite.dimensions();

    let conf = viuer::Config {
        transparent: true,
        width: Some(sprite_width),
        height: Some(sprite_height / 2),
        ..Default::default()
    };

    viuer::print(&sprite, &conf)?;

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
    draw_pokemon_sprite(&rustemon_client, &species).await?;

    let name = pokemon::get_pokemon_species_name(&species).await;
    println!("You rolled {}!", name);

    Ok(())
}
