use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quiz::Question;
use quote::quote;
use serde_json::Error;
use std::fs;
use syn;

mod quiz;

use crate::quiz::Quiz;

// Options / Attributes for derive macro
#[derive(FromDeriveInput, Debug, Default)]
#[darling(default, attributes(ask_quiz))]
struct Opts {
    file: String,
}

// Derive the AskQuiz Trait
#[proc_macro_derive(AskQuiz, attributes(ask_quiz))]
pub fn ask_quiz_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_quiz(&ast)
}

fn impl_quiz(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // Parse options and panic if incorrect
    let opts = Opts::from_derive_input(&ast).expect("Wrong options");
    println!("{:?}", opts);
    if opts.file == "" {
        panic!("No file provided");
    }

    // Read and parse file

    let file = opts.file;
    let file_content = fs::read_to_string(&file).unwrap();

    let quiz = match serde_json::from_str::<Quiz>(&file_content) {
        Ok(q) => q,
        Err(e) => panic!("{}", e),
    };

    // codegen for questions
    let questions: &Vec<TokenStream2> = &quiz.questions.iter().map(|q| gen_question(q)).collect();

    let gn_code = quote! {
        use std::collections::HashMap;
        use std::io::{self,Read};

        impl AskQuiz for #name {
             fn ask() {
                 let mut answers = HashMap::new();

                 let mut ans_string = String::new();

                #(#questions)*

                 println!("{:?}",answers);

            }
        }
    };

    gn_code.into()
}

/// Generates a code block (TokenStream2) for given question
fn gen_question(q: &Question) -> TokenStream2 {
    let q_text = &q.text;
    let q_num = &q.q_num;
    let q_choices = gen_choices(&q);

    quote! {
        println!("Q {}: {}",#q_num,#q_text);
        #(#q_choices)*

        // Read data now.
        ans_string.clear();
        io::stdin().read_line(&mut ans_string).unwrap();

        answers.insert(#q_num,ans_string.clone().replace("\n",""));
        println!("Ok, marked!");
    }
}

fn gen_choices(q: &Question) -> Vec<TokenStream2> {
    q.choices
        .iter()
        .map(|choice| {
            let text = &choice.text;
            let choice = &choice.choice;
            quote! {
                println!(" - {}) {}",#choice,#text);
            }
        })
        .collect()
}
