use actix_web::HttpResponse;
use actix_web::web::Data;
use tracing::instrument;

#[instrument]
pub async fn health_check(index: Data<u16>) -> HttpResponse {
	HttpResponse::Ok()
		.append_header(("thread-id", index.to_string()))
		.finish()
}
