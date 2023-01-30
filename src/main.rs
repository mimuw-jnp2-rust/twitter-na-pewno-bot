mod auth;
mod requests;
mod strings;

use crate::requests::{
    count_tweets_with_mistake, get_initial_tweet, get_latest_tweet, get_my_user_id,
    get_my_username, get_name_by_id, get_tweets_with_mistake, get_username_by_id,
    post_reply_with_message, post_tweet_with_message,
};
use crate::strings::{
    extract_statistics, generate_reply, generate_tweet, print_end_message, print_reply_message,
    print_start_message, print_update_message,
};

use std::thread::sleep;
use std::time::Duration;
use time::OffsetDateTime;

// Due to the limit of 100 Tweets per hour.
const REQUEST_TIMEOUT_SECS: u64 = 60;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file.
    dotenv::dotenv().expect(".env file should be readable");
    print_start_message();

    let my_id = get_my_user_id().await.expect("invalid authorization");
    let my_username = get_my_username().await.expect("invalid authorization");
    let my_latest_tweet = get_latest_tweet(my_id).await;

    // All time variables are in UTC.
    let cur_date = OffsetDateTime::now_utc().date();
    let prev_date = cur_date.previous_day().expect("invalid date");

    // Post daily updates with statistics on the profile.
    if let Some(tweet) = my_latest_tweet {
        let last_date = tweet.created_at.map(|t| t.date());

        // Do not post anything if update was already made today.
        if last_date != Some(cur_date) {
            let cur_stat = count_tweets_with_mistake(&prev_date).await;
            // Only extract previous stats if update was made on the previous day.
            let prev_stat = if last_date == cur_date.previous_day() {
                extract_statistics(tweet.text.as_str()).unwrap_or(0)
            } else {
                0
            };

            let msg = generate_tweet(prev_stat, cur_stat);
            post_tweet_with_message(msg).await;
            print_update_message(my_username);
        }
    } else {
        // No updates on the profile yet.
        let cur_stat = count_tweets_with_mistake(&prev_date).await;
        let msg = generate_tweet(0, cur_stat);
        post_tweet_with_message(msg).await;
        print_update_message(my_username);
    }

    let initial_tweet_id = get_initial_tweet(my_id).await;
    let tweets_with_mistake = get_tweets_with_mistake(initial_tweet_id).await;

    // Can not use iterator here, because of instability of async closures.
    for tweet in tweets_with_mistake {
        let id = tweet.author_id.expect("invalid user");
        let username = get_username_by_id(id).await.expect("invalid user");
        let name = get_name_by_id(id).await.expect("invalid user");
        let msg = generate_reply(name.as_str());
        post_reply_with_message(tweet.id, msg).await;
        print_reply_message(tweet.id, username);

        // Avoid shadowban and never exceed the limit of posts.
        sleep(Duration::from_secs(REQUEST_TIMEOUT_SECS));
    }

    print_end_message();
}
