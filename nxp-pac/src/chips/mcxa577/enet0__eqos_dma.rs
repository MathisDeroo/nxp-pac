#[doc = "no description available"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enet0_eqosDma {
    ptr: *mut u8,
}
unsafe impl Send for Enet0_eqosDma {}
unsafe impl Sync for Enet0_eqosDma {}
impl Enet0_eqosDma {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Bus Mode"]
    #[inline(always)]
    pub const fn dma_mode(self) -> crate::common::Reg<regs::DmaMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "System Bus Mode"]
    #[inline(always)]
    pub const fn dma_sys_bus_mode(
        self,
    ) -> crate::common::Reg<regs::DmaSysBusMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub const fn dma_interrupt_status(
        self,
    ) -> crate::common::Reg<regs::DmaInterruptStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Debug Status 0"]
    #[inline(always)]
    pub const fn dma_debug_status0(
        self,
    ) -> crate::common::Reg<regs::DmaDebugStatus0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
