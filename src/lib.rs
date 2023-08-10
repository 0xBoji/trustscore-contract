pub mod application;
pub mod models;

// use application::*;
use models::contract::{ThreadScoreContract, ThreadScoreContractExt, ThreadScoreContractMetadata};
use near_sdk::{env, near_bindgen, AccountId};

#[near_bindgen]
impl ThreadScoreContract {
  #[init]
  pub fn init() -> Self {
    Self::new(
      env::signer_account_id(),
      ThreadScoreContractMetadata {
        spec: "thread_score-v0.1".to_string(),
        name: "thread_score".to_string(),
        symbol: "TS".to_string(),
        icon: None,
        base_uri: None,
        reference: None,
      },
    )
  }

  fn new(owner_id: AccountId, meta_data: ThreadScoreContractMetadata) -> Self {
    Self {
      owner_id,
      metadata_contract: todo!(),
      user_metadata_by_id: todo!(),
      threads_per_user: todo!(),
      thread_metadata_by_id: todo!(),
      subscriber_users: todo!(),
    }
  }
}
