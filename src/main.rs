
use c_calls::c_calls::{r_log, r_pow};

use yew::prelude::*;


#[function_component(App)]
fn app() -> Html {
    let x = r_pow(2.0, 2.0);
    html! {
        <h1>{ x }</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}