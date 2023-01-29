# na pewno bot for Twitter ([@napewnobot](https://twitter.com/napewnobot))
[![Twitter URL](https://i.imgur.com/DPXcjpZ.png)](https://twitter.com/napewnobot)

## Authors
- Michał Skwarek ([@mskwr](https://github.com/mskwr))

## Description
Statistics show that for many years, by far the most common spelling mistake made by Poles online is the phrase "na pewno" (which means "certainly"), incorrectly spelled as "napewno". On Twitter, this mistake is made by up to 1000 unique users every day. The job of the na pewno bot for Twitter is to improve these terrible statistics.

The bot provides the following features:
* finding all tweets containing "napewno"
* correcting spelling by automatically replying to these tweets
* maintaining statistics, including how many people per day made this mistake
* providing daily reports on the profile, including comparing results with the previous day

## Getting started
1. Sign up for the Twitter API to get the keys and tokens necessary for authorization. Set them as an environment variables.
2. Clone the repository.
   ```sh
   git clone https://github.com/mimuw-jnp2-rust/twitter-na-pewno-bot.git
   cd twitter-na-pewno-bot
   ```
3. Build the project.
   ```sh
   cargo build --release
   ```
4. Run the project.
   ```sh
   cargo run
   ```

## License
Distributed under the MIT License. See `LICENSE.txt` for more information.

## Libraries
- [twitter_v2](https://docs.rs/twitter-v2/latest/twitter_v2/)
- [tokio](https://docs.rs/tokio/latest/tokio/)
- [rand](https://docs.rs/rand/latest/rand/)
- [time](https://docs.rs/time/latest/time/)
- [dotenv](https://docs.rs/dotenv/latest/dotenv/)
