// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DOMHTMLTitleElement(Object<ffi::WebKitDOMHTMLTitleElement, ffi::WebKitDOMHTMLTitleElementClass>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_title_element_get_type(),
    }
}

pub trait DOMHTMLTitleElementExt {
    fn get_text(&self) -> Option<String>;

    fn set_text(&self, value: &str);

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLTitleElement> + IsA<glib::object::Object>> DOMHTMLTitleElementExt for O {
    fn get_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_title_element_get_text(self.to_glib_none().0))
        }
    }

    fn set_text(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_title_element_set_text(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::text",
                transmute(notify_text_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_text_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTitleElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTitleElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTitleElement::from_glib_borrow(this).downcast_unchecked())
}
