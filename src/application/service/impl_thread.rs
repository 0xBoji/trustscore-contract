use std::collections::HashMap;

use crate::{
  application::repository::{convert_title_to_id, convert_title_to_id_no_account, hash_account_id, hash_space_id},
  models::{
    contract::{StorageKey, ThreadScoreContract, ThreadScoreContractExt},
    space::SpaceFeatures,
    thread::{ThreadFeatures, ThreadId, ThreadMetadata},
    user::{UserId, UserRoles},
  },
};
use near_sdk::{borsh::BorshSerialize, json_types::U64};
use near_sdk::{collections::UnorderedSet, env, near_bindgen, Balance};

#[near_bindgen]
impl ThreadFeatures for ThreadScoreContract {
  fn create_thread(
    &mut self,
    title: String,
    content: Option<String>,
    media_link: Option<String>,
    init_point: Balance,
    space_name: String,
    start_time: U64,
    end_time: U64,
  ) -> ThreadMetadata {
    let creator_id = env::signer_account_id();
    let thread_id = convert_title_to_id(&title, creator_id.to_string());

    let creator_metadata = self.user_metadata_by_id.get(&creator_id);
    
    assert!(creator_metadata.is_some(), "Your account is not created!");

    // TODO: validate check role
    let creator_role = creator_metadata.unwrap().metadata.role;

    assert!(creator_role == UserRoles::Verified, "Your account is not verified!");

    assert!(self.thread_metadata_by_id.get(&thread_id).is_none(), "This thread already created!");

    let thread_meta = ThreadMetadata {
      thread_id: thread_id.clone(),
      title,
      media_link,
      creator_id: creator_id.clone(),
      content,
      init_point,
      space_name: space_name.clone(),
      start_time: start_time.into(),
      end_time: end_time.into(),
      created_at: env::block_timestamp_ms(),
      users_map: HashMap::new(),
      choices_count: 2,
      choices_map: HashMap::new(),
      choices_rating: HashMap::new(),
    };

    let init_new_user_threads_list: UnorderedSet<String> = UnorderedSet::new(
      StorageKey::ThreadsPerUserInner { account_id_hash: hash_account_id(&creator_id) }.try_to_vec().unwrap(),
    );

    let mut new_user_threads_list = if let Some(user_threads_list) = self.threads_per_user.get(&creator_id) {
      user_threads_list
    } else {
      init_new_user_threads_list
    };

    new_user_threads_list.insert(&thread_id);

    self.threads_per_user.insert(&creator_id, &new_user_threads_list);

    self.thread_metadata_by_id.insert(&thread_id, &thread_meta);

    let space_id = convert_title_to_id_no_account(&space_name);
    let is_space_id_exists = self.space_metadata_by_id.contains_key(&space_id);

    if !is_space_id_exists {
      self.create_space(space_name);
    }

    let init_new_space_threads_list: UnorderedSet<String> = UnorderedSet::new(
      StorageKey::ThreadsPerSpaceInner { space_id_hash: hash_space_id(&space_id) }.try_to_vec().unwrap(),
    );

    let mut new_space_threads_list = if let Some(space_threads_list) = self.threads_per_space.get(&space_id) {
      space_threads_list
    } else {
      init_new_space_threads_list
    };

    new_space_threads_list.insert(&thread_id);

    self.threads_per_space.insert(&space_id, &new_space_threads_list);

    thread_meta
  }

  fn get_thread_metadata_by_thread_id(&self, thread_id: ThreadId) -> Option<ThreadMetadata> {
    let found_thread = self.thread_metadata_by_id.get(&thread_id);
    found_thread
  }

  /// Get all the thread per user have. Current and complete thread
  fn get_all_threads_per_user_own(
    &self,
    user_id: UserId,
    start: Option<u32>,
    limit: Option<u32>,
  ) -> Vec<ThreadMetadata> {
    let mut result: Vec<ThreadMetadata> = Vec::new();
    let thread_array = self.user_metadata_by_id.get(&user_id).unwrap().threads;

    // println!("{start} {limit}");

    for t in thread_array {
      let thread_found = self.thread_metadata_by_id.get(&t);
      result.push(thread_found.unwrap());
    }

    result
  }

  // /// Check user completed thread or not
  // fn check_thread_completed(&self, thread_id: ThreadId, user_id: UserId) -> bool;
}
