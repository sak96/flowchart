use parse::Graph;
use std::str::FromStr;
use yew::prelude::*;

mod parse;
#[function_component(App)]
fn app() -> Html {
    let input_text = use_state(|| "".to_string());
    let output_text = use_state(|| "".to_string());

    // Clone for closures
    let input_text_clone = input_text.clone();
    // let output_text_clone = output_text.clone();

    let oninput = Callback::from(move |e: InputEvent| {
        let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
        input_text_clone.set(input.value());
    });

    let onclick = {
        let input_text = input_text.clone();
        let output_text = output_text.clone();
        Callback::from(move |_| {
            let graph_result = Graph::from_str(&input_text);
            let debug_info = match graph_result {
                Ok(graph) => format!("Parsed graph:\n{:#?}", graph),
                Err(err) => format!("Error parsing graph:\n{}", err),
            };
            output_text.set(debug_info);
        })
    };

    html! {
        <div>
            <h3>{"Input Graph Text"}</h3>
            <textarea
                rows={10}
                cols={50}
                placeholder="Enter graph description here..."
                value={(*input_text).clone()}
                {oninput}
            />
            <br />
            <button {onclick}>{"Generate Graph"}</button>
            <h3>{"Output / Debug"}</h3>
            <textarea
                rows={10}
                cols={50}
                value={(*output_text).clone()}
                readonly=true
            />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
