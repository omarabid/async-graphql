#[cfg(feature = "no-send")]
pub trait ThreadedModel {}

#[cfg(not(feature = "no-send"))]
pub trait ThreadedModel: Send + Sync {}
