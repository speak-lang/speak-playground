use core::runtime::Context as SpeakCtx;
use serde::{Deserialize, Serialize};
use std::io::BufReader;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Eq, Store, Properties, Serialize, Deserialize)]
#[store(storage = "session")]
struct State {
    program_input: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            program_input: "\"Hello, World!\"".to_string(),
        }
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <>
            <Input/>

            <Output/>

            {token_stream()}

            {syntax_tree()}
        </>
    }
}

// input represents the playground user input.
#[function_component]
fn Input() -> Html {
    let (state, dispatch) = use_store::<State>();
    let input_node_ref = use_node_ref();

    let onclick = {
        let input_node_ref_ = input_node_ref.clone();
        dispatch.reduce_mut_callback(move |state| {
            state.program_input = input_node_ref_
                .cast::<web_sys::HtmlTextAreaElement>()
                .expect("This textarea node is valid")
                .value();
        })
    };

    html! {
        <>
            { "Program input:" }
            <textarea ref={input_node_ref}
             value={(*state).clone().program_input}
             rows="5"
             cols="50"
             ></textarea>
            <button {onclick}>{"Run"}</button>
        </>
    }
}

// output presents the playground output, if any.
#[function_component]
fn Output() -> Html {
    // get program input
    let (state, _) = use_store::<State>();

    // run against interpreter
    let res = speak_interpreter(&(*state).clone().program_input);
    let is_ok = res.is_ok();

    // Speak wasm-interpreter provides output if any
    // Render output
    html! {
        <div  style="border:1px solid black; width=100px; height=100px">
            if is_ok {
               <p> { res.ok().expect("value is ok") }</p>
            } else {
              <p>  { res.err().expect("value is error") } </p>
            }
        </div>
    }
}

fn token_stream() -> Html {
    // Speak wasm-interpreter provides token stream
    // Render token stream
    html! {
        <div  style="border:1px solid black; width=100px; height=100px">
            {"This is the token stream"}
        </div>
    }
}

fn syntax_tree() -> Html {
    // Speak wasm-interpreter provides syntax tree
    // Render syntax tree
    html! {
        <div  style="border:1px solid black; width=100px; height=100px">
            {"This is the syntax tree"}
        </div>
    }
}

fn speak_interpreter(input: &str) -> Result<String, String> {
    let mut ctx = SpeakCtx::new(&false);
    match ctx.exec(BufReader::new(input.as_bytes())) {
        Ok(val) => Ok(val.string()),
        Err(err) => Err(err.message),
    }
}

fn main() {
    // let res = speak_interpreter("print \"Hello, World!\"");

    // match res {
    //     Ok(val) => println!("Result is ok: {}", val),
    //     Err(err) => println!("Result is err: {}", err),
    // }

    yew::Renderer::<App>::new().render();
}
