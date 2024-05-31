use anyhow::Context as _;
use serenity::all::{Command, Guild, GuildId, Interaction, Ready};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{Error, MySqlPool};

mod commands;

struct Handler {
    pool: MySqlPool,
} 

struct GlobalMarshmellowCount {
    count: Option<f64>,
}


#[derive(Debug)]
struct CustomMessage {
    content: Message
}

trait SQLHandlers {
    async fn global_add_marshmellow_count(&self, guild_id: &GuildId) -> Result<u64, Error>;
    async fn add_server_row(&self, guild_id: &GuildId) -> Result<(), Error>;
    async fn get_global_marshmellow_posted_count(&self) -> Result<f64, Error>;
    async fn check_bytes_table(&self) -> Result<(), Error>;

}

impl SQLHandlers for Handler {
// Change the global count for marshmellows
    async fn global_add_marshmellow_count(&self, guild_id: &GuildId) -> Result<u64, Error> {
        // Make sure to connect with prepare and setup sql db beforehand to be able to detect your sql data
        print!("Guild Id: {}", guild_id.get());

        // Errors in lsp talks about sql errors
        let mellow_id = sqlx::query!(
            r#"
            UPDATE global_servers_data
            SET marshmellow_counts = marshmellow_counts + 1
            WHERE guild_id = ?
            "#,
            guild_id.get(),
        )
        .execute(&self.pool)
        // Wait for the future result as a query result
        .await?
        .last_insert_id(); // Id count is the row number

        print!("Guild Id Updated {}", mellow_id);

        Ok(mellow_id)
    }

    async fn add_server_row(&self, guild_id: &GuildId) -> Result<(), Error> {

        let _ = sqlx::query!(

            r#"
            INSERT INTO global_servers_data (guild_id, marshmellow_counts)
            SELECT ?, 0
            WHERE NOT EXISTS (SELECT 1 FROM global_servers_data WHERE guild_id = ?);
            "#,
            guild_id.get(),
            guild_id.get()
        )
        .execute(&self.pool)
        .await?;
        
       Ok(()) 
    }

    // Handle the sum of numbers and store the query result to the Count struct and return it
    async fn get_global_marshmellow_posted_count(&self) -> Result<f64, Error> {
        let global_count_instance = sqlx::query_as!(
            GlobalMarshmellowCount,
            r#"
            SELECT SUM(marshmellow_counts) as count
            FROM global_servers_data;
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        let handle_count = match global_count_instance.count {
            Some(count) => {
                println!("marshmellow count found");
                count
            },
            None => 0.0
        }; 

        println!("Current global count: {}", handle_count);

        Ok(handle_count)
    }

    async fn check_bytes_table(&self) -> Result<(), Error> {

    let table_exists: (i32,) = sqlx::query_as(r#"
            SELECT COUNT(*)
            FROM INFORMATION_SCHEMA.TABLES
            WHERE TABLE_CATALOG = 'defaultdb' AND TABLE_NAME = 'global_servers_data'
        "#)
        .fetch_one(&self.pool)
        .await?;

        if table_exists.0 == 0 {
            // The table does not exist, so create it
            sqlx::query(r#"
            CREATE TABLE global_servers_data(guild_id CHAR(255), marshmellow_counts Long)
            "#)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

}

#[async_trait]
impl EventHandler for Handler { 
    async fn ready(&self, ctx: Context, _ready: Ready) {
        let guild_commands = vec![commands::marshmellow_image::register(), commands::get_global_marshmellows::register()];

        for command in guild_commands.iter() {
            let guild_command =
                Command::create_global_command(&ctx.http, command.clone())
                    .await;

            println!("I created the following global slash command: {guild_command:#?}");
        }

        self.check_bytes_table().await; 

        for guild in _ready.guilds.iter() {
           self.add_server_row(&guild.id).await;
        };

    }
    async fn guild_create(&self, _ctx: Context, guild: Guild, _is_new: Option<bool>) {
      let _ = self.add_server_row(&guild.id).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            let resulting_response: CustomMessage = match msg.channel_id.say(&ctx.http, "hello").await {
                Ok(content) => {
                let temp_message = CustomMessage {
                    content
                };
                println!("{:?}", temp_message.content);
                temp_message
            }, 
                Err(error) => panic!("{}", error) 
            };
            println!("{:?}", resulting_response);

            // 
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {

            // Get responses content or run it from here from the message embed
            let _content = match command.data.name.as_str() {
                "marshmellow" => {
                    let _ = commands::marshmellow_image::run(&ctx.http, &command).await;
                    // We still want to use guild_id after running command
                    let guild_id = &command.guild_id.expect("Guild ID not found");
                    // Wait for the future / async to be finished and get a response
                    let _ = self.global_add_marshmellow_count(&guild_id).await;
                    None
                },
                "get-all-posted-marshmellows" => {

                    // Wait for the SQL data to be fetched from the future data instead of a builder
                    let count = self.get_global_marshmellow_posted_count().await.unwrap();
                    let _ = commands::get_global_marshmellows::run(&ctx.http, &command, count).await; // Wait for the future 
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

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Login with a bot token from the environment
    // To use: export DISCORD_TOKEN="{token_content}" in shell

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let token = secrets
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let database_url = secrets
        .get("DATABASE_URL")
        .context("'Database url' was not found")?;
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await.expect("DB cannot be created");
    println!("{}", database_url);

    // Create a new instance of the Client, logging in as a bot.
    // Await the future of event_handler to be able to fetch the Client Result after ClientBuilder is done
    let mut client =
        Client::builder(&token, intents).event_handler(Handler { pool }).await.expect("Client not found");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }

    Ok(client.into())
}

