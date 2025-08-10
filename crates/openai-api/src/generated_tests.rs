//! OpenAI types tests.
//!
//! This file is auto-generated.

use crate::test_utils::*;
/// Test for Model example value
#[test]
fn test_model() {
    const MODEL_EXAMPLE: &str = r#"{
  "id": "VAR_chat_model_id",
  "object": "model",
  "created": 1686935002,
  "owned_by": "openai"
}"#;
    round_trip_test::<crate::Model>(MODEL_EXAMPLE).unwrap();
}
/// Test for RealtimeClientEventConversationItemCreate example value
#[test]
fn test_realtime_client_event_conversation_item_create() {
    const REALTIME_CLIENT_EVENT_CONVERSATION_ITEM_CREATE_EXAMPLE: &str = r#"{
    "event_id": "event_345",
    "type": "conversation.item.create",
    "previous_item_id": null,
    "item": {
        "id": "msg_001",
        "type": "message",
        "role": "user",
        "content": [
            {
                "type": "input_text",
                "text": "Hello, how are you?"
            }
        ]
    }
}"#;
    round_trip_test::<crate::RealtimeClientEventConversationItemCreate>(
        REALTIME_CLIENT_EVENT_CONVERSATION_ITEM_CREATE_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeClientEventConversationItemDelete example value
#[test]
fn test_realtime_client_event_conversation_item_delete() {
    const REALTIME_CLIENT_EVENT_CONVERSATION_ITEM_DELETE_EXAMPLE: &str = r#"{
    "event_id": "event_901",
    "type": "conversation.item.delete",
    "item_id": "msg_003"
}"#;
    round_trip_test::<crate::RealtimeClientEventConversationItemDelete>(
        REALTIME_CLIENT_EVENT_CONVERSATION_ITEM_DELETE_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeClientEventConversationItemRetrieve example value
#[test]
fn test_realtime_client_event_conversation_item_retrieve() {
    const REALTIME_CLIENT_EVENT_CONVERSATION_ITEM_RETRIEVE_EXAMPLE: &str = r#"{
    "event_id": "event_901",
    "type": "conversation.item.retrieve",
    "item_id": "msg_003"
}"#;
    round_trip_test::<crate::RealtimeClientEventConversationItemRetrieve>(
        REALTIME_CLIENT_EVENT_CONVERSATION_ITEM_RETRIEVE_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeClientEventConversationItemTruncate example value
#[test]
fn test_realtime_client_event_conversation_item_truncate() {
    const REALTIME_CLIENT_EVENT_CONVERSATION_ITEM_TRUNCATE_EXAMPLE: &str = r#"{
    "event_id": "event_678",
    "type": "conversation.item.truncate",
    "item_id": "msg_002",
    "content_index": 0,
    "audio_end_ms": 1500
}"#;
    round_trip_test::<crate::RealtimeClientEventConversationItemTruncate>(
        REALTIME_CLIENT_EVENT_CONVERSATION_ITEM_TRUNCATE_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeClientEventInputAudioBufferAppend example value
#[test]
fn test_realtime_client_event_input_audio_buffer_append() {
    const REALTIME_CLIENT_EVENT_INPUT_AUDIO_BUFFER_APPEND_EXAMPLE: &str = r#"{
    "event_id": "event_456",
    "type": "input_audio_buffer.append",
    "audio": "Base64EncodedAudioData"
}"#;
    round_trip_test::<crate::RealtimeClientEventInputAudioBufferAppend>(
        REALTIME_CLIENT_EVENT_INPUT_AUDIO_BUFFER_APPEND_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeClientEventInputAudioBufferClear example value
#[test]
fn test_realtime_client_event_input_audio_buffer_clear() {
    const REALTIME_CLIENT_EVENT_INPUT_AUDIO_BUFFER_CLEAR_EXAMPLE: &str = r#"{
    "event_id": "event_012",
    "type": "input_audio_buffer.clear"
}"#;
    round_trip_test::<crate::RealtimeClientEventInputAudioBufferClear>(
        REALTIME_CLIENT_EVENT_INPUT_AUDIO_BUFFER_CLEAR_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeClientEventInputAudioBufferCommit example value
#[test]
fn test_realtime_client_event_input_audio_buffer_commit() {
    const REALTIME_CLIENT_EVENT_INPUT_AUDIO_BUFFER_COMMIT_EXAMPLE: &str = r#"{
    "event_id": "event_789",
    "type": "input_audio_buffer.commit"
}"#;
    round_trip_test::<crate::RealtimeClientEventInputAudioBufferCommit>(
        REALTIME_CLIENT_EVENT_INPUT_AUDIO_BUFFER_COMMIT_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeClientEventOutputAudioBufferClear example value
#[test]
fn test_realtime_client_event_output_audio_buffer_clear() {
    const REALTIME_CLIENT_EVENT_OUTPUT_AUDIO_BUFFER_CLEAR_EXAMPLE: &str = r#"{
    "event_id": "optional_client_event_id",
    "type": "output_audio_buffer.clear"
}"#;
    round_trip_test::<crate::RealtimeClientEventOutputAudioBufferClear>(
        REALTIME_CLIENT_EVENT_OUTPUT_AUDIO_BUFFER_CLEAR_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeClientEventResponseCancel example value
#[test]
fn test_realtime_client_event_response_cancel() {
    const REALTIME_CLIENT_EVENT_RESPONSE_CANCEL_EXAMPLE: &str = r#"{
    "event_id": "event_567",
    "type": "response.cancel"
}"#;
    round_trip_test::<crate::RealtimeClientEventResponseCancel>(
        REALTIME_CLIENT_EVENT_RESPONSE_CANCEL_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeClientEventResponseCreate example value
#[test]
#[should_panic]
fn test_realtime_client_event_response_create() {
    const REALTIME_CLIENT_EVENT_RESPONSE_CREATE_EXAMPLE: &str = r#"{
    "event_id": "event_234",
    "type": "response.create",
    "response": {
        "modalities": ["text", "audio"],
        "instructions": "Please assist the user.",
        "voice": "sage",
        "output_audio_format": "pcm16",
        "tools": [
            {
                "type": "function",
                "name": "calculate_sum",
                "description": "Calculates the sum of two numbers.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "a": { "type": "number" },
                        "b": { "type": "number" }
                    },
                    "required": ["a", "b"]
                }
            }
        ],
        "tool_choice": "auto",
        "temperature": 0.8,
        "max_output_tokens": 1024
    }
}"#;
    round_trip_test::<crate::RealtimeClientEventResponseCreate>(
        REALTIME_CLIENT_EVENT_RESPONSE_CREATE_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeClientEventSessionUpdate example value
#[test]
fn test_realtime_client_event_session_update() {
    const REALTIME_CLIENT_EVENT_SESSION_UPDATE_EXAMPLE: &str = r#"{
    "event_id": "event_123",
    "type": "session.update",
    "session": {
        "modalities": ["text", "audio"],
        "instructions": "You are a helpful assistant.",
        "voice": "sage",
        "input_audio_format": "pcm16",
        "output_audio_format": "pcm16",
        "input_audio_transcription": {
            "model": "whisper-1"
        },
        "turn_detection": {
            "type": "server_vad",
            "threshold": 0.5,
            "prefix_padding_ms": 300,
            "silence_duration_ms": 500,
            "create_response": true
        },
        "tools": [
            {
                "type": "function",
                "name": "get_weather",
                "description": "Get the current weather...",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "location": { "type": "string" }
                    },
                    "required": ["location"]
                }
            }
        ],
        "tool_choice": "auto",
        "temperature": 0.8,
        "max_response_output_tokens": "inf",
        "speed": 1.1,
        "tracing": "auto"
    }
}"#;
    round_trip_test::<crate::RealtimeClientEventSessionUpdate>(
        REALTIME_CLIENT_EVENT_SESSION_UPDATE_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeClientEventTranscriptionSessionUpdate example value
#[test]
fn test_realtime_client_event_transcription_session_update() {
    const REALTIME_CLIENT_EVENT_TRANSCRIPTION_SESSION_UPDATE_EXAMPLE: &str = r#"{
  "type": "transcription_session.update",
  "session": {
    "input_audio_format": "pcm16",
    "input_audio_transcription": {
      "model": "gpt-4o-transcribe",
      "prompt": "",
      "language": ""
    },
    "turn_detection": {
      "type": "server_vad",
      "threshold": 0.5,
      "prefix_padding_ms": 300,
      "silence_duration_ms": 500,
      "create_response": true
    },
    "input_audio_noise_reduction": {
      "type": "near_field"
    },
    "include": [
      "item.input_audio_transcription.logprobs"
    ]
  }
}"#;
    round_trip_test::<crate::RealtimeClientEventTranscriptionSessionUpdate>(
        REALTIME_CLIENT_EVENT_TRANSCRIPTION_SESSION_UPDATE_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventConversationCreated example value
#[test]
fn test_realtime_server_event_conversation_created() {
    const REALTIME_SERVER_EVENT_CONVERSATION_CREATED_EXAMPLE: &str = r#"{
    "event_id": "event_9101",
    "type": "conversation.created",
    "conversation": {
        "id": "conv_001",
        "object": "realtime.conversation"
    }
}"#;
    round_trip_test::<crate::RealtimeServerEventConversationCreated>(
        REALTIME_SERVER_EVENT_CONVERSATION_CREATED_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventConversationItemCreated example value
#[test]
fn test_realtime_server_event_conversation_item_created() {
    const REALTIME_SERVER_EVENT_CONVERSATION_ITEM_CREATED_EXAMPLE: &str = r#"{
    "event_id": "event_1920",
    "type": "conversation.item.created",
    "previous_item_id": "msg_002",
    "item": {
        "id": "msg_003",
        "object": "realtime.item",
        "type": "message",
        "status": "completed",
        "role": "user",
        "content": []
    }
}"#;
    round_trip_test::<crate::RealtimeServerEventConversationItemCreated>(
        REALTIME_SERVER_EVENT_CONVERSATION_ITEM_CREATED_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventConversationItemDeleted example value
#[test]
fn test_realtime_server_event_conversation_item_deleted() {
    const REALTIME_SERVER_EVENT_CONVERSATION_ITEM_DELETED_EXAMPLE: &str = r#"{
    "event_id": "event_2728",
    "type": "conversation.item.deleted",
    "item_id": "msg_005"
}"#;
    round_trip_test::<crate::RealtimeServerEventConversationItemDeleted>(
        REALTIME_SERVER_EVENT_CONVERSATION_ITEM_DELETED_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventConversationItemInputAudioTranscriptionCompleted example value
#[test]
fn test_realtime_server_event_conversation_item_input_audio_transcription_completed() {
    const REALTIME_SERVER_EVENT_CONVERSATION_ITEM_INPUT_AUDIO_TRANSCRIPTION_COMPLETED_EXAMPLE:
        &str = r#"{
    "event_id": "event_2122",
    "type": "conversation.item.input_audio_transcription.completed",
    "item_id": "msg_003",
    "content_index": 0,
    "transcript": "Hello, how are you?",
    "usage": {
      "type": "tokens",
      "total_tokens": 48,
      "input_tokens": 38,
      "input_token_details": {
        "text_tokens": 10,
        "audio_tokens": 28
      },
      "output_tokens": 10
    }
}"#;
    round_trip_test::<crate::RealtimeServerEventConversationItemInputAudioTranscriptionCompleted>(
        REALTIME_SERVER_EVENT_CONVERSATION_ITEM_INPUT_AUDIO_TRANSCRIPTION_COMPLETED_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventConversationItemInputAudioTranscriptionDelta example value
#[test]
fn test_realtime_server_event_conversation_item_input_audio_transcription_delta() {
    const REALTIME_SERVER_EVENT_CONVERSATION_ITEM_INPUT_AUDIO_TRANSCRIPTION_DELTA_EXAMPLE: &str = r#"{
  "type": "conversation.item.input_audio_transcription.delta",
  "event_id": "event_001",
  "item_id": "item_001",
  "content_index": 0,
  "delta": "Hello"
}"#;
    round_trip_test::<crate::RealtimeServerEventConversationItemInputAudioTranscriptionDelta>(
        REALTIME_SERVER_EVENT_CONVERSATION_ITEM_INPUT_AUDIO_TRANSCRIPTION_DELTA_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventConversationItemInputAudioTranscriptionFailed example value
#[test]
fn test_realtime_server_event_conversation_item_input_audio_transcription_failed() {
    const REALTIME_SERVER_EVENT_CONVERSATION_ITEM_INPUT_AUDIO_TRANSCRIPTION_FAILED_EXAMPLE: &str = r#"{
    "event_id": "event_2324",
    "type": "conversation.item.input_audio_transcription.failed",
    "item_id": "msg_003",
    "content_index": 0,
    "error": {
        "type": "transcription_error",
        "code": "audio_unintelligible",
        "message": "The audio could not be transcribed.",
        "param": null
    }
}"#;
    round_trip_test::<crate::RealtimeServerEventConversationItemInputAudioTranscriptionFailed>(
        REALTIME_SERVER_EVENT_CONVERSATION_ITEM_INPUT_AUDIO_TRANSCRIPTION_FAILED_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventConversationItemRetrieved example value
#[test]
#[should_panic]
fn test_realtime_server_event_conversation_item_retrieved() {
    const REALTIME_SERVER_EVENT_CONVERSATION_ITEM_RETRIEVED_EXAMPLE: &str = r#"{
    "event_id": "event_1920",
    "type": "conversation.item.created",
    "previous_item_id": "msg_002",
    "item": {
        "id": "msg_003",
        "object": "realtime.item",
        "type": "message",
        "status": "completed",
        "role": "user",
        "content": [
            {
                "type": "input_audio",
                "transcript": "hello how are you",
                "audio": "base64encodedaudio=="
            }
        ]
    }
}"#;
    round_trip_test::<crate::RealtimeServerEventConversationItemRetrieved>(
        REALTIME_SERVER_EVENT_CONVERSATION_ITEM_RETRIEVED_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventConversationItemTruncated example value
#[test]
fn test_realtime_server_event_conversation_item_truncated() {
    const REALTIME_SERVER_EVENT_CONVERSATION_ITEM_TRUNCATED_EXAMPLE: &str = r#"{
    "event_id": "event_2526",
    "type": "conversation.item.truncated",
    "item_id": "msg_004",
    "content_index": 0,
    "audio_end_ms": 1500
}"#;
    round_trip_test::<crate::RealtimeServerEventConversationItemTruncated>(
        REALTIME_SERVER_EVENT_CONVERSATION_ITEM_TRUNCATED_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventError example value
#[test]
fn test_realtime_server_event_error() {
    const REALTIME_SERVER_EVENT_ERROR_EXAMPLE: &str = r#"{
    "event_id": "event_890",
    "type": "error",
    "error": {
        "type": "invalid_request_error",
        "code": "invalid_event",
        "message": "The 'type' field is missing.",
        "param": null,
        "event_id": "event_567"
    }
}"#;
    round_trip_test::<crate::RealtimeServerEventError>(REALTIME_SERVER_EVENT_ERROR_EXAMPLE)
        .unwrap();
}
/// Test for RealtimeServerEventInputAudioBufferCleared example value
#[test]
fn test_realtime_server_event_input_audio_buffer_cleared() {
    const REALTIME_SERVER_EVENT_INPUT_AUDIO_BUFFER_CLEARED_EXAMPLE: &str = r#"{
    "event_id": "event_1314",
    "type": "input_audio_buffer.cleared"
}"#;
    round_trip_test::<crate::RealtimeServerEventInputAudioBufferCleared>(
        REALTIME_SERVER_EVENT_INPUT_AUDIO_BUFFER_CLEARED_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventInputAudioBufferCommitted example value
#[test]
fn test_realtime_server_event_input_audio_buffer_committed() {
    const REALTIME_SERVER_EVENT_INPUT_AUDIO_BUFFER_COMMITTED_EXAMPLE: &str = r#"{
    "event_id": "event_1121",
    "type": "input_audio_buffer.committed",
    "previous_item_id": "msg_001",
    "item_id": "msg_002"
}"#;
    round_trip_test::<crate::RealtimeServerEventInputAudioBufferCommitted>(
        REALTIME_SERVER_EVENT_INPUT_AUDIO_BUFFER_COMMITTED_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventInputAudioBufferSpeechStarted example value
#[test]
fn test_realtime_server_event_input_audio_buffer_speech_started() {
    const REALTIME_SERVER_EVENT_INPUT_AUDIO_BUFFER_SPEECH_STARTED_EXAMPLE: &str = r#"{
    "event_id": "event_1516",
    "type": "input_audio_buffer.speech_started",
    "audio_start_ms": 1000,
    "item_id": "msg_003"
}"#;
    round_trip_test::<crate::RealtimeServerEventInputAudioBufferSpeechStarted>(
        REALTIME_SERVER_EVENT_INPUT_AUDIO_BUFFER_SPEECH_STARTED_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventInputAudioBufferSpeechStopped example value
#[test]
fn test_realtime_server_event_input_audio_buffer_speech_stopped() {
    const REALTIME_SERVER_EVENT_INPUT_AUDIO_BUFFER_SPEECH_STOPPED_EXAMPLE: &str = r#"{
    "event_id": "event_1718",
    "type": "input_audio_buffer.speech_stopped",
    "audio_end_ms": 2000,
    "item_id": "msg_003"
}"#;
    round_trip_test::<crate::RealtimeServerEventInputAudioBufferSpeechStopped>(
        REALTIME_SERVER_EVENT_INPUT_AUDIO_BUFFER_SPEECH_STOPPED_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventOutputAudioBufferCleared example value
#[test]
fn test_realtime_server_event_output_audio_buffer_cleared() {
    const REALTIME_SERVER_EVENT_OUTPUT_AUDIO_BUFFER_CLEARED_EXAMPLE: &str = r#"{
    "event_id": "event_abc123",
    "type": "output_audio_buffer.cleared",
    "response_id": "resp_abc123"
}"#;
    round_trip_test::<crate::RealtimeServerEventOutputAudioBufferCleared>(
        REALTIME_SERVER_EVENT_OUTPUT_AUDIO_BUFFER_CLEARED_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventOutputAudioBufferStarted example value
#[test]
fn test_realtime_server_event_output_audio_buffer_started() {
    const REALTIME_SERVER_EVENT_OUTPUT_AUDIO_BUFFER_STARTED_EXAMPLE: &str = r#"{
    "event_id": "event_abc123",
    "type": "output_audio_buffer.started",
    "response_id": "resp_abc123"
}"#;
    round_trip_test::<crate::RealtimeServerEventOutputAudioBufferStarted>(
        REALTIME_SERVER_EVENT_OUTPUT_AUDIO_BUFFER_STARTED_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventOutputAudioBufferStopped example value
#[test]
fn test_realtime_server_event_output_audio_buffer_stopped() {
    const REALTIME_SERVER_EVENT_OUTPUT_AUDIO_BUFFER_STOPPED_EXAMPLE: &str = r#"{
    "event_id": "event_abc123",
    "type": "output_audio_buffer.stopped",
    "response_id": "resp_abc123"
}"#;
    round_trip_test::<crate::RealtimeServerEventOutputAudioBufferStopped>(
        REALTIME_SERVER_EVENT_OUTPUT_AUDIO_BUFFER_STOPPED_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventRateLimitsUpdated example value
#[test]
#[should_panic]
fn test_realtime_server_event_rate_limits_updated() {
    const REALTIME_SERVER_EVENT_RATE_LIMITS_UPDATED_EXAMPLE: &str = r#"{
    "event_id": "event_5758",
    "type": "rate_limits.updated",
    "rate_limits": [
        {
            "name": "requests",
            "limit": 1000,
            "remaining": 999,
            "reset_seconds": 60
        },
        {
            "name": "tokens",
            "limit": 50000,
            "remaining": 49950,
            "reset_seconds": 60
        }
    ]
}"#;
    round_trip_test::<crate::RealtimeServerEventRateLimitsUpdated>(
        REALTIME_SERVER_EVENT_RATE_LIMITS_UPDATED_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventResponseAudioDelta example value
#[test]
fn test_realtime_server_event_response_audio_delta() {
    const REALTIME_SERVER_EVENT_RESPONSE_AUDIO_DELTA_EXAMPLE: &str = r#"{
    "event_id": "event_4950",
    "type": "response.audio.delta",
    "response_id": "resp_001",
    "item_id": "msg_008",
    "output_index": 0,
    "content_index": 0,
    "delta": "Base64EncodedAudioDelta"
}"#;
    round_trip_test::<crate::RealtimeServerEventResponseAudioDelta>(
        REALTIME_SERVER_EVENT_RESPONSE_AUDIO_DELTA_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventResponseAudioDone example value
#[test]
fn test_realtime_server_event_response_audio_done() {
    const REALTIME_SERVER_EVENT_RESPONSE_AUDIO_DONE_EXAMPLE: &str = r#"{
    "event_id": "event_5152",
    "type": "response.audio.done",
    "response_id": "resp_001",
    "item_id": "msg_008",
    "output_index": 0,
    "content_index": 0
}"#;
    round_trip_test::<crate::RealtimeServerEventResponseAudioDone>(
        REALTIME_SERVER_EVENT_RESPONSE_AUDIO_DONE_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventResponseAudioTranscriptDelta example value
#[test]
fn test_realtime_server_event_response_audio_transcript_delta() {
    const REALTIME_SERVER_EVENT_RESPONSE_AUDIO_TRANSCRIPT_DELTA_EXAMPLE: &str = r#"{
    "event_id": "event_4546",
    "type": "response.audio_transcript.delta",
    "response_id": "resp_001",
    "item_id": "msg_008",
    "output_index": 0,
    "content_index": 0,
    "delta": "Hello, how can I a"
}"#;
    round_trip_test::<crate::RealtimeServerEventResponseAudioTranscriptDelta>(
        REALTIME_SERVER_EVENT_RESPONSE_AUDIO_TRANSCRIPT_DELTA_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventResponseAudioTranscriptDone example value
#[test]
fn test_realtime_server_event_response_audio_transcript_done() {
    const REALTIME_SERVER_EVENT_RESPONSE_AUDIO_TRANSCRIPT_DONE_EXAMPLE: &str = r#"{
    "event_id": "event_4748",
    "type": "response.audio_transcript.done",
    "response_id": "resp_001",
    "item_id": "msg_008",
    "output_index": 0,
    "content_index": 0,
    "transcript": "Hello, how can I assist you today?"
}"#;
    round_trip_test::<crate::RealtimeServerEventResponseAudioTranscriptDone>(
        REALTIME_SERVER_EVENT_RESPONSE_AUDIO_TRANSCRIPT_DONE_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventResponseContentPartAdded example value
#[test]
fn test_realtime_server_event_response_content_part_added() {
    const REALTIME_SERVER_EVENT_RESPONSE_CONTENT_PART_ADDED_EXAMPLE: &str = r#"{
    "event_id": "event_3738",
    "type": "response.content_part.added",
    "response_id": "resp_001",
    "item_id": "msg_007",
    "output_index": 0,
    "content_index": 0,
    "part": {
        "type": "text",
        "text": ""
    }
}"#;
    round_trip_test::<crate::RealtimeServerEventResponseContentPartAdded>(
        REALTIME_SERVER_EVENT_RESPONSE_CONTENT_PART_ADDED_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventResponseContentPartDone example value
#[test]
fn test_realtime_server_event_response_content_part_done() {
    const REALTIME_SERVER_EVENT_RESPONSE_CONTENT_PART_DONE_EXAMPLE: &str = r#"{
    "event_id": "event_3940",
    "type": "response.content_part.done",
    "response_id": "resp_001",
    "item_id": "msg_007",
    "output_index": 0,
    "content_index": 0,
    "part": {
        "type": "text",
        "text": "Sure, I can help with that."
    }
}"#;
    round_trip_test::<crate::RealtimeServerEventResponseContentPartDone>(
        REALTIME_SERVER_EVENT_RESPONSE_CONTENT_PART_DONE_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventResponseCreated example value
#[test]
fn test_realtime_server_event_response_created() {
    const REALTIME_SERVER_EVENT_RESPONSE_CREATED_EXAMPLE: &str = r#"{
    "event_id": "event_2930",
    "type": "response.created",
    "response": {
        "id": "resp_001",
        "object": "realtime.response",
        "status": "in_progress",
        "status_details": null,
        "output": [],
        "usage": null
    }
}"#;
    round_trip_test::<crate::RealtimeServerEventResponseCreated>(
        REALTIME_SERVER_EVENT_RESPONSE_CREATED_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventResponseDone example value
#[test]
#[should_panic]
fn test_realtime_server_event_response_done() {
    const REALTIME_SERVER_EVENT_RESPONSE_DONE_EXAMPLE: &str = r#"{
    "event_id": "event_3132",
    "type": "response.done",
    "response": {
        "id": "resp_001",
        "object": "realtime.response",
        "status": "completed",
        "status_details": null,
        "output": [
            {
                "id": "msg_006",
                "object": "realtime.item",
                "type": "message",
                "status": "completed",
                "role": "assistant",
                "content": [
                    {
                        "type": "text",
                        "text": "Sure, how can I assist you today?"
                    }
                ]
            }
        ],
        "usage": {
            "total_tokens":275,
            "input_tokens":127,
            "output_tokens":148,
            "input_token_details": {
                "cached_tokens":384,
                "text_tokens":119,
                "audio_tokens":8,
                "cached_tokens_details": {
                    "text_tokens": 128,
                    "audio_tokens": 256
                }
            },
            "output_token_details": {
              "text_tokens":36,
              "audio_tokens":112
            }
        }
    }
}"#;
    round_trip_test::<crate::RealtimeServerEventResponseDone>(
        REALTIME_SERVER_EVENT_RESPONSE_DONE_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventResponseFunctionCallArgumentsDelta example value
#[test]
fn test_realtime_server_event_response_function_call_arguments_delta() {
    const REALTIME_SERVER_EVENT_RESPONSE_FUNCTION_CALL_ARGUMENTS_DELTA_EXAMPLE: &str = r#"{
    "event_id": "event_5354",
    "type": "response.function_call_arguments.delta",
    "response_id": "resp_002",
    "item_id": "fc_001",
    "output_index": 0,
    "call_id": "call_001",
    "delta": "{\"location\": \"San\""
}"#;
    round_trip_test::<crate::RealtimeServerEventResponseFunctionCallArgumentsDelta>(
        REALTIME_SERVER_EVENT_RESPONSE_FUNCTION_CALL_ARGUMENTS_DELTA_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventResponseFunctionCallArgumentsDone example value
#[test]
fn test_realtime_server_event_response_function_call_arguments_done() {
    const REALTIME_SERVER_EVENT_RESPONSE_FUNCTION_CALL_ARGUMENTS_DONE_EXAMPLE: &str = r#"{
    "event_id": "event_5556",
    "type": "response.function_call_arguments.done",
    "response_id": "resp_002",
    "item_id": "fc_001",
    "output_index": 0,
    "call_id": "call_001",
    "arguments": "{\"location\": \"San Francisco\"}"
}"#;
    round_trip_test::<crate::RealtimeServerEventResponseFunctionCallArgumentsDone>(
        REALTIME_SERVER_EVENT_RESPONSE_FUNCTION_CALL_ARGUMENTS_DONE_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventResponseOutputItemAdded example value
#[test]
fn test_realtime_server_event_response_output_item_added() {
    const REALTIME_SERVER_EVENT_RESPONSE_OUTPUT_ITEM_ADDED_EXAMPLE: &str = r#"{
    "event_id": "event_3334",
    "type": "response.output_item.added",
    "response_id": "resp_001",
    "output_index": 0,
    "item": {
        "id": "msg_007",
        "object": "realtime.item",
        "type": "message",
        "status": "in_progress",
        "role": "assistant",
        "content": []
    }
}"#;
    round_trip_test::<crate::RealtimeServerEventResponseOutputItemAdded>(
        REALTIME_SERVER_EVENT_RESPONSE_OUTPUT_ITEM_ADDED_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventResponseOutputItemDone example value
#[test]
fn test_realtime_server_event_response_output_item_done() {
    const REALTIME_SERVER_EVENT_RESPONSE_OUTPUT_ITEM_DONE_EXAMPLE: &str = r#"{
    "event_id": "event_3536",
    "type": "response.output_item.done",
    "response_id": "resp_001",
    "output_index": 0,
    "item": {
        "id": "msg_007",
        "object": "realtime.item",
        "type": "message",
        "status": "completed",
        "role": "assistant",
        "content": [
            {
                "type": "text",
                "text": "Sure, I can help with that."
            }
        ]
    }
}"#;
    round_trip_test::<crate::RealtimeServerEventResponseOutputItemDone>(
        REALTIME_SERVER_EVENT_RESPONSE_OUTPUT_ITEM_DONE_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventResponseTextDelta example value
#[test]
fn test_realtime_server_event_response_text_delta() {
    const REALTIME_SERVER_EVENT_RESPONSE_TEXT_DELTA_EXAMPLE: &str = r#"{
    "event_id": "event_4142",
    "type": "response.text.delta",
    "response_id": "resp_001",
    "item_id": "msg_007",
    "output_index": 0,
    "content_index": 0,
    "delta": "Sure, I can h"
}"#;
    round_trip_test::<crate::RealtimeServerEventResponseTextDelta>(
        REALTIME_SERVER_EVENT_RESPONSE_TEXT_DELTA_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventResponseTextDone example value
#[test]
fn test_realtime_server_event_response_text_done() {
    const REALTIME_SERVER_EVENT_RESPONSE_TEXT_DONE_EXAMPLE: &str = r#"{
    "event_id": "event_4344",
    "type": "response.text.done",
    "response_id": "resp_001",
    "item_id": "msg_007",
    "output_index": 0,
    "content_index": 0,
    "text": "Sure, I can help with that."
}"#;
    round_trip_test::<crate::RealtimeServerEventResponseTextDone>(
        REALTIME_SERVER_EVENT_RESPONSE_TEXT_DONE_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventSessionCreated example value
#[test]
#[should_panic]
fn test_realtime_server_event_session_created() {
    const REALTIME_SERVER_EVENT_SESSION_CREATED_EXAMPLE: &str = r#"{
    "event_id": "event_1234",
    "type": "session.created",
    "session": {
        "id": "sess_001",
        "object": "realtime.session",
        "model": "gpt-4o-realtime-preview",
        "modalities": ["text", "audio"],
        "instructions": "...model instructions here...",
        "voice": "sage",
        "input_audio_format": "pcm16",
        "output_audio_format": "pcm16",
        "input_audio_transcription": null,
        "turn_detection": {
            "type": "server_vad",
            "threshold": 0.5,
            "prefix_padding_ms": 300,
            "silence_duration_ms": 200
        },
        "tools": [],
        "tool_choice": "auto",
        "temperature": 0.8,
        "max_response_output_tokens": "inf",
        "speed": 1.1,
        "tracing": "auto"
    }
}"#;
    round_trip_test::<crate::RealtimeServerEventSessionCreated>(
        REALTIME_SERVER_EVENT_SESSION_CREATED_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeServerEventSessionUpdated example value
#[test]
#[should_panic]
fn test_realtime_server_event_session_updated() {
    const REALTIME_SERVER_EVENT_SESSION_UPDATED_EXAMPLE: &str = r#"{
    "event_id": "event_5678",
    "type": "session.updated",
    "session": {
        "id": "sess_001",
        "object": "realtime.session",
        "model": "gpt-4o-realtime-preview",
        "modalities": ["text"],
        "instructions": "New instructions",
        "voice": "sage",
        "input_audio_format": "pcm16",
        "output_audio_format": "pcm16",
        "input_audio_transcription": {
            "model": "whisper-1"
        },
        "turn_detection": null,
        "tools": [],
        "tool_choice": "none",
        "temperature": 0.7,
        "max_response_output_tokens": 200,
        "speed": 1.1,
        "tracing": "auto"
    }
}"#;
    round_trip_test::<crate::RealtimeServerEventSessionUpdated>(
        REALTIME_SERVER_EVENT_SESSION_UPDATED_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeSessionCreateResponse example value
#[test]
#[should_panic]
fn test_realtime_session_create_response() {
    const REALTIME_SESSION_CREATE_RESPONSE_EXAMPLE: &str = r#"{
  "id": "sess_001",
  "object": "realtime.session",
  "model": "gpt-4o-realtime-preview",
  "modalities": ["audio", "text"],
  "instructions": "You are a friendly assistant.",
  "voice": "alloy",
  "input_audio_format": "pcm16",
  "output_audio_format": "pcm16",
  "input_audio_transcription": {
      "model": "whisper-1"
  },
  "turn_detection": null,
  "tools": [],
  "tool_choice": "none",
  "temperature": 0.7,
  "speed": 1.1,
  "tracing": "auto",
  "max_response_output_tokens": 200,
  "client_secret": {
    "value": "ek_abc123",
    "expires_at": 1234567890
  }
}"#;
    round_trip_test::<crate::RealtimeSessionCreateResponse>(
        REALTIME_SESSION_CREATE_RESPONSE_EXAMPLE,
    )
    .unwrap();
}
/// Test for RealtimeTranscriptionSessionCreateResponse example value
#[test]
#[should_panic]
fn test_realtime_transcription_session_create_response() {
    const REALTIME_TRANSCRIPTION_SESSION_CREATE_RESPONSE_EXAMPLE: &str = r#"{
  "id": "sess_BBwZc7cFV3XizEyKGDCGL",
  "object": "realtime.transcription_session",
  "expires_at": 1742188264,
  "modalities": ["audio", "text"],
  "turn_detection": {
    "type": "server_vad",
    "threshold": 0.5,
    "prefix_padding_ms": 300,
    "silence_duration_ms": 200
  },
  "input_audio_format": "pcm16",
  "input_audio_transcription": {
    "model": "gpt-4o-transcribe",
    "language": null,
    "prompt": ""
  },
  "client_secret": null
}"#;
    round_trip_test::<crate::RealtimeTranscriptionSessionCreateResponse>(
        REALTIME_TRANSCRIPTION_SESSION_CREATE_RESPONSE_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseAudioDeltaEvent example value
#[test]
fn test_response_audio_delta_event() {
    const RESPONSE_AUDIO_DELTA_EVENT_EXAMPLE: &str = r#"{
  "type": "response.audio.delta",
  "response_id": "resp_123",
  "delta": "base64encoded...",
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseAudioDeltaEvent>(RESPONSE_AUDIO_DELTA_EVENT_EXAMPLE).unwrap();
}
/// Test for ResponseAudioDoneEvent example value
#[test]
fn test_response_audio_done_event() {
    const RESPONSE_AUDIO_DONE_EVENT_EXAMPLE: &str = r#"{
  "type": "response.audio.done",
  "response_id": "resp-123",
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseAudioDoneEvent>(RESPONSE_AUDIO_DONE_EVENT_EXAMPLE).unwrap();
}
/// Test for ResponseAudioTranscriptDeltaEvent example value
#[test]
fn test_response_audio_transcript_delta_event() {
    const RESPONSE_AUDIO_TRANSCRIPT_DELTA_EVENT_EXAMPLE: &str = r#"{
  "type": "response.audio.transcript.delta",
  "response_id": "resp_123",
  "delta": " ... partial transcript ... ",
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseAudioTranscriptDeltaEvent>(
        RESPONSE_AUDIO_TRANSCRIPT_DELTA_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseAudioTranscriptDoneEvent example value
#[test]
fn test_response_audio_transcript_done_event() {
    const RESPONSE_AUDIO_TRANSCRIPT_DONE_EVENT_EXAMPLE: &str = r#"{
  "type": "response.audio.transcript.done",
  "response_id": "resp_123",
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseAudioTranscriptDoneEvent>(
        RESPONSE_AUDIO_TRANSCRIPT_DONE_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseCodeInterpreterCallCodeDeltaEvent example value
#[test]
fn test_response_code_interpreter_call_code_delta_event() {
    const RESPONSE_CODE_INTERPRETER_CALL_CODE_DELTA_EVENT_EXAMPLE: &str = r#"{
  "type": "response.code_interpreter_call_code.delta",
  "output_index": 0,
  "item_id": "ci_12345",
  "delta": "print('Hello, world')",
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseCodeInterpreterCallCodeDeltaEvent>(
        RESPONSE_CODE_INTERPRETER_CALL_CODE_DELTA_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseCodeInterpreterCallCodeDoneEvent example value
#[test]
fn test_response_code_interpreter_call_code_done_event() {
    const RESPONSE_CODE_INTERPRETER_CALL_CODE_DONE_EVENT_EXAMPLE: &str = r#"{
  "type": "response.code_interpreter_call_code.done",
  "output_index": 3,
  "item_id": "ci_12345",
  "code": "print('done')",
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseCodeInterpreterCallCodeDoneEvent>(
        RESPONSE_CODE_INTERPRETER_CALL_CODE_DONE_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseCodeInterpreterCallCompletedEvent example value
#[test]
fn test_response_code_interpreter_call_completed_event() {
    const RESPONSE_CODE_INTERPRETER_CALL_COMPLETED_EVENT_EXAMPLE: &str = r#"{
  "type": "response.code_interpreter_call.completed",
  "output_index": 5,
  "item_id": "ci_12345",
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseCodeInterpreterCallCompletedEvent>(
        RESPONSE_CODE_INTERPRETER_CALL_COMPLETED_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseCodeInterpreterCallInProgressEvent example value
#[test]
fn test_response_code_interpreter_call_in_progress_event() {
    const RESPONSE_CODE_INTERPRETER_CALL_IN_PROGRESS_EVENT_EXAMPLE: &str = r#"{
  "type": "response.code_interpreter_call.in_progress",
  "output_index": 0,
  "item_id": "ci_12345",
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseCodeInterpreterCallInProgressEvent>(
        RESPONSE_CODE_INTERPRETER_CALL_IN_PROGRESS_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseCodeInterpreterCallInterpretingEvent example value
#[test]
fn test_response_code_interpreter_call_interpreting_event() {
    const RESPONSE_CODE_INTERPRETER_CALL_INTERPRETING_EVENT_EXAMPLE: &str = r#"{
  "type": "response.code_interpreter_call.interpreting",
  "output_index": 4,
  "item_id": "ci_12345",
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseCodeInterpreterCallInterpretingEvent>(
        RESPONSE_CODE_INTERPRETER_CALL_INTERPRETING_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseCompletedEvent example value
#[test]
#[should_panic]
fn test_response_completed_event() {
    const RESPONSE_COMPLETED_EVENT_EXAMPLE: &str = r#"{
  "type": "response.completed",
  "response": {
    "id": "resp_123",
    "object": "response",
    "created_at": 1740855869,
    "status": "completed",
    "error": null,
    "incomplete_details": null,
    "input": [],
    "instructions": null,
    "max_output_tokens": null,
    "model": "gpt-4o-mini-2024-07-18",
    "output": [
      {
        "id": "msg_123",
        "type": "message",
        "role": "assistant",
        "content": [
          {
            "type": "output_text",
            "text": "In a shimmering forest under a sky full of stars, a lonely unicorn named Lila discovered a hidden pond that glowed with moonlight. Every night, she would leave sparkling, magical flowers by the water's edge, hoping to share her beauty with others. One enchanting evening, she woke to find a group of friendly animals gathered around, eager to be friends and share in her magic.",
            "annotations": []
          }
        ]
      }
    ],
    "previous_response_id": null,
    "reasoning_effort": null,
    "store": false,
    "temperature": 1,
    "text": {
      "format": {
        "type": "text"
      }
    },
    "tool_choice": "auto",
    "tools": [],
    "top_p": 1,
    "truncation": "disabled",
    "usage": {
      "input_tokens": 0,
      "output_tokens": 0,
      "output_tokens_details": {
        "reasoning_tokens": 0
      },
      "total_tokens": 0
    },
    "user": null,
    "metadata": {}
  },
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseCompletedEvent>(RESPONSE_COMPLETED_EVENT_EXAMPLE).unwrap();
}
/// Test for ResponseContentPartAddedEvent example value
#[test]
fn test_response_content_part_added_event() {
    const RESPONSE_CONTENT_PART_ADDED_EVENT_EXAMPLE: &str = r#"{
  "type": "response.content_part.added",
  "item_id": "msg_123",
  "output_index": 0,
  "content_index": 0,
  "part": {
    "type": "output_text",
    "text": "",
    "annotations": []
  },
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseContentPartAddedEvent>(
        RESPONSE_CONTENT_PART_ADDED_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseContentPartDoneEvent example value
#[test]
fn test_response_content_part_done_event() {
    const RESPONSE_CONTENT_PART_DONE_EVENT_EXAMPLE: &str = r#"{
  "type": "response.content_part.done",
  "item_id": "msg_123",
  "output_index": 0,
  "content_index": 0,
  "sequence_number": 1,
  "part": {
    "type": "output_text",
    "text": "In a shimmering forest under a sky full of stars, a lonely unicorn named Lila discovered a hidden pond that glowed with moonlight. Every night, she would leave sparkling, magical flowers by the water's edge, hoping to share her beauty with others. One enchanting evening, she woke to find a group of friendly animals gathered around, eager to be friends and share in her magic.",
    "annotations": []
  }
}"#;
    round_trip_test::<crate::ResponseContentPartDoneEvent>(
        RESPONSE_CONTENT_PART_DONE_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseCreatedEvent example value
#[test]
#[should_panic]
fn test_response_created_event() {
    const RESPONSE_CREATED_EVENT_EXAMPLE: &str = r#"{
  "type": "response.created",
  "response": {
    "id": "resp_67ccfcdd16748190a91872c75d38539e09e4d4aac714747c",
    "object": "response",
    "created_at": 1741487325,
    "status": "in_progress",
    "error": null,
    "incomplete_details": null,
    "instructions": null,
    "max_output_tokens": null,
    "model": "gpt-4o-2024-08-06",
    "output": [],
    "parallel_tool_calls": true,
    "previous_response_id": null,
    "reasoning": {
      "effort": null,
      "summary": null
    },
    "store": true,
    "temperature": 1,
    "text": {
      "format": {
        "type": "text"
      }
    },
    "tool_choice": "auto",
    "tools": [],
    "top_p": 1,
    "truncation": "disabled",
    "usage": null,
    "user": null,
    "metadata": {}
  },
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseCreatedEvent>(RESPONSE_CREATED_EVENT_EXAMPLE).unwrap();
}
/// Test for ResponseCustomToolCallInputDeltaEvent example value
#[test]
#[should_panic]
fn test_response_custom_tool_call_input_delta_event() {
    const RESPONSE_CUSTOM_TOOL_CALL_INPUT_DELTA_EVENT_EXAMPLE: &str = r#"{
  "type": "response.custom_tool_call_input.delta",
  "output_index": 0,
  "item_id": "ctc_1234567890abcdef",
  "delta": "partial input text"
}"#;
    round_trip_test::<crate::ResponseCustomToolCallInputDeltaEvent>(
        RESPONSE_CUSTOM_TOOL_CALL_INPUT_DELTA_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseCustomToolCallInputDoneEvent example value
#[test]
#[should_panic]
fn test_response_custom_tool_call_input_done_event() {
    const RESPONSE_CUSTOM_TOOL_CALL_INPUT_DONE_EVENT_EXAMPLE: &str = r#"{
  "type": "response.custom_tool_call_input.done",
  "output_index": 0,
  "item_id": "ctc_1234567890abcdef",
  "input": "final complete input text"
}"#;
    round_trip_test::<crate::ResponseCustomToolCallInputDoneEvent>(
        RESPONSE_CUSTOM_TOOL_CALL_INPUT_DONE_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseErrorEvent example value
#[test]
fn test_response_error_event() {
    const RESPONSE_ERROR_EVENT_EXAMPLE: &str = r#"{
  "type": "error",
  "code": "ERR_SOMETHING",
  "message": "Something went wrong",
  "param": null,
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseErrorEvent>(RESPONSE_ERROR_EVENT_EXAMPLE).unwrap();
}
/// Test for ResponseFailedEvent example value
#[test]
#[should_panic]
fn test_response_failed_event() {
    const RESPONSE_FAILED_EVENT_EXAMPLE: &str = r#"{
  "type": "response.failed",
  "response": {
    "id": "resp_123",
    "object": "response",
    "created_at": 1740855869,
    "status": "failed",
    "error": {
      "code": "server_error",
      "message": "The model failed to generate a response."
    },
    "incomplete_details": null,
    "instructions": null,
    "max_output_tokens": null,
    "model": "gpt-4o-mini-2024-07-18",
    "output": [],
    "previous_response_id": null,
    "reasoning_effort": null,
    "store": false,
    "temperature": 1,
    "text": {
      "format": {
        "type": "text"
      }
    },
    "tool_choice": "auto",
    "tools": [],
    "top_p": 1,
    "truncation": "disabled",
    "usage": null,
    "user": null,
    "metadata": {}
  }
}"#;
    round_trip_test::<crate::ResponseFailedEvent>(RESPONSE_FAILED_EVENT_EXAMPLE).unwrap();
}
/// Test for ResponseFileSearchCallCompletedEvent example value
#[test]
fn test_response_file_search_call_completed_event() {
    const RESPONSE_FILE_SEARCH_CALL_COMPLETED_EVENT_EXAMPLE: &str = r#"{
  "type": "response.file_search_call.completed",
  "output_index": 0,
  "item_id": "fs_123",
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseFileSearchCallCompletedEvent>(
        RESPONSE_FILE_SEARCH_CALL_COMPLETED_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseFileSearchCallInProgressEvent example value
#[test]
fn test_response_file_search_call_in_progress_event() {
    const RESPONSE_FILE_SEARCH_CALL_IN_PROGRESS_EVENT_EXAMPLE: &str = r#"{
  "type": "response.file_search_call.in_progress",
  "output_index": 0,
  "item_id": "fs_123",
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseFileSearchCallInProgressEvent>(
        RESPONSE_FILE_SEARCH_CALL_IN_PROGRESS_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseFileSearchCallSearchingEvent example value
#[test]
fn test_response_file_search_call_searching_event() {
    const RESPONSE_FILE_SEARCH_CALL_SEARCHING_EVENT_EXAMPLE: &str = r#"{
  "type": "response.file_search_call.searching",
  "output_index": 0,
  "item_id": "fs_123",
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseFileSearchCallSearchingEvent>(
        RESPONSE_FILE_SEARCH_CALL_SEARCHING_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseFunctionCallArgumentsDoneEvent example value
#[test]
fn test_response_function_call_arguments_done_event() {
    const RESPONSE_FUNCTION_CALL_ARGUMENTS_DONE_EVENT_EXAMPLE: &str = r#"{
  "type": "response.function_call_arguments.done",
  "item_id": "item-abc",
  "output_index": 1,
  "arguments": "{ \"arg\": 123 }",
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseFunctionCallArgumentsDoneEvent>(
        RESPONSE_FUNCTION_CALL_ARGUMENTS_DONE_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseImageGenCallCompletedEvent example value
#[test]
fn test_response_image_gen_call_completed_event() {
    const RESPONSE_IMAGE_GEN_CALL_COMPLETED_EVENT_EXAMPLE: &str = r#"{
  "type": "response.image_generation_call.completed",
  "output_index": 0,
  "item_id": "item-123",
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseImageGenCallCompletedEvent>(
        RESPONSE_IMAGE_GEN_CALL_COMPLETED_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseImageGenCallGeneratingEvent example value
#[test]
fn test_response_image_gen_call_generating_event() {
    const RESPONSE_IMAGE_GEN_CALL_GENERATING_EVENT_EXAMPLE: &str = r#"{
  "type": "response.image_generation_call.generating",
  "output_index": 0,
  "item_id": "item-123",
  "sequence_number": 0
}"#;
    round_trip_test::<crate::ResponseImageGenCallGeneratingEvent>(
        RESPONSE_IMAGE_GEN_CALL_GENERATING_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseImageGenCallInProgressEvent example value
#[test]
fn test_response_image_gen_call_in_progress_event() {
    const RESPONSE_IMAGE_GEN_CALL_IN_PROGRESS_EVENT_EXAMPLE: &str = r#"{
  "type": "response.image_generation_call.in_progress",
  "output_index": 0,
  "item_id": "item-123",
  "sequence_number": 0
}"#;
    round_trip_test::<crate::ResponseImageGenCallInProgressEvent>(
        RESPONSE_IMAGE_GEN_CALL_IN_PROGRESS_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseImageGenCallPartialImageEvent example value
#[test]
fn test_response_image_gen_call_partial_image_event() {
    const RESPONSE_IMAGE_GEN_CALL_PARTIAL_IMAGE_EVENT_EXAMPLE: &str = r#"{
  "type": "response.image_generation_call.partial_image",
  "output_index": 0,
  "item_id": "item-123",
  "sequence_number": 0,
  "partial_image_index": 0,
  "partial_image_b64": "..."
}"#;
    round_trip_test::<crate::ResponseImageGenCallPartialImageEvent>(
        RESPONSE_IMAGE_GEN_CALL_PARTIAL_IMAGE_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseInProgressEvent example value
#[test]
#[should_panic]
fn test_response_in_progress_event() {
    const RESPONSE_IN_PROGRESS_EVENT_EXAMPLE: &str = r#"{
  "type": "response.in_progress",
  "response": {
    "id": "resp_67ccfcdd16748190a91872c75d38539e09e4d4aac714747c",
    "object": "response",
    "created_at": 1741487325,
    "status": "in_progress",
    "error": null,
    "incomplete_details": null,
    "instructions": null,
    "max_output_tokens": null,
    "model": "gpt-4o-2024-08-06",
    "output": [],
    "parallel_tool_calls": true,
    "previous_response_id": null,
    "reasoning": {
      "effort": null,
      "summary": null
    },
    "store": true,
    "temperature": 1,
    "text": {
      "format": {
        "type": "text"
      }
    },
    "tool_choice": "auto",
    "tools": [],
    "top_p": 1,
    "truncation": "disabled",
    "usage": null,
    "user": null,
    "metadata": {}
  },
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseInProgressEvent>(RESPONSE_IN_PROGRESS_EVENT_EXAMPLE).unwrap();
}
/// Test for ResponseIncompleteEvent example value
#[test]
#[should_panic]
fn test_response_incomplete_event() {
    const RESPONSE_INCOMPLETE_EVENT_EXAMPLE: &str = r#"{
  "type": "response.incomplete",
  "response": {
    "id": "resp_123",
    "object": "response",
    "created_at": 1740855869,
    "status": "incomplete",
    "error": null,
    "incomplete_details": {
      "reason": "max_tokens"
    },
    "instructions": null,
    "max_output_tokens": null,
    "model": "gpt-4o-mini-2024-07-18",
    "output": [],
    "previous_response_id": null,
    "reasoning_effort": null,
    "store": false,
    "temperature": 1,
    "text": {
      "format": {
        "type": "text"
      }
    },
    "tool_choice": "auto",
    "tools": [],
    "top_p": 1,
    "truncation": "disabled",
    "usage": null,
    "user": null,
    "metadata": {}
  },
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseIncompleteEvent>(RESPONSE_INCOMPLETE_EVENT_EXAMPLE).unwrap();
}
/// Test for ResponseMCPCallArgumentsDeltaEvent example value
#[test]
fn test_response_m_c_p_call_arguments_delta_event() {
    const RESPONSE_M_C_P_CALL_ARGUMENTS_DELTA_EVENT_EXAMPLE: &str = r#"{
  "type": "response.mcp_call_arguments.delta",
  "output_index": 0,
  "item_id": "item-abc",
  "delta": "{",
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseMCPCallArgumentsDeltaEvent>(
        RESPONSE_M_C_P_CALL_ARGUMENTS_DELTA_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseMCPCallArgumentsDoneEvent example value
#[test]
fn test_response_m_c_p_call_arguments_done_event() {
    const RESPONSE_M_C_P_CALL_ARGUMENTS_DONE_EVENT_EXAMPLE: &str = r#"{
  "type": "response.mcp_call_arguments.done",
  "output_index": 0,
  "item_id": "item-abc",
  "arguments": "{\"arg1\": \"value1\", \"arg2\": \"value2\"}",
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseMCPCallArgumentsDoneEvent>(
        RESPONSE_M_C_P_CALL_ARGUMENTS_DONE_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseMCPCallCompletedEvent example value
#[test]
fn test_response_m_c_p_call_completed_event() {
    const RESPONSE_M_C_P_CALL_COMPLETED_EVENT_EXAMPLE: &str = r#"{
  "type": "response.mcp_call.completed",
  "sequence_number": 1,
  "item_id": "mcp_682d437d90a88191bf88cd03aae0c3e503937d5f622d7a90",
  "output_index": 0
}"#;
    round_trip_test::<crate::ResponseMCPCallCompletedEvent>(
        RESPONSE_M_C_P_CALL_COMPLETED_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseMCPCallFailedEvent example value
#[test]
fn test_response_m_c_p_call_failed_event() {
    const RESPONSE_M_C_P_CALL_FAILED_EVENT_EXAMPLE: &str = r#"{
  "type": "response.mcp_call.failed",
  "sequence_number": 1,
  "item_id": "mcp_682d437d90a88191bf88cd03aae0c3e503937d5f622d7a90",
  "output_index": 0
}"#;
    round_trip_test::<crate::ResponseMCPCallFailedEvent>(RESPONSE_M_C_P_CALL_FAILED_EVENT_EXAMPLE)
        .unwrap();
}
/// Test for ResponseMCPCallInProgressEvent example value
#[test]
fn test_response_m_c_p_call_in_progress_event() {
    const RESPONSE_M_C_P_CALL_IN_PROGRESS_EVENT_EXAMPLE: &str = r#"{
  "type": "response.mcp_call.in_progress",
  "sequence_number": 1,
  "output_index": 0,
  "item_id": "mcp_682d437d90a88191bf88cd03aae0c3e503937d5f622d7a90"
}"#;
    round_trip_test::<crate::ResponseMCPCallInProgressEvent>(
        RESPONSE_M_C_P_CALL_IN_PROGRESS_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseMCPListToolsCompletedEvent example value
#[test]
fn test_response_m_c_p_list_tools_completed_event() {
    const RESPONSE_M_C_P_LIST_TOOLS_COMPLETED_EVENT_EXAMPLE: &str = r#"{
  "type": "response.mcp_list_tools.completed",
  "sequence_number": 1,
  "output_index": 0,
  "item_id": "mcpl_682d4379df088191886b70f4ec39f90403937d5f622d7a90"
}"#;
    round_trip_test::<crate::ResponseMCPListToolsCompletedEvent>(
        RESPONSE_M_C_P_LIST_TOOLS_COMPLETED_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseMCPListToolsFailedEvent example value
#[test]
fn test_response_m_c_p_list_tools_failed_event() {
    const RESPONSE_M_C_P_LIST_TOOLS_FAILED_EVENT_EXAMPLE: &str = r#"{
  "type": "response.mcp_list_tools.failed",
  "sequence_number": 1,
  "output_index": 0,
  "item_id": "mcpl_682d4379df088191886b70f4ec39f90403937d5f622d7a90"
}"#;
    round_trip_test::<crate::ResponseMCPListToolsFailedEvent>(
        RESPONSE_M_C_P_LIST_TOOLS_FAILED_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseMCPListToolsInProgressEvent example value
#[test]
fn test_response_m_c_p_list_tools_in_progress_event() {
    const RESPONSE_M_C_P_LIST_TOOLS_IN_PROGRESS_EVENT_EXAMPLE: &str = r#"{
  "type": "response.mcp_list_tools.in_progress",
  "sequence_number": 1,
  "output_index": 0,
  "item_id": "mcpl_682d4379df088191886b70f4ec39f90403937d5f622d7a90"
}"#;
    round_trip_test::<crate::ResponseMCPListToolsInProgressEvent>(
        RESPONSE_M_C_P_LIST_TOOLS_IN_PROGRESS_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseOutputItemAddedEvent example value
#[test]
fn test_response_output_item_added_event() {
    const RESPONSE_OUTPUT_ITEM_ADDED_EVENT_EXAMPLE: &str = r#"{
  "type": "response.output_item.added",
  "output_index": 0,
  "item": {
    "id": "msg_123",
    "status": "in_progress",
    "type": "message",
    "role": "assistant",
    "content": []
  },
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseOutputItemAddedEvent>(
        RESPONSE_OUTPUT_ITEM_ADDED_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseOutputItemDoneEvent example value
#[test]
fn test_response_output_item_done_event() {
    const RESPONSE_OUTPUT_ITEM_DONE_EVENT_EXAMPLE: &str = r#"{
  "type": "response.output_item.done",
  "output_index": 0,
  "item": {
    "id": "msg_123",
    "status": "completed",
    "type": "message",
    "role": "assistant",
    "content": [
      {
        "type": "output_text",
        "text": "In a shimmering forest under a sky full of stars, a lonely unicorn named Lila discovered a hidden pond that glowed with moonlight. Every night, she would leave sparkling, magical flowers by the water's edge, hoping to share her beauty with others. One enchanting evening, she woke to find a group of friendly animals gathered around, eager to be friends and share in her magic.",
        "annotations": []
      }
    ]
  },
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseOutputItemDoneEvent>(RESPONSE_OUTPUT_ITEM_DONE_EVENT_EXAMPLE)
        .unwrap();
}
/// Test for ResponseOutputTextAnnotationAddedEvent example value
#[test]
fn test_response_output_text_annotation_added_event() {
    const RESPONSE_OUTPUT_TEXT_ANNOTATION_ADDED_EVENT_EXAMPLE: &str = r#"{
  "type": "response.output_text.annotation.added",
  "item_id": "item-abc",
  "output_index": 0,
  "content_index": 0,
  "annotation_index": 0,
  "annotation": {
    "type": "text_annotation",
    "text": "This is a test annotation",
    "start": 0,
    "end": 10
  },
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseOutputTextAnnotationAddedEvent>(
        RESPONSE_OUTPUT_TEXT_ANNOTATION_ADDED_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseQueuedEvent example value
#[test]
#[should_panic]
fn test_response_queued_event() {
    const RESPONSE_QUEUED_EVENT_EXAMPLE: &str = r#"{
  "type": "response.queued",
  "response": {
    "id": "res_123",
    "status": "queued",
    "created_at": "2021-01-01T00:00:00Z",
    "updated_at": "2021-01-01T00:00:00Z"
  },
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseQueuedEvent>(RESPONSE_QUEUED_EVENT_EXAMPLE).unwrap();
}
/// Test for ResponseReasoningSummaryPartAddedEvent example value
#[test]
fn test_response_reasoning_summary_part_added_event() {
    const RESPONSE_REASONING_SUMMARY_PART_ADDED_EVENT_EXAMPLE: &str = r#"{
  "type": "response.reasoning_summary_part.added",
  "item_id": "rs_6806bfca0b2481918a5748308061a2600d3ce51bdffd5476",
  "output_index": 0,
  "summary_index": 0,
  "part": {
    "type": "summary_text",
    "text": ""
  },
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseReasoningSummaryPartAddedEvent>(
        RESPONSE_REASONING_SUMMARY_PART_ADDED_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseReasoningSummaryPartDoneEvent example value
#[test]
fn test_response_reasoning_summary_part_done_event() {
    const RESPONSE_REASONING_SUMMARY_PART_DONE_EVENT_EXAMPLE: &str = r#"{
  "type": "response.reasoning_summary_part.done",
  "item_id": "rs_6806bfca0b2481918a5748308061a2600d3ce51bdffd5476",
  "output_index": 0,
  "summary_index": 0,
  "part": {
    "type": "summary_text",
    "text": "**Responding to a greeting**\n\nThe user just said, \"Hello!\" So, it seems I need to engage. I'll greet them back and offer help since they're looking to chat. I could say something like, \"Hello! How can I assist you today?\" That feels friendly and open. They didn't ask a specific question, so this approach will work well for starting a conversation. Let's see where it goes from there!"
  },
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseReasoningSummaryPartDoneEvent>(
        RESPONSE_REASONING_SUMMARY_PART_DONE_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseReasoningSummaryTextDeltaEvent example value
#[test]
fn test_response_reasoning_summary_text_delta_event() {
    const RESPONSE_REASONING_SUMMARY_TEXT_DELTA_EVENT_EXAMPLE: &str = r#"{
  "type": "response.reasoning_summary_text.delta",
  "item_id": "rs_6806bfca0b2481918a5748308061a2600d3ce51bdffd5476",
  "output_index": 0,
  "summary_index": 0,
  "delta": "**Responding to a greeting**\n\nThe user just said, \"Hello!\" So, it seems I need to engage. I'll greet them back and offer help since they're looking to chat. I could say something like, \"Hello! How can I assist you today?\" That feels friendly and open. They didn't ask a specific question, so this approach will work well for starting a conversation. Let's see where it goes from there!",
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseReasoningSummaryTextDeltaEvent>(
        RESPONSE_REASONING_SUMMARY_TEXT_DELTA_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseReasoningSummaryTextDoneEvent example value
#[test]
fn test_response_reasoning_summary_text_done_event() {
    const RESPONSE_REASONING_SUMMARY_TEXT_DONE_EVENT_EXAMPLE: &str = r#"{
  "type": "response.reasoning_summary_text.done",
  "item_id": "rs_6806bfca0b2481918a5748308061a2600d3ce51bdffd5476",
  "output_index": 0,
  "summary_index": 0,
  "text": "**Responding to a greeting**\n\nThe user just said, \"Hello!\" So, it seems I need to engage. I'll greet them back and offer help since they're looking to chat. I could say something like, \"Hello! How can I assist you today?\" That feels friendly and open. They didn't ask a specific question, so this approach will work well for starting a conversation. Let's see where it goes from there!",
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseReasoningSummaryTextDoneEvent>(
        RESPONSE_REASONING_SUMMARY_TEXT_DONE_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseReasoningTextDeltaEvent example value
#[test]
fn test_response_reasoning_text_delta_event() {
    const RESPONSE_REASONING_TEXT_DELTA_EVENT_EXAMPLE: &str = r#"{
  "type": "response.reasoning_text.delta",
  "item_id": "rs_123",
  "output_index": 0,
  "content_index": 0,
  "delta": "The",
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseReasoningTextDeltaEvent>(
        RESPONSE_REASONING_TEXT_DELTA_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseReasoningTextDoneEvent example value
#[test]
fn test_response_reasoning_text_done_event() {
    const RESPONSE_REASONING_TEXT_DONE_EVENT_EXAMPLE: &str = r#"{
  "type": "response.reasoning_text.done",
  "item_id": "rs_123",
  "output_index": 0,
  "content_index": 0,
  "text": "The user is asking...",
  "sequence_number": 4
}"#;
    round_trip_test::<crate::ResponseReasoningTextDoneEvent>(
        RESPONSE_REASONING_TEXT_DONE_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseRefusalDeltaEvent example value
#[test]
fn test_response_refusal_delta_event() {
    const RESPONSE_REFUSAL_DELTA_EVENT_EXAMPLE: &str = r#"{
  "type": "response.refusal.delta",
  "item_id": "msg_123",
  "output_index": 0,
  "content_index": 0,
  "delta": "refusal text so far",
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseRefusalDeltaEvent>(RESPONSE_REFUSAL_DELTA_EVENT_EXAMPLE)
        .unwrap();
}
/// Test for ResponseRefusalDoneEvent example value
#[test]
fn test_response_refusal_done_event() {
    const RESPONSE_REFUSAL_DONE_EVENT_EXAMPLE: &str = r#"{
  "type": "response.refusal.done",
  "item_id": "item-abc",
  "output_index": 1,
  "content_index": 2,
  "refusal": "final refusal text",
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseRefusalDoneEvent>(RESPONSE_REFUSAL_DONE_EVENT_EXAMPLE)
        .unwrap();
}
/// Test for ResponseTextDeltaEvent example value
#[test]
#[should_panic]
fn test_response_text_delta_event() {
    const RESPONSE_TEXT_DELTA_EVENT_EXAMPLE: &str = r#"{
  "type": "response.output_text.delta",
  "item_id": "msg_123",
  "output_index": 0,
  "content_index": 0,
  "delta": "In",
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseTextDeltaEvent>(RESPONSE_TEXT_DELTA_EVENT_EXAMPLE).unwrap();
}
/// Test for ResponseTextDoneEvent example value
#[test]
#[should_panic]
fn test_response_text_done_event() {
    const RESPONSE_TEXT_DONE_EVENT_EXAMPLE: &str = r#"{
  "type": "response.output_text.done",
  "item_id": "msg_123",
  "output_index": 0,
  "content_index": 0,
  "text": "In a shimmering forest under a sky full of stars, a lonely unicorn named Lila discovered a hidden pond that glowed with moonlight. Every night, she would leave sparkling, magical flowers by the water's edge, hoping to share her beauty with others. One enchanting evening, she woke to find a group of friendly animals gathered around, eager to be friends and share in her magic.",
  "sequence_number": 1
}"#;
    round_trip_test::<crate::ResponseTextDoneEvent>(RESPONSE_TEXT_DONE_EVENT_EXAMPLE).unwrap();
}
/// Test for ResponseWebSearchCallCompletedEvent example value
#[test]
fn test_response_web_search_call_completed_event() {
    const RESPONSE_WEB_SEARCH_CALL_COMPLETED_EVENT_EXAMPLE: &str = r#"{
  "type": "response.web_search_call.completed",
  "output_index": 0,
  "item_id": "ws_123",
  "sequence_number": 0
}"#;
    round_trip_test::<crate::ResponseWebSearchCallCompletedEvent>(
        RESPONSE_WEB_SEARCH_CALL_COMPLETED_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseWebSearchCallInProgressEvent example value
#[test]
fn test_response_web_search_call_in_progress_event() {
    const RESPONSE_WEB_SEARCH_CALL_IN_PROGRESS_EVENT_EXAMPLE: &str = r#"{
  "type": "response.web_search_call.in_progress",
  "output_index": 0,
  "item_id": "ws_123",
  "sequence_number": 0
}"#;
    round_trip_test::<crate::ResponseWebSearchCallInProgressEvent>(
        RESPONSE_WEB_SEARCH_CALL_IN_PROGRESS_EVENT_EXAMPLE,
    )
    .unwrap();
}
/// Test for ResponseWebSearchCallSearchingEvent example value
#[test]
fn test_response_web_search_call_searching_event() {
    const RESPONSE_WEB_SEARCH_CALL_SEARCHING_EVENT_EXAMPLE: &str = r#"{
  "type": "response.web_search_call.searching",
  "output_index": 0,
  "item_id": "ws_123",
  "sequence_number": 0
}"#;
    round_trip_test::<crate::ResponseWebSearchCallSearchingEvent>(
        RESPONSE_WEB_SEARCH_CALL_SEARCHING_EVENT_EXAMPLE,
    )
    .unwrap();
}
