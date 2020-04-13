#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(unused_imports)]

mod bindings;
mod traits;
mod context;
mod arrays;

use std::ffi::CStr;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::os::raw::c_char;
use std::result::Result as StdResult;

use crate::traits::*;
pub use crate::arrays::*;
pub use context::FutharkContext;

#[derive(Debug)]
pub enum Error {
    FutharkError(FutharkError),
    SizeMismatch(usize, usize),
}

type Result<T> = StdResult<T, Error>;

impl From<FutharkError> for Error {
    fn from(err: FutharkError) -> Self {
        Error::FutharkError(err)
    }
}

#[derive(Debug)]
pub struct FutharkError {
    error: String,
}

impl FutharkError {
    pub(crate) fn new(ctx: *mut bindings::futhark_context) -> Self {
        unsafe {
            Self::_new(bindings::futhark_context_get_error(ctx))
        }
    }
    
    pub(crate) fn _new(err: *mut ::std::os::raw::c_char) -> Self {
        unsafe {
            Self {
                error: CStr::from_ptr(err).to_string_lossy().into_owned(),
            }
        }
    }
}

impl Display for FutharkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.error)
    }
}


impl FutharkContext {
pub fn finalize(&mut self, in0: FutharkOpaque98D86A09, ) -> Result<(Array_u64_2d)>
{
let ctx = self.ptr();
unsafe{
_finalize(ctx, in0.as_raw_mut(), )
}}

pub fn simple11(&mut self, in0: i32, ) -> Result<(Array_u64_2d)>
{
let ctx = self.ptr();
unsafe{
_simple11(ctx, in0, )
}}

pub fn simple8(&mut self, in0: i32, ) -> Result<(Array_u64_2d)>
{
let ctx = self.ptr();
unsafe{
_simple8(ctx, in0, )
}}

pub fn simple2(&mut self, in0: i32, ) -> Result<(Array_u64_2d)>
{
let ctx = self.ptr();
unsafe{
_simple2(ctx, in0, )
}}

pub fn add_columns(&mut self, in0: FutharkOpaque98D86A09, in1: i32, in2: Array_u64_1d, ) -> Result<(FutharkOpaque98D86A09)>
{
let ctx = self.ptr();
unsafe{
_add_columns(ctx, in0.as_raw_mut(), in1, in2.as_raw_mut(), )
}}

pub fn hash2(&mut self, in0: FutharkOpaqueF51E26B8, in1: Array_u64_1d, ) -> Result<(Array_u64_1d)>
{
let ctx = self.ptr();
unsafe{
_hash2(ctx, in0.as_raw_mut(), in1.as_raw_mut(), )
}}

pub fn init(&mut self, in0: Array_u64_1d, in1: Array_u64_2d, in2: Array_u64_3d, in3: Array_u64_3d, in4: Array_u64_3d, in5: Array_u64_1d, in6: Array_u64_2d, in7: Array_u64_3d, in8: Array_u64_3d, in9: Array_u64_3d, ) -> Result<(FutharkOpaque98D86A09)>
{
let ctx = self.ptr();
unsafe{
_init(ctx, in0.as_raw_mut(), in1.as_raw_mut(), in2.as_raw_mut(), in3.as_raw_mut(), in4.as_raw_mut(), in5.as_raw_mut(), in6.as_raw_mut(), in7.as_raw_mut(), in8.as_raw_mut(), in9.as_raw_mut(), )
}}

pub fn init2(&mut self, in0: Array_u64_1d, in1: Array_u64_2d, in2: Array_u64_3d, in3: Array_u64_3d, in4: Array_u64_3d, ) -> Result<(FutharkOpaqueF51E26B8)>
{
let ctx = self.ptr();
unsafe{
_init2(ctx, in0.as_raw_mut(), in1.as_raw_mut(), in2.as_raw_mut(), in3.as_raw_mut(), in4.as_raw_mut(), )
}}

}
unsafe fn _finalize(ctx: *mut bindings::futhark_context, in0: *const bindings::futhark_opaque_98d86a09, ) -> Result<(Array_u64_2d)> {
let mut raw_out0 = std::ptr::null_mut();

if bindings::futhark_entry_finalize(ctx, &mut raw_out0, in0, ) != 0 {
return Err(FutharkError::new(ctx).into());}
Ok(Array_u64_2d::from_ptr(ctx, raw_out0)
)
}
unsafe fn _simple11(ctx: *mut bindings::futhark_context, in0: i32, ) -> Result<(Array_u64_2d)> {
let mut raw_out0 = std::ptr::null_mut();

if bindings::futhark_entry_simple11(ctx, &mut raw_out0, in0, ) != 0 {
return Err(FutharkError::new(ctx).into());}
Ok(Array_u64_2d::from_ptr(ctx, raw_out0)
)
}
unsafe fn _simple8(ctx: *mut bindings::futhark_context, in0: i32, ) -> Result<(Array_u64_2d)> {
let mut raw_out0 = std::ptr::null_mut();

if bindings::futhark_entry_simple8(ctx, &mut raw_out0, in0, ) != 0 {
return Err(FutharkError::new(ctx).into());}
Ok(Array_u64_2d::from_ptr(ctx, raw_out0)
)
}
unsafe fn _simple2(ctx: *mut bindings::futhark_context, in0: i32, ) -> Result<(Array_u64_2d)> {
let mut raw_out0 = std::ptr::null_mut();

if bindings::futhark_entry_simple2(ctx, &mut raw_out0, in0, ) != 0 {
return Err(FutharkError::new(ctx).into());}
Ok(Array_u64_2d::from_ptr(ctx, raw_out0)
)
}
unsafe fn _add_columns(ctx: *mut bindings::futhark_context, in0: *const bindings::futhark_opaque_98d86a09, in1: i32, in2: *const bindings::futhark_u64_1d, ) -> Result<(FutharkOpaque98D86A09)> {
let mut raw_out0 = std::ptr::null_mut();

if bindings::futhark_entry_add_columns(ctx, &mut raw_out0, in0, in1, in2, ) != 0 {
return Err(FutharkError::new(ctx).into());}
Ok(FutharkOpaque98D86A09::from_ptr(ctx, raw_out0)
)
}
unsafe fn _hash2(ctx: *mut bindings::futhark_context, in0: *const bindings::futhark_opaque_f51e26b8, in1: *const bindings::futhark_u64_1d, ) -> Result<(Array_u64_1d)> {
let mut raw_out0 = std::ptr::null_mut();

if bindings::futhark_entry_hash2(ctx, &mut raw_out0, in0, in1, ) != 0 {
return Err(FutharkError::new(ctx).into());}
Ok(Array_u64_1d::from_ptr(ctx, raw_out0)
)
}
unsafe fn _init(ctx: *mut bindings::futhark_context, in0: *const bindings::futhark_u64_1d, in1: *const bindings::futhark_u64_2d, in2: *const bindings::futhark_u64_3d, in3: *const bindings::futhark_u64_3d, in4: *const bindings::futhark_u64_3d, in5: *const bindings::futhark_u64_1d, in6: *const bindings::futhark_u64_2d, in7: *const bindings::futhark_u64_3d, in8: *const bindings::futhark_u64_3d, in9: *const bindings::futhark_u64_3d, ) -> Result<(FutharkOpaque98D86A09)> {
let mut raw_out0 = std::ptr::null_mut();

if bindings::futhark_entry_init(ctx, &mut raw_out0, in0, in1, in2, in3, in4, in5, in6, in7, in8, in9, ) != 0 {
return Err(FutharkError::new(ctx).into());}
Ok(FutharkOpaque98D86A09::from_ptr(ctx, raw_out0)
)
}
unsafe fn _init2(ctx: *mut bindings::futhark_context, in0: *const bindings::futhark_u64_1d, in1: *const bindings::futhark_u64_2d, in2: *const bindings::futhark_u64_3d, in3: *const bindings::futhark_u64_3d, in4: *const bindings::futhark_u64_3d, ) -> Result<(FutharkOpaqueF51E26B8)> {
let mut raw_out0 = std::ptr::null_mut();

if bindings::futhark_entry_init2(ctx, &mut raw_out0, in0, in1, in2, in3, in4, ) != 0 {
return Err(FutharkError::new(ctx).into());}
Ok(FutharkOpaqueF51E26B8::from_ptr(ctx, raw_out0)
)
}
#[derive(Debug)]
pub struct FutharkOpaque98D86A09 {
    ptr: *const bindings::futhark_opaque_98d86a09,
    ctx: *mut bindings::futhark_context,
}

impl FutharkOpaque98D86A09 {
    pub(crate) unsafe fn as_raw(&self) -> *const bindings::futhark_opaque_98d86a09 {
         self.ptr
    }

    pub(crate) unsafe fn as_raw_mut(&self) -> *mut bindings::futhark_opaque_98d86a09 {
         self.ptr as *mut bindings::futhark_opaque_98d86a09
    }
    pub(crate) unsafe fn from_ptr<T>(ctx: T, ptr: *const bindings::futhark_opaque_98d86a09) -> Self
        where
        T: Into<*mut bindings::futhark_context>,
    {
        let ctx = ctx.into();
        Self { ptr, ctx }
    }

    pub(crate) unsafe fn free_opaque(&mut self)
    {
        bindings::futhark_free_opaque_98d86a09(self.ctx, self.as_raw_mut());
    }
}

impl Drop for FutharkOpaque98D86A09 {
    fn drop(&mut self) {
        unsafe {
            self.free_opaque();
        }
    }
}

#[derive(Debug)]
pub struct FutharkOpaqueF51E26B8 {
    ptr: *const bindings::futhark_opaque_f51e26b8,
    ctx: *mut bindings::futhark_context,
}

impl FutharkOpaqueF51E26B8 {
    pub(crate) unsafe fn as_raw(&self) -> *const bindings::futhark_opaque_f51e26b8 {
         self.ptr
    }

    pub(crate) unsafe fn as_raw_mut(&self) -> *mut bindings::futhark_opaque_f51e26b8 {
         self.ptr as *mut bindings::futhark_opaque_f51e26b8
    }
    pub(crate) unsafe fn from_ptr<T>(ctx: T, ptr: *const bindings::futhark_opaque_f51e26b8) -> Self
        where
        T: Into<*mut bindings::futhark_context>,
    {
        let ctx = ctx.into();
        Self { ptr, ctx }
    }

    pub(crate) unsafe fn free_opaque(&mut self)
    {
        bindings::futhark_free_opaque_f51e26b8(self.ctx, self.as_raw_mut());
    }
}

impl Drop for FutharkOpaqueF51E26B8 {
    fn drop(&mut self) {
        unsafe {
            self.free_opaque();
        }
    }
}



