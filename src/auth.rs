use std::env::var;
use twitter_v2::authorization::{BearerToken, Oauth1aToken};
use twitter_v2::TwitterApi;

// Gets api from app context (Read).
#[allow(dead_code)]
pub fn get_api_app_context() -> TwitterApi<BearerToken> {
    TwitterApi::new(BearerToken::new(
        var("BEARER_TOKEN").expect("BEARER_TOKEN not found"),
    ))
}

// Gets api from user context (Read and Write).
#[allow(dead_code)]
pub fn get_api_user_context() -> TwitterApi<Oauth1aToken> {
    TwitterApi::new(Oauth1aToken::new(
        var("API_KEY").expect("API_KEY not found"),
        var("API_SECRET").expect("API_SECRET not found"),
        var("ACCESS_TOKEN").expect("ACCESS_TOKEN not found"),
        var("ACCESS_SECRET").expect("ACCESS_SECRET not found"),
    ))
}
