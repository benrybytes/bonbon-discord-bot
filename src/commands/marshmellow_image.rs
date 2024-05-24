
// Server command specifically made

use std::task::Context;

use serenity::all::{CacheHttp, CommandInteraction, CreateCommand, CreateEmbed, CreateMessage, MessageBuilder, ResolvedOption};
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;

#[derive(Debug)]
struct DefaultContent<'a> {
    title: &'a str,
    description: &'a str,
    image: &'a str
}

fn create_embed_image_and_text(content: DefaultContent) -> CreateEmbed {

    CreateEmbed::new()
        .title(content.title)
        .description(content.description)
        .image(content.image)

}

pub async fn run(ctx: impl CacheHttp, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let content = DefaultContent {
        title: "Le marshmellow", description: "swagger marsh", image: "https://media.istockphoto.com/id/1396739378/vector/cute-marshmallow-food-character-mascot-vector-illustration-design.jpg?s=612x612&w=0&k=20&c=FNpH5Lbcs1jVKWO_nvjN0pKIb9fHBcI_sCSQ73UbObw="
    };

    println!("{:#?}", content); 

    let message = CreateInteractionResponseMessage::new()
        .embed(create_embed_image_and_text(content));

    let _ = match interaction.create_response(ctx, CreateInteractionResponse::Message(message)).await {
        Ok(()) => (),
        Err(e) => panic!("{}", e)
    };
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new(String::from("marshmellow"))
    .description("Spawns a marshmellow image")
    .name("marshmellow")
}
