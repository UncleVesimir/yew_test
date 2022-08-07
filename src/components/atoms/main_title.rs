use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MainTitleProps {
    pub title: String,
}

#[function_component(MainTitle)]
pub fn main_title(props: &MainTitleProps) -> Html {
    html!(
      <h1>{&props.title} </h1>
    )
}
