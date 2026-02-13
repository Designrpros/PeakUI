#[cfg(feature = "voice")]
use crate::tools::IntelligenceTool;
#[cfg(feature = "voice")]
use anyhow::Result;
#[cfg(feature = "voice")]
use async_trait::async_trait;
#[cfg(feature = "voice")]
use serde_json::{json, Value};

#[cfg(feature = "voice")]
use crate::voice::VOICE;

#[cfg(feature = "voice")]
pub struct SttTool;

#[cfg(feature = "voice")]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl IntelligenceTool for SttTool {
    fn name(&self) -> &'static str {
        "intelligence/stt"
    }
    fn description(&self) -> &'static str {
        "Convert PCM audio data (f32, 16kHz) to text using Whisper."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "audio": { "type": "array", "items": { "type": "number" } }
            },
            "required": ["audio"]
        })
    }
    async fn execute(&self, args: Value) -> Result<Value> {
        let audio_val = args
            .get("audio")
            .ok_or_else(|| anyhow::anyhow!("Missing 'audio' argument"))?;
        let audio_arr = audio_val
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("'audio' must be an array"))?;
        let samples: Vec<f32> = audio_arr
            .iter()
            .filter_map(|v| v.as_f64().map(|f| f as f32))
            .collect();

        let mut manager = VOICE.lock().await;
        manager.init_whisper("tiny.en").await?;
        manager.transcribe(&samples).await.map(|s| json!(s))
    }
}

#[cfg(feature = "voice")]
pub struct TtsTool;

#[cfg(feature = "voice")]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl IntelligenceTool for TtsTool {
    fn name(&self) -> &'static str {
        "intelligence/tts"
    }
    fn description(&self) -> &'static str {
        "Convert text to speech samples (f32, 22kHz) using Piper."
    }
    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "text": { "type": "string" },
                "voice": { "type": "string", "default": "en_US-lessac-medium" }
            },
            "required": ["text"]
        })
    }
    async fn execute(&self, args: Value) -> Result<Value> {
        let text = args.get("text").and_then(|v| v.as_str()).unwrap_or("");
        let voice = args
            .get("voice")
            .and_then(|v| v.as_str())
            .unwrap_or("en_US-lessac-medium");

        let mut manager = VOICE.lock().await;
        manager
            .synthesize(text, voice)
            .await
            .map(|samples| json!({ "samples": samples }))
    }
}
