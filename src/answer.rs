use std::fs::File;
use xml::writer::{EventWriter, XmlEvent};

use crate::question::TextFormat;
use crate::quiz::QuizError;
use crate::xml_util::{write_named_formatted_scope, write_text_tag};

#[derive(Debug, Clone)]
pub struct Answer {
    pub fraction: u8,
    pub text: String,
    pub feedback: Option<String>,
    pub text_format: TextFormat,
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
            text_format: TextFormat::default(),
        }
    }
    /// Sets the text rendering format for the answer and feedback. Default is HTML.
    pub fn set_text_format(&mut self, text_format: TextFormat) {
        self.text_format = text_format;
    }
    /// Writes answer part of xml for EventWriter
    pub(crate) fn to_xml(&self, writer: &mut EventWriter<&File>) -> Result<(), QuizError> {
        if self.fraction > 100 {
            return Err(QuizError::AnswerFractionError(
                "Answer fraction is larger than 100".to_string(),
            ));
        }
        writer.write(
            XmlEvent::start_element("answer")
                .attr("fraction", self.fraction.to_string().as_str())
                .attr("format", self.text_format.name()),
        )?;
        write_text_tag(writer, self.text.as_str(), false)?;
        if let Some(string) = self.feedback.as_ref() {
            write_named_formatted_scope(writer, "feedback", self.text_format.into(), |writer| {
                write_text_tag(writer, string, false)?;
                Ok(())
            })?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl From<Answer> for Vec<Answer> {
    fn from(answer: Answer) -> Self {
        vec![answer]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Seek};
    use xml::writer::EmitterConfig;

    #[test]
    fn test_single_answer() {
        let mut tmp_file = tempfile::tempfile().unwrap();
        let mut writer = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(&tmp_file);

        let mut answer = Answer::new(
            100,
            "Answer text".to_string(),
            "Particularly well answered!".to_string().into(),
        );
        answer.set_text_format(TextFormat::Moodle);
        answer.to_xml(&mut writer).unwrap();
        let mut buf = String::new();
        tmp_file.seek(std::io::SeekFrom::Start(0)).unwrap();
        tmp_file.read_to_string(&mut buf).unwrap();
        let expected = r#"<?xml version="1.0" encoding="utf-8"?>
<answer fraction="100" format="moodle_auto_format">
  <text>Answer text</text>
  <feedback format="moodle_auto_format">
    <text>Particularly well answered!</text>
  </feedback>
</answer>"#;
        assert_eq!(expected, buf);
    }
}
