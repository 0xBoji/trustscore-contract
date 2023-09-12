use std::{collections::HashMap, u8};

use crate::{
  application::service::{convert_title_to_id, convert_title_to_id_no_account, hash_account_id, hash_space_id},
  models::{
    contract::{StorageKey, ThreadScoreContract, ThreadScoreContractExt},
    space::SpaceFeatures,
    thread::{ThreadFeatures, ThreadId, ThreadMetadata, ThreadState},
    user::UserId,
  },
};
use near_sdk::{borsh::BorshSerialize, collections::UnorderedSet, env, json_types::U64, near_bindgen, AccountId};

#[near_bindgen]
impl ThreadFeatures for ThreadScoreContract {
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
  ) -> ThreadMetadata {
    let creator_id = env::signer_account_id();

    // check option have 2
    assert!(options.len() == 2, "Vote option must be 2!");

    // check thread_mode
    assert!(thread_mode == 0_u8 || thread_mode == 1_u8, "Thread mode is not valid!");

    let mut choices_map = HashMap::<u8, String>::new();

    options.iter().enumerate().for_each(|(idx, option)| {
      choices_map.insert(idx as u8, option.to_owned());
    });

    // check user
    match self.user_metadata_by_id.get(&creator_id) {
      Some(creator_json) => {
        // assert!(creator_json.metadata.role == UserRoles::Verified, "Your account is not verified!");

        assert!(creator_json.total_point > init_point as i32, "Your trust point is not enough to create new thread!");
      },
      None => assert!(false, "Your account is not created!"),
    }

    // check thread
    let thread_id = convert_title_to_id(&title, creator_id.to_string());

    assert!(self.thread_metadata_by_id.get(&thread_id).is_none(), "This thread already created!");

    if thread_mode == 0 {
      assert!(partner_id.is_some(), "Fraud mode must have partner id!");

      // check partner
      match partner_id.clone() {
        Some(id) => match self.user_metadata_by_id.get(&id) {
          Some(partner_json) => {
            assert!(partner_json.user_id != creator_id, "Partner ID can not same as your account!");
          },
          None => (),
        },
        None => assert!(false, "Partner id is not valid!"),
      };
    }

    let thread_meta = ThreadMetadata {
      thread_id: thread_id.clone(),
      title,
      media_link,
      creator_id: creator_id.clone(),
      partner_id: partner_id.clone(),
      content,
      init_point,
      thread_mode,
      space_name: space_name.clone(),
      space_id: convert_title_to_id_no_account(&space_name),
      start_time: start_time.into(),
      end_time: end_time.into(),
      created_at: env::block_timestamp_ms(),
      choices_count: options.len() as u8,
      choices_map,
      user_votes_map: HashMap::new(),
      choices_rating: HashMap::new(),
      last_id: 0_u32,
    };

    // update thread
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

    // update space
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

    // update Total number of threads owned by the user.
    self.user_metadata_by_id.get(&creator_id);

    // update creator
    let mut new_json_creator = self.user_metadata_by_id.get(&creator_id).unwrap();
    new_json_creator.threads_owned += 1;
    // new_json_creator.total_point -= init_point;
    new_json_creator.total_point = new_json_creator.total_point.checked_sub_unsigned(init_point).unwrap();
    new_json_creator.threads_list.push(thread_id.clone());

    // update followed_space_list for user
    if new_json_creator.followed_space_list.contains(&space_id) == false {
      new_json_creator.followed_space_list.push(space_id.clone());
    }
    self.user_metadata_by_id.insert(&creator_id, &new_json_creator);

    // update followed_user for space
    let new_space_metadata = self.space_metadata_by_id.get(&space_id);
    match new_space_metadata {
      None => assert!(false, "Space is not existed!!!"),
      Some(mut meta) => {
        if !meta.followed_users.contains(&creator_id) {
          meta.followed_users.push(creator_id);

          self.space_metadata_by_id.insert(&space_id, &meta);
        }
      },
    }

    if thread_mode == 0 {
      // update partner

      match partner_id.clone() {
        Some(id) => match self.user_metadata_by_id.get(&id) {
          None => assert!(false, "Partner user is not valid!"),
          Some(mut new_json_partner) => {
            new_json_partner.fraud_threads_owned += 1;
            new_json_partner.total_point = new_json_partner.total_point.checked_sub_unsigned(init_point).unwrap();
            new_json_partner.fraud_list.push(thread_id);

            self.user_metadata_by_id.insert(&partner_id.unwrap(), &new_json_partner);
          },
        },
        None => assert!(false, "Partner id is not valid!"),
      };

      // update new point for space
      let new_space_metadata = self.space_metadata_by_id.get(&space_id);

      match new_space_metadata {
        None => assert!(false, "Space is not existed!"),
        Some(mut meta) => {
          meta.total_point = meta.total_point + (init_point as u64) * 2;
          self.space_metadata_by_id.insert(&space_id, &meta);
        },
      }
    }

    if thread_mode == 1 {
      // update new point for space
      let new_space_metadata = self.space_metadata_by_id.get(&space_id);

      match new_space_metadata {
        None => assert!(false, "Space is not existed!"),
        Some(mut meta) => {
          meta.total_point = meta.total_point + init_point as u64;
          self.space_metadata_by_id.insert(&space_id, &meta);
        },
      }
    }

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

    // let thread_array = self.threads_per_user.get(&user_id);
    match self.threads_per_user.get(&user_id) {
      None => assert!(false, "Thread list is empty!"),
      Some(thread_array) => {
        for thread_id in thread_array.iter().skip(start.unwrap_or(0_u32) as usize).take(limit.unwrap_or(5) as usize) {
          let thread_found = self.thread_metadata_by_id.get(&thread_id);
          result.push(thread_found.unwrap());
        }
      },
    }

    result
  }

  // Check thread status
  fn get_thread_status(&self, thread_id: &ThreadId) -> ThreadState {
    let thread_found = self.thread_metadata_by_id.get(&thread_id);

    assert!(thread_found.is_some(), "Thread not existed!");

    let current_time = env::block_timestamp_ms();
    let start_time = thread_found.clone().unwrap().start_time;
    let end_time = thread_found.unwrap().end_time;

    if current_time >= end_time {
      return ThreadState::Closed;
    }

    if current_time > start_time {
      return ThreadState::Open;
    }

    return ThreadState::Upcoming;
  }

  fn vote_thread(&mut self, thread_id: ThreadId, choice_number: u8, point: u32) -> Option<String> {
    let voter = env::signer_account_id();

    // check point of user > initial point
    let found_voter = self.user_metadata_by_id.get(&voter);
    assert!(found_voter.is_some(), "This user is not existed!");

    if let Some(json_user) = &found_voter {
      assert!(json_user.total_point > point as i32, "You don't have enough point!");
    }

    // check thread id valid
    let thread_found = self.thread_metadata_by_id.get(&thread_id);
    assert!(thread_found.is_some(), "Thread is not existed!");

    // check time is valid
    let cur_thread_state = self.get_thread_status(&thread_id);
    assert!(cur_thread_state != ThreadState::Upcoming, "This thread is not live yet!");
    assert!(cur_thread_state != ThreadState::Closed, "This thread is ended!");

    // check choice is valid
    if let Some(mut thread_metadata) = thread_found {
      assert!(
        point >= thread_metadata.init_point,
        "Your point must be greater than thread init_point! {}",
        thread_metadata.init_point
      );

      assert!(thread_metadata.choices_map.get(&choice_number).is_some(), "Your choice is not valid!");
      assert!(thread_metadata.creator_id != voter, "Thread creator can not vote!");

      if thread_metadata.thread_mode == 0 {
        match thread_metadata.partner_id.clone() {
          None => assert!(false, "partner_id is not valid!"),
          Some(id) => assert!(id != voter, "Thread partner can not vote!"),
        }
      }

      // update user_votes_map
      let new_user_votes_map = thread_metadata.user_votes_map.get_key_value(&voter);

      assert!(new_user_votes_map.is_none(), "This user already voted!");

      thread_metadata.user_votes_map.insert(voter.clone(), (choice_number, point));

      // update choices_rating
      // if let Some(cur_point) = thread_metadata.choices_rating.get_mut(&choice_number) {
      //   *cur_point += point;
      // }

      // update choices_rating
      let current_user_choice_rating = thread_metadata.choices_rating.get(&choice_number).unwrap_or(&0_u32);
      let new_user_choice_rating = current_user_choice_rating.checked_add(point).unwrap();

      thread_metadata.choices_rating.insert(choice_number, new_user_choice_rating);

      self.thread_metadata_by_id.insert(&thread_id, &thread_metadata);

      // update new point for space
      let new_space_metadata = self.space_metadata_by_id.get(&thread_metadata.space_id);

      match new_space_metadata {
        None => assert!(false, "Space is not existed!"),
        Some(mut meta) => {
          meta.total_point = meta.total_point + point as u64;
          self.space_metadata_by_id.insert(&thread_metadata.space_id, &meta);
        },
      }
    }

    // update new point for user
    let mut new_json_user = found_voter.unwrap();

    // new_json_user.total_point -= point;
    new_json_user.total_point = new_json_user.total_point.checked_sub_unsigned(point).unwrap();

    self.user_metadata_by_id.insert(&voter, &new_json_user);

    Some("OK".to_string())
  }

  fn end_thread(&mut self, thread_id: ThreadId) -> Option<String> {
    // check is admin
    // let user = env::signer_account_id();

    // let user_role = self.user_metadata_by_id.get(&user);

    // TODO: temp comment, remove comment later
    // match user_role {
    //   Some(json_user) => assert!(
    //     json_user.metadata.role == UserRoles::Admin,
    //     "Only admin can do this action!"
    //   ),
    //   None => assert!(false, "Your account is not created!"),
    // }

    let thread_metadata_found = self.thread_metadata_by_id.get(&thread_id);

    match thread_metadata_found {
      None => assert!(false, "Thread is not existed!"),
      Some(metadata) => {
        // check thread status
        let thread_status = self.get_thread_status(&thread_id);
        assert!(thread_status != ThreadState::Closed, "This thread is not ended!");

        // calculate which choice win
        let won_choice = find_max_pair(&metadata.choices_rating);
        assert!(won_choice.is_some(), "Won choice is not existed!");

        // calculate creator new point
        let creator = self.user_metadata_by_id.get(&metadata.creator_id);

        match creator {
          None => assert!(false, "Creator is not existed!"),
          Some(mut new_json_user) => {
            // calculate creator point * partner point
            // Mode of thread. 0 -> fraud
            if metadata.thread_mode == 0 {
              match won_choice.unwrap().0 {
                // 0 ~ creator
                0_u8 => {
                  new_json_user.total_point =
                    new_json_user.total_point + metadata.init_point.checked_mul(2).unwrap_or(0_u32) as i32;

                  self.user_metadata_by_id.insert(&metadata.creator_id, &new_json_user);
                },
                // 1 ~ partner
                1_u8 => {
                  let mut partner_metadata =
                    self.user_metadata_by_id.get(&metadata.partner_id.clone().unwrap()).unwrap();

                  partner_metadata.total_point =
                    partner_metadata.total_point + metadata.init_point.checked_mul(2).unwrap_or(0_u32) as i32;
                  self.user_metadata_by_id.insert(&metadata.partner_id.unwrap(), &partner_metadata);
                },
                _ => (),
              };
            }

            // calculate creator point
            // Mode of thread. 1 -> simple
            if metadata.thread_mode == 1 {
              let new_creator_point = match won_choice.unwrap().0 {
                // 0 ~ No
                0_u8 => new_json_user.total_point,
                // 1 ~ Yes
                1_u8 => new_json_user.total_point + metadata.init_point.checked_mul(2).unwrap_or(0_u32) as i32,
                _ => new_json_user.total_point,
              };

              new_json_user.total_point = new_creator_point;

              self.user_metadata_by_id.insert(&metadata.creator_id, &new_json_user);
            }
          },
        }

        // calc total point rating
        // let total_choices_rating: u32 = metadata.choices_rating.iter().map(|(_, r)| *r).sum();

        // calculate voter point
        for (voter_id, (choice, point)) in metadata.user_votes_map {
          let voter = self.user_metadata_by_id.get(&voter_id);

          match voter {
            None => assert!(false, "Voter is not existed!"),
            Some(mut new_json_user) => {
              let new_point = match choice {
                // 0 ~ No
                0_u8 => {
                  new_json_user.total_point + point as i32
                    - point
                      .checked_div(*metadata.choices_rating.get(&0_u8).unwrap())
                      .unwrap()
                      .checked_mul(metadata.init_point)
                      .unwrap() as i32
                },
                // 1 ~ Yes
                1_u8 => {
                  new_json_user.total_point
                    + point as i32
                    + point
                      .checked_div(*metadata.choices_rating.get(&1_u8).unwrap())
                      .unwrap()
                      .checked_mul(metadata.init_point)
                      .unwrap() as i32
                },
                // 2 ~ Draw
                2_u8 => new_json_user.total_point + point as i32,
                // _ ~ Other backup cover
                _ => new_json_user.total_point,
              };

              new_json_user.total_point = new_point;

              self.user_metadata_by_id.insert(&voter_id, &new_json_user);
            },
          }
        }
      },
    }

    // 100

    Some("OK".to_string())
  }
}

fn find_max_pair(choices_rating: &HashMap<u8, u32>) -> Option<(u8, u32)> {
  choices_rating.iter().map(|(&choice, &rating)| (choice, rating)).max_by_key(|(_, rating)| *rating)
}
