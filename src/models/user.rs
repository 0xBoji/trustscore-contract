#![allow(clippy::too_many_arguments)]

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

use super::thread::ThreadId;

/// `UserId` is a type alias for `AccountId`, typically representing a unique identifier for a user in the system.
pub type UserId = AccountId;

#[derive(Default, BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum UserRoles {
  #[default]
  Unverified,
  Verified,
  Admin,
}

/// This struct represents a user's metadata in the system.
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UserMetadata {
  /// Unique identifier of the user.
  pub user_id: UserId,

  /// Nickname chosen by the user.
  pub nickname: String,

  /// User role of the user. Unverified as default
  pub role: UserRoles,

  /// User's first name, if provided.
  pub first_name: Option<String>,

  /// User's last name, if provided.
  pub last_name: Option<String>,

  /// Short biographical note about the user, if provided.
  pub bio: Option<String>,

  /// URL or identifier of the user's avatar image, if provided.
  pub avatar: Option<String>,

  /// Unix timestamp (in seconds) when the user account was created.
  pub created_at: u64,

  /// Unix timestamp (in seconds) when the user account was last updated.
  pub updated_at: u64,

  /// Total number of threads owned by the user.
  pub threads_owned: u32,

  /// User's total points.
  pub total_point: u32,
}

/// The `JsonUser` struct provides a comprehensive view of a user in the system.
/// It includes metadata and threads.
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonUser {
  /// Unique identifier for the user, of type `UserId`.
  pub user_id: UserId,

  /// Detailed metadata about the user, of type `UserMetadata`.
  pub metadata: UserMetadata,

  /// Map of threads associated with the user.
  pub threads: Vec<ThreadId>,
}

/// The `UserTraits` trait defines a set of behaviors associated with a user in the system.
pub trait UserTraits {
  /// Creates a new user with the provided nickname, first name, last name, and bio.
  /// The fields first_name, last_name, and bio are optional.
  fn create_user(
    &mut self,
    nickname: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    avatar: Option<String>,
    bio: Option<String>,
  );

  // /// Returns a `JsonUser` representation of the user's metadata for the given user ID.
  fn get_user_metadata_by_user_id(&self, user_id: UserId) -> Option<JsonUser>;

  // TODO: add readme.md
  // /// Update user information
  fn update_user_information(
    &mut self,
    nickname: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    bio: Option<String>,
    avatar: Option<String>,
  ) -> JsonUser;

  // TODO: add readme.md
  // /// Get all information of users
  fn get_all_user_metadata(&self, from_index: Option<u32>, limit: Option<u32>) -> Vec<JsonUser>;

  // TODO: add readme.md
  /// Check user role
  fn check_user_role(&self, user_id: UserId) -> UserRoles;

  /// Check user role
  fn active_user_role(&mut self, user_id: UserId) -> Option<JsonUser>;
}
