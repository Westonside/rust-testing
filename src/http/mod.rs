//specify the public interface for the module
pub use method::Method;
pub use query_string::{QueryString, Value as QueryStringValue};
pub use request::{Request, ParseError};


pub mod request;
pub mod method;
pub mod query_string;