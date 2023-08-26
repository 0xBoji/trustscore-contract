use std::collections::HashMap;

use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  json_types::U64,
  serde::{Deserialize, Serialize},
  AccountId, Timestamp,
};

use super::user::UserId;

/// `ThreadId` is a type alias for `String`, typically representing a unique identifier for a thread
/// in the system.
pub type ThreadId = String;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum ThreadState {
  Closed,
  Open,
  Upcoming,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ThreadVote {
  thread_id: ThreadId,
  choice: u8,
  created_at: Timestamp,
  voter: UserId,
}
/// The `ThreadMetadata` struct represents metadata for a thread in the system.
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ThreadMetadata {
  /// Unique identifier for the thread, of type `ThreadId`.
  pub thread_id: ThreadId,

  /// Name of the thread.
  pub title: String,

  /// media_link of the thread
  pub media_link: Option<String>,

  /// Mode of thread. 0 -> fraud, 1 -> simple
  pub thread_mode: u8,

  /// Creator's account ID.
  pub creator_id: UserId,

  /// Partner's account ID.
  pub partner_id: Option<UserId>,

  /// The Content of this thread
  pub content: Option<String>,

  /// Date when the thread was created, represented as a timestamp.
  pub created_at: u64,

  /// Init point of this thread.
  pub init_point: u32,

  /// space_name of this thread
  pub space_name: String,

  /// space_name of this thread
  pub space_id: String,

  // /// Number of user currently subscribed this thread.
  // pub users_map: UnorderedSet<UserId>,
  pub last_id: u32,

  /// Number of choices this thread has.
  pub choices_count: u8,

  /// Number of choice this thread has.
  pub choices_map: HashMap<u8, String>,

  /// Number of choice this thread has.
  pub user_votes_map: HashMap<AccountId, (u8, u32)>,

  /// Count of all the ratings this thread has received.
  pub choices_rating: HashMap<u8, u32>,

  pub start_time: Timestamp,

  pub end_time: Timestamp,
}

pub trait ThreadFeatures {
  fn create_thread(
    &mut self,
    title: String,
    content: Option<String>,
    media_link: Option<String>,
    init_point: u32,
    partner_id: Option<AccountId>,
    thread_mode: u8,
    space_name: String,
    start_time: U64,
    end_time: U64,
    options: Vec<String>,
  ) -> ThreadMetadata;

  fn get_thread_metadata_by_thread_id(&self, thread_id: ThreadId) -> Option<ThreadMetadata>;

  // /// Get all the thread per user have. Current and complete thread
  fn get_all_threads_per_user_own(
    &self,
    user_id: UserId,
    start: Option<u32>,
    limit: Option<u32>,
  ) -> Vec<ThreadMetadata>;

  // /// Check thread status
  fn get_thread_status(&self, thread_id: &ThreadId) -> ThreadState;

  fn vote_thread(&mut self, thread_id: ThreadId, choice: u8, point: u32) -> Option<String>;

  fn end_thread(&mut self, thread_id: ThreadId) -> Option<String>;
}
