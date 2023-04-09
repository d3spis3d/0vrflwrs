use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct InputFieldProps {
    pub input_node_ref: NodeRef,
    pub field_type: String,
    pub label: String,
    pub name: String,
}

#[function_component(InputField)]
pub fn input_field(
    InputFieldProps {
        input_node_ref,
        field_type,
        label,
        name,
    }: &InputFieldProps,
) -> Html {
    html! {
        <label for={name.clone()}>
            {label}
            <input type={field_type.clone()} name={name.clone()} ref={input_node_ref.clone()} />
        </label>
    }
}
