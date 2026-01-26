//! Voice module for Speech-to-Text (STT) and Text-to-Speech (TTS)
//! This module is optional and only compiled with the `voice` feature.
#![allow(unused)]

use once_cell::sync::Lazy;
use std::path::PathBuf;
#[cfg(feature = "native")]
use tokio::sync::Mutex as TokioMutex;

#[cfg(feature = "voice")]
use std::sync::Arc;
#[cfg(feature = "voice")]
use tokio::fs;
#[cfg(feature = "voice")]
use tts::Tts;
#[cfg(feature = "voice")]
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

#[allow(dead_code)]
pub struct VoiceManager {
    #[cfg(feature = "voice")]
    whisper_context: Option<Arc<WhisperContext>>,
    #[cfg(feature = "voice")]
    tts: Option<Arc<TokioMutex<Tts>>>,
    #[allow(dead_code)]
    model_dir: PathBuf,
}

impl VoiceManager {
    pub async fn new(model_dir: PathBuf) -> Self {
        #[cfg(feature = "voice")]
        fs::create_dir_all(&model_dir).await.ok();

        Self {
            #[cfg(feature = "voice")]
            whisper_context: None,
            #[cfg(feature = "voice")]
            tts: None,
            model_dir,
        }
    }

    pub async fn init_whisper(&mut self, _model_name: &str) -> anyhow::Result<()> {
        #[cfg(feature = "voice")]
        {
            if self.whisper_context.is_some() {
                return Ok(());
            }

            let model_path = self.model_dir.join(format!("{}.bin", _model_name));

            if !model_path.exists() {
                return Err(anyhow::anyhow!(
                    "Whisper model not found at {:?}. Please download tiny.en.bin to this directory.",
                    model_path
                ));
            }

            let ctx = WhisperContext::new_with_params(
                model_path.to_str().unwrap(),
                WhisperContextParameters::default(),
            )?;

            self.whisper_context = Some(Arc::new(ctx));
            Ok(())
        }
        #[cfg(not(feature = "voice"))]
        {
            Err(anyhow::anyhow!(
                "Voice feature is disabled. Rebuild with --features voice to enable STT."
            ))
        }
    }

    pub async fn init_tts(&mut self) -> anyhow::Result<()> {
        #[cfg(feature = "voice")]
        {
            if self.tts.is_some() {
                return Ok(());
            }

            let tts = Tts::default()
                .map_err(|e| anyhow::anyhow!("Failed to init native TTS: {:?}", e))?;
            self.tts = Some(Arc::new(TokioMutex::new(tts)));
            Ok(())
        }
        #[cfg(not(feature = "voice"))]
        {
            Err(anyhow::anyhow!("Voice feature is disabled."))
        }
    }

    pub async fn transcribe(&self, _audio_data: &[f32]) -> anyhow::Result<String> {
        #[cfg(feature = "voice")]
        {
            let ctx = self
                .whisper_context
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("Whisper not initialized"))?;

            let mut state = ctx.create_state()?;
            let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

            params.set_language(Some("en"));
            params.set_print_special(false);
            params.set_print_progress(false);
            params.set_print_realtime(false);
            params.set_print_timestamps(false);

            state.full(params, _audio_data)?;

            let num_segments = state.full_n_segments()?;
            let mut result = String::new();

            for i in 0..num_segments {
                if let Ok(segment) = state.full_get_segment_text(i) {
                    result.push_str(&segment);
                }
            }

            Ok(result.trim().to_string())
        }
        #[cfg(not(feature = "voice"))]
        {
            Err(anyhow::anyhow!("Voice feature is disabled."))
        }
    }

    pub async fn synthesize(&mut self, _text: &str, _voice: &str) -> anyhow::Result<Vec<f32>> {
        #[cfg(feature = "voice")]
        {
            self.init_tts().await?;

            let tts_mutex = self
                .tts
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("TTS not initialized"))?;

            let mut tts = tts_mutex.lock().await;

            // On macOS, TTS speaks directly.
            // We can try to set the voice if the name matches one from tts.voices()
            // but for a "get it working" first pass, default voice is fine.
            tts.speak(_text, true)
                .map_err(|e| anyhow::anyhow!("TTS speak failed: {:?}", e))?;

            // Return empty samples because native TTS plays directly and doesn't easily return a buffer.
            // The shell/client will receive this empty vec and just do nothing, while the user hears the sound.
            Ok(Vec::new())
        }
        #[cfg(not(feature = "voice"))]
        {
            Err(anyhow::anyhow!("Voice feature is disabled."))
        }
    }
}

pub static VOICE: Lazy<TokioMutex<VoiceManager>> = Lazy::new(|| {
    #[cfg(not(target_arch = "wasm32"))]
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".into());
    #[cfg(target_arch = "wasm32")]
    let home = "/tmp";

    let path = std::path::PathBuf::from(home).join(".peak/intelligence/voice");
    TokioMutex::new(futures::executor::block_on(VoiceManager::new(path)))
});
