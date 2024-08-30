use crate::{
    answer::Answer,
    quiz::{EmptyError, QuizError},
    xml_util::{write_named_formatted_scope, write_text_tag},
};
use std::fs::File;
use xml::writer::{EventWriter, XmlEvent};

/// Common trait for all question types
pub trait Question {
    /// Returns the name of the question>
    fn get_name(&self) -> &str;
    /// Returns the description of the question.
    fn get_description(&self) -> &str;
    /// Set the text rendering format `TextFormat` for the question.
    fn set_text_format(&mut self, format: TextFormat);
    /// Adds all answers from type `Vec<Answer>` to the Question variant type.
    /// May return an error if there is a problem with the fractions or count of answers.
    fn add_answers(&mut self, answers: Vec<Answer>) -> Result<(), QuizError>;
    /// Writes the question in XML format to the provided file descriptor.
    fn to_xml(&self, writer: &mut EventWriter<&File>) -> Result<(), QuizError>;
}

/// Represents the formatting options for the question text, feedback text and in other situations where Moodle could render it differently.
#[derive(Debug, Default, Copy, Clone)]
pub enum TextFormat {
    #[default]
    HTML,
    Moodle,
    Markdown,
    PlainText,
}
impl TextFormat {
    pub fn name(&self) -> &'static str {
        match self {
            TextFormat::HTML => "html",
            TextFormat::Moodle => "moodle_auto_format",
            TextFormat::Markdown => "markdown",
            TextFormat::PlainText => "plain_text",
        }
    }
}

/// Represents a base for question in Moodle XML format.
///
/// # Fields
///
/// - `name`: The name of the question.
/// - `description`: A description of the question.
/// - `question_text_format`: The format that Moodle uses to render the question.
/// - `answers`: A vector of answer objects associated with the question.
///
#[derive(Debug, Clone)]
struct QuestionBase {
    pub name: String,
    pub description: String,
    pub question_text_format: TextFormat,
    pub answers: Vec<Answer>,
}
impl QuestionBase {
    fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            question_text_format: TextFormat::default(),
            answers: Vec::new(),
        }
    }
    /// Checks if the answers create the total fraction of 100% at least
    /// There can be also cases where the total fraction is more than 100% because of multiple correct answers
    fn check_answer_fraction(&mut self) -> Result<(), QuizError> {
        let mut total_fraction = 0usize;
        for answer in &self.answers {
            total_fraction += answer.fraction as usize;
        }
        if total_fraction < 100 {
            self.answers.clear();
            return Err(QuizError::AnswerFractionError(
                "The total fraction of answers must be at least 100".to_string(),
            ));
        }
        Ok(())
    }
}

impl Question for QuestionBase {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
    fn get_description(&self) -> &str {
        self.description.as_str()
    }
    fn set_text_format(&mut self, format: TextFormat) {
        self.question_text_format = format;
    }
    fn add_answers(&mut self, answers: Vec<Answer>) -> Result<(), QuizError> {
        self.answers.extend(answers);
        self.check_answer_fraction()?;
        Ok(())
    }
    /// Writes the common part between all types of the question for provided XML EventWriter<File>
    fn to_xml(&self, writer: &mut EventWriter<&File>) -> Result<(), QuizError> {
        writer.write(XmlEvent::start_element("name"))?;
        write_text_tag(writer, self.name.as_str(), false)?;
        writer.write(XmlEvent::end_element())?;
        writer.write(
            XmlEvent::start_element("questiontext")
                .attr("format", self.question_text_format.name()),
        )?;
        // By default, the text format should be specified on the parent of the <text> element.
        write_text_tag(writer, self.description.as_str(), true)?;
        writer.write(XmlEvent::end_element())?;
        if self.answers.is_empty() {
            return Err(EmptyError.into());
        }
        for answer in &self.answers {
            answer.to_xml(writer)?;
        }
        Ok(())
    }
}

/// Multiple choice question type.
#[derive(Debug, Clone)]
pub struct MultiChoiceQuestion {
    base: QuestionBase,
    pub single: bool,
    pub shuffleanswers: bool, // Should be casted to u8 for XML
    pub correctfeedback: String,
    pub partiallycorrectfeedback: String,
    pub incorrectfeedback: String,
    // TODO use constrained type instead of string
    pub answernumbering: String,
}

impl MultiChoiceQuestion {
    /// New must take all the required fields after base wrapped with Option<> so that I can use default when not provided.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        description: String,
        single: Option<bool>,
        shuffleanswers: Option<bool>,
        correctfeedback: Option<String>,
        partiallycorrectfeedback: Option<String>,
        incorrectfeedback: Option<String>,
        answernumbering: Option<String>,
    ) -> Self {
        Self {
            base: QuestionBase::new(name, description),
            single: single.unwrap_or(true),
            shuffleanswers: shuffleanswers.unwrap_or(true),
            correctfeedback: correctfeedback.unwrap_or_default(),
            partiallycorrectfeedback: partiallycorrectfeedback.unwrap_or_default(),
            incorrectfeedback: incorrectfeedback.unwrap_or_default(),
            answernumbering: answernumbering.unwrap_or_default(),
        }
    }
}

impl Question for MultiChoiceQuestion {
    fn get_name(&self) -> &str {
        self.base.get_name()
    }
    fn get_description(&self) -> &str {
        self.base.get_description()
    }
    fn set_text_format(&mut self, format: TextFormat) {
        self.base.question_text_format = format;
    }
    fn add_answers(&mut self, answers: Vec<Answer>) -> Result<(), QuizError> {
        self.base.add_answers(answers)
    }
    fn to_xml(&self, writer: &mut EventWriter<&File>) -> Result<(), QuizError> {
        // Start question tag
        writer.write(XmlEvent::start_element("question").attr("type", "multichoice"))?;
        // Write the common part of the question
        self.base.to_xml(writer)?;

        write_named_formatted_scope(writer, "single", None, |writer| {
            writer.write(XmlEvent::characters(&self.single.to_string()))?;
            Ok(())
        })?;
        write_named_formatted_scope(writer, "shuffleanswers", None, |writer| {
            writer.write(XmlEvent::characters(
                &(self.shuffleanswers as u8).to_string(),
            ))?;
            Ok(())
        })?;
        write_named_formatted_scope(
            writer,
            "correctfeedback",
            TextFormat::default().into(),
            |writer| write_text_tag(writer, &self.correctfeedback, false),
        )?;
        write_named_formatted_scope(
            writer,
            "partiallycorrectfeedback",
            TextFormat::default().into(),
            |writer| write_text_tag(writer, &self.partiallycorrectfeedback, false),
        )?;
        write_named_formatted_scope(
            writer,
            "incorrectfeedback",
            TextFormat::default().into(),
            |writer| write_text_tag(writer, &self.incorrectfeedback, false),
        )?;
        write_named_formatted_scope(writer, "answernumbering", None, |writer| {
            writer.write(XmlEvent::characters(&self.answernumbering.to_string()))?;
            Ok(())
        })?;
        // End question tag
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct TrueFalseQuestion {
    base: QuestionBase,
}
impl TrueFalseQuestion {
    pub fn new(name: String, description: String) -> Self {
        Self {
            base: QuestionBase::new(name, description),
        }
    }
}

impl Question for TrueFalseQuestion {
    fn get_name(&self) -> &str {
        self.base.get_name()
    }
    fn get_description(&self) -> &str {
        self.base.get_description()
    }
    fn set_text_format(&mut self, format: TextFormat) {
        self.base.question_text_format = format;
    }
    fn add_answers(&mut self, answers: Vec<Answer>) -> Result<(), QuizError> {
        if answers.len() != 2 {
            return Err(QuizError::AnswerCountError(
                "True/False questions must have exactly 2 answers".to_string(),
            ));
        }
        if answers[0].fraction == 100 {
            if answers[1].fraction == 0 {
                // good
            } else {
                return Err(QuizError::AnswerFractionError(
                    "Only fractions 100 and 0 are allowed in True/False questions".to_string(),
                ));
            }
        } else if answers[1].fraction == 100 {
            if answers[0].fraction == 0 {
                // good
            } else {
                return Err(QuizError::AnswerFractionError(
                    "Only fractions 100 and 0 are allowed in True/False questions".to_string(),
                ));
            }
        } else {
            return Err(QuizError::AnswerFractionError(
                "Only fractions 100 and 0 are allowed in True/False questions".to_string(),
            ));
        }
        self.base.add_answers(answers)
    }
    fn to_xml(&self, writer: &mut EventWriter<&File>) -> Result<(), QuizError> {
        // Start question tag
        writer.write(XmlEvent::start_element("question").attr("type", "truefalse"))?;
        // Write the common part of the question
        self.base.to_xml(writer)?;
        // End question tag
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ShortAnswerQuestion {
    base: QuestionBase,
    // The <usecase> tag toggles case-sensitivity with the values 1/0.
    pub usecase: bool,
}

impl ShortAnswerQuestion {
    pub fn new(name: String, description: String, usecase: Option<bool>) -> Self {
        Self {
            base: QuestionBase::new(name, description),
            usecase: usecase.unwrap_or_default(),
        }
    }
}

impl Question for ShortAnswerQuestion {
    fn get_name(&self) -> &str {
        self.base.get_name()
    }
    fn get_description(&self) -> &str {
        self.base.get_description()
    }
    fn set_text_format(&mut self, format: TextFormat) {
        self.base.question_text_format = format;
    }
    fn add_answers(&mut self, answers: Vec<Answer>) -> Result<(), QuizError> {
        self.base.add_answers(answers)
    }
    fn to_xml(&self, writer: &mut EventWriter<&File>) -> Result<(), QuizError> {
        // Start question tag
        writer.write(XmlEvent::start_element("question").attr("type", "shortanswer"))?;
        // Write the common part of the question
        self.base.to_xml(writer)?;
        write_named_formatted_scope(writer, "usecase", None, |writer| {
            writer.write(XmlEvent::characters(&(self.usecase as u8).to_string()))?;
            Ok(())
        })?;
        // End question tag
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct EssayQuestion {
    base: QuestionBase,
}

impl EssayQuestion {
    pub fn new(name: String, description: String) -> Self {
        Self {
            base: QuestionBase::new(name, description),
        }
    }
}

impl Question for EssayQuestion {
    fn get_name(&self) -> &str {
        self.base.get_name()
    }
    fn get_description(&self) -> &str {
        self.base.get_description()
    }
    fn set_text_format(&mut self, format: TextFormat) {
        self.base.question_text_format = format;
    }
    fn add_answers(&mut self, answers: Vec<Answer>) -> Result<(), QuizError> {
        if !answers.is_empty() {
            return Err(QuizError::AnswerCountError(
                "Essay questions must not have any answers".to_string(),
            ));
        }
        Ok(())
    }
    fn to_xml(&self, writer: &mut EventWriter<&File>) -> Result<(), QuizError> {
        // Start question tag
        writer.write(XmlEvent::start_element("question").attr("type", "essay"))?;
        // Write the common part of the question
        self.base.to_xml(writer)?;
        // End question tag
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

/// Represents the different types of questions that can be included in a quiz.
///
/// - `Multichoice`: A multiple-choice question with several answer options.
/// - `TrueFalse`: A true/false question.
/// - `ShortAnswer`: A short-answer question.
/// - TODO - `Matching`: A matching question where items need to be paired.
/// - TODO - `Cloze`: A cloze (fill-in-the-blank) question.
/// - `Essay`: An essay question.
/// - TODO `Numerical`: A numerical answer question.
/// - TODO - `Description`: A descriptive question.
pub enum QuestionType {
    Multichoice(MultiChoiceQuestion),
    TrueFalse(TrueFalseQuestion),
    ShortAnswer(ShortAnswerQuestion),
    // Matching,
    // Cloze,
    Essay(EssayQuestion),
    // Numerical,
    // Description,
}
impl QuestionType {
    pub fn to_xml(&self, writer: &mut EventWriter<&File>) -> Result<(), QuizError> {
        match self {
            QuestionType::Multichoice(q) => q.to_xml(writer),
            QuestionType::TrueFalse(q) => q.to_xml(writer),
            QuestionType::ShortAnswer(q) => q.to_xml(writer),
            QuestionType::Essay(q) => q.to_xml(writer),
        }
    }
}

// Make conversion from a single question to into a vector of questions easier with `.into()`
macro_rules! impl_from_question {
    ($(($question_type:ty, $variant:ident)),+) => {
        $(
            impl<Q> From<$question_type> for Vec<Q>
            where
                Q: Question,
                $question_type: Into<Q>,
            {
                fn from(question: $question_type) -> Self {
                    vec![question.into()]
                }
            }

            impl From<$question_type> for Vec<Box<dyn Question>>
            where
                $question_type: Question + 'static,
            {
                fn from(question: $question_type) -> Self {
                    vec![Box::new(question)]
                }
            }

            impl From<$question_type> for QuestionType {
                fn from(question: $question_type) -> Self {
                    QuestionType::$variant(question)
                }
            }

            impl From<$question_type> for Vec<QuestionType> {
                fn from(question: $question_type) -> Self {
                    vec![QuestionType::$variant(question)]
                }
            }
        )+
    };
}

impl_from_question!(
    (MultiChoiceQuestion, Multichoice),
    (TrueFalseQuestion, TrueFalse),
    (ShortAnswerQuestion, ShortAnswer),
    (EssayQuestion, Essay)
);

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Seek};
    use xml::writer::EmitterConfig;

    #[test]
    fn test_multichoice_question_xml() {
        let mut tmp_file = tempfile::tempfile().unwrap();
        let mut writer = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(&tmp_file);
        let multichoice_question = MultiChoiceQuestion {
            base: QuestionBase {
                name: "Name of question".to_string(),
                description: "What is the answer to this question?".to_string(),
                question_text_format: TextFormat::HTML,
                answers: vec![
                    Answer {
                        fraction: 100,
                        text: "The correct answer".to_string(),
                        feedback: "Correct!".to_string().into(),
                        text_format: TextFormat::HTML,
                    },
                    Answer {
                        fraction: 0,
                        text: "A distractor".to_string(),
                        feedback: "Ooops!".to_string().into(),
                        text_format: TextFormat::HTML,
                    },
                    Answer {
                        fraction: 0,
                        text: "Another distractor".to_string(),
                        feedback: "Ooops!".to_string().into(),
                        text_format: TextFormat::HTML,
                    },
                ],
            },
            single: true,
            shuffleanswers: true,
            correctfeedback: "Correct!".to_string(),
            partiallycorrectfeedback: "Partially correct!".to_string(),
            incorrectfeedback: "Incorrect!".to_string(),
            answernumbering: "abc".to_string(),
        };
        multichoice_question.to_xml(&mut writer).unwrap();

        let mut buf = String::new();
        tmp_file.seek(std::io::SeekFrom::Start(0)).unwrap();
        tmp_file.read_to_string(&mut buf).unwrap();
        let expected = r#"<?xml version="1.0" encoding="utf-8"?>
<question type="multichoice">
  <name>
    <text>Name of question</text>
  </name>
  <questiontext format="html">
    <text><![CDATA[What is the answer to this question?]]></text>
  </questiontext>
  <answer fraction="100" format="html">
    <text>The correct answer</text>
    <feedback format="html">
      <text>Correct!</text>
    </feedback>
  </answer>
  <answer fraction="0" format="html">
    <text>A distractor</text>
    <feedback format="html">
      <text>Ooops!</text>
    </feedback>
  </answer>
  <answer fraction="0" format="html">
    <text>Another distractor</text>
    <feedback format="html">
      <text>Ooops!</text>
    </feedback>
  </answer>
  <single>true</single>
  <shuffleanswers>1</shuffleanswers>
  <correctfeedback format="html">
    <text>Correct!</text>
  </correctfeedback>
  <partiallycorrectfeedback format="html">
    <text>Partially correct!</text>
  </partiallycorrectfeedback>
  <incorrectfeedback format="html">
    <text>Incorrect!</text>
  </incorrectfeedback>
  <answernumbering>abc</answernumbering>
</question>"#;
        assert_eq!(expected, buf);
    }
}
