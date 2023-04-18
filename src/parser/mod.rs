mod http_protocol;
use serde::{Deserialize, Serialize};

pub use self::http_protocol::{HttpParser, HttpRequest, HttpHeader, HttpBody};

mod parse_string;
pub use self::parse_string::{split_once_line, parse_meta_data};

mod path;
pub use self::path::Path;

