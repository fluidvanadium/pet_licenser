fn main() {
    for pet_species in PetSpecies::iter() {
        println!("{:?}", pet_species);
        let equipment = equipment::PetEquipment::assemble_default_for_species(&pet_species);
        println!("{:?}", equipment);
    }
}

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Eq, PartialEq)]
enum PetSpecies {
    Cat,
    Dog,
    Fish,
    Lizard,
    Parrot,
}

pub mod equipment {
    use crate::PetSpecies;

    #[derive(Debug)]
    struct Collar(String);
    #[derive(Debug)]
    struct MammalFood;
    #[derive(Debug)]
    enum HairCleaner {
        NVacuum2000,
    }
    #[derive(Debug)]
    struct Cage;

    #[derive(Debug)]
    struct MammalEquipment {
        collar: Collar,
        food: MammalFood,
        hair_cleaner: HairCleaner,
        cage: Option<Cage>,
    }

    #[derive(Debug)]
    struct Leash;

    #[derive(Debug)]
    struct HeatLamp(usize);

    #[derive(Debug)]
    struct ReptileEquipment {
        cage: Cage,
        heat_lamp: HeatLamp,
    }

    #[derive(Debug)]
    struct FishBowl;

    #[derive(Debug)]
    struct FishFood(i32);

    #[derive(Debug)]
    struct FishEquipment {
        bowl: FishBowl,
        food: FishFood,
    }

    #[derive(Debug)]
    struct BirdEquipment {
        cage: Cage,
    }

    #[derive(Debug)]
    pub enum PetEquipment {
        Cat(MammalEquipment),
        Dog((MammalEquipment, Leash)),
        Fish(FishEquipment),
        Lizard(ReptileEquipment),
        Parrot(BirdEquipment),
    }

    impl PetEquipment {
        pub fn assemble_default_for_species(pet_species: &PetSpecies) -> Self {
            match pet_species {
                PetSpecies::Cat => PetEquipment::Cat(MammalEquipment {
                    collar: Collar("garfield".to_string()),
                    food: MammalFood {},
                    hair_cleaner: HairCleaner::NVacuum2000,
                    cage: Some(Cage {}),
                }),
                PetSpecies::Dog => PetEquipment::Dog((
                    MammalEquipment {
                        collar: Collar("garfield".to_string()),
                        food: MammalFood {},
                        hair_cleaner: HairCleaner::NVacuum2000,
                        cage: None,
                    },
                    Leash {},
                )),
                PetSpecies::Fish => PetEquipment::Fish(FishEquipment {
                    bowl: FishBowl {},
                    food: FishFood(10),
                }),
                PetSpecies::Lizard => PetEquipment::Lizard(ReptileEquipment {
                    cage: Cage {},
                    heat_lamp: HeatLamp(60_000),
                }),
                PetSpecies::Parrot => PetEquipment::Parrot(BirdEquipment { cage: Cage {} }),
            }
        }
        pub fn for_which_species(&self) -> PetSpecies {
            match self {
                PetEquipment::Cat(_) => PetSpecies::Cat,
                PetEquipment::Dog(_) => PetSpecies::Dog,
                PetEquipment::Fish(_) => PetSpecies::Fish,
                PetEquipment::Lizard(_) => PetSpecies::Lizard,
                PetEquipment::Parrot(_) => PetSpecies::Parrot,
            }
        }
    }
}

pub mod license;

struct LicensedPet {
    license: license::PetLicense,
    equipment: equipment::PetEquipment,
}
