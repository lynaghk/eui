use leptos::logging::log;
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

#[component]
pub fn render_control(ty: &'static eui::schema::Type) -> impl IntoView {
    log!("{:?}", ty);

    use eui::schema::Type::*;
    use eui::schema::{Field, NamedVariant};
    match ty {
        Bool => todo!(),
        U8 => {
            view! {
                            <div class="type" data-type="u8">
            <input type="number" min="0" max="255" on:input=move |e| {
            log!("{:?}", event_target_value(&e))
            }/>
            </div>
                        }
        }
        U16 => todo!(),
        U32 => todo!(),
        U64 => todo!(),
        I8 => todo!(),
        I16 => todo!(),
        I32 => todo!(),
        I64 => todo!(),
        F32 => todo!(),
        F64 => todo!(),
        String => todo!(),
        ByteArray => todo!(),
        Option(_) => todo!(),
        Unit | UnitVariant | UnitStruct => {
            view! { <div class="type" data-type="a-unit"></div> }
        }
        NewtypeStruct(_) => todo!(),
        NewtypeVariant(_) => todo!(),
        Seq(_) => todo!(),
        Tuple(_) => todo!(),
        TupleStruct(_) => todo!(),
        TupleVariant(variants) => {
            view! { <div class="type" data-type="tuple-variant">
                     { variants.into_iter().map(|ty| view!{<RenderControl ty=ty />}

                     ).collect_view()

                     }
                     </div>


            }
        }
        Map { key, val } => todo!(),
        Struct(fields) => {
            view! { <div class="type" data-type="struct">
                     { fields.into_iter().map(|Field{name, ty}| view!{                       <div class="type" data-type="field">
                                                                                              <label> {name.to_string()} </label>
                                                                                              <RenderControl ty=ty />
                                                                                              </div>}

                     ).collect_view()

                     }
                     </div>


            }
        }
        StructVariant(_) => todo!(),
        Enum(variants) => {
            view! { <div class="type" data-type="enum">
                     { variants.into_iter().map(|NamedVariant{name, ty}| {
                         view!{
                             <div class="type" data-type="named-variant">
                                 <label> {name.to_string()} </label>
                                 <RenderControl ty=ty />
                                 </div>
                         }}).collect_view()

                     }
                     </div>


            }
        }
        NamedType { name, ty } => {
            view! { <div class="type" data-type="named-type">
                     <label> {name.to_string()} </label>
                     <RenderControl ty=ty />
                     </div>
            }
        }
    }
}

pub fn main() {
    use eui::schema::Schema;

    leptos::logging::log!("{:?}", leptos_test::Light::SCHEMA);

    mount_to_body(|| view! { <RenderControl ty=leptos_test::Light::SCHEMA  /> });

    // mount_to_body(|| {
    //     view! {
    //         <SimpleCounter initial_value=3 />
    //     }
    // })
}
