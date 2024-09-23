fn main() {
    for pet_species in PetSpecies::iter() {
        println!("{:?}", pet_species);
    }
}

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
enum PetSpecies {
    Cat,
    Dog,
    Fish,
    Lizard,
    Parrot,
}

#[derive(Debug)]
struct Collar(String);
#[derive(Debug)]
struct MammalFood;
#[derive(Debug)]
enum HairCleaner {}
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
enum PetEquipment {
    Cat(MammalEquipment),
    Dog((MammalEquipment, Leash)),
    Fish(FishEquipment),
    Lizard(ReptileEquipment),
    Parrot(BirdEquipment),
}

struct PetLicense(String);

struct LicensedPet {
    license: PetLicense,
    equipment: PetSpecies,
}
