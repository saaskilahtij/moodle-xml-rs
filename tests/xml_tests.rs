#[cfg(test)]
mod tests {
    use moodle_xml::prelude::*;
    use std::fs::File;
    use std::io::BufReader;
    use xml::reader::EventReader;

    #[test]
    fn test_file_creation() {
        let mut question =
            ShortAnswerQuestion::new("Easy question".into(), "Kenella on S rinnassa".into(), None);

        let answer = Answer::new(100, "Superman".into(), Some("Oikein".into()));
        question.add_answers(answer.into()).unwrap();

        let mut quiz = Quiz::new(question.into());
        let categories = vec!["testi_categoria".into()];
        quiz.set_categories(categories);

        let tmp_file = tempfile::NamedTempFile::new().unwrap();
        assert!(quiz.to_xml(tmp_file.path().to_str().unwrap()).is_ok());

        let file = File::open(tmp_file.path().to_str().unwrap()).expect("Cannot open file");
        let file = BufReader::new(file);

        let parser = EventReader::new(file);

        for e in parser {
            assert!(e.is_ok())
        }
    }

    #[test]
    fn pointlimit_test() {
        let mut question =
            ShortAnswerQuestion::new("Easy question".into(), "Kenella on S rinnassa".into(), None);
        let answer = Answer::new(200, "Superman".into(), Some("Oikein".into()));
        question.add_answers(answer.into()).unwrap();

        let mut quiz = Quiz::new(question.into());
        let category = vec!["testi_categoria".into()];
        quiz.set_categories(category);

        let tmp_file = tempfile::NamedTempFile::new().unwrap();
        assert!(quiz.to_xml(tmp_file.path().to_str().unwrap()).is_err());
    }

    #[test]
    fn no_answer_in_question() {
        let question =
            ShortAnswerQuestion::new("Easy question".into(), "Kenella on S rinnassa".into(), None);

        let category = vec!["testi_categoria".into()];
        let mut quiz = Quiz::new(question.into());
        quiz.set_categories(category);

        let tmp_file = tempfile::NamedTempFile::new().unwrap();
        assert!(quiz.to_xml(tmp_file.path().to_str().unwrap()).is_err());
    }

    #[test]
    fn character_test() {
        let mut question = ShortAnswerQuestion::new(
            "TRUE".into(),
            "(),./;'\"[]-=<>?:{}|\\_+!@#$%^&*()`~".into(),
            None,
        );

        let answer = Answer::new(100, "NaN".into(), Some("1E02".into()));
        question.add_answers(vec![answer]).unwrap();
        let mut quiz = Quiz::new(question.into());

        let tmp_file = tempfile::NamedTempFile::new().unwrap();
        assert!(quiz.to_xml(tmp_file.path().to_str().unwrap()).is_ok());
    }
    #[test]
    fn answer_test() {
        let mut question =
            ShortAnswerQuestion::new("Easy question".into(), "Kenella on S rinnassa".into(), None);

        let answer = Answer::new(100, "Superman".into(), Some("Oikein".into()));
        let answer2 = Answer::new(0, "Batman".into(), Some("Väärin".into()));
        let answer3 = Answer::new(0, "Robin".into(), None);
        let answer4 = Answer::new(0, "Spiderman".into(), Some("Oikein".into()));

        question
            .add_answers(vec![answer, answer2, answer3, answer4])
            .unwrap();
        let category: Category = "testi_categoria".into();
        let mut quiz = Quiz::new(question.into());
        quiz.set_categories(category.into());

        let tmp_file = tempfile::NamedTempFile::new().unwrap();
        assert!(quiz.to_xml(tmp_file.path().to_str().unwrap()).is_ok());
    }
    #[test]
    fn add_quiz_vec_xml() {
        let answer1 = Answer::new(100, "Superman".into(), Some("Oikein".into()));
        let answer2 = Answer::new(100, "Spiderman".into(), Some("Oikein".into()));
        let answer3 = Answer::new(100, "Superman".into(), Some("Oikein".into()));

        let answers = vec![answer1, answer2, answer3.clone()];

        let mut question1 =
            ShortAnswerQuestion::new("Easy question".into(), "Kenella on S rinnassa".into(), None);
        let mut question2 = ShortAnswerQuestion::new(
            "Easier question".into(),
            "Kenella on S rinnassa".into(),
            None,
        );

        let _ = question1.add_answers(answers);
        let _ = question2.add_answers(answer3.into());

        let questions: Vec<QuestionType> = vec![question1.into(), question2.into()];

        let category: Category = "testi_categoria".into();
        let mut quiz = Quiz::new(questions);
        quiz.set_categories(category.into());

        let tmp_file = tempfile::NamedTempFile::new().unwrap();
        assert!(quiz.to_xml(tmp_file.path().to_str().unwrap()).is_ok());
    }
}
