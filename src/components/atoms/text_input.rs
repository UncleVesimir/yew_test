use gloo::console::log;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub name:  String,
}




#[function_component(TextInput)]
pub fn text_input (props: &InputProps) -> Html {
    let onchange = Callback::from(|event: Event| {
        let target = event.target().unwrap();
        log!(target.value_of());
    });
    html!(
        <input type={"text"} name={props.name.to_string()} onchange={onchange}/>
    )
}