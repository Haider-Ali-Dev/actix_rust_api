use actix_web::{Result,post, web, Responder};
use crate::http_models::models::{Register, RegisterReponse, SignIn, SignInReponse};

// ---- Register Route ----

#[post("/register")]
pub async fn register(req: web::Json<Register>) -> Result<impl Responder> {
    let reponse = RegisterReponse {
        username: req.username.clone(),
        email: req.email.clone(),
        age: req.age
    };
    // Do Database stuff here
    Ok(web::Json(reponse))
}


// ---- Sign In Route ----
#[post("/test")]
pub async fn sign_in(req: web::Json<SignIn>) -> Result<impl Responder> {
    // You will get an Error here because SignInReponse returns field which needs to be "SELECTED" retrived from the database
    // thats a task for you :)
    // For now I'll put dummy fields :)

    let reponse = SignInReponse {
        username: "Programmer".to_string(),
        email: req.email.clone(),
        age: 20
    };
    // Do Database stuff here
    Ok(web::Json(reponse))
}
