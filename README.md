`cm clean`
`cm build`
`cm dev-deploy`
`cm call-self init`
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
`cm call --account_id gmfam.testnet get_owner_id`
```
dev-1691914103143-32544821462759
```
`cm call --account_id gmfam.testnet get_subscriber_users`

```
[]
```

`cm call create_user '{"nickname":"gmfam"}' --account_id gmfam.testnet`

```
' '
```

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

`cm call create_thread '{"title":"thread title 01", "description":"thread desc 01", "init_point": 9000000000000}' --account_id gmfam.testnet`

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
`cm call get_thread_metadata_by_thread_id '{"thread_id":"gmfam.testnet_thread_title_01"}' --account_id gmfam.testnet`




`cm call create_thread '{"title":"thread title 02", "description":"thread desc 02", "init_point": 111000000000000}' --account_id gmfam.testnet`