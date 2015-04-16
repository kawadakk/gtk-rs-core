// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! GtkTextTag — A tag that can be applied to text in a GtkTextBuffer

use ffi;
use glib::translate::ToGlibPtr;

pub struct TextTag {
    pointer: *mut ffi::C_GtkTextTag
}

impl TextTag {
    pub fn new(name: &str) -> Option<TextTag> {
        let tmp_pointer = unsafe {
            ffi::gtk_text_tag_new(name.borrow_to_glib().0)
        };

        if tmp_pointer.is_null() {
            None
        } else {
            Some(TextTag { pointer : tmp_pointer })
        }
    }

    pub fn get_priority(&self) -> i32 {
        unsafe { ffi::gtk_text_tag_get_priority(self.pointer) }
    }

    pub fn set_priority(&self, priority: i32) {
        unsafe { ffi::gtk_text_tag_set_priority(self.pointer, priority as ::libc::c_int) }
    }
}

impl_GObjectFunctions!(TextTag, C_GtkTextTag);
impl_TraitObject!(TextTag, C_GtkTextTag);