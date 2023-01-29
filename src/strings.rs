use std::string::ToString;
use time::OffsetDateTime;
use rand::Rng;

const GREETINGS: [&str; 5] = ["Cześć", "Czołem", "Hej", "Serwus", "Witaj"];
const GREETING_EMOJIS: [&str; 8] = ["👋", "🤝", "☺️", "🥰", "🤓", "🧐", "🤖", "👀"];
const APOLOGIES: [&str; 3] = ["Daruj", "Wybacz", "Przepraszam za"];
const BRAVERY_WORDS: [&str; 3] = ["brawurę", "śmiałość", "zuchwałość"];
const PHRASE_WORDS: [&str; 2] = ["frazę", "wyrażenie"];
const SPELLING_WORDS: [&str; 2] = ["piszemy", "pisze się"];
const DIVISION_WORDS: [&str; 3] = ["osobno", "rozdzielnie", "rozłącznie"];

// Generates reply for tweet containing 'napewno'.
#[allow(dead_code)]
pub fn generate_reply(username: &str) -> String {
    let mut rng = rand::thread_rng();
    return GREETINGS[rng.gen_range(0..GREETINGS.len())].to_string()
        + " "
        + &username.to_string()
        + "! "
        + &GREETING_EMOJIS[rng.gen_range(0..GREETING_EMOJIS.len())].to_string()
        + "\n"
        + &APOLOGIES[rng.gen_range(0..APOLOGIES.len())].to_string()
        + " moją "
        + &BRAVERY_WORDS[rng.gen_range(0..BRAVERY_WORDS.len())].to_string()
        + ", ale "
        + &PHRASE_WORDS[rng.gen_range(0..PHRASE_WORDS.len())].to_string()
        + " 'na pewno' "
        + &SPELLING_WORDS[rng.gen_range(0..SPELLING_WORDS.len())].to_string()
        + " "
        + &DIVISION_WORDS[rng.gen_range(0..DIVISION_WORDS.len())].to_string()
        + ".";
}

// Generates tweet with daily statistics.
#[allow(dead_code)]
pub fn generate_tweet(prev_stat: usize, cur_stat: usize) -> String {
    let diff = prev_stat.abs_diff(cur_stat);
    let comparison = if prev_stat == cur_stat {
        "To wynik taki sam jak poprzedniego dnia. 📊".to_string()
    } else {
        "To wynik o ".to_string()
            + &diff.to_string()
            + if prev_stat < cur_stat { " większy (+" } else { " mniejszy (-" }
            + &(diff as f32 / prev_stat as f32 * 100.0).round().to_string()
            + "%) względem poprzedniego dnia. "
            + if prev_stat < cur_stat { "📈" } else { "📉" }
    };

    let today = OffsetDateTime::now_utc().date();
    return "W dniu ".to_owned()
        + &today.previous_day().expect("invalid date").to_string()
        + " fraza 'na pewno' została błędnie napisana przez aż "
        + &cur_stat.to_string()
        + " użytkowników Twittera.\n\n"
        + &comparison
}