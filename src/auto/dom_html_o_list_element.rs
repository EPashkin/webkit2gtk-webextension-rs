// This file was generated by gir (e95e5d8) from gir-files (857b8f5)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DOMHTMLOListElement(Object<ffi::WebKitDOMHTMLOListElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_o_list_element_get_type(),
    }
}

pub trait DOMHTMLOListElementExt {
    fn get_compact(&self) -> bool;

    fn get_start(&self) -> libc::c_long;

    fn get_type_attr(&self) -> Option<String>;

    fn set_compact(&self, value: bool);

    fn set_start(&self, value: libc::c_long);

    fn set_type_attr(&self, value: &str);

    fn get_property_type(&self) -> Option<String>;

    fn set_property_type(&self, type_: Option<&str>);

    fn connect_property_compact_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLOListElement> + IsA<glib::object::Object>> DOMHTMLOListElementExt for O {
    fn get_compact(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_o_list_element_get_compact(self.to_glib_none().0))
        }
    }

    fn get_start(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_o_list_element_get_start(self.to_glib_none().0)
        }
    }

    fn get_type_attr(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_o_list_element_get_type_attr(self.to_glib_none().0))
        }
    }

    fn set_compact(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_o_list_element_set_compact(self.to_glib_none().0, value.to_glib());
        }
    }

    fn set_start(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_o_list_element_set_start(self.to_glib_none().0, value);
        }
    }

    fn set_type_attr(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_o_list_element_set_type_attr(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn get_property_type(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "type".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_type(&self, type_: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "type".to_glib_none().0, Value::from(type_).to_glib_none().0);
        }
    }

    fn connect_property_compact_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::compact",
                transmute(notify_compact_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::start",
                transmute(notify_start_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::type",
                transmute(notify_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_compact_trampoline<P>(this: *mut ffi::WebKitDOMHTMLOListElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLOListElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLOListElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_start_trampoline<P>(this: *mut ffi::WebKitDOMHTMLOListElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLOListElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLOListElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_type_trampoline<P>(this: *mut ffi::WebKitDOMHTMLOListElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLOListElement> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLOListElement::from_glib_borrow(this).downcast_unchecked())
}
