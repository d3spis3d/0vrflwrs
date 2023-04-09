use crate::{app::Route, components::input::InputField};
use common::QuestionDetail;
use gloo_net::http::{Headers, Request};
use serde_json;
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq)]
struct QuestionFormData {
    title: String,
    description: String,
}

impl QuestionFormData {
    fn validate(&self) -> bool {
        self.title.len() > 0 && self.description.len() > 0
    }
}
#[function_component(QuestionForm)]
pub fn question_form() -> Html {
    let form_invalid = use_state(|| false);
    let title = use_node_ref();
    let description = use_node_ref();
    let navigator = use_navigator().unwrap();

    let on_submit = {
        let title = title.clone();
        let description = description.clone();
        let form_invalid = form_invalid.clone();
        let navigator = navigator.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let navigator = navigator.clone();

            let title = title.cast::<HtmlInputElement>().unwrap().value();
            let description = description.cast::<HtmlInputElement>().unwrap().value();

            let question_form_data = QuestionFormData { title, description };
            let valid = question_form_data.validate();

            if !valid {
                form_invalid.set(true);
                return;
            }

            // Make POST request with data and call on_success
            wasm_bindgen_futures::spawn_local(async move {
                let new_question: QuestionDetail = Request::post("/api/v1/question")
                    .headers({
                        let headers = Headers::new();
                        headers.set("Content-Type", "application/json");
                        headers
                    })
                    .body(JsValue::from(
                        serde_json::to_string(&common::Question {
                            title: question_form_data.title,
                            description: question_form_data.description,
                        })
                        .unwrap(),
                    ))
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                navigator.push(&Route::Question {
                    id: new_question.question_uuid,
                });
            })
        })
    };

    html! {
        <form onsubmit={on_submit}>
            <InputField input_node_ref={title} label={"Title".to_owned()} name={"title".to_owned()} field_type={"text".to_owned()} />
            <InputField input_node_ref={description} label={"Description".to_owned()} name={"description".to_owned()} field_type={"text".to_owned()} />
            <button>{"Create Question"}</button>
        </form>
    }
}

#[function_component(Create)]
pub fn create() -> Html {
    html! {
        <>
            <h1>{"Create Question"}</h1>
            <QuestionForm />
        </>
    }
}
