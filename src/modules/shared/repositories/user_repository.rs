use actix_web::HttpResponse;
use actix_web::web::{Data, Json, Path};
use uuid::Uuid;

use crate::modules::shared::models::entities::user::User;
use crate::modules::shared::repositories::repository::Repository;

//monomorphism
pub async fn get<R: Repository>(user_id: Path<Uuid>, repo: Data<R>) -> HttpResponse {
	match repo.get_user(&user_id).await {
		Ok(user) => HttpResponse::Ok().json(user),
		Err(_) => HttpResponse::NotFound().body("Not Found")
	}
}

pub async fn post<R: Repository>(user: Json<User>, repo: Data<R>) -> HttpResponse {
	match repo.create_user(&user).await {
		Ok(user) => HttpResponse::Created().json(user),
		Err(e) => HttpResponse::InternalServerError().body(format!("Something went wrong: {}", e))
	}
}

pub async fn put<R: Repository>(user: Json<User>, repo: Data<R>) -> HttpResponse {
	match repo.update_user(&user).await {
		Ok(user) => HttpResponse::Ok().json(user),
		Err(_) => HttpResponse::NotFound().body("Not Found")
	}
}

pub async fn delete<R: Repository>(user_id: Path<Uuid>, repo: Data<R>) -> HttpResponse {
	match repo.delete_user(&user_id).await {
		Ok(user) => HttpResponse::Ok().body(user.to_string()),
		Err(e) => HttpResponse::InternalServerError().body(format!("Something went wrong: {}", e))
	}
}
