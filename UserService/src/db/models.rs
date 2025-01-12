use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,                      // Unique identifier
    pub first_name: String,            // First name
    pub last_name: String,             // Last name
    pub email: String,                 // Primary email address
    pub password_hash: String,         // Hashed password
    pub phone_number: Option<String>,  // Optional phone number
    pub secondary_email: Option<String>, // Optional secondary email
    pub mailing_address: Option<String>, // Mailing address
    pub secondary_address: Option<String>, // Secondary address
    pub is_active: bool,               // Indicates if the user account is active
    pub role: String,                  // Role (e.g., "customer", "admin")
    pub created_at: DateTime<Utc>,     // Timestamp for account creation
    pub updated_at: DateTime<Utc>,     // Timestamp for the last update
}
