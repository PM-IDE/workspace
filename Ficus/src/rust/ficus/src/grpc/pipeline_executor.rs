use crate::ficus_proto::grpc_pipeline_part_base::Part;
use crate::ficus_proto::{GrpcContextKeyValue, GrpcGuid, GrpcPipeline, GrpcPipelinePart};
use crate::grpc::backend_service::GrpcSender;
use crate::grpc::converters::put_into_user_data;
use crate::grpc::get_context_pipeline::GetContextValuePipelinePart;
use crate::grpc::logs_handler::{ConsoleLogMessageHandler, DelegatingLogMessageHandler, GrpcLogMessageHandlerImpl};
use crate::pipelines::context::{LogMessageHandler, PipelineContext, PipelineInfrastructure};
use crate::pipelines::errors::pipeline_errors::PipelinePartExecutionError;
use crate::pipelines::keys::context_keys::find_context_key;
use crate::pipelines::pipeline_parts::PipelineParts;
use crate::pipelines::pipelines::{DefaultPipelinePart, Pipeline, PipelinePart};
use crate::utils::user_data::user_data::UserDataImpl;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

pub(super) struct ServicePipelineExecutionContext<'a> {
    grpc_pipeline: &'a GrpcPipeline,
    context_values: &'a Vec<GrpcContextKeyValue>,
    pipeline_parts: Arc<Box<PipelineParts>>,
    sender: Arc<Box<GrpcSender>>,
    log_message_handler: Arc<Box<dyn LogMessageHandler>>,
}

impl<'a> ServicePipelineExecutionContext<'a> {
    pub fn new(
        grpc_pipeline: &'a GrpcPipeline,
        context_values: &'a Vec<GrpcContextKeyValue>,
        pipeline_parts: Arc<Box<PipelineParts>>,
        sender: GrpcSender,
    ) -> Self {
        let sender = Arc::new(Box::new(sender));
        let log_message_handler = Self::create_log_message_handler(sender.clone());

        Self {
            grpc_pipeline,
            context_values,
            pipeline_parts,
            sender,
            log_message_handler,
        }
    }

    fn create_log_message_handler(sender: Arc<Box<GrpcSender>>) -> Arc<Box<dyn LogMessageHandler>> {
        let grpc_handler = GrpcLogMessageHandlerImpl::new(sender.clone());
        let grpc_handler = Box::new(grpc_handler) as Box<dyn LogMessageHandler>;

        let console_handler = ConsoleLogMessageHandler::new();
        let console_handler = Box::new(console_handler) as Box<dyn LogMessageHandler>;

        let delegating_handler = DelegatingLogMessageHandler::new(vec![grpc_handler, console_handler]);

        Arc::new(Box::new(delegating_handler) as Box<dyn LogMessageHandler>)
    }

    pub fn sender(&self) -> Arc<Box<GrpcSender>> {
        self.sender.clone()
    }

    pub fn grpc_pipeline(&self) -> &GrpcPipeline {
        &self.grpc_pipeline
    }

    pub fn parts(&self) -> &PipelineParts {
        &self.pipeline_parts
    }

    pub fn context_values(&self) -> &Vec<GrpcContextKeyValue> {
        &self.context_values
    }

    pub fn log_message_handler(&self) -> Arc<Box<dyn LogMessageHandler>> {
        self.log_message_handler.clone()
    }

    pub fn with_pipeline(&self, new_grpc_pipeline: &'a GrpcPipeline) -> Self {
        Self {
            grpc_pipeline: new_grpc_pipeline,
            context_values: self.context_values,
            pipeline_parts: self.pipeline_parts.clone(),
            sender: self.sender.clone(),
            log_message_handler: self.log_message_handler.clone(),
        }
    }

    pub fn execute_grpc_pipeline(&self) -> Result<(GrpcGuid, UserDataImpl), PipelinePartExecutionError> {
        let id = Uuid::new_v4();
        let pipeline = self.to_pipeline();
        let mut pipeline_context = self.create_initial_context();
        let infra = PipelineInfrastructure::new(Some(self.log_message_handler()));

        match pipeline.execute(&mut pipeline_context, &infra) {
            Ok(()) => Ok((GrpcGuid { guid: id.to_string() }, pipeline_context.devastate_user_data())),
            Err(err) => Err(err),
        }
    }

    pub(super) fn to_pipeline(&self) -> Pipeline {
        let mut pipeline = Pipeline::empty();
        for grpc_part in &self.grpc_pipeline().parts {
            match grpc_part.part.as_ref().unwrap() {
                Part::DefaultPart(grpc_default_part) => match self.find_default_part(grpc_default_part) {
                    Some(found_part) => {
                        pipeline.push(found_part);
                    }
                    None => todo!(),
                },
                Part::ParallelPart(_) => todo!(),
                Part::SimpleContextRequestPart(part) => {
                    let key_name = part.key.as_ref().unwrap().name.clone();
                    let uuid = Uuid::from_str(&part.frontend_part_uuid.as_ref().unwrap().uuid).ok().unwrap();

                    pipeline.push(Self::create_get_context_part(vec![key_name], uuid, &self.sender(), None));
                }
                Part::ComplexContextRequestPart(part) => {
                    let grpc_default_part = part.before_pipeline_part.as_ref().unwrap();
                    let uuid = Uuid::from_str(&part.frontend_part_uuid.as_ref().unwrap().uuid).ok().unwrap();

                    match self.find_default_part(grpc_default_part) {
                        Some(found_part) => {
                            let key_names = part.keys.iter().map(|x| x.name.to_owned()).collect();
                            pipeline.push(Self::create_get_context_part(key_names, uuid, &self.sender(), Some(found_part)));
                        }
                        None => todo!(),
                    }
                }
            }
        }

        pipeline
    }

    fn create_get_context_part(
        key_names: Vec<String>,
        uuid: Uuid,
        sender: &Arc<Box<GrpcSender>>,
        before_part: Option<Box<DefaultPipelinePart>>,
    ) -> Box<GetContextValuePipelinePart> {
        let sender = sender.clone();
        GetContextValuePipelinePart::create_context_pipeline_part(key_names, uuid, sender, before_part)
    }

    fn find_default_part(&self, grpc_default_part: &GrpcPipelinePart) -> Option<Box<DefaultPipelinePart>> {
        let mut part_config = UserDataImpl::new();
        let grpc_config = &grpc_default_part.configuration.as_ref().unwrap();

        for conf_value in &grpc_config.configuration_parameters {
            let key_name = conf_value.key.as_ref().unwrap().name.as_ref();
            if let Some(key) = find_context_key(key_name) {
                let value = conf_value.value.as_ref().unwrap().context_value.as_ref().unwrap();
                put_into_user_data(key.key(), value, &mut part_config, &self);
            }
        }

        match self.parts().find_part(&grpc_default_part.name) {
            Some(default_part) => Some(Box::new(default_part(Box::new(part_config)))),
            None => None,
        }
    }

    pub(super) fn create_initial_context(&'a self) -> PipelineContext<'a> {
        let mut pipeline_context = PipelineContext::new_with_logging(self.parts());

        for value in self.context_values() {
            let key = find_context_key(&value.key.as_ref().unwrap().name).unwrap();
            let value = value.value.as_ref().unwrap().context_value.as_ref().unwrap();
            put_into_user_data(key.key(), value, &mut pipeline_context, self);
        }

        pipeline_context
    }
}
