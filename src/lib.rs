use yew::{function_component, html, AttrValue, Classes, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct IconProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(AttrValue::from("24"))]
    pub size: AttrValue,
    #[prop_or(AttrValue::from("none"))]
    pub fill: AttrValue,
    #[prop_or(AttrValue::from("currentColor"))]
    pub color: AttrValue,
    #[prop_or(AttrValue::from("2"))]
    pub stroke_width: AttrValue,
    #[prop_or(AttrValue::from("round"))]
    pub stroke_linecap: AttrValue,
    #[prop_or(AttrValue::from("round"))]
    pub stroke_linejoin: AttrValue,
}

#[function_component(CreditCard)]
pub fn r#credit_card(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="1" y="4" width="22" height="16" rx="2" ry="2"></rect><line x1="1" y1="10" x2="23" y2="10"></line>
        </svg>
    }
}


#[function_component(ArrowDownRight)]
pub fn r#arrow_down_right(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="7" y1="7" x2="17" y2="17"></line><polyline points="17 7 17 17 7 17"></polyline>
        </svg>
    }
}


#[function_component(BarChart)]
pub fn r#bar_chart(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="12" y1="20" x2="12" y2="10"></line><line x1="18" y1="20" x2="18" y2="4"></line><line x1="6" y1="20" x2="6" y2="16"></line>
        </svg>
    }
}


#[function_component(DivideSquare)]
pub fn r#divide_square(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect><line x1="8" y1="12" x2="16" y2="12"></line><line x1="12" y1="16" x2="12" y2="16"></line><line x1="12" y1="8" x2="12" y2="8"></line>
        </svg>
    }
}


#[function_component(ShoppingCart)]
pub fn r#shopping_cart(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="9" cy="21" r="1"></circle><circle cx="20" cy="21" r="1"></circle><path d="M1 1h4l2.68 13.39a2 2 0 0 0 2 1.61h9.72a2 2 0 0 0 2-1.61L23 6H6"></path>
        </svg>
    }
}


#[function_component(Edit)]
pub fn r#edit(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path>
        </svg>
    }
}


#[function_component(FolderPlus)]
pub fn r#folder_plus(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path><line x1="12" y1="11" x2="12" y2="17"></line><line x1="9" y1="14" x2="15" y2="14"></line>
        </svg>
    }
}


#[function_component(LogIn)]
pub fn r#log_in(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"></path><polyline points="10 17 15 12 10 7"></polyline><line x1="15" y1="12" x2="3" y2="12"></line>
        </svg>
    }
}


#[function_component(Code)]
pub fn r#code(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="16 18 22 12 16 6"></polyline><polyline points="8 6 2 12 8 18"></polyline>
        </svg>
    }
}


#[function_component(Send)]
pub fn r#send(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="22" y1="2" x2="11" y2="13"></line><polygon points="22 2 15 22 11 13 2 9 22 2"></polygon>
        </svg>
    }
}


#[function_component(Smile)]
pub fn r#smile(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle><path d="M8 14s1.5 2 4 2 4-2 4-2"></path><line x1="9" y1="9" x2="9.01" y2="9"></line><line x1="15" y1="9" x2="15.01" y2="9"></line>
        </svg>
    }
}


#[function_component(Tool)]
pub fn r#tool(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"></path>
        </svg>
    }
}


#[function_component(ToggleLeft)]
pub fn r#toggle_left(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="1" y="5" width="22" height="14" rx="7" ry="7"></rect><circle cx="8" cy="12" r="3"></circle>
        </svg>
    }
}


#[function_component(Trash)]
pub fn r#trash(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
        </svg>
    }
}


#[function_component(XSquare)]
pub fn r#x_square(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect><line x1="9" y1="9" x2="15" y2="15"></line><line x1="15" y1="9" x2="9" y2="15"></line>
        </svg>
    }
}


#[function_component(FastForward)]
pub fn r#fast_forward(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polygon points="13 19 22 12 13 5 13 19"></polygon><polygon points="2 19 11 12 2 5 2 19"></polygon>
        </svg>
    }
}


#[function_component(RotateCcw)]
pub fn r#rotate_ccw(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="1 4 1 10 7 10"></polyline><path d="M3.51 15a9 9 0 1 0 2.13-9.36L1 10"></path>
        </svg>
    }
}


#[function_component(Wifi)]
pub fn r#wifi(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M5 12.55a11 11 0 0 1 14.08 0"></path><path d="M1.42 9a16 16 0 0 1 21.16 0"></path><path d="M8.53 16.11a6 6 0 0 1 6.95 0"></path><line x1="12" y1="20" x2="12.01" y2="20"></line>
        </svg>
    }
}


#[function_component(Dribbble)]
pub fn r#dribbble(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle><path d="M8.56 2.75c4.37 6.03 6.02 9.42 8.03 17.72m2.54-15.38c-3.72 4.35-8.94 5.66-16.88 5.85m19.5 1.9c-3.5-.93-6.63-.82-8.94 0-2.58.92-5.01 2.86-7.44 6.32"></path>
        </svg>
    }
}


#[function_component(File)]
pub fn r#file(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"></path><polyline points="13 2 13 9 20 9"></polyline>
        </svg>
    }
}


#[function_component(MessageCircle)]
pub fn r#message_circle(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z"></path>
        </svg>
    }
}


#[function_component(Upload)]
pub fn r#upload(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="17 8 12 3 7 8"></polyline><line x1="12" y1="3" x2="12" y2="15"></line>
        </svg>
    }
}


#[function_component(ThumbsDown)]
pub fn r#thumbs_down(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M10 15v4a3 3 0 0 0 3 3l4-9V2H5.72a2 2 0 0 0-2 1.7l-1.38 9a2 2 0 0 0 2 2.3zm7-13h2.67A2.31 2.31 0 0 1 22 4v7a2.31 2.31 0 0 1-2.33 2H17"></path>
        </svg>
    }
}


#[function_component(ChevronsRight)]
pub fn r#chevrons_right(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="13 17 18 12 13 7"></polyline><polyline points="6 17 11 12 6 7"></polyline>
        </svg>
    }
}


#[function_component(Voicemail)]
pub fn r#voicemail(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="5.5" cy="11.5" r="4.5"></circle><circle cx="18.5" cy="11.5" r="4.5"></circle><line x1="5.5" y1="16" x2="18.5" y2="16"></line>
        </svg>
    }
}


#[function_component(Volume1)]
pub fn r#volume_1(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon><path d="M15.54 8.46a5 5 0 0 1 0 7.07"></path>
        </svg>
    }
}


#[function_component(Paperclip)]
pub fn r#paperclip(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48"></path>
        </svg>
    }
}


#[function_component(XCircle)]
pub fn r#x_circle(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle><line x1="15" y1="9" x2="9" y2="15"></line><line x1="9" y1="9" x2="15" y2="15"></line>
        </svg>
    }
}


#[function_component(XOctagon)]
pub fn r#x_octagon(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polygon points="7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2"></polygon><line x1="15" y1="9" x2="9" y2="15"></line><line x1="9" y1="9" x2="15" y2="15"></line>
        </svg>
    }
}


#[function_component(Tv)]
pub fn r#tv(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="2" y="7" width="20" height="15" rx="2" ry="2"></rect><polyline points="17 2 12 7 7 2"></polyline>
        </svg>
    }
}


#[function_component(Share)]
pub fn r#share(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8"></path><polyline points="16 6 12 2 8 6"></polyline><line x1="12" y1="2" x2="12" y2="15"></line>
        </svg>
    }
}


#[function_component(Smartphone)]
pub fn r#smartphone(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="5" y="2" width="14" height="20" rx="2" ry="2"></rect><line x1="12" y1="18" x2="12.01" y2="18"></line>
        </svg>
    }
}


#[function_component(Printer)]
pub fn r#printer(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="6 9 6 2 18 2 18 9"></polyline><path d="M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2"></path><rect x="6" y="14" width="12" height="8"></rect>
        </svg>
    }
}


#[function_component(DollarSign)]
pub fn r#dollar_sign(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="12" y1="1" x2="12" y2="23"></line><path d="M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"></path>
        </svg>
    }
}


#[function_component(ThumbsUp)]
pub fn r#thumbs_up(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M14 9V5a3 3 0 0 0-3-3l-4 9v11h11.28a2 2 0 0 0 2-1.7l1.38-9a2 2 0 0 0-2-2.3zM7 22H4a2 2 0 0 1-2-2v-7a2 2 0 0 1 2-2h3"></path>
        </svg>
    }
}


#[function_component(Power)]
pub fn r#power(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M18.36 6.64a9 9 0 1 1-12.73 0"></path><line x1="12" y1="2" x2="12" y2="12"></line>
        </svg>
    }
}


#[function_component(Minus)]
pub fn r#minus(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="5" y1="12" x2="19" y2="12"></line>
        </svg>
    }
}


#[function_component(HelpCircle)]
pub fn r#help_circle(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle><path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"></path><line x1="12" y1="17" x2="12.01" y2="17"></line>
        </svg>
    }
}


#[function_component(Aperture)]
pub fn r#aperture(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle><line x1="14.31" y1="8" x2="20.05" y2="17.94"></line><line x1="9.69" y1="8" x2="21.17" y2="8"></line><line x1="7.38" y1="12" x2="13.12" y2="2.06"></line><line x1="9.69" y1="16" x2="3.95" y2="6.06"></line><line x1="14.31" y1="16" x2="2.83" y2="16"></line><line x1="16.62" y1="12" x2="10.88" y2="21.94"></line>
        </svg>
    }
}


#[function_component(BatteryCharging)]
pub fn r#battery_charging(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M5 18H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h3.19M15 6h2a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-3.19"></path><line x1="23" y1="13" x2="23" y2="11"></line><polyline points="11 6 7 12 13 12 9 18"></polyline>
        </svg>
    }
}


#[function_component(Check)]
pub fn r#check(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="20 6 9 17 4 12"></polyline>
        </svg>
    }
}


#[function_component(EyeOff)]
pub fn r#eye_off(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"></path><line x1="1" y1="1" x2="23" y2="23"></line>
        </svg>
    }
}


#[function_component(Key)]
pub fn r#key(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M21 2l-2 2m-7.61 7.61a5.5 5.5 0 1 1-7.778 7.778 5.5 5.5 0 0 1 7.777-7.777zm0 0L15.5 7.5m0 0l3 3L22 7l-3-3m-3.5 3.5L19 4"></path>
        </svg>
    }
}


#[function_component(CloudSnow)]
pub fn r#cloud_snow(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M20 17.58A5 5 0 0 0 18 8h-1.26A8 8 0 1 0 4 16.25"></path><line x1="8" y1="16" x2="8.01" y2="16"></line><line x1="8" y1="20" x2="8.01" y2="20"></line><line x1="12" y1="18" x2="12.01" y2="18"></line><line x1="12" y1="22" x2="12.01" y2="22"></line><line x1="16" y1="16" x2="16.01" y2="16"></line><line x1="16" y1="20" x2="16.01" y2="20"></line>
        </svg>
    }
}


#[function_component(Sun)]
pub fn r#sun(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="5"></circle><line x1="12" y1="1" x2="12" y2="3"></line><line x1="12" y1="21" x2="12" y2="23"></line><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line><line x1="1" y1="12" x2="3" y2="12"></line><line x1="21" y1="12" x2="23" y2="12"></line><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
        </svg>
    }
}


#[function_component(Users)]
pub fn r#users(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path><circle cx="9" cy="7" r="4"></circle><path d="M23 21v-2a4 4 0 0 0-3-3.87"></path><path d="M16 3.13a4 4 0 0 1 0 7.75"></path>
        </svg>
    }
}


#[function_component(Coffee)]
pub fn r#coffee(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M18 8h1a4 4 0 0 1 0 8h-1"></path><path d="M2 8h16v9a4 4 0 0 1-4 4H6a4 4 0 0 1-4-4V8z"></path><line x1="6" y1="1" x2="6" y2="4"></line><line x1="10" y1="1" x2="10" y2="4"></line><line x1="14" y1="1" x2="14" y2="4"></line>
        </svg>
    }
}


#[function_component(UserX)]
pub fn r#user_x(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path><circle cx="8.5" cy="7" r="4"></circle><line x1="18" y1="8" x2="23" y2="13"></line><line x1="23" y1="8" x2="18" y2="13"></line>
        </svg>
    }
}


#[function_component(Anchor)]
pub fn r#anchor(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="5" r="3"></circle><line x1="12" y1="22" x2="12" y2="8"></line><path d="M5 12H2a10 10 0 0 0 20 0h-3"></path>
        </svg>
    }
}


#[function_component(Filter)]
pub fn r#filter(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polygon points="22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3"></polygon>
        </svg>
    }
}


#[function_component(Clock)]
pub fn r#clock(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle><polyline points="12 6 12 12 16 14"></polyline>
        </svg>
    }
}


#[function_component(ArrowRightCircle)]
pub fn r#arrow_right_circle(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle><polyline points="12 16 16 12 12 8"></polyline><line x1="8" y1="12" x2="16" y2="12"></line>
        </svg>
    }
}


#[function_component(Mail)]
pub fn r#mail(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"></path><polyline points="22,6 12,13 2,6"></polyline>
        </svg>
    }
}


#[function_component(PhoneForwarded)]
pub fn r#phone_forwarded(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="19 1 23 5 19 9"></polyline><line x1="15" y1="5" x2="23" y2="5"></line><path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path>
        </svg>
    }
}


#[function_component(Truck)]
pub fn r#truck(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="1" y="3" width="15" height="13"></rect><polygon points="16 8 20 8 23 11 23 16 16 16 16 8"></polygon><circle cx="5.5" cy="18.5" r="2.5"></circle><circle cx="18.5" cy="18.5" r="2.5"></circle>
        </svg>
    }
}


#[function_component(AlignJustify)]
pub fn r#align_justify(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="21" y1="10" x2="3" y2="10"></line><line x1="21" y1="6" x2="3" y2="6"></line><line x1="21" y1="14" x2="3" y2="14"></line><line x1="21" y1="18" x2="3" y2="18"></line>
        </svg>
    }
}


#[function_component(CornerDownRight)]
pub fn r#corner_down_right(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="15 10 20 15 15 20"></polyline><path d="M4 4v7a4 4 0 0 0 4 4h12"></path>
        </svg>
    }
}


#[function_component(FolderMinus)]
pub fn r#folder_minus(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path><line x1="9" y1="14" x2="15" y2="14"></line>
        </svg>
    }
}


#[function_component(DownloadCloud)]
pub fn r#download_cloud(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="8 17 12 21 16 17"></polyline><line x1="12" y1="12" x2="12" y2="21"></line><path d="M20.88 18.09A5 5 0 0 0 18 9h-1.26A8 8 0 1 0 3 16.29"></path>
        </svg>
    }
}


#[function_component(Target)]
pub fn r#target(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle><circle cx="12" cy="12" r="6"></circle><circle cx="12" cy="12" r="2"></circle>
        </svg>
    }
}


#[function_component(Phone)]
pub fn r#phone(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path>
        </svg>
    }
}


#[function_component(PlayCircle)]
pub fn r#play_circle(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle><polygon points="10 8 16 12 10 16 10 8"></polygon>
        </svg>
    }
}


#[function_component(AlertCircle)]
pub fn r#alert_circle(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle><line x1="12" y1="8" x2="12" y2="12"></line><line x1="12" y1="16" x2="12.01" y2="16"></line>
        </svg>
    }
}


#[function_component(Activity)]
pub fn r#activity(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="22 12 18 12 15 21 9 3 6 12 2 12"></polyline>
        </svg>
    }
}


#[function_component(Moon)]
pub fn r#moon(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
        </svg>
    }
}


#[function_component(Underline)]
pub fn r#underline(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M6 3v7a6 6 0 0 0 6 6 6 6 0 0 0 6-6V3"></path><line x1="4" y1="21" x2="20" y2="21"></line>
        </svg>
    }
}


#[function_component(X)]
pub fn r#x(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line>
        </svg>
    }
}


#[function_component(Gitlab)]
pub fn r#gitlab(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M22.65 14.39L12 22.13 1.35 14.39a.84.84 0 0 1-.3-.94l1.22-3.78 2.44-7.51A.42.42 0 0 1 4.82 2a.43.43 0 0 1 .58 0 .42.42 0 0 1 .11.18l2.44 7.49h8.1l2.44-7.51A.42.42 0 0 1 18.6 2a.43.43 0 0 1 .58 0 .42.42 0 0 1 .11.18l2.44 7.51L23 13.45a.84.84 0 0 1-.35.94z"></path>
        </svg>
    }
}


#[function_component(Minimize)]
pub fn r#minimize(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M8 3v3a2 2 0 0 1-2 2H3m18 0h-3a2 2 0 0 1-2-2V3m0 18v-3a2 2 0 0 1 2-2h3M3 16h3a2 2 0 0 1 2 2v3"></path>
        </svg>
    }
}


#[function_component(Twitter)]
pub fn r#twitter(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M23 3a10.9 10.9 0 0 1-3.14 1.53 4.48 4.48 0 0 0-7.86 3v1A10.66 10.66 0 0 1 3 4s-4 9 5 13a11.64 11.64 0 0 1-7 2c9 5 20 0 20-11.5a4.5 4.5 0 0 0-.08-.83A7.72 7.72 0 0 0 23 3z"></path>
        </svg>
    }
}


#[function_component(Columns)]
pub fn r#columns(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M12 3h7a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-7m0-18H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h7m0-18v18"></path>
        </svg>
    }
}


#[function_component(Feather)]
pub fn r#feather(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M20.24 12.24a6 6 0 0 0-8.49-8.49L5 10.5V19h8.5z"></path><line x1="16" y1="8" x2="2" y2="22"></line><line x1="17.5" y1="15" x2="9" y2="15"></line>
        </svg>
    }
}


#[function_component(Frown)]
pub fn r#frown(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle><path d="M16 16s-1.5-2-4-2-4 2-4 2"></path><line x1="9" y1="9" x2="9.01" y2="9"></line><line x1="15" y1="9" x2="15.01" y2="9"></line>
        </svg>
    }
}


#[function_component(CheckCircle)]
pub fn r#check_circle(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path><polyline points="22 4 12 14.01 9 11.01"></polyline>
        </svg>
    }
}


#[function_component(Video)]
pub fn r#video(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polygon points="23 7 16 12 23 17 23 7"></polygon><rect x="1" y="5" width="15" height="14" rx="2" ry="2"></rect>
        </svg>
    }
}


#[function_component(MapPin)]
pub fn r#map_pin(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z"></path><circle cx="12" cy="10" r="3"></circle>
        </svg>
    }
}


#[function_component(Shield)]
pub fn r#shield(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"></path>
        </svg>
    }
}


#[function_component(Music)]
pub fn r#music(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M9 18V5l12-2v13"></path><circle cx="6" cy="18" r="3"></circle><circle cx="18" cy="16" r="3"></circle>
        </svg>
    }
}


#[function_component(VolumeX)]
pub fn r#volume_x(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon><line x1="23" y1="9" x2="17" y2="15"></line><line x1="17" y1="9" x2="23" y2="15"></line>
        </svg>
    }
}


#[function_component(Cast)]
pub fn r#cast(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M2 16.1A5 5 0 0 1 5.9 20M2 12.05A9 9 0 0 1 9.95 20M2 8V6a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2h-6"></path><line x1="2" y1="20" x2="2.01" y2="20"></line>
        </svg>
    }
}


#[function_component(Database)]
pub fn r#database(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <ellipse cx="12" cy="5" rx="9" ry="3"></ellipse><path d="M21 12c0 1.66-4 3-9 3s-9-1.34-9-3"></path><path d="M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5"></path>
        </svg>
    }
}


#[function_component(Edit3)]
pub fn r#edit_3(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M12 20h9"></path><path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z"></path>
        </svg>
    }
}


#[function_component(MousePointer)]
pub fn r#mouse_pointer(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M3 3l7.07 16.97 2.51-7.39 7.39-2.51L3 3z"></path><path d="M13 13l6 6"></path>
        </svg>
    }
}


#[function_component(Package)]
pub fn r#package(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="16.5" y1="9.4" x2="7.5" y2="4.21"></line><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"></path><polyline points="3.27 6.96 12 12.01 20.73 6.96"></polyline><line x1="12" y1="22.08" x2="12" y2="12"></line>
        </svg>
    }
}


#[function_component(CornerRightUp)]
pub fn r#corner_right_up(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="10 9 15 4 20 9"></polyline><path d="M4 20h7a4 4 0 0 0 4-4V4"></path>
        </svg>
    }
}


#[function_component(Slack)]
pub fn r#slack(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M14.5 10c-.83 0-1.5-.67-1.5-1.5v-5c0-.83.67-1.5 1.5-1.5s1.5.67 1.5 1.5v5c0 .83-.67 1.5-1.5 1.5z"></path><path d="M20.5 10H19V8.5c0-.83.67-1.5 1.5-1.5s1.5.67 1.5 1.5-.67 1.5-1.5 1.5z"></path><path d="M9.5 14c.83 0 1.5.67 1.5 1.5v5c0 .83-.67 1.5-1.5 1.5S8 21.33 8 20.5v-5c0-.83.67-1.5 1.5-1.5z"></path><path d="M3.5 14H5v1.5c0 .83-.67 1.5-1.5 1.5S2 16.33 2 15.5 2.67 14 3.5 14z"></path><path d="M14 14.5c0-.83.67-1.5 1.5-1.5h5c.83 0 1.5.67 1.5 1.5s-.67 1.5-1.5 1.5h-5c-.83 0-1.5-.67-1.5-1.5z"></path><path d="M15.5 19H14v1.5c0 .83.67 1.5 1.5 1.5s1.5-.67 1.5-1.5-.67-1.5-1.5-1.5z"></path><path d="M10 9.5C10 8.67 9.33 8 8.5 8h-5C2.67 8 2 8.67 2 9.5S2.67 11 3.5 11h5c.83 0 1.5-.67 1.5-1.5z"></path><path d="M8.5 5H10V3.5C10 2.67 9.33 2 8.5 2S7 2.67 7 3.5 7.67 5 8.5 5z"></path>
        </svg>
    }
}


#[function_component(GitBranch)]
pub fn r#git_branch(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="6" y1="3" x2="6" y2="15"></line><circle cx="18" cy="6" r="3"></circle><circle cx="6" cy="18" r="3"></circle><path d="M18 9a9 9 0 0 1-9 9"></path>
        </svg>
    }
}


#[function_component(RotateCw)]
pub fn r#rotate_cw(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="23 4 23 10 17 10"></polyline><path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10"></path>
        </svg>
    }
}


#[function_component(Loader)]
pub fn r#loader(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="12" y1="2" x2="12" y2="6"></line><line x1="12" y1="18" x2="12" y2="22"></line><line x1="4.93" y1="4.93" x2="7.76" y2="7.76"></line><line x1="16.24" y1="16.24" x2="19.07" y2="19.07"></line><line x1="2" y1="12" x2="6" y2="12"></line><line x1="18" y1="12" x2="22" y2="12"></line><line x1="4.93" y1="19.07" x2="7.76" y2="16.24"></line><line x1="16.24" y1="7.76" x2="19.07" y2="4.93"></line>
        </svg>
    }
}


#[function_component(Film)]
pub fn r#film(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="2" y="2" width="20" height="20" rx="2.18" ry="2.18"></rect><line x1="7" y1="2" x2="7" y2="22"></line><line x1="17" y1="2" x2="17" y2="22"></line><line x1="2" y1="12" x2="22" y2="12"></line><line x1="2" y1="7" x2="7" y2="7"></line><line x1="2" y1="17" x2="7" y2="17"></line><line x1="17" y1="17" x2="22" y2="17"></line><line x1="17" y1="7" x2="22" y2="7"></line>
        </svg>
    }
}


#[function_component(TrendingUp)]
pub fn r#trending_up(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="23 6 13.5 15.5 8.5 10.5 1 18"></polyline><polyline points="17 6 23 6 23 12"></polyline>
        </svg>
    }
}


#[function_component(Layout)]
pub fn r#layout(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect><line x1="3" y1="9" x2="21" y2="9"></line><line x1="9" y1="21" x2="9" y2="9"></line>
        </svg>
    }
}


#[function_component(Trash2)]
pub fn r#trash_2(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path><line x1="10" y1="11" x2="10" y2="17"></line><line x1="14" y1="11" x2="14" y2="17"></line>
        </svg>
    }
}


#[function_component(Navigation)]
pub fn r#navigation(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polygon points="3 11 22 2 13 21 11 13 3 11"></polygon>
        </svg>
    }
}


#[function_component(Flag)]
pub fn r#flag(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M4 15s1-1 4-1 5 2 8 2 4-1 4-1V3s-1 1-4 1-5-2-8-2-4 1-4 1z"></path><line x1="4" y1="22" x2="4" y2="15"></line>
        </svg>
    }
}


#[function_component(BarChart2)]
pub fn r#bar_chart_2(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="18" y1="20" x2="18" y2="10"></line><line x1="12" y1="20" x2="12" y2="4"></line><line x1="6" y1="20" x2="6" y2="14"></line>
        </svg>
    }
}


#[function_component(Mic)]
pub fn r#mic(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z"></path><path d="M19 10v2a7 7 0 0 1-14 0v-2"></path><line x1="12" y1="19" x2="12" y2="23"></line><line x1="8" y1="23" x2="16" y2="23"></line>
        </svg>
    }
}


#[function_component(MoreVertical)]
pub fn r#more_vertical(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="1"></circle><circle cx="12" cy="5" r="1"></circle><circle cx="12" cy="19" r="1"></circle>
        </svg>
    }
}


#[function_component(Layers)]
pub fn r#layers(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polygon points="12 2 2 7 12 12 22 7 12 2"></polygon><polyline points="2 17 12 22 22 17"></polyline><polyline points="2 12 12 17 22 12"></polyline>
        </svg>
    }
}


#[function_component(Image)]
pub fn r#image(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect><circle cx="8.5" cy="8.5" r="1.5"></circle><polyline points="21 15 16 10 5 21"></polyline>
        </svg>
    }
}


#[function_component(CheckSquare)]
pub fn r#check_square(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="9 11 12 14 22 4"></polyline><path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11"></path>
        </svg>
    }
}


#[function_component(Clipboard)]
pub fn r#clipboard(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"></path><rect x="8" y="2" width="8" height="4" rx="1" ry="1"></rect>
        </svg>
    }
}


#[function_component(Lock)]
pub fn r#lock(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect><path d="M7 11V7a5 5 0 0 1 10 0v4"></path>
        </svg>
    }
}


#[function_component(Percent)]
pub fn r#percent(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="19" y1="5" x2="5" y2="19"></line><circle cx="6.5" cy="6.5" r="2.5"></circle><circle cx="17.5" cy="17.5" r="2.5"></circle>
        </svg>
    }
}


#[function_component(ArrowRight)]
pub fn r#arrow_right(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="5" y1="12" x2="19" y2="12"></line><polyline points="12 5 19 12 12 19"></polyline>
        </svg>
    }
}


#[function_component(Bold)]
pub fn r#bold(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M6 4h8a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z"></path><path d="M6 12h9a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z"></path>
        </svg>
    }
}


#[function_component(Headphones)]
pub fn r#headphones(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M3 18v-6a9 9 0 0 1 18 0v6"></path><path d="M21 19a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3zM3 19a2 2 0 0 0 2 2h1a2 2 0 0 0 2-2v-3a2 2 0 0 0-2-2H3z"></path>
        </svg>
    }
}


#[function_component(Umbrella)]
pub fn r#umbrella(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M23 12a11.05 11.05 0 0 0-22 0zm-5 7a3 3 0 0 1-6 0v-7"></path>
        </svg>
    }
}


#[function_component(Download)]
pub fn r#download(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="7 10 12 15 17 10"></polyline><line x1="12" y1="15" x2="12" y2="3"></line>
        </svg>
    }
}


#[function_component(LifeBuoy)]
pub fn r#life_buoy(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle><circle cx="12" cy="12" r="4"></circle><line x1="4.93" y1="4.93" x2="9.17" y2="9.17"></line><line x1="14.83" y1="14.83" x2="19.07" y2="19.07"></line><line x1="14.83" y1="9.17" x2="19.07" y2="4.93"></line><line x1="14.83" y1="9.17" x2="18.36" y2="5.64"></line><line x1="4.93" y1="19.07" x2="9.17" y2="14.83"></line>
        </svg>
    }
}


#[function_component(Wind)]
pub fn r#wind(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M9.59 4.59A2 2 0 1 1 11 8H2m10.59 11.41A2 2 0 1 0 14 16H2m15.73-8.27A2.5 2.5 0 1 1 19.5 12H2"></path>
        </svg>
    }
}


#[function_component(Codesandbox)]
pub fn r#codesandbox(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"></path><polyline points="7.5 4.21 12 6.81 16.5 4.21"></polyline><polyline points="7.5 19.79 7.5 14.6 3 12"></polyline><polyline points="21 12 16.5 14.6 16.5 19.79"></polyline><polyline points="3.27 6.96 12 12.01 20.73 6.96"></polyline><line x1="12" y1="22.08" x2="12" y2="12"></line>
        </svg>
    }
}


#[function_component(Scissors)]
pub fn r#scissors(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="6" cy="6" r="3"></circle><circle cx="6" cy="18" r="3"></circle><line x1="20" y1="4" x2="8.12" y2="15.88"></line><line x1="14.47" y1="14.48" x2="20" y2="20"></line><line x1="8.12" y1="8.12" x2="12" y2="12"></line>
        </svg>
    }
}


#[function_component(Calendar)]
pub fn r#calendar(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="3" y="4" width="18" height="18" rx="2" ry="2"></rect><line x1="16" y1="2" x2="16" y2="6"></line><line x1="8" y1="2" x2="8" y2="6"></line><line x1="3" y1="10" x2="21" y2="10"></line>
        </svg>
    }
}


#[function_component(Book)]
pub fn r#book(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"></path><path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"></path>
        </svg>
    }
}


#[function_component(Battery)]
pub fn r#battery(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="1" y="6" width="18" height="12" rx="2" ry="2"></rect><line x1="23" y1="13" x2="23" y2="11"></line>
        </svg>
    }
}


#[function_component(Plus)]
pub fn r#plus(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line>
        </svg>
    }
}


#[function_component(DivideCircle)]
pub fn r#divide_circle(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="8" y1="12" x2="16" y2="12"></line><line x1="12" y1="16" x2="12" y2="16"></line><line x1="12" y1="8" x2="12" y2="8"></line><circle cx="12" cy="12" r="10"></circle>
        </svg>
    }
}


#[function_component(ArrowDownCircle)]
pub fn r#arrow_down_circle(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle><polyline points="8 12 12 16 16 12"></polyline><line x1="12" y1="8" x2="12" y2="16"></line>
        </svg>
    }
}


#[function_component(Minimize2)]
pub fn r#minimize_2(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="4 14 10 14 10 20"></polyline><polyline points="20 10 14 10 14 4"></polyline><line x1="14" y1="10" x2="21" y2="3"></line><line x1="3" y1="21" x2="10" y2="14"></line>
        </svg>
    }
}


#[function_component(Airplay)]
pub fn r#airplay(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M5 17H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-1"></path><polygon points="12 15 17 21 7 21 12 15"></polygon>
        </svg>
    }
}


#[function_component(CloudLightning)]
pub fn r#cloud_lightning(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M19 16.9A5 5 0 0 0 18 7h-1.26a8 8 0 1 0-11.62 9"></path><polyline points="13 11 9 17 15 17 11 23"></polyline>
        </svg>
    }
}


#[function_component(GitPullRequest)]
pub fn r#git_pull_request(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="18" cy="18" r="3"></circle><circle cx="6" cy="6" r="3"></circle><path d="M13 6h3a2 2 0 0 1 2 2v7"></path><line x1="6" y1="9" x2="6" y2="21"></line>
        </svg>
    }
}


#[function_component(ChevronDown)]
pub fn r#chevron_down(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="6 9 12 15 18 9"></polyline>
        </svg>
    }
}


#[function_component(GitCommit)]
pub fn r#git_commit(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="4"></circle><line x1="1.05" y1="12" x2="7" y2="12"></line><line x1="17.01" y1="12" x2="22.96" y2="12"></line>
        </svg>
    }
}


#[function_component(Inbox)]
pub fn r#inbox(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="22 12 16 12 14 15 10 15 8 12 2 12"></polyline><path d="M5.45 5.11L2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"></path>
        </svg>
    }
}


#[function_component(MicOff)]
pub fn r#mic_off(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="1" y1="1" x2="23" y2="23"></line><path d="M9 9v3a3 3 0 0 0 5.12 2.12M15 9.34V4a3 3 0 0 0-5.94-.6"></path><path d="M17 16.95A7 7 0 0 1 5 12v-2m14 0v2a7 7 0 0 1-.11 1.23"></path><line x1="12" y1="19" x2="12" y2="23"></line><line x1="8" y1="23" x2="16" y2="23"></line>
        </svg>
    }
}


#[function_component(Rss)]
pub fn r#rss(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M4 11a9 9 0 0 1 9 9"></path><path d="M4 4a16 16 0 0 1 16 16"></path><circle cx="5" cy="19" r="1"></circle>
        </svg>
    }
}


#[function_component(WifiOff)]
pub fn r#wifi_off(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="1" y1="1" x2="23" y2="23"></line><path d="M16.72 11.06A10.94 10.94 0 0 1 19 12.55"></path><path d="M5 12.55a10.94 10.94 0 0 1 5.17-2.39"></path><path d="M10.71 5.05A16 16 0 0 1 22.58 9"></path><path d="M1.42 9a15.91 15.91 0 0 1 4.7-2.88"></path><path d="M8.53 16.11a6 6 0 0 1 6.95 0"></path><line x1="12" y1="20" x2="12.01" y2="20"></line>
        </svg>
    }
}


#[function_component(PenTool)]
pub fn r#pen_tool(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M12 19l7-7 3 3-7 7-3-3z"></path><path d="M18 13l-1.5-7.5L2 2l3.5 14.5L13 18l5-5z"></path><path d="M2 2l7.586 7.586"></path><circle cx="11" cy="11" r="2"></circle>
        </svg>
    }
}


#[function_component(AtSign)]
pub fn r#at_sign(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="4"></circle><path d="M16 8v5a3 3 0 0 0 6 0v-1a10 10 0 1 0-3.92 7.94"></path>
        </svg>
    }
}


#[function_component(HardDrive)]
pub fn r#hard_drive(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="22" y1="12" x2="2" y2="12"></line><path d="M5.45 5.11L2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"></path><line x1="6" y1="16" x2="6.01" y2="16"></line><line x1="10" y1="16" x2="10.01" y2="16"></line>
        </svg>
    }
}


#[function_component(Table)]
pub fn r#table(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M9 3H5a2 2 0 0 0-2 2v4m6-6h10a2 2 0 0 1 2 2v4M9 3v18m0 0h10a2 2 0 0 0 2-2V9M9 21H5a2 2 0 0 1-2-2V9m0 0h18"></path>
        </svg>
    }
}


#[function_component(PhoneOutgoing)]
pub fn r#phone_outgoing(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="23 7 23 1 17 1"></polyline><line x1="16" y1="8" x2="23" y2="1"></line><path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path>
        </svg>
    }
}


#[function_component(Divide)]
pub fn r#divide(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="6" r="2"></circle><line x1="5" y1="12" x2="19" y2="12"></line><circle cx="12" cy="18" r="2"></circle>
        </svg>
    }
}


#[function_component(Octagon)]
pub fn r#octagon(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polygon points="7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2"></polygon>
        </svg>
    }
}


#[function_component(Command)]
pub fn r#command(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M18 3a3 3 0 0 0-3 3v12a3 3 0 0 0 3 3 3 3 0 0 0 3-3 3 3 0 0 0-3-3H6a3 3 0 0 0-3 3 3 3 0 0 0 3 3 3 3 0 0 0 3-3V6a3 3 0 0 0-3-3 3 3 0 0 0-3 3 3 3 0 0 0 3 3h12a3 3 0 0 0 3-3 3 3 0 0 0-3-3z"></path>
        </svg>
    }
}


#[function_component(Triangle)]
pub fn r#triangle(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"></path>
        </svg>
    }
}


#[function_component(Sunset)]
pub fn r#sunset(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M17 18a5 5 0 0 0-10 0"></path><line x1="12" y1="9" x2="12" y2="2"></line><line x1="4.22" y1="10.22" x2="5.64" y2="11.64"></line><line x1="1" y1="18" x2="3" y2="18"></line><line x1="21" y1="18" x2="23" y2="18"></line><line x1="18.36" y1="11.64" x2="19.78" y2="10.22"></line><line x1="23" y1="22" x2="1" y2="22"></line><polyline points="16 5 12 9 8 5"></polyline>
        </svg>
    }
}


#[function_component(Speaker)]
pub fn r#speaker(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="4" y="2" width="16" height="20" rx="2" ry="2"></rect><circle cx="12" cy="14" r="4"></circle><line x1="12" y1="6" x2="12.01" y2="6"></line>
        </svg>
    }
}


#[function_component(UserPlus)]
pub fn r#user_plus(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path><circle cx="8.5" cy="7" r="4"></circle><line x1="20" y1="8" x2="20" y2="14"></line><line x1="23" y1="11" x2="17" y2="11"></line>
        </svg>
    }
}


#[function_component(ArrowLeftCircle)]
pub fn r#arrow_left_circle(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle><polyline points="12 8 8 12 12 16"></polyline><line x1="16" y1="12" x2="8" y2="12"></line>
        </svg>
    }
}


#[function_component(Eye)]
pub fn r#eye(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path><circle cx="12" cy="12" r="3"></circle>
        </svg>
    }
}


#[function_component(LogOut)]
pub fn r#log_out(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path><polyline points="16 17 21 12 16 7"></polyline><line x1="21" y1="12" x2="9" y2="12"></line>
        </svg>
    }
}


#[function_component(Figma)]
pub fn r#figma(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M5 5.5A3.5 3.5 0 0 1 8.5 2H12v7H8.5A3.5 3.5 0 0 1 5 5.5z"></path><path d="M12 2h3.5a3.5 3.5 0 1 1 0 7H12V2z"></path><path d="M12 12.5a3.5 3.5 0 1 1 7 0 3.5 3.5 0 1 1-7 0z"></path><path d="M5 19.5A3.5 3.5 0 0 1 8.5 16H12v3.5a3.5 3.5 0 1 1-7 0z"></path><path d="M5 12.5A3.5 3.5 0 0 1 8.5 9H12v7H8.5A3.5 3.5 0 0 1 5 12.5z"></path>
        </svg>
    }
}


#[function_component(ArrowDown)]
pub fn r#arrow_down(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="12" y1="5" x2="12" y2="19"></line><polyline points="19 12 12 19 5 12"></polyline>
        </svg>
    }
}


#[function_component(Meh)]
pub fn r#meh(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle><line x1="8" y1="15" x2="16" y2="15"></line><line x1="9" y1="9" x2="9.01" y2="9"></line><line x1="15" y1="9" x2="15.01" y2="9"></line>
        </svg>
    }
}


#[function_component(Box)]
pub fn r#box(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"></path><polyline points="3.27 6.96 12 12.01 20.73 6.96"></polyline><line x1="12" y1="22.08" x2="12" y2="12"></line>
        </svg>
    }
}


#[function_component(SkipBack)]
pub fn r#skip_back(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polygon points="19 20 9 12 19 4 19 20"></polygon><line x1="5" y1="19" x2="5" y2="5"></line>
        </svg>
    }
}


#[function_component(ChevronsLeft)]
pub fn r#chevrons_left(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="11 17 6 12 11 7"></polyline><polyline points="18 17 13 12 18 7"></polyline>
        </svg>
    }
}


#[function_component(Compass)]
pub fn r#compass(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle><polygon points="16.24 7.76 14.12 14.12 7.76 16.24 9.88 9.88 16.24 7.76"></polygon>
        </svg>
    }
}


#[function_component(ChevronLeft)]
pub fn r#chevron_left(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="15 18 9 12 15 6"></polyline>
        </svg>
    }
}


#[function_component(List)]
pub fn r#list(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="8" y1="6" x2="21" y2="6"></line><line x1="8" y1="12" x2="21" y2="12"></line><line x1="8" y1="18" x2="21" y2="18"></line><line x1="3" y1="6" x2="3.01" y2="6"></line><line x1="3" y1="12" x2="3.01" y2="12"></line><line x1="3" y1="18" x2="3.01" y2="18"></line>
        </svg>
    }
}


#[function_component(CornerUpLeft)]
pub fn r#corner_up_left(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="9 14 4 9 9 4"></polyline><path d="M20 20v-7a4 4 0 0 0-4-4H4"></path>
        </svg>
    }
}


#[function_component(StopCircle)]
pub fn r#stop_circle(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle><rect x="9" y="9" width="6" height="6"></rect>
        </svg>
    }
}


#[function_component(Pause)]
pub fn r#pause(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="6" y="4" width="4" height="16"></rect><rect x="14" y="4" width="4" height="16"></rect>
        </svg>
    }
}


#[function_component(Radio)]
pub fn r#radio(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="2"></circle><path d="M16.24 7.76a6 6 0 0 1 0 8.49m-8.48-.01a6 6 0 0 1 0-8.49m11.31-2.82a10 10 0 0 1 0 14.14m-14.14 0a10 10 0 0 1 0-14.14"></path>
        </svg>
    }
}


#[function_component(Search)]
pub fn r#search(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line>
        </svg>
    }
}


#[function_component(Cpu)]
pub fn r#cpu(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="4" y="4" width="16" height="16" rx="2" ry="2"></rect><rect x="9" y="9" width="6" height="6"></rect><line x1="9" y1="1" x2="9" y2="4"></line><line x1="15" y1="1" x2="15" y2="4"></line><line x1="9" y1="20" x2="9" y2="23"></line><line x1="15" y1="20" x2="15" y2="23"></line><line x1="20" y1="9" x2="23" y2="9"></line><line x1="20" y1="14" x2="23" y2="14"></line><line x1="1" y1="9" x2="4" y2="9"></line><line x1="1" y1="14" x2="4" y2="14"></line>
        </svg>
    }
}


#[function_component(ArrowUp)]
pub fn r#arrow_up(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="12" y1="19" x2="12" y2="5"></line><polyline points="5 12 12 5 19 12"></polyline>
        </svg>
    }
}


#[function_component(ArrowDownLeft)]
pub fn r#arrow_down_left(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="17" y1="7" x2="7" y2="17"></line><polyline points="17 17 7 17 7 7"></polyline>
        </svg>
    }
}


#[function_component(Navigation2)]
pub fn r#navigation_2(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polygon points="12 2 19 21 12 17 5 21 12 2"></polygon>
        </svg>
    }
}


#[function_component(Instagram)]
pub fn r#instagram(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="2" y="2" width="20" height="20" rx="5" ry="5"></rect><path d="M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z"></path><line x1="17.5" y1="6.5" x2="17.51" y2="6.5"></line>
        </svg>
    }
}


#[function_component(Volume2)]
pub fn r#volume_2(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon><path d="M19.07 4.93a10 10 0 0 1 0 14.14M15.54 8.46a5 5 0 0 1 0 7.07"></path>
        </svg>
    }
}


#[function_component(Bookmark)]
pub fn r#bookmark(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z"></path>
        </svg>
    }
}


#[function_component(Rewind)]
pub fn r#rewind(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polygon points="11 19 2 12 11 5 11 19"></polygon><polygon points="22 19 13 12 22 5 22 19"></polygon>
        </svg>
    }
}


#[function_component(CameraOff)]
pub fn r#camera_off(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="1" y1="1" x2="23" y2="23"></line><path d="M21 21H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h3m3-3h6l2 3h4a2 2 0 0 1 2 2v9.34m-7.72-2.06a4 4 0 1 1-5.56-5.56"></path>
        </svg>
    }
}


#[function_component(CornerUpRight)]
pub fn r#corner_up_right(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="15 14 20 9 15 4"></polyline><path d="M4 20v-7a4 4 0 0 1 4-4h12"></path>
        </svg>
    }
}


#[function_component(Gift)]
pub fn r#gift(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="20 12 20 22 4 22 4 12"></polyline><rect x="2" y="7" width="20" height="5"></rect><line x1="12" y1="22" x2="12" y2="7"></line><path d="M12 7H7.5a2.5 2.5 0 0 1 0-5C11 2 12 7 12 7z"></path><path d="M12 7h4.5a2.5 2.5 0 0 0 0-5C13 2 12 7 12 7z"></path>
        </svg>
    }
}


#[function_component(UploadCloud)]
pub fn r#upload_cloud(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="16 16 12 12 8 16"></polyline><line x1="12" y1="12" x2="12" y2="21"></line><path d="M20.39 18.39A5 5 0 0 0 18 9h-1.26A8 8 0 1 0 3 16.3"></path><polyline points="16 16 12 12 8 16"></polyline>
        </svg>
    }
}


#[function_component(Framer)]
pub fn r#framer(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M5 16V9h14V2H5l14 14h-7m-7 0l7 7v-7m-7 0h7"></path>
        </svg>
    }
}


#[function_component(Square)]
pub fn r#square(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
        </svg>
    }
}


#[function_component(Heart)]
pub fn r#heart(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"></path>
        </svg>
    }
}


#[function_component(Zap)]
pub fn r#zap(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"></polygon>
        </svg>
    }
}


#[function_component(CornerRightDown)]
pub fn r#corner_right_down(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="10 15 15 20 20 15"></polyline><path d="M4 4h7a4 4 0 0 1 4 4v12"></path>
        </svg>
    }
}


#[function_component(ChevronsDown)]
pub fn r#chevrons_down(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="7 13 12 18 17 13"></polyline><polyline points="7 6 12 11 17 6"></polyline>
        </svg>
    }
}


#[function_component(Camera)]
pub fn r#camera(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z"></path><circle cx="12" cy="13" r="4"></circle>
        </svg>
    }
}


#[function_component(AlignRight)]
pub fn r#align_right(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="21" y1="10" x2="7" y2="10"></line><line x1="21" y1="6" x2="3" y2="6"></line><line x1="21" y1="14" x2="3" y2="14"></line><line x1="21" y1="18" x2="7" y2="18"></line>
        </svg>
    }
}


#[function_component(RefreshCcw)]
pub fn r#refresh_ccw(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="1 4 1 10 7 10"></polyline><polyline points="23 20 23 14 17 14"></polyline><path d="M20.49 9A9 9 0 0 0 5.64 5.64L1 10m22 4l-4.64 4.36A9 9 0 0 1 3.51 15"></path>
        </svg>
    }
}


#[function_component(Italic)]
pub fn r#italic(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="19" y1="4" x2="10" y2="4"></line><line x1="14" y1="20" x2="5" y2="20"></line><line x1="15" y1="4" x2="9" y2="20"></line>
        </svg>
    }
}


#[function_component(Watch)]
pub fn r#watch(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="7"></circle><polyline points="12 9 12 12 13.5 13.5"></polyline><path d="M16.51 17.35l-.35 3.83a2 2 0 0 1-2 1.82H9.83a2 2 0 0 1-2-1.82l-.35-3.83m.01-10.7l.35-3.83A2 2 0 0 1 9.83 1h4.35a2 2 0 0 1 2 1.82l.35 3.83"></path>
        </svg>
    }
}


#[function_component(Archive)]
pub fn r#archive(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="21 8 21 21 3 21 3 8"></polyline><rect x="1" y="3" width="22" height="5"></rect><line x1="10" y1="12" x2="14" y2="12"></line>
        </svg>
    }
}


#[function_component(MinusSquare)]
pub fn r#minus_square(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect><line x1="8" y1="12" x2="16" y2="12"></line>
        </svg>
    }
}


#[function_component(VideoOff)]
pub fn r#video_off(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M16 16v1a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h2m5.66 0H14a2 2 0 0 1 2 2v3.34l1 1L23 7v10"></path><line x1="1" y1="1" x2="23" y2="23"></line>
        </svg>
    }
}


#[function_component(FilePlus)]
pub fn r#file_plus(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path><polyline points="14 2 14 8 20 8"></polyline><line x1="12" y1="18" x2="12" y2="12"></line><line x1="9" y1="15" x2="15" y2="15"></line>
        </svg>
    }
}


#[function_component(MoreHorizontal)]
pub fn r#more_horizontal(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="1"></circle><circle cx="19" cy="12" r="1"></circle><circle cx="5" cy="12" r="1"></circle>
        </svg>
    }
}


#[function_component(Hash)]
pub fn r#hash(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="4" y1="9" x2="20" y2="9"></line><line x1="4" y1="15" x2="20" y2="15"></line><line x1="10" y1="3" x2="8" y2="21"></line><line x1="16" y1="3" x2="14" y2="21"></line>
        </svg>
    }
}


#[function_component(Grid)]
pub fn r#grid(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="3" y="3" width="7" height="7"></rect><rect x="14" y="3" width="7" height="7"></rect><rect x="14" y="14" width="7" height="7"></rect><rect x="3" y="14" width="7" height="7"></rect>
        </svg>
    }
}


#[function_component(Link)]
pub fn r#link(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"></path><path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"></path>
        </svg>
    }
}


#[function_component(PlusSquare)]
pub fn r#plus_square(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect><line x1="12" y1="8" x2="12" y2="16"></line><line x1="8" y1="12" x2="16" y2="12"></line>
        </svg>
    }
}


#[function_component(Share2)]
pub fn r#share_2(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="18" cy="5" r="3"></circle><circle cx="6" cy="12" r="3"></circle><circle cx="18" cy="19" r="3"></circle><line x1="8.59" y1="13.51" x2="15.42" y2="17.49"></line><line x1="15.41" y1="6.51" x2="8.59" y2="10.49"></line>
        </svg>
    }
}


#[function_component(CloudDrizzle)]
pub fn r#cloud_drizzle(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="8" y1="19" x2="8" y2="21"></line><line x1="8" y1="13" x2="8" y2="15"></line><line x1="16" y1="19" x2="16" y2="21"></line><line x1="16" y1="13" x2="16" y2="15"></line><line x1="12" y1="21" x2="12" y2="23"></line><line x1="12" y1="15" x2="12" y2="17"></line><path d="M20 16.58A5 5 0 0 0 18 7h-1.26A8 8 0 1 0 4 15.25"></path>
        </svg>
    }
}


#[function_component(Settings)]
pub fn r#settings(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="3"></circle><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
        </svg>
    }
}


#[function_component(ChevronRight)]
pub fn r#chevron_right(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="9 18 15 12 9 6"></polyline>
        </svg>
    }
}


#[function_component(Sliders)]
pub fn r#sliders(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="4" y1="21" x2="4" y2="14"></line><line x1="4" y1="10" x2="4" y2="3"></line><line x1="12" y1="21" x2="12" y2="12"></line><line x1="12" y1="8" x2="12" y2="3"></line><line x1="20" y1="21" x2="20" y2="16"></line><line x1="20" y1="12" x2="20" y2="3"></line><line x1="1" y1="14" x2="7" y2="14"></line><line x1="9" y1="8" x2="15" y2="8"></line><line x1="17" y1="16" x2="23" y2="16"></line>
        </svg>
    }
}


#[function_component(ChevronsUp)]
pub fn r#chevrons_up(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="17 11 12 6 7 11"></polyline><polyline points="17 18 12 13 7 18"></polyline>
        </svg>
    }
}


#[function_component(Monitor)]
pub fn r#monitor(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="2" y="3" width="20" height="14" rx="2" ry="2"></rect><line x1="8" y1="21" x2="16" y2="21"></line><line x1="12" y1="17" x2="12" y2="21"></line>
        </svg>
    }
}


#[function_component(PieChart)]
pub fn r#pie_chart(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M21.21 15.89A10 10 0 1 1 8 2.83"></path><path d="M22 12A10 10 0 0 0 12 2v10z"></path>
        </svg>
    }
}


#[function_component(ChevronUp)]
pub fn r#chevron_up(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="18 15 12 9 6 15"></polyline>
        </svg>
    }
}


#[function_component(User)]
pub fn r#user(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path><circle cx="12" cy="7" r="4"></circle>
        </svg>
    }
}


#[function_component(Sidebar)]
pub fn r#sidebar(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect><line x1="9" y1="3" x2="9" y2="21"></line>
        </svg>
    }
}


#[function_component(Chrome)]
pub fn r#chrome(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle><circle cx="12" cy="12" r="4"></circle><line x1="21.17" y1="8" x2="12" y2="8"></line><line x1="3.95" y1="6.06" x2="8.54" y2="14"></line><line x1="10.88" y1="21.94" x2="15.46" y2="14"></line>
        </svg>
    }
}


#[function_component(BellOff)]
pub fn r#bell_off(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M13.73 21a2 2 0 0 1-3.46 0"></path><path d="M18.63 13A17.89 17.89 0 0 1 18 8"></path><path d="M6.26 6.26A5.86 5.86 0 0 0 6 8c0 7-3 9-3 9h14"></path><path d="M18 8a6 6 0 0 0-9.33-5"></path><line x1="1" y1="1" x2="23" y2="23"></line>
        </svg>
    }
}


#[function_component(Maximize)]
pub fn r#maximize(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"></path>
        </svg>
    }
}


#[function_component(Thermometer)]
pub fn r#thermometer(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M14 14.76V3.5a2.5 2.5 0 0 0-5 0v11.26a4.5 4.5 0 1 0 5 0z"></path>
        </svg>
    }
}


#[function_component(Delete)]
pub fn r#delete(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M21 4H8l-7 8 7 8h13a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2z"></path><line x1="18" y1="9" x2="12" y2="15"></line><line x1="12" y1="9" x2="18" y2="15"></line>
        </svg>
    }
}


#[function_component(TrendingDown)]
pub fn r#trending_down(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="23 18 13.5 8.5 8.5 13.5 1 6"></polyline><polyline points="17 18 23 18 23 12"></polyline>
        </svg>
    }
}


#[function_component(Star)]
pub fn r#star(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"></polygon>
        </svg>
    }
}


#[function_component(Hexagon)]
pub fn r#hexagon(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"></path>
        </svg>
    }
}


#[function_component(ArrowUpLeft)]
pub fn r#arrow_up_left(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="17" y1="17" x2="7" y2="7"></line><polyline points="7 17 7 7 17 7"></polyline>
        </svg>
    }
}


#[function_component(AlignLeft)]
pub fn r#align_left(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="17" y1="10" x2="3" y2="10"></line><line x1="21" y1="6" x2="3" y2="6"></line><line x1="21" y1="14" x2="3" y2="14"></line><line x1="17" y1="18" x2="3" y2="18"></line>
        </svg>
    }
}


#[function_component(Tag)]
pub fn r#tag(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M20.59 13.41l-7.17 7.17a2 2 0 0 1-2.83 0L2 12V2h10l8.59 8.59a2 2 0 0 1 0 2.82z"></path><line x1="7" y1="7" x2="7.01" y2="7"></line>
        </svg>
    }
}


#[function_component(Folder)]
pub fn r#folder(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path>
        </svg>
    }
}


#[function_component(ZapOff)]
pub fn r#zap_off(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="12.41 6.75 13 2 10.57 4.92"></polyline><polyline points="18.57 12.91 21 10 15.66 10"></polyline><polyline points="8 8 3 14 12 14 11 22 16 16"></polyline><line x1="1" y1="1" x2="23" y2="23"></line>
        </svg>
    }
}


#[function_component(Trello)]
pub fn r#trello(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect><rect x="7" y="7" width="3" height="9"></rect><rect x="14" y="7" width="3" height="5"></rect>
        </svg>
    }
}


#[function_component(AlertTriangle)]
pub fn r#alert_triangle(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"></path><line x1="12" y1="9" x2="12" y2="13"></line><line x1="12" y1="17" x2="12.01" y2="17"></line>
        </svg>
    }
}


#[function_component(Award)]
pub fn r#award(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="8" r="7"></circle><polyline points="8.21 13.89 7 23 12 20 17 23 15.79 13.88"></polyline>
        </svg>
    }
}


#[function_component(Repeat)]
pub fn r#repeat(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="17 1 21 5 17 9"></polyline><path d="M3 11V9a4 4 0 0 1 4-4h14"></path><polyline points="7 23 3 19 7 15"></polyline><path d="M21 13v2a4 4 0 0 1-4 4H3"></path>
        </svg>
    }
}


#[function_component(ZoomOut)]
pub fn r#zoom_out(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line><line x1="8" y1="11" x2="14" y2="11"></line>
        </svg>
    }
}


#[function_component(Tablet)]
pub fn r#tablet(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="4" y="2" width="16" height="20" rx="2" ry="2"></rect><line x1="12" y1="18" x2="12.01" y2="18"></line>
        </svg>
    }
}


#[function_component(Crosshair)]
pub fn r#crosshair(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle><line x1="22" y1="12" x2="18" y2="12"></line><line x1="6" y1="12" x2="2" y2="12"></line><line x1="12" y1="6" x2="12" y2="2"></line><line x1="12" y1="22" x2="12" y2="18"></line>
        </svg>
    }
}


#[function_component(Maximize2)]
pub fn r#maximize_2(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="15 3 21 3 21 9"></polyline><polyline points="9 21 3 21 3 15"></polyline><line x1="21" y1="3" x2="14" y2="10"></line><line x1="3" y1="21" x2="10" y2="14"></line>
        </svg>
    }
}


#[function_component(CloudRain)]
pub fn r#cloud_rain(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="16" y1="13" x2="16" y2="21"></line><line x1="8" y1="13" x2="8" y2="21"></line><line x1="12" y1="15" x2="12" y2="23"></line><path d="M20 16.58A5 5 0 0 0 18 7h-1.26A8 8 0 1 0 4 15.25"></path>
        </svg>
    }
}


#[function_component(Briefcase)]
pub fn r#briefcase(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="2" y="7" width="20" height="14" rx="2" ry="2"></rect><path d="M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16"></path>
        </svg>
    }
}


#[function_component(Type)]
pub fn r#type(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="4 7 4 4 20 4 20 7"></polyline><line x1="9" y1="20" x2="15" y2="20"></line><line x1="12" y1="4" x2="12" y2="20"></line>
        </svg>
    }
}


#[function_component(Facebook)]
pub fn r#facebook(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M18 2h-3a5 5 0 0 0-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 0 1 1-1h3z"></path>
        </svg>
    }
}


#[function_component(MessageSquare)]
pub fn r#message_square(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path>
        </svg>
    }
}


#[function_component(AlignCenter)]
pub fn r#align_center(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="18" y1="10" x2="6" y2="10"></line><line x1="21" y1="6" x2="3" y2="6"></line><line x1="21" y1="14" x2="3" y2="14"></line><line x1="18" y1="18" x2="6" y2="18"></line>
        </svg>
    }
}


#[function_component(Cloud)]
pub fn r#cloud(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M18 10h-1.26A8 8 0 1 0 9 20h9a5 5 0 0 0 0-10z"></path>
        </svg>
    }
}


#[function_component(CornerLeftUp)]
pub fn r#corner_left_up(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="14 9 9 4 4 9"></polyline><path d="M20 20h-7a4 4 0 0 1-4-4V4"></path>
        </svg>
    }
}


#[function_component(PlusCircle)]
pub fn r#plus_circle(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle><line x1="12" y1="8" x2="12" y2="16"></line><line x1="8" y1="12" x2="16" y2="12"></line>
        </svg>
    }
}


#[function_component(Shuffle)]
pub fn r#shuffle(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="16 3 21 3 21 8"></polyline><line x1="4" y1="20" x2="21" y2="3"></line><polyline points="21 16 21 21 16 21"></polyline><line x1="15" y1="15" x2="21" y2="21"></line><line x1="4" y1="4" x2="9" y2="9"></line>
        </svg>
    }
}


#[function_component(CornerDownLeft)]
pub fn r#corner_down_left(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="9 10 4 15 9 20"></polyline><path d="M20 4v7a4 4 0 0 1-4 4H4"></path>
        </svg>
    }
}


#[function_component(Twitch)]
pub fn r#twitch(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M21 2H3v16h5v4l4-4h5l4-4V2zM11 11V7M16 11V7"></path>
        </svg>
    }
}


#[function_component(UserMinus)]
pub fn r#user_minus(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path><circle cx="8.5" cy="7" r="4"></circle><line x1="23" y1="11" x2="17" y2="11"></line>
        </svg>
    }
}


#[function_component(Droplet)]
pub fn r#droplet(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M12 2.69l5.66 5.66a8 8 0 1 1-11.31 0z"></path>
        </svg>
    }
}


#[function_component(PhoneCall)]
pub fn r#phone_call(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M15.05 5A5 5 0 0 1 19 8.95M15.05 1A9 9 0 0 1 23 8.94m-1 7.98v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path>
        </svg>
    }
}


#[function_component(ArrowUpCircle)]
pub fn r#arrow_up_circle(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle><polyline points="16 12 12 8 8 12"></polyline><line x1="12" y1="16" x2="12" y2="8"></line>
        </svg>
    }
}


#[function_component(ArrowUpRight)]
pub fn r#arrow_up_right(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="7" y1="17" x2="17" y2="7"></line><polyline points="7 7 17 7 17 17"></polyline>
        </svg>
    }
}


#[function_component(BookOpen)]
pub fn r#book_open(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z"></path><path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z"></path>
        </svg>
    }
}


#[function_component(ExternalLink)]
pub fn r#external_link(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path><polyline points="15 3 21 3 21 9"></polyline><line x1="10" y1="14" x2="21" y2="3"></line>
        </svg>
    }
}


#[function_component(Terminal)]
pub fn r#terminal(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="4 17 10 11 4 5"></polyline><line x1="12" y1="19" x2="20" y2="19"></line>
        </svg>
    }
}


#[function_component(Codepen)]
pub fn r#codepen(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polygon points="12 2 22 8.5 22 15.5 12 22 2 15.5 2 8.5 12 2"></polygon><line x1="12" y1="22" x2="12" y2="15.5"></line><polyline points="22 8.5 12 15.5 2 8.5"></polyline><polyline points="2 15.5 12 8.5 22 15.5"></polyline><line x1="12" y1="2" x2="12" y2="8.5"></line>
        </svg>
    }
}


#[function_component(MinusCircle)]
pub fn r#minus_circle(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle><line x1="8" y1="12" x2="16" y2="12"></line>
        </svg>
    }
}


#[function_component(Slash)]
pub fn r#slash(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle><line x1="4.93" y1="4.93" x2="19.07" y2="19.07"></line>
        </svg>
    }
}


#[function_component(ShieldOff)]
pub fn r#shield_off(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M19.69 14a6.9 6.9 0 0 0 .31-2V5l-8-3-3.16 1.18"></path><path d="M4.73 4.73L4 5v7c0 6 8 10 8 10a20.29 20.29 0 0 0 5.62-4.38"></path><line x1="1" y1="1" x2="23" y2="23"></line>
        </svg>
    }
}


#[function_component(Copy)]
pub fn r#copy(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
        </svg>
    }
}


#[function_component(Youtube)]
pub fn r#youtube(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M22.54 6.42a2.78 2.78 0 0 0-1.94-2C18.88 4 12 4 12 4s-6.88 0-8.6.46a2.78 2.78 0 0 0-1.94 2A29 29 0 0 0 1 11.75a29 29 0 0 0 .46 5.33A2.78 2.78 0 0 0 3.4 19c1.72.46 8.6.46 8.6.46s6.88 0 8.6-.46a2.78 2.78 0 0 0 1.94-2 29 29 0 0 0 .46-5.25 29 29 0 0 0-.46-5.33z"></path><polygon points="9.75 15.02 15.5 11.75 9.75 8.48 9.75 15.02"></polygon>
        </svg>
    }
}


#[function_component(Bell)]
pub fn r#bell(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"></path><path d="M13.73 21a2 2 0 0 1-3.46 0"></path>
        </svg>
    }
}


#[function_component(AlertOctagon)]
pub fn r#alert_octagon(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polygon points="7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2"></polygon><line x1="12" y1="8" x2="12" y2="12"></line><line x1="12" y1="16" x2="12.01" y2="16"></line>
        </svg>
    }
}


#[function_component(Pocket)]
pub fn r#pocket(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M4 3h16a2 2 0 0 1 2 2v6a10 10 0 0 1-10 10A10 10 0 0 1 2 11V5a2 2 0 0 1 2-2z"></path><polyline points="8 10 12 14 16 10"></polyline>
        </svg>
    }
}


#[function_component(Link2)]
pub fn r#link_2(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M15 7h3a5 5 0 0 1 5 5 5 5 0 0 1-5 5h-3m-6 0H6a5 5 0 0 1-5-5 5 5 0 0 1 5-5h3"></path><line x1="8" y1="12" x2="16" y2="12"></line>
        </svg>
    }
}


#[function_component(Edit2)]
pub fn r#edit_2(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M17 3a2.828 2.828 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z"></path>
        </svg>
    }
}


#[function_component(Map)]
pub fn r#map(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polygon points="1 6 1 22 8 18 16 22 23 18 23 2 16 6 8 2 1 6"></polygon><line x1="8" y1="2" x2="8" y2="18"></line><line x1="16" y1="6" x2="16" y2="22"></line>
        </svg>
    }
}


#[function_component(Crop)]
pub fn r#crop(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M6.13 1L6 16a2 2 0 0 0 2 2h15"></path><path d="M1 6.13L16 6a2 2 0 0 1 2 2v15"></path>
        </svg>
    }
}


#[function_component(ShoppingBag)]
pub fn r#shopping_bag(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M6 2L3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6l-3-4z"></path><line x1="3" y1="6" x2="21" y2="6"></line><path d="M16 10a4 4 0 0 1-8 0"></path>
        </svg>
    }
}


#[function_component(SkipForward)]
pub fn r#skip_forward(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polygon points="5 4 15 12 5 20 5 4"></polygon><line x1="19" y1="5" x2="19" y2="19"></line>
        </svg>
    }
}


#[function_component(Play)]
pub fn r#play(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polygon points="5 3 19 12 5 21 5 3"></polygon>
        </svg>
    }
}


#[function_component(FileMinus)]
pub fn r#file_minus(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path><polyline points="14 2 14 8 20 8"></polyline><line x1="9" y1="15" x2="15" y2="15"></line>
        </svg>
    }
}


#[function_component(GitMerge)]
pub fn r#git_merge(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="18" cy="18" r="3"></circle><circle cx="6" cy="6" r="3"></circle><path d="M6 21V9a9 9 0 0 0 9 9"></path>
        </svg>
    }
}


#[function_component(Home)]
pub fn r#home(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path><polyline points="9 22 9 12 15 12 15 22"></polyline>
        </svg>
    }
}


#[function_component(ToggleRight)]
pub fn r#toggle_right(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="1" y="5" width="22" height="14" rx="7" ry="7"></rect><circle cx="16" cy="12" r="3"></circle>
        </svg>
    }
}


#[function_component(Disc)]
pub fn r#disc(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle><circle cx="12" cy="12" r="3"></circle>
        </svg>
    }
}


#[function_component(PauseCircle)]
pub fn r#pause_circle(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle><line x1="10" y1="15" x2="10" y2="9"></line><line x1="14" y1="15" x2="14" y2="9"></line>
        </svg>
    }
}


#[function_component(CornerLeftDown)]
pub fn r#corner_left_down(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="14 15 9 20 4 15"></polyline><path d="M20 4h-7a4 4 0 0 0-4 4v12"></path>
        </svg>
    }
}


#[function_component(PhoneIncoming)]
pub fn r#phone_incoming(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="16 2 16 8 22 8"></polyline><line x1="23" y1="1" x2="16" y2="8"></line><path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path>
        </svg>
    }
}


#[function_component(Github)]
pub fn r#github(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>
        </svg>
    }
}


#[function_component(ArrowLeft)]
pub fn r#arrow_left(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="19" y1="12" x2="5" y2="12"></line><polyline points="12 19 5 12 12 5"></polyline>
        </svg>
    }
}


#[function_component(FileText)]
pub fn r#file_text(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path><polyline points="14 2 14 8 20 8"></polyline><line x1="16" y1="13" x2="8" y2="13"></line><line x1="16" y1="17" x2="8" y2="17"></line><polyline points="10 9 9 9 8 9"></polyline>
        </svg>
    }
}


#[function_component(Globe)]
pub fn r#globe(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle><line x1="2" y1="12" x2="22" y2="12"></line><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"></path>
        </svg>
    }
}


#[function_component(Info)]
pub fn r#info(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle><line x1="12" y1="16" x2="12" y2="12"></line><line x1="12" y1="8" x2="12.01" y2="8"></line>
        </svg>
    }
}


#[function_component(Server)]
pub fn r#server(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="2" y="2" width="20" height="8" rx="2" ry="2"></rect><rect x="2" y="14" width="20" height="8" rx="2" ry="2"></rect><line x1="6" y1="6" x2="6.01" y2="6"></line><line x1="6" y1="18" x2="6.01" y2="18"></line>
        </svg>
    }
}


#[function_component(Menu)]
pub fn r#menu(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="3" y1="12" x2="21" y2="12"></line><line x1="3" y1="6" x2="21" y2="6"></line><line x1="3" y1="18" x2="21" y2="18"></line>
        </svg>
    }
}


#[function_component(PhoneMissed)]
pub fn r#phone_missed(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <line x1="23" y1="1" x2="17" y2="7"></line><line x1="17" y1="1" x2="23" y2="7"></line><path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path>
        </svg>
    }
}


#[function_component(Sunrise)]
pub fn r#sunrise(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M17 18a5 5 0 0 0-10 0"></path><line x1="12" y1="2" x2="12" y2="9"></line><line x1="4.22" y1="10.22" x2="5.64" y2="11.64"></line><line x1="1" y1="18" x2="3" y2="18"></line><line x1="21" y1="18" x2="23" y2="18"></line><line x1="18.36" y1="11.64" x2="19.78" y2="10.22"></line><line x1="23" y1="22" x2="1" y2="22"></line><polyline points="8 6 12 2 16 6"></polyline>
        </svg>
    }
}


#[function_component(ZoomIn)]
pub fn r#zoom_in(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line><line x1="11" y1="8" x2="11" y2="14"></line><line x1="8" y1="11" x2="14" y2="11"></line>
        </svg>
    }
}


#[function_component(Save)]
pub fn r#save(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"></path><polyline points="17 21 17 13 7 13 7 21"></polyline><polyline points="7 3 7 8 15 8"></polyline>
        </svg>
    }
}


#[function_component(Circle)]
pub fn r#circle(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <circle cx="12" cy="12" r="10"></circle>
        </svg>
    }
}


#[function_component(Move)]
pub fn r#move(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="5 9 2 12 5 15"></polyline><polyline points="9 5 12 2 15 5"></polyline><polyline points="15 19 12 22 9 19"></polyline><polyline points="19 9 22 12 19 15"></polyline><line x1="2" y1="12" x2="22" y2="12"></line><line x1="12" y1="2" x2="12" y2="22"></line>
        </svg>
    }
}


#[function_component(RefreshCw)]
pub fn r#refresh_cw(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="23 4 23 10 17 10"></polyline><polyline points="1 20 1 14 7 14"></polyline><path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"></path>
        </svg>
    }
}


#[function_component(PhoneOff)]
pub fn r#phone_off(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M10.68 13.31a16 16 0 0 0 3.41 2.6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7 2 2 0 0 1 1.72 2v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.42 19.42 0 0 1-3.33-2.67m-2.67-3.34a19.79 19.79 0 0 1-3.07-8.63A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91"></path><line x1="23" y1="1" x2="1" y2="23"></line>
        </svg>
    }
}


#[function_component(CloudOff)]
pub fn r#cloud_off(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M22.61 16.95A5 5 0 0 0 18 10h-1.26a8 8 0 0 0-7.05-6M5 5a8 8 0 0 0 4 15h9a5 5 0 0 0 1.7-.3"></path><line x1="1" y1="1" x2="23" y2="23"></line>
        </svg>
    }
}


#[function_component(Unlock)]
pub fn r#unlock(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect><path d="M7 11V7a5 5 0 0 1 9.9-1"></path>
        </svg>
    }
}


#[function_component(Linkedin)]
pub fn r#linkedin(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z"></path><rect x="2" y="9" width="4" height="12"></rect><circle cx="4" cy="4" r="2"></circle>
        </svg>
    }
}


#[function_component(Volume)]
pub fn r#volume(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5"></polygon>
        </svg>
    }
}


#[function_component(Bluetooth)]
pub fn r#bluetooth(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <polyline points="6.5 6.5 17.5 17.5 12 23 12 1 17.5 6.5 6.5 17.5"></polyline>
        </svg>
    }
}


#[function_component(UserCheck)]
pub fn r#user_check(
    IconProps {
        class,
        size,
        fill,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
    }: &IconProps,
) -> Html {
    html! {
        <svg
            class={class.clone()}
            width={size.clone()}
            height={size}
            viewBox="0 0 24 24"
            {fill}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
        >
            <path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path><circle cx="8.5" cy="7" r="4"></circle><polyline points="17 11 19 13 23 9"></polyline>
        </svg>
    }
}

