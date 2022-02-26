extern crate core;

use std::sync::Arc;
use std::sync::atomic::{AtomicU16, Ordering};

use actix_web::{App, HttpServer, web};
use actix_web::web::Data;
use tracing::{self as log};
use tracing_subscriber::EnvFilter;

use crate::modules::shared::repositories::postgres_repository::PostgresRepository;

mod modules;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv::dotenv().ok();

	let tracing = tracing_subscriber::fmt()
		.with_span_events(
			tracing_subscriber::fmt::format::FmtSpan::ENTER |
				tracing_subscriber::fmt::format::FmtSpan::CLOSE
		)
		.with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339())
		.with_env_filter(EnvFilter::from_default_env());


	if cfg!(debug_assertions){
	 tracing
		 .pretty()
		 .init();
	}else{
		tracing
			.json()
			.init();
	}

	let port = std::env::var("PORT").unwrap_or("8080".to_string());
	let address = format!("127.0.0.1:{}", port);

	log::debug!("Starting our Server at: {}", address);
	let thread_counter = Arc::new(AtomicU16::new(1));

	// let repo = web::Data::new(MemoryRepository::default());
	let repo = PostgresRepository::from_env().await.expect("Repository initialization error");
	let repo = web::Data::new(repo);

	HttpServer::new(move || {
		let thread_index = thread_counter.fetch_add(1, Ordering::SeqCst);
		log::trace!("Starting Threads {}", thread_index);

		App::new()
			.app_data(Data::new(thread_index))
			.app_data(repo.clone())
			.configure(modules::health::routes::routes)
			.configure(modules::user::v1::routes::<PostgresRepository>)
	})
		.bind(&address)
		.unwrap_or_else(|err| panic!("🔥🔥🔥 Couldn't start server: {}: {:?}", port, err))
		.run()
		.await
}
