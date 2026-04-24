use axum::Json;
use leptos::serde_json::Value;
use leptos::server_fn::serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct CalculatePayload {
    expression: String,
    is_correct: bool,
}

pub async fn calculate(Json(payload): Json<CalculatePayload>) -> Json<Value> {
    let expression = payload.expression;
    let mut result = meval::eval_str(&expression).unwrap_or(0.0);

    if !payload.is_correct {
        result -= 1.0;
    }

    let formatted = if result.fract() == 0.0 {
        json!(result as i64)
    } else {
        json!(result)
    };

    Json(json!({
        "result": formatted,
    }))
}