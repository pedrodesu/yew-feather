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
