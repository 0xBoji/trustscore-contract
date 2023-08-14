use std::collections::HashMap;

use crate::{
  application::repository::{convert_coure_title_to_cousrse_id, hash_account_id},
  models::{
    contract::{StorageKey, ThreadScoreContract, ThreadScoreContractExt},
    thread::{ThreadFeatures, ThreadId, ThreadMetadata},
    user::{UserId, UserRoles},
  },
};
use near_sdk::borsh::BorshSerialize;
use near_sdk::{collections::UnorderedSet, env, near_bindgen, Balance};

#[near_bindgen]
impl ThreadFeatures for ThreadScoreContract {
  fn create_thread(
    &mut self,
    title: String,
    description: Option<String>,
    content: Option<String>,
    media: Option<String>,
    init_point: Balance,
  ) -> ThreadMetadata {
    let creator_id = env::signer_account_id();
    let thread_id = convert_coure_title_to_cousrse_id(&title, creator_id.to_string());

    let creator_metadata = self.user_metadata_by_id.get(&creator_id);

    // TODO: validate check role
    let creator_role = match creator_metadata {
      Some(creator_metadata) => creator_metadata.metadata.role,
      None => UserRoles::NoRole,
    };

    let thread_meta = ThreadMetadata {
      thread_id: thread_id.clone(),
      title,
      description,
      media,
      creator_id: creator_id.clone(),
      content,
      init_point,
      created_at: env::block_timestamp_ms(),
      users_map: HashMap::new(),
      choices_count: 0,
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
