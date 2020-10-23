// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use gobject_sys;
use graphene_sys;
use Point;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct Rect(Boxed<graphene_sys::graphene_rect_t>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(graphene_sys::graphene_rect_get_type(), ptr as *mut _) as *mut graphene_sys::graphene_rect_t,
        free => |ptr| gobject_sys::g_boxed_free(graphene_sys::graphene_rect_get_type(), ptr as *mut _),
        init => |_ptr| (),
        clear => |_ptr| (),
        get_type => || graphene_sys::graphene_rect_get_type(),
    }
}

impl Rect {
    pub fn contains_point(&self, p: &Point) -> bool {
        unsafe {
            from_glib(graphene_sys::graphene_rect_contains_point(
                self.to_glib_none().0,
                p.to_glib_none().0,
            ))
        }
    }

    pub fn contains_rect(&self, b: &Rect) -> bool {
        unsafe {
            from_glib(graphene_sys::graphene_rect_contains_rect(
                self.to_glib_none().0,
                b.to_glib_none().0,
            ))
        }
    }

    fn equal(&self, b: &Rect) -> bool {
        unsafe {
            from_glib(graphene_sys::graphene_rect_equal(
                self.to_glib_none().0,
                b.to_glib_none().0,
            ))
        }
    }

    pub fn expand(&self, p: &Point) -> Rect {
        unsafe {
            let mut res = Rect::uninitialized();
            graphene_sys::graphene_rect_expand(
                self.to_glib_none().0,
                p.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    pub fn get_bottom_left(&self) -> Point {
        unsafe {
            let mut p = Point::uninitialized();
            graphene_sys::graphene_rect_get_bottom_left(
                self.to_glib_none().0,
                p.to_glib_none_mut().0,
            );
            p
        }
    }

    pub fn get_bottom_right(&self) -> Point {
        unsafe {
            let mut p = Point::uninitialized();
            graphene_sys::graphene_rect_get_bottom_right(
                self.to_glib_none().0,
                p.to_glib_none_mut().0,
            );
            p
        }
    }

    pub fn get_center(&self) -> Point {
        unsafe {
            let mut p = Point::uninitialized();
            graphene_sys::graphene_rect_get_center(self.to_glib_none().0, p.to_glib_none_mut().0);
            p
        }
    }

    pub fn get_height(&self) -> f32 {
        unsafe { graphene_sys::graphene_rect_get_height(self.to_glib_none().0) }
    }

    pub fn get_top_left(&self) -> Point {
        unsafe {
            let mut p = Point::uninitialized();
            graphene_sys::graphene_rect_get_top_left(self.to_glib_none().0, p.to_glib_none_mut().0);
            p
        }
    }

    pub fn get_top_right(&self) -> Point {
        unsafe {
            let mut p = Point::uninitialized();
            graphene_sys::graphene_rect_get_top_right(
                self.to_glib_none().0,
                p.to_glib_none_mut().0,
            );
            p
        }
    }

    //pub fn get_vertices(&self, vertices: /*Unimplemented*/FixedArray TypeId { ns_id: 1, id: 16 }; 4) {
    //    unsafe { TODO: call graphene_sys:graphene_rect_get_vertices() }
    //}

    pub fn get_width(&self) -> f32 {
        unsafe { graphene_sys::graphene_rect_get_width(self.to_glib_none().0) }
    }

    pub fn get_x(&self) -> f32 {
        unsafe { graphene_sys::graphene_rect_get_x(self.to_glib_none().0) }
    }

    pub fn get_y(&self) -> f32 {
        unsafe { graphene_sys::graphene_rect_get_y(self.to_glib_none().0) }
    }

    pub fn init(&mut self, x: f32, y: f32, width: f32, height: f32) {
        unsafe {
            graphene_sys::graphene_rect_init(self.to_glib_none_mut().0, x, y, width, height);
        }
    }

    pub fn init_from_rect(&mut self, src: &Rect) {
        unsafe {
            graphene_sys::graphene_rect_init_from_rect(
                self.to_glib_none_mut().0,
                src.to_glib_none().0,
            );
        }
    }

    pub fn inset(&mut self, d_x: f32, d_y: f32) -> Option<Rect> {
        unsafe {
            from_glib_none(graphene_sys::graphene_rect_inset(
                self.to_glib_none_mut().0,
                d_x,
                d_y,
            ))
        }
    }

    pub fn inset_r(&self, d_x: f32, d_y: f32) -> Rect {
        unsafe {
            let mut res = Rect::uninitialized();
            graphene_sys::graphene_rect_inset_r(
                self.to_glib_none().0,
                d_x,
                d_y,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    pub fn interpolate(&self, b: &Rect, factor: f64) -> Rect {
        unsafe {
            let mut res = Rect::uninitialized();
            graphene_sys::graphene_rect_interpolate(
                self.to_glib_none().0,
                b.to_glib_none().0,
                factor,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    pub fn intersection(&self, b: &Rect) -> Option<Rect> {
        unsafe {
            let mut res = Rect::uninitialized();
            let ret = from_glib(graphene_sys::graphene_rect_intersection(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            ));
            if ret {
                Some(res)
            } else {
                None
            }
        }
    }

    pub fn normalize(&mut self) -> Option<Rect> {
        unsafe {
            from_glib_none(graphene_sys::graphene_rect_normalize(
                self.to_glib_none_mut().0,
            ))
        }
    }

    pub fn normalize_r(&self) -> Rect {
        unsafe {
            let mut res = Rect::uninitialized();
            graphene_sys::graphene_rect_normalize_r(
                self.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    pub fn offset(&mut self, d_x: f32, d_y: f32) -> Option<Rect> {
        unsafe {
            from_glib_none(graphene_sys::graphene_rect_offset(
                self.to_glib_none_mut().0,
                d_x,
                d_y,
            ))
        }
    }

    pub fn offset_r(&self, d_x: f32, d_y: f32) -> Rect {
        unsafe {
            let mut res = Rect::uninitialized();
            graphene_sys::graphene_rect_offset_r(
                self.to_glib_none().0,
                d_x,
                d_y,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    pub fn round(&self) -> Rect {
        unsafe {
            let mut res = Rect::uninitialized();
            graphene_sys::graphene_rect_round(self.to_glib_none().0, res.to_glib_none_mut().0);
            res
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn scale(&self, s_h: f32, s_v: f32) -> Rect {
        unsafe {
            let mut res = Rect::uninitialized();
            graphene_sys::graphene_rect_scale(
                self.to_glib_none().0,
                s_h,
                s_v,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    pub fn union(&self, b: &Rect) -> Rect {
        unsafe {
            let mut res = Rect::uninitialized();
            graphene_sys::graphene_rect_union(
                self.to_glib_none().0,
                b.to_glib_none().0,
                res.to_glib_none_mut().0,
            );
            res
        }
    }

    pub fn zero() -> Rect {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(graphene_sys::graphene_rect_zero()) }
    }
}

impl PartialEq for Rect {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Rect {}
