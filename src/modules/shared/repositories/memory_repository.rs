use std::sync::RwLock;

use chrono::Utc;
use uuid::Uuid;
use async_trait::async_trait;
use tracing::instrument;
use crate::modules::shared::models::entities::user::User;
use crate::modules::shared::models::enums::repositories_enum::RepositoryError;
use crate::modules::shared::repositories::repository::{Repository, RepositoryResult};

pub struct MemoryRepository {
	users: RwLock<Vec<User>>,
}

impl Default for MemoryRepository {
	fn default() -> Self {
		Self {
			users: RwLock::new(vec![])
		}
	}
}

#[async_trait]
impl Repository for MemoryRepository {
	#[instrument(skip(self))]
	async fn get_user(&self, user_id: &Uuid) -> RepositoryResult<User> {
		let users = self.users.read()?;
		users
			.iter()
			.find(|u| &u.id == user_id)
			.ok_or_else(|| RepositoryError::InvalidId)
			.cloned()
	}
	
	#[instrument(skip(self))]
	async fn create_user(&self, user: &User) -> RepositoryResult<User> {
		if self.get_user(&user.id).await.is_ok() {
			return Err(RepositoryError::AlreadyExists);
		}
		
		let mut new_user = user.to_owned();
		new_user.created_at = Some(Utc::now());
		let mut users = self.users.write().unwrap();
		users.push(new_user.clone());
		Ok(new_user)
	}
	
	#[instrument(skip(self))]
	async fn update_user(&self, user: &User) -> RepositoryResult<User> {
		if let Ok(old_user) = self.get_user(&user.id).await {
			let mut update_user = user.to_owned();
			update_user.created_at = old_user.updated_at;
			update_user.updated_at = Some(Utc::now());
			let mut users = self.users.write().unwrap();
			users.retain(|r| r.id != user.id);
			users.push(update_user.clone());
			tracing::debug!("User with id {} correctly updated", user.id);
			Ok(update_user)
		} else {
			tracing::error!("User {} does not exits", user.id);
			Err(RepositoryError::DoesNotExist)
		}
	}
	
	#[instrument(skip(self))]
	async fn delete_user(&self, user_id: &Uuid) -> RepositoryResult<Uuid> {
		let mut users = self.users.write()?;
		users.retain(|r| &r.id != user_id);
		Ok(user_id.to_owned())
	}
}
