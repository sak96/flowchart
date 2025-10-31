use parse::Graph;
use std::str::FromStr;
use yew::prelude::*;

mod parse;
#[function_component(App)]
fn app() -> Html {
    let input_text = use_state(|| "".to_string());
    let output_text = use_state(|| "".to_string());
    let invalid = use_state(|| false);

    // Clone for closures
    let input_text_clone = input_text.clone();
    // let output_text_clone = output_text.clone();

    let oninput = Callback::from(move |e: InputEvent| {
        let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
        input_text_clone.set(input.value());
    });

    let invalid_clone = invalid.clone();
    let onclick = {
        let input_text = input_text.clone();
        let output_text = output_text.clone();
        Callback::from(move |_| {
            let graph_result = Graph::from_str(&input_text);

            let debug_info = match graph_result {
                Ok(graph) => {
                    invalid_clone.set(false);
                    format!("{:#?}", graph)
                }
                Err(err) => {
                    invalid_clone.set(true);
                    format!("Error parsing graph:\n{}", err)
                }
            };
            output_text.set(debug_info);
        })
    };
    let aria = format!("{}", *invalid.clone());
    html! {
        <div class="container-fluid">
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
                aria-invalid={aria}
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
