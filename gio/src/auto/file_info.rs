// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::FileAttributeMatcher;
use crate::FileAttributeStatus;
use crate::FileAttributeType;
use crate::FileType;
use crate::Icon;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GFileInfo")]
    pub struct FileInfo(Object<ffi::GFileInfo, ffi::GFileInfoClass>);

    match fn {
        type_ => || ffi::g_file_info_get_type(),
    }
}

impl FileInfo {
    #[doc(alias = "g_file_info_new")]
    pub fn new() -> FileInfo {
        unsafe { from_glib_full(ffi::g_file_info_new()) }
    }

    #[doc(alias = "g_file_info_clear_status")]
    pub fn clear_status(&self) {
        unsafe {
            ffi::g_file_info_clear_status(self.to_glib_none().0);
        }
    }

    #[doc(alias = "g_file_info_copy_into")]
    pub fn copy_into(&self, dest_info: &FileInfo) {
        unsafe {
            ffi::g_file_info_copy_into(self.to_glib_none().0, dest_info.to_glib_none().0);
        }
    }

    #[doc(alias = "g_file_info_dup")]
    #[must_use]
    pub fn dup(&self) -> FileInfo {
        unsafe { from_glib_full(ffi::g_file_info_dup(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v2_70", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_70")))]
    #[doc(alias = "g_file_info_get_access_date_time")]
    #[doc(alias = "get_access_date_time")]
    pub fn access_date_time(&self) -> Option<glib::DateTime> {
        unsafe { from_glib_full(ffi::g_file_info_get_access_date_time(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_file_info_get_attribute_as_string")]
    #[doc(alias = "get_attribute_as_string")]
    pub fn attribute_as_string(&self, attribute: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::g_file_info_get_attribute_as_string(
                self.to_glib_none().0,
                attribute.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_file_info_get_attribute_boolean")]
    #[doc(alias = "get_attribute_boolean")]
    pub fn boolean(&self, attribute: &str) -> bool {
        unsafe {
            from_glib(ffi::g_file_info_get_attribute_boolean(
                self.to_glib_none().0,
                attribute.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_file_info_get_attribute_byte_string")]
    #[doc(alias = "get_attribute_byte_string")]
    pub fn attribute_byte_string(&self, attribute: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_file_info_get_attribute_byte_string(
                self.to_glib_none().0,
                attribute.to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "g_file_info_get_attribute_data")]
    //#[doc(alias = "get_attribute_data")]
    //pub fn attribute_data(&self, attribute: &str, value_pp: /*Unimplemented*/&mut Basic: Pointer) -> Option<(FileAttributeType, FileAttributeStatus)> {
    //    unsafe { TODO: call ffi:g_file_info_get_attribute_data() }
    //}

    #[doc(alias = "g_file_info_get_attribute_int32")]
    #[doc(alias = "get_attribute_int32")]
    pub fn attribute_int32(&self, attribute: &str) -> i32 {
        unsafe {
            ffi::g_file_info_get_attribute_int32(self.to_glib_none().0, attribute.to_glib_none().0)
        }
    }

    #[doc(alias = "g_file_info_get_attribute_int64")]
    #[doc(alias = "get_attribute_int64")]
    pub fn attribute_int64(&self, attribute: &str) -> i64 {
        unsafe {
            ffi::g_file_info_get_attribute_int64(self.to_glib_none().0, attribute.to_glib_none().0)
        }
    }

    #[doc(alias = "g_file_info_get_attribute_object")]
    #[doc(alias = "get_attribute_object")]
    pub fn attribute_object(&self, attribute: &str) -> Option<glib::Object> {
        unsafe {
            from_glib_none(ffi::g_file_info_get_attribute_object(
                self.to_glib_none().0,
                attribute.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_file_info_get_attribute_status")]
    #[doc(alias = "get_attribute_status")]
    pub fn attribute_status(&self, attribute: &str) -> FileAttributeStatus {
        unsafe {
            from_glib(ffi::g_file_info_get_attribute_status(
                self.to_glib_none().0,
                attribute.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_file_info_get_attribute_string")]
    #[doc(alias = "get_attribute_string")]
    pub fn attribute_string(&self, attribute: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_file_info_get_attribute_string(
                self.to_glib_none().0,
                attribute.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_file_info_get_attribute_stringv")]
    #[doc(alias = "get_attribute_stringv")]
    pub fn attribute_stringv(&self, attribute: &str) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_file_info_get_attribute_stringv(
                self.to_glib_none().0,
                attribute.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_file_info_get_attribute_type")]
    #[doc(alias = "get_attribute_type")]
    pub fn attribute_type(&self, attribute: &str) -> FileAttributeType {
        unsafe {
            from_glib(ffi::g_file_info_get_attribute_type(
                self.to_glib_none().0,
                attribute.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_file_info_get_attribute_uint32")]
    #[doc(alias = "get_attribute_uint32")]
    pub fn attribute_uint32(&self, attribute: &str) -> u32 {
        unsafe {
            ffi::g_file_info_get_attribute_uint32(self.to_glib_none().0, attribute.to_glib_none().0)
        }
    }

    #[doc(alias = "g_file_info_get_attribute_uint64")]
    #[doc(alias = "get_attribute_uint64")]
    pub fn attribute_uint64(&self, attribute: &str) -> u64 {
        unsafe {
            ffi::g_file_info_get_attribute_uint64(self.to_glib_none().0, attribute.to_glib_none().0)
        }
    }

    #[doc(alias = "g_file_info_get_content_type")]
    #[doc(alias = "get_content_type")]
    pub fn content_type(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::g_file_info_get_content_type(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v2_70", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_70")))]
    #[doc(alias = "g_file_info_get_creation_date_time")]
    #[doc(alias = "get_creation_date_time")]
    pub fn creation_date_time(&self) -> Option<glib::DateTime> {
        unsafe {
            from_glib_full(ffi::g_file_info_get_creation_date_time(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_file_info_get_deletion_date")]
    #[doc(alias = "get_deletion_date")]
    pub fn deletion_date(&self) -> Option<glib::DateTime> {
        unsafe { from_glib_full(ffi::g_file_info_get_deletion_date(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_file_info_get_display_name")]
    #[doc(alias = "get_display_name")]
    pub fn display_name(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::g_file_info_get_display_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_file_info_get_edit_name")]
    #[doc(alias = "get_edit_name")]
    pub fn edit_name(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::g_file_info_get_edit_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_file_info_get_etag")]
    #[doc(alias = "get_etag")]
    pub fn etag(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::g_file_info_get_etag(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_file_info_get_file_type")]
    #[doc(alias = "get_file_type")]
    pub fn file_type(&self) -> FileType {
        unsafe { from_glib(ffi::g_file_info_get_file_type(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_file_info_get_icon")]
    #[doc(alias = "get_icon")]
    pub fn icon(&self) -> Option<Icon> {
        unsafe { from_glib_none(ffi::g_file_info_get_icon(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_file_info_get_is_backup")]
    #[doc(alias = "get_is_backup")]
    pub fn is_backup(&self) -> bool {
        unsafe { from_glib(ffi::g_file_info_get_is_backup(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_file_info_get_is_hidden")]
    #[doc(alias = "get_is_hidden")]
    pub fn is_hidden(&self) -> bool {
        unsafe { from_glib(ffi::g_file_info_get_is_hidden(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_file_info_get_is_symlink")]
    #[doc(alias = "get_is_symlink")]
    pub fn is_symlink(&self) -> bool {
        unsafe { from_glib(ffi::g_file_info_get_is_symlink(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v2_62", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_62")))]
    #[doc(alias = "g_file_info_get_modification_date_time")]
    #[doc(alias = "get_modification_date_time")]
    pub fn modification_date_time(&self) -> Option<glib::DateTime> {
        unsafe {
            from_glib_full(ffi::g_file_info_get_modification_date_time(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_file_info_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(&self) -> std::path::PathBuf {
        unsafe { from_glib_none(ffi::g_file_info_get_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_file_info_get_size")]
    #[doc(alias = "get_size")]
    pub fn size(&self) -> i64 {
        unsafe { ffi::g_file_info_get_size(self.to_glib_none().0) }
    }

    #[doc(alias = "g_file_info_get_sort_order")]
    #[doc(alias = "get_sort_order")]
    pub fn sort_order(&self) -> i32 {
        unsafe { ffi::g_file_info_get_sort_order(self.to_glib_none().0) }
    }

    #[doc(alias = "g_file_info_get_symbolic_icon")]
    #[doc(alias = "get_symbolic_icon")]
    pub fn symbolic_icon(&self) -> Option<Icon> {
        unsafe { from_glib_none(ffi::g_file_info_get_symbolic_icon(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_file_info_get_symlink_target")]
    #[doc(alias = "get_symlink_target")]
    pub fn symlink_target(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::g_file_info_get_symlink_target(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_file_info_has_attribute")]
    pub fn has_attribute(&self, attribute: &str) -> bool {
        unsafe {
            from_glib(ffi::g_file_info_has_attribute(
                self.to_glib_none().0,
                attribute.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_file_info_has_namespace")]
    pub fn has_namespace(&self, name_space: &str) -> bool {
        unsafe {
            from_glib(ffi::g_file_info_has_namespace(
                self.to_glib_none().0,
                name_space.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_file_info_list_attributes")]
    pub fn list_attributes(&self, name_space: Option<&str>) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_file_info_list_attributes(
                self.to_glib_none().0,
                name_space.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_file_info_remove_attribute")]
    pub fn remove_attribute(&self, attribute: &str) {
        unsafe {
            ffi::g_file_info_remove_attribute(self.to_glib_none().0, attribute.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_70", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_70")))]
    #[doc(alias = "g_file_info_set_access_date_time")]
    pub fn set_access_date_time(&self, atime: &glib::DateTime) {
        unsafe {
            ffi::g_file_info_set_access_date_time(self.to_glib_none().0, atime.to_glib_none().0);
        }
    }

    //#[doc(alias = "g_file_info_set_attribute")]
    //pub fn set_attribute(&self, attribute: &str, type_: FileAttributeType, value_p: /*Unimplemented*/Basic: Pointer) {
    //    unsafe { TODO: call ffi:g_file_info_set_attribute() }
    //}

    #[doc(alias = "g_file_info_set_attribute_boolean")]
    pub fn set_attribute_boolean(&self, attribute: &str, attr_value: bool) {
        unsafe {
            ffi::g_file_info_set_attribute_boolean(
                self.to_glib_none().0,
                attribute.to_glib_none().0,
                attr_value.into_glib(),
            );
        }
    }

    #[doc(alias = "g_file_info_set_attribute_byte_string")]
    pub fn set_attribute_byte_string(&self, attribute: &str, attr_value: &str) {
        unsafe {
            ffi::g_file_info_set_attribute_byte_string(
                self.to_glib_none().0,
                attribute.to_glib_none().0,
                attr_value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_file_info_set_attribute_int32")]
    pub fn set_attribute_int32(&self, attribute: &str, attr_value: i32) {
        unsafe {
            ffi::g_file_info_set_attribute_int32(
                self.to_glib_none().0,
                attribute.to_glib_none().0,
                attr_value,
            );
        }
    }

    #[doc(alias = "g_file_info_set_attribute_int64")]
    pub fn set_attribute_int64(&self, attribute: &str, attr_value: i64) {
        unsafe {
            ffi::g_file_info_set_attribute_int64(
                self.to_glib_none().0,
                attribute.to_glib_none().0,
                attr_value,
            );
        }
    }

    #[doc(alias = "g_file_info_set_attribute_mask")]
    pub fn set_attribute_mask(&self, mask: &FileAttributeMatcher) {
        unsafe {
            ffi::g_file_info_set_attribute_mask(self.to_glib_none().0, mask.to_glib_none().0);
        }
    }

    #[doc(alias = "g_file_info_set_attribute_object")]
    pub fn set_attribute_object(&self, attribute: &str, attr_value: &impl IsA<glib::Object>) {
        unsafe {
            ffi::g_file_info_set_attribute_object(
                self.to_glib_none().0,
                attribute.to_glib_none().0,
                attr_value.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_file_info_set_attribute_status")]
    pub fn set_attribute_status(&self, attribute: &str, status: FileAttributeStatus) -> bool {
        unsafe {
            from_glib(ffi::g_file_info_set_attribute_status(
                self.to_glib_none().0,
                attribute.to_glib_none().0,
                status.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_file_info_set_attribute_string")]
    pub fn set_attribute_string(&self, attribute: &str, attr_value: &str) {
        unsafe {
            ffi::g_file_info_set_attribute_string(
                self.to_glib_none().0,
                attribute.to_glib_none().0,
                attr_value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_file_info_set_attribute_stringv")]
    pub fn set_attribute_stringv(&self, attribute: &str, attr_value: &[&str]) {
        unsafe {
            ffi::g_file_info_set_attribute_stringv(
                self.to_glib_none().0,
                attribute.to_glib_none().0,
                attr_value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_file_info_set_attribute_uint32")]
    pub fn set_attribute_uint32(&self, attribute: &str, attr_value: u32) {
        unsafe {
            ffi::g_file_info_set_attribute_uint32(
                self.to_glib_none().0,
                attribute.to_glib_none().0,
                attr_value,
            );
        }
    }

    #[doc(alias = "g_file_info_set_attribute_uint64")]
    pub fn set_attribute_uint64(&self, attribute: &str, attr_value: u64) {
        unsafe {
            ffi::g_file_info_set_attribute_uint64(
                self.to_glib_none().0,
                attribute.to_glib_none().0,
                attr_value,
            );
        }
    }

    #[doc(alias = "g_file_info_set_content_type")]
    pub fn set_content_type(&self, content_type: &str) {
        unsafe {
            ffi::g_file_info_set_content_type(self.to_glib_none().0, content_type.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_70", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_70")))]
    #[doc(alias = "g_file_info_set_creation_date_time")]
    pub fn set_creation_date_time(&self, creation_time: &glib::DateTime) {
        unsafe {
            ffi::g_file_info_set_creation_date_time(
                self.to_glib_none().0,
                creation_time.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_file_info_set_display_name")]
    pub fn set_display_name(&self, display_name: &str) {
        unsafe {
            ffi::g_file_info_set_display_name(self.to_glib_none().0, display_name.to_glib_none().0);
        }
    }

    #[doc(alias = "g_file_info_set_edit_name")]
    pub fn set_edit_name(&self, edit_name: &str) {
        unsafe {
            ffi::g_file_info_set_edit_name(self.to_glib_none().0, edit_name.to_glib_none().0);
        }
    }

    #[doc(alias = "g_file_info_set_file_type")]
    pub fn set_file_type(&self, type_: FileType) {
        unsafe {
            ffi::g_file_info_set_file_type(self.to_glib_none().0, type_.into_glib());
        }
    }

    #[doc(alias = "g_file_info_set_icon")]
    pub fn set_icon(&self, icon: &impl IsA<Icon>) {
        unsafe {
            ffi::g_file_info_set_icon(self.to_glib_none().0, icon.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "g_file_info_set_is_hidden")]
    pub fn set_is_hidden(&self, is_hidden: bool) {
        unsafe {
            ffi::g_file_info_set_is_hidden(self.to_glib_none().0, is_hidden.into_glib());
        }
    }

    #[doc(alias = "g_file_info_set_is_symlink")]
    pub fn set_is_symlink(&self, is_symlink: bool) {
        unsafe {
            ffi::g_file_info_set_is_symlink(self.to_glib_none().0, is_symlink.into_glib());
        }
    }

    #[cfg(any(feature = "v2_62", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_62")))]
    #[doc(alias = "g_file_info_set_modification_date_time")]
    pub fn set_modification_date_time(&self, mtime: &glib::DateTime) {
        unsafe {
            ffi::g_file_info_set_modification_date_time(
                self.to_glib_none().0,
                mtime.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_file_info_set_name")]
    pub fn set_name(&self, name: impl AsRef<std::path::Path>) {
        unsafe {
            ffi::g_file_info_set_name(self.to_glib_none().0, name.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "g_file_info_set_size")]
    pub fn set_size(&self, size: i64) {
        unsafe {
            ffi::g_file_info_set_size(self.to_glib_none().0, size);
        }
    }

    #[doc(alias = "g_file_info_set_sort_order")]
    pub fn set_sort_order(&self, sort_order: i32) {
        unsafe {
            ffi::g_file_info_set_sort_order(self.to_glib_none().0, sort_order);
        }
    }

    #[doc(alias = "g_file_info_set_symbolic_icon")]
    pub fn set_symbolic_icon(&self, icon: &impl IsA<Icon>) {
        unsafe {
            ffi::g_file_info_set_symbolic_icon(
                self.to_glib_none().0,
                icon.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_file_info_set_symlink_target")]
    pub fn set_symlink_target(&self, symlink_target: &str) {
        unsafe {
            ffi::g_file_info_set_symlink_target(
                self.to_glib_none().0,
                symlink_target.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_file_info_unset_attribute_mask")]
    pub fn unset_attribute_mask(&self) {
        unsafe {
            ffi::g_file_info_unset_attribute_mask(self.to_glib_none().0);
        }
    }
}

impl Default for FileInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for FileInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FileInfo")
    }
}
