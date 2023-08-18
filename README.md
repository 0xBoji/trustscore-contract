### Step to run local

- clean up after update code
`cm clean`

- build lại contract 
`cm build`

- deploy lại contract
`cm dev-deploy`

- init contract (bắt buộc)
`cm call-self init`

- re check lại init ok không
`cm call --account_id gmfam.testnet get_me`
  ```
  {
    spec: 'thread_score-0.0.1',
    name: 'thread_score',
    symbol: 'TS',
    icon: null,
    base_uri: null,
    reference: null
  }
  ```
- get owner id
`cm call --account_id gmfam.testnet get_owner_id`
  ```
  dev-1691914103143-32544821462759
  ```
- get array danh sách id all user (sau khi đã đăng ký vào TS)
`cm call --account_id gmfam.testnet get_subscriber_users`

  ```
  []
  ```
- tạo 1 user trên hệ thống TS
`cm call create_user '{"nickname":"long"}' --account_id gmfam.testnet`
  ```
  ' '
  ```
- get info của 1 user by user_id
`cm call get_user_metadata_by_user_id '{"user_id":"gmfam.testnet"}' --account_id gmfam.testnet`
  ```
  {
    user_id: 'gmfam.testnet',
    metadata: {
      user_id: 'gmfam.testnet',
      nickname: 'gmfam',
      role: 'Unverified',
      first_name: null,
      last_name: null,
      bio: null,
      avatar: null,
      created_at: 1691910372793,
      updated_at: 1691910372793,
      threads_owned: 0,
      total_point: 1000
    },
    threads: []
  }
  ```
- tạo 1 thread mới
`cm call create_thread '{"title":"thread title 01", "space_name":"cryto", "description":"thread desc 01",  "init_point": 9000000000000, "start_time": "1692351435321", "end_time": "1692352310056"}' --account_id gmfam.testnet`

  ```
  {
    thread_id: 'gmfam.testnet_thread_title_01',
    title: 'thread title 01',
    description: 'thread desc 01',
    media: null,
    creator_id: 'gmfam.testnet',
    content: null,
    created_at: 1691910188430,
    init_point: 9000000000000,
    users_map: {},
    choices_count: 0,
    choices_map: {},
    choices_rating: {}
  }
  ```
- lấy info của 1 thread by thread_id
`cm call get_thread_metadata_by_thread_id '{"thread_id":"gmfam.testnet_thread_title_01"}' --account_id gmfam.testnet`

  ```
  {
    thread_id: 'gmfam.testnet_thread_title_01',
    title: 'thread title 01',
    media_link: null,
    creator_id: 'gmfam.testnet',
    content: null,
    created_at: 1692352351446,
    init_point: 9000000000000,
    space_name: 'cryto',
    users_map: {},
    choices_count: 2,
    choices_map: {},
    choices_rating: {},
    start_time: 1692351435321,
    end_time: 1692352310056
  }
  ```
- get info của 1 space
`cm call get_space_metadata_by_thread_id '{"space_id":"cryto"}' --account_id gmfam.testnet`
  ```{
    space_id: 'cryto',
    space_name: 'cryto',
    creator_id: 'gmfam.testnet',
    created_at: 1692352351446,
    threads_count: 0, // đoạn này mới làm, bị lỗi đang debug
    threads: [] //
  }
  ```

### Requirements
+ Cargo Make
`cargo install cargo-make`    
`cargo install cargo-generate`

+ Install Near Cli
`npm install -g near-cli`

+ Prepare
`cargo make prepare`

+ Build Contract
`cargo make build`

+ deploy Contract
`cargo make dev-deploy`
