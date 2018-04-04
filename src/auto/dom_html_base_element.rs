// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ 3fde76b)
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
    pub struct DOMHTMLBaseElement(Object<ffi::WebKitDOMHTMLBaseElement, ffi::WebKitDOMHTMLBaseElementClass>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_base_element_get_type(),
    }
}

pub trait DOMHTMLBaseElementExt {
    fn get_href(&self) -> Option<String>;

    fn get_target(&self) -> Option<String>;

    fn set_href(&self, value: &str);

    fn set_target(&self, value: &str);

    fn connect_property_href_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLBaseElement> + IsA<glib::object::Object>> DOMHTMLBaseElementExt for O {
    fn get_href(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_base_element_get_href(self.to_glib_none().0))
        }
    }

    fn get_target(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_base_element_get_target(self.to_glib_none().0))
        }
    }

    fn set_href(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_base_element_set_href(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_target(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_base_element_set_target(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn connect_property_href_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::href",
                transmute(notify_href_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::target",
                transmute(notify_target_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_href_trampoline<P>(this: *mut ffi::WebKitDOMHTMLBaseElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLBaseElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLBaseElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_target_trampoline<P>(this: *mut ffi::WebKitDOMHTMLBaseElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLBaseElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLBaseElement::from_glib_borrow(this).downcast_unchecked())
}
