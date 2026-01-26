use super::app::Message;
use crate::prelude::{IcedBackend, PageResult as FrameworkPageResult};

/// Reference-app specific type alias for the framework's contextual UI result.
pub type PageResult = FrameworkPageResult<Message, IcedBackend>;
