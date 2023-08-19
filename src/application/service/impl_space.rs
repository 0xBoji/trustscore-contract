use near_sdk::{env, near_bindgen};

use crate::{
  application::repository::convert_title_to_id_no_account,
  models::{
    contract::{ThreadScoreContract, ThreadScoreContractExt},
    space::{SpaceFeatures, SpaceId, SpaceMetadata},
    thread::ThreadId,
  },
};

#[near_bindgen]
impl SpaceFeatures for ThreadScoreContract {
  fn create_space(&mut self, space_name: String) -> SpaceMetadata {
    let space_id = convert_title_to_id_no_account(&space_name);

    let new_space = SpaceMetadata {
      space_id: space_id.clone(),
      space_name: space_name.clone(),
      creator_id: env::signer_account_id(),
      created_at: env::block_timestamp_ms(),
    };

    self.space_metadata_by_id.insert(&space_id, &new_space);

    new_space
  }

  fn get_space_metadata_by_thread_id(&self, space_id: SpaceId) -> Option<SpaceMetadata> {
    self.space_metadata_by_id.get(&space_id)
  }

  fn get_all_threads_of_space_by_space_id(&self, space_id: SpaceId) -> Vec<ThreadId> {
    let thread_list = self.threads_per_space.get(&space_id).unwrap();
    thread_list.to_vec()
  }

  // /// Get all the space. Current and complete space
  // fn get_all_spaces(&self, start: Option<u32>, limit: Option<u32>) -> Vec<SpaceMetadata>;
}
