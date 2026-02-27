#[doc = "SECCON"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seccon {
    ptr: *mut u8,
}
unsafe impl Send for Seccon {}
unsafe impl Sync for Seccon {}
impl Seccon {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Secure CPU0 System Tick Calibration"]
    #[inline(always)]
    pub const fn cpu0stckcal(self) -> crate::common::Reg<regs::Cpu0stckcal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "MSF Configuration"]
    #[inline(always)]
    pub const fn msfcfg(self) -> crate::common::Reg<regs::Msfcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x061cusize) as _) }
    }
    #[doc = "CFPA State Register"]
    #[inline(always)]
    pub const fn cfpa_lc_state(self) -> crate::common::Reg<regs::CfpaLcState, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x063cusize) as _) }
    }
    #[doc = "RAM XEN Control"]
    #[inline(always)]
    pub const fn ram_xen(self) -> crate::common::Reg<regs::RamXen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0658usize) as _) }
    }
    #[doc = "RAM XEN Control (Duplicate)"]
    #[inline(always)]
    pub const fn ram_xen_dp(self) -> crate::common::Reg<regs::RamXenDp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x065cusize) as _) }
    }
    #[doc = "GDET0 Control Register"]
    #[inline(always)]
    pub const fn gdet0_ctrl(self) -> crate::common::Reg<regs::Gdet0Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x066cusize) as _) }
    }
    #[doc = "Life Cycle State Register"]
    #[inline(always)]
    pub const fn els_otp_lc_state(
        self,
    ) -> crate::common::Reg<regs::ElsOtpLcState, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0680usize) as _) }
    }
    #[doc = "Life Cycle State Register (Duplicate)"]
    #[inline(always)]
    pub const fn els_otp_lc_state_dp(
        self,
    ) -> crate::common::Reg<regs::ElsOtpLcStateDp, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0684usize) as _) }
    }
    #[doc = "Control Write Access to Security"]
    #[inline(always)]
    pub const fn debug_lock_en(self) -> crate::common::Reg<regs::DebugLockEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07a0usize) as _) }
    }
    #[doc = "Cortex Debug Features Control"]
    #[inline(always)]
    pub const fn debug_features(
        self,
    ) -> crate::common::Reg<regs::DebugFeatures, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07a4usize) as _) }
    }
    #[doc = "Cortex Debug Features Control (Duplicate)"]
    #[inline(always)]
    pub const fn debug_features_dp(
        self,
    ) -> crate::common::Reg<regs::DebugFeaturesDp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07a8usize) as _) }
    }
    #[doc = "CPU0 Software Debug Access"]
    #[inline(always)]
    pub const fn swd_access_cpu0(
        self,
    ) -> crate::common::Reg<regs::SwdAccessCpu0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07b4usize) as _) }
    }
    #[doc = "Debug Authentication BEACON"]
    #[inline(always)]
    pub const fn debug_auth_beacon(
        self,
    ) -> crate::common::Reg<regs::DebugAuthBeacon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07c0usize) as _) }
    }
}
pub mod regs;
pub mod vals;
