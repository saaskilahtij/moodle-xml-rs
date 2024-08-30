use crate::question::QuestionType;
use std::fs::File;
use std::{fmt, ops::Deref};
use xml::writer::{EmitterConfig, XmlEvent};

/// Error type for Quiz, Question and Answer struct
///
/// ### Errors
///
/// XMLWriterError ```xml::writer::Error``` - xml-rs writer error
///
/// ```EmptyError``` - Error when generating empty quiz or question
///
/// ```ValueError``` - Error when generating answer with too much points
/// AnswerFractionError - Error when answer fraction is larger than 100
/// AnswerCountError - Error when answer count is different than required
#[derive(Debug)]
pub enum QuizError {
    XMLWriterError(xml::writer::Error),
    EmptyError(String),
    ValueError(String),
    AnswerFractionError(String),
    AnswerCountError(String),
}
impl From<xml::writer::Error> for QuizError {
    fn from(e: xml::writer::Error) -> Self {
        QuizError::XMLWriterError(e)
    }
}
impl From<EmptyError> for QuizError {
    fn from(e: EmptyError) -> Self {
        QuizError::EmptyError(e.to_string())
    }
}
impl From<ValueError> for QuizError {
    fn from(e: ValueError) -> Self {
        QuizError::ValueError(e.to_string())
    }
}

#[derive(Debug)]
pub struct EmptyError;

impl fmt::Display for EmptyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Quiz questions or answer is empty")
    }
}

#[derive(Debug)]
pub struct ValueError;

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Answer type has value outside of limits")
    }
}

/// A category for the quiz, can be used to categorize questions.
/// A string type is used to represent the category.
#[derive(Debug, Clone)]
pub struct Category(String);

impl Deref for Category {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<String> for Category {
    fn from(s: String) -> Self {
        Category(s)
    }
}
impl From<&str> for Category {
    fn from(s: &str) -> Self {
        Category(s.to_string())
    }
}
impl From<Category> for Vec<Category> {
    fn from(category: Category) -> Self {
        vec![category]
    }
}

pub struct Quiz {
    /// A vector of questions, can be any type of a question
    questions: Vec<QuestionType>,
    categories: Option<Vec<Category>>,
}
impl Quiz {
    /// Creates a new quiz instance with the specified moodle categories and questions.
    /// Categories are not mandatory.
    /// See [Moodle XML format](https://docs.moodle.org/404/en/Moodle_XML_format) for more information.
    /// Category entry is appended after `$course$/` mark.
    pub fn new(questions: Vec<QuestionType>) -> Self {
        Self {
            questions,
            categories: None,
        }
    }
    /// Adds categories to the quiz.
    pub fn set_categories(&mut self, categories: Vec<Category>) {
        self.categories = Some(categories);
    }
    /// Creates an XML file from quiz object, containing question and answer objects.
    ///
    /// # Arguments
    ///
    /// - `filename`: The name of the XML file, including whole filepath.
    ///
    /// # Errors
    ///
    /// Returns an QuizError if the problem occurs during writing the XML file or requirements are not met.

    pub fn to_xml(&mut self, filename: &str) -> Result<(), QuizError> {
        if self.questions.is_empty() {
            return Err(EmptyError.into());
        }
        let output: File = File::create(filename)
            .unwrap_or_else(|e| panic!("Bad file path: {} More: {}", filename, e));
        let mut writer = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(&output);

        writer.write(XmlEvent::start_element("quiz"))?;
        if let Some(categories) = self.categories.as_ref() {
            for category in categories {
                writer.write(XmlEvent::start_element("question").attr("type", "category"))?;
                writer.write(XmlEvent::start_element("category"))?;
                writer.write(XmlEvent::start_element("text"))?;
                let string = ["$course$/", category.as_str(), "/"].concat();
                writer.write(XmlEvent::characters(string.as_str()))?;
                writer.write(XmlEvent::end_element())?;
                writer.write(XmlEvent::end_element())?;
                writer.write(XmlEvent::end_element())?;
            }
        }

        if self.questions.is_empty() {
            return Err(EmptyError.into());
        }
        for question in &self.questions {
            question.to_xml(&mut writer)?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}
