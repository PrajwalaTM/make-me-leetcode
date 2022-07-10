use graphql_client::{GraphQLQuery, Response};
use reqwest;
use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result as AnyhowResult;

#[derive(GraphQLQuery)]
#[graphql(schema_path = "graphql/schema.graphql",
    query_path = "graphql/query.graphql",
    response_derives = "Debug",)]
pub struct QuestionOfToday;

const LEETCODE_GRAPHQL_ENDPOINT:&str= "https://leetcode.com/graphql";
const LEETCODE_BASE_URL:&str = "https://leetcode.com";

async fn perform_query(variables: question_of_today::Variables,) -> Result<Response<question_of_today::ResponseData>, Error> {
    let request_body = QuestionOfToday::build_query(variables);
    let client = reqwest::Client::new();
    let res = client.post(LEETCODE_GRAPHQL_ENDPOINT).json(&request_body).send().await?;
    
    let response_body: Response<question_of_today::ResponseData> = res.json().await?;
    Ok(response_body)
}

#[tokio::main]
async fn main() -> AnyhowResult<(), Error> {
    let variables = question_of_today::Variables;

    let question_of_today = perform_query(variables).await?.data.ok_or(anyhow!("Query failed"))?.active_daily_coding_challenge_question.link;
    let fq_question_of_today = format!("{}{}",LEETCODE_BASE_URL, question_of_today);
    println!("{:?}", fq_question_of_today);
    Ok(())
}
