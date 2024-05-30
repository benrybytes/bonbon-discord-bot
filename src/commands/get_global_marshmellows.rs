

// Server command specifically made

use std::task::Context;

use serenity::all::{CacheHttp, CommandInteraction, CreateCommand};
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};

// Future function that needs to be waited in the parent function for a response to be made
pub async fn run(ctx: impl CacheHttp, interaction: &CommandInteraction, count: f64) -> Result<(), serenity::Error> {
    let message = CreateInteractionResponseMessage::new()
        .content(format!("Total marshmellows posted: {}", count));

    let _ = match interaction.create_response(ctx, CreateInteractionResponse::Message(message)).await {
        Ok(()) => (),
        Err(e) => panic!("{}", e)
    };
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new(String::from("get-all-posted-marshmellows"))
    .description("All marshmellows made in all servers")
    .name("get-all-posted-marshmellows")
}
