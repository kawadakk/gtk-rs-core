// This file was generated by gir (1644ef1) from gir-files (71d73f0)
// DO NOT EDIT

use Box;
use Button;
use Container;
use MessageType;
use Orientable;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct InfoBar(Object<ffi::GtkInfoBar>): Box, Container, Widget, Orientable;

    match fn {
        get_type => || ffi::gtk_info_bar_get_type(),
    }
}

impl InfoBar {
    pub fn new() -> InfoBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_info_bar_new()).downcast_unchecked()
        }
    }

    //pub fn new_with_buttons(first_button_text: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> InfoBar {
    //    unsafe { TODO: call ffi::gtk_info_bar_new_with_buttons() }
    //}

    pub fn add_action_widget<T: IsA<Widget>>(&self, child: &T, response_id: i32) {
        unsafe {
            ffi::gtk_info_bar_add_action_widget(self.to_glib_none().0, child.to_glib_none().0, response_id);
        }
    }

    pub fn add_button(&self, button_text: &str, response_id: i32) -> Option<Button> {
        unsafe {
            from_glib_none(ffi::gtk_info_bar_add_button(self.to_glib_none().0, button_text.to_glib_none().0, response_id))
        }
    }

    //pub fn add_buttons(&self, first_button_text: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_info_bar_add_buttons() }
    //}

    pub fn get_action_area(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_info_bar_get_action_area(self.to_glib_none().0))
        }
    }

    pub fn get_content_area(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_info_bar_get_content_area(self.to_glib_none().0))
        }
    }

    pub fn get_message_type(&self) -> MessageType {
        unsafe {
            from_glib(ffi::gtk_info_bar_get_message_type(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn get_show_close_button(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_info_bar_get_show_close_button(self.to_glib_none().0))
        }
    }

    pub fn response(&self, response_id: i32) {
        unsafe {
            ffi::gtk_info_bar_response(self.to_glib_none().0, response_id);
        }
    }

    pub fn set_default_response(&self, response_id: i32) {
        unsafe {
            ffi::gtk_info_bar_set_default_response(self.to_glib_none().0, response_id);
        }
    }

    pub fn set_message_type(&self, message_type: MessageType) {
        unsafe {
            ffi::gtk_info_bar_set_message_type(self.to_glib_none().0, message_type.to_glib());
        }
    }

    pub fn set_response_sensitive(&self, response_id: i32, setting: bool) {
        unsafe {
            ffi::gtk_info_bar_set_response_sensitive(self.to_glib_none().0, response_id, setting.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn set_show_close_button(&self, setting: bool) {
        unsafe {
            ffi::gtk_info_bar_set_show_close_button(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn connect_close<F: Fn(&InfoBar) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&InfoBar) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "close",
                transmute(close_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_response<F: Fn(&InfoBar, i32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&InfoBar, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "response",
                transmute(response_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn close_trampoline(this: *mut ffi::GtkInfoBar, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&InfoBar) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn response_trampoline(this: *mut ffi::GtkInfoBar, response_id: libc::c_int, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&InfoBar, i32) + 'static> = transmute(f);
    f(&from_glib_none(this), response_id)
}
