use goodmorning_bindings::{
    services::v1::V1Error,
    structs::{ProfileAccount, ProfileCustomisable},
};

use crate::structs::Account;

pub fn validate_profile(
    profile: &ProfileCustomisable,
    details_max: usize,
    desc_max: usize,
) -> Result<(), V1Error> {
    if profile.details.len() > details_max {
        return Err(V1Error::TooManyProfileDetails);
    }

    if profile.description.len() > desc_max {
        return Err(V1Error::ExceedsMaximumLength);
    }

    for (i, detail) in profile.details.iter().enumerate() {
        if !detail.validate() {
            return Err(V1Error::InvalidDetail { index: i as u8 });
        }
    }

    Ok(())
}

pub fn to_profile_acccount(account: Account) -> ProfileAccount {
    ProfileAccount {
        id: account.id,
        username: account.username,
        verified: account.verified,
        created: account.created,
        status: account.status,
    }
}
