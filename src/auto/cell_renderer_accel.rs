// This file was generated by gir (17af302) from gir-files (11e0e6d)
// DO NOT EDIT

use CellRenderer;
use CellRendererText;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct CellRendererAccel(Object<ffi::GtkCellRendererAccel>): CellRenderer, CellRendererText;

    match fn {
        get_type => || ffi::gtk_cell_renderer_accel_get_type(),
    }
}

impl CellRendererAccel {
    pub fn new() -> CellRendererAccel {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_accel_new()).downcast_unchecked()
        }
    }
}
