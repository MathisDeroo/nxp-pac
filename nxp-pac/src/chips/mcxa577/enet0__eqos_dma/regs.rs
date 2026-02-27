#[doc = "Debug Status 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaDebugStatus0(pub u32);
impl DmaDebugStatus0 {
    #[doc = "AHB Master Status"]
    #[must_use]
    #[inline(always)]
    pub const fn axwhsts(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Master Status"]
    #[inline(always)]
    pub const fn set_axwhsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA Channel 0 Receive Process State"]
    #[must_use]
    #[inline(always)]
    pub const fn rps0(&self) -> super::vals::Rps0 {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Rps0::from_bits(val as u8)
    }
    #[doc = "DMA Channel 0 Receive Process State"]
    #[inline(always)]
    pub const fn set_rps0(&mut self, val: super::vals::Rps0) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "DMA Channel 0 Transmit Process State"]
    #[must_use]
    #[inline(always)]
    pub const fn tps0(&self) -> super::vals::Tps0 {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Tps0::from_bits(val as u8)
    }
    #[doc = "DMA Channel 0 Transmit Process State"]
    #[inline(always)]
    pub const fn set_tps0(&mut self, val: super::vals::Tps0) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
}
impl Default for DmaDebugStatus0 {
    #[inline(always)]
    fn default() -> DmaDebugStatus0 {
        DmaDebugStatus0(0)
    }
}
impl core::fmt::Debug for DmaDebugStatus0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaDebugStatus0")
            .field("axwhsts", &self.axwhsts())
            .field("rps0", &self.rps0())
            .field("tps0", &self.tps0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaDebugStatus0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaDebugStatus0 {{ axwhsts: {=bool:?}, rps0: {:?}, tps0: {:?} }}",
            self.axwhsts(),
            self.rps0(),
            self.tps0()
        )
    }
}
#[doc = "Interrupt Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaInterruptStatus(pub u32);
impl DmaInterruptStatus {
    #[doc = "DMA Channel 0 Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn dc0is(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Channel 0 Interrupt Status"]
    #[inline(always)]
    pub const fn set_dc0is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "MTL Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn mtlis(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "MTL Interrupt Status"]
    #[inline(always)]
    pub const fn set_mtlis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "MAC Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn macis(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "MAC Interrupt Status"]
    #[inline(always)]
    pub const fn set_macis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for DmaInterruptStatus {
    #[inline(always)]
    fn default() -> DmaInterruptStatus {
        DmaInterruptStatus(0)
    }
}
impl core::fmt::Debug for DmaInterruptStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaInterruptStatus")
            .field("dc0is", &self.dc0is())
            .field("mtlis", &self.mtlis())
            .field("macis", &self.macis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaInterruptStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaInterruptStatus {{ dc0is: {=bool:?}, mtlis: {=bool:?}, macis: {=bool:?} }}",
            self.dc0is(),
            self.mtlis(),
            self.macis()
        )
    }
}
#[doc = "Bus Mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaMode(pub u32);
impl DmaMode {
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn swr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_swr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA Tx or Rx Arbitration Scheme"]
    #[must_use]
    #[inline(always)]
    pub const fn da(&self) -> super::vals::Da {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Da::from_bits(val as u8)
    }
    #[doc = "DMA Tx or Rx Arbitration Scheme"]
    #[inline(always)]
    pub const fn set_da(&mut self, val: super::vals::Da) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "ARBC is Synopsys Reserved, This field must be set to \"0\"."]
    #[must_use]
    #[inline(always)]
    pub const fn arbc(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "ARBC is Synopsys Reserved, This field must be set to \"0\"."]
    #[inline(always)]
    pub const fn set_arbc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Transmit Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn txpr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Priority"]
    #[inline(always)]
    pub const fn set_txpr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Priority Ratio"]
    #[must_use]
    #[inline(always)]
    pub const fn pr(&self) -> super::vals::Pr {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Pr::from_bits(val as u8)
    }
    #[doc = "Priority Ratio"]
    #[inline(always)]
    pub const fn set_pr(&mut self, val: super::vals::Pr) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Interrupt Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn intm(&self) -> super::vals::Intm {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Intm::from_bits(val as u8)
    }
    #[doc = "Interrupt Mode"]
    #[inline(always)]
    pub const fn set_intm(&mut self, val: super::vals::Intm) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
}
impl Default for DmaMode {
    #[inline(always)]
    fn default() -> DmaMode {
        DmaMode(0)
    }
}
impl core::fmt::Debug for DmaMode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaMode")
            .field("swr", &self.swr())
            .field("da", &self.da())
            .field("arbc", &self.arbc())
            .field("txpr", &self.txpr())
            .field("pr", &self.pr())
            .field("intm", &self.intm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaMode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaMode {{ swr: {=bool:?}, da: {:?}, arbc: {=bool:?}, txpr: {=bool:?}, pr: {:?}, intm: {:?} }}",
            self.swr(),
            self.da(),
            self.arbc(),
            self.txpr(),
            self.pr(),
            self.intm()
        )
    }
}
#[doc = "System Bus Mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaSysBusMode(pub u32);
impl DmaSysBusMode {
    #[doc = "Fixed Burst Length"]
    #[must_use]
    #[inline(always)]
    pub const fn fb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Fixed Burst Length"]
    #[inline(always)]
    pub const fn set_fb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Address-Aligned Beats"]
    #[must_use]
    #[inline(always)]
    pub const fn aal(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Address-Aligned Beats"]
    #[inline(always)]
    pub const fn set_aal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Mixed Burst"]
    #[must_use]
    #[inline(always)]
    pub const fn mb(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Mixed Burst"]
    #[inline(always)]
    pub const fn set_mb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Rebuild INCRx Burst"]
    #[must_use]
    #[inline(always)]
    pub const fn rb(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Rebuild INCRx Burst"]
    #[inline(always)]
    pub const fn set_rb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for DmaSysBusMode {
    #[inline(always)]
    fn default() -> DmaSysBusMode {
        DmaSysBusMode(0)
    }
}
impl core::fmt::Debug for DmaSysBusMode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaSysBusMode")
            .field("fb", &self.fb())
            .field("aal", &self.aal())
            .field("mb", &self.mb())
            .field("rb", &self.rb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaSysBusMode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaSysBusMode {{ fb: {=bool:?}, aal: {=bool:?}, mb: {=bool:?}, rb: {=bool:?} }}",
            self.fb(),
            self.aal(),
            self.mb(),
            self.rb()
        )
    }
}
