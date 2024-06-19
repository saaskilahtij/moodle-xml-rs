#[cfg(test)]
mod tests{

    use moodle_xml::quiz::Quiz;
    use moodle_xml::question::Question;
    use moodle_xml::question::QuestionType;
    use moodle_xml::answer::Answer;
    

    #[test]
    fn add_quiz_xml(){
        let mut quiz = Quiz::new("testi_categoria".into(),None);

        let shortq = QuestionType::ShortAnswer;

        let mut question = Question::new("Easy question".into(), "Kenella on S rinnassa".into(), shortq);
        
        let answer = Answer::new(100, "Superman".into(), Some("Oikein".into()));

        question.add_answer(answer);
        quiz.add_question(question);

        
        assert!(quiz.quiz_xml("".into(), "testi_quiz.xml".into()).is_ok());
    }
    #[test]
    fn pointlimit_test(){
        let mut quiz = Quiz::new("testi_categoria".into(),None);

        let shortq = QuestionType::ShortAnswer;

        let mut question = Question::new("Easy question".into(), "Kenella on S rinnassa".into(), shortq);
        
        let answer = Answer::new(200, "Superman".into(), Some("Oikein".into()));

        question.add_answer(answer);
        quiz.add_question(question);

        
        assert!(quiz.quiz_xml("".into(), "testi_quiz.xml".into()).is_err());
    }

    #[test]
    fn empty_quiz(){
        let mut quiz = Quiz::new("testi_categoria".into(),None);

        // Should return error with no questions
        assert!(quiz.quiz_xml("".into(), "testi_quiz.xml".into()).is_err());
    }

    #[test]
    fn empty_question(){
        let mut quiz = Quiz::new("testi_categoria".into(),None);

        let shortq = QuestionType::ShortAnswer;

        let question = Question::new("Easy question".into(), "Kenella on S rinnassa".into(), shortq);
        
        
        quiz.add_question(question);

        
        assert!(quiz.quiz_xml("".into(), "testi_quiz.xml".into()).is_err());
    }

    #[test]
    fn character_test(){
        let mut quiz = Quiz::new("undefined".into(),None);

        let shortq = QuestionType::ShortAnswer;

        let mut question = Question::new("TRUE".into(), "(),./;'\"[]-=<>?:{}|\\_+!@#$%^&*()`~".into(), shortq);
        
        let answer = Answer::new(100, "NaN".into(), Some("1E02".into()));

        question.add_answer(answer);
        quiz.add_question(question);

        
        assert!(quiz.quiz_xml("".into(), "testi_quiz.xml".into()).is_ok());
    }
    #[test]
    fn answer_test(){
        let mut quiz = Quiz::new("testi_categoria".into(),None);

        let shortq = QuestionType::ShortAnswer;

        let mut question = Question::new("Easy question".into(), "Kenella on S rinnassa".into(), shortq);
        
        let answer = Answer::new(100, "Superman".into(), Some("Oikein".into()));
        let answer2 = Answer::new(0, "Batman".into(), Some("Väärin".into()));
        let answer3 = Answer::new(0, "Robin".into(), None);
        let answer4 =  Answer::new(0, "Spiderman".into(), Some("Oikein".into()));

        question.add_answer(answer);
        question.add_answer(answer2);
        question.add_answer(answer3);
        question.add_answer(answer4);
        quiz.add_question(question);

        
        assert!(quiz.quiz_xml("".into(), "testi_quiz.xml".into()).is_ok());
    }
    #[test]
    fn add_quiz_vec_xml(){
        let mut quiz = Quiz::new("testi_categoria".into(),None);

        let shortq = QuestionType::ShortAnswer;

        let answer1 = Answer::new(100, "Superman".into(), Some("Oikein".into()));
        let answer2 = Answer::new(100, "Spiderman".into(), Some("Oikein".into()));
        let answer3 = Answer::new(100, "Superman".into(), Some("Oikein".into()));

        let mut answers = Vec::new();
        answers.push(answer1);
        answers.push(answer2);

        let mut question1 = Question::new("Easy question".into(), "Kenella on S rinnassa".into(), shortq);
        let mut question2 = Question::new("Easier question".into(), "Kenella on S rinnassa".into(), QuestionType::ShortAnswer);

        question1.add_answers(answers);
        question2.add_answer(answer3);


        let mut questions = Vec::new();
        questions.push(question1);
        questions.push(question2);
        
        
        quiz.add_questions(questions);

        
        assert!(quiz.quiz_xml("".into(), "testi_quiz.xml".into()).is_ok());
    }

}