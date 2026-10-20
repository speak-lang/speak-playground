use speak_core::runtime::Context;
use std::io::BufReader;
use web_sys::{HtmlSelectElement, HtmlTextAreaElement};
use yew::prelude::*;

const LANGS: [(&str, &str); 2] = [("en", "English"), ("sw", "Kiswahili")];

// (English name, Swahili name, English source, Swahili source)
const SAMPLES: [(&str, &str, &str, &str); 3] = [
    ("Hello world", "Habari dunia", include_str!("samples/hello.en.spk"), include_str!("samples/hello.sw.spk")),
    ("FizzBuzz", "FizzBuzz", include_str!("samples/fizzbuzz.en.spk"), include_str!("samples/fizzbuzz.sw.spk")),
    ("Fibonacci", "Fibonacci", include_str!("samples/fib.en.spk"), include_str!("samples/fib.sw.spk")),
];

fn sample(lang: &str, i: usize) -> &'static str {
    if lang == "sw" { SAMPLES[i].3 } else { SAMPLES[i].2 }
}

/// Runs a program and returns the value of its last expression, or the error text.
fn run(lang: &str, code: &str) -> Result<String, String> {
    // speak-core panics on empty input (parser::guard_unexpected_input_end), and the wasm build aborts on panic.
    if code.trim().is_empty() {
        return Ok(String::new());
    }
    let mut ctx = Context::new(&false);
    match ctx.exec(lang, BufReader::new(code.as_bytes())) {
        Ok((val, _, _)) => Ok(val.string()),
        Err(e) => Err(e.string()),
    }
}

#[function_component]
fn App() -> Html {
    let lang = use_state(|| "en".to_string());
    let idx = use_state(|| 0usize);
    let code = use_state(|| sample("en", 0).to_string());
    let result = use_state(|| None::<Result<String, String>>);

    // Changing language or sample reloads the matching sample.
    let load = {
        let (lang, idx, code, result) = (lang.clone(), idx.clone(), code.clone(), result.clone());
        move |l: String, i: usize| {
            code.set(sample(&l, i).to_string());
            lang.set(l);
            idx.set(i);
            result.set(None);
        }
    };
    let on_lang = {
        let (load, idx) = (load.clone(), idx.clone());
        Callback::from(move |e: Event| load(e.target_unchecked_into::<HtmlSelectElement>().value(), *idx))
    };
    let on_sample = {
        let (load, lang) = (load, lang.clone());
        Callback::from(move |e: Event| {
            let i = e.target_unchecked_into::<HtmlSelectElement>().value().parse().unwrap_or(0);
            load((*lang).clone(), i)
        })
    };
    let on_input = {
        let code = code.clone();
        Callback::from(move |e: InputEvent| code.set(e.target_unchecked_into::<HtmlTextAreaElement>().value()))
    };
    let on_run = {
        let (lang, code, result) = (lang.clone(), code.clone(), result.clone());
        Callback::from(move |_| result.set(Some(run(&lang, &code))))
    };

    let sw = *lang == "sw";
    let (title, l_lang, l_sample, l_code, l_run, l_out) = if sw {
        ("Uwanja wa Speak", "Lugha", "Mfano", "Programu", "Endesha", "Matokeo")
    } else {
        ("Speak playground", "Language", "Example", "Program", "Run", "Output")
    };

    html! {
        <>
        <header>
            <h1>{ title }</h1>
            <nav aria-label="Speak">
                <a href="https://speak-lang.github.io/">{ "Website" }</a>
                <a href="https://github.com/speak-lang/speak">{ "GitHub" }</a>
            </nav>
        </header>
        <main>
            <div class="controls">
                <label>{ l_lang }
                    <select onchange={on_lang}>
                        { for LANGS.iter().map(|(v, n)| html!{ <option value={*v} selected={*v == *lang}>{ *n }</option> }) }
                    </select>
                </label>
                <label>{ l_sample }
                    <select onchange={on_sample}>
                        { for SAMPLES.iter().enumerate().map(|(i, s)| html!{
                            <option value={i.to_string()} selected={i == *idx}>{ if sw { s.1 } else { s.0 } }</option> }) }
                    </select>
                </label>
            </div>
            <label class="block">{ l_code }
                <textarea rows="14" spellcheck="false" autocapitalize="off" autocomplete="off"
                    value={(*code).clone()} oninput={on_input} />
            </label>
            <button type="button" onclick={on_run}>{ l_run }</button>
            <h2 id="out-h">{ l_out }</h2>
            <pre role="status" aria-labelledby="out-h" class={ if matches!(*result, Some(Err(_))) { "error" } else { "" } }>
                { match &*result { Some(Ok(s)) | Some(Err(s)) => s.clone(), None => String::new() } }
            </pre>
            <p class="note">{
                if sw { "Uwanja huonyesha thamani ya mstari wa mwisho wa programu. Tumia mlolongo_andika, si andika." }
                else { "The playground shows the value of the program's last line. Use sprint rather than print." }
            }</p>
        </main>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples_run() {
        let want = [
            ("Hello, World!", "Habari Dunia!"),
            ("fizzbuzz result for 45 is FizzBuzz", "matokeo ya fizzbuzz kwa 45 ni FizzBuzz yenyewe"),
            ("Naive solution: 6765, memoized solution: 6765", "Matokeo ya utekelezaji jinga: 6765, wa kumbukumbu: 6765"),
        ];
        for (i, (en, sw)) in want.iter().enumerate() {
            assert_eq!(run("en", sample("en", i)).as_deref(), Ok(*en));
            assert_eq!(run("sw", sample("sw", i)).as_deref(), Ok(*sw));
        }
        assert!(run("en", "sprint missing").is_err());
    }
}
