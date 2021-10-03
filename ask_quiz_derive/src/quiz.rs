use darling::ToTokens;
use quote::quote;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Quiz {
    pub questions: Vec<Question>,
}

#[derive(Deserialize, Debug)]
pub struct Question {
    pub q_num: u32,
    pub text: String,
    pub choices: Vec<Choice>,
    pub correct: String,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub choice: String,
    pub text: String,
}
