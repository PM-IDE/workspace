use crate::ficus_proto::GrpcPipelineStreamingConfiguration;
use crate::grpc::kafka::streaming::processors::TracesProcessor;
use crate::grpc::kafka::streaming::t1::configs::T1StreamingConfiguration;
use crate::grpc::kafka::streaming::t1::filterers::T1LogFilterer;
use crate::grpc::kafka::streaming::t1::processors::T1StreamingProcessor;
use crate::grpc::kafka::streaming::t2::configs::T2StreamingConfiguration;

type StreamingConfigurationEnum = crate::ficus_proto::grpc_pipeline_streaming_configuration::Configuration;

pub enum StreamingConfiguration {
    NotSpecified,
    T1(T1StreamingConfiguration),
    T2(T2StreamingConfiguration),
}

impl StreamingConfiguration {
    pub fn new(grpc_config: &GrpcPipelineStreamingConfiguration) -> Option<Self> {
        match grpc_config.configuration.as_ref() {
            None => None,
            Some(c) => match c {
                StreamingConfigurationEnum::NotSpecified(_) => Some(StreamingConfiguration::NotSpecified),
                StreamingConfigurationEnum::T1Configuration(t1) => match T1StreamingConfiguration::new(t1) {
                    None => None,
                    Some(t1) => Some(StreamingConfiguration::T1(t1)),
                },
                StreamingConfigurationEnum::T2Configuration(t2) => match T2StreamingConfiguration::new(t2) {
                    None => None,
                    Some(t2) => Some(StreamingConfiguration::T2(t2)),
                },
            },
        }
    }

    pub fn create_processor(&self) -> TracesProcessor {
        match self {
            StreamingConfiguration::NotSpecified => TracesProcessor::T1(T1StreamingProcessor::new(T1LogFilterer::None)),
            StreamingConfiguration::T1(c) => TracesProcessor::T1(c.create_processor()),
            StreamingConfiguration::T2(_) => todo!(),
        }
    }
}
