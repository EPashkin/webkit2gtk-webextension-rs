// This file was generated by gir (0f7cd61) from gir-files (0bcaef9)
// DO NOT EDIT

use DOMNode;
use DOMObject;
use Error;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct DOMXPathResult(Object<ffi::WebKitDOMXPathResult>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_xpath_result_get_type(),
    }
}

pub trait DOMXPathResultExt {
    fn get_boolean_value(&self) -> Result<(), Error>;

    fn get_invalid_iterator_state(&self) -> bool;

    fn get_number_value(&self) -> Result<f64, Error>;

    fn get_result_type(&self) -> libc::c_ushort;

    fn get_single_node_value(&self) -> Result<DOMNode, Error>;

    fn get_snapshot_length(&self) -> Result<libc::c_ulong, Error>;

    fn get_string_value(&self) -> Result<String, Error>;

    fn iterate_next(&self) -> Result<DOMNode, Error>;

    fn snapshot_item(&self, index: libc::c_ulong) -> Result<DOMNode, Error>;
}

impl<O: IsA<DOMXPathResult>> DOMXPathResultExt for O {
    fn get_boolean_value(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_xpath_result_get_boolean_value(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_invalid_iterator_state(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_xpath_result_get_invalid_iterator_state(self.to_glib_none().0))
        }
    }

    fn get_number_value(&self) -> Result<f64, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_xpath_result_get_number_value(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_result_type(&self) -> libc::c_ushort {
        unsafe {
            ffi::webkit_dom_xpath_result_get_result_type(self.to_glib_none().0)
        }
    }

    fn get_single_node_value(&self) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_xpath_result_get_single_node_value(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_snapshot_length(&self) -> Result<libc::c_ulong, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_xpath_result_get_snapshot_length(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_string_value(&self) -> Result<String, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_xpath_result_get_string_value(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn iterate_next(&self) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_xpath_result_iterate_next(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn snapshot_item(&self, index: libc::c_ulong) -> Result<DOMNode, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_xpath_result_snapshot_item(self.to_glib_none().0, index, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}
