pub mod impl_space;
pub mod impl_thread;
pub mod impl_user;

use near_sdk::{env, AccountId, CryptoHash};
use unidecode::unidecode;

use crate::models::space::SpaceId;

pub(crate) fn convert_title_to_id(title: &str, account_id: String) -> String {
  let unaccented = unidecode(title);
  let lowercased = unaccented.to_ascii_lowercase();
  let result = account_id.to_string() + "_" + &lowercased;

  return result.replace(" ", "_");
}

pub(crate) fn convert_title_to_id_no_account(title: &str) -> String {
  let unaccented = unidecode(title);
  let lowercased = unaccented.to_ascii_lowercase();

  return lowercased.replace(" ", "_");
}

// used to generate a unique prefix in our storage collections (this is to avoid data collisions)
pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
  // get the default hash
  let mut hash = CryptoHash::default();
  // we hash the account ID and return it
  hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
  hash
}

// used to generate a unique prefix in our storage collections (this is to avoid data collisions)
pub(crate) fn hash_space_id(space_id: &SpaceId) -> CryptoHash {
  // get the default hash
  let mut hash = CryptoHash::default();
  // we hash the account ID and return it
  hash.copy_from_slice(&env::sha256(space_id.as_bytes()));
  hash
}
