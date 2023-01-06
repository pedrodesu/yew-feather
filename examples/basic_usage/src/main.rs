use yew::{function_component, html, Html};
use yew_feather::Camera;

#[function_component(App)]
fn app() -> Html {
    html! { <Camera /> }
}

fn main() {
    yew::start_app::<App>();
}
