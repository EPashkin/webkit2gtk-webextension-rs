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
use glib::Value;
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
    pub struct DOMHTMLAreaElement(Object<ffi::WebKitDOMHTMLAreaElement, ffi::WebKitDOMHTMLAreaElementClass>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_area_element_get_type(),
    }
}

pub trait DOMHTMLAreaElementExt {
    fn get_alt(&self) -> Option<String>;

    fn get_coords(&self) -> Option<String>;

    fn get_hash(&self) -> Option<String>;

    fn get_host(&self) -> Option<String>;

    fn get_hostname(&self) -> Option<String>;

    fn get_href(&self) -> Option<String>;

    fn get_no_href(&self) -> bool;

    fn get_pathname(&self) -> Option<String>;

    fn get_port(&self) -> Option<String>;

    fn get_protocol(&self) -> Option<String>;

    fn get_search(&self) -> Option<String>;

    fn get_shape(&self) -> Option<String>;

    fn get_target(&self) -> Option<String>;

    fn set_alt(&self, value: &str);

    fn set_coords(&self, value: &str);

    fn set_hash(&self, value: &str);

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_host(&self, value: &str);

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_hostname(&self, value: &str);

    fn set_href(&self, value: &str);

    fn set_no_href(&self, value: bool);

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_pathname(&self, value: &str);

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_port(&self, value: &str);

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_protocol(&self, value: &str);

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_search(&self, value: &str);

    fn set_shape(&self, value: &str);

    fn set_target(&self, value: &str);

    fn set_property_host(&self, host: Option<&str>);

    fn set_property_hostname(&self, hostname: Option<&str>);

    fn set_property_pathname(&self, pathname: Option<&str>);

    fn set_property_port(&self, port: Option<&str>);

    fn set_property_protocol(&self, protocol: Option<&str>);

    fn set_property_search(&self, search: Option<&str>);

    fn connect_property_alt_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_coords_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_hash_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_host_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_hostname_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_href_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_no_href_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pathname_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_port_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_protocol_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_search_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_shape_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLAreaElement> + IsA<glib::object::Object>> DOMHTMLAreaElementExt for O {
    fn get_alt(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_area_element_get_alt(self.to_glib_none().0))
        }
    }

    fn get_coords(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_area_element_get_coords(self.to_glib_none().0))
        }
    }

    fn get_hash(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_area_element_get_hash(self.to_glib_none().0))
        }
    }

    fn get_host(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_area_element_get_host(self.to_glib_none().0))
        }
    }

    fn get_hostname(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_area_element_get_hostname(self.to_glib_none().0))
        }
    }

    fn get_href(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_area_element_get_href(self.to_glib_none().0))
        }
    }

    fn get_no_href(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_area_element_get_no_href(self.to_glib_none().0))
        }
    }

    fn get_pathname(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_area_element_get_pathname(self.to_glib_none().0))
        }
    }

    fn get_port(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_area_element_get_port(self.to_glib_none().0))
        }
    }

    fn get_protocol(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_area_element_get_protocol(self.to_glib_none().0))
        }
    }

    fn get_search(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_area_element_get_search(self.to_glib_none().0))
        }
    }

    fn get_shape(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_area_element_get_shape(self.to_glib_none().0))
        }
    }

    fn get_target(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_area_element_get_target(self.to_glib_none().0))
        }
    }

    fn set_alt(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_area_element_set_alt(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_coords(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_area_element_set_coords(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_hash(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_area_element_set_hash(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_host(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_area_element_set_host(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_hostname(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_area_element_set_hostname(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_href(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_area_element_set_href(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_no_href(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_area_element_set_no_href(self.to_glib_none().0, value.to_glib());
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_pathname(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_area_element_set_pathname(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_port(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_area_element_set_port(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_protocol(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_area_element_set_protocol(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_search(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_area_element_set_search(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_shape(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_area_element_set_shape(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_target(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_area_element_set_target(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_property_host(&self, host: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "host".to_glib_none().0, Value::from(host).to_glib_none().0);
        }
    }

    fn set_property_hostname(&self, hostname: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "hostname".to_glib_none().0, Value::from(hostname).to_glib_none().0);
        }
    }

    fn set_property_pathname(&self, pathname: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "pathname".to_glib_none().0, Value::from(pathname).to_glib_none().0);
        }
    }

    fn set_property_port(&self, port: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "port".to_glib_none().0, Value::from(port).to_glib_none().0);
        }
    }

    fn set_property_protocol(&self, protocol: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "protocol".to_glib_none().0, Value::from(protocol).to_glib_none().0);
        }
    }

    fn set_property_search(&self, search: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "search".to_glib_none().0, Value::from(search).to_glib_none().0);
        }
    }

    fn connect_property_alt_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::alt",
                transmute(notify_alt_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_coords_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::coords",
                transmute(notify_coords_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_hash_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::hash",
                transmute(notify_hash_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_host_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::host",
                transmute(notify_host_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_hostname_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::hostname",
                transmute(notify_hostname_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_href_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::href",
                transmute(notify_href_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_no_href_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::no-href",
                transmute(notify_no_href_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_pathname_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::pathname",
                transmute(notify_pathname_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_port_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::port",
                transmute(notify_port_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_protocol_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::protocol",
                transmute(notify_protocol_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_search_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::search",
                transmute(notify_search_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_shape_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::shape",
                transmute(notify_shape_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
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

unsafe extern "C" fn notify_alt_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAreaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAreaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAreaElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_coords_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAreaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAreaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAreaElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_hash_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAreaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAreaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAreaElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_host_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAreaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAreaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAreaElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_hostname_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAreaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAreaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAreaElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_href_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAreaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAreaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAreaElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_no_href_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAreaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAreaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAreaElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_pathname_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAreaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAreaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAreaElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_port_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAreaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAreaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAreaElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_protocol_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAreaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAreaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAreaElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_search_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAreaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAreaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAreaElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_shape_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAreaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAreaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAreaElement::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_target_trampoline<P>(this: *mut ffi::WebKitDOMHTMLAreaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLAreaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLAreaElement::from_glib_borrow(this).downcast_unchecked())
}
