// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AsyncResult;
use crate::Cancellable;
use crate::SocketAddress;
use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GSocketAddressEnumerator")]
    pub struct SocketAddressEnumerator(Object<ffi::GSocketAddressEnumerator, ffi::GSocketAddressEnumeratorClass>);

    match fn {
        type_ => || ffi::g_socket_address_enumerator_get_type(),
    }
}

impl SocketAddressEnumerator {
    pub const NONE: Option<&'static SocketAddressEnumerator> = None;
}

pub trait SocketAddressEnumeratorExt: 'static {
    #[doc(alias = "g_socket_address_enumerator_next")]
    fn next(
        &self,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<SocketAddress, glib::Error>;

    #[doc(alias = "g_socket_address_enumerator_next_async")]
    fn next_async<P: FnOnce(Result<SocketAddress, glib::Error>) + Send + 'static>(
        &self,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    );

    fn next_async_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<SocketAddress, glib::Error>> + 'static>>;
}

impl<O: IsA<SocketAddressEnumerator>> SocketAddressEnumeratorExt for O {
    fn next(
        &self,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<SocketAddress, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_address_enumerator_next(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn next_async<P: FnOnce(Result<SocketAddress, glib::Error>) + Send + 'static>(
        &self,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let user_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn next_async_trampoline<
            P: FnOnce(Result<SocketAddress, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_address_enumerator_next_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<P> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = next_async_trampoline::<P>;
        unsafe {
            ffi::g_socket_address_enumerator_next_async(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn next_async_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<SocketAddress, glib::Error>> + 'static>>
    {
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.next_async(Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }
}

impl fmt::Display for SocketAddressEnumerator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SocketAddressEnumerator")
    }
}
