// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ 3fde76b)
// DO NOT EDIT

use DOMCSSStyleSheet;
use DOMDocument;
use DOMDocumentType;
use DOMHTMLDocument;
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
    pub struct DOMDOMImplementation(Object<ffi::WebKitDOMDOMImplementation, ffi::WebKitDOMDOMImplementationClass>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_dom_implementation_get_type(),
    }
}

pub trait DOMDOMImplementationExt {
    fn create_css_style_sheet(&self, title: &str, media: &str) -> Result<DOMCSSStyleSheet, Error>;

    fn create_document<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b DOMDocumentType>>>(&self, namespaceURI: P, qualifiedName: &str, doctype: Q) -> Result<DOMDocument, Error>;

    fn create_document_type(&self, qualifiedName: &str, publicId: &str, systemId: &str) -> Result<DOMDocumentType, Error>;

    fn create_html_document(&self, title: &str) -> Option<DOMHTMLDocument>;

    fn has_feature(&self, feature: &str, version: &str) -> bool;
}

impl<O: IsA<DOMDOMImplementation>> DOMDOMImplementationExt for O {
    fn create_css_style_sheet(&self, title: &str, media: &str) -> Result<DOMCSSStyleSheet, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_dom_implementation_create_css_style_sheet(self.to_glib_none().0, title.to_glib_none().0, media.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn create_document<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b DOMDocumentType>>>(&self, namespaceURI: P, qualifiedName: &str, doctype: Q) -> Result<DOMDocument, Error> {
        let namespaceURI = namespaceURI.into();
        let namespaceURI = namespaceURI.to_glib_none();
        let doctype = doctype.into();
        let doctype = doctype.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_dom_implementation_create_document(self.to_glib_none().0, namespaceURI.0, qualifiedName.to_glib_none().0, doctype.0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn create_document_type(&self, qualifiedName: &str, publicId: &str, systemId: &str) -> Result<DOMDocumentType, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_dom_implementation_create_document_type(self.to_glib_none().0, qualifiedName.to_glib_none().0, publicId.to_glib_none().0, systemId.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn create_html_document(&self, title: &str) -> Option<DOMHTMLDocument> {
        unsafe {
            from_glib_none(ffi::webkit_dom_dom_implementation_create_html_document(self.to_glib_none().0, title.to_glib_none().0))
        }
    }

    fn has_feature(&self, feature: &str, version: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_dom_implementation_has_feature(self.to_glib_none().0, feature.to_glib_none().0, version.to_glib_none().0))
        }
    }
}
