## yew-feather

[![crates.io version](https://img.shields.io/crates/v/yew-feather.svg?style=flat-square)](https://www.npmjs.com/package/react-feather)
[![crates.io downloads](https://img.shields.io/crates/d/yew-feather.svg?style=flat-square)](https://www.npmjs.com/package/react-feather)

#### What is yew-feather?

yew-feather is a collection of simply beautiful open source icons for Yew. Each icon is designed on a 24x24 grid with an emphasis on simplicity, consistency and readability.

#### Based on Feather Icons `v4.28.0`

https://feathericons.com/

### Usage

```rust
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::{html, App, Component, ComponentLink, Html, ShouldRender};
use yew_feather::camera::Camera;

struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <span>Hello, world!</span>
                <Camera />
            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
```

Icons can be configured with inline props:

```rust
<Camera size=48 />
```
