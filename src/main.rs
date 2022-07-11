use crate::random_question::QuestionListFilterInput;
use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result as AnyhowResult;
use clap::Parser;
use graphql_client::{GraphQLQuery, Response};
use reqwest;
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

async fn get_question_of_today(
    variables: question_of_today::Variables,
) -> Result<Response<question_of_today::ResponseData>, Error> {
    let request_body = QuestionOfToday::build_query(variables);
    let client = reqwest::Client::new();
    let res = client
        .post(LEETCODE_GRAPHQL_ENDPOINT)
        .json(&request_body)
        .send()
        .await?;

    let response_body: Response<question_of_today::ResponseData> = res.json().await?;
    Ok(response_body)
}

async fn get_random_question(
    variables: random_question::Variables,
) -> Result<Response<random_question::ResponseData>, Error> {
    let request_body = RandomQuestion::build_query(variables);
    let client = reqwest::Client::new();
    let res = client
        .post(LEETCODE_GRAPHQL_ENDPOINT)
        .json(&request_body)
        .send()
        .await?;
    let response_body: Response<random_question::ResponseData> = res.json().await?;
    Ok(response_body)
}

#[tokio::main]
async fn main() -> AnyhowResult<(), Error> {
    //Read and validate command line args
    let args = args::Args::parse();
    let _difficulty = args::Difficulty::from_str(&args.difficulty)?;

    let variables = question_of_today::Variables;
    let question_of_today = get_question_of_today(variables)
        .await?
        .data
        .ok_or(anyhow!("Query failed"))?
        .active_daily_coding_challenge_question
        .link;
    let question_of_today_uri = format!("{}{}", LEETCODE_BASE_URL, question_of_today);
    println!("{:?}", question_of_today_uri);

    let variables_random = random_question::Variables {
        category_slug: "".to_string(),
        filters: QuestionListFilterInput {
            difficulty: args.difficulty,
        },
    };
    let random_question = get_random_question(variables_random)
        .await?
        .data
        .ok_or(anyhow!("Random question query failed"))?
        .random_question
        .title_slug;
    let random_question_uri = format!(
        "{}{}{}",
        LEETCODE_BASE_URL, LEETCODE_PROBLEMS_PREFIX, random_question
    );
    println!("{:?}", random_question_uri);
    Ok(())
}
