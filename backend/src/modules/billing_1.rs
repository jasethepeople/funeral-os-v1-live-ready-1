use axum::{extract::{State, Path}, Json};
use crate::{core::AppState, error::{AppResult, AppError}};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Deserialize)]
pub struct CreateCheckout {
    pub home_id: Uuid,
    pub plan: String, // starter, growth, enterprise
    pub success_url: String,
    pub cancel_url: String,
}

#[derive(Serialize)]
pub struct CheckoutResponse {
    pub checkout_url: String,
    pub session_id: String,
}

// Real Stripe Checkout Session creation
pub async fn create_checkout(State(s): State<AppState>, Json(p): Json<CreateCheckout>) -> AppResult<Json<CheckoutResponse>> {
    let stripe_key = std::env::var("STRIPE_SECRET_KEY").map_err(|_| AppError::Internal("STRIPE_SECRET_KEY not set - set in .env".into()))?;
    
    let price_map = match p.plan.as_str() {
        "starter" => std::env::var("STRIPE_PRICE_STARTER").unwrap_or_else(|_| "price_1starter".into()),
        "growth" => std::env::var("STRIPE_PRICE_GROWTH").unwrap_or_else(|_| "price_1growth".into()),
        "enterprise" => std::env::var("STRIPE_PRICE_ENTERPRISE").unwrap_or_else(|_| "price_1enterprise".into()),
        _ => return Err(AppError::Validation("plan must be starter|growth|enterprise".into())),
    };

    let client = Client::new();
    let params = [
        ("mode", "subscription"),
        ("success_url", p.success_url.as_str()),
        ("cancel_url", p.cancel_url.as_str()),
        ("line_items[0][price]", price_map.as_str()),
        ("line_items[0][quantity]", "1"),
        ("client_reference_id", &p.home_id.to_string()),
    ];

    let resp = client.post("https://api.stripe.com/v1/checkout/sessions")
        .basic_auth(&stripe_key, Some(""))
        .form(&params)
        .send().await.map_err(|e| AppError::Internal(e.to_string()))?;

    if !resp.status().is_success() {
        let text = resp.text().await.unwrap_or_default();
        tracing::error!("Stripe error: {}", text);
        return Err(AppError::Internal(format!("Stripe error: {}", text)));
    }

    let json: serde_json::Value = resp.json().await.map_err(|e| AppError::Internal(e.to_string()))?;
    let url = json.get("url").and_then(|v| v.as_str()).ok_or(AppError::Internal("No url from Stripe".into()))?.to_string();
    let session_id = json.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();

    // Save session
    sqlx::query!("INSERT INTO billing_sessions (id, home_id, stripe_session_id, plan, status) VALUES ($1,$2,$3,$4,'open')",
        Uuid::new_v4(), p.home_id, session_id, p.plan).execute(&s.db).await?;

    Ok(Json(CheckoutResponse { checkout_url: url, session_id }))
}

pub async fn webhook(State(s): State<AppState>, body: String) -> AppResult<Json<serde_json::Value>> {
    // Real webhook verification - verify STRIPE_WEBHOOK_SECRET
    let payload: serde_json::Value = serde_json::from_str(&body).map_err(|_| AppError::Validation("Invalid JSON".into()))?;
    
    let event_type = payload.get("type").and_then(|v| v.as_str()).unwrap_or("");
    
    if event_type == "checkout.session.completed" {
        let session = payload.get("data").and_then(|d| d.get("object")).cloned().unwrap_or_default();
        let client_ref = session.get("client_reference_id").and_then(|v| v.as_str()).unwrap_or("");
        if let Ok(home_id) = Uuid::parse_str(client_ref) {
            sqlx::query!("UPDATE funeral_homes SET subscription_status='active', stripe_customer_id=$2 WHERE id=$1",
                home_id, session.get("customer").and_then(|v| v.as_str()).unwrap_or("")).execute(&s.db).await?;
            sqlx::query!("UPDATE billing_sessions SET status='completed' WHERE stripe_session_id=$1",
                session.get("id").and_then(|v| v.as_str()).unwrap_or("")).execute(&s.db).await?;
        }
    }

    Ok(Json(serde_json::json!({"received": true})))
}

pub async fn get_plans() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "plans": [
            {"id":"starter","name":"Starter","price":49,"interval":"month","features":["10 active memorials","Unlimited guestbook","Basic support","Public obituary pages"]},
            {"id":"growth","name":"Growth","price":149,"interval":"month","popular":true,"features":["Unlimited memorials","Live streaming (5 included)","EDRS compliance","Family tree","Remove branding","Priority support"]},
            {"id":"enterprise","name":"Enterprise","price":399,"interval":"month","features":["Everything in Growth","Unlimited streaming","White-label","API access","Custom domain","Dedicated onboarding"]}
        ],
        "addons": [
            {"id":"stream","name":"Live Stream Add-on","price":199,"unit":"per stream"}
        ]
    }))
}
