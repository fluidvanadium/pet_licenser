use crate::{equipment::PetEquipment, PetSpecies};
use getset::Getters;
use thiserror::Error;

#[derive(Getters)]
pub struct PetLicense {
    #[getset(get = "pub")]
    license_text: String,
}

#[derive(Debug, Error)]
pub enum LicensingError {
    #[error("Equipment for {0:?}. You have a {1:?}")]
    WrongSpecies(PetSpecies, PetSpecies),
    #[error("Cage is required.")]
    NoCage,
}

impl PetLicense {
    pub fn apply_for_license(
        equipment: &PetEquipment,
        pet: PetSpecies,
    ) -> Result<PetLicense, LicensingError> {
        let expected_pet_species = equipment.for_which_species();
        if expected_pet_species == pet {
            if equipment.cage().is_some() {
                Ok(PetLicense {
                    license_text:
                        "its the species you claim? and you have got a cage? good enough."
                            .to_string(),
                })
            } else {
                Err(LicensingError::NoCage)
            }
        } else {
            Err(LicensingError::WrongSpecies(expected_pet_species, pet))
        }
    }
}
