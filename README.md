# moodle-xml-rs

Moodle has a specific XML format for importing and exporting questions from [Quiz Module](https://docs.moodle.org/404/en/Quiz_activity).

This project provides a Rust library for generating such XML-based Moodle quizzes.
The library attempts to ensure that the generated XML is valid and can be imported into Moodle without a hassle.

## Usage

Currently, multiple-choice, true-false and short answer questions are mainly supported.

```rust
use moodle_xml::prelude::*;

// Create a short answer question, with name and description. Use default case sensitivity which is false.
let mut question = ShortAnswerQuestion::new("Knowing capitals part 1".into(), "What is the capital of France?".into(), None);
// Define the fraction value for the answer, correct answer and correct answer feedback.
let answer = Answer::new(100, "Paris".into(), Some("Yes, correct!".into()));
question.add_answers(answer.into()).unwrap();

// Create a quiz with questions
let mut quiz = Quiz::new(question.into());

// Sets a category for the quiz, which will result as "$course$/capitals" in XML, creating a new category "capitals" if it doesn't exist
// when importing the quiz into Moodle.
let categories = vec!["capitals".into()];
quiz.set_categories(categories);

// Generate the XML.
let filename = "quiz.xml";
quiz.to_xml(filename).unwrap();
```

The previous will generate a file named "quiz.xml" with the following content:

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
    </quiz>
```

## License

MIT
