// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ 3fde76b)
// DO NOT EDIT

use WebPage;
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
    pub struct WebExtension(Object<ffi::WebKitWebExtension, ffi::WebKitWebExtensionClass>);

    match fn {
        get_type => || ffi::webkit_web_extension_get_type(),
    }
}

pub trait WebExtensionExt {
    fn get_page(&self, page_id: u64) -> Option<WebPage>;

    fn connect_page_created<F: Fn(&Self, &WebPage) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<WebExtension> + IsA<glib::object::Object>> WebExtensionExt for O {
    fn get_page(&self, page_id: u64) -> Option<WebPage> {
        unsafe {
            from_glib_none(ffi::webkit_web_extension_get_page(self.to_glib_none().0, page_id))
        }
    }

    fn connect_page_created<F: Fn(&Self, &WebPage) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &WebPage) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "page-created",
                transmute(page_created_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn page_created_trampoline<P>(this: *mut ffi::WebKitWebExtension, web_page: *mut ffi::WebKitWebPage, f: glib_ffi::gpointer)
where P: IsA<WebExtension> {
    callback_guard!();
    let f: &&(Fn(&P, &WebPage) + 'static) = transmute(f);
    f(&WebExtension::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(web_page))
}
