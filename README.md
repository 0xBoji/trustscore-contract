page structure

- /
  - /spaces
    - /spaces/forex_trading **#forex_trading**
      - /thread/thread_001 **thread_001**
      - /thread/thread_002 **thread_002**
    - /spaces/crypto_trading **#crypto_trading**
      - /thread/thread_003 **thread_003**
      - /thread/thread_004 **thread_004**
  - /me
    - /me/create

### Step to run local

- clean up after update code

  ```
  cm clean
  ```

- build lại contract

  ```
  cm build
  ```

- deploy lại contract

  ```
  cm dev-deploy
  ```

- init contract (bắt buộc)

  ```
  cm call-self init
  ```

- re check lại init ok không
  ```
  cm call --account_id gmfam.testnet get_me
  ```
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
  ```
  cm call --account_id gmfam.testnet get_owner_id
  ```
  ```
  dev-1693000504580-67464128643370
  ```
- get array danh sách id all user (sau khi đã đăng ký vào TS)

  ```
  cm call --account_id gmfam.testnet get_subscriber_users
  ```

  ```
  []
  ```

- tạo 1 user trên hệ thống TS
  ```
  cm call create_user '{"nickname":"long","first_name":"Ân","last_name":"Nguyễn Văn", "bio":"this is my bio" ,"avatar":"bafkreibnaelo4monu6jpndqtb3pmza22j7k77gcak3xrux6mrkdq5fakuu"}' --account_id gmfam.testnet
  ```
  ```
  ''
  ```
- active role cho 1 user by user_id

  ```
  cm call active_user_role '{"user_id":"gmfam.testnet"}' --account_id gmfam.testnet
  ```

  ```
  {
    user_id: 'gmfam.testnet',
    metadata: {
      user_id: 'gmfam.testnet',
      nickname: 'longn',
      role: 'Verified',
      first_name: 'Ân',
      last_name: 'Nguyễn Văn',
      bio: 'this is my bio',
      avatar: 'bafkreibnaelo4monu6jpndqtb3pmza22j7k77gcak3xrux6mrkdq5fakuu',
      created_at: 1692823540083,
      updated_at: 1692823540083
    },
    threads: [],
    total_point: 1000,
    threads_owned: 0
  }
  ```

- get array danh sách id all user (sau khi đã đăng ký vào TS)

  ```
  cm call --account_id gmfam.testnet get_subscriber_users
  ```

  ```
  [ 'gmfam.testnet' ]
  ```

- get info của 1 user by user_id

  ```
  cm call get_user_metadata_by_user_id '{"user_id":"gmfam.testnet"}' --account_id gmfam.testnet
  ```

  ```
  {
    user_id: 'gmfam.testnet',
    metadata: {
      user_id: 'gmfam.testnet',
      nickname: 'longn',
      role: 'Unverified',
      first_name: 'Ân',
      last_name: 'Nguyễn Văn',
      bio: 'this is my bio',
      avatar: 'bafkreibnaelo4monu6jpndqtb3pmza22j7k77gcak3xrux6mrkdq5fakuu',
      created_at: 1693000706705,
      updated_at: 1693000706705
    },
    threads_list: [],
    fraud_list: [],
    total_point: 1000,
    threads_owned: 0,
    fraud_threads_owned: 0
  }
  ```

- update info của 1 user by user_id

  ```
  cm call update_user_information '{"nickname":"longlong"}' --account_id gmfam.testnet
  ```

  ```
  {
    user_id: 'gmfam.testnet',
    metadata: {
      user_id: 'gmfam.testnet',
      nickname: 'longlong',
      role: 'Verified',
      first_name: 'Ân',
      last_name: 'Nguyễn Văn',
      bio: 'this is my bio',
      avatar: 'bafkreibnaelo4monu6jpndqtb3pmza22j7k77gcak3xrux6mrkdq5fakuu',
      created_at: 1693105707209,
      updated_at: 1693107578924
    },
    threads_list: [],
    fraud_list: [],
    total_point: 1000,
    threads_owned: 0,
    fraud_threads_owned: 0
  }
  ```

- tạo 1 thread mới. mode 0 fraud

  ```
  cm call create_thread '{"title": "I am a fraud patient", "description": "this is my first fraud post", "media_link":"bafkreifko42xz73mizlglr235icoexdicld5xqutbsymwph4fvnoktvnym", "init_point": 10, "space_name": "crypto trading", "start_time": "1692824790000", "end_time": "1702162578000", "options": ["gmfam.testnet", "phapdev.testnet"], "partner_id":"phapdev.testnet", "thread_mode": 0} ' --account_id gmfam.testnet
  ```

  ```
  {
    thread_id: 'gmfam.testnet_thread_title_01_fraud_mode',
    title: 'thread title 01 fraud mode',
    media_link: 'bafkreifko42xz73mizlglr235icoexdicld5xqutbsymwph4fvnoktvnym',
    thread_mode: 0,
    creator_id: 'gmfam.testnet',
    partner_id: 'long6789.testnet',
    content: null,
    created_at: 1693003446366,
    init_point: 90,
    space_name: 'crypto trading',
    last_id: 0,
    choices_count: 2,
    choices_map: { '0': 'gmfam.testnet', '1': 'long6789.testnet' },
    user_votes_map: {},
    choices_rating: {},
    start_time: 1692824790000,
    end_time: 1695477352000
  }

  // user creator updated
  {
    user_id: 'gmfam.testnet',
    ...
    threads_list: [ 'gmfam.testnet_thread_title_01_fraud_mode' ],
    fraud_list: [],
    total_point: 910,
    threads_owned: 1,
    fraud_threads_owned: 0
  }

  // user fraud updated
  {
    user_id: 'long6789.testnet',
    ...
    threads_list: [],
    fraud_list: [ 'gmfam.testnet_thread_title_01_fraud_mode' ],
    total_point: 910,
    threads_owned: 0,
    fraud_threads_owned: 1
  }
  ```

- tạo 1 thread mới. mode 1 simple

  ```
  cm call create_thread '{"title": "Is crypto is the future?", "description": "Is crypto is the future? Lets discuss ...", "media_link":"bafkreifko42xz73mizlglr235icoexdicld5xqutbsymwph4fvnoktvnym", "init_point": 20, "space_name": "crypto trading", "start_time": "1692824790000", "end_time": "1702162578000", "options": ["No", "Yes"], "thread_mode": 1} ' --account_id gmfam.testnet
  ```

  ```
  {
    thread_id: 'gmfam.testnet_thread_title_02_simple_mode',
    title: 'thread title 02 simple mode',
    media_link: 'bafkreifko42xz73mizlglr235icoexdicld5xqutbsymwph4fvnoktvnym',
    thread_mode: 1,
    creator_id: 'gmfam.testnet',
    partner_id: null,
    content: null,
    created_at: 1693005707584,
    init_point: 200,
    space_name: 'crypto trading',
    last_id: 0,
    choices_count: 2,
    choices_map: { '0': 'No', '1': 'Yes' },
    user_votes_map: {},
    choices_rating: {},
    start_time: 1692824790000,
    end_time: 1695477352000
  }

  // creator after updated
  {
    user_id: 'gmfam.testnet',
    ...
    threads_list: [
      'gmfam.testnet_thread_title_01_fraud_mode',
      'gmfam.testnet_thread_title_02_simple_mode'
    ],
    fraud_list: [],
    total_point: 710,
    threads_owned: 2,
    fraud_threads_owned: 0
  }
  ```

- lấy info của 1 thread by thread_id

  ```
  cm call get_thread_metadata_by_thread_id '{"thread_id":"gmfam.testnet_thread_title_01_fraud_mode"}' --account_id gmfam.testnet
  ```

  ```
  {
    thread_id: 'gmfam.testnet_thread_title_02',
    title: 'thread title 02',
    media_link: 'bafkreifko42xz73mizlglr235icoexdicld5xqutbsymwph4fvnoktvnym',
    creator_id: 'gmfam.testnet',
    content: null,
    created_at: 1692874267501,
    init_point: 190,
    space_name: 'future trading',
    last_id: 0,
    choices_count: 2,
    choices_map: { '0': 'No', '1': 'Yes' },
    user_votes_map: {},
    choices_rating: {},
    start_time: 1692824790000,
    end_time: 1695477352000
  }
  ```

- lấy status của 1 thread by thread_id

  ```
  cm call get_thread_status '{"thread_id":"gmfam.testnet_thread_title_01_fraud_mode"}' --account_id gmfam.testnet
  ```

  ```
  'Closed' // 'Open', 'Upcoming'

  ```

- lấy info các thread của 1 user own by user_id

  ```
  cm call get_all_threads_per_user_own '{"user_id":"gmfam.testnet"}' --account_id gmfam.testnet
  ```

  ```
  [
    {
      thread_id: 'gmfam.testnet_thread_title_01_fraud_mode',
      title: 'thread title 01',
      media_link: 'bafkreifko42xz73mizlglr235icoexdicld5xqutbsymwph4fvnoktvnym',
      creator_id: 'gmfam.testnet',
      content: null,
      created_at: 1692874190923,
      init_point: 90,
      space_name: 'crypto trading',
      last_id: 0,
      choices_count: 2,
      choices_map: { '0': 'No', '1': 'Yes' },
      user_votes_map: {},
      choices_rating: {},
      start_time: 1692824790000,
      end_time: 1695477352000
    },
    {
      thread_id: 'gmfam.testnet_thread_title_02',
      title: 'thread title 02',
      media_link: 'bafkreifko42xz73mizlglr235icoexdicld5xqutbsymwph4fvnoktvnym',
      creator_id: 'gmfam.testnet',
      content: null,
      created_at: 1692874267501,
      init_point: 190,
      space_name: 'future trading',
      last_id: 0,
      choices_count: 2,
      choices_map: { '0': 'No', '1': 'Yes' },
      user_votes_map: {},
      choices_rating: {},
      start_time: 1692824790000,
      end_time: 1695477352000
    }
  ]
  ```

- get info của 1 space not existed
  ```
  cm call get_space_metadata_by_space_id '{"space_id":"crypto"}' --account_id gmfam.testnet
  ```
  ```
  null
  ```
- get info của 1 space
  ```
  cm call get_space_metadata_by_space_id '{"space_id":"crypto_trading"}' --account_id gmfam.testnet
  ```
  ```
  {
    space_id: 'crypto_trading',
    space_name: 'crypto trading',
    creator_id: 'gmfam.testnet',
    created_at: 1692737457246
  }
  ```
- get_all_threads_of_space_by_space_id info của 1 space

  ```
  cm call get_all_threads_of_space_by_space_id '{"space_id": "crypto_trading"}' --account_id gmfam.testnet
  ```

  ```
  [ 'gmfam.testnet_thread_title_01_fraud_mode', 'gmfam.testnet_thread_title_02' ]

  ```

- get_all_threads_of_space_by_space_id info của 1 space

  ```
  cm call get_all_threads_of_space_by_space_id '{"space_id":"trading"}' --account_id gmfam.testnet
  ```

  ```
  ['gmfam.testnet_thread_title_03' ]

  ```

- get all spaces info

  ```
  cm call get_all_spaces --account_id gmfam.testnet
  ```

  ```
  [
    {
      space_id: 'crypto_trading',
      space_name: 'crypto trading',
      creator_id: 'gmfam.testnet',
      created_at: 1692737457246
    },
    {
      space_id: 'trading',
      space_name: 'trading',
      creator_id: 'gmfam.testnet',
      created_at: 1692739794022
    }
  ]
  ```

- create new spaces

  ```
  cm call create_space '{"space_name":"crypto trading"}'  --account_id gmfam.testnet
  ```

  ```
  {
    space_id: 'crypto_trading',
    space_name: 'crypto trading',
    creator_id: 'gmfam.testnet',
    created_at: 1693107266493,
    followed_users: []
  }
  ```

- follow spaces_id

  ```
  cm call follow_space '{"space_id":"crypto_discuss"}'  --account_id gmfam.testnet
  ```

  ```
  'OK'
  ```

- get all user followed by spaces_id

  ```
  cm call get_followed_user_of_space_by_space_id '{"space_id":"crypto_discuss"}' --account_id gmfam.testnet
  ```

  ```
  'OK'
  ```

- vote thread by thread_id

  ```
  cm call vote_thread '{"thread_id":"gmfam.testnet_thread_title_01_fraud_mode", "choice_number": 0, "point": 50}' --account_id gmfam.testnet
  cm call vote_thread '{"thread_id":"gmfam.testnet_thread_title_02_simple_mode", "choice_number": 1, "point": 150}' --account_id gmfam.testnet
  ```

  ```
  OK
  ```

- end thread by thread_id

  ```
  cm call end_thread '{"thread_id":"gmfam.testnet_thread_title_01_fraud_mode"}' --account_id gmfam.testnet
  ```

  ```
  OK
  ```

### **function list for reference**

- main contract
  ```
  init()
  ```
  ```
  new()
  ```
  ```
  get_me()
  ```
  ```
  get_owner_id()
  ```
  ```
  get_subscriber_users()
  ```
- thread module
  ```
  create_thread()
  ```
  ```
  get_thread_metadata_by_thread_id()
  ```
  ```
  get_all_threads_per_user_own()
  ```
  ```
  get_thread_status()
  ```
  ```
  vote_thread()
  ```
  ```
  end_thread()
  ```
- space module
  ```
  create_space()
  ```
  ```
  get_space_metadata_by_space_id()
  ```
  ```
  get_all_threads_of_space_by_space_id()
  ```
  ```
  get_all_spaces()
  ```
  ```
  follow_space()
  ```
  ```
  get_followed_user_of_space_by_space_id()
  ```
- user module
  ```
  create_user()
  ```
  ```
  get_user_metadata_by_user_id()
  ```
  ```
  update_user_information()
  ```
  ```
  get_all_user_metadata()
  ```
  ```
  check_user_role()
  ```
  ```
  active_user_role()
  ```
  ```
  set_admin_user_role()
  ```

### Requirements

- Install Cargo Make
  `cargo install cargo-make`
  `cargo install cargo-generate`

* Install Near Cli
  `npm install -g near-cli`

* Prepare
  `cargo make prepare`

* Build Contract
  `cargo make build`

* deploy Contract
  `cargo make dev-deploy`

near deploy --accountId trust-score-project.gmfam.testnet --wasmFile target/wasm32-unknown-unknown/release/thread_score.wasm
