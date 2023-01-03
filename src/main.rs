mod auth;

use auth::get_api_user_ctx;
use std::thread::sleep;
use std::time::Duration;
use time::OffsetDateTime;
use twitter_v2::Error;

const MINIMUM_PRIOR_SECS: u64 = 10;
const REQUEST_TIMEOUT_SECS: u64 = 300;
const SINCE_LAST_MILLIS: u64 = 1;
const KEYWORD: &str = "napewno";
const MSG: &str = "Witam. Proszę wybaczyć moją śmiałość, ale 'na pewno' piszemy rozdzielnie.";

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Authorizing with Twitter API...");

    // Getting Twitter Api from user context.
    let api = get_api_user_ctx();

    println!("Authorized. The bot has been launched.");

    // All time variables are in UTC.
    let mut cur_time = OffsetDateTime::now_utc();
    let mut start_time = cur_time - Duration::from_secs(MINIMUM_PRIOR_SECS);

    loop {
        let tweets = api
            .get_tweets_search_recent(KEYWORD)
            .start_time(start_time)
            .send()
            .await?
            .into_data();

        if tweets != None {
            for tweet in tweets.unwrap() {
                api.post_tweet()
                    .text(MSG.parse().unwrap())
                    .in_reply_to_tweet_id(tweet.id)
                    .send()
                    .await?;

                println!("I replied to the tweet: {}", tweet.text);
            }
        }

        // Due to the limit of requests per month, the tweets are requested
        // every certain period of time.
        sleep(Duration::from_secs(REQUEST_TIMEOUT_SECS));

        start_time = cur_time + Duration::from_millis(SINCE_LAST_MILLIS);
        cur_time = OffsetDateTime::now_utc();
    }
}
