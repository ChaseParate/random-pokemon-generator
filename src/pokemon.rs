use rand::seq::SliceRandom;
use rand::thread_rng;
use rustemon::client::RustemonClient;
use rustemon::error::Error;
use rustemon::model::pokemon::PokemonSpecies;
use rustemon::pokemon::pokemon_species;
use rustemon::Follow;

pub async fn get_random_pokemon_species(
    rustemon_client: &RustemonClient,
) -> Result<PokemonSpecies, Error> {
    let mut rng = thread_rng();
    let all_pokemon_species = pokemon_species::get_all_entries(rustemon_client).await?;

    let species_resource = all_pokemon_species.choose(&mut rng).unwrap();
    let species = species_resource.follow(rustemon_client).await?;

    Ok(species)
}

pub async fn get_pokemon_species_name(pokemon_species: &PokemonSpecies) -> String {
    let name = pokemon_species
        .names
        .iter()
        .find(|name| name.language.name == "en")
        .unwrap();

    name.name.clone()
}

pub async fn get_pokemon_species_default_sprite_url(
    rustemon_client: &RustemonClient,
    pokemon_species: &PokemonSpecies,
) -> Result<Option<String>, Error> {
    let variety = pokemon_species
        .varieties
        .iter()
        .find(|variety| variety.is_default)
        .unwrap();
    let pokemon = variety.pokemon.follow(rustemon_client).await?;

    Ok(pokemon.sprites.front_default)
}
