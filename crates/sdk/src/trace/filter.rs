use super::TraceKind;

#[repr(C)]
struct VTable<F>
where
    F: super::Filter,
{
    should_hit: unsafe extern "C" fn(this: *const Filter<F>, entity: *const u8, mask: i32) -> bool,
    trace_kind: unsafe extern "C" fn(this: *const Filter<F>) -> TraceKind,
}

#[repr(C)]
pub struct Filter<F>
where
    F: super::Filter,
{
    // FIXME: `&'static` causes `F` to require `'static`.
    vtable: *const VTable<F>,
    filter: F,
}

impl<F> Filter<F>
where
    F: super::Filter,
{
    pub const fn new(filter: F) -> Self {
        Self {
            vtable: &VTable {
                should_hit,
                trace_kind,
            },
            filter,
        }
    }

    pub const fn as_ptr(&self) -> *const u8 {
        self as *const Self as *const u8
    }
}

// error[E0570]: `"thiscall"` is not a supported ABI for the current target
unsafe extern "C" fn should_hit<F>(this: *const Filter<F>, entity: *const u8, mask: i32) -> bool
where
    F: super::Filter,
{
    (*this).filter.should_hit(entity, mask)
}

// error[E0570]: `"thiscall"` is not a supported ABI for the current target
unsafe extern "C" fn trace_kind<F>(this: *const Filter<F>) -> TraceKind
where
    F: super::Filter,
{
    (*this).filter.trace_kind()
}
