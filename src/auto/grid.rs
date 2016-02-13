// This file was generated by gir (17af302) from gir-files (11e0e6d)
// DO NOT EDIT

#[cfg(feature = "3.10")]
use BaselinePosition;
use Buildable;
use Container;
use Orientable;
use PositionType;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct Grid(Object<ffi::GtkGrid>): Widget, Container, Buildable, Orientable;

    match fn {
        get_type => || ffi::gtk_grid_get_type(),
    }
}

impl Grid {
    pub fn new() -> Grid {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_grid_new()).downcast_unchecked()
        }
    }

    pub fn attach<T: IsA<Widget>>(&self, child: &T, left: i32, top: i32, width: i32, height: i32) {
        unsafe {
            ffi::gtk_grid_attach(self.to_glib_none().0, child.to_glib_none().0, left, top, width, height);
        }
    }

    pub fn attach_next_to<T: IsA<Widget>, U: IsA<Widget>>(&self, child: &T, sibling: Option<&U>, side: PositionType, width: i32, height: i32) {
        unsafe {
            ffi::gtk_grid_attach_next_to(self.to_glib_none().0, child.to_glib_none().0, sibling.to_glib_none().0, side, width, height);
        }
    }

    #[cfg(feature = "3.10")]
    pub fn get_baseline_row(&self) -> i32 {
        unsafe {
            ffi::gtk_grid_get_baseline_row(self.to_glib_none().0)
        }
    }

    pub fn get_child_at(&self, left: i32, top: i32) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_grid_get_child_at(self.to_glib_none().0, left, top))
        }
    }

    pub fn get_column_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_grid_get_column_homogeneous(self.to_glib_none().0))
        }
    }

    pub fn get_column_spacing(&self) -> u32 {
        unsafe {
            ffi::gtk_grid_get_column_spacing(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "3.10")]
    pub fn get_row_baseline_position(&self, row: i32) -> BaselinePosition {
        unsafe {
            ffi::gtk_grid_get_row_baseline_position(self.to_glib_none().0, row)
        }
    }

    pub fn get_row_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_grid_get_row_homogeneous(self.to_glib_none().0))
        }
    }

    pub fn get_row_spacing(&self) -> u32 {
        unsafe {
            ffi::gtk_grid_get_row_spacing(self.to_glib_none().0)
        }
    }

    pub fn insert_column(&self, position: i32) {
        unsafe {
            ffi::gtk_grid_insert_column(self.to_glib_none().0, position);
        }
    }

    pub fn insert_next_to<T: IsA<Widget>>(&self, sibling: &T, side: PositionType) {
        unsafe {
            ffi::gtk_grid_insert_next_to(self.to_glib_none().0, sibling.to_glib_none().0, side);
        }
    }

    pub fn insert_row(&self, position: i32) {
        unsafe {
            ffi::gtk_grid_insert_row(self.to_glib_none().0, position);
        }
    }

    #[cfg(feature = "3.10")]
    pub fn remove_column(&self, position: i32) {
        unsafe {
            ffi::gtk_grid_remove_column(self.to_glib_none().0, position);
        }
    }

    #[cfg(feature = "3.10")]
    pub fn remove_row(&self, position: i32) {
        unsafe {
            ffi::gtk_grid_remove_row(self.to_glib_none().0, position);
        }
    }

    #[cfg(feature = "3.10")]
    pub fn set_baseline_row(&self, row: i32) {
        unsafe {
            ffi::gtk_grid_set_baseline_row(self.to_glib_none().0, row);
        }
    }

    pub fn set_column_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_grid_set_column_homogeneous(self.to_glib_none().0, homogeneous.to_glib());
        }
    }

    pub fn set_column_spacing(&self, spacing: u32) {
        unsafe {
            ffi::gtk_grid_set_column_spacing(self.to_glib_none().0, spacing);
        }
    }

    #[cfg(feature = "3.10")]
    pub fn set_row_baseline_position(&self, row: i32, pos: BaselinePosition) {
        unsafe {
            ffi::gtk_grid_set_row_baseline_position(self.to_glib_none().0, row, pos);
        }
    }

    pub fn set_row_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_grid_set_row_homogeneous(self.to_glib_none().0, homogeneous.to_glib());
        }
    }

    pub fn set_row_spacing(&self, spacing: u32) {
        unsafe {
            ffi::gtk_grid_set_row_spacing(self.to_glib_none().0, spacing);
        }
    }
}
