mod auth;
mod requests;
mod strings;

use crate::requests::{
    count_tweets_with_mistake, get_latest_reply_id, get_latest_tweet, get_my_user_id,
    get_tweets_with_mistake, post_tweet_with_message,
};
use crate::strings::{extract_statistics, generate_tweet};
use time::OffsetDateTime;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file.
    dotenv::dotenv().expect(".env file should be readable");

    let my_id = get_my_user_id().await.expect("invalid authorization");
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
        }
    } else {
        // No updates on the profile yet.
        let cur_stat = count_tweets_with_mistake(&prev_date).await;
        let msg = generate_tweet(0, cur_stat);
        post_tweet_with_message(msg).await;
    }

    let my_latest_reply = get_latest_reply_id(my_id).await;
    let tweets_with_mistake = get_tweets_with_mistake(my_latest_reply).await;

    for tweet in tweets_with_mistake {
        println!("[{}] {}", tweet.created_at.unwrap(), tweet.text);
    }
}
