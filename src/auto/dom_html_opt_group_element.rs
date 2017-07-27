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
    pub struct DOMHTMLOptGroupElement(Object<ffi::WebKitDOMHTMLOptGroupElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_opt_group_element_get_type(),
    }
}

pub trait DOMHTMLOptGroupElementExt {
    fn get_disabled(&self) -> bool;

    fn get_label(&self) -> Option<String>;

    fn set_disabled(&self, value: bool);

    fn set_label(&self, value: &str);
}

impl<O: IsA<DOMHTMLOptGroupElement>> DOMHTMLOptGroupElementExt for O {
    fn get_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_opt_group_element_get_disabled(self.to_glib_none().0))
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_opt_group_element_get_label(self.to_glib_none().0))
        }
    }

    fn set_disabled(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_opt_group_element_set_disabled(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_label(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_opt_group_element_set_label(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}
