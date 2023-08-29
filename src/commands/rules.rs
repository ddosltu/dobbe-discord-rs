use serenity::framework::standard::{Args, CommandResult};
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::{prelude::*, async_trait};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct RulesConfig {
    rules_role_id: Option<RoleId>
}

const RULES_CONFIG_PATH: &str = "rules_config.toml";

fn read_config(path: &str) -> Result<RulesConfig, Box<dyn std::error::Error>> {    
    if let Ok(data) = fs::read(path) {
        let text = String::from_utf8(data)?;
        let config: RulesConfig = toml::from_str(&text)?;
        Ok(config)
    } else {
        Ok(
            RulesConfig {
                rules_role_id: None
            }
        )
    }
}

fn write_config(config: &RulesConfig, path: &str) -> Result<(), Box<dyn std::error::Error>> {
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

    if let Ok(guild) = Guild::get(&ctx, msg.guild_id.unwrap()).await  {
        match args.single_quoted::<String>() {
            Ok(x) => {
                match x.as_str() {
                    "role" => {
                        let role_name = args.rest();
                        if let Some(role) = guild.role_by_name(role_name) {
                            if let Ok(mut config) = read_config(RULES_CONFIG_PATH) {
                                config.rules_role_id = Some(role.id);
                                write_config(&config, RULES_CONFIG_PATH)
                                    .expect("Failed to write rules config");
                            }
                        } 
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

pub struct RulesHandler;

#[async_trait]
impl EventHandler for RulesHandler {
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        println!("Handling an reaction_add");
        if let Ok(guild) = Guild::get(&ctx, reaction.guild_id.unwrap()).await  {
            println!("Found guild");

            let rules_channel_id = guild.rules_channel_id.expect("Could not find rules channel");
            let role = read_config(RULES_CONFIG_PATH).unwrap().rules_role_id.expect("Could not get role id");

            if reaction.channel_id.eq(&rules_channel_id) {
                if let Some(user_id) = reaction.user_id {
                    if let Ok(mut member) = guild.member(&ctx, user_id).await {
                        member.add_role(&ctx, role).await.unwrap();
                        println!("Gave {} the role {}", member.display_name(), role.to_string())
                    }
                }
            }

        }
    }
}