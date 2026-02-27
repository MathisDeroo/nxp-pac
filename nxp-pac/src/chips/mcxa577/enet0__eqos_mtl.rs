#[doc = "no description available"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enet0_eqosMtl {
    ptr: *mut u8,
}
unsafe impl Send for Enet0_eqosMtl {}
unsafe impl Sync for Enet0_eqosMtl {}
impl Enet0_eqosMtl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Operation Mode"]
    #[inline(always)]
    pub const fn mtl_operation_mode(
        self,
    ) -> crate::common::Reg<regs::MtlOperationMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub const fn mtl_interrupt_status(
        self,
    ) -> crate::common::Reg<regs::MtlInterruptStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
}
pub mod regs;
