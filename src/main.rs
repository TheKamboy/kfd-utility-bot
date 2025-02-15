use anyhow::Context as _;
use poise;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::ActivityData;
use rand::Rng;
use serenity::all::CreateMessage;
use serenity::model::id::UserId;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use std::ptr::null;

/* Projects
DONEPROJ: Basics
    DONE: Add Help Command
    DONE: Add Versions
    DONE: Add ChangeLog
PROJ: v0.0.5-alpha milestone
    TODO: Verbal Warning Command
    DONE: Finish porting commands to poise
 */

/* Category's:
, category = "Utility"
, category = "Info"
, category = "Moderation"
*/

// Bot Settings
/// The name you want to give the bot ( default: KamFurDev's Utility Bot )
const NAME: &str = "KamFurDev's Utility Bot";

/// The prefix for text commands ( default: ; )
const COMMAND_PREFIX: &str = ";";

/// The current version of the bot
const VERSION: &str = "v0.0.5-alpha";

/// Blocks commands from being sent unless it is sent from the owners ( default: false )
const DEVELOPMENT: bool = true;

// Channels
const LOG_CHANNEL: u64 = 1314766735030747218;

// Roles
const OWNER_ROLES: [u64; 2] = [1314451651704393770, 1314451866289176616]; // owner and co-owner
const ADMIN_ROLE: u64 = 1314453508409262130;
const HIGHER_MOD_ROLE: u64 = 1320629413154525265; // higher ranked mod
const MOD_ROLE: u64 = 1314454674111467602;
const TRIAL_MOD_ROLE: u64 = 1314454804369510421;

// Changelog
const CHANGELOG_MSG: &str = "# Changelog
## v0.0.5-alpha
- moved commands to poise framework (so you can have slash commands as well)
- verbal warning command
## v0.0.1-alpha
- default serenity hello world command added
- help message added
- version message added
- changelog added";

// fuckin hell i gotta do a rewrite of all my shit
struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
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

/// Sends a verbal warning to the user specified, with a reason
#[poise::command(slash_command, prefix_command, category = "Moderation")]
async fn verbal_warn(
    ctx: Context<'_>,
    #[description = "User"] user: serenity::User,
    #[description = "Reason"] reason: String,
) -> Result<(), Error> {
    if !check_for_roles(
        &ctx,
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

/// Sends the changelog to your DMs
#[poise::command(slash_command, prefix_command, category = "Info")]
async fn changelog(ctx: Context<'_>) -> Result<(), Error> {
    let message = CreateMessage::new().content(CHANGELOG_MSG);
    ctx.reply("I sent the changelog to your DMs.").await?;
    ctx.author().dm(ctx.http(), message).await?;
    Ok(())
}

/// Displays the current version of the bot
#[poise::command(slash_command, prefix_command, category = "Info")]
async fn version(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(format!("{NAME} {VERSION}").as_str()).await?;
    Ok(())
}

/// Shows commands you can run using the bot
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
    let mut message = "You do not have permission to do that.".to_string();
    let num = rand::rng().random_range(0..100);

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

async fn check_for_roles(ctx: &Context<'_>, roles: &[u64]) -> bool {
    for role in roles {
        if ctx
            .author()
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
            commands: vec![help(), version(), changelog(), verbal_warn(), account_age()],
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
