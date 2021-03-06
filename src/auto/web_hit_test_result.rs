// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMNode;
use HitTestResult;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct WebHitTestResult(Object<ffi::WebKitWebHitTestResult, ffi::WebKitWebHitTestResultClass>): HitTestResult;

    match fn {
        get_type => || ffi::webkit_web_hit_test_result_get_type(),
    }
}

pub trait WebHitTestResultExt {
    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn get_node(&self) -> Option<DOMNode>;

    fn get_property_node(&self) -> Option<DOMNode>;
}

impl<O: IsA<WebHitTestResult> + IsA<glib::object::Object>> WebHitTestResultExt for O {
    #[cfg(any(feature = "v2_8", feature = "dox"))]
    fn get_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_web_hit_test_result_get_node(self.to_glib_none().0))
        }
    }

    fn get_property_node(&self) -> Option<DOMNode> {
        unsafe {
            let mut value = Value::from_type(<DOMNode as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "node".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }
}
