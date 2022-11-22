#[macro_use]
extern crate rocket;

mod api;
mod utils;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = api::rocket().launch().await?;
    Ok(())
}