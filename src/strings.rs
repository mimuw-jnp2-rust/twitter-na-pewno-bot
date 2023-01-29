use rand::Rng;
use std::string::ToString;
use time::OffsetDateTime;
use twitter_v2::id::NumericId;

const GREETINGS: [&str; 5] = ["Cześć", "Czołem", "Hej", "Serwus", "Witaj"];
const GREETING_EMOJIS: [&str; 8] = ["👋", "🤝", "☺️", "🤓", "🫡", "🍻", "🤖", "👀"];
const APOLOGIES: [&str; 3] = ["Daruj mi", "Wybacz mi", "Przepraszam za"];
const BRAVERY_WORDS: [&str; 3] = ["brawurę", "śmiałość", "zuchwałość"];
const PHRASE_WORDS: [&str; 2] = ["frazę", "wyrażenie"];
const SPELLING_WORDS: [&str; 2] = ["piszemy", "pisze się"];
const DIVISION_WORDS: [&str; 3] = ["osobno", "rozdzielnie", "rozłącznie"];

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
    let size = text
        .split_whitespace()
        .map(|s| s.parse::<usize>())
        .find(|s| s.is_ok())
        .expect("invalid string");

    size.ok()
}

// Prints start message.
pub fn print_start_message() {
    let msg = "[".to_string() + &OffsetDateTime::now_utc().to_string() + "] Starting a job...";

    println!("\x1b[1m\x1b[32m{}\x1b[0m", msg);
}

// Prints message after updating statistics on the profile.
pub fn print_update_message(username: String) {
    let msg = "[".to_string()
        + &OffsetDateTime::now_utc().to_string()
        + "] Posted a profile update with the latest statistics - "
        + "https://twitter.com/"
        + &username
        + ".";

    println!("\x1b[1m{}\x1b[0m", msg);
}

// Prints message after replying to certain user.
pub fn print_reply_message(post_id: NumericId, username: String) {
    let msg = "[".to_string()
        + &OffsetDateTime::now_utc().to_string()
        + "] Posted a reply to the following Tweet: "
        + "https://twitter.com/"
        + &username
        + "/status/"
        + &post_id.to_string()
        + ".";

    println!("{}", msg);
}

// Prints end message.
pub fn print_end_message() {
    let msg = "[".to_string() + &OffsetDateTime::now_utc().to_string() + "] The job is over.";

    println!("\x1b[1m\x1b[31m{}\x1b[0m", msg);
}
