#!/usr/bin/env rustx

use crate::random_question::QuestionListFilterInput;
use anyhow::anyhow;
use anyhow::Result as AnyhowResult;
use clap::Parser;
use graphql_client::{GraphQLQuery, Response};
use reqwest;
use serenity::http::client::Http;
use std::env;
use std::str::FromStr;

pub mod args;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/query.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct QuestionOfToday;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/query.graphql",
    response_derives = "Debug, PartialEq"
)]
pub struct RandomQuestion;

const LEETCODE_GRAPHQL_ENDPOINT: &str = "https://leetcode.com/graphql";
const LEETCODE_BASE_URL: &str = "https://leetcode.com";
const LEETCODE_PROBLEMS_PREFIX: &str = "/problems/";
const DISCORD_WEBHOOK_URL: &str = "DISCORD_WEBHOOK_URL";

async fn make_post_query<T: GraphQLQuery>(
    variables: T::Variables,
) -> AnyhowResult<Response<T::ResponseData>> {
    let request_body = T::build_query(variables);
    let client = reqwest::Client::new();
    let res = client
        .post(LEETCODE_GRAPHQL_ENDPOINT)
        .json(&request_body)
        .send()
        .await?;

    let response_body: Response<T::ResponseData> = res.json().await?;
    Ok(response_body)
}

async fn fetch_and_post_question_of_today() -> AnyhowResult<()> {
    let variables = question_of_today::Variables;
    let question_of_today = make_post_query::<QuestionOfToday>(variables)
        .await?
        .data
        .ok_or(anyhow!("Question of Today query failed"))?
        .active_daily_coding_challenge_question
        .link;
    let question_of_today_uri = format!("{}{}", LEETCODE_BASE_URL, question_of_today);
    post_to_discord(&question_of_today_uri).await;

    Ok(())
}

async fn fetch_and_post_random_question(difficulty: String) -> AnyhowResult<()> {
    let variables_random = random_question::Variables {
        category_slug: "".to_string(),
        filters: QuestionListFilterInput { difficulty },
    };

    let random_question = make_post_query::<RandomQuestion>(variables_random)
        .await?
        .data
        .ok_or(anyhow!("Random question query failed"))?
        .random_question
        .title_slug;
    let random_question_uri = format!(
        "{}{}{}",
        LEETCODE_BASE_URL, LEETCODE_PROBLEMS_PREFIX, random_question
    );
    post_to_discord(&random_question_uri).await;

    Ok(())
}

async fn post_to_discord(message: &str) {
    //Read discord webhook url
    let discord_webhook_url =
        env::var(DISCORD_WEBHOOK_URL).expect(&format!("{} is not set", DISCORD_WEBHOOK_URL));
    let token = discord_webhook_url.split("/").last().unwrap();

    let http = Http::new(token);

    //Panic if anything related to discord fails - fetching webhook from url, posting to channel
    //etc.

    let webhook = http
        .get_webhook_from_url(&discord_webhook_url)
        .await
        .expect("Getting webhook from url failed; Verify if the url is correct!");

    webhook
        .execute(&http, false, |w| w.content(message))
        .await
        .expect("Posting to discord failed");
}

#[tokio::main]
async fn main() -> AnyhowResult<()> {
    //Read and validate command line args
    let args = args::Args::parse();
    let _difficulty = args::Difficulty::from_str(&args.difficulty)?;

    let fut_question_of_today = fetch_and_post_question_of_today();
    let fut_random_question = fetch_and_post_random_question(args.difficulty);
    tokio::try_join!(fut_question_of_today, fut_random_question)?;
    Ok(())
}
