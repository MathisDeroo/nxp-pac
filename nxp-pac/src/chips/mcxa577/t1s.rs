#[doc = "TENBASET_PHY"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T1s {
    ptr: *mut u8,
}
unsafe impl Send for T1s {}
unsafe impl Sync for T1s {}
impl T1s {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PHY control"]
    #[inline(always)]
    pub const fn phyctrl(self) -> crate::common::Reg<regs::Phyctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "PHY status"]
    #[inline(always)]
    pub const fn phystat(self) -> crate::common::Reg<regs::Phystat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "PHY Identifier0"]
    #[inline(always)]
    pub const fn phyid0(self) -> crate::common::Reg<regs::Phyid0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "PHY Identifier1"]
    #[inline(always)]
    pub const fn phyid1(self) -> crate::common::Reg<regs::Phyid1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "PHY BASE-T1 Extended"]
    #[inline(always)]
    pub const fn base1ext(self) -> crate::common::Reg<regs::Base1ext, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "PMA/PMD Control"]
    #[inline(always)]
    pub const fn pmapmdctrl(self) -> crate::common::Reg<regs::Pmapmdctrl, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "PMA CTRL"]
    #[inline(always)]
    pub const fn pmactrl(self) -> crate::common::Reg<regs::Pmactrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x92usize) as _) }
    }
    #[doc = "PMA Status"]
    #[inline(always)]
    pub const fn pmastat(self) -> crate::common::Reg<regs::Pmastat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "PMA Test Mode"]
    #[inline(always)]
    pub const fn pmatm(self) -> crate::common::Reg<regs::Pmatm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x96usize) as _) }
    }
    #[doc = "PCS Control"]
    #[inline(always)]
    pub const fn pcsctrl(self) -> crate::common::Reg<regs::Pcsctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa6usize) as _) }
    }
    #[doc = "PCS Status"]
    #[inline(always)]
    pub const fn pcsstat(self) -> crate::common::Reg<regs::Pcsstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "PCS Remote Jabber Counter"]
    #[inline(always)]
    pub const fn pcsdiag1(self) -> crate::common::Reg<regs::Pcsdiag1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xaausize) as _) }
    }
    #[doc = "PCS Physical Collisions Counter"]
    #[inline(always)]
    pub const fn pcsdiag2(self) -> crate::common::Reg<regs::Pcsdiag2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "PLCA Identification and Version"]
    #[inline(always)]
    pub const fn plcaidver(self) -> crate::common::Reg<regs::Plcaidver, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "PLCA Control0"]
    #[inline(always)]
    pub const fn plcactrl0(self) -> crate::common::Reg<regs::Plcactrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc2usize) as _) }
    }
    #[doc = "PLCA Control1"]
    #[inline(always)]
    pub const fn plcactrl1(self) -> crate::common::Reg<regs::Plcactrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "PLCA Status"]
    #[inline(always)]
    pub const fn plcastat(self) -> crate::common::Reg<regs::Plcastat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc6usize) as _) }
    }
    #[doc = "PLCA Transmit Opportunity Timer"]
    #[inline(always)]
    pub const fn plcatotmr(self) -> crate::common::Reg<regs::Plcatotmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "PLCA Burst Mode Configuration"]
    #[inline(always)]
    pub const fn plcaburst(self) -> crate::common::Reg<regs::Plcaburst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xcausize) as _) }
    }
    #[doc = "PLCA Status"]
    #[inline(always)]
    pub const fn plcarecst(self) -> crate::common::Reg<regs::Plcarecst, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "MODE Control"]
    #[inline(always)]
    pub const fn modectrl(self) -> crate::common::Reg<regs::Modectrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Mode Configuration"]
    #[inline(always)]
    pub const fn modecfg(self) -> crate::common::Reg<regs::Modecfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0102usize) as _) }
    }
    #[doc = "Mode Status"]
    #[inline(always)]
    pub const fn modestat(self) -> crate::common::Reg<regs::Modestat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Wake Control"]
    #[inline(always)]
    pub const fn wkctrl(self) -> crate::common::Reg<regs::Wkctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Wake Configuration"]
    #[inline(always)]
    pub const fn wkcfg(self) -> crate::common::Reg<regs::Wkcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0122usize) as _) }
    }
    #[doc = "Wake Control"]
    #[inline(always)]
    pub const fn wupctrl(self) -> crate::common::Reg<regs::Wupctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0126usize) as _) }
    }
    #[doc = "Wakeup Confguration"]
    #[inline(always)]
    pub const fn wupcfg(self) -> crate::common::Reg<regs::Wupcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Wakeup Status"]
    #[inline(always)]
    pub const fn wupstat(self) -> crate::common::Reg<regs::Wupstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012ausize) as _) }
    }
    #[doc = "SMI Frame DATA"]
    #[inline(always)]
    pub const fn smidata(self) -> crate::common::Reg<regs::Smidata, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "SMI Control"]
    #[inline(always)]
    pub const fn smictrl(self) -> crate::common::Reg<regs::Smictrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0142usize) as _) }
    }
    #[doc = "SMI Configuration"]
    #[inline(always)]
    pub const fn smicfg(self) -> crate::common::Reg<regs::Smicfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "SMI Status"]
    #[inline(always)]
    pub const fn smistat(self) -> crate::common::Reg<regs::Smistat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0146usize) as _) }
    }
    #[doc = "Interrupt Capture1"]
    #[inline(always)]
    pub const fn intencapt1(self) -> crate::common::Reg<regs::Intencapt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Interrupt Enable Set1"]
    #[inline(always)]
    pub const fn intenset1(self) -> crate::common::Reg<regs::Intenset1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0162usize) as _) }
    }
    #[doc = "Interrupt Enable Clear1"]
    #[inline(always)]
    pub const fn intenclr1(self) -> crate::common::Reg<regs::Intenclr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Interrupt Status1"]
    #[inline(always)]
    pub const fn intstat1(self) -> crate::common::Reg<regs::Intstat1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0166usize) as _) }
    }
    #[doc = "Interrupt Capture2"]
    #[inline(always)]
    pub const fn intencapt2(self) -> crate::common::Reg<regs::Intencapt2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "Interrupt Enable Set1"]
    #[inline(always)]
    pub const fn intenset2(self) -> crate::common::Reg<regs::Intenset2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0172usize) as _) }
    }
    #[doc = "Interrupt Enable Clear2"]
    #[inline(always)]
    pub const fn intenclr2(self) -> crate::common::Reg<regs::Intenclr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "Interrupt Status2"]
    #[inline(always)]
    pub const fn intstat2(self) -> crate::common::Reg<regs::Intstat2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0176usize) as _) }
    }
    #[doc = "PLCA TC14 Adv diagnostics"]
    #[inline(always)]
    pub const fn plcadiag1(self) -> crate::common::Reg<regs::Plcadiag1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "PLCA NXP prop diagnostics"]
    #[inline(always)]
    pub const fn plcadiag2(self) -> crate::common::Reg<regs::Plcadiag2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a2usize) as _) }
    }
    #[doc = "PLCA Beacon Counter Diagnostics"]
    #[inline(always)]
    pub const fn plcadiag3(self) -> crate::common::Reg<regs::Plcadiag3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "PLCA TO Counter Diagnostics"]
    #[inline(always)]
    pub const fn plcadiag4(self) -> crate::common::Reg<regs::Plcadiag4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a6usize) as _) }
    }
    #[doc = "Transceiver Diagnostics"]
    #[inline(always)]
    pub const fn txcdiag(self) -> crate::common::Reg<regs::Txcdiag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "Mask Control PLCADIAG1 Flag"]
    #[inline(always)]
    pub const fn mskplcadiag1(self) -> crate::common::Reg<regs::Mskplcadiag1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "Mask Control PLCADIAG2 Flags"]
    #[inline(always)]
    pub const fn mskplcadiag2(self) -> crate::common::Reg<regs::Mskplcadiag2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b2usize) as _) }
    }
    #[doc = "Mask Control TXCDIAG Flag"]
    #[inline(always)]
    pub const fn msktxcdiag(self) -> crate::common::Reg<regs::Msktxcdiag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "Global Access Control"]
    #[inline(always)]
    pub const fn accessctrl(self) -> crate::common::Reg<regs::Accessctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "Debug Control"]
    #[inline(always)]
    pub const fn dbgctrl(self) -> crate::common::Reg<regs::Dbgctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e2usize) as _) }
    }
    #[doc = "Version Information"]
    #[inline(always)]
    pub const fn version(self) -> crate::common::Reg<regs::Version, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01feusize) as _) }
    }
}
pub mod regs;
pub mod vals;
