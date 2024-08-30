pub mod answer;
pub mod question;
pub mod quiz;
mod xml_util;

/// A prelude containing the esstential types
pub mod prelude {
    pub use crate::{
        answer::Answer,
        question::{
            EssayQuestion, MultiChoiceQuestion, Question, QuestionType, ShortAnswerQuestion,
            TextFormat, TrueFalseQuestion,
        },
        quiz::{Category, Quiz, QuizError},
    };
}
