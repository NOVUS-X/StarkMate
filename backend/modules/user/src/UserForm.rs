use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use validator::Validate;

//[derive(Debug, Serialize, Deserialize, Validate)]
struct UserName {
    //[validate(length(min = 1))]
    username: String,
}

//[derive(Debug, Serialize, Deserialize, Validate)]
struct Profile {
    flag: Option<String>,
    //validate(length(max = 80))]
    location: Option<String>,
    //[validate(length(max = 400))]
    bio: Option<String>,
    //[validate(length(min = 1, max = 100))]
    real_name: Option<String>,
    //[validate(range(min = 1400, max = 3000))]
    fide_rating: Option<i32>,
    #[validate(range(min = 100, max = 3000))]
    uscf_rating: Option<i32>,
    #[validate(range(min = 0, max = 3000))]
    ecf_rating: Option<i32>,
    #[validate(range(min = 0, max = 3000))]
    rcf_rating: Option<i32>,
    #[validate(range(min = 200, max = 3000))]
    cfc_rating: Option<i32>,
    #[validate(range(min = 0, max = 3000))]
    dsb_rating: Option<i32>,
    #[validate(length(max = 3000))]
    links: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct NoteData {
    #[validate(length(min = 3, max = 2000))]
    text: String,
    mod_flag: bool,
    dox: bool,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct Flair {
    flair: Option<String>,
}

async fn update_username(user: web::Json<UserName>) -> impl Responder {
    if let Err(e) = user.validate() {
        return HttpResponse::BadRequest().json(e);
    }
    HttpResponse::Ok().json(user.into_inner())
}

async fn update_profile(profile: web::Json<Profile>) -> impl Responder {
    if let Err(e) = profile.validate() {
        return HttpResponse::BadRequest().json(e);
    }
    HttpResponse::Ok().json(profile.into_inner())
}

async fn add_note(note: web::Json<NoteData>) -> impl Responder {
    if let Err(e) = note.validate() {
        return HttpResponse::BadRequest().json(e);
    }
    HttpResponse::Ok().json(note.into_inner())
}

async fn update_flair(flair: web::Json<Flair>) -> impl Responder {
    HttpResponse::Ok().json(flair.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .route("/username", web::post().to(update_username))
            .route("/profile", web::post().to(update_profile))
            .route("/note", web::post().to(add_note))
            .route("/flair", web::post().to(update_flair))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
