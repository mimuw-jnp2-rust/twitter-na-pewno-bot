# na pewno bot for Twitter
[![Twitter URL](https://i.imgur.com/7P6IaZw.jpg)](https://twitter.com/napewnobot)

## Authors
- Michał Skwarek ([@mskwr](https://github.com/mskwr))

## Description
Statistics show that for many years, by far the most common spelling mistake made by Poles online is the phrase "na pewno" (which means "certainly"), incorrectly spelled as "napewno". On Twitter, this mistake is made by more than 1,000 people every day. The job of the na pewno bot for Twitter is to improve these terrible statistics.

## Features
- finding all tweets containing "napewno"
- correcting spelling by automatically replying to these tweets
- maintaining statistics, including how many people per day made this mistake
- providing daily reports on the profile, including comparing results with the previous day

## Plan
In the first part, I will implement the basics - a bot that will find all "napewno" mistakes and respond to them in an identical way. In the second part I will implement extensions - differentiated bot responses, maintaining statistics, daily reports.

## Libraries
- [tokio](https://docs.rs/tokio/latest/tokio/)
- [time](https://docs.rs/time/latest/time/)
- [dotenv](https://docs.rs/dotenv/latest/dotenv/)
- [twitter_v2](https://docs.rs/twitter-v2/latest/twitter_v2/)
