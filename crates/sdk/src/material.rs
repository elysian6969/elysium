use crate::{ffi, vtable_validate, Vdf};
use cake::ffi::vtable;
use core::ptr;
use std::ffi::OsStr;

pub use flag::MaterialFlag;
pub use kind::MaterialKind;
pub use material::Material;
pub use var::Var;

mod flag;
mod kind;
mod material;
mod var;

#[repr(C)]
struct VTable {
    _pad0: vtable::Pad<83>,
    create: unsafe extern "thiscall" fn(
        this: *const MaterialSystem,
        name: *const u8,
        vdf: *const Vdf,
    ) -> *const u8,
    find: unsafe extern "thiscall" fn(
        this: *const MaterialSystem,
        name: *const u8,
        texture_group: *const u8,
        complain: bool,
        complain_prefix: *const u8,
    ) -> *const u8,
}

vtable_validate! {
    create => 83,
    find => 84,
}

#[repr(C)]
pub struct MaterialSystem {
    vtable: &'static VTable,
}

impl MaterialSystem {
    #[inline]
    pub fn create<S>(&self, name: S, vdf: &Vdf) -> *const u8
    where
        S: AsRef<OsStr>,
    {
        let cstr = ffi::osstr_to_cstr_cow(name);
        let ptr = ffi::cstr_cow_as_ptr(cstr.as_ref());

        unsafe { (self.vtable.create)(self, ptr, vdf) }
    }

    #[inline]
    pub fn find<S, T>(&self, name: &str, texture_group: &str) -> *const u8
    where
        S: AsRef<OsStr>,
        T: AsRef<OsStr>,
    {
        let cstr = ffi::osstr_to_cstr_cow(name);
        let name = ffi::cstr_cow_as_ptr(cstr.as_ref());

        let cstr = ffi::osstr_to_cstr_cow(texture_group);
        let texture_group = ffi::cstr_cow_as_ptr(cstr.as_ref());

        unsafe { (self.vtable.find)(self, name, texture_group, true, ptr::null()) }
    }
}
