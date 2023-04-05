use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct QuestionProps {
    pub id: String,
}

#[function_component(Question)]
pub fn question(QuestionProps { id }: &QuestionProps) -> Html {
    html! {
        <h1>{format!("Question: {}", id)}</h1>
    }
}
