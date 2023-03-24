use rocket::serde::json::Json;

use crate::models::*;

// ---- CRUD for Questions ----

#[post("/question", data = "<question>")]
pub async fn create_question(question: Json<Question>) -> Json<QuestionDetail> {
    Json(QuestionDetail {
        question_uuid: "0001".to_owned(),
        title: "Dummy Question".to_owned(),
        description: "What's a quetion".to_owned(),
        created_at: "Now".to_owned(),
    })
}

#[get("/questions")]
pub async fn read_questions() -> Json<Vec<QuestionDetail>> {
    Json(vec![QuestionDetail {
        question_uuid: "0001".to_owned(),
        title: "Dummy Question".to_owned(),
        description: "What's a quetion".to_owned(),
        created_at: "Now".to_owned(),
    }])
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(question_uuid: Json<QuestionId>) {}

#[post("/answer", data = "<answer>")]
pub async fn create_answer(answer: Json<Answer>) -> Json<AnswerDetail> {
    Json(AnswerDetail {
        answer_uuid: "0002".to_owned(),
        question_uuid: "0001".to_owned(),
        content: "An answer".to_owned(),
        created_at: "Before".to_owned(),
    })
}

#[get("/answers", data = "<question_uuid>")]
pub async fn read_answers(question_uuid: Json<QuestionId>) -> Json<Vec<AnswerDetail>> {
    Json(vec![AnswerDetail {
        answer_uuid: "0002".to_owned(),
        question_uuid: "0001".to_owned(),
        content: "An answer".to_owned(),
        created_at: "Before".to_owned(),
    }])
}

#[delete("/answer", data = "<answer_uuid>")]
pub async fn delete_answer(answer_uuid: Json<AnswerId>) {}
