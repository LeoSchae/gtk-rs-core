// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::TlsPasswordFlags;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GTlsPassword")]
    pub struct TlsPassword(Object<ffi::GTlsPassword, ffi::GTlsPasswordClass>);

    match fn {
        type_ => || ffi::g_tls_password_get_type(),
    }
}

impl TlsPassword {
    #[doc(alias = "g_tls_password_new")]
    pub fn new(flags: TlsPasswordFlags, description: &str) -> TlsPassword {
        unsafe {
            from_glib_full(ffi::g_tls_password_new(
                flags.into_glib(),
                description.to_glib_none().0,
            ))
        }
    }
}

pub const NONE_TLS_PASSWORD: Option<&TlsPassword> = None;

pub trait TlsPasswordExt: 'static {
    #[doc(alias = "g_tls_password_get_description")]
    #[doc(alias = "get_description")]
    fn description(&self) -> glib::GString;

    #[doc(alias = "g_tls_password_get_flags")]
    #[doc(alias = "get_flags")]
    fn flags(&self) -> TlsPasswordFlags;

    #[doc(alias = "g_tls_password_get_warning")]
    #[doc(alias = "get_warning")]
    fn warning(&self) -> glib::GString;

    #[doc(alias = "g_tls_password_set_description")]
    fn set_description(&self, description: &str);

    #[doc(alias = "g_tls_password_set_flags")]
    fn set_flags(&self, flags: TlsPasswordFlags);

    //#[doc(alias = "g_tls_password_set_value_full")]
    //fn set_value_full(&self, value: &[u8]);

    #[doc(alias = "g_tls_password_set_warning")]
    fn set_warning(&self, warning: &str);

    #[doc(alias = "description")]
    fn connect_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "flags")]
    fn connect_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "warning")]
    fn connect_warning_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TlsPassword>> TlsPasswordExt for O {
    fn description(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::g_tls_password_get_description(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn flags(&self) -> TlsPasswordFlags {
        unsafe {
            from_glib(ffi::g_tls_password_get_flags(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn warning(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::g_tls_password_get_warning(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_description(&self, description: &str) {
        unsafe {
            ffi::g_tls_password_set_description(
                self.as_ref().to_glib_none().0,
                description.to_glib_none().0,
            );
        }
    }

    fn set_flags(&self, flags: TlsPasswordFlags) {
        unsafe {
            ffi::g_tls_password_set_flags(self.as_ref().to_glib_none().0, flags.into_glib());
        }
    }

    //fn set_value_full(&self, value: &[u8]) {
    //    unsafe { TODO: call ffi:g_tls_password_set_value_full() }
    //}

    fn set_warning(&self, warning: &str) {
        unsafe {
            ffi::g_tls_password_set_warning(
                self.as_ref().to_glib_none().0,
                warning.to_glib_none().0,
            );
        }
    }

    fn connect_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_description_trampoline<
            P: IsA<TlsPassword>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GTlsPassword,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&TlsPassword::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::description\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_description_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_flags_trampoline<P: IsA<TlsPassword>, F: Fn(&P) + 'static>(
            this: *mut ffi::GTlsPassword,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&TlsPassword::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_flags_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_warning_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_warning_trampoline<P: IsA<TlsPassword>, F: Fn(&P) + 'static>(
            this: *mut ffi::GTlsPassword,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&TlsPassword::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::warning\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_warning_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for TlsPassword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TlsPassword")
    }
}
