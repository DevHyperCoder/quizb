use ask_quiz_derive::AskQuiz;

pub trait AskQuiz {
    fn ask();
}

#[derive(AskQuiz)]
#[ask_quiz(file = "./res/quiz.json")]
struct Quiz;

fn main() {
    println!("quizb");

    Quiz::ask();
}
