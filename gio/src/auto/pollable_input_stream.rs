// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::InputStream;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GPollableInputStream")]
    pub struct PollableInputStream(Interface<ffi::GPollableInputStream, ffi::GPollableInputStreamInterface>) @requires InputStream;

    match fn {
        type_ => || ffi::g_pollable_input_stream_get_type(),
    }
}

impl PollableInputStream {
    pub const NONE: Option<&'static PollableInputStream> = None;
}

pub trait PollableInputStreamExt: 'static {
    #[doc(alias = "g_pollable_input_stream_can_poll")]
    fn can_poll(&self) -> bool;

    #[doc(alias = "g_pollable_input_stream_is_readable")]
    fn is_readable(&self) -> bool;
}

impl<O: IsA<PollableInputStream>> PollableInputStreamExt for O {
    fn can_poll(&self) -> bool {
        unsafe {
            from_glib(ffi::g_pollable_input_stream_can_poll(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_readable(&self) -> bool {
        unsafe {
            from_glib(ffi::g_pollable_input_stream_is_readable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for PollableInputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PollableInputStream")
    }
}
