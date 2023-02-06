use rustemon::client::{CacheMode, Environment, RustemonClientBuilder};

mod pokemon;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rustemon_client = RustemonClientBuilder::default()
        .with_environment(Environment::Production)
        .with_mode(CacheMode::ForceCache)
        .try_build()?;

    let species = pokemon::get_random_pokemon_species(&rustemon_client).await?;
    let name = pokemon::get_pokemon_species_name(&species).await;
    println!("{}", name);

    let default_sprite_url =
        pokemon::get_pokemon_species_default_sprite_url(&rustemon_client, &species).await?;
    println!("{}", default_sprite_url.unwrap());

    Ok(())
}
