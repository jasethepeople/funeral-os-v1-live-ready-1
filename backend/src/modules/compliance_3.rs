use axum::{extract::{State, Path}, Json};
use crate::{core::AppState, error::{AppResult, AppError}, core::crypto::{encrypt_field, decrypt_field}};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct DeathCert {
    pub obituary_id: Uuid,
    pub state_code: String, // e.g. "UT", "CA"
    pub deceased_ssn_last4: String,
    pub cause_of_death: String,
    pub certifier_name: String,
    pub certifier_license: String,
    pub funeral_director_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct ComplianceResponse { pub id: Uuid, pub status: String, pub edrs_tracking: String, pub state_code: String }

pub async fn submit_certificate(State(s): State<AppState>, Json(p): Json<DeathCert>) -> AppResult<Json<ComplianceResponse>> {
    if p.state_code.len() != 2 {
        return Err(AppError::Validation("State code must be 2 letters".into()));
    }
    
    // Encrypt sensitive fields
    let encrypted_ssn = encrypt_field(&p.deceased_ssn_last4).map_err(|e| AppError::Internal(e.to_string()))?;
    let encrypted_cause = encrypt_field(&p.cause_of_death).map_err(|e| AppError::Internal(e.to_string()))?;
    
    let id = Uuid::new_v4();
    let tracking = format!("{}-{}-{}", p.state_code.to_uppercase(), chrono::Utc::now().format("%Y%m%d"), &id.to_string()[..8].to_uppercase());
    
    // State-specific validation (Lisp-style rules engine - implemented as Rust match)
    let required_fields_ok = match p.state_code.to_uppercase().as_str() {
        "UT" => p.certifier_license.len() >= 5,
        "CA" => p.certifier_license.len() >= 6,
        _ => true, // default
    };
    if !required_fields_ok {
        return Err(AppError::Validation(format!("Invalid license format for {}", p.state_code)));
    }
    
    let data = serde_json::json!({
        "ssn_last4_enc": encrypted_ssn,
        "cause_enc": encrypted_cause,
        "certifier_name": p.certifier_name,
        "certifier_license": p.certifier_license,
        "submitted_at": chrono::Utc::now()
    });
    
    sqlx::query!("INSERT INTO compliance_docs (id, obituary_id, state_code, status, data, tracking_number) VALUES ($1,$2,$3,'submitted',$4,$5)",
        id, p.obituary_id, p.state_code.to_uppercase(), data, tracking)
        .execute(&s.db).await?;
    
    // In production, this would call state EDRS API (e.g., Utah's UT-EDRS)
    // For now, we simulate submission and return tracking
    
    Ok(Json(ComplianceResponse { id, status: "submitted".into(), edrs_tracking: tracking, state_code: p.state_code.to_uppercase() }))
}

pub async fn get_status(State(s): State<AppState>, Path(id): Path<Uuid>) -> AppResult<Json<serde_json::Value>> {
    let row = sqlx::query!("SELECT id, state_code, status, tracking_number, created_at FROM compliance_docs WHERE id=$1", id)
        .fetch_optional(&s.db).await?.ok_or(AppError::NotFound("Doc not found".into()))?;
    Ok(Json(serde_json::json!({
        "id": row.id,
        "state_code": row.state_code,
        "status": row.status,
        "tracking": row.tracking_number,
        "created_at": row.created_at
    })))
}
