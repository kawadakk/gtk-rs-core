// This file was generated by gir (1644ef1) from gir-files (71d73f0)
// DO NOT EDIT

use Bin;
use Container;
use Widget;
use ffi;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct Overlay(Object<ffi::GtkOverlay>): Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_overlay_get_type(),
    }
}

impl Overlay {
    pub fn new() -> Overlay {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_overlay_new()).downcast_unchecked()
        }
    }

    pub fn add_overlay<T: IsA<Widget>>(&self, widget: &T) {
        unsafe {
            ffi::gtk_overlay_add_overlay(self.to_glib_none().0, widget.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_18")]
    pub fn get_overlay_pass_through<T: IsA<Widget>>(&self, widget: &T) -> bool {
        unsafe {
            from_glib(ffi::gtk_overlay_get_overlay_pass_through(self.to_glib_none().0, widget.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_18")]
    pub fn reorder_overlay<T: IsA<Widget>>(&self, child: &T, position: i32) {
        unsafe {
            ffi::gtk_overlay_reorder_overlay(self.to_glib_none().0, child.to_glib_none().0, position);
        }
    }

    #[cfg(feature = "v3_18")]
    pub fn set_overlay_pass_through<T: IsA<Widget>>(&self, widget: &T, pass_through: bool) {
        unsafe {
            ffi::gtk_overlay_set_overlay_pass_through(self.to_glib_none().0, widget.to_glib_none().0, pass_through.to_glib());
        }
    }

    pub fn get_child_index<T: IsA<Widget>>(&self, item: &T) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "index".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_child_index<T: IsA<Widget>>(&self, item: &T, index: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "index".to_glib_none().0, Value::from(&index).to_glib_none().0);
        }
    }

    //pub fn connect_get_child_position<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Out allocation: Gdk.Rectangle
    //}
}
