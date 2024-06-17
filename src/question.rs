use xml::writer::{EventWriter, XmlEvent};
use std::fs::File;
use crate::{answer::Answer, quiz::{EmptyError, QuizError}};

/// Represents a question in Moodle XML format.
///
/// # Fields
///
/// - `name`: The name of the question.
/// - `description`: A description of the question.
/// - `question_type`: The type of the question (e.g., multiple choice, true/false).
/// - `answers`: A vector of answer objects associated with the question.
///
pub struct Question{
    pub name: String,
    pub description: String,
    pub question_type: QuestionType,
    pub answers: Vec<Answer>
}


/// Represents the different types of questions that can be included in a quiz.
///
/// - `Multichoice`: A multiple-choice question with several answer options.
/// - `TrueFalse`: A true/false question.
/// - `ShortAnswer`: A short-answer question.
/// - `Matching`: A matching question where items need to be paired.
/// - `Cloze`: A cloze (fill-in-the-blank) question.
/// - `Essay`: An essay question.
/// - `Numerical`: A numerical answer question.
/// - `Description`: A descriptive question.
pub enum QuestionType{
    Multichoice,
    TrueFalse,
    ShortAnswer,
    Matching,
    Cloze,
    Essay,
    Numerical,
    Description
}
impl QuestionType{
    /// Returns QuestionType name as &str
    pub fn name(&self) -> &'static str{
        match self{
            QuestionType::Multichoice => "multichoice",
            QuestionType::TrueFalse => "truefalse",
            QuestionType::ShortAnswer => "shortanswer",
            QuestionType::Matching => "matching",
            QuestionType::Cloze => "cloze",
            QuestionType::Essay => "essay",
            QuestionType::Numerical => "numerical",
            QuestionType::Description => "description"
        }
    }
}
impl Question{
    /// Creates a new instance of the Question struct.
    /// 
    /// ### Arguments
    /// 
    /// * `new_name` - The name for the new instance.
    /// * `new_description` - The description for the new instance.
    /// * `new_question_type` - The question type for the new instance.
    /// 
    /// ### Returns
    /// 
    /// A new instance of the Question struct.
    pub fn new(new_name: String, new_description: String, new_question_type: QuestionType) -> Self{
        Self {
            name: new_name,
            description: new_description,
            question_type: new_question_type,
            answers: Vec::new()
        }
    }
    /// Adds an Answer type to the Question struct.
    /// 
    /// ### Arguments
    /// 
    /// * `answer` - The Answer to be added.
    pub fn add_answer(&mut self, answer: Answer){
        let new_answer = Answer{
            fraction: answer.fraction,
            text: answer.text,
            feedback: answer.feedback
        };
        self.answers.push(new_answer)
    }
    /// Adds all answers from type Vec<Answer> to the Question struct
    /// 
    /// ### Arguments
    /// 
    /// * `answers` - Vector of answers to be added
    pub fn add_answers(&mut self, answers: Vec<Answer>){
        for answer in answers{
            let new_answer = Answer{
                fraction: answer.fraction,
                text: answer.text,
                feedback: answer.feedback
            };
            self.answers.push(new_answer)
        }
    }
    /// writes the question part of xml for provided EventWriter<File>
    pub fn question_xml(&mut self,  writer: &mut EventWriter<File>) -> Result<(),QuizError>{
        writer.write(XmlEvent::start_element("question").attr("type", self.question_type.name()))?;
        writer.write(XmlEvent::start_element("name"))?;
        writer.write(XmlEvent::start_element("text"))?;
        writer.write(XmlEvent::characters(self.name.as_str()))?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::start_element("questiontext").attr("format","html"))?;
        writer.write(XmlEvent::start_element("text"))?;
        writer.write(XmlEvent::cdata(self.description.as_str()))?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        if self.answers.is_empty(){
            return Err(EmptyError.into())
        }
        for answer in &mut self.answers{
            answer.answer_xml(writer)?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}