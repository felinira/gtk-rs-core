// This file was generated by gir (0d8699e) from gir-files (11e0e6d)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct FlowBoxChild(Object<ffi::GtkFlowBoxChild>): Bin, Container, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_flow_box_child_get_type(),
    }
}

impl FlowBoxChild {
    #[cfg(feature = "v3_12")]
    pub fn new() -> FlowBoxChild {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_flow_box_child_new()).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn changed(&self) {
        unsafe {
            ffi::gtk_flow_box_child_changed(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn get_index(&self) -> i32 {
        unsafe {
            ffi::gtk_flow_box_child_get_index(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn is_selected(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_flow_box_child_is_selected(self.to_glib_none().0))
        }
    }
}
