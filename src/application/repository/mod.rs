use near_sdk::{AccountId, CryptoHash, env};
use unidecode::unidecode;

pub(crate) fn convert_coure_title_to_cousrse_id(title: &str, account_id: String) -> String {
  let unaccented = unidecode(title);
  let lowercased = unaccented.to_ascii_lowercase();
  let result = account_id.to_string()  + "_" + &lowercased;

  return result.replace(" ", "_");
}

//used to generate a unique prefix in our storage collections (this is to avoid data collisions)
pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
  //get the default hash
  let mut hash = CryptoHash::default();
  //we hash the account ID and return it
  hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
  hash
}