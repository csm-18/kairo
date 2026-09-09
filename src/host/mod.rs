// host module abstracts host environment specific utilities (like file-system, io, etc)

mod native;
pub use native::*;
