use rocket::{serde::json::Json, State};

use crate::models::*;
use crate::persistance::{answers_dao::AnswersDao, questions_dao::QuestionsDao};

mod handlers_inner;

// ---- CRUD for Questions ----

#[post("/question", data = "<question>")]
pub async fn create_question(
    question: Json<Question>,
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) -> Json<QuestionDetail> {
    Json(QuestionDetail {
        question_uuid: "0001".to_owned(),
        title: "Dummy Question".to_owned(),
        description: "What's a quetion".to_owned(),
        created_at: "Now".to_owned(),
    })
}

#[get("/questions")]
pub async fn read_questions(
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) -> Json<Vec<QuestionDetail>> {
    Json(vec![QuestionDetail {
        question_uuid: "0001".to_owned(),
        title: "Dummy Question".to_owned(),
        description: "What's a quetion".to_owned(),
        created_at: "Now".to_owned(),
    }])
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(
    question_uuid: Json<QuestionId>,
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) {
}

#[post("/answer", data = "<answer>")]
pub async fn create_answer(
    answer: Json<Answer>,
    answers_dao: &State<Box<dyn AnswersDao + Send + Sync>>,
) -> Json<AnswerDetail> {
    Json(AnswerDetail {
        answer_uuid: "0002".to_owned(),
        question_uuid: "0001".to_owned(),
        content: "An answer".to_owned(),
        created_at: "Before".to_owned(),
    })
}

#[get("/answers", data = "<question_uuid>")]
pub async fn read_answers(
    question_uuid: Json<QuestionId>,
    answers_dao: &State<Box<dyn AnswersDao + Send + Sync>>,
) -> Json<Vec<AnswerDetail>> {
    Json(vec![AnswerDetail {
        answer_uuid: "0002".to_owned(),
        question_uuid: "0001".to_owned(),
        content: "An answer".to_owned(),
        created_at: "Before".to_owned(),
    }])
}

#[delete("/answer", data = "<answer_uuid>")]
pub async fn delete_answer(
    answer_uuid: Json<AnswerId>,
    answers_dao: &State<Box<dyn AnswersDao + Send + Sync>>,
) {
}
