use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub label: String
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html!(
        <button>{props.label.clone()}</button>
    )
}