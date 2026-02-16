use fluent::{FluentArgs, FluentBundle, FluentResource};
use std::sync::Arc;
use unic_langid::LanguageIdentifier;

/// A handle to the localization engine.
/// This is meant to be stored in the App's Context.
#[derive(Clone)]
pub struct Localization {
    pub language: LanguageIdentifier,
    bundle: Arc<FluentBundle<FluentResource>>,
}

impl std::fmt::Debug for Localization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Localization")
            .field("language", &self.language)
            .finish()
    }
}

impl Localization {
    /// Creates a new localization context for the given language.
    pub fn new(lang: &str, resources: Vec<String>) -> Self {
        let lang_id: LanguageIdentifier = lang.parse().expect("Parsing failed");
        let mut bundle = FluentBundle::new(vec![lang_id.clone()]);

        for r in resources {
            let res = FluentResource::try_new(r).expect("Failed to parse FTL");
            bundle
                .add_resource(res)
                .expect("Failed to add FTL resource");
        }

        // Turn off isolation for simpler text rendering in UI
        bundle.set_use_isolating(false);

        Self {
            language: lang_id,
            bundle: Arc::new(bundle),
        }
    }

    pub fn t(&self, key: &str, args: Option<&FluentArgs>) -> String {
        let msg = self.bundle.get_message(key);

        if let Some(msg) = msg {
            if let Some(pattern) = msg.value() {
                let mut errors = vec![];
                let value = self.bundle.format_pattern(pattern, args, &mut errors);
                return value.to_string();
            }
        }

        // Fallback: return key if missing
        format!("MISSING:{}", key)
    }

    pub fn format(&self, key: &str, args: FluentArgs) -> String {
        self.t(key, Some(&args))
    }

    pub fn simple(&self, key: &str) -> String {
        self.t(key, None)
    }

    pub fn set_language(&mut self, lang: &str, resources: Vec<String>) {
        *self = Self::new(lang, resources);
    }
}

impl Default for Localization {
    fn default() -> Self {
        Self::new(
            "en-US",
            vec![
                "hello-world = Hello World from Fluent!\nwelcome = Welcome, { $name }!".to_string(),
            ],
        )
    }
}
