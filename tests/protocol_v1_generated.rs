#[allow(dead_code)]
#[path = "../protocol/generated/rust/protocol_v1.rs"]
mod protocol_v1;

use serde_json::Value;

fn example(schema: &str) -> Vec<u8> {
    let parsed: Value = serde_json::from_str(schema).unwrap();
    serde_json::to_vec(&parsed["examples"][0]).unwrap()
}

#[test]
fn generated_protocol_examples_decode_structurally() {
    protocol_v1::decode_artifact(&example(protocol_v1::ARTIFACT_SCHEMA_JSON)).unwrap();
    protocol_v1::decode_canonical_run(&example(protocol_v1::CANONICAL_RUN_SCHEMA_JSON)).unwrap();
    protocol_v1::decode_extension_manifest(&example(protocol_v1::EXTENSION_MANIFEST_SCHEMA_JSON))
        .unwrap();
    protocol_v1::decode_initialize(&example(protocol_v1::INITIALIZE_SCHEMA_JSON)).unwrap();
    protocol_v1::decode_initialized(&example(protocol_v1::INITIALIZED_SCHEMA_JSON)).unwrap();
    protocol_v1::decode_protocol_error(&example(protocol_v1::PROTOCOL_ERROR_SCHEMA_JSON)).unwrap();
    protocol_v1::decode_protocol_message(&example(protocol_v1::PROTOCOL_MESSAGE_SCHEMA_JSON))
        .unwrap();
    protocol_v1::decode_run_event(&example(protocol_v1::RUN_EVENT_SCHEMA_JSON)).unwrap();
}

#[test]
fn generated_protocol_rejects_structural_drift_and_matches_manifest() {
    let schema: Value = serde_json::from_str(protocol_v1::PROTOCOL_MESSAGE_SCHEMA_JSON).unwrap();
    let mut message = schema["examples"][0].clone();
    message["undeclared"] = Value::Bool(true);
    let bytes = serde_json::to_vec(&message).unwrap();
    assert!(protocol_v1::decode_protocol_message(&bytes).is_err());

    let manifest: Value = serde_json::from_str(include_str!("../ai-mux.extension.json")).unwrap();
    assert_eq!(
        manifest["protocolSchemaHash"],
        protocol_v1::PROTOCOL_V1_SCHEMA_SHA256
    );
    assert_eq!(protocol_v1::PROTOCOL_V1_RELEASE, "1.0.0-rc.1");
}

#[test]
fn vendored_conformance_package_matches_generated_protocol() {
    let release: Value =
        serde_json::from_str(include_str!("fixtures/ai-mux-conformance-v1/release.json")).unwrap();
    let cases: Value =
        serde_json::from_str(include_str!("fixtures/ai-mux-conformance-v1/cases.json")).unwrap();
    assert_eq!(
        release["protocolSchemaHash"],
        protocol_v1::PROTOCOL_V1_SCHEMA_SHA256
    );
    assert_eq!(release["release"], protocol_v1::PROTOCOL_V1_RELEASE);
    assert_eq!(cases["toolCases"].as_array().unwrap().len(), 4);
    assert_eq!(cases["workflowCases"].as_array().unwrap().len(), 5);
}
