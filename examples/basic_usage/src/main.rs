use yew::prelude::{html, Component, Context, Html};
use yew_feather::camera::Camera;

struct Model {}

impl Component for Model
{
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self
    {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html
    {
        html! {
            <>
                <span>{"Hello, world!"}</span>
                <Camera />
            </>
        }
    }
}

pub fn main()
{
    yew::start_app::<Model>();
}
