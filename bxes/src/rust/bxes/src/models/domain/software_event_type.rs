use variant_count::VariantCount;

#[derive(FromPrimitive, ToPrimitive, VariantCount, Clone, Debug, Hash, PartialEq, Eq)]
pub enum SoftwareEventType {
    Unspecified = 0,
    Call = 1,
    Return = 2,
    Throws = 3,
    Handle = 4,
    Calling = 5,
    Returning = 6,
}
