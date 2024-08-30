# moodle-xml-rs

Moodle has a specific XML format for importing and exporting questions from [Quiz Module](https://docs.moodle.org/404/en/Quiz_activity).

This project provides a Rust library for generating such XML-based Moodle quizzes.
The library attempts to ensure that the generated XML is valid and can be imported into Moodle without a hassle.

## Usage

Currently, multiple-choice, true-false and short answer questions are mainly supported.

To install, run:

```sh
cargo add moodle-xml
```

Then, you can use the library as follows:

```rust
use moodle_xml::prelude::*;

// Create a short answer question, with name and description. Use default case sensitivity which is false.
let mut question1 = ShortAnswerQuestion::new("Knowing capitals part 1".into(), "What is the capital of France?".into(), None);
// Define the fraction value for the answer, correct answer and correct answer feedback.
let answer = Answer::new(100, "Paris".into(), Some("Yes, correct!".into()));
question1.add_answers(answer.into()).unwrap();
// Create a multiple-choice question, with name and description. Set that question has a single answer and questions are shuffled.
// Also the "abc" format is used to show the answers.
let mut question2 = MultiChoiceQuestion::new(
    "Name of question".into(),
    "What is the answer to this question?".into(),
    true.into(),
    true.into(),
    // Following are the general feedbacks for the question.
    "Correct!".to_string().into(),
    "Partially correct!".to_string().into(),
    "Incorrect!".to_string().into(),
    "abc".to_string().into(),
);
let answers = vec![
    Answer::new(
        100,
        "The correct answer".into(),
        "Correct!".to_string().into(),
    ),
    Answer::new(0, "A distractor".into(), "Ooops!".to_string().into()),
    Answer::new(0, "Another distractor".into(), "Ooops!".to_string().into()),
];
question2.add_answers(answers).unwrap();

// Create a quiz with questions
let mut quiz = Quiz::new(vec![question1.into(), question2.into()]);

// Sets a category for the quiz, which will result as "$course$/capitals" in XML, creating a new category "capitals" if it doesn't exist
// when importing the quiz into Moodle.
let categories = vec!["capitals".into()];
quiz.set_categories(categories);

// Generate the XML.
// let filename = "quiz.xml";
// Since this runs as part of tests, we use a temporary file.
let tmp_file = tempfile::NamedTempFile::new().unwrap();
quiz.to_xml(tmp_file.path().to_str().unwrap());
```

The previous will generate a file named `quiz.xml` with the following content:

```xml
<?xml version="1.0" encoding="utf-8"?>
<quiz>
  <question type="category">
    <category>
      <text>$course$/capitals/</text>
    </category>
  </question>
  <question type="shortanswer">
    <name>
      <text>Knowing capitals part 1</text>
    </name>
    <questiontext format="html">
      <text><![CDATA[What is the capital of France?]]></text>
    </questiontext>
    <answer fraction="100" format="html">
      <text>Paris</text>
      <feedback format="html">
        <text>Yes, correct!</text>
      </feedback>
    </answer>
    <usecase>0</usecase>
  </question>
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
  </question>
</quiz>
```

## License

MIT
