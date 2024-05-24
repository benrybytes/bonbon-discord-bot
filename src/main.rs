use std::env;

use serenity::all::{Command, GuildId, Interaction, Ready};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

mod commands;

struct Handler;

#[derive(Debug)]
struct CustomMessage {
    content: Message
}

#[async_trait]
impl EventHandler for Handler { 
    async fn ready(&self, ctx: Context, ready: Ready) {
        /* let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        ); */
        

        let guild_command =
            Command::create_global_command(&ctx.http, commands::marshmellow_image::register())
                .await;

        println!("I created the following global slash command: {guild_command:#?}");
    }
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            let resulting_response: CustomMessage = match msg.channel_id.say(&ctx.http, "hello").await {
                Ok(content) => {
                let cum = CustomMessage {
                    content
                };
                println!("{:?}", cum.content);
                cum
            }, 
                Err(error) => panic!("{}", error) 
            };
            println!("{:?}", resulting_response);

            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {

            // Get responses or run it from here from the message embed
            let content = match command.data.name.as_str() {
                "marshmellow" => {
                    let _ = commands::marshmellow_image::run(&ctx.http, &command).await;
                    None
                },
                _ => Some("Command not made".to_string()),
            };


            // Send messages if were returned in content and not ran prior to here, run the commands that return a value besides None
            /* if let Some(Content) = content {
                let data =
            } */



        }
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    // To use: export DISCORD_TOKEN="{token_content}" in shell
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");


    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
