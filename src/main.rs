fn main() {
    for pet_species in PetSpecies::iter() {
        println!("{:?}", pet_species);
    }
}

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum PetSpecies {
    Cat,
    Dog,
    Fish,
    Lizard,
    Parrot,
}
