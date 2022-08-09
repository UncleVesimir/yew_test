use yew::prelude::*;
use crate::components::atoms::{
    text_input::TextInput,
    button::Button
}; 

#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    html!(
        <form>
            <TextInput name="username" />
            <Button label="Submit"/>
        </form>
    )
}