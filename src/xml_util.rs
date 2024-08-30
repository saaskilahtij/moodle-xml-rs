//! Utility functions for writing XML elements
//! The main purpose is to reduce code duplication
//! and make sure that every element has a single start and end event.
//! The functions are specific to Moodle XML structure.

use crate::question::TextFormat;
use crate::quiz::QuizError;
use std::fs::File;
use xml::writer::{EventWriter, XmlEvent};

/// Writes a text named tag and a add text inside it, wheter plain or wrapped with CDATA
pub fn write_text_tag(
    writer: &mut EventWriter<&File>,
    data: &str,
    cdata: bool,
) -> Result<(), QuizError> {
    writer.write(XmlEvent::start_element("text"))?;
    if cdata {
        writer.write(XmlEvent::cdata(data))?;
    } else {
        writer.write(XmlEvent::characters(data))?;
    }
    writer.write(XmlEvent::end_element())?;
    Ok(())
}
/// Writes something inside a tag with name `name` which has an optional format attribute `format`
///
/// <correctfeedback format="html">
/// ... scope start...
/// <text>The Answer is good!</text>
/// ... scope end...
/// </correctfeedback>
pub fn write_named_formatted_scope<F>(
    writer: &mut EventWriter<&File>,
    name: &str,
    format: Option<TextFormat>,
    scope: F,
) -> Result<(), QuizError>
where
    F: FnOnce(&mut EventWriter<&File>) -> Result<(), QuizError>,
{
    if let Some(format) = format {
        writer.write(XmlEvent::start_element(name).attr("format", format.name()))?;
    } else {
        writer.write(XmlEvent::start_element(name))?;
    }
    scope(writer)?;
    writer.write(XmlEvent::end_element())?;
    Ok(())
}
