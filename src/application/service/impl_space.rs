use near_sdk::{env, near_bindgen};

use crate::{
  application::service::convert_title_to_id_no_account,
  models::{
    contract::{ThreadScoreContract, ThreadScoreContractExt},
    space::{SpaceFeatures, SpaceId, SpaceMetadata},
    thread::ThreadId,
    user::UserId,
  },
};

#[near_bindgen]
impl SpaceFeatures for ThreadScoreContract {
  fn create_space(&mut self, space_name: String) -> SpaceMetadata {
    let space_id = convert_title_to_id_no_account(&space_name);

    assert!(!self.space_metadata_by_id.contains_key(&space_id), "Space name is existed!");

    let new_space = SpaceMetadata {
      space_id: space_id.clone(),
      space_name: space_name.clone(),
      creator_id: env::signer_account_id(),
      created_at: env::block_timestamp_ms(),
      followed_users: Vec::new(),
      total_point: 0_u64,
    };

    self.space_metadata_by_id.insert(&space_id, &new_space);
    self.space_list.insert(&space_id);
    new_space
  }

  fn get_space_metadata_by_space_id(&self, space_id: SpaceId) -> Option<SpaceMetadata> {
    self.space_metadata_by_id.get(&space_id)
  }

  fn get_all_threads_of_space_by_space_id(&self, space_id: SpaceId) -> Vec<ThreadId> {
    let thread_list = self.threads_per_space.get(&space_id);

    assert!(thread_list.is_some(), "Space not existed!");

    thread_list.unwrap().to_vec()
  }

  /// Get all the space. Current and complete space
  fn get_all_spaces(&self, from_index: Option<u32>, limit: Option<u32>) -> Vec<SpaceMetadata> {
    let mut all_space = Vec::<SpaceMetadata>::new();

    for space_id in self.space_list.iter().skip(from_index.unwrap_or(0) as usize).take(limit.unwrap_or(20) as usize) {
      all_space.push(self.space_metadata_by_id.get(&space_id).unwrap());
    }

    all_space
  }

  fn follow_space(&mut self, space_id: String) -> Option<String> {
    let user = env::signer_account_id();

    let space_metadata = self.space_metadata_by_id.get(&space_id);

    match space_metadata {
      None => assert!(false, "Space is not existed"),
      Some(mut metadata) => {
        let is_followed = metadata.followed_users.contains(&user);

        if is_followed {
          assert!(false, "You already follow this space!");
        } else {
          metadata.followed_users.push(user.clone());
          self.space_metadata_by_id.insert(&space_id, &metadata);

          let json_user_option = self.user_metadata_by_id.get(&user);

          match json_user_option {
            None => assert!(false, "User is not existed!"),
            Some(mut json_user) => {
              json_user.followed_space_list.push(space_id);
              self.user_metadata_by_id.insert(&user, &json_user);
            },
          }
        }
      },
    }

    Some(String::from("OK"))
  }

  fn get_followed_user_of_space_by_space_id(&self, space_id: SpaceId) -> Vec<UserId> {
    let space_metadata = self.space_metadata_by_id.get(&space_id);

    assert!(space_metadata.is_some(), "Space is not existed");

    space_metadata.unwrap().followed_users.to_vec()
  }
}
