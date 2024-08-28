use std::fs::File;
use xml::writer::{EventWriter, XmlEvent};

use crate::quiz::{QuizError, ValueError};

pub struct Answer {
    pub fraction: u8,
    pub text: String,
    pub feedback: Option<String>,
}

impl Answer {
    /// Generates a new Answer type struct
    ///
    /// ### Arguments
    /// * `new_fraction` - The amount of points answer gives from 0-100
    /// * `new_text` - Text displayed on the answer.
    /// * `new_feedback` - Feedback displayed on the answer can be left empty with None.
    pub fn new(new_fraction: u8, new_text: String, new_feedback: Option<String>) -> Self {
        Self {
            fraction: new_fraction,
            text: new_text,
            feedback: new_feedback,
        }
    }
    /// Writes answer part of xml for EventWriter<File>
    pub fn answer_xml(&mut self, writer: &mut EventWriter<File>) -> Result<(), QuizError> {
        if self.fraction > 100 {
            return Err(ValueError.into());
        }
        writer.write(
            XmlEvent::start_element("answer").attr("fraction", self.fraction.to_string().as_str()),
        )?;
        writer.write(XmlEvent::start_element("text"))?;
        writer.write(XmlEvent::characters(self.text.as_str()))?;
        writer.write(XmlEvent::end_element())?;
        if self.feedback.is_some() {
            writer.write(XmlEvent::start_element("feedback"))?;
            writer.write(XmlEvent::start_element("text"))?;
            let string = self.feedback.as_mut().expect("value should be String");
            writer.write(XmlEvent::characters(string.as_str()))?;
            writer.write(XmlEvent::end_element())?;
            writer.write(XmlEvent::end_element())?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}
