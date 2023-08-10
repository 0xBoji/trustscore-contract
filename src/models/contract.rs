use std::thread::ThreadId;

use near_sdk::{
  collections::{LazyOption, LookupMap, UnorderedSet},
  near_bindgen,
  serde::{Deserialize, Serialize},
  AccountId, PanicOnDefault,
};

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use super::{
  thread::ThreadMetadata,
  user::{JsonUser, UserId},
};

/// The `ThreadScoreContractMetadata` struct represents metadata for an ThreadScore contract.
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ThreadScoreContractMetadata {
  /// Specification associated with the ThreadScore contract.
  pub spec: String,

  /// Name of the ThreadScore contract.
  pub name: String,

  /// Symbol associated with the ThreadScore contract.
  pub symbol: String,

  /// Optional icon for the ThreadScore contract.
  pub icon: Option<String>,

  /// Optional base URI for the ThreadScore contract.
  pub base_uri: Option<String>,

  /// Optional reference string for the ThreadScore contract.
  pub reference: Option<String>,
}

/// The `ThreadScoreContract` struct represents an ThreadScore Contract in the system.
#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct ThreadScoreContract {
  /// Account ID of the owner of the contract.
  pub owner_id: AccountId,

  /// Metadata associated with the ThreadScore contract.
  pub metadata_contract: LazyOption<ThreadScoreContractMetadata>,

  /// Users
  /// Storage all user_id of subscriber users -> For count all of users in the system
  pub subscriber_users: UnorderedSet<UserId>,

  /// Map of `JsonUser` metadata by user ID.
  pub user_metadata_by_id: LookupMap<UserId, JsonUser>,

  /// Threads
  /// Map of thread sets by user ID.
  pub threads_per_user: LookupMap<UserId, UnorderedSet<ThreadId>>,

  /// Map of `ThreadMetadata` by Thread ID.
  pub thread_metadata_by_id: LookupMap<ThreadId, ThreadMetadata>,
}