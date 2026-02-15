pub mod init;
pub mod runner;

// Embedded assets for scaffolding
pub static LOGO_PNG: &[u8] = include_bytes!("../../../apps/showcase/assets/app_logo.png");
pub static FAVICON_ICO: &[u8] = include_bytes!("../../../apps/showcase/assets/favicon.ico");
