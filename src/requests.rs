use crate::auth::{get_api_app_context, get_api_user_context};
use std::collections::HashSet;
use time::{Date, OffsetDateTime};
use twitter_v2::id::NumericId;
use twitter_v2::query::Exclude::Replies;
use twitter_v2::query::TweetField::{AuthorId, CreatedAt, InReplyToUserId};

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

    me.map(|me| me.id)
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
        .into_data();

    my_tweets?
        .iter()
        .find(|tweet| tweet.in_reply_to_user_id.is_some())
        .map(|tweet| tweet.id)
}

// Gets date of the latest tweet of given user.
#[allow(dead_code)]
pub async fn get_latest_tweet_date(user: NumericId) -> Option<OffsetDateTime> {
    let api = get_api_app_context();
    let my_tweets = api
        .get_user_tweets(user)
        .tweet_fields([CreatedAt])
        .exclude([Replies])
        .max_results(MINIMUM_NUMBER_OF_RESULTS)
        .send()
        .await
        .expect("invalid user")
        .into_data();

    match my_tweets {
        None => None,
        Some(tweets) => tweets[0].created_at,
    }
}

// Counts all unique users whose tweets included given keyword on a given day.
#[allow(dead_code)]
pub async fn count_tweets_with_keyword(keyword: &str, date: &Date) -> usize {
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
            .into_data();

        match tweets.as_ref() {
            None => {
                size = 0;
            }
            Some(tweets) => {
                for tweet in tweets {
                    users.insert(tweet.author_id);
                }

                size = tweets.len();
                end_date = tweets[size - 1].created_at.expect("invalid size");
            }
        }
    }

    users.len()
}
