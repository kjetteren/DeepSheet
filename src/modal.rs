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
    let data = MyModal::execute(ctx).await?;
    println!("Got data: {:?}", data);

    Ok(())
}