use crate::ficus_proto::grpc_t2_streaming_configuration::Configuration;
use crate::ficus_proto::GrpcT2StreamingConfiguration;

pub enum T2StreamingConfiguration {
    LossyCount(LossyCountConfiguration),
}

impl T2StreamingConfiguration {
    pub fn new(grpc_config: &GrpcT2StreamingConfiguration) -> Option<Self> {
        match grpc_config.configuration.as_ref() {
            None => None,
            Some(c) => Some(match c {
                Configuration::LossyCount(lc) => T2StreamingConfiguration::LossyCount(LossyCountConfiguration {
                    error: lc.error,
                    support: lc.support,
                }),
            }),
        }
    }
}

pub struct LossyCountConfiguration {
    error: f64,
    support: f64,
}
