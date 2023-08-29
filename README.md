# dobbe-dirscord-rs
Because rust is fun.

## Features
- Accepted role by reaction
- Courseroom commands

## Commands
- role: Join courserooms
- courseroom: Administer courseroom roles and discord channels
- rules: Administer reaction rules roles

Saves channel and role ids in an Postgres database

```
table! {
    Guilds {
        rules_channel_id,
        rules_accepted_role_id,
        Courserooms {
            name,
            channel_id,
            role_id
        }
    }
}
```