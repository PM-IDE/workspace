use super::{braf_lifecycle::XesBrafLifecycle, standard_lifecycle::XesStandardLifecycle};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Lifecycle {
    XesStandardLifecycle(XesStandardLifecycle),
    BrafLifecycle(XesBrafLifecycle),
}

impl ToString for Lifecycle {
    fn to_string(&self) -> String {
        match self {
            Self::XesStandardLifecycle(xes_lifecycle) => xes_lifecycle.to_string(),
            Self::BrafLifecycle(braf_lifecycle) => braf_lifecycle.to_string(),
        }
    }
}
