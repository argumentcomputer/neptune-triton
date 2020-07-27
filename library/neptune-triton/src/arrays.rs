use crate::bindings;
use crate::traits::*;
use crate::{Error, Result};

pub(crate) trait FutharkType {
    type RustType: Default;
    const DIM: usize;

    unsafe fn shape(ctx: &crate::context::FutharkContext, ptr: *const Self) -> *const i64;
    unsafe fn values(
        ctx: &crate::context::FutharkContext,
        ptr: *mut Self,
        dst: *mut Self::RustType,
    );
    unsafe fn free(ctx: &crate::context::FutharkContext, ptr: *mut Self);
}

use crate::bindings::*;

impl futhark_i64_1d {
    unsafe fn new(ctx: &crate::context::FutharkContext, arr: &[i64], dim: &[i64]) -> *const Self {
        let ctx = ctx.ptr();
        bindings::futhark_new_i64_1d(ctx, arr.as_ptr() as *mut i64, dim[0])
    }
}

impl FutharkType for futhark_i64_1d {
    type RustType = i64;
    const DIM: usize = 1;

    unsafe fn shape(
        ctx: &crate::context::FutharkContext,
        ptr: *const bindings::futhark_i64_1d,
    ) -> *const i64 {
        let ctx = ctx.ptr();
        bindings::futhark_shape_i64_1d(ctx, ptr as *mut bindings::futhark_i64_1d)
    }
    unsafe fn values(
        ctx: &crate::context::FutharkContext,
        ptr: *mut Self,
        dst: *mut Self::RustType,
    ) {
        let ctx = ctx.ptr();
        bindings::futhark_values_i64_1d(ctx, ptr, dst);
        // Sync the values to the array.
        bindings::futhark_context_sync(ctx);
    }
    unsafe fn free(ctx: &crate::context::FutharkContext, ptr: *mut Self) {
        let ctx = ctx.ptr();
        bindings::futhark_free_i64_1d(ctx, ptr);
    }
}

impl futhark_i64_2d {
    unsafe fn new(ctx: &crate::context::FutharkContext, arr: &[i64], dim: &[i64]) -> *const Self {
        let ctx = ctx.ptr();
        bindings::futhark_new_i64_2d(ctx, arr.as_ptr() as *mut i64, dim[0], dim[1])
    }
}

impl FutharkType for futhark_i64_2d {
    type RustType = i64;
    const DIM: usize = 2;

    unsafe fn shape(
        ctx: &crate::context::FutharkContext,
        ptr: *const bindings::futhark_i64_2d,
    ) -> *const i64 {
        let ctx = ctx.ptr();
        bindings::futhark_shape_i64_2d(ctx, ptr as *mut bindings::futhark_i64_2d)
    }
    unsafe fn values(
        ctx: &crate::context::FutharkContext,
        ptr: *mut Self,
        dst: *mut Self::RustType,
    ) {
        let ctx = ctx.ptr();
        bindings::futhark_values_i64_2d(ctx, ptr, dst);
        // Sync the values to the array.
        bindings::futhark_context_sync(ctx);
    }
    unsafe fn free(ctx: &crate::context::FutharkContext, ptr: *mut Self) {
        let ctx = ctx.ptr();
        bindings::futhark_free_i64_2d(ctx, ptr);
    }
}

impl futhark_u64_1d {
    unsafe fn new(ctx: &crate::context::FutharkContext, arr: &[u64], dim: &[i64]) -> *const Self {
        let ctx = ctx.ptr();
        bindings::futhark_new_u64_1d(ctx, arr.as_ptr() as *mut u64, dim[0])
    }
}

impl FutharkType for futhark_u64_1d {
    type RustType = u64;
    const DIM: usize = 1;

    unsafe fn shape(
        ctx: &crate::context::FutharkContext,
        ptr: *const bindings::futhark_u64_1d,
    ) -> *const i64 {
        let ctx = ctx.ptr();
        bindings::futhark_shape_u64_1d(ctx, ptr as *mut bindings::futhark_u64_1d)
    }
    unsafe fn values(
        ctx: &crate::context::FutharkContext,
        ptr: *mut Self,
        dst: *mut Self::RustType,
    ) {
        let ctx = ctx.ptr();
        bindings::futhark_values_u64_1d(ctx, ptr, dst);
        // Sync the values to the array.
        bindings::futhark_context_sync(ctx);
    }
    unsafe fn free(ctx: &crate::context::FutharkContext, ptr: *mut Self) {
        let ctx = ctx.ptr();
        bindings::futhark_free_u64_1d(ctx, ptr);
    }
}

impl futhark_u64_2d {
    unsafe fn new(ctx: &crate::context::FutharkContext, arr: &[u64], dim: &[i64]) -> *const Self {
        let ctx = ctx.ptr();
        bindings::futhark_new_u64_2d(ctx, arr.as_ptr() as *mut u64, dim[0], dim[1])
    }
}

impl FutharkType for futhark_u64_2d {
    type RustType = u64;
    const DIM: usize = 2;

    unsafe fn shape(
        ctx: &crate::context::FutharkContext,
        ptr: *const bindings::futhark_u64_2d,
    ) -> *const i64 {
        let ctx = ctx.ptr();
        bindings::futhark_shape_u64_2d(ctx, ptr as *mut bindings::futhark_u64_2d)
    }
    unsafe fn values(
        ctx: &crate::context::FutharkContext,
        ptr: *mut Self,
        dst: *mut Self::RustType,
    ) {
        let ctx = ctx.ptr();
        bindings::futhark_values_u64_2d(ctx, ptr, dst);
        // Sync the values to the array.
        bindings::futhark_context_sync(ctx);
    }
    unsafe fn free(ctx: &crate::context::FutharkContext, ptr: *mut Self) {
        let ctx = ctx.ptr();
        bindings::futhark_free_u64_2d(ctx, ptr);
    }
}

impl futhark_u64_3d {
    unsafe fn new(ctx: &crate::context::FutharkContext, arr: &[u64], dim: &[i64]) -> *const Self {
        let ctx = ctx.ptr();
        bindings::futhark_new_u64_3d(ctx, arr.as_ptr() as *mut u64, dim[0], dim[1], dim[2])
    }
}

impl FutharkType for futhark_u64_3d {
    type RustType = u64;
    const DIM: usize = 3;

    unsafe fn shape(
        ctx: &crate::context::FutharkContext,
        ptr: *const bindings::futhark_u64_3d,
    ) -> *const i64 {
        let ctx = ctx.ptr();
        bindings::futhark_shape_u64_3d(ctx, ptr as *mut bindings::futhark_u64_3d)
    }
    unsafe fn values(
        ctx: &crate::context::FutharkContext,
        ptr: *mut Self,
        dst: *mut Self::RustType,
    ) {
        let ctx = ctx.ptr();
        bindings::futhark_values_u64_3d(ctx, ptr, dst);
        // Sync the values to the array.
        bindings::futhark_context_sync(ctx);
    }
    unsafe fn free(ctx: &crate::context::FutharkContext, ptr: *mut Self) {
        let ctx = ctx.ptr();
        bindings::futhark_free_u64_3d(ctx, ptr);
    }
}
#[derive(Debug)]
pub struct Array_i64_1d<'a> {
    ptr: *const futhark_i64_1d,
    ctx: &'a crate::context::FutharkContext,
}

impl<'a> Array_i64_1d<'a> {
    pub(crate) unsafe fn as_raw(&self) -> *const futhark_i64_1d {
        self.ptr
    }

    pub(crate) unsafe fn as_raw_mut(&self) -> *mut futhark_i64_1d {
        self.ptr as *mut futhark_i64_1d
    }
    pub(crate) unsafe fn from_ptr(
        ctx: &'a crate::context::FutharkContext,
        ptr: *const futhark_i64_1d,
    ) -> Self {
        Self { ptr, ctx }
    }

    pub(crate) unsafe fn shape(
        ctx: &crate::context::FutharkContext,
        ptr: *const futhark_i64_1d,
    ) -> Vec<i64> {
        let shape_ptr: *const i64 = futhark_i64_1d::shape(ctx, ptr);
        let shape = std::slice::from_raw_parts(shape_ptr, 1);
        Vec::from(shape)
    }

    pub fn from_vec(
        ctx: &'a crate::context::FutharkContext,
        arr: &[i64],
        dim: &[i64],
    ) -> Result<Self> {
        let expected = (dim.iter().fold(1, |acc, e| acc * e)) as usize;
        if arr.len() != expected {
            return Err(Error::SizeMismatch(arr.len(), expected));
        }

        unsafe {
            let ptr = futhark_i64_1d::new(ctx, arr, dim);
            Ok(Array_i64_1d { ptr, ctx })
        }
    }

    pub fn to_vec(&self) -> (Vec<i64>, Vec<i64>) {
        let ctx = self.ctx;
        unsafe {
            futhark_context_sync(ctx.ptr());
            let shape = Self::shape(ctx, self.as_raw());
            let elems = shape.iter().fold(1, |acc, e| acc * e) as usize;
            let mut buffer: Vec<i64> = vec![i64::default(); elems];
            let cint = futhark_i64_1d::values(ctx, self.as_raw_mut(), buffer.as_mut_ptr());
            (buffer, shape.to_owned())
        }
    }

    pub(crate) unsafe fn free_array(&mut self) {
        futhark_i64_1d::free(self.ctx, self.as_raw_mut());
    }
}

impl Drop for Array_i64_1d<'_> {
    fn drop(&mut self) {
        unsafe {
            self.free_array();
        }
    }
}

#[derive(Debug)]
pub struct Array_i64_2d<'a> {
    ptr: *const futhark_i64_2d,
    ctx: &'a crate::context::FutharkContext,
}

impl<'a> Array_i64_2d<'a> {
    pub(crate) unsafe fn as_raw(&self) -> *const futhark_i64_2d {
        self.ptr
    }

    pub(crate) unsafe fn as_raw_mut(&self) -> *mut futhark_i64_2d {
        self.ptr as *mut futhark_i64_2d
    }
    pub(crate) unsafe fn from_ptr(
        ctx: &'a crate::context::FutharkContext,
        ptr: *const futhark_i64_2d,
    ) -> Self {
        Self { ptr, ctx }
    }

    pub(crate) unsafe fn shape(
        ctx: &crate::context::FutharkContext,
        ptr: *const futhark_i64_2d,
    ) -> Vec<i64> {
        let shape_ptr: *const i64 = futhark_i64_2d::shape(ctx, ptr);
        let shape = std::slice::from_raw_parts(shape_ptr, 2);
        Vec::from(shape)
    }

    pub fn from_vec(
        ctx: &'a crate::context::FutharkContext,
        arr: &[i64],
        dim: &[i64],
    ) -> Result<Self> {
        let expected = (dim.iter().fold(1, |acc, e| acc * e)) as usize;
        if arr.len() != expected {
            return Err(Error::SizeMismatch(arr.len(), expected));
        }

        unsafe {
            let ptr = futhark_i64_2d::new(ctx, arr, dim);
            Ok(Array_i64_2d { ptr, ctx })
        }
    }

    pub fn to_vec(&self) -> (Vec<i64>, Vec<i64>) {
        let ctx = self.ctx;
        unsafe {
            futhark_context_sync(ctx.ptr());
            let shape = Self::shape(ctx, self.as_raw());
            let elems = shape.iter().fold(1, |acc, e| acc * e) as usize;
            let mut buffer: Vec<i64> = vec![i64::default(); elems];
            let cint = futhark_i64_2d::values(ctx, self.as_raw_mut(), buffer.as_mut_ptr());
            (buffer, shape.to_owned())
        }
    }

    pub(crate) unsafe fn free_array(&mut self) {
        futhark_i64_2d::free(self.ctx, self.as_raw_mut());
    }
}

impl Drop for Array_i64_2d<'_> {
    fn drop(&mut self) {
        unsafe {
            self.free_array();
        }
    }
}

#[derive(Debug)]
pub struct Array_u64_1d<'a> {
    ptr: *const futhark_u64_1d,
    ctx: &'a crate::context::FutharkContext,
}

impl<'a> Array_u64_1d<'a> {
    pub(crate) unsafe fn as_raw(&self) -> *const futhark_u64_1d {
        self.ptr
    }

    pub(crate) unsafe fn as_raw_mut(&self) -> *mut futhark_u64_1d {
        self.ptr as *mut futhark_u64_1d
    }
    pub(crate) unsafe fn from_ptr(
        ctx: &'a crate::context::FutharkContext,
        ptr: *const futhark_u64_1d,
    ) -> Self {
        Self { ptr, ctx }
    }

    pub(crate) unsafe fn shape(
        ctx: &crate::context::FutharkContext,
        ptr: *const futhark_u64_1d,
    ) -> Vec<i64> {
        let shape_ptr: *const i64 = futhark_u64_1d::shape(ctx, ptr);
        let shape = std::slice::from_raw_parts(shape_ptr, 1);
        Vec::from(shape)
    }

    pub fn from_vec(
        ctx: &'a crate::context::FutharkContext,
        arr: &[u64],
        dim: &[i64],
    ) -> Result<Self> {
        let expected = (dim.iter().fold(1, |acc, e| acc * e)) as usize;
        if arr.len() != expected {
            return Err(Error::SizeMismatch(arr.len(), expected));
        }

        unsafe {
            let ptr = futhark_u64_1d::new(ctx, arr, dim);
            Ok(Array_u64_1d { ptr, ctx })
        }
    }

    pub fn to_vec(&self) -> (Vec<u64>, Vec<i64>) {
        let ctx = self.ctx;
        unsafe {
            futhark_context_sync(ctx.ptr());
            let shape = Self::shape(ctx, self.as_raw());
            let elems = shape.iter().fold(1, |acc, e| acc * e) as usize;
            let mut buffer: Vec<u64> = vec![u64::default(); elems];
            let cint = futhark_u64_1d::values(ctx, self.as_raw_mut(), buffer.as_mut_ptr());
            (buffer, shape.to_owned())
        }
    }

    pub(crate) unsafe fn free_array(&mut self) {
        futhark_u64_1d::free(self.ctx, self.as_raw_mut());
    }
}

impl Drop for Array_u64_1d<'_> {
    fn drop(&mut self) {
        unsafe {
            self.free_array();
        }
    }
}

#[derive(Debug)]
pub struct Array_u64_2d<'a> {
    ptr: *const futhark_u64_2d,
    ctx: &'a crate::context::FutharkContext,
}

impl<'a> Array_u64_2d<'a> {
    pub(crate) unsafe fn as_raw(&self) -> *const futhark_u64_2d {
        self.ptr
    }

    pub(crate) unsafe fn as_raw_mut(&self) -> *mut futhark_u64_2d {
        self.ptr as *mut futhark_u64_2d
    }
    pub(crate) unsafe fn from_ptr(
        ctx: &'a crate::context::FutharkContext,
        ptr: *const futhark_u64_2d,
    ) -> Self {
        Self { ptr, ctx }
    }

    pub(crate) unsafe fn shape(
        ctx: &crate::context::FutharkContext,
        ptr: *const futhark_u64_2d,
    ) -> Vec<i64> {
        let shape_ptr: *const i64 = futhark_u64_2d::shape(ctx, ptr);
        let shape = std::slice::from_raw_parts(shape_ptr, 2);
        Vec::from(shape)
    }

    pub fn from_vec(
        ctx: &'a crate::context::FutharkContext,
        arr: &[u64],
        dim: &[i64],
    ) -> Result<Self> {
        let expected = (dim.iter().fold(1, |acc, e| acc * e)) as usize;
        if arr.len() != expected {
            return Err(Error::SizeMismatch(arr.len(), expected));
        }

        unsafe {
            let ptr = futhark_u64_2d::new(ctx, arr, dim);
            Ok(Array_u64_2d { ptr, ctx })
        }
    }

    pub fn to_vec(&self) -> (Vec<u64>, Vec<i64>) {
        let ctx = self.ctx;
        unsafe {
            futhark_context_sync(ctx.ptr());
            let shape = Self::shape(ctx, self.as_raw());
            let elems = shape.iter().fold(1, |acc, e| acc * e) as usize;
            let mut buffer: Vec<u64> = vec![u64::default(); elems];
            let cint = futhark_u64_2d::values(ctx, self.as_raw_mut(), buffer.as_mut_ptr());
            (buffer, shape.to_owned())
        }
    }

    pub(crate) unsafe fn free_array(&mut self) {
        futhark_u64_2d::free(self.ctx, self.as_raw_mut());
    }
}

impl Drop for Array_u64_2d<'_> {
    fn drop(&mut self) {
        unsafe {
            self.free_array();
        }
    }
}

#[derive(Debug)]
pub struct Array_u64_3d<'a> {
    ptr: *const futhark_u64_3d,
    ctx: &'a crate::context::FutharkContext,
}

impl<'a> Array_u64_3d<'a> {
    pub(crate) unsafe fn as_raw(&self) -> *const futhark_u64_3d {
        self.ptr
    }

    pub(crate) unsafe fn as_raw_mut(&self) -> *mut futhark_u64_3d {
        self.ptr as *mut futhark_u64_3d
    }
    pub(crate) unsafe fn from_ptr(
        ctx: &'a crate::context::FutharkContext,
        ptr: *const futhark_u64_3d,
    ) -> Self {
        Self { ptr, ctx }
    }

    pub(crate) unsafe fn shape(
        ctx: &crate::context::FutharkContext,
        ptr: *const futhark_u64_3d,
    ) -> Vec<i64> {
        let shape_ptr: *const i64 = futhark_u64_3d::shape(ctx, ptr);
        let shape = std::slice::from_raw_parts(shape_ptr, 3);
        Vec::from(shape)
    }

    pub fn from_vec(
        ctx: &'a crate::context::FutharkContext,
        arr: &[u64],
        dim: &[i64],
    ) -> Result<Self> {
        let expected = (dim.iter().fold(1, |acc, e| acc * e)) as usize;
        if arr.len() != expected {
            return Err(Error::SizeMismatch(arr.len(), expected));
        }

        unsafe {
            let ptr = futhark_u64_3d::new(ctx, arr, dim);
            Ok(Array_u64_3d { ptr, ctx })
        }
    }

    pub fn to_vec(&self) -> (Vec<u64>, Vec<i64>) {
        let ctx = self.ctx;
        unsafe {
            futhark_context_sync(ctx.ptr());
            let shape = Self::shape(ctx, self.as_raw());
            let elems = shape.iter().fold(1, |acc, e| acc * e) as usize;
            let mut buffer: Vec<u64> = vec![u64::default(); elems];
            let cint = futhark_u64_3d::values(ctx, self.as_raw_mut(), buffer.as_mut_ptr());
            (buffer, shape.to_owned())
        }
    }

    pub(crate) unsafe fn free_array(&mut self) {
        futhark_u64_3d::free(self.ctx, self.as_raw_mut());
    }
}

impl Drop for Array_u64_3d<'_> {
    fn drop(&mut self) {
        unsafe {
            self.free_array();
        }
    }
}
