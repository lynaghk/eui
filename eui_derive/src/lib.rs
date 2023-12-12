// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{
//     parse_macro_input, AngleBracketedGenericArguments, Data, DeriveInput, GenericArgument, Path,
//     PathArguments, PathSegment, Type, TypePath,
// };

// #[proc_macro_derive(Reflect)]
// pub fn reflect(input: TokenStream) -> TokenStream {
//     // 1. Use syn to parse the input tokens into a syntax tree.
//     // 2. Use quote to generate new tokens based on what we parsed.
//     // 3. Return the generated tokens.
//     let DeriveInput {
//         ident: struct_name,
//         data,
//         ..
//     } = parse_macro_input!(input as DeriveInput);

//     let Data::Struct(struct_data) = data else {
//         unimplemented!("enums");
//     };

//     let segment_to_string = |s: &PathSegment| {
//         let ident = s.ident.to_string();

//         match (ident.as_str(), &s.arguments) {
//             (
//                 "Result" | "Option",
//                 PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }),
//             ) => match args.first() {
//                 Some(GenericArgument::Type(Type::Path(TypePath {
//                     path: Path { segments, .. },
//                     ..
//                 }))) => vec![ident, segments[0].ident.to_string()],
//                 _ => unreachable!(),
//             },
//             (_, _) => vec![ident],
//         }
//     };

//     let mut fields = vec![];
//     for f in struct_data.fields {
//         match f.ty {
//             syn::Type::Path(t) => fields.push((
//                 f.ident.unwrap().to_string(),
//                 segment_to_string(&t.path.segments[0]),
//             )),
//             _ => unreachable!(),
//         }
//     }

//     let schema = serde_json::to_string(&fields).unwrap();

//     let ex = quote! {
//       impl #struct_name {
//         const SCHEMA: &'static str = #schema;
//       }
//     };

//     ex.into()
// }

mod schema;

#[proc_macro_derive(Schema)]
pub fn derive_schema(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    schema::do_derive_schema(item)
}
