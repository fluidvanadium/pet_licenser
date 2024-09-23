use crate::{equipment::PetEquipment, PetSpecies};
use thiserror::Error;

pub struct PetLicense {
    license_text: String,
}

#[derive(Debug, Error)]
pub enum LicensingError {
    #[error("Equipment for {0:?}. You have a {1:?}")]
    WrongSpecies(PetSpecies, PetSpecies),
}

impl PetLicense {
    fn apply_for_license(
        equipment: PetEquipment,
        pet: PetSpecies,
    ) -> Result<PetLicense, LicensingError> {
        let expected_pet_species = equipment.for_which_species();
        if expected_pet_species == pet {
            Ok(PetLicense {
                license_text: "same species? good enough".to_string(),
            })
        } else {
            Err(LicensingError::WrongSpecies(expected_pet_species, pet))
        }
    }
}
