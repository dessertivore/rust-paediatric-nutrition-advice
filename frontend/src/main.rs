use serde::Serialize;
use web_sys::HtmlInputElement;
use web_sys::HtmlSelectElement;
use yew;
use yew::prelude::*;

#[derive(Clone, PartialEq, Serialize, Default)]
struct ChildProfile {
    name: String,
    birthday: String,
    gender: String,
    weight: Option<f32>,
    height: Option<f32>,
}

#[function_component(App)]
fn app() -> Html {
    let profile = use_state(ChildProfile::default);

    let on_name_input = {
        let profile = profile.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut updated = (*profile).clone();
            updated.name = input.value();
            profile.set(updated);
        })
    };

    let on_birthday_input = {
        let profile = profile.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut updated = (*profile).clone();
            updated.birthday = input.value();
            profile.set(updated);
        })
    };

    let on_gender_change = {
        let profile = profile.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            let mut updated = (*profile).clone();
            updated.gender = select.value();
            profile.set(updated);
        })
    };

    let on_weight_input = {
        let profile = profile.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut updated = (*profile).clone();
            updated.weight = input.value().parse::<f32>().ok();
            profile.set(updated);
        })
    };

    let on_height_input = {
        let profile = profile.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut updated = (*profile).clone();
            updated.height = input.value().parse::<f32>().ok();
            profile.set(updated);
        })
    };

    let on_submit = {
        let profile = profile.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let json = serde_json::to_string_pretty(&*profile).unwrap();
            web_sys::console::log_1(&json.into());
        })
    };

    html! {
        <div style="max-width: 400px; margin: 40px auto; font-family: sans-serif;">
            <h2>{"Child Profile Form"}</h2>

            <form onsubmit={on_submit}>

                <label>{"Name:"}</label>
                <input type="text" oninput={on_name_input} required=true />
                <br/><br/>

                <label>{"Birthday:"}</label>
                <input type="date" oninput={on_birthday_input} required=true />
                <br/><br/>

                <label>{"Gender:"}</label>
                <select onchange={on_gender_change} required=true>
                    <option value="">{"Select..."}</option>
                    <option value="Male">{"Male"}</option>
                    <option value="Female">{"Female"}</option>
                    <option value="Other">{"Other"}</option>
                </select>
                <br/><br/>

                <label>{"Weight (kg) - optional:"}</label>
                <input type="number" step="0.1" oninput={on_weight_input} />
                <br/><br/>

                <label>{"Height (cm) - optional:"}</label>
                <input type="number" step="0.1" oninput={on_height_input} />
                <br/><br/>

                <button type="submit">{"Submit"}</button>

            </form>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
