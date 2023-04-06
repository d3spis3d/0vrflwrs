use rocket::{serde::json::Json, State};

use crate::models::*;
use crate::persistance::{answers_dao::AnswersDao, questions_dao::QuestionsDao};

mod handlers_inner;

use handlers_inner::*;

#[derive(Responder)]
pub enum APIError {
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 500)]
    InternalError(String),
}

impl From<HandlerError> for APIError {
    fn from(value: HandlerError) -> Self {
        match value {
            HandlerError::BadRequest(s) => Self::BadRequest(s),
            HandlerError::InternalError(s) => Self::InternalError(s),
        }
    }
}

// ---- CRUD for Questions ----

#[post("/question", data = "<question>")]
pub async fn create_question(
    question: Json<Question>,
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) -> Result<Json<QuestionDetail>, APIError> {
    let question_detail =
        handlers_inner::create_question(question.into_inner(), questions_dao).await?;

    Ok(Json(question_detail))
}

#[get("/questions")]
pub async fn read_questions(
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) -> Result<Json<Vec<QuestionDetail>>, APIError> {
    let questions = handlers_inner::read_questions(questions_dao).await?;

    Ok(Json(questions))
}

#[get("/question/<question_uuid>")]
pub async fn read_question(
    question_uuid: &str,
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) -> Result<Json<QuestionDetail>, APIError> {
    let question = handlers_inner::read_question(question_uuid, questions_dao).await?;

    Ok(Json(question))
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(
    question_uuid: Json<QuestionId>,
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) -> Result<(), APIError> {
    handlers_inner::delete_question(question_uuid.into_inner(), questions_dao).await?;

    Ok(())
}

#[post("/answer", data = "<answer>")]
pub async fn create_answer(
    answer: Json<Answer>,
    answers_dao: &State<Box<dyn AnswersDao + Send + Sync>>,
) -> Result<Json<AnswerDetail>, APIError> {
    let answer_detail = handlers_inner::create_answer(answer.into_inner(), answers_dao).await?;

    Ok(Json(answer_detail))
}

#[get("/question/<question_uuid>/answers")]
pub async fn read_answers(
    question_uuid: &str,
    answers_dao: &State<Box<dyn AnswersDao + Send + Sync>>,
) -> Result<Json<Vec<AnswerDetail>>, APIError> {
    let question_id = QuestionId {
        question_uuid: question_uuid.to_owned(),
    };
    let answers = handlers_inner::read_answers(question_id, answers_dao).await?;

    Ok(Json(answers))
}

#[delete("/answer", data = "<answer_uuid>")]
pub async fn delete_answer(
    answer_uuid: Json<AnswerId>,
    answers_dao: &State<Box<dyn AnswersDao + Send + Sync>>,
) -> Result<(), APIError> {
    handlers_inner::delete_answer(answer_uuid.into_inner(), answers_dao).await?;

    Ok(())
}
