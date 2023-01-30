use std::env::var;
use twitter_v2::authorization::{BearerToken, Oauth1aToken};
use twitter_v2::TwitterApi;

// Gets api from app context (Read).
pub fn get_api_app_context() -> TwitterApi<BearerToken> {
    TwitterApi::new(BearerToken::new(
        var("BEARER_TOKEN").expect("BEARER_TOKEN not found"),
    ))
}

// Gets api from user context (Read and Write).
pub fn get_api_user_context() -> TwitterApi<Oauth1aToken> {
    TwitterApi::new(Oauth1aToken::new(
        var("API_KEY").expect("API_KEY not found"),
        var("API_SECRET").expect("API_SECRET not found"),
        var("ACCESS_TOKEN").expect("ACCESS_TOKEN not found"),
        var("ACCESS_SECRET").expect("ACCESS_SECRET not found"),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use twitter_v2::Result;

    #[tokio::test]
    async fn test_get_api_app_context() -> Result<()> {
        dotenv::dotenv().expect(".env file should be readable");
        assert!(get_api_app_context().with_user_ctx().await.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_api_user_context() -> Result<()> {
        dotenv::dotenv().expect(".env file should be readable");
        assert!(get_api_user_context().with_user_ctx().await.is_ok());
        Ok(())
    }
}
