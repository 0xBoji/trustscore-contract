use near_sdk::{env, near_bindgen};

use crate::models::{
  contract::{ThreadScoreContract, ThreadScoreContractExt},
  user::{JsonUser, UserId, UserMetadata, UserRoles, UserTraits},
};

#[near_bindgen]
impl UserTraits for ThreadScoreContract {
  /// Creates a new user with the provided nickname, first name, last name, and bio.
  /// The fields first_name, last_name, and bio are optional.
  fn create_user(
    &mut self,
    nickname: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    avatar: Option<String>,
    bio: Option<String>,
  ) {
    let user_id = env::signer_account_id();
    let new_nickname = match nickname {
      Some(n) => n,
      None => user_id.to_string(),
    };

    assert!(self.user_metadata_by_id.get(&user_id).is_none(), "This user already created!");

    let new_user_metadata = UserMetadata {
      user_id: user_id.clone(),
      nickname: new_nickname,
      role: UserRoles::Verified,
      first_name,
      last_name,
      bio,
      avatar,
      created_at: env::block_timestamp_ms(),
      updated_at: env::block_timestamp_ms(),
    };

    let new_json_user = JsonUser {
      user_id: user_id.clone(),
      metadata: new_user_metadata,
      threads_list: Vec::new(),
      followed_space_list: Vec::new(),
      threads_owned: 0_u32,
      total_point: 1000,
      fraud_list: Vec::new(),
      fraud_threads_owned: 0_u32,
    };

    self.user_metadata_by_id.insert(&user_id, &new_json_user);

    self.subscriber_users.insert(&user_id);
  }

  // /// Returns a `JsonUser` representation of the user's metadata for the given user ID.
  fn get_user_metadata_by_user_id(&self, user_id: UserId) -> Option<JsonUser> {
    let found_user = self.user_metadata_by_id.get(&user_id);
    found_user
  }

  // /// Update user information
  fn update_user_information(
    &mut self,
    nickname: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    bio: Option<String>,
    avatar: Option<String>,
  ) -> JsonUser {
    // Check access
    assert!(self.user_metadata_by_id.contains_key(&env::signer_account_id()), "You don't have access");

    let mut user = self.user_metadata_by_id.get(&env::signer_account_id()).unwrap();

    // Check attribute. If it have some -> update
    if let Some(n) = nickname {
      user.metadata.nickname = n
    };
    if let Some(f) = first_name {
      user.metadata.first_name = Some(f)
    }

    if let Some(l) = last_name {
      user.metadata.last_name = Some(l)
    }

    if let Some(b) = bio {
      user.metadata.bio = Some(b)
    }

    if let Some(a) = avatar {
      user.metadata.avatar = Some(a)
    }

    // Storage time information when user update
    user.metadata.updated_at = env::block_timestamp_ms();

    // Storage the change
    self.user_metadata_by_id.insert(&env::signer_account_id(), &user);

    // Return
    user
  }

  /// Get all information of users
  fn get_all_user_metadata(&self, from_index: Option<u32>, limit: Option<u32>) -> Vec<JsonUser> {
    let mut all_user = Vec::new();
    for user_id in
      self.subscriber_users.iter().skip(from_index.unwrap_or(0) as usize).take(limit.unwrap_or(20) as usize)
    {
      all_user.push(self.user_metadata_by_id.get(&user_id).unwrap());
    }
    all_user
  }

  /// Check user role
  fn check_user_role(&self, user_id: UserId) -> UserRoles {
    // check exit

    let found_user = self.user_metadata_by_id.get(&user_id);
    assert!(!found_user.is_none(), "Your account is not created!");

    found_user.unwrap().metadata.role
  }

  /// Check user role
  fn active_user_role(&mut self, user_id: UserId) -> Option<String> {
    // // check is admin
    // let user = env::signer_account_id();

    // let user_role = self.user_metadata_by_id.get(&user);

    // // TODO: temp comment, remove comment later
    // validate ADMIN role
    // match user_role {
    //   Some(json_user) => assert!(
    //     json_user.metadata.role == UserRoles::Admin,
    //     "Only admin can do this action!"
    //   ),
    //   None => assert!(false, "Your account is not created!"),
    // }

    // check exit
    match self.user_metadata_by_id.get(&user_id) {
      None => assert!(false, "User Id is not created yet!"),
      Some(mut user) => {
        user.metadata.role = UserRoles::Verified;
        self.user_metadata_by_id.insert(&user_id, &user);
      },
    }

    Some(String::from("OK"))
  }

  /// Set admin role
  fn set_admin_user_role(&mut self, user_id: UserId) -> Option<String> {
    // check is owner
    let caller = env::signer_account_id();
    let owner = self.get_owner_id();

    assert!(caller == owner, "Only owner can do this action!");

    // check exit
    match self.user_metadata_by_id.get(&user_id) {
      None => assert!(false, "User Id is not created yet!"),
      Some(mut user) => {
        user.metadata.role = UserRoles::Admin;
        self.user_metadata_by_id.insert(&user_id, &user);
      },
    }

    Some(String::from("OK"))
  }
}
