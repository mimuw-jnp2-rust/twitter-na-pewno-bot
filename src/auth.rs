use twitter_v2::authorization::{BearerToken, Oauth1aToken};
use twitter_v2::TwitterApi;

#[allow(dead_code)]
// Getting api from app context (Read).
pub fn get_api_app_ctx() -> TwitterApi<BearerToken> {
    TwitterApi::new(BearerToken::new(
        std::env::var("BEARER_TOKEN").expect("BEARER_TOKEN not found"),
    ))
}

#[allow(dead_code)]
// Getting api from user (@napewnobot) context (Read and Write).
pub fn get_api_user_ctx() -> TwitterApi<Oauth1aToken> {
    TwitterApi::new(Oauth1aToken::new(
        std::env::var("API_KEY").expect("API_KEY not found"),
        std::env::var("API_SECRET").expect("API_SECRET not found"),
        std::env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN not found"),
        std::env::var("ACCESS_SECRET").expect("ACCESS_SECRET not found"),
    ))
}
