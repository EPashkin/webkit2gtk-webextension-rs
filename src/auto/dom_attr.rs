// This file was generated by gir (0f7cd61) from gir-files (0bcaef9)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMNode;
use DOMObject;
use Error;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct DOMAttr(Object<ffi::WebKitDOMAttr>): DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_attr_get_type(),
    }
}

pub trait DOMAttrExt {
    fn get_name(&self) -> Option<String>;

    fn get_owner_element(&self) -> Option<DOMElement>;

    fn get_specified(&self) -> bool;

    fn get_value(&self) -> Option<String>;

    fn set_value(&self, value: &str) -> Result<(), Error>;
}

impl<O: IsA<DOMAttr>> DOMAttrExt for O {
    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_attr_get_name(self.to_glib_none().0))
        }
    }

    fn get_owner_element(&self) -> Option<DOMElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_attr_get_owner_element(self.to_glib_none().0))
        }
    }

    fn get_specified(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_attr_get_specified(self.to_glib_none().0))
        }
    }

    fn get_value(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_attr_get_value(self.to_glib_none().0))
        }
    }

    fn set_value(&self, value: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_attr_set_value(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
}
