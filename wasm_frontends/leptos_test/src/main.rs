use leptos::*;

// #[component]
// pub fn SimpleCounter(
//     /// The starting value for the counter
//     initial_value: i32,
//     /// The change that should be applied each time the button is clicked.
//     step: i32,
// ) -> impl IntoView {
//     let (value, set_value) = create_signal(initial_value);

//     view! {
//         <div>
//             <button on:click=move |_| set_value.set(0)>"Clear"</button>
//             <button on:click=move |_| set_value.update(|value| *value -= step)>"-1"</button>
//             <span>"Value: " {value} "!"</span>
//             <button on:click=move |_| set_value.update(|value| *value += step)>"+1"</button>
//         </div>
//     }
// }

// Easy to use with Trunk (trunkrs.dev) or with a simple wasm-bindgen setup
pub fn main() {
    leptos::logging::log!("hello");
    mount_to_body(|| view! { <p>"Hello, world!"</p> });

    // mount_to_body(|| {
    //     view! {
    //         <SimpleCounter initial_value=3 />
    //     }
    // })
}
