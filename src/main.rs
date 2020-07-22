use aws_lambda_events::event::apigw::ApiGatewayV2httpResponse;
use lambda_runtime::{error::HandlerError, lambda, Context};
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
struct ResponseBody<T: Serialize> {
    message: String,
    input: T,
}

fn main() {
    lambda!(handler);
}

fn handler(e: Value, _: Context) -> Result<ApiGatewayV2httpResponse, HandlerError> {
    let body = {
        let response = ResponseBody {
            message: "Go Serverless v1.0! Your function executed successfully!".into(),
            input: e,
        };
        serde_json::to_string(&response)?
    };

    Ok(ApiGatewayV2httpResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(body),
        is_base64_encoded: None,
        cookies: vec![]
    })
}
