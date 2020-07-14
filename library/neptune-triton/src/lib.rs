#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(unused_imports)]

mod arrays;
pub mod bindings;
mod context;
mod traits;

use std::ffi::CStr;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::os::raw::c_char;
use std::result::Result as StdResult;

pub use crate::arrays::*;
use crate::traits::*;
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
        unsafe { Self::_new(bindings::futhark_context_get_error(ctx)) }
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
    pub fn build_tree8_64m(
        &mut self,
        in0: &FutharkOpaqueT864MState,
        in1: Array_u64_1d,
    ) -> Result<(Array_u64_2d)> {
        let ctx = self.ptr();
        unsafe { _build_tree8_64m(ctx, in0.as_raw_mut(), in1.as_raw_mut()) }
    }

    pub fn hash8(
        &mut self,
        in0: &FutharkOpaqueP8State,
        in1: Array_u64_1d,
    ) -> Result<(Array_u64_1d, FutharkOpaqueP8State)> {
        let ctx = self.ptr();
        unsafe { _hash8(ctx, in0.as_raw_mut(), in1.as_raw_mut()) }
    }

    pub fn init11(
        &mut self,
        in0: Array_u64_1d,
        in1: Array_u64_2d,
        in2: Array_u64_3d,
        in3: Array_u64_3d,
        in4: Array_u64_3d,
    ) -> Result<(FutharkOpaqueP11State)> {
        let ctx = self.ptr();
        unsafe {
            _init11(
                ctx,
                in0.as_raw_mut(),
                in1.as_raw_mut(),
                in2.as_raw_mut(),
                in3.as_raw_mut(),
                in4.as_raw_mut(),
            )
        }
    }

    pub fn init11s(
        &mut self,
        in0: Array_u64_1d,
        in1: Array_u64_2d,
        in2: Array_u64_3d,
        in3: Array_u64_3d,
        in4: Array_u64_3d,
    ) -> Result<(FutharkOpaqueS11State)> {
        let ctx = self.ptr();
        unsafe {
            _init11s(
                ctx,
                in0.as_raw_mut(),
                in1.as_raw_mut(),
                in2.as_raw_mut(),
                in3.as_raw_mut(),
                in4.as_raw_mut(),
            )
        }
    }

    pub fn init2(
        &mut self,
        in0: Array_u64_1d,
        in1: Array_u64_2d,
        in2: Array_u64_3d,
        in3: Array_u64_3d,
        in4: Array_u64_3d,
    ) -> Result<(FutharkOpaqueP2State)> {
        let ctx = self.ptr();
        unsafe {
            _init2(
                ctx,
                in0.as_raw_mut(),
                in1.as_raw_mut(),
                in2.as_raw_mut(),
                in3.as_raw_mut(),
                in4.as_raw_mut(),
            )
        }
    }

    pub fn init2s(
        &mut self,
        in0: Array_u64_1d,
        in1: Array_u64_2d,
        in2: Array_u64_3d,
        in3: Array_u64_3d,
        in4: Array_u64_3d,
    ) -> Result<(FutharkOpaqueS2State)> {
        let ctx = self.ptr();
        unsafe {
            _init2s(
                ctx,
                in0.as_raw_mut(),
                in1.as_raw_mut(),
                in2.as_raw_mut(),
                in3.as_raw_mut(),
                in4.as_raw_mut(),
            )
        }
    }

    pub fn init8(
        &mut self,
        in0: Array_u64_1d,
        in1: Array_u64_2d,
        in2: Array_u64_3d,
        in3: Array_u64_3d,
        in4: Array_u64_3d,
    ) -> Result<(FutharkOpaqueP8State)> {
        let ctx = self.ptr();
        unsafe {
            _init8(
                ctx,
                in0.as_raw_mut(),
                in1.as_raw_mut(),
                in2.as_raw_mut(),
                in3.as_raw_mut(),
                in4.as_raw_mut(),
            )
        }
    }

    pub fn init8s(
        &mut self,
        in0: Array_u64_1d,
        in1: Array_u64_2d,
        in2: Array_u64_3d,
        in3: Array_u64_3d,
        in4: Array_u64_3d,
    ) -> Result<(FutharkOpaqueS8State)> {
        let ctx = self.ptr();
        unsafe {
            _init8s(
                ctx,
                in0.as_raw_mut(),
                in1.as_raw_mut(),
                in2.as_raw_mut(),
                in3.as_raw_mut(),
                in4.as_raw_mut(),
            )
        }
    }

    pub fn init_t8_64m(
        &mut self,
        in0: Array_u64_1d,
        in1: Array_u64_2d,
        in2: Array_u64_3d,
        in3: Array_u64_3d,
        in4: Array_u64_3d,
    ) -> Result<(FutharkOpaqueT864MState)> {
        let ctx = self.ptr();
        unsafe {
            _init_t8_64m(
                ctx,
                in0.as_raw_mut(),
                in1.as_raw_mut(),
                in2.as_raw_mut(),
                in3.as_raw_mut(),
                in4.as_raw_mut(),
            )
        }
    }

    pub fn mbatch_hash11(
        &mut self,
        in0: &FutharkOpaqueP11State,
        in1: Array_u64_1d,
    ) -> Result<(Array_u64_2d, FutharkOpaqueP11State)> {
        let ctx = self.ptr();
        unsafe { _mbatch_hash11(ctx, in0.as_raw_mut(), in1.as_raw_mut()) }
    }

    pub fn mbatch_hash11s(
        &mut self,
        in0: &FutharkOpaqueS11State,
        in1: Array_u64_1d,
    ) -> Result<(Array_u64_2d, FutharkOpaqueS11State)> {
        let ctx = self.ptr();
        unsafe { _mbatch_hash11s(ctx, in0.as_raw_mut(), in1.as_raw_mut()) }
    }

    pub fn mbatch_hash2(
        &mut self,
        in0: &FutharkOpaqueP2State,
        in1: Array_u64_1d,
    ) -> Result<(Array_u64_2d, FutharkOpaqueP2State)> {
        let ctx = self.ptr();
        unsafe { _mbatch_hash2(ctx, in0.as_raw_mut(), in1.as_raw_mut()) }
    }

    pub fn mbatch_hash2s(
        &mut self,
        in0: &FutharkOpaqueS2State,
        in1: Array_u64_1d,
    ) -> Result<(Array_u64_2d, FutharkOpaqueS2State)> {
        let ctx = self.ptr();
        unsafe { _mbatch_hash2s(ctx, in0.as_raw_mut(), in1.as_raw_mut()) }
    }

    pub fn mbatch_hash8(
        &mut self,
        in0: &FutharkOpaqueP8State,
        in1: Array_u64_1d,
    ) -> Result<(Array_u64_2d, FutharkOpaqueP8State)> {
        let ctx = self.ptr();
        unsafe { _mbatch_hash8(ctx, in0.as_raw_mut(), in1.as_raw_mut()) }
    }

    pub fn mbatch_hash8s(
        &mut self,
        in0: &FutharkOpaqueS8State,
        in1: Array_u64_1d,
    ) -> Result<(Array_u64_2d, FutharkOpaqueS8State)> {
        let ctx = self.ptr();
        unsafe { _mbatch_hash8s(ctx, in0.as_raw_mut(), in1.as_raw_mut()) }
    }

    pub fn simple8(&mut self, in0: i32) -> Result<(Array_u64_2d)> {
        let ctx = self.ptr();
        unsafe { _simple8(ctx, in0) }
    }
}
unsafe fn _build_tree8_64m(
    ctx: *mut bindings::futhark_context,
    in0: *const bindings::futhark_opaque_t8_64m_state,
    in1: *const bindings::futhark_u64_1d,
) -> Result<(Array_u64_2d)> {
    let mut raw_out0 = std::ptr::null_mut();

    if bindings::futhark_entry_build_tree8_64m(ctx, &mut raw_out0, in0, in1) != 0 {
        return Err(FutharkError::new(ctx).into());
    }
    Ok((Array_u64_2d::from_ptr(ctx, raw_out0)))
}
unsafe fn _hash8(
    ctx: *mut bindings::futhark_context,
    in0: *const bindings::futhark_opaque_p8_state,
    in1: *const bindings::futhark_u64_1d,
) -> Result<(Array_u64_1d, FutharkOpaqueP8State)> {
    let mut raw_out0 = std::ptr::null_mut();
    let mut raw_out1 = std::ptr::null_mut();

    if bindings::futhark_entry_hash8(ctx, &mut raw_out0, &mut raw_out1, in0, in1) != 0 {
        return Err(FutharkError::new(ctx).into());
    }
    Ok((
        Array_u64_1d::from_ptr(ctx, raw_out0),
        FutharkOpaqueP8State::from_ptr(ctx, raw_out1),
    ))
}
unsafe fn _init11(
    ctx: *mut bindings::futhark_context,
    in0: *const bindings::futhark_u64_1d,
    in1: *const bindings::futhark_u64_2d,
    in2: *const bindings::futhark_u64_3d,
    in3: *const bindings::futhark_u64_3d,
    in4: *const bindings::futhark_u64_3d,
) -> Result<(FutharkOpaqueP11State)> {
    let mut raw_out0 = std::ptr::null_mut();

    if bindings::futhark_entry_init11(ctx, &mut raw_out0, in0, in1, in2, in3, in4) != 0 {
        return Err(FutharkError::new(ctx).into());
    }
    Ok((FutharkOpaqueP11State::from_ptr(ctx, raw_out0)))
}
unsafe fn _init11s(
    ctx: *mut bindings::futhark_context,
    in0: *const bindings::futhark_u64_1d,
    in1: *const bindings::futhark_u64_2d,
    in2: *const bindings::futhark_u64_3d,
    in3: *const bindings::futhark_u64_3d,
    in4: *const bindings::futhark_u64_3d,
) -> Result<(FutharkOpaqueS11State)> {
    let mut raw_out0 = std::ptr::null_mut();

    if bindings::futhark_entry_init11s(ctx, &mut raw_out0, in0, in1, in2, in3, in4) != 0 {
        return Err(FutharkError::new(ctx).into());
    }
    Ok((FutharkOpaqueS11State::from_ptr(ctx, raw_out0)))
}
unsafe fn _init2(
    ctx: *mut bindings::futhark_context,
    in0: *const bindings::futhark_u64_1d,
    in1: *const bindings::futhark_u64_2d,
    in2: *const bindings::futhark_u64_3d,
    in3: *const bindings::futhark_u64_3d,
    in4: *const bindings::futhark_u64_3d,
) -> Result<(FutharkOpaqueP2State)> {
    let mut raw_out0 = std::ptr::null_mut();

    if bindings::futhark_entry_init2(ctx, &mut raw_out0, in0, in1, in2, in3, in4) != 0 {
        return Err(FutharkError::new(ctx).into());
    }
    Ok((FutharkOpaqueP2State::from_ptr(ctx, raw_out0)))
}
unsafe fn _init2s(
    ctx: *mut bindings::futhark_context,
    in0: *const bindings::futhark_u64_1d,
    in1: *const bindings::futhark_u64_2d,
    in2: *const bindings::futhark_u64_3d,
    in3: *const bindings::futhark_u64_3d,
    in4: *const bindings::futhark_u64_3d,
) -> Result<(FutharkOpaqueS2State)> {
    let mut raw_out0 = std::ptr::null_mut();

    if bindings::futhark_entry_init2s(ctx, &mut raw_out0, in0, in1, in2, in3, in4) != 0 {
        return Err(FutharkError::new(ctx).into());
    }
    Ok((FutharkOpaqueS2State::from_ptr(ctx, raw_out0)))
}
unsafe fn _init8(
    ctx: *mut bindings::futhark_context,
    in0: *const bindings::futhark_u64_1d,
    in1: *const bindings::futhark_u64_2d,
    in2: *const bindings::futhark_u64_3d,
    in3: *const bindings::futhark_u64_3d,
    in4: *const bindings::futhark_u64_3d,
) -> Result<(FutharkOpaqueP8State)> {
    let mut raw_out0 = std::ptr::null_mut();

    if bindings::futhark_entry_init8(ctx, &mut raw_out0, in0, in1, in2, in3, in4) != 0 {
        return Err(FutharkError::new(ctx).into());
    }
    Ok((FutharkOpaqueP8State::from_ptr(ctx, raw_out0)))
}
unsafe fn _init8s(
    ctx: *mut bindings::futhark_context,
    in0: *const bindings::futhark_u64_1d,
    in1: *const bindings::futhark_u64_2d,
    in2: *const bindings::futhark_u64_3d,
    in3: *const bindings::futhark_u64_3d,
    in4: *const bindings::futhark_u64_3d,
) -> Result<(FutharkOpaqueS8State)> {
    let mut raw_out0 = std::ptr::null_mut();

    if bindings::futhark_entry_init8s(ctx, &mut raw_out0, in0, in1, in2, in3, in4) != 0 {
        return Err(FutharkError::new(ctx).into());
    }
    Ok((FutharkOpaqueS8State::from_ptr(ctx, raw_out0)))
}
unsafe fn _init_t8_64m(
    ctx: *mut bindings::futhark_context,
    in0: *const bindings::futhark_u64_1d,
    in1: *const bindings::futhark_u64_2d,
    in2: *const bindings::futhark_u64_3d,
    in3: *const bindings::futhark_u64_3d,
    in4: *const bindings::futhark_u64_3d,
) -> Result<(FutharkOpaqueT864MState)> {
    let mut raw_out0 = std::ptr::null_mut();

    if bindings::futhark_entry_init_t8_64m(ctx, &mut raw_out0, in0, in1, in2, in3, in4) != 0 {
        return Err(FutharkError::new(ctx).into());
    }
    Ok((FutharkOpaqueT864MState::from_ptr(ctx, raw_out0)))
}
unsafe fn _mbatch_hash11(
    ctx: *mut bindings::futhark_context,
    in0: *const bindings::futhark_opaque_p11_state,
    in1: *const bindings::futhark_u64_1d,
) -> Result<(Array_u64_2d, FutharkOpaqueP11State)> {
    let mut raw_out0 = std::ptr::null_mut();
    let mut raw_out1 = std::ptr::null_mut();

    if bindings::futhark_entry_mbatch_hash11(ctx, &mut raw_out0, &mut raw_out1, in0, in1) != 0 {
        return Err(FutharkError::new(ctx).into());
    }
    Ok((
        Array_u64_2d::from_ptr(ctx, raw_out0),
        FutharkOpaqueP11State::from_ptr(ctx, raw_out1),
    ))
}
unsafe fn _mbatch_hash11s(
    ctx: *mut bindings::futhark_context,
    in0: *const bindings::futhark_opaque_s11_state,
    in1: *const bindings::futhark_u64_1d,
) -> Result<(Array_u64_2d, FutharkOpaqueS11State)> {
    let mut raw_out0 = std::ptr::null_mut();
    let mut raw_out1 = std::ptr::null_mut();

    if bindings::futhark_entry_mbatch_hash11s(ctx, &mut raw_out0, &mut raw_out1, in0, in1) != 0 {
        return Err(FutharkError::new(ctx).into());
    }
    Ok((
        Array_u64_2d::from_ptr(ctx, raw_out0),
        FutharkOpaqueS11State::from_ptr(ctx, raw_out1),
    ))
}
unsafe fn _mbatch_hash2(
    ctx: *mut bindings::futhark_context,
    in0: *const bindings::futhark_opaque_p2_state,
    in1: *const bindings::futhark_u64_1d,
) -> Result<(Array_u64_2d, FutharkOpaqueP2State)> {
    let mut raw_out0 = std::ptr::null_mut();
    let mut raw_out1 = std::ptr::null_mut();

    if bindings::futhark_entry_mbatch_hash2(ctx, &mut raw_out0, &mut raw_out1, in0, in1) != 0 {
        return Err(FutharkError::new(ctx).into());
    }
    Ok((
        Array_u64_2d::from_ptr(ctx, raw_out0),
        FutharkOpaqueP2State::from_ptr(ctx, raw_out1),
    ))
}
unsafe fn _mbatch_hash2s(
    ctx: *mut bindings::futhark_context,
    in0: *const bindings::futhark_opaque_s2_state,
    in1: *const bindings::futhark_u64_1d,
) -> Result<(Array_u64_2d, FutharkOpaqueS2State)> {
    let mut raw_out0 = std::ptr::null_mut();
    let mut raw_out1 = std::ptr::null_mut();

    if bindings::futhark_entry_mbatch_hash2s(ctx, &mut raw_out0, &mut raw_out1, in0, in1) != 0 {
        return Err(FutharkError::new(ctx).into());
    }
    Ok((
        Array_u64_2d::from_ptr(ctx, raw_out0),
        FutharkOpaqueS2State::from_ptr(ctx, raw_out1),
    ))
}
unsafe fn _mbatch_hash8(
    ctx: *mut bindings::futhark_context,
    in0: *const bindings::futhark_opaque_p8_state,
    in1: *const bindings::futhark_u64_1d,
) -> Result<(Array_u64_2d, FutharkOpaqueP8State)> {
    let mut raw_out0 = std::ptr::null_mut();
    let mut raw_out1 = std::ptr::null_mut();

    if bindings::futhark_entry_mbatch_hash8(ctx, &mut raw_out0, &mut raw_out1, in0, in1) != 0 {
        return Err(FutharkError::new(ctx).into());
    }
    Ok((
        Array_u64_2d::from_ptr(ctx, raw_out0),
        FutharkOpaqueP8State::from_ptr(ctx, raw_out1),
    ))
}
unsafe fn _mbatch_hash8s(
    ctx: *mut bindings::futhark_context,
    in0: *const bindings::futhark_opaque_s8_state,
    in1: *const bindings::futhark_u64_1d,
) -> Result<(Array_u64_2d, FutharkOpaqueS8State)> {
    let mut raw_out0 = std::ptr::null_mut();
    let mut raw_out1 = std::ptr::null_mut();

    if bindings::futhark_entry_mbatch_hash8s(ctx, &mut raw_out0, &mut raw_out1, in0, in1) != 0 {
        return Err(FutharkError::new(ctx).into());
    }
    Ok((
        Array_u64_2d::from_ptr(ctx, raw_out0),
        FutharkOpaqueS8State::from_ptr(ctx, raw_out1),
    ))
}
unsafe fn _simple8(ctx: *mut bindings::futhark_context, in0: i32) -> Result<(Array_u64_2d)> {
    let mut raw_out0 = std::ptr::null_mut();

    if bindings::futhark_entry_simple8(ctx, &mut raw_out0, in0) != 0 {
        return Err(FutharkError::new(ctx).into());
    }
    Ok((Array_u64_2d::from_ptr(ctx, raw_out0)))
}
#[derive(Debug)]
pub struct FutharkOpaqueP11State {
    ptr: *const bindings::futhark_opaque_p11_state,
    ctx: *mut bindings::futhark_context,
}

impl FutharkOpaqueP11State {
    pub(crate) unsafe fn as_raw(&self) -> *const bindings::futhark_opaque_p11_state {
        self.ptr
    }

    pub(crate) unsafe fn as_raw_mut(&self) -> *mut bindings::futhark_opaque_p11_state {
        self.ptr as *mut bindings::futhark_opaque_p11_state
    }
    pub(crate) unsafe fn from_ptr<T>(ctx: T, ptr: *const bindings::futhark_opaque_p11_state) -> Self
    where
        T: Into<*mut bindings::futhark_context>,
    {
        let ctx = ctx.into();
        Self { ptr, ctx }
    }

    pub(crate) unsafe fn free_opaque(&mut self) {
        bindings::futhark_free_opaque_p11_state(self.ctx, self.as_raw_mut());
    }
}

impl Drop for FutharkOpaqueP11State {
    fn drop(&mut self) {
        unsafe {
            self.free_opaque();
        }
    }
}

#[derive(Debug)]
pub struct FutharkOpaqueP2State {
    ptr: *const bindings::futhark_opaque_p2_state,
    ctx: *mut bindings::futhark_context,
}

impl FutharkOpaqueP2State {
    pub(crate) unsafe fn as_raw(&self) -> *const bindings::futhark_opaque_p2_state {
        self.ptr
    }

    pub(crate) unsafe fn as_raw_mut(&self) -> *mut bindings::futhark_opaque_p2_state {
        self.ptr as *mut bindings::futhark_opaque_p2_state
    }
    pub(crate) unsafe fn from_ptr<T>(ctx: T, ptr: *const bindings::futhark_opaque_p2_state) -> Self
    where
        T: Into<*mut bindings::futhark_context>,
    {
        let ctx = ctx.into();
        Self { ptr, ctx }
    }

    pub(crate) unsafe fn free_opaque(&mut self) {
        bindings::futhark_free_opaque_p2_state(self.ctx, self.as_raw_mut());
    }
}

impl Drop for FutharkOpaqueP2State {
    fn drop(&mut self) {
        unsafe {
            self.free_opaque();
        }
    }
}

#[derive(Debug)]
pub struct FutharkOpaqueP8State {
    ptr: *const bindings::futhark_opaque_p8_state,
    ctx: *mut bindings::futhark_context,
}

impl FutharkOpaqueP8State {
    pub(crate) unsafe fn as_raw(&self) -> *const bindings::futhark_opaque_p8_state {
        self.ptr
    }

    pub(crate) unsafe fn as_raw_mut(&self) -> *mut bindings::futhark_opaque_p8_state {
        self.ptr as *mut bindings::futhark_opaque_p8_state
    }
    pub(crate) unsafe fn from_ptr<T>(ctx: T, ptr: *const bindings::futhark_opaque_p8_state) -> Self
    where
        T: Into<*mut bindings::futhark_context>,
    {
        let ctx = ctx.into();
        Self { ptr, ctx }
    }

    pub(crate) unsafe fn free_opaque(&mut self) {
        bindings::futhark_free_opaque_p8_state(self.ctx, self.as_raw_mut());
    }
}

impl Drop for FutharkOpaqueP8State {
    fn drop(&mut self) {
        unsafe {
            self.free_opaque();
        }
    }
}

#[derive(Debug)]
pub struct FutharkOpaqueS11State {
    ptr: *const bindings::futhark_opaque_s11_state,
    ctx: *mut bindings::futhark_context,
}

impl FutharkOpaqueS11State {
    pub(crate) unsafe fn as_raw(&self) -> *const bindings::futhark_opaque_s11_state {
        self.ptr
    }

    pub(crate) unsafe fn as_raw_mut(&self) -> *mut bindings::futhark_opaque_s11_state {
        self.ptr as *mut bindings::futhark_opaque_s11_state
    }
    pub(crate) unsafe fn from_ptr<T>(ctx: T, ptr: *const bindings::futhark_opaque_s11_state) -> Self
    where
        T: Into<*mut bindings::futhark_context>,
    {
        let ctx = ctx.into();
        Self { ptr, ctx }
    }

    pub(crate) unsafe fn free_opaque(&mut self) {
        bindings::futhark_free_opaque_s11_state(self.ctx, self.as_raw_mut());
    }
}

impl Drop for FutharkOpaqueS11State {
    fn drop(&mut self) {
        unsafe {
            self.free_opaque();
        }
    }
}

#[derive(Debug)]
pub struct FutharkOpaqueS2State {
    ptr: *const bindings::futhark_opaque_s2_state,
    ctx: *mut bindings::futhark_context,
}

impl FutharkOpaqueS2State {
    pub(crate) unsafe fn as_raw(&self) -> *const bindings::futhark_opaque_s2_state {
        self.ptr
    }

    pub(crate) unsafe fn as_raw_mut(&self) -> *mut bindings::futhark_opaque_s2_state {
        self.ptr as *mut bindings::futhark_opaque_s2_state
    }
    pub(crate) unsafe fn from_ptr<T>(ctx: T, ptr: *const bindings::futhark_opaque_s2_state) -> Self
    where
        T: Into<*mut bindings::futhark_context>,
    {
        let ctx = ctx.into();
        Self { ptr, ctx }
    }

    pub(crate) unsafe fn free_opaque(&mut self) {
        bindings::futhark_free_opaque_s2_state(self.ctx, self.as_raw_mut());
    }
}

impl Drop for FutharkOpaqueS2State {
    fn drop(&mut self) {
        unsafe {
            self.free_opaque();
        }
    }
}

#[derive(Debug)]
pub struct FutharkOpaqueS8State {
    ptr: *const bindings::futhark_opaque_s8_state,
    ctx: *mut bindings::futhark_context,
}

impl FutharkOpaqueS8State {
    pub(crate) unsafe fn as_raw(&self) -> *const bindings::futhark_opaque_s8_state {
        self.ptr
    }

    pub(crate) unsafe fn as_raw_mut(&self) -> *mut bindings::futhark_opaque_s8_state {
        self.ptr as *mut bindings::futhark_opaque_s8_state
    }
    pub(crate) unsafe fn from_ptr<T>(ctx: T, ptr: *const bindings::futhark_opaque_s8_state) -> Self
    where
        T: Into<*mut bindings::futhark_context>,
    {
        let ctx = ctx.into();
        Self { ptr, ctx }
    }

    pub(crate) unsafe fn free_opaque(&mut self) {
        bindings::futhark_free_opaque_s8_state(self.ctx, self.as_raw_mut());
    }
}

impl Drop for FutharkOpaqueS8State {
    fn drop(&mut self) {
        unsafe {
            self.free_opaque();
        }
    }
}

#[derive(Debug)]
pub struct FutharkOpaqueT864MState {
    ptr: *const bindings::futhark_opaque_t8_64m_state,
    ctx: *mut bindings::futhark_context,
}

impl FutharkOpaqueT864MState {
    pub(crate) unsafe fn as_raw(&self) -> *const bindings::futhark_opaque_t8_64m_state {
        self.ptr
    }

    pub(crate) unsafe fn as_raw_mut(&self) -> *mut bindings::futhark_opaque_t8_64m_state {
        self.ptr as *mut bindings::futhark_opaque_t8_64m_state
    }
    pub(crate) unsafe fn from_ptr<T>(
        ctx: T,
        ptr: *const bindings::futhark_opaque_t8_64m_state,
    ) -> Self
    where
        T: Into<*mut bindings::futhark_context>,
    {
        let ctx = ctx.into();
        Self { ptr, ctx }
    }

    pub(crate) unsafe fn free_opaque(&mut self) {
        bindings::futhark_free_opaque_t8_64m_state(self.ctx, self.as_raw_mut());
    }
}

impl Drop for FutharkOpaqueT864MState {
    fn drop(&mut self) {
        unsafe {
            self.free_opaque();
        }
    }
}
