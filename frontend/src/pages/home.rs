use crate::app::Route;
use common::QuestionDetail;
use gloo_net::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Properties)]
struct QuestionProps {
    question: QuestionDetail,
}

#[function_component(Question)]
fn question(QuestionProps { question }: &QuestionProps) -> Html {
    let navigator = use_navigator().unwrap();

    let on_question_click = {
        let navigator = navigator.clone();
        let question = question.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Question {
                id: question.question_uuid.clone(),
            });
        })
    };

    html! {
        <div onclick={on_question_click}>
            <h3>{question.title.clone()}</h3>
            <p>{question.created_at.clone()}</p>
            <p>{question.description.clone()}</p>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct QuestionListProps {
    questions: Vec<QuestionDetail>,
}

#[function_component(QuestionList)]
fn question_list(QuestionListProps { questions }: &QuestionListProps) -> Html {
    let question_list = questions
        .iter()
        .map(|question| {
            html! {
                <Question question={question.clone()} />
            }
        })
        .collect::<Html>();

    html! {
        <div>{question_list}</div>
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    let questions = use_state(|| vec![]);
    {
        let questions = questions.clone();
        use_effect_with_deps(
            move |_| {
                let questions = questions.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_questions: Vec<QuestionDetail> = Request::get("/api/v1/questions")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    questions.set(fetched_questions);
                })
            },
            (),
        )
    }

    html! {
        <>
            <h1>{ "Welcome to 0vrflwrs" }</h1>
            <QuestionList questions={(*questions).clone()} />
        </>
    }
}
