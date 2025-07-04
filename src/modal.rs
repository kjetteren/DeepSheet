use chrono::{NaiveDateTime, TimeZone};
use chrono_tz::Europe::Brussels;
use poise::Modal;
use poise::serenity_prelude::utils::MessageBuilder;
use crate::{Data, Error};
type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>;

#[derive(Debug, Modal)]
#[allow(dead_code)]
#[name = "Create an Announcement"]
struct MyModal {
    #[name = "Movie Title"]
    #[placeholder = "Enter the title of the movie"]
    title: String,
    #[name = "Date and Time"]
    #[placeholder = "DD/MM/YY HH:MM"]
    date_time: String,
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
            let dt = NaiveDateTime::parse_from_str(&*data.date_time, "%d/%m/%y %H:%M").unwrap();
            let dt_timezone = Brussels.from_local_datetime(&dt).unwrap();
            let timestamp = dt_timezone.timestamp();

            let content = MessageBuilder::new()
                .role(role_id)
                .push(", ")
                .push(data.title)
                .push(": <t:")
                .push(timestamp.to_string())
                .push(":F>")
                .build();

            ctx.say(content)
                .await
                .map_err(Error::from)?;
        }
    }

    Ok(())
}