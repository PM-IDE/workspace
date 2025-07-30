use chrono::Utc;
use ficus::event_log::core::event::event::EventPayloadValue;
use ficus::event_log::xes::xes_event::XesEventImpl;
use ficus::features::discovery::timeline::software_data::extraction_config::{AllocationExtractionConfig, ArrayPoolExtractionConfig, AssemblyExtractionConfig, ExceptionExtractionConfig, ExtractionConfig, HTTPExtractionConfig, MethodCommonAttributesConfig, MethodInliningConfig, MethodInliningFailedConfig, MethodInliningSucceededConfig, MethodLoadUnloadConfig, NameCreationStrategy, PieChartExtractionConfig, SimpleCountExtractionConfig, SingleAttribute, SocketAcceptConnectFailedConfig, SocketConnectAcceptStartConfig, SoftwareDataExtractionConfig, ThreadExtractionConfig};
use ficus::features::discovery::timeline::software_data::extractors::allocations::AllocationDataExtractor;
use ficus::features::discovery::timeline::software_data::extractors::array_pools::ArrayPoolDataExtractor;
use ficus::features::discovery::timeline::software_data::extractors::assemblies::AssemblySoftwareDataExtractor;
use ficus::features::discovery::timeline::software_data::extractors::core::SoftwareDataExtractor;
use ficus::features::discovery::timeline::software_data::extractors::exceptions::ExceptionDataExtractor;
use ficus::features::discovery::timeline::software_data::extractors::general::pie_chart_extractor::PieChartExtractor;
use ficus::features::discovery::timeline::software_data::extractors::general::simple_counter::SimpleCounterExtractor;
use ficus::features::discovery::timeline::software_data::extractors::http::HTTPSoftwareDataExtractor;
use ficus::features::discovery::timeline::software_data::extractors::methods::MethodsDataExtractor;
use ficus::features::discovery::timeline::software_data::extractors::sockets::SocketsDataExtractor;
use ficus::features::discovery::timeline::software_data::extractors::threads::ThreadDataExtractor;
use ficus::features::discovery::timeline::software_data::models::SoftwareData;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_allocations_extraction_1() {
  execute_test_with_software_data(
    r#"{"allocation_events":[{"type_name":"xd","objects_count":32,"allocated_bytes":1234}]}"#,
    || {
      let events = [
        create_event_with_attributes("Alloc".to_string(), vec![
          ("count".to_string(), EventPayloadValue::Int32(32)),
          ("type".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd".to_string())))),
          ("total".to_string(), EventPayloadValue::Int64(1234)),
        ]),
        create_event_with_attributes("All1oc".to_string(), vec![
          ("count".to_string(), EventPayloadValue::Int32(32)),
          ("type".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd".to_string())))),
          ("total".to_string(), EventPayloadValue::Int64(1234)),
        ])
      ];

      let mut config = SoftwareDataExtractionConfig::empty();
      let alloc_config = AllocationExtractionConfig::new("type".to_string(), "count".to_string(), None, Some("total".to_string()));
      config.set_allocation(Some(ExtractionConfig::new("Alloc".to_string(), alloc_config)));

      let extractor = AllocationDataExtractor::new(&config);
      let mut software_data = SoftwareData::empty();
      extractor.extract_from_events(&mut software_data, &events).ok().unwrap();

      software_data
    },
  )
}

#[test]
fn test_allocations_extraction_2() {
  execute_test_with_software_data(
    r#"{"allocation_events":[{"type_name":"xd","objects_count":32,"allocated_bytes":39488}]}"#,
    || {
      let events = [
        create_event_with_attributes("Alloc".to_string(), vec![
          ("count".to_string(), EventPayloadValue::Int32(32)),
          ("type".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd".to_string())))),
          ("total".to_string(), EventPayloadValue::Int64(1234)),
        ]),
        create_event_with_attributes("Al123loc".to_string(), vec![
          ("count".to_string(), EventPayloadValue::Int32(32)),
          ("type".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd".to_string())))),
          ("total".to_string(), EventPayloadValue::Int64(1234)),
        ])
      ];

      let mut config = SoftwareDataExtractionConfig::empty();
      let alloc_config = AllocationExtractionConfig::new("type".to_string(), "count".to_string(), Some("total".to_string()), None);
      config.set_allocation(Some(ExtractionConfig::new("Alloc".to_string(), alloc_config)));

      let extractor = AllocationDataExtractor::new(&config);
      let mut software_data = SoftwareData::empty();
      extractor.extract_from_events(&mut software_data, &events).ok().unwrap();

      software_data
    },
  )
}

#[test]
fn test_inlining_success_extraction() {
  execute_test_with_software_data(
    r#"{"method_inlinings_events":[{"InliningSuccess":{"inlinee_info":{"name":"xd1","namespace":"xd2","signature":"xd3"},"inliner_info":{"name":"xd4","namespace":"xd5","signature":"xd6"}}}]}"#,
    || {
      let events = [
        create_event_with_attributes("Inlining".to_string(), vec![
          ("m1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd1".to_string())))),
          ("n1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd2".to_string())))),
          ("s1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd3".to_string())))),
          ("m2".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd4".to_string())))),
          ("n2".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd5".to_string())))),
          ("s2".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd6".to_string())))),
        ]),
        create_event_with_attributes("Inli123ning".to_string(), vec![
          ("m1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd1".to_string())))),
          ("n1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd2".to_string())))),
          ("s1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd3".to_string())))),
          ("m2".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd4".to_string())))),
          ("n2".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd5".to_string())))),
          ("s2".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd6".to_string())))),
        ]),
      ];

      let mut config = SoftwareDataExtractionConfig::empty();
      let inlining_success_config = MethodInliningSucceededConfig::new(MethodInliningConfig::new(
        MethodCommonAttributesConfig::new("m1".to_string(), "n1".to_string(), "s1".to_string()),
        MethodCommonAttributesConfig::new("m2".to_string(), "n2".to_string(), "s2".to_string()),
      ));

      config.set_method_inlining_success(Some(ExtractionConfig::new("Inlining".to_string(), inlining_success_config)));

      let extractor = MethodsDataExtractor::new(&config);
      let mut software_data = SoftwareData::empty();
      extractor.extract_from_events(&mut software_data, &events).ok().unwrap();

      software_data
    },
  )
}

#[test]
fn test_inlining_failed_extraction() {
  execute_test_with_software_data(
    r#"{"method_inlinings_events":[{"InliningFailed":[{"inlinee_info":{"name":"xd1","namespace":"xd2","signature":"xd3"},"inliner_info":{"name":"xd4","namespace":"xd5","signature":"xd6"}},"failed"]}]}"#,
    || {
      let events = [
        create_event_with_attributes("Inlining".to_string(), vec![
          ("m1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd1".to_string())))),
          ("n1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd2".to_string())))),
          ("s1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd3".to_string())))),
          ("m2".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd4".to_string())))),
          ("n2".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd5".to_string())))),
          ("s2".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd6".to_string())))),
          ("reason".to_string(), EventPayloadValue::String(Rc::new(Box::new("failed".to_string()))))
        ]),
        create_event_with_attributes("Inli123ning".to_string(), vec![
          ("m1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd1".to_string())))),
          ("n1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd2".to_string())))),
          ("s1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd3".to_string())))),
          ("m2".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd4".to_string())))),
          ("n2".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd5".to_string())))),
          ("s2".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd6".to_string())))),
          ("reason".to_string(), EventPayloadValue::String(Rc::new(Box::new("failed".to_string()))))
        ]),
      ];

      let mut config = SoftwareDataExtractionConfig::empty();
      let inlining_success_config = MethodInliningFailedConfig::new(
        MethodInliningConfig::new(
          MethodCommonAttributesConfig::new("m1".to_string(), "n1".to_string(), "s1".to_string()),
          MethodCommonAttributesConfig::new("m2".to_string(), "n2".to_string(), "s2".to_string()),
        ),
        "reason".to_string(),
      );

      config.set_method_inlining_failed(Some(ExtractionConfig::new("Inlining".to_string(), inlining_success_config)));

      let extractor = MethodsDataExtractor::new(&config);
      let mut software_data = SoftwareData::empty();
      extractor.extract_from_events(&mut software_data, &events).ok().unwrap();

      software_data
    },
  )
}

#[test]
fn test_method_load_extraction() {
  execute_test_with_software_data(
    r#"{"method_load_unload_events":[{"Load":{"name":"xd1","namespace":"xd2","signature":"xd3"}}]}"#,
    || {
      let events = [
        create_event_with_attributes("MethodLoad".to_string(), vec![
          ("m1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd1".to_string())))),
          ("n1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd2".to_string())))),
          ("s1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd3".to_string())))),
        ]),
        create_event_with_attributes("Method12312Start".to_string(), vec![
          ("m1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd1".to_string())))),
          ("n1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd2".to_string())))),
          ("s1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd3".to_string())))),
        ]),
      ];

      let mut config = SoftwareDataExtractionConfig::empty();
      let method_start = MethodLoadUnloadConfig::new(MethodCommonAttributesConfig::new("m1".to_string(), "n1".to_string(), "s1".to_string()));

      config.set_method_load(Some(ExtractionConfig::new("MethodLoad".to_string(), method_start)));

      let extractor = MethodsDataExtractor::new(&config);
      let mut software_data = SoftwareData::empty();
      extractor.extract_from_events(&mut software_data, &events).ok().unwrap();

      software_data
    },
  )
}

#[test]
fn test_method_unload_extraction() {
  execute_test_with_software_data(
    r#"{"method_load_unload_events":[{"Unload":{"name":"xd1","namespace":"xd2","signature":"xd3"}}]}"#,
    || {
      let events = [
        create_event_with_attributes("MethodUnload".to_string(), vec![
          ("m1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd1".to_string())))),
          ("n1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd2".to_string())))),
          ("s1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd3".to_string())))),
        ]),
        create_event_with_attributes("Method123123Unload".to_string(), vec![
          ("m1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd1".to_string())))),
          ("n1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd2".to_string())))),
          ("s1".to_string(), EventPayloadValue::String(Rc::new(Box::new("xd3".to_string())))),
        ]),
      ];

      let mut config = SoftwareDataExtractionConfig::empty();
      let method_start = MethodLoadUnloadConfig::new(MethodCommonAttributesConfig::new("m1".to_string(), "n1".to_string(), "s1".to_string()));

      config.set_method_unload(Some(ExtractionConfig::new("MethodUnload".to_string(), method_start)));

      let extractor = MethodsDataExtractor::new(&config);
      let mut software_data = SoftwareData::empty();
      extractor.extract_from_events(&mut software_data, &events).ok().unwrap();

      software_data
    },
  )
}

#[test]
fn test_exception_extraction() {
  execute_test_with_software_data(
    r#"{"exception_events":[{"exception_type":"ArgumentOutOfRange"}]}"#,
    || {
      let events = [
        create_event_with_attributes("Exception".to_string(), vec![
          ("type".to_string(), EventPayloadValue::String(Rc::new(Box::new("ArgumentOutOfRange".to_string())))),
        ]),
        create_event_with_attributes("Excepasdsdtion".to_string(), vec![
          ("type".to_string(), EventPayloadValue::String(Rc::new(Box::new("ArgumentOutOfRange".to_string())))),
        ]),
      ];

      let mut config = SoftwareDataExtractionConfig::empty();
      let method_start = ExceptionExtractionConfig::new("type".to_string());

      config.set_exceptions(Some(ExtractionConfig::new("Exception".to_string(), method_start)));

      let extractor = ExceptionDataExtractor::new(&config);
      let mut software_data = SoftwareData::empty();
      extractor.extract_from_events(&mut software_data, &events).ok().unwrap();

      software_data
    },
  )
}

#[test]
fn test_assembly_load_extraction() {
  execute_test_with_software_data(
    r#"{"assembly_events":[{"name":"System.Private.CorLib","kind":"Load"}]}"#,
    || {
      let events = [
        create_event_with_attributes("AssemblyLoad".to_string(), vec![
          ("type".to_string(), EventPayloadValue::String(Rc::new(Box::new("System.Private.CorLib".to_string())))),
        ]),
        create_event_with_attributes("AssembasdsadyLoad".to_string(), vec![
          ("type".to_string(), EventPayloadValue::String(Rc::new(Box::new("System.Private.CorLib".to_string())))),
        ]),
      ];

      let mut config = SoftwareDataExtractionConfig::empty();
      let method_start = AssemblyExtractionConfig::new("type".to_string());

      config.set_assembly_load(Some(ExtractionConfig::new("AssemblyLoad".to_string(), method_start)));

      let extractor = AssemblySoftwareDataExtractor::new(&config);
      let mut software_data = SoftwareData::empty();
      extractor.extract_from_events(&mut software_data, &events).ok().unwrap();

      software_data
    },
  )
}

#[test]
fn test_assembly_unload_extraction() {
  execute_test_with_software_data(
    r#"{"assembly_events":[{"name":"System.Private.CorLib","kind":"Unload"}]}"#,
    || {
      let events = [
        create_event_with_attributes("AssemblyUnload".to_string(), vec![
          ("type".to_string(), EventPayloadValue::String(Rc::new(Box::new("System.Private.CorLib".to_string())))),
        ]),
        create_event_with_attributes("AssemqweqweblyUnload".to_string(), vec![
          ("type".to_string(), EventPayloadValue::String(Rc::new(Box::new("System.Private.CorLib".to_string())))),
        ]),
      ];

      let mut config = SoftwareDataExtractionConfig::empty();
      let method_start = AssemblyExtractionConfig::new("type".to_string());

      config.set_assembly_unload(Some(ExtractionConfig::new("AssemblyUnload".to_string(), method_start)));

      let extractor = AssemblySoftwareDataExtractor::new(&config);
      let mut software_data = SoftwareData::empty();
      extractor.extract_from_events(&mut software_data, &events).ok().unwrap();

      software_data
    },
  )
}

#[test]
fn test_thread_created_extraction() {
  execute_test_with_software_data(
    r#"{"thread_events":[{"thread_id":123,"kind":"Created"}]}"#,
    || {
      let events = [
        create_event_with_attributes("ThreadCreated".to_string(), vec![
          ("id".to_string(), EventPayloadValue::Int64(123)),
        ]),
        create_event_with_attributes("Thre23123123123adCreated".to_string(), vec![
          ("id".to_string(), EventPayloadValue::Int64(123)),
        ]),
      ];

      let mut config = SoftwareDataExtractionConfig::empty();
      let thread_created_config = ThreadExtractionConfig::new("id".to_string());

      config.set_thread_created(Some(ExtractionConfig::new("ThreadCreated".to_string(), thread_created_config)));

      let extractor = ThreadDataExtractor::new(&config);
      let mut software_data = SoftwareData::empty();
      extractor.extract_from_events(&mut software_data, &events).ok().unwrap();

      software_data
    },
  )
}

#[test]
fn test_http_extraction() {
  execute_test_with_software_data(
    r#"{"http_events":[{"host":"localhost","port":"1234","scheme":"https","path_and_query":"/xd"}]}"#,
    || {
      let events = [
        create_event_with_attributes("HTTP".to_string(), vec![
          ("host".to_string(), EventPayloadValue::String(Rc::new(Box::new("localhost".to_string())))),
          ("port".to_string(), EventPayloadValue::String(Rc::new(Box::new("1234".to_string())))),
          ("scheme".to_string(), EventPayloadValue::String(Rc::new(Box::new("https".to_string())))),
          ("path_and_query".to_string(), EventPayloadValue::String(Rc::new(Box::new("/xd".to_string())))),
        ]),
        create_event_with_attributes("HT123wasdsaTP".to_string(), vec![
          ("host".to_string(), EventPayloadValue::String(Rc::new(Box::new("localhost".to_string())))),
          ("port".to_string(), EventPayloadValue::String(Rc::new(Box::new("1234".to_string())))),
          ("scheme".to_string(), EventPayloadValue::String(Rc::new(Box::new("https".to_string())))),
          ("path_and_query".to_string(), EventPayloadValue::String(Rc::new(Box::new("/xd".to_string())))),
        ]),
      ];

      let mut config = SoftwareDataExtractionConfig::empty();
      let http_config = HTTPExtractionConfig::new("host".to_string(), "port".to_string(), "scheme".to_string(), "path_and_query".to_string());
      config.set_http(Some(ExtractionConfig::new("HTTP".to_string(), http_config)));

      let extractor = HTTPSoftwareDataExtractor::new(&config);
      let mut software_data = SoftwareData::empty();
      extractor.extract_from_events(&mut software_data, &events).ok().unwrap();

      software_data
    },
  )
}

#[test]
fn test_array_pools_extraction() {
  execute_test_with_software_data(
    r#"{"pool_events":[{"buffer_id":123,"buffer_size_bytes":3213123123,"event_kind":"Created"},{"buffer_id":123,"buffer_size_bytes":3213123123,"event_kind":"Rented"},{"buffer_id":123,"buffer_size_bytes":3213123123,"event_kind":"Trimmed"},{"buffer_id":123,"buffer_size_bytes":3213123123,"event_kind":"Returned"}]}"#,
    || {
      let events = [
        create_event_with_attributes("Created".to_string(), vec![
          ("buffer_id".to_string(), EventPayloadValue::Int64(123)),
          ("buffer_size".to_string(), EventPayloadValue::Int64(3213123123)),
        ]),
        create_event_with_attributes("Rented".to_string(), vec![
          ("buffer_id".to_string(), EventPayloadValue::Int64(123)),
          ("buffer_size".to_string(), EventPayloadValue::Int64(3213123123)),
        ]),
        create_event_with_attributes("Trimmed".to_string(), vec![
          ("buffer_id".to_string(), EventPayloadValue::Int64(123)),
          ("buffer_size".to_string(), EventPayloadValue::Int64(3213123123)),
        ]),
        create_event_with_attributes("Returned".to_string(), vec![
          ("buffer_id".to_string(), EventPayloadValue::Int64(123)),
          ("buffer_size".to_string(), EventPayloadValue::Int64(3213123123)),
        ]),
        create_event_with_attributes("Creaasasdted".to_string(), vec![
          ("buffer_id".to_string(), EventPayloadValue::Int64(123)),
          ("buffer_size".to_string(), EventPayloadValue::Int64(3213123123)),
        ]),
        create_event_with_attributes("Rent1323ed".to_string(), vec![
          ("buffer_id".to_string(), EventPayloadValue::Int64(123)),
          ("buffer_size".to_string(), EventPayloadValue::Int64(3213123123)),
        ]),
        create_event_with_attributes("Trimmasdaded".to_string(), vec![
          ("buffer_id".to_string(), EventPayloadValue::Int64(123)),
          ("buffer_size".to_string(), EventPayloadValue::Int64(3213123123)),
        ]),
        create_event_with_attributes("Re123123turned".to_string(), vec![
          ("buffer_id".to_string(), EventPayloadValue::Int64(123)),
          ("buffer_size".to_string(), EventPayloadValue::Int64(3213123123)),
        ]),
      ];

      let mut config = SoftwareDataExtractionConfig::empty();
      let array_pool_config = ArrayPoolExtractionConfig::new("buffer_id".to_string(), "buffer_size".to_string());

      config.set_array_pool_array_created(Some(ExtractionConfig::new("Created".to_string(), array_pool_config.clone())));
      config.set_array_pool_array_rented(Some(ExtractionConfig::new("Rented".to_string(), array_pool_config.clone())));
      config.set_array_pool_array_trimmed(Some(ExtractionConfig::new("Trimmed".to_string(), array_pool_config.clone())));
      config.set_array_pool_array_returned(Some(ExtractionConfig::new("Returned".to_string(), array_pool_config.clone())));

      let extractor = ArrayPoolDataExtractor::new(&config);
      let mut software_data = SoftwareData::empty();
      extractor.extract_from_events(&mut software_data, &events).ok().unwrap();

      software_data
    },
  )
}

#[test]
fn test_sockets_extraction() {
  execute_test_with_software_data(
    r#"{"socket_events":[{"ConnectStart":{"address":"localhost1"}},{"AcceptStart":{"address":"localhost2"}},{"ConnectFailed":{"error_code":"error","error_message":"=))"}},{"AcceptFailed":{"error_code":"error1","error_message":"=(("}},"ConnectStop","AcceptStop"]}"#,
    || {
      let events = [
        create_event_with_attributes("ConnectStart".to_string(), vec![
          ("address".to_string(), EventPayloadValue::String(Rc::new(Box::new("localhost1".to_string())))),
        ]),
        create_event_with_attributes("AcceptStart".to_string(), vec![
          ("address".to_string(), EventPayloadValue::String(Rc::new(Box::new("localhost2".to_string())))),
        ]),
        create_event_with_attributes("ConnectFailed".to_string(), vec![
          ("error_code".to_string(), EventPayloadValue::String(Rc::new(Box::new("error".to_string())))),
          ("message".to_string(), EventPayloadValue::String(Rc::new(Box::new("=))".to_string())))),
        ]),
        create_event_with_attributes("AcceptFailed".to_string(), vec![
          ("error_code".to_string(), EventPayloadValue::String(Rc::new(Box::new("error1".to_string())))),
          ("message".to_string(), EventPayloadValue::String(Rc::new(Box::new("=((".to_string())))),
        ]),
        create_event_with_attributes("ConnectStop".to_string(), vec![]),
        create_event_with_attributes("AcceptStop".to_string(), vec![]),
      ];

      let mut config = SoftwareDataExtractionConfig::empty();
      let start_config = SocketConnectAcceptStartConfig::new("address".to_string());

      config.set_socket_accept_start(Some(ExtractionConfig::new("AcceptStart".to_string(), start_config.clone())));
      config.set_socket_connect_start(Some(ExtractionConfig::new("ConnectStart".to_string(), start_config.clone())));

      let failed_config = SocketAcceptConnectFailedConfig::new("error_code".to_string(), "message".to_string());
      config.set_socket_accept_failed(Some(ExtractionConfig::new("AcceptFailed".to_string(), failed_config.clone())));
      config.set_socket_connect_failed(Some(ExtractionConfig::new("ConnectFailed".to_string(), failed_config.clone())));

      config.set_socket_connect_stop(Some(ExtractionConfig::new("ConnectStop".to_string(), ())));
      config.set_socket_accept_stop(Some(ExtractionConfig::new("AcceptStop".to_string(), ())));

      let extractor = SocketsDataExtractor::new(&config);
      let mut software_data = SoftwareData::empty();
      extractor.extract_from_events(&mut software_data, &events).ok().unwrap();

      software_data
    },
  )
}

#[test]
fn test_general_histogram() {
  execute_test_with_software_data(
    r#"{"histograms":[{"name":"g1","units":"units","entries":[{"name":"type1","value":246.0},{"name":"type2","value":123.0}]},{"name":"g2","units":"units","entries":[{"name":"type1","value":123.0},{"name":"type2","value":123.0}]}]}"#,
    || {
      let events = [
        create_event_with_attributes(
          "histogram_event".to_string(),
          vec![
            ("type".to_string(), EventPayloadValue::String(Rc::new(Box::new("type1".to_string())))),
            ("count".to_string(), EventPayloadValue::Float64(123.)),
          ],
        ),
        create_event_with_attributes(
          "histogram_event".to_string(),
          vec![
            ("type".to_string(), EventPayloadValue::String(Rc::new(Box::new("type2".to_string())))),
            ("count".to_string(), EventPayloadValue::Float32(123.)),
          ],
        ),
        create_event_with_attributes(
          "histogram_event".to_string(),
          vec![
            ("type".to_string(), EventPayloadValue::String(Rc::new(Box::new("type1".to_string())))),
            ("count".to_string(), EventPayloadValue::Uint64(123)),
          ],
        ),
        create_event_with_attributes(
          "unknown".to_string(),
          vec![
            ("type".to_string(), EventPayloadValue::String(Rc::new(Box::new("type1".to_string())))),
            ("count".to_string(), EventPayloadValue::Uint32(123)),
          ],
        ),
        create_event_with_attributes(
          "hst_event".to_string(),
          vec![
            ("type".to_string(), EventPayloadValue::String(Rc::new(Box::new("type1".to_string())))),
            ("count".to_string(), EventPayloadValue::Int64(123)),
          ],
        ),
        create_event_with_attributes(
          "hst_event".to_string(),
          vec![
            ("type".to_string(), EventPayloadValue::String(Rc::new(Box::new("type2".to_string())))),
            ("count".to_string(), EventPayloadValue::Int32(123)),
          ],
        ),
      ];

      let mut config = SoftwareDataExtractionConfig::empty();
      config.set_pie_chart_extraction_configs(vec![
        ExtractionConfig::new(
          "histogram_event".to_string(),
          PieChartExtractionConfig::new(
            "g1".to_string(),
            Some(NameCreationStrategy::SingleAttribute(SingleAttribute::new("type".to_string(), "xd".to_string()))),
            Some("count".to_string()),
            "units".to_string()
          )
        ),
        ExtractionConfig::new(
          "hst_event".to_string(),
          PieChartExtractionConfig::new(
            "g2".to_string(),
            Some(NameCreationStrategy::SingleAttribute(SingleAttribute::new("type".to_string(), "xd".to_string()))),
            Some("count".to_string()),
            "units".to_string()
          )
        )
      ]);

      let extractor = PieChartExtractor::new(&config);
      let mut software_data = SoftwareData::empty();
      extractor.extract_from_events(&mut software_data, &events).ok().unwrap();

      software_data.histograms_mut().sort_by(|f, s| f.name().cmp(s.name()));
      software_data.histograms_mut().iter_mut().for_each(|counts| counts.entries_mut().sort_by(|f, s| f.name().cmp(s.name())));

      software_data
    },
  )
}

#[test]
fn test_simple_counter() {
  execute_test_with_software_data(
    r#"{"simple_counters":[{"name":"counter1","value":3.0,"units":"units"},{"name":"counter2","value":246.0,"units":"units"}]}"#,
    || {
      let events = [
        create_event_with_attributes(
          "histogram_event".to_string(),
          vec![
            ("type".to_string(), EventPayloadValue::String(Rc::new(Box::new("type1".to_string())))),
            ("count".to_string(), EventPayloadValue::Float64(123.)),
          ],
        ),
        create_event_with_attributes(
          "histogram_event".to_string(),
          vec![
            ("type".to_string(), EventPayloadValue::String(Rc::new(Box::new("type2".to_string())))),
            ("count".to_string(), EventPayloadValue::Float32(123.)),
          ],
        ),
        create_event_with_attributes(
          "histogram_event".to_string(),
          vec![
            ("type".to_string(), EventPayloadValue::String(Rc::new(Box::new("type1".to_string())))),
            ("count".to_string(), EventPayloadValue::Uint64(123)),
          ],
        ),
        create_event_with_attributes(
          "unknown".to_string(),
          vec![
            ("type".to_string(), EventPayloadValue::String(Rc::new(Box::new("type1".to_string())))),
            ("count".to_string(), EventPayloadValue::Uint32(123)),
          ],
        ),
        create_event_with_attributes(
          "hst_event".to_string(),
          vec![
            ("type".to_string(), EventPayloadValue::String(Rc::new(Box::new("type1".to_string())))),
            ("count".to_string(), EventPayloadValue::Int64(123)),
          ],
        ),
        create_event_with_attributes(
          "hst_event".to_string(),
          vec![
            ("type".to_string(), EventPayloadValue::String(Rc::new(Box::new("type2".to_string())))),
            ("count".to_string(), EventPayloadValue::Int32(123)),
          ],
        ),
      ];

      let mut config = SoftwareDataExtractionConfig::empty();
      config.set_simple_counter_configs(vec![
        ExtractionConfig::new(
          "histogram_event".to_string(),
          SimpleCountExtractionConfig::new("counter1".to_string(), None, "units".to_string())
        ),
        ExtractionConfig::new(
          "hst_event".to_string(),
          SimpleCountExtractionConfig::new("counter2".to_string(), Some("count".to_string()), "units".to_string())
        ),
      ]);

      let extractor = SimpleCounterExtractor::new(&config);
      let mut software_data = SoftwareData::empty();
      extractor.extract_from_events(&mut software_data, &events).ok().unwrap();

      software_data.simple_counters_mut().sort_by(|f, s| f.name().cmp(s.name()));
      software_data
    },
  )
}

fn create_event_with_attributes(name: String, attributes: Vec<(String, EventPayloadValue)>) -> Rc<RefCell<XesEventImpl>> {
  Rc::new(RefCell::new(XesEventImpl::new_all_fields(Rc::new(Box::new(name)), Utc::now(), Some(attributes.into_iter().collect()))))
}

fn execute_test_with_software_data(gold: &str, test: impl Fn() -> SoftwareData) {
  assert_eq!(gold, serde_json::to_string(&test()).unwrap());
}