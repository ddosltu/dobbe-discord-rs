use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandResult, Args};
use serenity::model::prelude::*;
use serenity::{prelude::*, async_trait};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct RulesConfig {
    channel_id: u64,
    rules_role_id: u64
}

fn read_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let data = fs::read(path)?;
    let text = String::from_utf8(data)?;
    let config: Config = toml::from_str(&text)?;
    Ok(config)
}

fn write_config(config: &Config, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let text = toml::to_string(config)?;
    std::fs::write(path, text)?;
    Ok(())
}

#[command]
#[required_permissions(ADMINISTRATOR)]
async fn rules(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    if args.len() < 2 {
        msg.reply(&ctx, "Wrong usage!").await?;
    };

    if let Some(guild) = msg.guild(&ctx) {
        match args.single_quoted::<String>() {
            Ok(x) => {
                match x.as_str() {
                    "channel" => {
                        let channel_name = args.rest();
                        if let Some(channel) = guild.channel_id_from_name(&ctx, channel_name) {
                            
                        }
                    },
                    "role" => {
    
                    },
                    _ => {}
                }
            }
            Err(_) => {
                msg.reply(&ctx, "Error!").await?;
            }
        };
    
    }

    Ok(())
}

const RULES_CHANNEL_ID: u64 = 1110930405667905596;
const ACCEPTED_RULES_ROLE_ID: u64 = 1110930504313733281;

pub struct RulesHandler;

#[async_trait]
impl EventHandler for RulesHandler {
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        println!("Handling an reaction_add");
        if reaction.channel_id.0 == RULES_CHANNEL_ID {
            if let Some(guild_id) = reaction.guild_id {
                if let Some(user_id) = reaction.user_id {
                    if let Ok(mut member) = guild_id.member(&ctx, user_id).await {
                        member.add_role(&ctx, ACCEPTED_RULES_ROLE_ID).await.unwrap();
                    }
                }
            }
        }
    }
}