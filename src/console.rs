use crate::managed::{handle, Managed};
use std::borrow::Cow;
use std::ffi::CStr;

pub use self::var::{Kind, Var};

mod var;

/// The console interface.
#[derive(Debug)]
#[repr(transparent)]
pub struct Console(Managed<handle::Console>);

impl Console {
    pub fn new(ptr: *mut handle::Console) -> Option<Self> {
        Some(Self(Managed::new(ptr)?))
    }

    pub unsafe fn new_unchecked(ptr: *mut handle::Console) -> Self {
        Self(Managed::new_unchecked(ptr))
    }

    pub fn as_ptr(&self) -> *const handle::Console {
        self.0.as_ptr()
    }

    /// Returns a pointer to the first element within the virtual table.
    pub unsafe fn virtual_table(&self) -> *const () {
        self.0.virtual_table()
    }

    /// Returns a pointer to the object at `offset` in the virtual table.
    pub unsafe fn virtual_offset(&self, offset: usize) -> *const () {
        self.0.virtual_offset(offset)
    }

    /// Returns the object at `offset` as a function signature.
    pub unsafe fn virtual_entry<U>(&self, offset: usize) -> U
    where
        U: Sized,
    {
        self.0.virtual_entry(offset)
    }

    /// Returns a pointer to the object at `offset` (in bytes).
    pub unsafe fn relative_offset(&self, offset: usize) -> *const () {
        self.0.relative_offset(offset)
    }

    /// Returns an object at `offset` (in bytes).
    pub unsafe fn relative_entry<U>(&self, offset: usize) -> U
    where
        U: Sized,
    {
        self.0.relative_entry(offset)
    }

    pub fn var<'a, T, V>(&self, var: V) -> Option<Var<T>>
    where
        T: Kind,
        V: Into<Cow<'a, CStr>>,
    {
        type Fn = unsafe extern "C" fn(
            this: *const handle::Console,
            var: *const i8,
        ) -> *mut handle::ConsoleVar;

        unsafe {
            let ptr = self.virtual_entry::<Fn>(15)(self.as_ptr(), var.into().as_ptr());

            Var::new(ptr)
        }
    }

    pub fn write<'a, S>(&self, string: S)
    where
        S: Into<Cow<'a, CStr>>,
    {
        type Fn =
            unsafe extern "C" fn(this: *const handle::Console, fmt: *const i8, txt: *const i8);

        unsafe {
            self.virtual_entry::<Fn>(27)(
                self.as_ptr(),
                b"%s\0".as_ptr().cast(),
                string.into().as_ptr(),
            );
        }
    }
}
