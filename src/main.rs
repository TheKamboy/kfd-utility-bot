use ::serenity::all::{EmojiIdentifier, Reaction, ReactionType};
use anyhow::Context as _;
use poise;
use poise::futures_util::TryFutureExt;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::ActivityData;
use rand::Rng;
use serenity::all::{CreateMessage, User};
use serenity::model::id::UserId;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use shuttle_runtime::SecretStore;
use std::ptr::null;
/* Projects
BACKLOG:
    !: Add poll lock
PROJ: v0.0.6a-alpha
    TODO: Add random warn messages
 */

/* Category's:
, category = "Utility"
, category = "Info"
, category = "Moderation"
*/

// Bot Settings
/// The name you want to give the bot. ( default: KamFurDev's Utility Bot )
const NAME: &str = "KamFurDev's Utility Bot";

/// The prefix for text commands. ( default: ; )
const COMMAND_PREFIX: &str = ";";

/// The current version of the bot.
const VERSION: &str = "v0.0.6a-alpha";

/// Blocks commands from being sent unless it is sent from the owners. ( default: false )
const DEVELOPMENT: bool = false;

// Channels
const LOG_CHANNEL: u64 = 1314766735030747218;

// Roles
const OWNER_ROLES: [u64; 2] = [1314451651704393770, 1314451866289176616]; // owner and co-owner
const ADMIN_ROLE: u64 = 1314453508409262130;
const HIGHER_MOD_ROLE: u64 = 1320629413154525265; // higher ranked mod
const MOD_ROLE: u64 = 1314454674111467602;
const TRIAL_MOD_ROLE: u64 = 1314454804369510421;

// Random Messages
const NOT_ALLOWED_MESSAGES: [&'static str; 5] = ["This is not a fucking painting!!! (unallowed action)", "You...shall not...pass! (unallowed action)", "My programmer doesn't want you doing this, so go away until you are allowed to do so!", "No.", "I'm sorry Dave, I'm afraid I can't do that."];

// Command Locks
// / Locks the `/poll` command for any role not in the list. If empty, any user can use it.

// Changelog
const CHANGELOG_MSG: &str = "# Current Update (v0.0.6a-alpha)
- random \"not allowed\" messages";

// fuckin hell i gotta do a rewrite of all my shit
struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Creates a poll for users to vote on.
#[poise::command(slash_command, prefix_command, category = "Utility")]
async fn poll(
    ctx: Context<'_>,
    #[description = "Title"] title: String,
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
    let o3 = option3.as_ref().unwrap_or_else(|| emptystring);
    let o4 = option4.as_ref().unwrap_or_else(|| emptystring);
    let o5 = option5.as_ref().unwrap_or_else(|| emptystring);
    let o6 = option6.as_ref().unwrap_or_else(|| emptystring);
    let o7 = option7.as_ref().unwrap_or_else(|| emptystring);
    let o8 = option8.as_ref().unwrap_or_else(|| emptystring);
    let o9 = option9.as_ref().unwrap_or_else(|| emptystring);
    let o0 = option10.as_ref().unwrap_or_else(|| emptystring);

    let authorname = &ctx.author().id.to_string();

    let mut message = format!("# {title}\n> Asked by: <@{authorname}>\n\n");
    message = format!("{message}:one: {option1}\n:two: {option2}");

    if o3 != "" {
        message = format!("{message}\n:three: {o3}");
    }
    if o4 != "" {
        message = format!("{message}\n:four: {o4}");
    }
    if o5 != "" {
        message = format!("{message}\n:five: {o5}");
    }
    if o6 != "" {
        message = format!("{message}\n:six: {o6}");
    }
    if o7 != "" {
        message = format!("{message}\n:seven: {o7}");
    }
    if o8 != "" {
        message = format!("{message}\n:eight: {o8}");
    }
    if o9 != "" {
        message = format!("{message}\n:nine: {o9}");
    }
    if o0 != "" {
        message = format!("{message}\n:keycap_ten: {o0}");
    }

    let msg = ctx.say(message).await?.into_message().await?;
    msg.react(ctx.http(), ReactionType::Unicode("1️⃣".to_string()))
        .await?;
    msg.react(ctx.http(), ReactionType::Unicode("2️⃣".to_string()))
        .await?;

    if o3 != "" {
        msg.react(ctx.http(), ReactionType::Unicode("3️⃣".to_string()))
            .await?;
    }
    if o4 != "" {
        msg.react(ctx.http(), ReactionType::Unicode("4️⃣".to_string()))
            .await?;
    }
    if o5 != "" {
        msg.react(ctx.http(), ReactionType::Unicode("5️⃣".to_string()))
            .await?;
    }
    if o6 != "" {
        msg.react(ctx.http(), ReactionType::Unicode("6️⃣".to_string()))
            .await?;
    }
    if o7 != "" {
        msg.react(ctx.http(), ReactionType::Unicode("7️⃣".to_string()))
            .await?;
    }
    if o8 != "" {
        msg.react(ctx.http(), ReactionType::Unicode("8️⃣".to_string()))
            .await?;
    }
    if o9 != "" {
        msg.react(ctx.http(), ReactionType::Unicode("9️⃣".to_string()))
            .await?;
    }
    if o0 != "" {
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

/// Sends a verbal warning to the user specified, with a reason.
#[poise::command(slash_command, prefix_command, category = "Moderation")]
async fn verbal_warn(
    ctx: Context<'_>,
    #[description = "User"] user: serenity::User,
    #[description = "Reason"] reason: String,
) -> Result<(), Error> {
    if !check_for_roles(
        &ctx,
        &ctx.author(),
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
    umention.push_str(">");
    let moderatorname = &ctx.author().name;
    let msgreason = reason;

    let logchannel = serenity::ChannelId::new(LOG_CHANNEL);

    logchannel
        .say(
            ctx.http(),
            format!("Sent a verbal warning to {umention}.\nModerator: {moderatorname}\nReason: {msgreason}"),
        )
        .await?;
    u.dm(
        ctx.http(),
        CreateMessage::new().content(format!(
            "{umention}\nThis is a verbal warning! Continued action with have consequences!\nModerator: {moderatorname}\nReason: {msgreason}"
        )),
    )
        .await?;

    ctx.reply("Done!").await?;
    Ok(())
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

// TODO
fn random_not_allowed_message() -> String {
    let mut message = String::new();
    let num = rand::rng().random_range(0..NOT_ALLOWED_MESSAGES.len());

    message = NOT_ALLOWED_MESSAGES[num].to_string();

    return message;
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

    return false;
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
                // utility
                account_age(),
                test_random_error(),
                poll(),
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
