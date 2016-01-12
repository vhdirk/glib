// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ffi::CStr;
use std::error;
use std::fmt;
use std::str;
use translate::*;
use glib_ffi;

glib_wrapper! {
    pub struct Error(Boxed<glib_ffi::GError>);

    match fn {
        copy => |ptr| glib_ffi::g_error_copy(ptr),
        free => |ptr| glib_ffi::g_error_free(ptr),
    }
}

impl Error {
    /// Creates an error with supplied error enum variant and message.
    pub fn new<T: ErrorDomain>(error: T, message: &str) -> Error {
        unsafe {
            from_glib_full(
                glib_ffi::g_error_new_literal(T::domain(), error.code(), message.to_glib_none().0))
        }
    }

    /// Tries to match against an error enum.
    ///
    /// Example
    /// ```ignore
    /// if let Some(file_error) = error.matches::<FileError>() {
    ///     match file_error {
    ///         Exist => ...
    ///         Isdir => ...
    ///         ...
    ///     }
    /// }
    /// ```
    pub fn matches<T: ErrorDomain>(&self) -> Option<T> {
        if self.0.domain == T::domain() {
            T::from(self.0.code)
        }
        else {
            None
        }
    }

    fn message(&self) -> &str {
        unsafe {
            let bytes = CStr::from_ptr(self.0.message).to_bytes();
            str::from_utf8(bytes).unwrap_or_else(|err| {
                str::from_utf8(&bytes[..err.valid_up_to()]).unwrap()
            })
        }
    }

    // backcompat shim
    pub fn wrap(ptr: *mut glib_ffi::GError) -> Error {
        unsafe { from_glib_full(ptr) }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.message()
    }
}

/// `GLib` error domain.
///
/// This trait is implemented by error enums and is not intended for end users.
pub trait ErrorDomain: Copy {
    /// Returns the quark identifying the error domain.
    ///
    /// As returned from `g_some_error_quark`.
    fn domain() -> glib_ffi::GQuark;

    /// Gets the integer representation of the variant.
    fn code(self) -> i32;

    /// Tries to convert an integer code to an enum variant.
    ///
    /// By convention, the `Failed` variant, if present, is a catch-all,
    /// i.e. any unrecognized codes map to it.
    fn from(code: i32) -> Option<Self> where Self: Sized;
}
