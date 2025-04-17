# KamFurDev's Utility Bot

The Discord bot I use in my discord server!

<!-- markdownlint-disable MD033 -->
<a href="#usage"><kbd> <br> Usage <br> </kbd></a>&ensp;&ensp;
<a href="#features"><kbd> <br> Features <br> </kbd></a>&ensp;&ensp;
<a href="#configuration"><kbd> <br> Configuration <br> </kbd></a>&ensp;&ensp;
<a href="todo"><kbd> <br> Wiki <br> </kbd></a>&ensp;&ensp;
<!-- markdownlint-enable MD033 -->

## Usage

You will need the following:

- `cargo` (required): Rust Package Manager.
- `shuttle` (required): For bot running/hosting.

Clone the repository: `git clone https://github.com/TheKamboy/[placeholder]`

Next, you need to make a `Secrets.toml` file in the root of the repository.

After that, add this to the file:

```toml
DISCORD_TOKEN = 'add your bot token here'
```

Once that is done, you can run the bot using `shuttle run`
 or deploy it to shuttle using `shuttle deploy`.

## Features

- Moderation
  - Verbal Warns.
  - Banning, Unbanning.
  - Ban Appeal System.
  - Logging in a channel for the items mentioned above.
- Information
  - Display commands.
  - Show bot version.
  - Show the current changelog.
- Utilities
  - Check account age.
  - Test the random error functionality.
  - Create polls.
- Fun (Most are suggested by my discord server)
  - Give the command runner money (totally...).
  - Recite a single digit of pi.
  - Play Rock Paper Scissors.

## Configuration

- Metadata
  - `NAME: &str`: The name of the bot.
  - `COMMAND_PREFIX: &str`: The prefix for text commands.
  - `VERSION: &str`: The current version of the bot.
  - `DEVELOPMENT: bool`: Disables commands for anyone that isn't the owner.
  - `CHANGELOG_MSG: &str`: The current updates changelog message.
- Channels
  - `LOG_CHANNEL: u64`: The Channel ID for the logging channel.
- Roles
  - `OWNER_ROLES: [u64; 2]`: The roles for the owners (my server has an owner,
  and co owner).
  - `ADMIN_ROLE: u64`: The Role ID for admins.
  - `HIGHER_MOD_ROLE: u64`: The Role ID for higher ranked mods
  (kinda like an in between moderator and admin).
  - `MOD_ROLE: u64`: The Role ID for moderators.
  - `TRIAL_MOD_ROLE: u64`: The Role ID for new moderators.
  - `BANNED_ROLE: u64`: The Role ID for banned users.
- Banned Settings
  - `SUBMIT_BAN_APPEAL_ROLE: u64`: The role given to users who send a
  ban appeal so they can't spam them.
  - `BANNED_INFO_LINK: &str`: A message link to information about how to submit
  ban appeals and other things relating to being banned.
- Other
  - `NOT_ALLOWED_MESSAGES: [&'static str; 6]`:
  Random messages for when the user runs a command that they aren't allowed to run.
