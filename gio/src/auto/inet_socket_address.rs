// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::InetAddress;
use crate::SocketAddress;
use crate::SocketConnectable;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GInetSocketAddress")]
    pub struct InetSocketAddress(Object<ffi::GInetSocketAddress, ffi::GInetSocketAddressClass>) @extends SocketAddress, @implements SocketConnectable;

    match fn {
        type_ => || ffi::g_inet_socket_address_get_type(),
    }
}

impl InetSocketAddress {
    #[doc(alias = "g_inet_socket_address_new")]
    pub fn new(address: &impl IsA<InetAddress>, port: u16) -> InetSocketAddress {
        unsafe {
            SocketAddress::from_glib_full(ffi::g_inet_socket_address_new(
                address.as_ref().to_glib_none().0,
                port,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "g_inet_socket_address_new_from_string")]
    #[doc(alias = "new_from_string")]
    pub fn from_string(address: &str, port: u32) -> Option<InetSocketAddress> {
        unsafe {
            Option::<SocketAddress>::from_glib_full(ffi::g_inet_socket_address_new_from_string(
                address.to_glib_none().0,
                port,
            ))
            .map(|o| o.unsafe_cast())
        }
    }
}

unsafe impl Send for InetSocketAddress {}
unsafe impl Sync for InetSocketAddress {}

impl InetSocketAddress {
    pub const NONE: Option<&'static InetSocketAddress> = None;
}

pub trait InetSocketAddressExt: 'static {
    #[doc(alias = "g_inet_socket_address_get_address")]
    #[doc(alias = "get_address")]
    fn address(&self) -> InetAddress;

    #[doc(alias = "g_inet_socket_address_get_flowinfo")]
    #[doc(alias = "get_flowinfo")]
    fn flowinfo(&self) -> u32;

    #[doc(alias = "g_inet_socket_address_get_port")]
    #[doc(alias = "get_port")]
    fn port(&self) -> u16;

    #[doc(alias = "g_inet_socket_address_get_scope_id")]
    #[doc(alias = "get_scope_id")]
    fn scope_id(&self) -> u32;
}

impl<O: IsA<InetSocketAddress>> InetSocketAddressExt for O {
    fn address(&self) -> InetAddress {
        unsafe {
            from_glib_none(ffi::g_inet_socket_address_get_address(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn flowinfo(&self) -> u32 {
        unsafe { ffi::g_inet_socket_address_get_flowinfo(self.as_ref().to_glib_none().0) }
    }

    fn port(&self) -> u16 {
        unsafe { ffi::g_inet_socket_address_get_port(self.as_ref().to_glib_none().0) }
    }

    fn scope_id(&self) -> u32 {
        unsafe { ffi::g_inet_socket_address_get_scope_id(self.as_ref().to_glib_none().0) }
    }
}

impl fmt::Display for InetSocketAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("InetSocketAddress")
    }
}
