use actix_web::web::{scope, ServiceConfig};
use crate::modules::shared::repositories::repository::Repository;

mod routes;

pub fn routes<R: Repository>(cfg: &mut ServiceConfig){
	cfg.service(scope("v1").configure(routes::routes::<R>));
}
