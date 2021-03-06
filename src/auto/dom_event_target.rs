// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMEvent;
use Error;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct DOMEventTarget(Object<ffi::WebKitDOMEventTarget, ffi::WebKitDOMEventTargetIface>);

    match fn {
        get_type => || ffi::webkit_dom_event_target_get_type(),
    }
}

pub trait DOMEventTargetExt {
    //fn add_event_listener(&self, event_name: &str, handler: /*Unknown conversion*//*Unimplemented*/Callback, use_capture: bool) -> bool;

    fn add_event_listener_with_closure(&self, event_name: &str, handler: &glib::Closure, use_capture: bool) -> bool;

    fn dispatch_event<P: IsA<DOMEvent>>(&self, event: &P) -> Result<(), Error>;

    //fn remove_event_listener(&self, event_name: &str, handler: /*Unknown conversion*//*Unimplemented*/Callback, use_capture: bool) -> bool;

    fn remove_event_listener_with_closure(&self, event_name: &str, handler: &glib::Closure, use_capture: bool) -> bool;
}

impl<O: IsA<DOMEventTarget>> DOMEventTargetExt for O {
    //fn add_event_listener(&self, event_name: &str, handler: /*Unknown conversion*//*Unimplemented*/Callback, use_capture: bool) -> bool {
    //    unsafe { TODO: call ffi::webkit_dom_event_target_add_event_listener() }
    //}

    fn add_event_listener_with_closure(&self, event_name: &str, handler: &glib::Closure, use_capture: bool) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_event_target_add_event_listener_with_closure(self.to_glib_none().0, event_name.to_glib_none().0, handler.to_glib_none().0, use_capture.to_glib()))
        }
    }

    fn dispatch_event<P: IsA<DOMEvent>>(&self, event: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_event_target_dispatch_event(self.to_glib_none().0, event.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    //fn remove_event_listener(&self, event_name: &str, handler: /*Unknown conversion*//*Unimplemented*/Callback, use_capture: bool) -> bool {
    //    unsafe { TODO: call ffi::webkit_dom_event_target_remove_event_listener() }
    //}

    fn remove_event_listener_with_closure(&self, event_name: &str, handler: &glib::Closure, use_capture: bool) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_event_target_remove_event_listener_with_closure(self.to_glib_none().0, event_name.to_glib_none().0, handler.to_glib_none().0, use_capture.to_glib()))
        }
    }
}
