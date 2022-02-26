use std::sync::PoisonError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
	#[error("Poison Error: `{0}`")]
	LockError(String),
	#[error("This entity already exists")]
	AlreadyExists,
	#[error("This entity does not exists")]
	DoesNotExist,
	#[error("This id format is not valid")]
	InvalidId
}

impl<T> From<PoisonError<T>> for RepositoryError {
	fn from(poison_error: PoisonError<T>) -> Self {
		RepositoryError::LockError(poison_error.to_string())
	}
}
