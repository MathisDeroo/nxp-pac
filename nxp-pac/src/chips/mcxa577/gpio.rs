#[doc = "GPIO"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version ID"]
    #[inline(always)]
    pub const fn verid(self) -> crate::common::Reg<regs::Verid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Parameter"]
    #[inline(always)]
    pub const fn param(self) -> crate::common::Reg<regs::Param, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Pin Control Nonsecure"]
    #[inline(always)]
    pub const fn pcns(self) -> crate::common::Reg<regs::Pcns, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Interrupt Control Nonsecure"]
    #[inline(always)]
    pub const fn icns(self) -> crate::common::Reg<regs::Icns, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Pin Control Nonprivilege"]
    #[inline(always)]
    pub const fn pcnp(self) -> crate::common::Reg<regs::Pcnp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Interrupt Control Nonprivilege"]
    #[inline(always)]
    pub const fn icnp(self) -> crate::common::Reg<regs::Icnp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Port Data Output"]
    #[inline(always)]
    pub const fn pdor(self) -> crate::common::Reg<regs::Pdor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Port Set Output"]
    #[inline(always)]
    pub const fn psor(self) -> crate::common::Reg<regs::Psor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Port Clear Output"]
    #[inline(always)]
    pub const fn pcor(self) -> crate::common::Reg<regs::Pcor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Port Toggle Output"]
    #[inline(always)]
    pub const fn ptor(self) -> crate::common::Reg<regs::Ptor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn pdir(self) -> crate::common::Reg<regs::Pdir, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Port Data Direction"]
    #[inline(always)]
    pub const fn pddr(self) -> crate::common::Reg<regs::Pddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Port Input Disable"]
    #[inline(always)]
    pub const fn pidr(self) -> crate::common::Reg<regs::Pidr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Pin Data"]
    #[inline(always)]
    pub const fn pdr(self, n: usize) -> crate::common::Reg<regs::Pdr, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 1usize) as _) }
    }
    #[doc = "Interrupt Control index"]
    #[inline(always)]
    pub const fn icr(self, n: usize) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "Global Interrupt Control Low"]
    #[inline(always)]
    pub const fn giclr(self) -> crate::common::Reg<regs::Giclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Global Interrupt Control High"]
    #[inline(always)]
    pub const fn gichr(self) -> crate::common::Reg<regs::Gichr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn isfr(self, n: usize) -> crate::common::Reg<regs::Isfr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize + n * 4usize) as _)
        }
    }
}
pub mod regs;
pub mod vals;
