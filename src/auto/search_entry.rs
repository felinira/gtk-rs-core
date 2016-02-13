// This file was generated by gir (17af302) from gir-files (11e0e6d)
// DO NOT EDIT

use Buildable;
use CellEditable;
use Editable;
use Entry;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct SearchEntry(Object<ffi::GtkSearchEntry>): Widget, Entry, Buildable, CellEditable, Editable;

    match fn {
        get_type => || ffi::gtk_search_entry_get_type(),
    }
}

impl SearchEntry {
    #[cfg(feature = "3.6")]
    pub fn new() -> SearchEntry {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_search_entry_new()).downcast_unchecked()
        }
    }

    //#[cfg(feature = "3.16")]
    //pub fn handle_event(&self, event: /*Unknown conversion*//*Unimplemented*/Event) -> bool {
    //    unsafe { TODO: call ffi::gtk_search_entry_handle_event() }
    //}
}
