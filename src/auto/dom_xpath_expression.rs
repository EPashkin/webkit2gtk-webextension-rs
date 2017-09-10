// This file was generated by gir (e95e5d8) from gir-files (857b8f5)
// DO NOT EDIT

use DOMNode;
use DOMObject;
use DOMXPathResult;
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
    pub struct DOMXPathExpression(Object<ffi::WebKitDOMXPathExpression>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_xpath_expression_get_type(),
    }
}

pub trait DOMXPathExpressionExt {
    fn evaluate<P: IsA<DOMNode>>(&self, contextNode: &P, type_: libc::c_ushort, inResult: &DOMXPathResult) -> Result<DOMXPathResult, Error>;
}

impl<O: IsA<DOMXPathExpression>> DOMXPathExpressionExt for O {
    fn evaluate<P: IsA<DOMNode>>(&self, contextNode: &P, type_: libc::c_ushort, inResult: &DOMXPathResult) -> Result<DOMXPathResult, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_xpath_expression_evaluate(self.to_glib_none().0, contextNode.to_glib_none().0, type_, inResult.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}
