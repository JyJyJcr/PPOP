use std::any::TypeId;

#[allow(non_camel_case_types)]
pub enum SpecializedTypeId {
    u8,
    String,
    u64,
    i64,
    f64,
    usize,
    isize,
    Other(TypeId),
}

impl From<TypeId> for SpecializedTypeId {
    fn from(value: TypeId) -> Self {
        match value {
            value if value == TypeId::of::<u8>() => Self::u8,
            value if value == TypeId::of::<String>() => Self::String,
            value if value == TypeId::of::<u64>() => Self::u64,
            value if value == TypeId::of::<i64>() => Self::i64,
            value if value == TypeId::of::<f64>() => Self::f64,
            value if value == TypeId::of::<usize>() => Self::usize,
            value if value == TypeId::of::<isize>() => Self::isize,
            value => Self::Other(value),
        }
    }
}
