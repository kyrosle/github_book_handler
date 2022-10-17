use dioxus::prelude::*;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let value = use_state(&cx, || String::from("hello world!"));
    cx.render(rsx! {
        input {
            oninput : move |evt| value.set(evt.value.clone()),
            value: "{value}"
        }
    })
}
