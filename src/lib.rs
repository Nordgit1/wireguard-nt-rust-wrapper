//! Safe rust idiomatic bindings for the Wireguard NT C library: <https://git.zx2c4.com/wireguard-nt/about>
//!
//! All features of the Wireguard NT library are wrapped using pure rust types and functions to make
//! usage feel ergonomic.  
//!
//! # Usage
//!
//! Add a dependency on this library to your `Cargo.toml`
//!
//! ```toml
//! [dependencies]
//! wireguard-nt = "0.1"
//! ```
//!
//! Inside your code load the wireguard.dll signed driver file, downloaded from <https://git.zx2c4.com/wireguard-nt/about>
//!
//! Then either call [`Adapter::create`] or [`Adapter::open`] to obtain a wireguard
//! adapter. Start set its config with [`Adapter::set_config`].
//!
//! # Example
//! ```no_run
//! use std::sync::Arc;
//!
//! //Must be run as Administrator because we create network adapters
//! //Load the wireguard dll file so that we can call the underlying C functions
//! //Unsafe because we are loading an arbitrary dll file
//! let wireguard = unsafe { wireguard_nt::load_from_path("path/to/wireguard.dll") }.expect("Failed to load wireguard dll");
//! //Try to open an adapter from the given pool with the name "Demo"
//! let adapter = match wireguard_nt::Adapter::open(&wireguard, "WireGuard", "Demo") {
//!     Ok(a) => a,
//!     Err(_) =>
//!         //If loading failed (most likely it didn't exist), create a new one
//!         wireguard_nt::Adapter::create(&wireguard, "WireGuard", "Demo", None).expect("Failed to create wireguard adapter!").adapter,
//! };
//!
//! todo!("Set config");
//! //Delete the adapter when finished.
//! adapter.delete().unwrap();
//! //drop(adapter)
//! //And the adapter closes its resources when dropped
//!    
//! ```
//!    
//! See `examples/demo_server.rs` that connects to the wireguard demo server
//!

mod adapter;
mod log;
mod util;

//Generated by bingen, so ignore lints
#[allow(
    non_snake_case,
    dead_code,
    unused_variables,
    non_camel_case_types,
    deref_nullptr,
    clippy::all
)]
mod wireguard_nt_raw;

pub(crate) const MAX_POOL: usize = 256;

pub use crate::adapter::*;
pub use crate::log::*;
pub use crate::util::get_running_driver_version;

pub use wireguard_nt_raw::wireguard as dll;

use std::sync::Arc;

/// Attempts to load the Wireguand NT library from the current directory using the default name "wireguard.dll".
///
/// Use [`load_from_path`] with an absolute path when more control is needed as to where wireguard.dll is
///
///
/// # Safety
/// This function loads a dll file with the name wireguard.dll using the default system search paths.
/// This is inherently unsafe as a user could simply rename undefined_behavior.dll to wireguard.dll
/// and do nefarious things inside of its DllMain function. In most cases, a regular wireguard.dll
/// file which exports all of the required functions for these bindings to work is loaded. Because
/// Wireguard NT is a well-written and well-tested library, loading a _normal_ wireguard.dll file should be safe.
/// Hoverer one can never be too cautious when loading a dll file.
///
/// For more information see [`libloading`]'s dynamic library safety guarantees: [`libloading`][`libloading::Library::new`]
pub unsafe fn load() -> Result<Arc<dll>, libloading::Error> {
    load_from_path("wireguard")
}

/// Attempts to load the wireguard library as a dynamic library from the given path.
///
///
/// # Safety
/// This function loads a dll file with the path provided.
/// This is inherently unsafe as a user could simply rename undefined_behavior.dll to wireguard.dll
/// and do nefarious things inside of its DllMain function. In most cases, a regular wireguard.dll
/// file which exports all of the required functions for these bindings to work is loaded. Because
/// Wireguard NT is a well-written and well-tested library, loading a _normal_ wireguard.dll file should be safe.
/// Hoverer one can never be too cautious when loading a dll file.
///
/// For more information see [`libloading`]'s dynamic library safety guarantees: [`libloading`][`libloading::Library::new`]
pub unsafe fn load_from_path<P>(path: P) -> Result<Arc<dll>, libloading::Error>
where
    P: AsRef<::std::ffi::OsStr>,
{
    Ok(Arc::new(wireguard_nt_raw::wireguard::new(path)?))
}

/// Attempts to load the Wireguard NT library from an existing [`libloading::Library`].
///
///
/// # Safety
/// This function loads the required Wireguard NT functions using the provided library. Reading a symbol table
/// of a dynamic library and transmuting the function pointers inside to have the parameters and return
/// values expected by the functions documented at: <https://git.zx2c4.com/wireguard-nt/about/>
/// is inherently unsafe.
///
/// For more information see [`libloading`]'s dynamic library safety guarantees: [`libloading::Library::new`]
pub unsafe fn load_from_library<L>(library: L) -> Result<Arc<dll>, libloading::Error>
where
    L: Into<libloading::Library>,
{
    Ok(Arc::new(wireguard_nt_raw::wireguard::from_library(
        library,
    )?))
}

pub type WireGuardError = Box<dyn std::error::Error>;
