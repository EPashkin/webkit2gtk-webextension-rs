// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMNode;
use DOMObject;
use Error;
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
    pub struct DOMXPathResult(Object<ffi::WebKitDOMXPathResult, ffi::WebKitDOMXPathResultClass>): DOMObject;

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

    fn connect_property_boolean_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_invalid_iterator_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_number_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_result_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_single_node_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_snapshot_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_string_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMXPathResult> + IsA<glib::object::Object>> DOMXPathResultExt for O {
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

    fn connect_property_boolean_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::boolean-value",
                transmute(notify_boolean_value_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_invalid_iterator_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::invalid-iterator-state",
                transmute(notify_invalid_iterator_state_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_number_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::number-value",
                transmute(notify_number_value_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_result_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::result-type",
                transmute(notify_result_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_single_node_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::single-node-value",
                transmute(notify_single_node_value_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_snapshot_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::snapshot-length",
                transmute(notify_snapshot_length_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_string_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::string-value",
                transmute(notify_string_value_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_boolean_value_trampoline<P>(this: *mut ffi::WebKitDOMXPathResult, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMXPathResult> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMXPathResult::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_invalid_iterator_state_trampoline<P>(this: *mut ffi::WebKitDOMXPathResult, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMXPathResult> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMXPathResult::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_number_value_trampoline<P>(this: *mut ffi::WebKitDOMXPathResult, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMXPathResult> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMXPathResult::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_result_type_trampoline<P>(this: *mut ffi::WebKitDOMXPathResult, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMXPathResult> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMXPathResult::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_single_node_value_trampoline<P>(this: *mut ffi::WebKitDOMXPathResult, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMXPathResult> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMXPathResult::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_snapshot_length_trampoline<P>(this: *mut ffi::WebKitDOMXPathResult, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMXPathResult> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMXPathResult::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_string_value_trampoline<P>(this: *mut ffi::WebKitDOMXPathResult, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMXPathResult> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMXPathResult::from_glib_borrow(this).downcast_unchecked())
}
