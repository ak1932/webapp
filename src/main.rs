use yew::prelude::*;
// use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    slider_value: UseStateHandle<i32>,
    color_value: UseStateHandle<AttrValue>,
    name_value: UseStateHandle<AttrValue>,
}

#[function_component]
fn Section(props: &SectionProps) -> Html {
    html! (
        <div class={classes!{"section"}}>
            <Knob slider_value={*props.slider_value} />
            <DetentPicker slider_value={props.slider_value.clone()} />
            <ColorPicker color_value={props.color_value.clone()} />
            <NamePicker name_value={props.name_value.clone()} />
        </div>
    )
}

#[derive(Properties, PartialEq)]
pub struct KnobProps {
    #[prop_or(1)]
    pub slider_value: i32,
}

#[function_component]
fn Knob(props: &KnobProps) -> Html {
    let items = {0..props.slider_value}
    .map(|n| {
        let a=360 * n /props.slider_value;
        html_nested! { <div class={classes!{"dot_space"}} style={format!("transform: rotate({}deg)", a)}>
            <section class={classes!{"dot"}}></section>
        </div>}})
        .collect::<Html>();

    html!(
        <div class={classes!{"knob"}}>
            {items}
            <p class={classes!{"current_value"}}>{props.slider_value}</p>
        </div>
    )
}

#[derive(Properties, PartialEq)]
pub struct DetentPickerProps {
    slider_value: UseStateHandle<i32>,
}

#[function_component]
fn DetentPicker(props: &DetentPickerProps) -> Html {
    let detent_input = {
        let slider_value = props.slider_value.clone();
        move |e: InputEvent| { 
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            slider_value.set(input.value_as_number() as i32);
        }
    };
    html!(
        <div class={classes!{"detent_picker"}}>
            <input class={classes!{"detent"}} min={"1"} max={"100"} type={"range"} value = {format!("{}",*props.slider_value)} oninput = {detent_input}/> 
        </div>
     )
}

#[derive(Properties, PartialEq)]
pub struct ColorPickerProps {
    color_value: UseStateHandle<AttrValue>,
}

#[function_component]
fn ColorPicker(props: &ColorPickerProps) -> Html {
    let color_input = {
        let color_value=props.color_value.clone();
        move |e: InputEvent| {
            let target: web_sys::EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            color_value.set(target.unchecked_into::<web_sys::HtmlInputElement>().value().into());
        }};

    html!(
        <div class={classes!{"color_picker"}}>
            <label for={"color"} class={classes!{"color_label"}}>{"Mode Color"}</label>
            <input class={classes!{"color"}} name={"color"} type={"color"} value = {format!("{}",*props.color_value)} oninput = {color_input}/> 
        </div>
     )
}


#[derive(Properties, PartialEq)]
pub struct NamePickerProps {
    name_value: UseStateHandle<AttrValue>,
}

#[function_component]
fn NamePicker(props: &NamePickerProps) -> Html {

    let name_input = {
        let name_value=props.name_value.clone();
        move |e: InputEvent| {
            let target: web_sys::EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            name_value.set(target.unchecked_into::<web_sys::HtmlInputElement>().value().into());
        }};

    html!(
        <div class = {classes!{"name_picker"}}>
            // <label for={"color"} id={"color_label"}>{"Select color for the mode:"}</label>
            <input class={classes!{"name"}} name={"name"} type={"text"} oninput = {name_input} placeholder = {"Name"} autofocus={true}/> 
        </div>
     )
}

#[function_component]
fn App() -> Html {
    let mut sections: Vec<SectionProps> = Vec::new();
    let slider_value = use_state(|| 1 as i32);
    let color_value = use_state(|| AttrValue::from("#ffffff"));
    let name_value = use_state(|| AttrValue::from(""));

    let section_props = yew::props!(SectionProps {slider_value,color_value,name_value});
    sections.push(section_props);

    let root_element_style = web_sys::window().unwrap().document().unwrap().document_element().expect("should have #loading on the page").dyn_into::<web_sys::HtmlElement>().expect("#loading should be an `HtmlElement`").style();

    // root_element_style.set_property("--chosen-color", &*section_props.color_value).unwrap();

//     let add_section = {|_| {
//         let slider_value = use_state(|| 1 as i32);
//         let color_value = use_state(|| String::from("#ffffff"));
//         let name_value = use_state(|| String::from(""));
//         sections.push(
//             yew::props!(
//                 SectionProps {
//                     slider_value,
//                     color_value,
//                     name_value,
//                 })
//         )
//     }};


    html! {
        <>
        <h1 id={"main_heading"}>{"SmartKnob"}</h1>
        { sections.into_iter()
            .map(|n| {
                html_nested! { <Section ..n/> }
            })
            .collect::<Html>()
        }
        <button id={"add_section"}>{"+"}</button> 
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
