#[doc = "SYSCON"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscon {
    ptr: *mut u8,
}
unsafe impl Send for Syscon {}
unsafe impl Sync for Syscon {}
impl Syscon {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "AHB Matrix Remap Control"]
    #[inline(always)]
    pub const fn remap(self) -> crate::common::Reg<regs::Remap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "AHB Matrix Priority Control"]
    #[inline(always)]
    pub const fn ahbmatprio(self) -> crate::common::Reg<regs::Ahbmatprio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "NMI Source Select"]
    #[inline(always)]
    pub const fn nmisrc(self) -> crate::common::Reg<regs::Nmisrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Protect Level Control"]
    #[inline(always)]
    pub const fn protlvl(self) -> crate::common::Reg<regs::Protlvl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Non-Secure CPU0 System Tick Calibration"]
    #[inline(always)]
    pub const fn cpu0nstckcal(self) -> crate::common::Reg<regs::Cpu0nstckcal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "System Clock Divider"]
    #[inline(always)]
    pub const fn ahbclkdiv(self) -> crate::common::Reg<regs::Ahbclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "BUS_CLK Clock Divider"]
    #[inline(always)]
    pub const fn busclkdiv(self) -> crate::common::Reg<regs::Busclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "SLOW_CLK Clock Divider"]
    #[inline(always)]
    pub const fn slowclkdiv(self) -> crate::common::Reg<regs::Slowclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "FRO_HF_DIV Clock Divider"]
    #[inline(always)]
    pub const fn frohfdiv(self) -> crate::common::Reg<regs::Frohfdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "FRO_LF_DIV Clock Divider"]
    #[inline(always)]
    pub const fn frolfdiv(self) -> crate::common::Reg<regs::Frolfdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "PLL1_CLK_DIV Clock Divider"]
    #[inline(always)]
    pub const fn pll1clkdiv(self) -> crate::common::Reg<regs::Pll1clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Clock Configuration Unlock"]
    #[inline(always)]
    pub const fn clkunlock(self) -> crate::common::Reg<regs::Clkunlock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Gray to Binary Converter Gray Code \\[31:0\\]"]
    #[inline(always)]
    pub const fn gray_code_lsb(self) -> crate::common::Reg<regs::GrayCodeLsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Gray to Binary Converter Gray Code \\[41:32\\]"]
    #[inline(always)]
    pub const fn gray_code_msb(self) -> crate::common::Reg<regs::GrayCodeMsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Gray to Binary Converter Binary Code \\[31:0\\]"]
    #[inline(always)]
    pub const fn binary_code_lsb(
        self,
    ) -> crate::common::Reg<regs::BinaryCodeLsb, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "Gray to Binary Converter Binary Code \\[41:32\\]"]
    #[inline(always)]
    pub const fn binary_code_msb(
        self,
    ) -> crate::common::Reg<regs::BinaryCodeMsb, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "Ethernet Control"]
    #[inline(always)]
    pub const fn enet_ctrl(self) -> crate::common::Reg<regs::EnetCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "Sideband Flow Control"]
    #[inline(always)]
    pub const fn enet_sbd_flow_ctrl(
        self,
    ) -> crate::common::Reg<regs::EnetSbdFlowCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "NVM Control"]
    #[inline(always)]
    pub const fn nvm_ctrl(self) -> crate::common::Reg<regs::NvmCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "SmartDMA Interrupt Hijack"]
    #[inline(always)]
    pub const fn smart_dmaint(self) -> crate::common::Reg<regs::SmartDmaint, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "Immediate cessation of execution following the completion of ROM execution"]
    #[inline(always)]
    pub const fn bootrom(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "Controls Shared RAM Integration"]
    #[inline(always)]
    pub const fn ram_casp_ctrl(self) -> crate::common::Reg<regs::RamCaspCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "CPU Status"]
    #[inline(always)]
    pub const fn cpustat(self) -> crate::common::Reg<regs::Cpustat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x030cusize) as _) }
    }
    #[doc = "LPCAC Control"]
    #[inline(always)]
    pub const fn lpcac_ctrl(self) -> crate::common::Reg<regs::LpcacCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0324usize) as _) }
    }
    #[doc = "I3C Misc Control"]
    #[inline(always)]
    pub const fn i3c_misc_ctrl(self) -> crate::common::Reg<regs::I3cMiscCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0328usize) as _) }
    }
    #[doc = "CTIMER Global Start Enable"]
    #[inline(always)]
    pub const fn ctimerglobalstarten(
        self,
    ) -> crate::common::Reg<regs::Ctimerglobalstarten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize) as _) }
    }
    #[doc = "RAM Control"]
    #[inline(always)]
    pub const fn ram_ctrl(self) -> crate::common::Reg<regs::RamCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0444usize) as _) }
    }
    #[doc = "JTAG Chip ID"]
    #[inline(always)]
    pub const fn jtag_id(self) -> crate::common::Reg<regs::JtagId, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07f0usize) as _) }
    }
    #[doc = "Device Type"]
    #[inline(always)]
    pub const fn device_type(self) -> crate::common::Reg<regs::DeviceType, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07f4usize) as _) }
    }
    #[doc = "Device ID"]
    #[inline(always)]
    pub const fn device_id0(self) -> crate::common::Reg<regs::DeviceId0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07f8usize) as _) }
    }
}
pub mod regs;
pub mod vals;
