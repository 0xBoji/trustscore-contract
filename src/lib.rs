pub mod application;
pub mod models;

// use application::*;
use models::contract::{
  StorageKey::{
    ContractMetadata, SpaceList, SpaceMetadataById, SubscriberUsers, ThreadMetadataById, ThreadsPerSpace,
    ThreadsPerUser, UserMetadataById,
  },
  ThreadScoreContract, ThreadScoreContractExt, ThreadScoreContractMetadata,
};
use near_sdk::{
  borsh::BorshSerialize,
  collections::{LazyOption, LookupMap, UnorderedSet},
  env, near_bindgen, AccountId,
};

#[near_bindgen]
impl ThreadScoreContract {
  #[init]
  pub fn init() -> Self {
    Self::new(
      env::signer_account_id(),
      ThreadScoreContractMetadata {
        spec:      "thread_score-0.0.1".to_string(),
        name:      "thread_score".to_string(),
        symbol:    "TS".to_string(),
        icon:      None,
        base_uri:  None,
        reference: None,
      },
    )
  }

  fn new(owner_id: AccountId, meta_data: ThreadScoreContractMetadata) -> Self {
    Self {
      owner_id,
      contract_metadata: LazyOption::new(ContractMetadata.try_to_vec().unwrap(), Some(&meta_data)),

      subscriber_users: UnorderedSet::new(SubscriberUsers.try_to_vec().unwrap()),
      user_metadata_by_id: LookupMap::new(UserMetadataById.try_to_vec().unwrap()),

      threads_per_user: LookupMap::new(ThreadsPerUser.try_to_vec().unwrap()),
      thread_metadata_by_id: LookupMap::new(ThreadMetadataById.try_to_vec().unwrap()),

      space_metadata_by_id: LookupMap::new(SpaceMetadataById.try_to_vec().unwrap()),
      threads_per_space: LookupMap::new(ThreadsPerSpace.try_to_vec().unwrap()),
      space_list: UnorderedSet::new(SpaceList.try_to_vec().unwrap()),
    }
  }

  pub fn get_me(&self) -> Option<ThreadScoreContractMetadata> {
    let x = &self.contract_metadata;
    let ret = x.get();
    ret
  }

  pub fn get_owner_id(&self) -> AccountId {
    let found_owner_id = &self.owner_id;
    found_owner_id.clone()
  }

  pub fn get_subscriber_users(&self) -> Vec<AccountId> {
    let x = &self.subscriber_users;
    // let ret = x.get();
    x.to_vec()
  }
}
