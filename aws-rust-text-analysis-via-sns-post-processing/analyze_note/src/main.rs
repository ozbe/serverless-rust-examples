use aws_lambda_events::event::sns::SnsEvent;
use lambda::{handler_fn, Context};
use log::info;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    pretty_env_logger::init();
    lambda::run(handler_fn(analyze_note)).await?;
    Ok(())
}

async fn analyze_note(event: SnsEvent, _: Context) -> Result<(), Error> {
    let note = event
        .records
        .first()
        .and_then(|r| r.sns.message.as_ref())
        .ok_or("Missing note")?;
    let result = sentiment::analyze(note.clone());
    if result.score > 2f32 {
        info!("Positive note - will be published: {}", note);
    } else {
        info!("Negative note - won't be published: {}", note);
    }
    Ok(())
}
