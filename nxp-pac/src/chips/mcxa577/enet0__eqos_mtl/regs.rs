#[doc = "Interrupt Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlInterruptStatus(pub u32);
impl MtlInterruptStatus {
    #[doc = "Queue 0 Interrupt status"]
    #[must_use]
    #[inline(always)]
    pub const fn q0is(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Queue 0 Interrupt status"]
    #[inline(always)]
    pub const fn set_q0is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for MtlInterruptStatus {
    #[inline(always)]
    fn default() -> MtlInterruptStatus {
        MtlInterruptStatus(0)
    }
}
impl core::fmt::Debug for MtlInterruptStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlInterruptStatus")
            .field("q0is", &self.q0is())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlInterruptStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MtlInterruptStatus {{ q0is: {=bool:?} }}", self.q0is())
    }
}
#[doc = "Operation Mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlOperationMode(pub u32);
impl MtlOperationMode {
    #[doc = "Drop Transmit Status"]
    #[must_use]
    #[inline(always)]
    pub const fn dtxsts(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Drop Transmit Status"]
    #[inline(always)]
    pub const fn set_dtxsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Counters Preset"]
    #[must_use]
    #[inline(always)]
    pub const fn cntprst(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Counters Preset"]
    #[inline(always)]
    pub const fn set_cntprst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Counters Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn cntclr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Counters Reset"]
    #[inline(always)]
    pub const fn set_cntclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for MtlOperationMode {
    #[inline(always)]
    fn default() -> MtlOperationMode {
        MtlOperationMode(0)
    }
}
impl core::fmt::Debug for MtlOperationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlOperationMode")
            .field("dtxsts", &self.dtxsts())
            .field("cntprst", &self.cntprst())
            .field("cntclr", &self.cntclr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlOperationMode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlOperationMode {{ dtxsts: {=bool:?}, cntprst: {=bool:?}, cntclr: {=bool:?} }}",
            self.dtxsts(),
            self.cntprst(),
            self.cntclr()
        )
    }
}
