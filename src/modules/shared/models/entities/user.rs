use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;
use sqlx::{FromRow, Type};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
	pub id: Uuid,
	pub name: String,
	pub birth_date: NaiveDate,
	pub custom_data: CustomData,
	pub created_at: Option<DateTime<Utc>>,
	pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name="custom_data")]
pub struct CustomData {
	pub random: i32,
}

