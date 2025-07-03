use crate::{Data, Error};
use poise::serenity_prelude as serenity;

#[derive(Debug, poise::Modal)]
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
pub async fn modal(ctx: poise::ApplicationContext<'_, Data, Error>) -> Result<(), Error> {
    use poise::Modal as _;

    let data = MyModal::execute(ctx).await?;
    println!("Got data: {:?}", data);

    Ok(())
}

#[poise::command(slash_command)]
pub async fn component_modal(ctx: crate::Context<'_>) -> Result<(), Error> {
    let reply = {
        let components = vec![serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new("open_modal")
                .label("Open modal")
                .style(poise::serenity_prelude::ButtonStyle::Success),
        ])];

        poise::CreateReply::default()
            .content("Click the button below to open the modal")
            .components(components)
    };

    ctx.send(reply).await?;

    while let Some(mci) = serenity::ComponentInteractionCollector::new(ctx.serenity_context())
        .timeout(std::time::Duration::from_secs(120))
        .filter(move |mci| mci.data.custom_id == "open_modal")
        .await
    {
        let data =
            poise::execute_modal_on_component_interaction::<MyModal>(ctx, mci, None, None).await?;
        println!("Got data: {:?}", data);
    }
    Ok(())
}