use serde_derive::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum Type {
    Bool,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    String,
    ByteArray,
    Option(&'static Type),

    Unit,
    UnitStruct,
    UnitVariant,

    Tuple(&'static [&'static Type]),
    TupleStruct(&'static [&'static Type]),
    TupleVariant(&'static [&'static Type]),

    Struct(&'static [&'static Field]),
    StructVariant(&'static [&'static Field]),
    Enum(&'static [&'static NamedVariant]),

    Seq(&'static Type),
    // Map {
    //     key: &'static Type,
    //     val: &'static Type,
    // },
    NamedType {
        name: &'static str,
        ty: &'static Type,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Field {
    pub name: &'static str,
    pub ty: &'static Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct NamedVariant {
    pub name: &'static str,
    pub ty: &'static Type,
}

/// A trait that represents a compile time calculated schema
pub trait Schema {
    const SCHEMA: &'static Type;
}

macro_rules! impl_schema {
    ($($t:ty: $type:expr),*) => {
        $(
            impl Schema for $t {
                const SCHEMA: &'static Type = &$type;
            }
        )*
    };

    (tuple => [$(($($generic:ident),*)),*]) => {
        $(
            impl<$($generic: Schema),*> Schema for ($($generic,)*) {
                const SCHEMA: &'static Type = &Type::Tuple(&[$($generic::SCHEMA),*]);
            }
        )*
    };
}

impl_schema![
    u8: Type::U8,
    u32: Type::U32,
    u64: Type::U64,
    i8: Type::I8,
    i32: Type::I32,
    i64: Type::I64,
    bool: Type::Bool,
    f32: Type::F32,
    f64: Type::F64,
    str: Type::String,
    (): Type::Unit
];

impl_schema!(tuple => [
    (A),
    (A, B),
    (A, B, C),
    (A, B, C, D),
    (A, B, C, D, E),
    (A, B, C, D, E, F)
]);

impl<T: Schema> Schema for Option<T> {
    const SCHEMA: &'static Type = &Type::Option(T::SCHEMA);
}

impl<T: Schema, E: Schema> Schema for Result<T, E> {
    const SCHEMA: &'static Type = &Type::NamedType {
        name: "Result<T, E>",
        ty: &Type::Enum(&[
            &NamedVariant {
                name: "Ok",
                ty: &Type::TupleVariant(&[T::SCHEMA]),
            },
            &NamedVariant {
                name: "Err",
                ty: &Type::TupleVariant(&[E::SCHEMA]),
            },
        ]),
    };
}

impl<T: Schema> Schema for &'_ T {
    const SCHEMA: &'static Type = T::SCHEMA;
}

impl<T: Schema> Schema for [T] {
    const SCHEMA: &'static Type = &Type::Seq(T::SCHEMA);
}

impl<T: Schema, const N: usize> Schema for [T; N] {
    const SCHEMA: &'static Type = &Type::Tuple(&[T::SCHEMA; N]);
}

impl<T: Schema> Schema for std::vec::Vec<T> {
    const SCHEMA: &'static Type = &Type::Seq(T::SCHEMA);
}

impl Schema for std::string::String {
    const SCHEMA: &'static Type = &Type::String;
}
