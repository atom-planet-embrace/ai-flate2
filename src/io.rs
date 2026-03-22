//! I/O abstraction layer.
//!
//! When the `std` feature is enabled, this re-exports from `std::io`.
//! When `std` is not enabled, this re-exports from `no_std_io::io`.

#[cfg(any(feature = "std", test))]
pub use std::io::*;

#[cfg(not(any(feature = "std", test)))]
pub use no_std_io::io::*;

/// Re-export of the I/O prelude traits.
pub mod prelude {
    #[cfg(any(feature = "std", test))]
    pub use std::io::prelude::*;

    #[cfg(not(any(feature = "std", test)))]
    pub use no_std_io::io::{BufRead, Read, Seek, Write};
}
