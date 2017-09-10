// This file was generated by gir (e95e5d8) from gir-files (857b8f5)
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
    pub struct DOMHTMLDListElement(Object<ffi::WebKitDOMHTMLDListElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_d_list_element_get_type(),
    }
}

pub trait DOMHTMLDListElementExt {
    fn get_compact(&self) -> bool;

    fn set_compact(&self, value: bool);

    fn connect_property_compact_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLDListElement> + IsA<glib::object::Object>> DOMHTMLDListElementExt for O {
    fn get_compact(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_d_list_element_get_compact(self.to_glib_none().0))
        }
    }

    fn set_compact(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_d_list_element_set_compact(self.to_glib_none().0, value.to_glib());
        }
    }

    fn connect_property_compact_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::compact",
                transmute(notify_compact_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_compact_trampoline<P>(this: *mut ffi::WebKitDOMHTMLDListElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLDListElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLDListElement::from_glib_borrow(this).downcast_unchecked())
}
