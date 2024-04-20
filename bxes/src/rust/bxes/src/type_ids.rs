use variant_count::VariantCount;

#[derive(FromPrimitive, ToPrimitive, VariantCount, Debug, PartialEq, Eq)]
pub enum TypeIds {
    Null = 0,
    I32 = 1,
    I64 = 2,
    U32 = 3,
    U64 = 4,
    F32 = 5,
    F64 = 6,
    String = 7,
    Bool = 8,
    Timestamp = 9,
    BrafLifecycle = 10,
    StandardLifecycle = 11,
    Artifact = 12,
    Drivers = 13,
    Guid = 14,
    SoftwareEventType = 15,
}
