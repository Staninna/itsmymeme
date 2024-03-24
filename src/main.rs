use dotenvy::var;
use rocket::{
    fs::NamedFile, get, launch, request::FromRequest, response::content::RawHtml, routes, Request,
};
const DISCORD_USER_AGENT: [&str; 2] = ["Discordbot/2.0", "Intel Mac OS X 11.6"];

struct UserAgent<'r>(&'r str);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAgent<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let user_agent = request.headers().get_one("User-Agent").unwrap_or("unknown");
        rocket::request::Outcome::Success(UserAgent(user_agent))
    }
}

#[get("/<file..>")]
async fn serve(file: std::path::PathBuf, user_agent: UserAgent<'_>) -> Result<NamedFile, String> {
    if DISCORD_USER_AGENT
        .iter()
        .any(|ua| user_agent.0.contains(ua))
    {
        return match NamedFile::open(file).await {
            Ok(file) => Ok(file),
            Err(_) => Err(var("NOT_FOUND_MESSAGE")
                .unwrap_or("File not found".to_string())
                .to_string()),
        };
    }

    Err(var("ERROR_MESSAGE").unwrap_or("You are not allowed to access this file".to_string()))
}

#[get("/upload")]
async fn upload_page() -> RawHtml<&'static str> {
    RawHtml(include_str!("upload.html"))
}

#[launch]
fn rocket() -> _ {
    dotenvy::dotenv().ok();

    rocket::build().mount("/", routes![serve, upload_page])
}
