use lambda_http::{handler, http::header, lambda, Body, Context, IntoResponse, Request, Response};
use log::error;
use mime::{Mime, APPLICATION_JAVASCRIPT, TEXT_PLAIN};
use rusoto_sns::{PublishInput, PublishResponse, Sns, SnsClient};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

thread_local!(
    static SNS: SnsClient = SnsClient::new(Default::default());
);

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[derive(Deserialize)]
struct RequestBody {
    note: String,
}

#[derive(Serialize)]
struct ResponseBody {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    pretty_env_logger::init();
    lambda::run(handler(add_note)).await?;
    Ok(())
}

async fn add_note(request: Request, ctx: Context) -> Result<impl IntoResponse, Error> {
    let note = if Ok(n) = convert_to_note(request) {
        n
    } else {
        error!("Validation Failed");
        return build_response(400, TEXT_PLAIN, "Couldn\'t add the note.".to_string());
    };
    let aws_account_id = parse_aws_account_id(&ctx);

    publish(note, aws_account_id)
        .await
        .and_then(|_| {
            let response = &ResponseBody {
                message: "Successfully added the note.".into(),
            };
            build_response(
                200,
                APPLICATION_JAVASCRIPT,
                serde_json::to_string(&response)?,
            )
        })
        .or_else(|e| {
            error!("{}", e);
            build_response(
                501,
                TEXT_PLAIN,
                "Couldn\'t add the note due an internal error. Please try again later.".into(),
            )
        })
}

fn convert_to_note(request: Request) -> Result<String, Error> {
    request
        .body()
        .json()
        .map(|r: RequestBody| r.note)
        .ok_or_else(|| "Invalid body type".into())
}

async fn publish(note: impl ToString, aws_account_id: &str) -> Result<PublishResponse, Error> {
    let mut params = PublishInput::default();
    params.message = note.to_string();
    params.topic_arn = Some(format!(
        "arn:aws:sns:us-east-1:{}:analyzeNote",
        aws_account_id
    ));
    SNS.with(|sns| {
        let sns = sns.clone();
        async move { sns.publish(params).await }
    })
    .await
    .map_err(|e| e.into())
}

fn build_response<T>(status: u16, content_type: Mime, body: T) -> Result<Response<T>, Error> {
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, content_type.to_string())
        .body(body)
        .map_err(|e| e.into())
}

fn parse_aws_account_id(ctx: &Context) -> &str {
    ctx.invoked_function_arn.split(':').nth(4).unwrap()
}

trait BodyExt {
    fn text(&self) -> Option<&str>;
    fn json<T: DeserializeOwned>(&self) -> Option<T>;
}

impl BodyExt for Body {
    fn text(&self) -> Option<&str> {
        if let Body::Text(ref t) = self {
            Some(t)
        } else {
            None
        }
    }

    fn json<T: DeserializeOwned>(&self) -> Option<T> {
        self.text().and_then(|t| serde_json::from_str(t).ok())
    }
}
