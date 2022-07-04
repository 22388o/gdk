use std::{
    fs::File,
    sync::{MutexGuard, PoisonError},
};

/// Result type alias of the `gdk_registry` crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Error enum of the `gdk_registry` crate.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Thrown when calling `crate::init` more than once.
    #[error("Cannot call `init` more than once")]
    AlreadyInitialized,

    /// Thrown when neither assets nor icons are requested in
    /// [`crate::RefreshAssetsParams`]
    #[error("Neither assets nor icons were requested")]
    BothAssetsIconsFalse,

    /// Returned when calling `ExtendedPubKey::from_str` with an invalid
    /// string.
    #[error(transparent)]
    BtcBip32Error(#[from] bitcoin::util::bip32::Error),

    /// Wraps hex parsing error
    #[error(transparent)]
    Hex(#[from] elements::bitcoin::hashes::hex::Error),

    /// An invalid network as been specified
    #[error("InvalidNetwork({0})")]
    InvalidNetwork(String),

    /// Wraps IO errors.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Wrap a poison error as string to avoid pollute with lifetimes.
    #[error("{0}")]
    Poison(String),

    /// Returned when a registry cache file has yet to be created.
    #[error("Registry cache for this wallet has not been created")]
    RegistryCacheNotCreated,

    /// Thrown when a method requires the registry to be initialized (via the
    /// [`crate::init`] call) but it wasn't initialized.
    #[error("Registry has not been initialized")]
    RegistryUninitialized,

    /// Wraps errors happened when serializing or deserializing JSONs.
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    /// Wraps errors happened when serializing or deserializing CBORs.
    #[error(transparent)]
    SerdeCbor(#[from] serde_cbor::Error),

    /// Wraps http errors.
    #[error(transparent)]
    Ureq(#[from] ureq::Error),

    /// A generic error.
    #[error("{0:?}")]
    Generic(Box<dyn std::fmt::Debug>),
}

// This error type doesn't implement `std::error::Error`.
impl From<aes_gcm_siv::aead::Error> for Error {
    fn from(err: aes_gcm_siv::aead::Error) -> Self {
        Self::Generic(Box::new(err))
    }
}

impl From<PoisonError<MutexGuard<'_, File>>> for Error {
    fn from(e: PoisonError<MutexGuard<'_, File>>) -> Self {
        Error::Poison(e.to_string())
    }
}