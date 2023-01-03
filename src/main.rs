mod auth;

use auth::get_api_user_ctx;
use twitter_v2::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let api = get_api_user_ctx();
    Ok(())
}
