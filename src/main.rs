//use yew::prelude::*;
use yew::{classes, function_component, html, use_state, Html};

#[function_component]
fn App() -> Html {
    html! {
        <div>
            {input()}

            {output()}

            {token_stream()}

            {syntax_tree()}
        </div>
    }
}

// input represents the playground user input.
fn input() -> Html {
    // HTML form to capture the input
    // style that numbers rows
    // form button `Run`
    //
    // Speak wasm-interpreter build bundled with playground
    html! {
        <>
                { "Program input:" }
                <textarea rows="5" cols="50">
                </textarea>

                // type="text"
                // value={"print \"Hello, world!\""}
                // />
                <button>{"Run"}</button>
        </>
    }
}

// output presents the playground output, if any.
fn output() -> Html {
    // Speak wasm-interpreter provides output if any
    // Render output
    html! {
        <div  style="border:1px solid black; width=100px; height=100px">
            {"This is the output"}
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

fn main() {
    yew::Renderer::<App>::new().render();
}
