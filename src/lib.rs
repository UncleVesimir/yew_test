use gloo::console::log;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

mod components;

use components::atoms::main_title::{MainTitle, Color};
use components::molecules::custom_form::CustomForm;

const STYLE_FILE: &str = include_str!("main.css");

#[styled_component(App)]
pub fn app() -> Html {
    let tasks = vec!["record vid", "grocery shopping", "pet cat"];
    let stylesheet = Style::new(STYLE_FILE).unwrap();
    let class = "my_title";

    let main_title_load = Callback::from(|message: String| log!(message));

    html! (
        <div class={stylesheet}>
            <MainTitle title="New Props Header!" color={Color::Normal} on_load={main_title_load}/>
            <CustomForm/>
            <h1>{"Hello, World!"}</h1>
            if class == "my_titles" {
                <p>{"Hi there!"}</p>
                }
            else {
                <p> {"This shit is bussin!"} </p>
                }
            <ul>
                {list_to_html(tasks)}
            </ul>
        </div>
    )
}

fn list_to_html(list: Vec<&str>) -> Html {
    list.into_iter()
        .map(|list_item| html!(<li> {list_item} </li>))
        .collect::<Html>()
}
