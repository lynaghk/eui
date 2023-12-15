use leptos::logging::log;
use leptos::*;

use eui::bevy_reflect::{self, Reflect, TypeInfo, Typed, *};

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
pub fn render_control(
    ty: &'static TypeInfo,
    #[prop(into)] cb: Callback<Box<dyn Reflect>>,
) -> impl IntoView {
    log!("{:?}", ty);

    match ty {
        TypeInfo::Struct(info) => {
            // view! { <div class="type" data-type="struct">
            //         { (0..info.field_len()).iter().map(|idx| {

            // view!{                       <div class="type" data-type="field">
            //                                                                                               <label> {field.name()} </label>
            //                                                                                               // <RenderControl ty=ty cb=cb />
            //                                                                                               </div>}
            // }
            //                      ).collect_view()

            //                      }
            //                      </div>
            //             }
            todo!();
        }
        TypeInfo::TupleStruct(_) => todo!(),
        TypeInfo::Tuple(_) => todo!(),
        TypeInfo::List(_) => todo!(),
        TypeInfo::Array(_) => todo!(),
        TypeInfo::Map(_) => todo!(),
        TypeInfo::Enum(info) => {
            // let vview = |variant: &VariantInfo| {
            //     let variant = variant.clone();
            //     let name = variant.name();
            //     let click = move |_| {
            //         let name = variant.name();
            //         log!("{:?}", variant);
            //     view! { <button on:click = click> {name} </button> }
            //         match &variant {
            //             VariantInfo::Unit(_) => {
            //                 let x = UnitVariantInfo::new(&name);
            //                 log!("{:?}", x);

            //             }
            //             VariantInfo::Struct(_) => todo!(),
            //             VariantInfo::Tuple(info) => {
            //                 // let x = TupleVariantInfo::new(&name);
            //                 // log!("{:?}", x);
            //             }
            //         };
            //     };
            // };

            // view! {
            //             <div class="type" data-type="enum">
            //               { info.iter().map(vview).collect_view() }
            //             </div>
            // }
            todo!()
        }
        TypeInfo::Value(info) => {
            view! {
                            <div class="type" data-type="u8">
            <input type="number" min="0" max="255" on:input=move |e| {
            log!("{:?}", event_target_value(&e))
            }/>
            </div>
                        }
        }
    }

    // use eui::schema::Type::*;
    // use eui::schema::{Field, NamedVariant};
    // match ty {
    //     Bool => todo!(),
    //     U8 => {
    //         view! {
    //                         <div class="type" data-type="u8">
    //         <input type="number" min="0" max="255" on:input=move |e| {
    //         log!("{:?}", event_target_value(&e))
    //         }/>
    //         </div>
    //                     }
    //     }
    //     U16 => todo!(),
    //     U32 => todo!(),
    //     U64 => todo!(),
    //     I8 => todo!(),
    //     I16 => todo!(),
    //     I32 => todo!(),
    //     I64 => todo!(),
    //     F32 => todo!(),
    //     F64 => todo!(),
    //     String => todo!(),
    //     ByteArray => todo!(),
    //     Option(_) => todo!(),
    //     Unit | UnitVariant | UnitStruct => {
    //         view! { <div class="type" data-type="a-unit"></div> }
    //     }
    //     NewtypeStruct(_) => todo!(),
    //     NewtypeVariant(_) => todo!(),
    //     Seq(_) => todo!(),
    //     Tuple(_) => todo!(),
    //     TupleStruct(_) => todo!(),
    //     TupleVariant(variants) => {
    //         view! { <div class="type" data-type="tuple-variant">
    //                  { variants.into_iter().map(|ty| view!{<RenderControl ty=ty />}

    //                  ).collect_view()

    //                  }
    //                  </div>

    //         }
    //     }
    //     Map { key, val } => todo!(),
    //     Struct(fields) => {
    //         view! { <div class="type" data-type="struct">
    //                  { fields.into_iter().map(|Field{name, ty}| view!{                       <div class="type" data-type="field">
    //                                                                                           <label> {name.to_string()} </label>
    //                                                                                           <RenderControl ty=ty />
    //                                                                                           </div>}

    //                  ).collect_view()

    //                  }
    //                  </div>

    //         }
    //     }
    //     StructVariant(_) => todo!(),
    //     Enum(variants) => {
    //         view! { <div class="type" data-type="enum">
    //                  { variants.into_iter().map(|NamedVariant{name, ty}| {
    //                      view!{
    //                          <div class="type" data-type="named-variant">
    //                              <label> {name.to_string()} </label>
    //                              <RenderControl ty=ty />
    //                              </div>
    //                      }}).collect_view()

    //                  }
    //                  </div>

    //         }
    //     }
    //     NamedType { name, ty } => {
    //         view! { <div class="type" data-type="named-type">
    //                  <label> {name.to_string()} </label>
    //                  <RenderControl ty=ty />
    //                  </div>
    //         }
    //     }
    // }
}

#[component]
pub fn render_control_dyn(
    x: Box<dyn Reflect>,
    #[prop(into)] cb: Callback<Box<dyn Reflect>>,
) -> impl IntoView {
    let ty = x.get_represented_type_info().unwrap();
    log!("{:?}", ty);
    // match ty {
    //     TypeInfo::Struct(info) => {
    //         let s = x.clone_value();
    //         s.set_represented_type(&ty);

    //         let idx = 0;
    //         view! {
    //         <div>
    //         </div>
    //         }
    //     }
    //     TypeInfo::TupleStruct(_) => todo!(),
    //     TypeInfo::Tuple(_) => todo!(),
    //     TypeInfo::List(_) => todo!(),
    //     TypeInfo::Array(_) => todo!(),
    //     TypeInfo::Map(_) => todo!(),
    //     TypeInfo::Enum(_) => todo!(),
    //     TypeInfo::Value(info) => view! {<div>{format!("{:?}", info)}</div>},
    // }

    match x.reflect_owned() {
        ReflectOwned::Struct(s) => {
            let field_names = match ty {
                TypeInfo::Struct(info) => info.field_names(),
                _ => unreachable!(),
            };

            let field_views = (0..field_names.len())
                .map(move |idx| {
                    let f = s.field_at(idx).unwrap().clone_value();
                    let s = s.clone_dynamic();
                    let local = move |x| {
                        let mut s = s.clone_dynamic();
                        let f = s.field_at_mut(idx).unwrap();
                        f.set(x).unwrap();
                        cb.call(Box::new(s));
                    };
                    view! {
                                <div class="field">
                    { field_names[idx] } <RenderControlDyn x=f cb=local />
                                </div>
                                }
                })
                .collect_view();
            view! {
            <div class="type" data-type="struct">
            {field_views}
            </div>
            }
        }
        ReflectOwned::TupleStruct(_) => todo!(),
        ReflectOwned::Tuple(_) => todo!(),
        ReflectOwned::List(_) => todo!(),
        ReflectOwned::Array(_) => todo!(),
        ReflectOwned::Map(_) => todo!(),
        ReflectOwned::Enum(_) => todo!(),
        ReflectOwned::Value(v) => match v.get_represented_type_info().unwrap().type_path() {
            "u64" => {
                view! {
                                <div class="type" data-type="u64">
                                <input type="number" min="0" max="255" on:input=move |e| {
                if let Ok(new) = event_target_value(&e).parse::<u64>(){
                                  cb.call(Box::new(new));
                }

                                                                 }/>
                                                                 </div>
                                                                             }
            }

            unknown => view! { <div> "Unknown type: " { unknown } </div> },
        },
    }
}

pub fn main() {
    // use eui::schema::Schema;
    // log!("{:?}", leptos_test::Light::SCHEMA);

    //let ty = leptos_test::Light::type_info();

    #[derive(Debug, Reflect, Clone)]
    struct Foo {
        x: u64,
        y: u64,
    }

    // let ty = Foo::type_info();
    //mount_to_body(move || view! { <RenderControl ty=ty cb=cb /> });

    mount_to_body(move || {
        let x = Box::new(Foo { x: 100, y: 10 });
        let (value, set_value) = create_signal(x);

        let cb = move |x: Box<dyn Reflect>| {
            let new = Foo::from_reflect(&*x).unwrap();
            log!("toplevel got: {:?}", new);
            set_value.set(Box::new(new));
        };

        view! { <RenderControlDyn x=value.get() cb=cb /> }
    });

    // mount_to_body(|| {
    //     view! {
    //         <SimpleCounter initial_value=3 />
    //     }
    // })
}
