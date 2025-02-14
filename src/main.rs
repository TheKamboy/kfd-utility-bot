use anyhow::Context as _;
use serenity::async_trait;
use serenity::builder::CreateMessage;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use shuttle_runtime::SecretStore;
use tracing::{error, info};

/* TODOs
DONEPROJ: Basics
    DONE: Add Help Command
    DONE: Add Versions
    DONE: Add ChangeLog
PROJ: Moderation Commands
    TODO: Verbal Warning Command
CANNED: Updates
    TODO: Utility Bot Changelog as a channel
    TODO: Update Changelog command
 */

// Bot Settings
const NAME: &str = "KamFurDev's Utility Bot";
const COMMAND_PREFIX: &str = ";";
const VERSION: &str = "v0.0.2-alpha";

// Channels
const LOG_CHANNEL: i64 = 1314766735030747218;

// Roles
const OWNER_ROLES: [i64; 2] = [1314451651704393770, 1314451866289176616]; // owner and co-owner
const ADMIN_ROLE: i64 = 1314453508409262130;
const HIGHER_MOD_ROLE: i64 = 1320629413154525265; // higher ranked mod
const MOD_ROLE: i64 = 1314454674111467602;
const TRIAL_MOD_ROLE: i64 = 1314454804369510421;

// Help Command
const HELP_COMMANDS: &str = "## Basic Commands
- `help`: shows this message
- `version`: gives the version of the bot
- `changelog`: sends the changelog to your dms
- `hello`: responds with \"world!\"
## Moderation Commands
- `/warn verbal`: gives a user a verbal warning";

// Changelog
const CHANGELOG_MSG: &str = "# Changelog
## v0.0.1-alpha
- default serenity hello world command added
- help message added
- version message added
- changelog added";

fn check_command(msg: &Message, command: &str) -> bool {
    let mut full_command = "".to_owned();
    full_command.push_str(COMMAND_PREFIX);
    full_command.push_str(command);
    if msg.content.to_lowercase() == full_command {
        return true;
    }
    return false;
}

fn check_command_start(msg: &Message, command: &str) -> bool {
    let mut full_command = "".to_owned();
    full_command.push_str(COMMAND_PREFIX);
    full_command.push_str(command);
    full_command.push_str(" ");
    if msg.content.to_lowercase().contains(full_command.as_str()) {
        return true;
    }
    return false;
}

/// mode: 0 = the regular help message, 1 = just the version
async fn send_help_message(ctx: &Context, msg: &Message, mode: i32) {
    if mode == 0 {
        let response = MessageBuilder::new()
            .push_bold_safe(NAME)
            .push(" ")
            .push_bold_safe(VERSION)
            .push("\n")
            .push("Prefix: ")
            .push_mono_safe(COMMAND_PREFIX)
            .push("\n")
            .push("# Commands\n")
            .push(HELP_COMMANDS)
            .build();

        let finish = CreateMessage::new().content(response);

        if let Err(e) = msg.author.dm(&ctx.http, finish).await {
            error!("Error sending message: {:?}", e);
        }
    } else if mode == 1 {
        let response = MessageBuilder::new()
            .push_bold_safe(NAME)
            .push(" ")
            .push_bold_safe(VERSION)
            .build();

        if let Err(e) = msg.channel_id.say(&ctx.http, &response).await {
            error!("Error sending message: {:?}", e);
        }
    }
}

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        // basic shit
        if check_command(&msg, "hello") {
            if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
                error!("Error sending message: {:?}", e);
            }
        } else if check_command(&msg, "help") {
            send_help_message(&ctx, &msg, 0).await;
        } else if check_command(&msg, "version") {
            send_help_message(&ctx, &msg, 1).await;
        } else if check_command(&msg, "changelog") {
            let response = CreateMessage::new().content(CHANGELOG_MSG);

            if let Err(e) = msg.author.dm(&ctx.http, response).await {
                error!("Error sending message: {:?}", e);
            }
        }

        // mod commands

    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
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

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
