use std::future::{Ready, ready};
use std::ops::Deref;
use std::sync::Arc;

use actix_web::{Error, FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::error::ErrorBadRequest;
use async_trait::async_trait;
use uuid::Uuid;

use crate::modules::shared::models::entities::user::User;
use crate::modules::shared::models::enums::repositories_enum::RepositoryError;

pub type RepositoryResult<T> = Result<T, RepositoryError>;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait Repository: Send + Sync + 'static {
	async fn get_user(&self, user_id: &Uuid) -> RepositoryResult<User>;
	async fn create_user(&self, user: &User) -> RepositoryResult<User>;
	async fn update_user(&self, user: &User) -> RepositoryResult<User>;
	async fn delete_user(&self, user_id: &Uuid) -> RepositoryResult<Uuid>;
}

pub struct RepositoryInjector(Arc<Box<dyn Repository>>);

impl Clone for RepositoryInjector {
	fn clone(&self) -> Self {
		let repo = self.0.clone();
		Self(repo)
	}
}

impl Deref for RepositoryInjector {
	type Target = dyn Repository;
	
	fn deref(&self) -> &Self::Target {
		self.0.as_ref().as_ref()
	}
}

impl FromRequest for RepositoryInjector {
	type Error = Error;
	type Future = Ready<Result<Self, Self::Error>>;
	
	fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
		if let Some(injector) = req.app_data::<Self>() {
			ready(Ok(injector.to_owned()))
		} else {
			ready(Err(ErrorBadRequest("Not repository injector was found in the request")))
		}
	}
}
