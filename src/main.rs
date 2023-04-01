use core::{parser::Node, runtime::Context as SpeakCtx};
use serde::{Deserialize, Serialize};
use std::io::BufReader;
use web_sys;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Eq, Store, Serialize, Deserialize)]
#[store(storage = "session")]
struct ProgramInput {
    input: String,
    language: String,
}

#[derive(Properties, PartialEq)]
struct ProgramOutput {
    value: Vec<String>,
}

impl Default for ProgramInput {
    fn default() -> Self {
        Self {
            input: String::from("\"Habari, Dunia!\""),
            language: String::from("sw"),
        }
    }
}

#[function_component]
fn App() -> Html {
    // get program input
    let (state, dispatch) = use_store::<ProgramInput>();

    // retrieve the language variant
    let select_node_ref = use_node_ref();
    let onchange = {
        let select_node_ref = select_node_ref.clone();
        dispatch.reduce_mut_callback(move |state| {
            state.language = select_node_ref
                .cast::<web_sys::HtmlSelectElement>()
                .expect("This select node is valid")
                .value();
        })
    };

    // retrieve the program input
    let input_node_ref = use_node_ref();
    let onclick = {
        let input_node_ref_ = input_node_ref.clone();
        dispatch.reduce_mut_callback(move |state| {
            state.input = input_node_ref_
                .cast::<web_sys::HtmlTextAreaElement>()
                .expect("This textarea node is valid")
                .value();
        })
    };

    // run against interpreter
    let res = speak_interpreter(&state);

    html! {
        <div class={classes!("max-w-2xl", "mx-auto", "p-6")}>
            <h2 class={classes!("text-2xl", "font-bold", "mb-4")}>{ "Speak Interpreter" }</h2>
            <div class={classes!("mb-6", "flex", "flex-col", "items-center")} >
                <label for="languages" class={classes!("mb-2", "font-medium")}>{"Language Variant"}</label>
                <select ref={select_node_ref} {onchange} name="languages" class={classes!("w-48", "h-10", "border-gray-300", "rounded-md", "shadow-sm")}>
                    <option value="en" class={classes!("text-center")}>{"English"}</option>
                    <option value="sw" class={classes!("text-center")} selected=true>{"Swahili"}</option>
                </select>
            </div>


            // Program Input
            <div class={classes!("mb-6")}>
                <h3 class={classes!("font-medium", "mb-2")}>{ "Program Input" }</h3>
                <div class={classes!("relative")}>
                    <textarea ref={input_node_ref} value={(*state).clone().input}
                    class={classes!("block", "w-full", "border-gray-300", "rounded-md", "shadow-sm", "pl-2", "pt-2", "pr-8", "pb-8")}
                    rows=6></textarea>
                    <button {onclick}
                    class={classes!("absolute", "bottom-2", "right-2", "px-3", "py-1", "font-medium", "text-white", "bg-red-500", "rounded-md", "hover:bg-red-600")}
                    >{"Run"}</button>
                </div>
            </div>

            // Render output or error
            <div class={classes!("mb-6")}>
                if res.is_ok() {
                    // Render output
                    <div class={classes!("mb-4")}>
                        <h3 class={classes!("font-medium", "mb-2")}>{ "Output" }</h3>
                        <div class={classes!("bg-white", "rounded-md", "shadow-sm", "p-4")}>{ res.clone().expect("output exists as res is ok").0 }</div>
                    </div>

                    // Render token stream
                    <div class={classes!("mb-4")}>
                        <h3 class={classes!("font-medium", "mb-2")}>{ "Token Stream" }</h3>
                        <div class={classes!("bg-white", "rounded-md", "shadow-sm", "p-4")}>
                            {
                                res.clone().expect("token_stream exists as res is ok").1.into_iter().enumerate().map(|(i, entry)| {
                                html!{< key={entry}><p> { format!("{}. {}", i+1, entry.clone()) } </p></>}
                                }).collect::<Html>()
                            }
                        </div>
                    </div>

                    { res.expect("syntax_tree exists as res is ok").2 }
            } else {
                // Render error
                <p class={classes!("text-red-400")}> { res.unwrap_err() } </p>
            }
            </div>
        </div>
    }
}

fn speak_interpreter(program_input: &ProgramInput) -> Result<(String, Vec<String>, Html), String> {
    let mut ctx = SpeakCtx::new(&false);
    match ctx.exec(
        &program_input.language,
        BufReader::new(program_input.input.as_bytes()),
    ) {
        Ok((val, tok_stream, syntax_tree)) => {
            let tok_stream = tok_stream
                .iter()
                .map(|val| val.string())
                .collect::<Vec<String>>();

            Ok((
                val.string(),
                tok_stream,
                html! {
                    <div>
                        <h3 class={classes!("font-medium", "mb-2")}>{ "Syntax Tree" }</h3>
                        <ul class={classes!("border", "border-gray-300", "rounded-md", "p-4")}>
                            {nest_tree(&syntax_tree)}
                        </ul>
                    </div>
                },
            ))
        }
        Err(err) => Err(err.message),
    }
}

fn nest_tree(nodes: &Vec<Node>) -> Html {
    nodes
        .iter()
        .map(|node| match node {
            Node::FunctionLiteral { body, .. } => {
                html! {
                    <li class={classes!("mb-2")}>
                        <label class={classes!("font-medium")}>{ node.string() }</label>
                        <ul class={classes!("ml-4")}> { nest_tree(body)} </ul>
                    </li>
                }
            }
            _ => {
                html! {
                 <li><span class={classes!("font-medium")}>{ node.string() }</span></li>
                }
            }
        })
        .collect::<Html>()
}

fn main() {
    yew::Renderer::<App>::new().render();
}
