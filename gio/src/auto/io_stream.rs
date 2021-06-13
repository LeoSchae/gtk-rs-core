// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AsyncResult;
use crate::Cancellable;
use crate::InputStream;
use crate::OutputStream;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GIOStream")]
    pub struct IOStream(Object<ffi::GIOStream, ffi::GIOStreamClass>);

    match fn {
        type_ => || ffi::g_io_stream_get_type(),
    }
}

impl IOStream {}

pub const NONE_IO_STREAM: Option<&IOStream> = None;

pub trait IOStreamExt: 'static {
    #[doc(alias = "g_io_stream_clear_pending")]
    fn clear_pending(&self);

    #[doc(alias = "g_io_stream_close")]
    fn close<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), glib::Error>;

    #[doc(alias = "g_io_stream_close_async")]
    fn close_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn close_async_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[doc(alias = "g_io_stream_get_input_stream")]
    #[doc(alias = "get_input_stream")]
    fn input_stream(&self) -> InputStream;

    #[doc(alias = "g_io_stream_get_output_stream")]
    #[doc(alias = "get_output_stream")]
    fn output_stream(&self) -> OutputStream;

    #[doc(alias = "g_io_stream_has_pending")]
    fn has_pending(&self) -> bool;

    #[doc(alias = "g_io_stream_is_closed")]
    fn is_closed(&self) -> bool;

    #[doc(alias = "g_io_stream_set_pending")]
    fn set_pending(&self) -> Result<(), glib::Error>;

    #[doc(alias = "closed")]
    fn connect_closed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<IOStream>> IOStreamExt for O {
    fn clear_pending(&self) {
        unsafe {
            ffi::g_io_stream_clear_pending(self.as_ref().to_glib_none().0);
        }
    }

    fn close<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_io_stream_close(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn close_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn close_async_trampoline<
            Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_io_stream_close_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = close_async_trampoline::<Q>;
        unsafe {
            ffi::g_io_stream_close_async(
                self.as_ref().to_glib_none().0,
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn close_async_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.close_async(io_priority, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    fn input_stream(&self) -> InputStream {
        unsafe {
            from_glib_none(ffi::g_io_stream_get_input_stream(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn output_stream(&self) -> OutputStream {
        unsafe {
            from_glib_none(ffi::g_io_stream_get_output_stream(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_pending(&self) -> bool {
        unsafe { from_glib(ffi::g_io_stream_has_pending(self.as_ref().to_glib_none().0)) }
    }

    fn is_closed(&self) -> bool {
        unsafe { from_glib(ffi::g_io_stream_is_closed(self.as_ref().to_glib_none().0)) }
    }

    fn set_pending(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_io_stream_set_pending(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn connect_closed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_closed_trampoline<P: IsA<IOStream>, F: Fn(&P) + 'static>(
            this: *mut ffi::GIOStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&IOStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::closed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_closed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for IOStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("IOStream")
    }
}
