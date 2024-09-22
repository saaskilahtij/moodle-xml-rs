#![no_main]

use libfuzzer_sys::fuzz_target;
use moodle_xml::prelude::*;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = ShortAnswerQuestion::new(s.into(), s.into(), None);
    }
});
