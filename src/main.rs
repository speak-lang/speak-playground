use core::runtime::Context as SpeakCtx;
use serde::{Deserialize, Serialize};
use std::io::BufReader;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Eq, Store, Serialize, Deserialize)]
#[store(storage = "session")]
struct State {
    program_input: String,
}

#[derive(Properties, PartialEq)]
struct Result_ {
    value: Vec<String>,
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
               <p> { res.clone().ok().expect("value is ok").0 }</p>

               <TokenStream value={ res.clone().ok().expect("value is ok").1}/>

               <SyntaxTree value={ res.ok().expect("value is ok").2} />
            } else {
              <p>  { res.err().expect("value is error") } </p>
            }
        </div>
    }
}

#[function_component]
fn TokenStream(prop: &Result_) -> Html {
    // Speak wasm-interpreter provides token stream
    // Render token stream
    html! {
        <div  style="border:1px solid black; width=100px; height=100px">
            <p> { prop.value.clone() } </p>
        </div>
    }
}

#[function_component]
fn SyntaxTree(prop: &Result_) -> Html {
    // Speak wasm-interpreter provides syntax tree
    // Render syntax tree
    html! {
        <div  style="border:1px solid black; width=100px; height=100px">
           <p> {prop.value.clone()}</p>
        </div>
    }
}

fn speak_interpreter(input: &str) -> Result<(String, Vec<String>, Vec<String>), String> {
    let mut ctx = SpeakCtx::new(&false);
    match ctx.exec(BufReader::new(input.as_bytes())) {
        Ok((val, tok_stream, syntax_tree)) => {
            let tok_stream = tok_stream
                .iter()
                .map(|val| val.string())
                .collect::<Vec<String>>();
            let syntax_tree = syntax_tree
                .iter()
                .map(|val| val.string())
                .collect::<Vec<String>>();

            Ok((val.string(), tok_stream, syntax_tree))
        }
        Err(err) => Err(err.message),
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
