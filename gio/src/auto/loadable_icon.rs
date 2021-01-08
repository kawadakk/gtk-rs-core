// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AsyncResult;
use crate::Cancellable;
use crate::Icon;
use crate::InputStream;
use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    pub struct LoadableIcon(Interface<ffi::GLoadableIcon>) @requires Icon;

    match fn {
        get_type => || ffi::g_loadable_icon_get_type(),
    }
}

pub const NONE_LOADABLE_ICON: Option<&LoadableIcon> = None;

pub trait LoadableIconExt: 'static {
    #[doc(alias = "g_loadable_icon_load")]
    fn load<P: IsA<Cancellable>>(
        &self,
        size: i32,
        cancellable: Option<&P>,
    ) -> Result<(InputStream, glib::GString), glib::Error>;

    #[doc(alias = "g_loadable_icon_load_async")]
    fn load_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<(InputStream, glib::GString), glib::Error>) + Send + 'static,
    >(
        &self,
        size: i32,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn load_async_future(
        &self,
        size: i32,
    ) -> Pin<
        Box_<
            dyn std::future::Future<Output = Result<(InputStream, glib::GString), glib::Error>>
                + 'static,
        >,
    >;
}

impl<O: IsA<LoadableIcon>> LoadableIconExt for O {
    fn load<P: IsA<Cancellable>>(
        &self,
        size: i32,
        cancellable: Option<&P>,
    ) -> Result<(InputStream, glib::GString), glib::Error> {
        unsafe {
            let mut type_ = ptr::null_mut();
            let mut error = ptr::null_mut();
            let ret = ffi::g_loadable_icon_load(
                self.as_ref().to_glib_none().0,
                size,
                &mut type_,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok((from_glib_full(ret), from_glib_full(type_)))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn load_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<(InputStream, glib::GString), glib::Error>) + Send + 'static,
    >(
        &self,
        size: i32,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn load_async_trampoline<
            Q: FnOnce(Result<(InputStream, glib::GString), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut type_ = ptr::null_mut();
            let ret = ffi::g_loadable_icon_load_finish(
                _source_object as *mut _,
                res,
                &mut type_,
                &mut error,
            );
            let result = if error.is_null() {
                Ok((from_glib_full(ret), from_glib_full(type_)))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = load_async_trampoline::<Q>;
        unsafe {
            ffi::g_loadable_icon_load_async(
                self.as_ref().to_glib_none().0,
                size,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn load_async_future(
        &self,
        size: i32,
    ) -> Pin<
        Box_<
            dyn std::future::Future<Output = Result<(InputStream, glib::GString), glib::Error>>
                + 'static,
        >,
    > {
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.load_async(size, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }
}

impl fmt::Display for LoadableIcon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("LoadableIcon")
    }
}
