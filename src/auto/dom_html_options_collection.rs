// This file was generated by gir (e95e5d8) from gir-files (857b8f5)
// DO NOT EDIT

use DOMHTMLCollection;
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
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DOMHTMLOptionsCollection(Object<ffi::WebKitDOMHTMLOptionsCollection>): DOMHTMLCollection, DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_html_options_collection_get_type(),
    }
}

pub trait DOMHTMLOptionsCollectionExt {
    fn get_selected_index(&self) -> libc::c_long;

    fn set_selected_index(&self, value: libc::c_long);

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_selected_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLOptionsCollection> + IsA<glib::object::Object>> DOMHTMLOptionsCollectionExt for O {
    fn get_selected_index(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_options_collection_get_selected_index(self.to_glib_none().0)
        }
    }

    fn set_selected_index(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_options_collection_set_selected_index(self.to_glib_none().0, value);
        }
    }

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::length",
                transmute(notify_length_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_selected_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::selected-index",
                transmute(notify_selected_index_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_length_trampoline<P>(this: *mut ffi::WebKitDOMHTMLOptionsCollection, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLOptionsCollection> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLOptionsCollection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_selected_index_trampoline<P>(this: *mut ffi::WebKitDOMHTMLOptionsCollection, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLOptionsCollection> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLOptionsCollection::from_glib_borrow(this).downcast_unchecked())
}
