use actix_web::web::{get, ServiceConfig};
use crate::modules::health::services::health_check;

pub fn routes(cfg: &mut ServiceConfig) {
	cfg.route("/health", get().to(health_check));
}
