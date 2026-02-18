use fluent::{FluentArgs, FluentBundle, FluentResource};
use std::cell::RefCell;
use unic_langid::LanguageIdentifier;

/// A handle to the localization engine.
/// This is meant to be stored in the App's Context.
/// It is Send + Sync because it only contains thread-safe metadata.
/// The actual FluentBundle is cached per-thread for performance and thread-safety.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Localization {
    pub language: LanguageIdentifier,
    pub resources: Vec<String>,
}

thread_local! {
    static BUNDLE_CACHE: RefCell<Option<(LanguageIdentifier, Vec<String>, FluentBundle<FluentResource>)>> = RefCell::new(None);
}

impl std::fmt::Debug for Localization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Localization")
            .field("language", &self.language)
            .field("resources_count", &self.resources.len())
            .finish()
    }
}

impl Localization {
    /// Creates a new localization context for the given language.
    pub fn new(lang: &str, resources: Vec<String>) -> Self {
        let lang_id: LanguageIdentifier = lang.parse().expect("Language parsing failed");
        Self {
            language: lang_id,
            resources,
        }
    }

    /// Accesses the bundle for the current thread, rebuilding it if necessary.
    fn with_bundle<R>(&self, f: impl FnOnce(&FluentBundle<FluentResource>) -> R) -> R {
        BUNDLE_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();

            let needs_rebuild = match &*cache {
                Some((lang, res, _)) => lang != &self.language || res != &self.resources,
                None => true,
            };

            if needs_rebuild {
                let mut bundle = FluentBundle::new(vec![self.language.clone()]);
                for r_str in &self.resources {
                    let res = FluentResource::try_new(r_str.clone())
                        .expect("Failed to parse FTL resource");
                    bundle
                        .add_resource(res)
                        .expect("Failed to add FTL resource to bundle");
                }
                bundle.set_use_isolating(false);
                *cache = Some((self.language.clone(), self.resources.clone(), bundle));
            }

            let (_, _, bundle) = cache.as_ref().unwrap();
            f(bundle)
        })
    }

    pub fn t(&self, key: &str, args: Option<&FluentArgs>) -> String {
        self.with_bundle(|bundle| {
            let msg = bundle.get_message(key);

            if let Some(msg) = msg {
                if let Some(pattern) = msg.value() {
                    let mut errors = vec![];
                    let value = bundle.format_pattern(pattern, args, &mut errors);
                    return value.to_string();
                }
            }

            // Fallback: return key if missing
            format!("MISSING:{}", key)
        })
    }

    pub fn format(&self, key: &str, args: FluentArgs) -> String {
        self.t(key, Some(&args))
    }

    pub fn simple(&self, key: &str) -> String {
        self.t(key, None)
    }

    pub fn set_language(&mut self, lang: &str, resources: Vec<String>) {
        let lang_id: LanguageIdentifier = lang.parse().expect("Language parsing failed");
        self.language = lang_id;
        self.resources = resources;
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
