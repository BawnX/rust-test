use actix_web::{Error, HttpRequest};
use actix_web::error::PathError;
use actix_web::web::{delete, get, PathConfig, post, put, resource, scope, ServiceConfig};

use crate::modules::shared::repositories::{repository::Repository, user_repository};

pub fn routes<R: Repository>(cfg: &mut ServiceConfig) {
	cfg.service(
		scope("user")
			.route("/{user_id}", delete().to(user_repository::delete::<R>))
			.service(
				resource("/{user_id}")
					.app_data(PathConfig::default().error_handler(path_config_handler))
					.route(get().to(user_repository::get::<R>))
			)
			.route("/", post().to(user_repository::post::<R>))
			.route("/", put().to(user_repository::put::<R>))
	);
}

fn path_config_handler(_err: PathError, _req: &HttpRequest) -> Error {
	actix_web::error::ErrorBadRequest("Error")
}
