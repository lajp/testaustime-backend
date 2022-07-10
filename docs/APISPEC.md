# Testaustime-rs api documentation

## General info

- Ratelimit: 10 req/m

The desired interval at which to send heartbeats is immidiately when editing a file, and after that at max every 30 seconds, and only when the user does something actively in the editor.

## Endpoints

### POST /activity/update

Logs current activity.

This is the main endpoint this service is based on, this is where you current coding session data is sent.

Accepts:
```
{
    "language": string,
    "hostname": string,
    "editor_name": string,
    "project_name": string
}
```

Required headers:
```
Authorization: Bearer <token>
Content-type: application/json
```

### POST /activity/flush

Flushes any currently active coding session

Required headers:
```
Authorization: Bearer <token>
```

### DELETE /activity/delete

Deletes an activity with the specified id

Accepts:
```
ACTIVITYID
```

Required headers:
```
Authorization: Bearer <token>
```

### GET /users/{username}/activity/data

Get your coding activity data

Url params:
- {username}
- language
- editor
- project_name
- hostname
- min_duration

The users with `{username}` has to be a friend or self of the auth_token provided

A special case of `{username}` is `@me` where the response will include the data of the authenticating user

Returns:
```
[
    {
        "id": int,
        "language": string,
        "hostname": string,
        "editor_name": string,
        "project_name": string,
        "start_time": number,
        "duration": number
    },
    ...
]
```

Required headers:
```
Authorization: Bearer <token>
```

### GET /users/@me

Gets the profile of the authenticating user

Returns:
```
{
    "id": int,
    "user_name": string,
    "friend_code": string,
    "registration_time": string,
}
```

Required headers:
```
Authorization: Bearer <token>
```

### DELETE /users/@me

Deletes the users

**WARNING** This operation is final, there is no going back

Accepts:
```
{
    "username": string,
    "password": string
}
```

Required headers:
```
Content-Type: application/json
```

### GET /users/@me/leaderboards

Gets the authenticating users leaderboards

Required headers:
```
Authorization: Bearer <token>
```

Returns:
```
[
    {
        "name": string,
        member_count: int,
    },
    ...
]
```

### POST /auth/register

Registers a new user and returns the users auth token

The register endpoint has a special ratelimit which is by default configured to be 3 per day (this also includes unsuccesfull requests)

Accepts:
```
{
    "username": string,
    "password": string
}
```

Required headers:
```
Content-type: application/json
```

Returns:
```
{
    "auth_token": string,
    "username": string,
    "friend_code": string,
    "registration_time": string
}
```

### POST /auth/login

Logins to a users account returning the auth token

Accepts:
```
{
    "username": string,
    "password": string
}
```

Required headers:
```
Content-type: application/json
```

Returns:
```
{
    "id": int,
    "auth_token": string,
    "username": string,
    "friend_code": string,
    "registration_time": string
}
```

### POST /auth/changeusername

Changes the users username

Accepts:
```
{
    "new": string
}
```

Required headers:
```
Content-Type: application/json
Authorization: Bearer <token>
```

### POST /auth/changepassword

Changes the users password

Accepts:
```
{
    "old": string,
    "new": string
}
```

Required headers:
```
Authorization: Bearer <token>
Content-type: application/json
```

### POST /auth/regenerate

Regenerate users auth token

Required headers:
```
Authorization: Bearer <token>
```

Returns:
```
{
    "token": string
}
```


### POST /friends/add

Add the holder of the friend token as a friend of the authenticating user

Accepts:
```
ttfc_FRIENDCODE
```

*Note: The friend code is valid with or without the "ttfc_" prefix*


Required headers:
```
Authorization: Bearer <token>
```

Returns:
```
{
    "username": string,
    "coding_time": {
        "all_time": int,
        "past_month": int,
        "past_week": int
    }
}
```

### DELETE /friends/remove

Remove a friend

Accepts:
```
FRIENDS_USERNAME
```

Required headers:
```
Authorization: Bearer <token>
```

### GET /friends/list

Gets a list of the authenticating users friends

Returns:
```
[
    {
        "username": string,
        "coding_time": {
            "all_time": int,
            "past_month": int,
            "past_week": int
        }
    },
    ...
]
```

Required headers:
```
Authorization: Bearer <token>
```

### POST /friends/regenerate

Regenerates the authenticating users friend code

Required headers:
```
Authorization: Bearer <token>
```

Returns:
```
{
    "friend_code": string
}
```

### POST /leaderboards/create

Creates a new leaderboard adding the authenticating user to it as an admin

Required headers:
```
Authorization: Bearer <token>
Content-Type: application/json
```

Accepts:
```
{
    "name": string
}
```

