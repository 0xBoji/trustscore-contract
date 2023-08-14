use std::collections::HashMap;

use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  serde::{Deserialize, Serialize},
  AccountId, Balance,
};

use super::user::UserId;

/// `ThreadId` is a type alias for `String`, typically representing a unique identifier for a thread in the system.
pub type ThreadId = String;

/// The `ThreadMetadata` struct represents metadata for a thread in the system.
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ThreadMetadata {
  /// Unique identifier for the thread, of type `ThreadId`.
  pub thread_id: ThreadId,

  /// Name of the thread.
  pub title: String,

  /// Detailed description of the thread.
  pub description: Option<String>,

  /// Thumbnail of the thread
  pub media: Option<String>,

  /// Creator's account ID.
  pub creator_id: UserId,

  /// The Content of this thread
  pub content: Option<String>,

  /// Date when the thread was created, represented as a timestamp.
  pub created_at: u64,

  /// Init point of this thread, of type `U128`.
  pub init_point: Balance,

  /// Number of user currently subscribed this thread.
  pub users_map: HashMap<AccountId, u64>,

  /// Number of choices this thread has.
  pub choices_count: u8,

  /// Number of choice this thread has.
  pub choices_map: HashMap<AccountId, u8>,

  /// Count of all the ratings this thread has received.
  pub choices_rating: HashMap<u64, u8>,
}

pub trait ThreadFeatures {
  fn create_thread(
    &mut self,
    title: String,
    description: Option<String>,
    content: Option<String>,
    media: Option<String>,
    init_point: Balance,
  ) -> ThreadMetadata;

   fn get_thread_metadata_by_thread_id(&self, thread_id: ThreadId) -> Option<ThreadMetadata>;

  // /// Get all the thread per user have. Current and complete thread
  fn get_all_threads_per_user_own(
    &self,
    user_id: UserId,
    start: Option<u32>,
    limit: Option<u32>,
  ) -> Vec<ThreadMetadata>;

  // /// Check user completed thread or not
  // fn check_thread_completed(&self, thread_id: ThreadId, user_id: UserId) -> bool;
}
