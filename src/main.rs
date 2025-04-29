use ::serenity::all::ReactionType;
use anyhow::Context as _;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::ActivityData;
use rand::Rng;
use serenity::all::{CreateMessage, User};
use serenity::prelude::*;
use shuttle_runtime::SecretStore;

/* Category's:
, category = "Utility"
, category = "Info"
, category = "Moderation"
*/

// Bot Settings

/// The name you want to give the bot. ( default: `KamFurDev's Utility Bot` )
const NAME: &str = "KamFurDev's Utility Bot";

/// The prefix for text commands. ( default: ; )
const COMMAND_PREFIX: &str = ";";

/// The current version of the bot.
const VERSION: &str = "v0.5.1-alpha";

/// Blocks commands from being sent unless it is sent from the owners. ( default: false )
const DEVELOPMENT: bool = false;

// Changelog
const CHANGELOG_MSG: &str = "# Current Update (v0.5.1-alpha)
- Quick Fix: Fixed accepting and denying appeals not checking user ranking.
- Moderation: You can now unban, accept and deny appeals!
- Error Messages: A new \"action not allowed\" message has been added. It's a server inside joke that just happened (as of writing this).";

// Channels
const LOG_CHANNEL: u64 = 1314766735030747218;

// Roles
const OWNER_ROLES: [u64; 2] = [1314451651704393770, 1314451866289176616]; // owner and co-owner
const ADMIN_ROLE: u64 = 1314453508409262130;
const HIGHER_MOD_ROLE: u64 = 1320629413154525265; // higher ranked mod
const MOD_ROLE: u64 = 1314454674111467602;
const TRIAL_MOD_ROLE: u64 = 1314454804369510421;
const BANNED_ROLE: u64 = 1314772212921929818;

// banned stuff
const SUBMIT_BAN_APPEAL_ROLE: u64 = 1360727888399175811;
const BANNED_INFO_LINK: &str =
    "https://discord.com/channels/1229999485854154833/1355646747661041842/1355648740928520384";

// Random Messages
const NOT_ALLOWED_MESSAGES: [&str; 6] = [
    "This is not a fucking painting!!!",
    "You...shall not...pass!",
    "My programmer doesn't want you doing this, so go away until you are allowed to do so!",
    "No.",
    "I'm sorry Dave, I'm afraid I can't do that.",
    "\"bro you havent any power here\"",
];

// Command Locks
// / Locks the `/poll` command for any role not in the list. If empty, any user can use it.

// fuckin hell i gotta do a rewrite of all my shit
struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Creates a poll for users to vote on.
#[allow(clippy::too_many_arguments)]
#[poise::command(slash_command, prefix_command, category = "Utility")]
async fn poll(
    ctx: Context<'_>,
    #[description = "Title"] title: String,
    #[description = "Description"] description: Option<String>,
    #[description = "Option 1"] option1: String,
    #[description = "Option 2"] option2: String,
    #[description = "Option 3"] option3: Option<String>,
    #[description = "Option 4"] option4: Option<String>,
    #[description = "Option 5"] option5: Option<String>,
    #[description = "Option 6"] option6: Option<String>,
    #[description = "Option 7"] option7: Option<String>,
    #[description = "Option 8"] option8: Option<String>,
    #[description = "Option 9"] option9: Option<String>,
    #[description = "Option 10"] option10: Option<String>,
) -> Result<(), Error> {
    let emptystring: &String = &String::new();
    let dec = description.as_ref().unwrap_or(emptystring);
    let o3 = option3.as_ref().unwrap_or(emptystring);
    let o4 = option4.as_ref().unwrap_or(emptystring);
    let o5 = option5.as_ref().unwrap_or(emptystring);
    let o6 = option6.as_ref().unwrap_or(emptystring);
    let o7 = option7.as_ref().unwrap_or(emptystring);
    let o8 = option8.as_ref().unwrap_or(emptystring);
    let o9 = option9.as_ref().unwrap_or(emptystring);
    let o0 = option10.as_ref().unwrap_or(emptystring);

    let authorname = &ctx.author().id.to_string();

    let mut message = format!("# {title}\n> Asked by: <@{authorname}>\n");
    if dec != emptystring {
        message = format!("{message}> {dec}\n\n");
    } else {
        message = format!("{message}\n");
    }

    message = format!("{message}:one: {option1}\n:two: {option2}");

    if !o3.is_empty() {
        message = format!("{message}\n:three: {o3}");
    }
    if !o4.is_empty() {
        message = format!("{message}\n:four: {o4}");
    }
    if !o5.is_empty() {
        message = format!("{message}\n:five: {o5}");
    }
    if !o6.is_empty() {
        message = format!("{message}\n:six: {o6}");
    }
    if !o7.is_empty() {
        message = format!("{message}\n:seven: {o7}");
    }
    if !o8.is_empty() {
        message = format!("{message}\n:eight: {o8}");
    }
    if !o9.is_empty() {
        message = format!("{message}\n:nine: {o9}");
    }
    if !o0.is_empty() {
        message = format!("{message}\n:keycap_ten: {o0}");
    }

    let msg = ctx.say(message).await?.into_message().await?;
    msg.react(ctx.http(), ReactionType::Unicode("1️⃣".to_string()))
        .await?;
    msg.react(ctx.http(), ReactionType::Unicode("2️⃣".to_string()))
        .await?;

    if !o3.is_empty() {
        msg.react(ctx.http(), ReactionType::Unicode("3️⃣".to_string()))
            .await?;
    }
    if !o4.is_empty() {
        msg.react(ctx.http(), ReactionType::Unicode("4️⃣".to_string()))
            .await?;
    }
    if !o5.is_empty() {
        msg.react(ctx.http(), ReactionType::Unicode("5️⃣".to_string()))
            .await?;
    }
    if !o6.is_empty() {
        msg.react(ctx.http(), ReactionType::Unicode("6️⃣".to_string()))
            .await?;
    }
    if !o7.is_empty() {
        msg.react(ctx.http(), ReactionType::Unicode("7️⃣".to_string()))
            .await?;
    }
    if !o8.is_empty() {
        msg.react(ctx.http(), ReactionType::Unicode("8️⃣".to_string()))
            .await?;
    }
    if !o9.is_empty() {
        msg.react(ctx.http(), ReactionType::Unicode("9️⃣".to_string()))
            .await?;
    }
    if !o0.is_empty() {
        msg.react(ctx.http(), ReactionType::Unicode("🔟".to_string()))
            .await?;
    }

    Ok(())
}

/// Displays your or another user's account creation date.
#[poise::command(slash_command, prefix_command, category = "Utility")]
async fn account_age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    category = "Utility",
    subcommands("test_error_notallowed"),
    owners_only,
    hide_in_help
)]
async fn test_random_error(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Select an error to generate.").await?;
    Ok(())
}

#[poise::command(
    slash_command,
    prefix_command,
    category = "Utility",
    owners_only,
    hide_in_help
)]
async fn test_error_notallowed(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(random_not_allowed_message()).await?;
    Ok(())
}

// Sends a verbal warning to the user specified, with a reason.
// #[poise::command(slash_command, prefix_command, category = "Moderation")]
// async fn timeout() {}

/// Sends a verbal warning to the user specified, with a reason.
#[poise::command(slash_command, prefix_command, category = "Moderation")]
async fn verbal_warn(
    ctx: Context<'_>,
    #[description = "User"] user: serenity::User,
    #[description = "Reason"] reason: String,
) -> Result<(), Error> {
    if !check_for_roles(
        &ctx,
        ctx.author(),
        [
            OWNER_ROLES[0],
            OWNER_ROLES[1],
            ADMIN_ROLE,
            HIGHER_MOD_ROLE,
            MOD_ROLE,
            TRIAL_MOD_ROLE,
        ]
        .as_ref(),
    )
    .await
    {
        ctx.reply(random_not_allowed_message()).await?;
        return Ok(());
    }

    let u = user;

    if check_for_roles(
        &ctx,
        &u,
        [
            OWNER_ROLES[0],
            OWNER_ROLES[1],
            ADMIN_ROLE,
            HIGHER_MOD_ROLE,
            MOD_ROLE,
            TRIAL_MOD_ROLE,
        ]
        .as_ref(),
    )
    .await
        && !check_for_roles(
            &ctx,
            ctx.author(),
            [OWNER_ROLES[0], OWNER_ROLES[1]].as_ref(),
        )
        .await
    {
        ctx.reply("User is a moderator or higher.").await?;
        return Ok(());
    }

    let mut umention = "".to_owned();
    umention.push_str("<@");
    umention.push_str(&u.id.to_string());
    umention.push('>');
    let moderatorname = &ctx.author().name;
    let msgreason = reason;

    let logchannel = serenity::ChannelId::new(LOG_CHANNEL);

    let mut log_message = String::new().to_owned();

    log_message.push_str(
        format!(
            "Sent a verbal warning to {umention}.\nModerator: {moderatorname}\nReason: {msgreason}"
        )
        .as_str(),
    );

    let mut dm_message = String::new().to_owned();

    dm_message.push_str(format!(
            "{umention}\nThis is a verbal warning! Continued action with have consequences!\nModerator: {moderatorname}\nReason: {msgreason}"
        ).as_str());

    logchannel.say(ctx.http(), log_message).await?;
    u.dm(ctx.http(), CreateMessage::new().content(dm_message))
        .await?;

    ctx.reply("Done!").await?;
    Ok(())
}

/// Drops the Ban Role on a user of your choosing.
#[poise::command(slash_command, prefix_command, category = "Moderation")]
async fn ban_user(
    ctx: Context<'_>,
    #[description = "User"] user: serenity::User,
    #[description = "Reason"] reason: String,
) -> Result<(), Error> {
    if !check_for_roles(
        &ctx,
        ctx.author(),
        [OWNER_ROLES[0], OWNER_ROLES[1], ADMIN_ROLE, HIGHER_MOD_ROLE].as_ref(),
    )
    .await
    {
        ctx.reply(random_not_allowed_message()).await?;
        return Ok(());
    }

    let u = user;

    if check_for_roles(
        &ctx,
        &u,
        [OWNER_ROLES[0], OWNER_ROLES[1], ADMIN_ROLE].as_ref(),
    )
    .await
        && !check_for_roles(
            &ctx,
            ctx.author(),
            [OWNER_ROLES[0], OWNER_ROLES[1]].as_ref(),
        )
        .await
    {
        ctx.reply("User is a higher moderator or higher.").await?;
        return Ok(());
    }

    if u.id == ctx.author().id {
        ctx.reply("You can't ban yourself!").await?;
        return Ok(());
    }

    let mut umention = "".to_owned();
    umention.push_str("<@");
    umention.push_str(&u.id.to_string());
    umention.push('>');
    let moderatorname = &ctx.author().name;

    let logchannel = serenity::ChannelId::new(LOG_CHANNEL);

    let mut log_message = String::new().to_owned();

    log_message.push_str(
        format!("Banned {umention}.\nModerator: {moderatorname}\nReason: {reason}").as_str(),
    );

    let mut dm_message = String::new().to_owned();

    dm_message.push_str(format!(
            "{umention}\nYou have been banned! For more info, go here: {BANNED_INFO_LINK}\nModerator: {moderatorname}\nReason: {reason}"
        ).as_str());

    logchannel.say(ctx.http(), log_message).await?;
    u.dm(ctx.http(), CreateMessage::new().content(dm_message))
        .await?;

    let ban_role = serenity::RoleId::new(BANNED_ROLE);

    ctx.http()
        .add_member_role(ctx.guild_id().unwrap(), u.id, ban_role, Some(&reason))
        .await?;

    ctx.reply("Done!").await?;

    Ok(())
}

/// Submit a ban appeal, if you are banned.
#[poise::command(slash_command, prefix_command, category = "Moderation")]
async fn submit_ban_appeal(
    ctx: Context<'_>,
    #[description = "When was the ban?"] ban_time: String,
    #[description = "Why did we ban you?"] ban_reason: String,
    #[description = "Why do you want to be unbanned?"] want_unban_reason: String,
    #[description = "If you select True, you won't cause trouble in the server again."]
    confirm: bool,
) -> Result<(), Error> {
    if !check_for_roles(&ctx, ctx.author(), [BANNED_ROLE].as_ref()).await {
        ctx.reply(random_not_allowed_message()).await?;
        return Ok(());
    }

    if check_for_roles(&ctx, ctx.author(), [SUBMIT_BAN_APPEAL_ROLE].as_ref()).await {
        ctx.reply("You have already sent a ban appeal! Please wait.")
            .await?;
        return Ok(());
    }

    let logchannel = serenity::ChannelId::new(LOG_CHANNEL);

    let mut log_message = String::new().to_owned();

    let mut umention = "".to_owned();
    umention.push_str("<@");
    umention.push_str(&ctx.author().id.to_string());
    umention.push('>');

    let userid = ctx.author().id;

    let accepted = if confirm { "Yes" } else { "No" };

    log_message.push_str(
        format!(
            "{umention} has submitted a ban appeal!\nUser ID: {userid}\n\nTime of Ban: {ban_time}\n\nBan Reason: {ban_reason}\n\nUnban Reason: {want_unban_reason}\n\nAccepted Terms: {accepted}"
        )
        .as_str(),
    );

    let ban_role = serenity::RoleId::new(SUBMIT_BAN_APPEAL_ROLE);

    ctx.http()
        .add_member_role(
            ctx.guild_id().unwrap(),
            ctx.author().id,
            ban_role,
            Some("User submitted a ban appeal."),
        )
        .await?;

    logchannel.say(ctx.http(), log_message).await?;

    ctx.reply("Submitted Ban Appeal.").await?;

    Ok(())
}

/// Accepts a ban appeal sent by a banned user.
#[poise::command(slash_command, prefix_command, category = "Moderation")]
async fn accept_ban_appeal(
    ctx: Context<'_>,
    #[description = "User"] user: serenity::User,
    #[description = "Reason"] reason: String,
) -> Result<(), Error> {
    if !check_for_roles(
        &ctx,
        ctx.author(),
        [OWNER_ROLES[0], OWNER_ROLES[1], ADMIN_ROLE, HIGHER_MOD_ROLE].as_ref(),
    )
    .await
    {
        ctx.reply(random_not_allowed_message()).await?;
        return Ok(());
    }

    let ban_role = serenity::RoleId::new(SUBMIT_BAN_APPEAL_ROLE);
    let ban_role2 = serenity::RoleId::new(BANNED_ROLE);

    ctx.http()
        .remove_member_role(
            ctx.guild_id().unwrap(),
            user.id,
            ban_role,
            Some("Appeal action accepted."),
        )
        .await?;

    ctx.http()
        .remove_member_role(
            ctx.guild_id().unwrap(),
            user.id,
            ban_role2,
            Some(format!("Staff Reason: {reason}").as_str()),
        )
        .await?;

    let logchannel = serenity::ChannelId::new(LOG_CHANNEL);

    let mut log_message = String::new().to_owned();

    let mut umention = "".to_owned();
    umention.push_str("<@");
    umention.push_str(&user.id.to_string());
    umention.push('>');

    let moderatorname = &ctx.author().name;

    log_message.push_str(
        format!(
            "{umention} had their appeal accepted!\nReason: {reason}\nModerator: {moderatorname}"
        )
        .as_str(),
    );

    let mut dm_message = String::new().to_owned();

    dm_message.push_str(
        format!("Your appeal has been accepted, and you have been unbanned!\nReason: {reason}")
            .as_str(),
    );

    let message = CreateMessage::new().content(dm_message);

    logchannel.say(ctx.http(), log_message).await?;
    user.dm(ctx.http(), message).await?;
    ctx.reply("Done!").await?;

    Ok(())
}

/// Denies a ban appeal sent by a banned user.
#[poise::command(slash_command, prefix_command, category = "Moderation")]
async fn deny_ban_appeal(
    ctx: Context<'_>,
    #[description = "User"] user: serenity::User,
    #[description = "Reason"] reason: String,
) -> Result<(), Error> {
    if !check_for_roles(
        &ctx,
        ctx.author(),
        [OWNER_ROLES[0], OWNER_ROLES[1], ADMIN_ROLE, HIGHER_MOD_ROLE].as_ref(),
    )
    .await
    {
        ctx.reply(random_not_allowed_message()).await?;
        return Ok(());
    }

    let ban_role = serenity::RoleId::new(SUBMIT_BAN_APPEAL_ROLE);

    ctx.http()
        .remove_member_role(
            ctx.guild_id().unwrap(),
            user.id,
            ban_role,
            Some("Appeal action denied."),
        )
        .await?;

    let logchannel = serenity::ChannelId::new(LOG_CHANNEL);

    let mut log_message = String::new().to_owned();

    let mut umention = "".to_owned();
    umention.push_str("<@");
    umention.push_str(&user.id.to_string());
    umention.push('>');

    let moderatorname = &ctx.author().name;

    log_message.push_str(
        format!(
            "{umention} had their appeal denied!\nReason: {reason}\nModerator: {moderatorname}"
        )
        .as_str(),
    );

    let mut dm_message = String::new().to_owned();

    dm_message.push_str(
        format!("Your appeal has been denied! You can now re-appeal.\nReason: {reason}").as_str(),
    );

    let message = CreateMessage::new().content(dm_message);

    logchannel.say(ctx.http(), log_message).await?;
    user.dm(ctx.http(), message).await?;
    ctx.reply("Done!").await?;

    Ok(())
}

/// Unbans a user.
#[poise::command(slash_command, prefix_command, category = "Moderation")]
async fn unban_user(
    ctx: Context<'_>,
    #[description = "User"] user: serenity::User,
    #[description = "Reason"] reason: String,
) -> Result<(), Error> {
    if !check_for_roles(
        &ctx,
        ctx.author(),
        [OWNER_ROLES[0], OWNER_ROLES[1], ADMIN_ROLE, HIGHER_MOD_ROLE].as_ref(),
    )
    .await
    {
        ctx.reply(random_not_allowed_message()).await?;
        return Ok(());
    }
    let ban_role = serenity::RoleId::new(SUBMIT_BAN_APPEAL_ROLE);
    let ban_role2 = serenity::RoleId::new(BANNED_ROLE);

    ctx.http()
        .remove_member_role(
            ctx.guild_id().unwrap(),
            user.id,
            ban_role,
            Some("Staff unban."),
        )
        .await?;

    ctx.http()
        .remove_member_role(
            ctx.guild_id().unwrap(),
            user.id,
            ban_role2,
            Some(format!("Staff Reason: {reason}").as_str()),
        )
        .await?;

    let logchannel = serenity::ChannelId::new(LOG_CHANNEL);

    let mut log_message = String::new().to_owned();

    let mut umention = "".to_owned();
    umention.push_str("<@");
    umention.push_str(&user.id.to_string());
    umention.push('>');

    let moderatorname = &ctx.author().name;

    log_message.push_str(
        format!("{umention} has been unbanned!\nReason: {reason}\nModerator: {moderatorname}")
            .as_str(),
    );

    let mut dm_message = String::new().to_owned();

    dm_message.push_str(format!("You have been unbanned!\nReason: {reason}").as_str());

    let message = CreateMessage::new().content(dm_message);

    logchannel.say(ctx.http(), log_message).await?;
    user.dm(ctx.http(), message).await?;
    ctx.reply("Done!").await?;

    Ok(())
}

// first fun command
/// Gives you money, totally...
#[poise::command(slash_command, prefix_command, category = "Fun")]
async fn give_me_money(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("Sent! Check your bank account.").await?;
    Ok(())
}

/// Recites a digit of pi.
#[poise::command(slash_command, prefix_command, category = "Fun")]
async fn recite_digit_pi(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("3").await?;
    Ok(())
}

#[derive(Debug, poise::ChoiceParameter, PartialEq)]
pub enum RockPaperScissors {
    #[name = "Rock"]
    ROCK,
    #[name = "Paper"]
    PAPER,
    #[name = "Scissors"]
    SCISSORS,
}

/// Recites a digit of pi.
#[poise::command(slash_command, prefix_command, category = "Fun")]
async fn rock_paper_scissor(
    ctx: Context<'_>,
    #[description = "Rock, Paper, or Scissors?"] choice: RockPaperScissors,
) -> Result<(), Error> {
    let num = rand::rng().random_range(0..3);
    let mut bot_choice = "SCISSORS";
    let mut human_choice = "SCISSORS";

    if choice == RockPaperScissors::ROCK {
        human_choice = "ROCK"
    } else if choice == RockPaperScissors::PAPER {
        human_choice = "PAPER"
    }

    if num == 0 {
        bot_choice = "ROCK";
    } else if num == 1 {
        bot_choice = "PAPER";
    }

    // easy fail safe for a tie
    if bot_choice == human_choice {
        ctx.reply(format!("We both chose {human_choice}. It's a tie!"))
            .await?;
        return Ok(());
    }

    if bot_choice == "ROCK" {
        if human_choice == "PAPER" {
            ctx.reply(format!(
                "I chose {bot_choice} and you chose {human_choice}. You win!"
            ))
            .await?;
            Ok(())
        } else {
            ctx.reply(format!(
                "I chose {bot_choice} and you chose {human_choice}. I win!"
            ))
            .await?;
            Ok(())
        }
    } else if bot_choice == "PAPER" {
        if human_choice == "SCISSORS" {
            ctx.reply(format!(
                "I chose {bot_choice} and you chose {human_choice}. You win!"
            ))
            .await?;
            Ok(())
        } else {
            ctx.reply(format!(
                "I chose {bot_choice} and you chose {human_choice}. I win!"
            ))
            .await?;
            Ok(())
        }
    } else if human_choice == "ROCK" {
        ctx.reply(format!(
            "I chose {bot_choice} and you chose {human_choice}. You win!"
        ))
        .await?;
        Ok(())
    } else {
        ctx.reply(format!(
            "I chose {bot_choice} and you chose {human_choice}. I win!"
        ))
        .await?;
        Ok(())
    }
}

/// Sends the changelog to your DMs.
#[poise::command(slash_command, prefix_command, category = "Info")]
async fn changelog(ctx: Context<'_>) -> Result<(), Error> {
    let message = CreateMessage::new().content(CHANGELOG_MSG);
    ctx.reply("I sent the changelog to your DMs.").await?;
    ctx.author().dm(ctx.http(), message).await?;
    Ok(())
}

/// Displays the current version of the bot.
#[poise::command(slash_command, prefix_command, category = "Info")]
async fn version(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(format!("{NAME} {VERSION}").as_str()).await?;
    Ok(())
}

/// Shows commands you can run using the bot.
#[poise::command(slash_command, track_edits, prefix_command, category = "Info")]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            ephemeral: true,
            extra_text_at_bottom: format!("Prefix = {COMMAND_PREFIX}\n{NAME} {VERSION}").as_str(),
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

fn random_not_allowed_message() -> String {
    let mut _message = String::new();
    let num = rand::rng().random_range(0..NOT_ALLOWED_MESSAGES.len());

    _message = NOT_ALLOWED_MESSAGES[num].to_string();

    _message
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

async fn check_for_roles(ctx: &Context<'_>, user: &User, roles: &[u64]) -> bool {
    for role in roles {
        if user
            .has_role(ctx.http(), ctx.guild_id().unwrap(), role.to_owned())
            .await
            .unwrap()
        {
            return true;
        }
    }

    false
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = secrets
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                // info
                help(),
                version(),
                changelog(),
                // moderation
                verbal_warn(),
                ban_user(),
                unban_user(),
                submit_ban_appeal(),
                accept_ban_appeal(),
                deny_ban_appeal(),
                // utility
                account_age(),
                test_random_error(),
                poll(),
                // fun
                give_me_money(),
                recite_digit_pi(),
                rock_paper_scissor(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(COMMAND_PREFIX.to_string()),
                ..Default::default()
            },
            // The global error handler for all error cases that may occur
            on_error: |error| Box::pin(on_error(error)),
            command_check: Some(|ctx| {
                Box::pin(async move {
                    let mut check = false;
                    if DEVELOPMENT {
                        for role in OWNER_ROLES {
                            if ctx
                                .author()
                                .has_role(ctx.http(), ctx.guild_id().unwrap(), role)
                                .await?
                            {
                                check = true;
                                break;
                            }
                        }
                    } else {
                        check = true;
                    }

                    if !check {
                        ctx.say("The bot is currently in development mode (which means only the server owner(s) can use the bot), please try again later.")
                            .await?;
                        return Ok(false);
                    }
                    Ok(true)
                })
            }),
            // This code is run before every command
            pre_command: |ctx| {
                Box::pin(async move {
                    println!("Executing command {}...", ctx.command().qualified_name);
                })
            },
            // This code is run after a command if it was successful (returned Ok)
            post_command: |ctx| {
                Box::pin(async move {
                    println!("Executed command {}!", ctx.command().qualified_name);
                })
            },
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(async move {
                    println!(
                        "Got an event in event handler: {:?}",
                        event.snake_case_name()
                    );
                    Ok(())
                })
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let mut activity = "".to_owned();
    activity.push_str(COMMAND_PREFIX);
    activity.push_str("help");

    if DEVELOPMENT {
        activity.push_str(" | DEV MODE");
    }

    let client = Client::builder(&token, intents)
        .framework(framework)
        .activity(ActivityData::custom(activity))
        .await
        .expect("Err creating client");

    Ok(client.into())
}
