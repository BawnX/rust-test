use uuid::Uuid;
use crate::modules::shared::models::entities::user::User;
use crate::modules::shared::repositories::repository::{Repository, RepositoryResult};

use async_trait::async_trait;
use chrono::Utc;
use tracing::instrument;
use crate::modules::shared::models::enums::repositories_enum::RepositoryError;

pub struct PostgresRepository {
	pool: sqlx::PgPool,
}

impl PostgresRepository {
	pub async fn from_env() -> sqlx::Result<Self> {
		let conn_str = std::env::var("DATABASE_URL").map_err(|e| sqlx::Error::Configuration(Box::new(e)))?;
		let pool = sqlx::PgPool::connect(&conn_str).await?;
		Ok(Self { pool })
	}
}

#[async_trait]
impl Repository for PostgresRepository {
	#[instrument(skip(self))]
	async fn get_user(&self, user_id: &Uuid) -> RepositoryResult<User> {
		let result = sqlx::query_as::<_, User>(
			"select * from users where id = $1",
		).bind(user_id).fetch_one(&self.pool).await;
		
		result.map_err(|e| {
			tracing::error!("{:?}", e);
			RepositoryError::InvalidId
		})
	}
	
	#[instrument(skip(self))]
	async fn create_user(&self, user: &User) -> RepositoryResult<User> {
		let result = sqlx::query_as::<_, User>(
			r#"
			insert into users (id, name, birth_date, custom_data)
			values ($1, $2, $3, $4 )
			returning id, name, birth_date, custom_data, created_at, updated_at
			"#,
		).bind(&user.id).bind(&user.name).bind(&user.birth_date).bind(&user.custom_data).fetch_one(&self.pool).await;

		result.map_err(|e| {
			tracing::error!("{:?} database pool", e);
			RepositoryError::AlreadyExists
		})
	}
	
	#[instrument(skip(self))]
	async fn update_user(&self, user: &User) -> RepositoryResult<User> {
		let result = sqlx::query_as::<_, User>(
			r#"
			update users
			set custom_data = $1, updated_at = $2
			where id = $3
			returning id, name, birth_date, custom_data, created_at, updated_at
			"#,
		).bind(&user.custom_data).bind(Utc::now()).bind(&user.id).fetch_one(&self.pool).await;
		
		result.map_err(|e| {
			tracing::error!("{:?}", e);
			RepositoryError::DoesNotExist
		})
	}
	
	#[instrument(skip(self))]
	async fn delete_user(&self, user_id: &Uuid) -> RepositoryResult<Uuid> {
		let result = sqlx::query_as::<_, User>(
			r#"
			delete from users
			where id = $1
			returning id
			"#,
		).bind(&user_id).fetch_one(&self.pool).await;
		
		result.map(|u| u.id).map_err(|e| {
			tracing::error!("{:?}", e);
			RepositoryError::DoesNotExist
		})
	}
}
