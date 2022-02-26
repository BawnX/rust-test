use std::borrow::Borrow;
use actix_web::body::MessageBody;
use actix_web::web::{Data, Json, Path};
use chrono::{NaiveDate, Utc};

use crate::modules::shared::models::entities::user::{CustomData, User};
use crate::modules::shared::repositories::repository::{MockRepository};
use crate::modules::shared::repositories::user_repository::{delete, get, post, put};

pub fn create_test_user(id: uuid::Uuid, name: String, birth_date_ymd: (i32, u32, u32)) -> User {
	let (y, m, d) = birth_date_ymd;
	User {
		// id: Uuid::new_v4(),
		id,
		name,
		birth_date: NaiveDate::from_ymd(y, m, d),
		custom_data: CustomData { random: 1 },
		created_at: Some(Utc::now()),
		updated_at: None,
	}
}

#[actix_rt::test]
async fn get_by_id_works() {
	let user_id = uuid::Uuid::new_v4();
	let user_name = "Nombre";
	
	let mut repo = MockRepository::default();
	
	repo.expect_get_user().returning(move |id| {
		let user = create_test_user(*id, user_name.to_string(), (1996, 06, 06));
		Ok(user)
	});
	
	let result = get(Path::from(user_id), Data::new(repo)).await;
	// let user_bytes =;
	let user = match result.body() {
		actix_web::body::to_bytes(x) => serde_json::from_slice::<'_, User>(x.).ok(),
		_ => None
	};
	
	// {
	// 	 => serde_json::from_slice::<'_, User>(x).ok(),
	// 	_ => None,
	
	// }
	//
	// 	.map(|b| match b {
	//
	// 	})
	// 	.flatten()
	// 	.unwrap();
	//
	// assert_eq!(user.id, user_id);
	// assert_eq!(user.name, user_name);
}
//
// #[actix_rt::test]
// async fn create_works() {
// 	let user_id = uuid::Uuid::new_v4();
// 	let user_name = "Nombre";
// 	let new_user = create_test_user(user_id, user_name.to_string(), (1996, 06, 06));
//
// 	let mut repo = MockRepository::default();
//
// 	repo.expect_create_user().returning(move |user| { Ok(user.to_owned()) });
//
// 	let mut result = post(Json(new_user), Data::new(repo)).await;
// 	let user = result.take_body()
// 		.as_ref()
// 		.map(|b| match b {
// 			actix_web::dev::Body::Bytes(x) => serde_json::from_slice::<'_, User>(x).ok(),
// 			_ => None,
// 		})
// 		.flatten()
// 		.unwrap();
//
// 	assert_eq!(user.id, user_id);
// 	assert_eq!(user.name, user_name);
// }
//
//
// #[actix_rt::test]
// async fn update_works() {
// 	let user_id = uuid::Uuid::new_v4();
// 	let user_name = "Nombre";
// 	let new_user = create_test_user(user_id, user_name.to_string(), (1996, 06, 06));
//
// 	let mut repo = MockRepository::default();
//
// 	repo.expect_update_user().returning(move |user| { Ok(user.to_owned()) });
//
// 	let mut result = put(Json(new_user), Data::new(repo)).await;
// 	let user = result.take_body()
// 		.as_ref()
// 		.map(|b| match b {
// 			actix_web::dev::Body::Bytes(x) => serde_json::from_slice::<'_, User>(x).ok(),
// 			_ => None,
// 		})
// 		.flatten()
// 		.unwrap();
//
// 	assert_eq!(user.id, user_id);
// 	assert_eq!(user.name, user_name);
// }
//
// #[actix_rt::test]
// async fn delete_works() {
// 	let user_id = uuid::Uuid::new_v4();
//
// 	let mut repo = MockRepository::default();
//
// 	repo.expect_delete_user().returning(move |id| Ok(*id));
//
// 	let mut result = delete(Path::from(user_id), Data::new(repo)).await;
//
// 	let result = result.take_body();
// 	let id = result
// 		.as_ref()
// 		.map(|b| match b {
// 			actix_web::dev::Body::Bytes(x) => std::str::from_utf8(x).ok(),
// 			_ => None,
// 		})
// 		.flatten()
// 		.unwrap();
//
// 	assert_eq!(id, user_id.to_string());
// }
