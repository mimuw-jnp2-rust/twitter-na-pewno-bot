use twitter_v2::authorization::Oauth1aToken;
use twitter_v2::id::NumericId;
use twitter_v2::query::TweetField::InReplyToUserId;
use twitter_v2::TwitterApi;

const MINIMUM_NUMBER_OF_RESULTS: usize = 5;

// Getting id of currently authorized user.
#[allow(dead_code)]
pub async fn get_my_user_id(api: &TwitterApi<Oauth1aToken>) -> Option<NumericId> {
    let me = api
        .get_users_me()
        .send()
        .await
        .expect("user not found")
        .into_data();

    match me {
        None => None,
        Some(me) => Option::from(me.id),
    }
}

// Getting id of the latest reply of given user.
#[allow(dead_code)]
pub async fn get_last_user_reply_id(
    user_id: NumericId,
    api: &TwitterApi<Oauth1aToken>,
) -> Option<NumericId> {
    let my_tweets = api
        .get_user_tweets(user_id)
        .tweet_fields([InReplyToUserId])
        .max_results(MINIMUM_NUMBER_OF_RESULTS)
        .send()
        .await
        .expect("user not found")
        .into_data();

    match my_tweets {
        None => None,
        Some(tweets) => {
            for tweet in tweets {
                if tweet.in_reply_to_user_id.is_some() {
                    return Option::from(tweet.id);
                }
            }
            return None;
        }
    }
}
