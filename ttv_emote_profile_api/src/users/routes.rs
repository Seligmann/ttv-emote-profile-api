#[get("/users")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let users = web::block(|| users::find_all()).await.unwrap();
    Ok(HttpResponse::Ok().json(users))
}

#[get("/users/{id}")]
async fn find(id: web::Path) -> Result<HttpResponse, CustomError> {
    let user = users::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/users")]
async fn create(user: web::Json) -> Result<HttpResponse, CustomError> {
    let user = users::create(user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[put("/users/{id}")]
async fn update(
    id: web::Path,
    user: web::Json,
) -> Result<HttpResponse, CustomError> {
    let user = users::update(id.into_inner(), user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/users/{id}")]
async fn delete(id: web::Path) -> Result<HttpResponse, CustomError> {
    let deleted_user = users::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_user })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}