use crate::auth::{get_api_app_context, get_api_user_context};

use std::collections::HashSet;
use std::ops::Sub;
use time::Date;
use twitter_v2::id::NumericId;
use twitter_v2::query::Exclude::Replies;
use twitter_v2::query::TweetField::{AuthorId, CreatedAt};
use twitter_v2::Tweet;

const MINIMUM_NUMBER_OF_RESULTS: usize = 5;
const MAXIMUM_NUMBER_OF_RESULTS: usize = 100;
const MINIMUM_BREAK_AFTER_RUN: f32 = 100.0;
const MISTAKE: &str = "napewno -is:retweet";

// Gets id of currently authorized user.
pub async fn get_my_user_id() -> Option<NumericId> {
    let api = get_api_user_context();
    let me = api
        .get_users_me()
        .send()
        .await
        .expect("invalid user")
        .into_data();

    me.map(|user| user.id)
}

// Gets username of currently authorized user.
pub async fn get_my_username() -> Option<String> {
    let api = get_api_user_context();
    let me = api
        .get_users_me()
        .send()
        .await
        .expect("invalid user")
        .into_data();

    me.map(|user| user.username)
}

// Gets username by id.
pub async fn get_username_by_id(id: NumericId) -> Option<String> {
    let api = get_api_user_context();
    let user = api
        .get_user(id)
        .send()
        .await
        .expect("invalid id")
        .into_data();

    user.map(|user| user.username)
}

// Gets name by id.
pub async fn get_name_by_id(id: NumericId) -> Option<String> {
    let api = get_api_user_context();
    let user = api
        .get_user(id)
        .send()
        .await
        .expect("invalid id")
        .into_data();

    user.map(|user| user.name)
}

// Gets the id of the latest tweet, after which user stopped searching.
pub async fn get_initial_tweet(user: NumericId) -> NumericId {
    let api = get_api_app_context();
    let tweets = api
        .get_user_tweets(user)
        .tweet_fields([CreatedAt])
        .max_results(MAXIMUM_NUMBER_OF_RESULTS)
        .send()
        .await
        .expect("invalid user")
        .into_data()
        .unwrap_or_default();

    let index = (0..tweets.len() - 1)
        .find(|&i| {
            let cur_date = tweets[i].created_at.expect("invalid date");
            let next_date = tweets[i + 1].created_at.expect("invalid date");
            cur_date.sub(next_date).as_seconds_f32() > MINIMUM_BREAK_AFTER_RUN
        })
        .unwrap_or(tweets.len() - 1);

    tweets[index].id
}

// Gets the latest tweet of given user.
pub async fn get_latest_tweet(user: NumericId) -> Option<Tweet> {
    let api = get_api_app_context();
    let tweets = api
        .get_user_tweets(user)
        .tweet_fields([CreatedAt])
        .exclude([Replies])
        .max_results(MINIMUM_NUMBER_OF_RESULTS)
        .send()
        .await
        .expect("invalid user")
        .into_data()
        .unwrap_or_default();

    tweets.first().cloned()
}

// Gets tweets with mistake since given tweet.
pub async fn get_tweets_with_mistake(id: NumericId) -> Vec<Tweet> {
    let api = get_api_app_context();
    // Gets no more than last MAXIMUM_NUMBER_OF_RESULTS tweets.
    let tweets = api
        .get_tweets_search_recent(MISTAKE)
        .tweet_fields([AuthorId, CreatedAt])
        .since_id(id)
        .max_results(MAXIMUM_NUMBER_OF_RESULTS)
        .send()
        .await
        .expect("invalid query")
        .into_data()
        .unwrap_or_default();

    // Take oldest tweets first.
    tweets.into_iter().rev().collect::<Vec<_>>()
}

// Counts all unique users whose tweets included given word on a given day.
pub async fn count_tweets_with_mistake(date: &Date) -> usize {
    let api = get_api_app_context();
    let mut users = HashSet::new();
    let mut size = 1;

    let mut end_date = date
        .next_day()
        .expect("invalid date")
        .midnight()
        .assume_utc();

    while size != 0 {
        let tweets = api
            .get_tweets_search_recent(MISTAKE)
            .tweet_fields([AuthorId, CreatedAt])
            .start_time(date.midnight().assume_utc())
            .end_time(end_date)
            .max_results(MAXIMUM_NUMBER_OF_RESULTS)
            .send()
            .await
            .expect("invalid query")
            .into_data()
            .unwrap_or_default();

        size = tweets
            .iter()
            .map(|tweet| users.insert(tweet.author_id))
            .count();

        if size > 0 {
            end_date = tweets[size - 1].created_at.expect("invalid size");
        }
    }

    users.len()
}

// Posts tweet with given message.
pub async fn post_tweet_with_message(message: String) {
    let api = get_api_user_context();
    api.post_tweet()
        .text(message)
        .send()
        .await
        .expect("invalid message");
}

// Posts reply to provided tweet with given message.
pub async fn post_reply_with_message(id: NumericId, message: String) {
    let api = get_api_user_context();
    let result = api
        .post_tweet()
        .text(message)
        .in_reply_to_tweet_id(id)
        .send()
        .await;

    // There are several reasons why replying to certain tweets is just
    // impossible - ignore such cases and do not stop program.
    if result.is_ok() {
        result.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use twitter_v2::Result;

    #[tokio::test]
    async fn test_get_username_by_id() -> Result<()> {
        dotenv::dotenv().expect(".env file should be readable");
        let id = NumericId::new(1608833104919117824);
        let username = "napewnobot".to_string();
        assert_eq!(get_username_by_id(id).await.unwrap(), username);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_name_by_id() -> Result<()> {
        dotenv::dotenv().expect(".env file should be readable");
        let id = NumericId::new(1608833104919117824);
        let name = "na pewno bot 📢".to_string();
        assert_eq!(get_name_by_id(id).await.unwrap(), name);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_initial_tweet() -> Result<()> {
        dotenv::dotenv().expect(".env file should be readable");
        let user_id = NumericId::new(841324337978306562);
        let tweet_id = NumericId::new(1620564152187232256);
        assert_eq!(get_initial_tweet(user_id).await, tweet_id);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_latest_tweet() -> Result<()> {
        dotenv::dotenv().expect(".env file should be readable");
        let user_id = NumericId::new(841324337978306562);
        let tweet_id = NumericId::new(1355831924690923520);
        assert_eq!(get_latest_tweet(user_id).await.unwrap().id, tweet_id);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_tweets_with_mistake() -> Result<()> {
        dotenv::dotenv().expect(".env file should be readable");
        let tweet_id = NumericId::new(1619713517258522625);
        let tweets = get_tweets_with_mistake(tweet_id).await;
        tweets
            .iter()
            .for_each(|t| assert!(t.text.to_lowercase().contains("napewno")));
        Ok(())
    }
}
