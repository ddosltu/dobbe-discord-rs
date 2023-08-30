use serenity::framework::standard::{Args, CommandResult};
use serenity::framework::standard::macros::command;
use serenity::model::prelude::*;
use serenity::{prelude::*, async_trait};

use crate::data::{get_guild_config, write_guild_config};

#[command]
#[required_permissions(ADMINISTRATOR)]
async fn rules(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    if args.len() < 2 {
        msg.reply(&ctx, "Wrong usage!").await?;
    };

    let guild_id = msg.guild_id.expect("Could not find guild id");
    let guild = Guild::get(&ctx, guild_id).await.expect("Could not find guild");
    
    match args.single_quoted::<String>() {
        Ok(x) => {
            match x.as_str() {
                "role" => {
                    let role_name = args.rest();
                    if let Some(role) = guild.role_by_name(role_name) {
                        if let Ok(mut config) = get_guild_config(&guild_id) {
                            config.rules_role_id = Some(role.id);
                            write_guild_config(&guild_id, &config).expect("Error writing config");
                        }
                    } 
                },
                _ => {
                    msg.reply(&ctx, "Unknown argument!").await?;
                }
            }
        }
        Err(_) => {
            msg.reply(&ctx, "Could not parse arguments").await?;
        }
    }

    Ok(())
}

pub struct RulesHandler;

#[async_trait]
impl EventHandler for RulesHandler {
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        println!("Handling an reaction_add");
        
        let guild_id = reaction.guild_id.expect("Could not find guild id");
        let guild = Guild::get(&ctx, guild_id).await.expect("Could not find guild");
        
        let rules_channel_id = guild.rules_channel_id.expect("Could not find rules channel");
        let role = get_guild_config(&guild_id).unwrap().rules_role_id.expect("Could not get role id");

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