// This file was generated by gir (0f7cd61) from gir-files (0bcaef9)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct DOMHTMLFrameSetElement(Object<ffi::WebKitDOMHTMLFrameSetElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_frame_set_element_get_type(),
    }
}

pub trait DOMHTMLFrameSetElementExt {
    fn get_cols(&self) -> Option<String>;

    fn get_rows(&self) -> Option<String>;

    fn set_cols(&self, value: &str);

    fn set_rows(&self, value: &str);
}

impl<O: IsA<DOMHTMLFrameSetElement>> DOMHTMLFrameSetElementExt for O {
    fn get_cols(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_frame_set_element_get_cols(self.to_glib_none().0))
        }
    }

    fn get_rows(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_frame_set_element_get_rows(self.to_glib_none().0))
        }
    }

    fn set_cols(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_frame_set_element_set_cols(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_rows(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_frame_set_element_set_rows(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}
