use actix_web::App;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use crate::modules::health::routes::routes;
use crate::modules::health::services::health_check;

#[actix_rt::test]
async fn health_check_works() {
	let res = health_check(Data::new(5)).await;
	
	assert!(res.status().is_success());
	assert_eq!(res.status(), StatusCode::OK);
	
	let data = res
		.headers()
		.get("thread-id")
		.map(|h| h.to_str().ok())
		.flatten();
	
	assert_eq!(data, Some("5"));
}

#[actix_rt::test]
async fn health_check_integration_works() {
	let app = App::new()
		.app_data(Data::new(5u16))
		.configure(routes);
	let mut app = actix_web::test::init_service(app).await;
	let req = actix_web::test::TestRequest::get().uri("/health").to_request();
	let res = actix_web::test::call_service(&mut app, req).await;
	
	assert!(res.status().is_success());
	assert_eq!(res.status(), StatusCode::OK);
	
	let data = res
		.headers()
		.get("thread-id")
		.map(|h| h.to_str().ok())
		.flatten();
	
	assert_eq!(data, Some("5"));
}

