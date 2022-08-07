use gloo::console::log;
use serde::{Serialize, Deserialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct MyObj {
    username: String,
    fav_lang: String
}

#[function_component(App)]
pub fn app () -> Html {
    let name = "James";
    let my_obj = MyObj {
        username: name.to_string(),
        fav_lang: "Rust".to_string()
    };
    let mut out_obj = String::new();


    if let Ok(obj) = serde_json::to_string_pretty(&my_obj) {
        out_obj = obj;
    }


    log!("my name is ", out_obj);

    let class = "my_title";
    html! (
        <>
        <h1 class="title">{"Hello, World!"}</h1>
       if class == "my_titles" { <p>{"Hi there!"}</p> }
       else { <p> {"This shit is bussin!"} </p> }
        </>
    )
}