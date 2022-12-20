use std::sync::Mutex;
use serde::Serialize;
use actix_web::{
	get,
	post,
	web,
	App,
	HttpRequest,
	HttpResponse,
	HttpServer,
	Responder,
	body::BoxBody,
	http::header::ContentType,
};

struct AppState {
	counter: Mutex<i32>,
}

struct AppState2 {
	counter: Mutex<i32>,
}

#[derive(Serialize)]
struct JsonResponse {
	user_id: u32,
	friend_id: u32,
}

impl Responder for JsonResponse {
	type Body = BoxBody;

	fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
		let body = serde_json::to_string(&self).unwrap();
		HttpResponse::Ok()
			.content_type(ContentType::json())
			.body(body)
	}
}

#[get("/user/{user_id}/{friend_id}")]
async fn get_user_friend(path: web::Path<(u32, u32)>) -> impl Responder {
	let (user_id, friend_id) = path.into_inner();
	JsonResponse {
		user_id: user_id,
		friend_id: friend_id,
	}
}

#[get("/request-counter")]
async fn counter(data: web::Data<AppState>) -> String {
	let mut c = data.counter.lock().unwrap(); // <- get counter's MutexGuard
	*c += 1; // <- access counter inside MutexGuard
	format!("Request number: {c}") // <- response with count
}

#[get("/request-counter-continue")]
async fn counter_continue(data: web::Data<AppState>) -> String {
	let mut c = data.counter.lock().unwrap(); // <- get counter's MutexGuard
	*c += 1; // <- access counter inside MutexGuard
	format!("Request number: {c}") // <- response with count
}

#[get("/request-counter-2")]
async fn counter2(data: web::Data<AppState2>) -> String {
	let mut c = data.counter.lock().unwrap(); // <- get counter's MutexGuard
	*c += 1; // <- access counter inside MutexGuard
	format!("Request number: {c}") // <- response with count
}

#[get("/")]
async fn hello_get() -> impl Responder {
	HttpResponse::Ok().body("Get Hello world!")
}

#[post("/")]
async fn hello_post() -> impl Responder {
	HttpResponse::Ok().body("Post Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
	HttpResponse::Ok().body(req_body)
}

#[get("/hey")]
async fn manual_hello() -> impl Responder {
	HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let app_state = web::Data::new(AppState {
		counter: Mutex::new(0),
	});

	let app_state_2 = web::Data::new(AppState2 {
		counter: Mutex::new(0),
	});

	HttpServer::new(move || {
		App::new()
			.app_data(app_state.clone())
			.app_data(app_state_2.clone())
			.service(hello_get)
			.service(hello_post)
			.service(echo)
			.service(counter)
			.service(counter_continue)
			.service(counter2)
			.service(manual_hello)
			.service(get_user_friend)
	})
	.bind(("0.0.0.0", 8080))?
	.run()
	.await
}
