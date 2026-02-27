#[doc = "DA_IP_TSI_UG_3V_CLN40ULP"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsi {
    ptr: *mut u8,
}
unsafe impl Send for Tsi {}
unsafe impl Sync for Tsi {}
impl Tsi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "TSI CONFIG Register (TSI_CONFIG) for self-capacitor (CONFIG)"]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "TSI CONFIG Register (TSI_CONFIG) for mutual-capacitor (CONFIG_MUTUAL)"]
    #[inline(always)]
    pub const fn config_mutual(self) -> crate::common::Reg<regs::ConfigMutual, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "TSI Threshold Register"]
    #[inline(always)]
    pub const fn tshd(self) -> crate::common::Reg<regs::Tshd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "TSI General Control and Status Register"]
    #[inline(always)]
    pub const fn gencs(self) -> crate::common::Reg<regs::Gencs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "TSI Mutual-cap Register"]
    #[inline(always)]
    pub const fn mul(self) -> crate::common::Reg<regs::Mul, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "TSI SINC filter Register"]
    #[inline(always)]
    pub const fn sinc(self) -> crate::common::Reg<regs::Sinc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "TSI SSC Register 0"]
    #[inline(always)]
    pub const fn ssc0(self) -> crate::common::Reg<regs::Ssc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "TSI SSC Register 1"]
    #[inline(always)]
    pub const fn ssc1(self) -> crate::common::Reg<regs::Ssc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "TSI SSC Register 2"]
    #[inline(always)]
    pub const fn ssc2(self) -> crate::common::Reg<regs::Ssc2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "TSI Baseline Register"]
    #[inline(always)]
    pub const fn baseline(self) -> crate::common::Reg<regs::Baseline, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "TSI Shield Register"]
    #[inline(always)]
    pub const fn shield(self) -> crate::common::Reg<regs::Shield, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Self-cap mode channels selection(CH31~CH0)"]
    #[inline(always)]
    pub const fn self_sel_31_0(self) -> crate::common::Reg<regs::SelfSel310, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Self-cap mode channels selection(CH63~CH32)"]
    #[inline(always)]
    pub const fn self_sel_63_32(self) -> crate::common::Reg<regs::SelfSel6332, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Self-cap mode channels selection(CH69~CH64)"]
    #[inline(always)]
    pub const fn self_sel_69_64(self) -> crate::common::Reg<regs::SelfSel6964, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Shield mode channels selection(CH31~CH0)"]
    #[inline(always)]
    pub const fn shield_sel_31_0(
        self,
    ) -> crate::common::Reg<regs::ShieldSel310, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Shield mode channels selection(CH63~CH32)"]
    #[inline(always)]
    pub const fn shield_sel_63_32(
        self,
    ) -> crate::common::Reg<regs::ShieldSel6332, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Shield mode channels selection(CH69~CH64)"]
    #[inline(always)]
    pub const fn shield_sel_69_64(
        self,
    ) -> crate::common::Reg<regs::ShieldSel6964, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Mutual TX mode channels selection(CH31~CH0)"]
    #[inline(always)]
    pub const fn mutual_tx_sel_31_0(
        self,
    ) -> crate::common::Reg<regs::MutualTxSel310, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Mutual TX mode channels selection(CH63~CH32)"]
    #[inline(always)]
    pub const fn mutual_tx_sel_63_32(
        self,
    ) -> crate::common::Reg<regs::MutualTxSel6332, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Mutual TX mode channels selection(CH69~CH64)"]
    #[inline(always)]
    pub const fn mutual_tx_sel_69_64(
        self,
    ) -> crate::common::Reg<regs::MutualTxSel6964, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Mutual RX mode channels selection(CH31~CH0)"]
    #[inline(always)]
    pub const fn mutual_rx_sel_31_0(
        self,
    ) -> crate::common::Reg<regs::MutualRxSel310, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Mutual RX mode channels selection(CH63~CH32)"]
    #[inline(always)]
    pub const fn mutual_rx_sel_63_32(
        self,
    ) -> crate::common::Reg<regs::MutualRxSel6332, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Mutual RX mode channels selection(CH69~CH64)"]
    #[inline(always)]
    pub const fn mutual_rx_sel_69_64(
        self,
    ) -> crate::common::Reg<regs::MutualRxSel6964, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "TSI Data and Status Register"]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<regs::Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "TSI Miscellaneous Register"]
    #[inline(always)]
    pub const fn misc(self) -> crate::common::Reg<regs::Misc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "TSI AUTO TRIG Register"]
    #[inline(always)]
    pub const fn trig(self) -> crate::common::Reg<regs::Trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "TSI TEST Register"]
    #[inline(always)]
    pub const fn test(self) -> crate::common::Reg<regs::Test, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
}
pub mod regs;
pub mod vals;
