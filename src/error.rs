//! Main Crate Error

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Generic {0}")]
	Generic(String),

	#[error(transparent)]
	IO(#[from] std::io::Error),
}
