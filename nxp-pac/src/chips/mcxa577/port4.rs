#[doc = "Port Control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Port4 {
    ptr: *mut u8,
}
unsafe impl Send for Port4 {}
unsafe impl Sync for Port4 {}
impl Port4 {
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
    #[doc = "Global Pin Control Low"]
    #[inline(always)]
    pub const fn gpclr(self) -> crate::common::Reg<regs::Gpclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Global Pin Control High"]
    #[inline(always)]
    pub const fn gpchr(self) -> crate::common::Reg<regs::Gpchr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Configuration"]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Calibration 0"]
    #[inline(always)]
    pub const fn calib0(self) -> crate::common::Reg<regs::Calib0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Calibration 1"]
    #[inline(always)]
    pub const fn calib1(self) -> crate::common::Reg<regs::Calib1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Pin Control 0"]
    #[inline(always)]
    pub const fn pcr0(self) -> crate::common::Reg<regs::Pcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Pin Control 1"]
    #[inline(always)]
    pub const fn pcr1(self) -> crate::common::Reg<regs::Pcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Pin Control 2"]
    #[inline(always)]
    pub const fn pcr2(self) -> crate::common::Reg<regs::Pcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Pin Control 3"]
    #[inline(always)]
    pub const fn pcr3(self) -> crate::common::Reg<regs::Pcr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Pin Control 4"]
    #[inline(always)]
    pub const fn pcr4(self) -> crate::common::Reg<regs::Pcr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Pin Control 5"]
    #[inline(always)]
    pub const fn pcr5(self) -> crate::common::Reg<regs::Pcr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Pin Control 6"]
    #[inline(always)]
    pub const fn pcr6(self) -> crate::common::Reg<regs::Pcr6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Pin Control 7"]
    #[inline(always)]
    pub const fn pcr7(self) -> crate::common::Reg<regs::Pcr7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Pin Control 8"]
    #[inline(always)]
    pub const fn pcr8(self) -> crate::common::Reg<regs::Pcr8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Pin Control 9"]
    #[inline(always)]
    pub const fn pcr9(self) -> crate::common::Reg<regs::Pcr9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Pin Control 10"]
    #[inline(always)]
    pub const fn pcr10(self) -> crate::common::Reg<regs::Pcr10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Pin Control 11"]
    #[inline(always)]
    pub const fn pcr11(self) -> crate::common::Reg<regs::Pcr11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "Pin Control 12"]
    #[inline(always)]
    pub const fn pcr12(self) -> crate::common::Reg<regs::Pcr12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Pin Control 13"]
    #[inline(always)]
    pub const fn pcr13(self) -> crate::common::Reg<regs::Pcr13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
