//! I/O abstraction layer.
//!
//! Re-exports from `no_std_io::io`, which forwards to `std::io` when the `std`
//! feature is enabled (via `no_std_io/std`).

pub use no_std_io::io::*;

/// Re-export of the I/O prelude traits.
pub mod prelude {
    pub use no_std_io::io::{BufRead, Read, Seek, Write};
}
