use core::{parser::Node, runtime::Context as SpeakCtx};
use serde::{Deserialize, Serialize};
use std::io::BufReader;
use web_sys;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Eq, Store, Serialize, Deserialize)]
#[store(storage = "session")]
struct Input_ {
    program_input: String,
    language: String,
}

#[derive(Properties, PartialEq)]
struct Result_ {
    value: Vec<String>,
}

impl Default for Input_ {
    fn default() -> Self {
        Self {
            program_input: "\"Hello, World!\"".to_string(),
            language: "en".to_string(),
        }
    }
}

#[function_component]
fn App() -> Html {
    let select_node_ref = use_node_ref();
    let (_, dispatch) = use_store::<Input_>();

    let onchange = {
        let select_node_ref = select_node_ref.clone();
        dispatch.reduce_mut_callback(move |state| {
            state.language = select_node_ref
                .clone()
                .cast::<web_sys::HtmlSelectElement>()
                .expect("This select node is valid")
                .value()
                .to_string();
        })
    };

    html! {
        <div>
            <label for="languages">{"Choose an interpreter language variant:"}</label>
            <select ref={select_node_ref} {onchange} name="languages">
                <option value="en" selected=true>{"English"}</option>
                <option value="sw">{"Swahili"}</option>
            </select>
            <Input/>
            <Output/>
        </div>
    }
}

// input represents the playground user input.
#[function_component]
fn Input() -> Html {
    let (state, dispatch) = use_store::<Input_>();
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
        <div>
            <h3>{ "Program Input" }</h3>
            <div>
                <textarea ref={input_node_ref} value={(*state).clone().program_input}></textarea>
                <button {onclick}>{"Run"}</button>
            </div>
        </div>
    }
}

// output presents the playground output, if any.
#[function_component]
fn Output() -> Html {
    // get program input
    let (state, _) = use_store::<Input_>();

    // run against interpreter
    let res = speak_interpreter(&state.language, &state.program_input);

    // Speak wasm-interpreter provides output if any
    match res {
        Ok((program_output, token_stream, syntax_tree)) => {
            html! {
                <div>
                    <div >
                        <h3>{ "Output" }</h3>
                        { program_output }
                    </div>

                    <TokenStream value={ token_stream}/>

                    { syntax_tree }
                </div>
            }
        }
        Err(err) => {
            html! {
                <p>  { err } </p>
            }
        }
    }
}

#[function_component]
fn TokenStream(prop: &Result_) -> Html {
    // Speak wasm-interpreter provides token stream
    // Render token stream
    html! {
        <div>
        <h3>{ "Token Stream" }</h3>
        {
            prop.value.clone().into_iter().enumerate().map(|(i, entry)| {
                html!{< key={entry}><p> { format!("{}. {}", i+1, entry.clone()) } </p></>}
            }).collect::<Html>()
        }
        </div>
    }
}

fn speak_interpreter(lang: &str, input: &str) -> Result<(String, Vec<String>, Html), String> {
    let mut ctx = SpeakCtx::new(&false);
    match ctx.exec(lang, BufReader::new(input.as_bytes())) {
        Ok((val, tok_stream, syntax_tree)) => {
            let tok_stream = tok_stream
                .iter()
                .map(|val| val.string())
                .collect::<Vec<String>>();

            Ok((val.string(), tok_stream, compose_tree(syntax_tree)))
        }
        Err(err) => Err(err.message),
    }
}

fn compose_tree(nodes: Vec<Node>) -> Html {
    html! {
        <>
            <h3>{ "Syntax Tree" }</h3>
            <ul>
                {nest_tree(&nodes)}
            </ul>
        </>
    }
}

fn nest_tree(nodes: &Vec<Node>) -> Html {
    nodes
        .into_iter()
        .enumerate()
        .map(|(i, node)| match node {
            Node::FunctionLiteral { body, .. } => {
                let id = format!("c{}", i + 1); // The first checkbox is c1
                html! {
                    <li>
                        <input type="checkbox" checked=true id={ id.clone() } />
                        <label for={id}>{ node.string() }</label>
                        <ul> { nest_tree(body)} </ul>
                    </li>
                }
            }

            _ => {
                html! {
                 <li><span>{ node.string() }</span></li>
                }
            }
        })
        .collect::<Html>()
}

fn main() {
    yew::Renderer::<App>::new().render();
}
