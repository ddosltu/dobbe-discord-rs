mod commands;

use serenity::async_trait;
use serenity::framework::StandardFramework;
use serenity::framework::standard::macros::group;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::commands::ping::*;
use crate::commands::rules::*;

#[group]
#[only_in(guilds)]
#[commands(ping, rules)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {

    // Configure the client with your Discord bot token in the environment.
    dotenv::dotenv().expect("Could not read .env");
    let token = dotenv::var("DISCORD_TOKEN").expect("Expected a token in .env");
    
    let framework = StandardFramework::new()
                        .configure(|c| c.prefix("*")) // set the bot's prefix
                        .group(&GENERAL_GROUP);

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::DIRECT_MESSAGES
                | GatewayIntents::MESSAGE_CONTENT
                | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client =
        Client::builder(&token, intents)
        .event_handler(Handler)
        .event_handler(RulesHandler)
        .framework(framework)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

