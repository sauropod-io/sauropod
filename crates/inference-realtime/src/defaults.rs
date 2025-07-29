pub fn make_default_vad() -> Option<sauropod_openai_api::RealtimeSessionTurnDetection> {
    Some(sauropod_openai_api::RealtimeSessionTurnDetection {
        create_response: None,
        eagerness: None,
        interrupt_response: None,
        prefix_padding_ms: Some(300),
        silence_duration_ms: Some(500),
        threshold: Some(0.5),
        r#type: Some(sauropod_openai_api::RealtimeSessionTurnDetectionType::ServerVad),
    })
}
