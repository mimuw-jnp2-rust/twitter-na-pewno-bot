use crate::auth::{get_api_app_context, get_api_user_context};
use std::collections::HashSet;
use time::Date;
use twitter_v2::id::NumericId;
use twitter_v2::query::Exclude::Replies;
use twitter_v2::query::TweetField::{AuthorId, CreatedAt, InReplyToUserId};
use twitter_v2::Tweet;

const MINIMUM_NUMBER_OF_RESULTS: usize = 5;
const MAXIMUM_NUMBER_OF_RESULTS: usize = 100;

// Gets id of currently authorized user.
#[allow(dead_code)]
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

// Gets id of the latest reply of given user.
#[allow(dead_code)]
pub async fn get_latest_reply_id(user: NumericId) -> Option<NumericId> {
    let api = get_api_app_context();
    let my_tweets = api
        .get_user_tweets(user)
        .tweet_fields([InReplyToUserId])
        .max_results(MINIMUM_NUMBER_OF_RESULTS)
        .send()
        .await
        .expect("invalid user")
        .into_data()
        .unwrap_or_default();

    my_tweets
        .iter()
        .find(|tweet| tweet.in_reply_to_user_id.is_some())
        .map(|tweet| tweet.id)
}

// Gets date of the latest tweet of given user.
#[allow(dead_code)]
pub async fn get_latest_tweet(user: NumericId) -> Option<Tweet> {
    let api = get_api_app_context();
    let my_tweets = api
        .get_user_tweets(user)
        .tweet_fields([CreatedAt])
        .exclude([Replies])
        .max_results(MINIMUM_NUMBER_OF_RESULTS)
        .send()
        .await
        .expect("invalid user")
        .into_data()
        .unwrap_or_default();

    if my_tweets.is_empty() {
        None
    } else {
        Some(my_tweets[0].clone())
    }
}

// Counts all unique users whose tweets included given word on a given day.
#[allow(dead_code)]
pub async fn count_tweets_with_word(keyword: &str, date: &Date) -> usize {
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
            .get_tweets_search_recent(keyword)
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
#[allow(dead_code)]
pub async fn post_tweet_with_message(message: String) {
    let api = get_api_user_context();
    api.post_tweet()
        .text(message)
        .send()
        .await
        .expect("invalid message");
}
