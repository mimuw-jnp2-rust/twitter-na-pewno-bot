# na pewno bot for Twitter

## Authors
- Michał Skwarek (@mskwr on GitHub)

## Description
Annual surveys show that the most popular orthographic error among Poles online for 2020, 2021 and 2022 is the phrase "na pewno" (which means "certainly"), incorrectly spelled as "napewno". On Twitter, this mistake is made by more than 1,000 people every day. The job of the na pewno bot for Twitter is to improve these stats.

## Features
- finding all tweets containing "napewno"
- correcting orthography by automatically replying to these tweets
- maintaining statistics, including how many people per day made the mistake
- providing daily reports on the profile, comparing the results with the previous day

## Plan
In the first part I am going to implement the basics: a bot that will find "napewno" errors and respond to them in the identical, same way. In the second part I am going to add varied answers, keeping statistics, daily posting of reports.

## Libraries
- egg-mode
