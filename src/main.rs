#![forbid(unsafe_code)]

fn main() {
    let mut pets = vec![];
    let mut sets_of_equipment = vec![];
    for pet_species in PetSpecies::iter() {
        // println!("{:?}", pet_species);
        let equipment = equipment::PetEquipment::assemble_default_for_species(&pet_species);
        // println!("{:?}", equipment);
        pets.push(pet_species);
        sets_of_equipment.push(equipment);
    }
    try_licensing(&pets[2], &sets_of_equipment[3]);
    try_licensing(&pets[2], &sets_of_equipment[2]);
}

pub fn try_licensing(pet: &PetSpecies, equipment: &PetEquipment) {
    match PetLicense::apply_for_license(equipment, pet.clone()) {
        Ok(license) => println!("licensing succeeded! {}", license.license_text()),
        Err(licensing_error) => println!("licensing failed with error {}", licensing_error),
    }
}
use equipment::PetEquipment;
use license::PetLicense;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Debug, EnumIter, Eq, PartialEq)]
pub enum PetSpecies {
    Cat,
    Dog,
    Fish,
    Lizard,
    Parrot,
}

pub(crate) mod equipment {
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
    pub struct Cage;

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
                    cage: None,
                }),
                PetSpecies::Dog => PetEquipment::Dog((
                    MammalEquipment {
                        collar: Collar("garfield".to_string()),
                        food: MammalFood {},
                        hair_cleaner: HairCleaner::NVacuum2000,
                        cage: Some(Cage {}),
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
        pub fn cage(&self) -> Option<&Cage> {
            match self {
                PetEquipment::Cat(equipment) => equipment.cage.as_ref(),
                PetEquipment::Dog(equipment) => equipment.0.cage.as_ref(),
                PetEquipment::Fish(equipment) => None,
                PetEquipment::Lizard(equipment) => Some(&equipment.cage),
                PetEquipment::Parrot(equipment) => Some(&equipment.cage),
            }
        }
    }
}

pub mod license;

struct LicensedPet {
    license: license::PetLicense,
    equipment: equipment::PetEquipment,
}
