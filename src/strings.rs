use rand::Rng;
use std::string::ToString;
use time::{format_description, OffsetDateTime};
use twitter_v2::id::NumericId;

const GREETINGS: [&str; 5] = ["Cześć", "Czołem", "Hej", "Serwus", "Witaj"];
const GREETING_EMOJIS: [&str; 8] = ["👋", "🤝", "☺️", "🤓", "🫡", "🍻", "🤖", "👀"];
const APOLOGIES: [&str; 3] = ["Daruj mi", "Wybacz mi", "Przepraszam za"];
const BRAVERY_WORDS: [&str; 3] = ["brawurę", "śmiałość", "zuchwałość"];
const PHRASE_WORDS: [&str; 2] = ["frazę", "wyrażenie"];
const SPELLING_WORDS: [&str; 2] = ["piszemy", "pisze się"];
const DIVISION_WORDS: [&str; 3] = ["osobno", "rozdzielnie", "rozłącznie"];

const DATE_FORMAT: &str = "[hour]:[minute]:[second]";

// Generates reply for tweet containing 'napewno'.
pub fn generate_reply(username: &str) -> String {
    let mut rng = rand::thread_rng();
    GREETINGS[rng.gen_range(0..GREETINGS.len())].to_string()
        + " "
        + username
        + "! "
        + GREETING_EMOJIS[rng.gen_range(0..GREETING_EMOJIS.len())]
        + "\n"
        + APOLOGIES[rng.gen_range(0..APOLOGIES.len())]
        + " moją "
        + BRAVERY_WORDS[rng.gen_range(0..BRAVERY_WORDS.len())]
        + ", ale "
        + PHRASE_WORDS[rng.gen_range(0..PHRASE_WORDS.len())]
        + " 'na pewno' "
        + SPELLING_WORDS[rng.gen_range(0..SPELLING_WORDS.len())]
        + " "
        + DIVISION_WORDS[rng.gen_range(0..DIVISION_WORDS.len())]
        + "."
}

// Generates tweet with daily statistics.
pub fn generate_tweet(prev_stat: usize, cur_stat: usize) -> String {
    let diff = prev_stat.abs_diff(cur_stat);
    let comparison = if prev_stat == 0 {
        // No statistics on the profile yet.
        "".to_string()
    } else if prev_stat == cur_stat {
        "\n\nTo wynik taki sam jak poprzedniego dnia. 📊".to_string()
    } else {
        "\n\nTo wynik o ".to_string()
            + &diff.to_string()
            + if prev_stat < cur_stat {
                " większy "
            } else {
                " mniejszy "
            }
            + "względem poprzedniego dnia "
            + if prev_stat < cur_stat { "(+" } else { "(-" }
            + &(diff as f32 / prev_stat as f32 * 100.0).round().to_string()
            + if prev_stat < cur_stat {
                "%). 📈"
            } else {
                "%). 📉"
            }
    };

    let today = OffsetDateTime::now_utc().date();
    "W dniu ".to_owned()
        + &today.previous_day().expect("invalid date").to_string()
        + " wyrażenie 'na pewno' zostało błędnie napisane przez "
        + &cur_stat.to_string()
        + " użytkowników Twittera."
        + &comparison
}

// Extracts statistics (first integer) from text.
pub fn extract_statistics(text: &str) -> Option<usize> {
    let stats = text
        .split_whitespace()
        .map(|s| s.parse::<usize>())
        .find(|s| s.is_ok())
        .expect("invalid string");

    stats.ok()
}

// Prints start message.
pub fn print_start_message() {
    let format = format_description::parse(DATE_FORMAT).expect("invalid date");
    let msg = "[".to_string()
        + &OffsetDateTime::now_utc()
            .format(&format)
            .expect("invalid date")
        + "] Running the bot...";

    println!("\x1b[1m\x1b[32m{}\x1b[0m", msg);
}

// Prints message after updating statistics on the profile.
pub fn print_update_message(username: String) {
    let format = format_description::parse(DATE_FORMAT).expect("invalid date");
    let msg = "[".to_string()
        + &OffsetDateTime::now_utc()
            .format(&format)
            .expect("invalid date")
        + "] Posted a profile update: "
        + "https://twitter.com/"
        + &username
        + ".";

    println!("\x1b[1m{}\x1b[0m", msg);
}

// Prints message after replying to certain user.
pub fn print_reply_message(post_id: NumericId, username: String) {
    let format = format_description::parse(DATE_FORMAT).expect("invalid date");
    let msg = "[".to_string()
        + &OffsetDateTime::now_utc()
            .format(&format)
            .expect("invalid date")
        + "] Posted a reply: "
        + "https://twitter.com/"
        + &username
        + "/status/"
        + &post_id.to_string()
        + ".";

    println!("{}", msg);
}

// Prints end message.
pub fn print_end_message() {
    let format = format_description::parse(DATE_FORMAT).expect("invalid date");
    let msg = "[".to_string()
        + &OffsetDateTime::now_utc()
            .format(&format)
            .expect("invalid date")
        + "] Finished.";

    println!("\x1b[1m\x1b[32m{}\x1b[0m", msg);
}

#[cfg(test)]
mod tests {
    use super::*;
    use twitter_v2::Result;

    const LONGEST_USERNAME: usize = 15;
    const CHARACTERS_LIMIT: usize = 280;
    const MAX_ERRORS_DAILY: usize = 100000;

    #[tokio::test]
    async fn test_generate_reply() -> Result<()> {
        let limit = CHARACTERS_LIMIT - LONGEST_USERNAME;
        assert!(generate_reply("").len().le(&limit));
        Ok(())
    }

    #[tokio::test]
    async fn test_generate_tweet() -> Result<()> {
        let longest = MAX_ERRORS_DAILY;
        let limit = CHARACTERS_LIMIT;
        assert!(generate_tweet(longest, longest).len().le(&limit));
        Ok(())
    }

    #[tokio::test]
    async fn test_extract_statistics() -> Result<()> {
        let string = "   \n\n\0\0 01a a1 1111-11-11 111";
        assert_eq!(extract_statistics(string).unwrap(), 111);
        Ok(())
    }
}
