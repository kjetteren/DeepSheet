use poise::Modal;
use crate::{Data, Error};
type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>;

#[derive(Debug, Modal)]
#[allow(dead_code)]
#[name = "Create an Announcement"]
struct MyModal {
    #[name = "Movie Title"]
    #[placeholder = "Enter the title of the movie"]
    title: String,
    #[placeholder = "MM/DD"]
    date: String,
    #[placeholder = "HH:MM"]
    time: String,
}

#[poise::command(slash_command)]
pub async fn announce(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    let mut role_id = None;

    if let Some(guild_id) = ctx.guild_id() {
        if let Some(guild) = guild_id.to_guild_cached(&ctx) {
            role_id = guild
                .role_by_name("Kino Enjoyers")
                .map(|role| role.id);
        }
    }

    if let Some(data) = MyModal::execute(ctx).await? {
        if let Some(role_id) = role_id {
            let content = poise::serenity_prelude::utils::MessageBuilder::new()
                .role(role_id)
                .push(", ")
                .push(data.title)
                .push(": on ")
                .push(data.date)
                .push(" at ")
                .push(data.time)
                .build();

            ctx.say(content)
                .await
                .map_err(Error::from)?;
        }
    }

    Ok(())
}