#[cfg(not(feature = "parallel"))]
pub use std::rc::Rc as Lrc;

#[cfg(feature = "parallel")]
pub use std::sync::Arc as Lrc;

#[cfg(not(feature = "parallel"))]
pub use std::cell::RefCell as Lock;

#[cfg(feature = "parallel")]
pub use std::sync::RwLock as Lock;
