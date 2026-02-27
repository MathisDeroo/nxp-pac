#[doc = "USBNC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbnc {
    ptr: *mut u8,
}
unsafe impl Send for Usbnc {}
unsafe impl Sync for Usbnc {}
impl Usbnc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "USB Control 1"]
    #[inline(always)]
    pub const fn ctrl1(self) -> crate::common::Reg<regs::Ctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "USB Control 2"]
    #[inline(always)]
    pub const fn ctrl2(self) -> crate::common::Reg<regs::Ctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "HSIC DLL Configure Register 4"]
    #[inline(always)]
    pub const fn hsic_dll_cfg4(self) -> crate::common::Reg<regs::HsicDllCfg4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "USB LPM Control and Status 0"]
    #[inline(always)]
    pub const fn lpm_csr0(self) -> crate::common::Reg<regs::LpmCsr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "USB LPM Control and Status 1"]
    #[inline(always)]
    pub const fn lpm_csr1(self) -> crate::common::Reg<regs::LpmCsr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "USB LPM Control and Status 2"]
    #[inline(always)]
    pub const fn lpm_csr2(self) -> crate::common::Reg<regs::LpmCsr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "USB Clock Recovery Control"]
    #[inline(always)]
    pub const fn clk_recover_ctrl(
        self,
    ) -> crate::common::Reg<regs::ClkRecoverCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "FIRC Oscillator Enable"]
    #[inline(always)]
    pub const fn clk_recover_irc_en(
        self,
    ) -> crate::common::Reg<regs::ClkRecoverIrcEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Clock Recovery Combined Interrupt Enable"]
    #[inline(always)]
    pub const fn clk_recover_int_en(
        self,
    ) -> crate::common::Reg<regs::ClkRecoverIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "Clock Recovery Separated Interrupt Status"]
    #[inline(always)]
    pub const fn clk_recover_int_status(
        self,
    ) -> crate::common::Reg<regs::ClkRecoverIntStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
