use std::fmt::Debug;
use getset::Getters;

#[derive(Clone, Debug, Getters)]
pub struct SoftwareDataExtractionInfo {
  #[getset(get="pub")] allocation: ExtractionInfo<AllocationExtractionInfo>,
  #[getset(get="pub")] exceptions: ExtractionInfo<ExceptionExtractionInfo>,
  #[getset(get="pub")] http: ExtractionInfo<HTTPExtractionInfo>,

  #[getset(get="pub")] method_inlining_success: ExtractionInfo<()>,
  #[getset(get="pub")] method_inlining_failed: ExtractionInfo<MethodInliningFailedExtractionInfo>,

  #[getset(get="pub")] contention: ExtractionInfo<ContentionExtractionInfo>,
  #[getset(get="pub")] socket: ExtractionInfo<SocketExtractionInfo>,
  #[getset(get="pub")] thread: ExtractionInfo<ThreadExtractionInfo>,

  #[getset(get="pub")] array_pool_array_created: ExtractionInfo<ArrayPoolExtractionInfo>,
  #[getset(get="pub")] array_pool_array_rented: ExtractionInfo<ArrayPoolExtractionInfo>,
  #[getset(get="pub")] array_pool_array_returned: ExtractionInfo<ArrayPoolExtractionInfo>,
  #[getset(get="pub")] array_pool_array_trimmed: ExtractionInfo<ArrayPoolExtractionInfo>,
  
  #[getset(get="pub")] assembly_load: ExtractionInfo<AssemblyExtractionInfo>,
  #[getset(get="pub")] assembly_unload: ExtractionInfo<AssemblyExtractionInfo>,
}

#[derive(Clone, Debug, Getters)]
pub struct ExtractionInfo<TConcreteInfo: Clone + Debug> {
  #[getset(get="pub")] event_class_regex: String,
  #[getset(get="pub")] info: TConcreteInfo,
}

#[derive(Clone, Debug, Getters)]
pub struct AllocationExtractionInfo {
  #[getset(get="pub")] type_name_attr: String,
  #[getset(get="pub")] allocated_count_attr: String,
  #[getset(get="pub")] object_size_attr: String
}

#[derive(Clone, Debug, Getters)]
pub struct ExceptionExtractionInfo {
  #[getset(get="pub")] type_name_attr: String
}

#[derive(Clone, Debug, Getters)]
pub struct HTTPExtractionInfo {
  #[getset(get="pub")] host_attr: String,
  #[getset(get="pub")] port_attr: String,
  #[getset(get="pub")] scheme_attr: String,
  #[getset(get="pub")] path_attr: String,
  #[getset(get="pub")] query_attr: String,
}

#[derive(Clone, Debug)]
pub struct MethodInliningSuccessExtractionInfo {}

#[derive(Clone, Debug, Getters)]
pub struct MethodInliningFailedExtractionInfo {
  #[getset(get="pub")] reason_attr: String
}

#[derive(Clone, Debug)]
pub struct ContentionExtractionInfo {}

#[derive(Clone, Debug, Getters)]
pub struct SocketExtractionInfo {
  #[getset(get="pub")] address_attr: String
}

#[derive(Clone, Debug, Getters)]
pub struct ThreadExtractionInfo {
  #[getset(get="pub")] thread_id_attr: String
}

#[derive(Clone, Debug, Getters)]
pub struct ArrayPoolExtractionInfo {
  #[getset(get="pub")] buffer_id: String
}

#[derive(Clone, Debug, Getters)]
pub struct AssemblyExtractionInfo {
  name_attr: String
}