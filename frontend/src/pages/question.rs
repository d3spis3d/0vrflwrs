use common::{AnswerDetail, QuestionDetail};
use gloo_net::http::Request;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
struct AnswerListProps {
    answers: Vec<AnswerDetail>,
}

#[function_component(AnswerList)]
fn answer_list(AnswerListProps { answers }: &AnswerListProps) -> Html {
    if answers.len() == 0 {
        return html! {
            <div>{"There are no answers at this time"}</div>
        };
    }
    html! {
        <div>{
            answers.iter().enumerate().map(|(i, answer)| html! {
                <div>
                    <h4>{"Answer "}{i+1}</h4>
                    <p>{answer.content.clone()}</p>
                    <p>{answer.created_at.clone()}</p>
                </div>
            }).collect::<Html>()
        }</div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct QuestionProps {
    pub id: String,
}

#[function_component(Question)]
pub fn question(QuestionProps { id }: &QuestionProps) -> Html {
    let question = use_state(|| None);
    let answers = use_state(|| vec![]);

    {
        let question = question.clone();
        let answers = answers.clone();
        let id = id.clone();

        use_effect_with_deps(
            move |_| {
                let question = question.clone();
                let answers = answers.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_question: QuestionDetail =
                        Request::get(&format!("/api/v1/question/{}", id).as_str())
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    question.set(Some(fetched_question));

                    let fetched_answers: Vec<AnswerDetail> =
                        Request::get(format!("/api/v1/question/{}/answers", id).as_str())
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    answers.set(fetched_answers);
                })
            },
            (),
        )
    }

    match (*question).as_ref() {
        Some(question) => {
            html! {
                <>
                    <h1>{question.title.clone()}</h1>
                    <p>{question.description.clone()}</p>
                    <hr />
                    <AnswerList answers={(*answers).clone()} />
                </>
            }
        }
        None => html! {
            <h1>{"Error :("}</h1>
        },
    }
}
