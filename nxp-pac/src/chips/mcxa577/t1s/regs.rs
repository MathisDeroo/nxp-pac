#[doc = "Global Access Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accessctrl(pub u16);
impl Accessctrl {
    #[doc = "APB Write Access"]
    #[must_use]
    #[inline(always)]
    pub const fn access(&self) -> super::vals::Access {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Access::from_bits(val as u8)
    }
    #[doc = "APB Write Access"]
    #[inline(always)]
    pub const fn set_access(&mut self, val: super::vals::Access) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
}
impl Default for Accessctrl {
    #[inline(always)]
    fn default() -> Accessctrl {
        Accessctrl(0)
    }
}
impl core::fmt::Debug for Accessctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Accessctrl")
            .field("access", &self.access())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Accessctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Accessctrl {{ access: {:?} }}", self.access())
    }
}
#[doc = "PHY BASE-T1 Extended"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Base1ext(pub u16);
impl Base1ext {
    #[doc = "Capability Info"]
    #[must_use]
    #[inline(always)]
    pub const fn baset1ext_bit(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capability Info"]
    #[inline(always)]
    pub const fn set_baset1ext_bit(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Base1ext {
    #[inline(always)]
    fn default() -> Base1ext {
        Base1ext(0)
    }
}
impl core::fmt::Debug for Base1ext {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Base1ext")
            .field("baset1ext_bit", &self.baset1ext_bit())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Base1ext {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Base1ext {{ baset1ext_bit: {=u16:?} }}",
            self.baset1ext_bit()
        )
    }
}
#[doc = "Debug Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgctrl(pub u16);
impl Dbgctrl {
    #[doc = "Debug"]
    #[must_use]
    #[inline(always)]
    pub const fn debug(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Debug"]
    #[inline(always)]
    pub const fn set_debug(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for Dbgctrl {
    #[inline(always)]
    fn default() -> Dbgctrl {
        Dbgctrl(0)
    }
}
impl core::fmt::Debug for Dbgctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbgctrl")
            .field("debug", &self.debug())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbgctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dbgctrl {{ debug: {=u8:?} }}", self.debug())
    }
}
#[doc = "Interrupt Capture1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intencapt1(pub u16);
impl Intencapt1 {
    #[doc = "Physical Collision"]
    #[must_use]
    #[inline(always)]
    pub const fn physcol(&self) -> super::vals::Intencapt1Physcol {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Intencapt1Physcol::from_bits(val as u8)
    }
    #[doc = "Physical Collision"]
    #[inline(always)]
    pub const fn set_physcol(&mut self, val: super::vals::Intencapt1Physcol) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "PLCA Recovery"]
    #[must_use]
    #[inline(always)]
    pub const fn plcarec(&self) -> super::vals::Intencapt1Plcarec {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Intencapt1Plcarec::from_bits(val as u8)
    }
    #[doc = "PLCA Recovery"]
    #[inline(always)]
    pub const fn set_plcarec(&mut self, val: super::vals::Intencapt1Plcarec) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "PLCA Status"]
    #[must_use]
    #[inline(always)]
    pub const fn plcastat(&self) -> super::vals::Intencapt1Plcastat {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Intencapt1Plcastat::from_bits(val as u8)
    }
    #[doc = "PLCA Status"]
    #[inline(always)]
    pub const fn set_plcastat(&mut self, val: super::vals::Intencapt1Plcastat) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "MODE Status"]
    #[must_use]
    #[inline(always)]
    pub const fn modestat(&self) -> super::vals::Intencapt1Modestat {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Intencapt1Modestat::from_bits(val as u8)
    }
    #[doc = "MODE Status"]
    #[inline(always)]
    pub const fn set_modestat(&mut self, val: super::vals::Intencapt1Modestat) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "Invalid APB"]
    #[must_use]
    #[inline(always)]
    pub const fn invldapb(&self) -> super::vals::Intencapt1Invldapb {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Intencapt1Invldapb::from_bits(val as u8)
    }
    #[doc = "Invalid APB"]
    #[inline(always)]
    pub const fn set_invldapb(&mut self, val: super::vals::Intencapt1Invldapb) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Local Jabber"]
    #[must_use]
    #[inline(always)]
    pub const fn locjab(&self) -> super::vals::Intencapt1Locjab {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Intencapt1Locjab::from_bits(val as u8)
    }
    #[doc = "Local Jabber"]
    #[inline(always)]
    pub const fn set_locjab(&mut self, val: super::vals::Intencapt1Locjab) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "Remote Jabber"]
    #[must_use]
    #[inline(always)]
    pub const fn remjab(&self) -> super::vals::Intencapt1Remjab {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Intencapt1Remjab::from_bits(val as u8)
    }
    #[doc = "Remote Jabber"]
    #[inline(always)]
    pub const fn set_remjab(&mut self, val: super::vals::Intencapt1Remjab) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "PIN Fault Monitor"]
    #[must_use]
    #[inline(always)]
    pub const fn pinfault(&self) -> super::vals::Intencapt1Pinfault {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Intencapt1Pinfault::from_bits(val as u8)
    }
    #[doc = "PIN Fault Monitor"]
    #[inline(always)]
    pub const fn set_pinfault(&mut self, val: super::vals::Intencapt1Pinfault) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "PLCA Diagnostics"]
    #[must_use]
    #[inline(always)]
    pub const fn plcadiag(&self) -> super::vals::Intencapt1Plcadiag {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Intencapt1Plcadiag::from_bits(val as u8)
    }
    #[doc = "PLCA Diagnostics"]
    #[inline(always)]
    pub const fn set_plcadiag(&mut self, val: super::vals::Intencapt1Plcadiag) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "SMI Access"]
    #[must_use]
    #[inline(always)]
    pub const fn smiaccess(&self) -> super::vals::Intencapt1Smiaccess {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Intencapt1Smiaccess::from_bits(val as u8)
    }
    #[doc = "SMI Access"]
    #[inline(always)]
    pub const fn set_smiaccess(&mut self, val: super::vals::Intencapt1Smiaccess) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "Local Wake"]
    #[must_use]
    #[inline(always)]
    pub const fn lclwk(&self) -> super::vals::Intencapt1Lclwk {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Intencapt1Lclwk::from_bits(val as u8)
    }
    #[doc = "Local Wake"]
    #[inline(always)]
    pub const fn set_lclwk(&mut self, val: super::vals::Intencapt1Lclwk) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "Suaspend Symbol Detects"]
    #[must_use]
    #[inline(always)]
    pub const fn sspdet(&self) -> super::vals::Intencapt1Sspdet {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Intencapt1Sspdet::from_bits(val as u8)
    }
    #[doc = "Suaspend Symbol Detects"]
    #[inline(always)]
    pub const fn set_sspdet(&mut self, val: super::vals::Intencapt1Sspdet) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "WUT Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn wutdet(&self) -> super::vals::Intencapt1Wutdet {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Intencapt1Wutdet::from_bits(val as u8)
    }
    #[doc = "WUT Detect"]
    #[inline(always)]
    pub const fn set_wutdet(&mut self, val: super::vals::Intencapt1Wutdet) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "WUP transmission completed Write 1 clears bits."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdone(&self) -> super::vals::Intencapt1Wupdone {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Intencapt1Wupdone::from_bits(val as u8)
    }
    #[doc = "WUP transmission completed Write 1 clears bits."]
    #[inline(always)]
    pub const fn set_wupdone(&mut self, val: super::vals::Intencapt1Wupdone) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "APB Parity"]
    #[must_use]
    #[inline(always)]
    pub const fn apbparity(&self) -> super::vals::Intencapt1Apbparity {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Intencapt1Apbparity::from_bits(val as u8)
    }
    #[doc = "APB Parity"]
    #[inline(always)]
    pub const fn set_apbparity(&mut self, val: super::vals::Intencapt1Apbparity) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
}
impl Default for Intencapt1 {
    #[inline(always)]
    fn default() -> Intencapt1 {
        Intencapt1(0)
    }
}
impl core::fmt::Debug for Intencapt1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intencapt1")
            .field("physcol", &self.physcol())
            .field("plcarec", &self.plcarec())
            .field("plcastat", &self.plcastat())
            .field("modestat", &self.modestat())
            .field("invldapb", &self.invldapb())
            .field("locjab", &self.locjab())
            .field("remjab", &self.remjab())
            .field("pinfault", &self.pinfault())
            .field("plcadiag", &self.plcadiag())
            .field("smiaccess", &self.smiaccess())
            .field("lclwk", &self.lclwk())
            .field("sspdet", &self.sspdet())
            .field("wutdet", &self.wutdet())
            .field("wupdone", &self.wupdone())
            .field("apbparity", &self.apbparity())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intencapt1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intencapt1 {{ physcol: {:?}, plcarec: {:?}, plcastat: {:?}, modestat: {:?}, invldapb: {:?}, locjab: {:?}, remjab: {:?}, pinfault: {:?}, plcadiag: {:?}, smiaccess: {:?}, lclwk: {:?}, sspdet: {:?}, wutdet: {:?}, wupdone: {:?}, apbparity: {:?} }}",
            self.physcol(),
            self.plcarec(),
            self.plcastat(),
            self.modestat(),
            self.invldapb(),
            self.locjab(),
            self.remjab(),
            self.pinfault(),
            self.plcadiag(),
            self.smiaccess(),
            self.lclwk(),
            self.sspdet(),
            self.wutdet(),
            self.wupdone(),
            self.apbparity()
        )
    }
}
#[doc = "Interrupt Capture2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intencapt2(pub u16);
impl Intencapt2 {
    #[doc = "Physical Collision"]
    #[must_use]
    #[inline(always)]
    pub const fn physcol(&self) -> super::vals::Intencapt2Physcol {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Intencapt2Physcol::from_bits(val as u8)
    }
    #[doc = "Physical Collision"]
    #[inline(always)]
    pub const fn set_physcol(&mut self, val: super::vals::Intencapt2Physcol) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "PLCA Recovery"]
    #[must_use]
    #[inline(always)]
    pub const fn plcarec(&self) -> super::vals::Intencapt2Plcarec {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Intencapt2Plcarec::from_bits(val as u8)
    }
    #[doc = "PLCA Recovery"]
    #[inline(always)]
    pub const fn set_plcarec(&mut self, val: super::vals::Intencapt2Plcarec) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "PLCA Status"]
    #[must_use]
    #[inline(always)]
    pub const fn plcastat(&self) -> super::vals::Intencapt2Plcastat {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Intencapt2Plcastat::from_bits(val as u8)
    }
    #[doc = "PLCA Status"]
    #[inline(always)]
    pub const fn set_plcastat(&mut self, val: super::vals::Intencapt2Plcastat) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "MODE Status"]
    #[must_use]
    #[inline(always)]
    pub const fn modestat(&self) -> super::vals::Intencapt2Modestat {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Intencapt2Modestat::from_bits(val as u8)
    }
    #[doc = "MODE Status"]
    #[inline(always)]
    pub const fn set_modestat(&mut self, val: super::vals::Intencapt2Modestat) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "Invalid APB"]
    #[must_use]
    #[inline(always)]
    pub const fn invldapb(&self) -> super::vals::Intencapt2Invldapb {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Intencapt2Invldapb::from_bits(val as u8)
    }
    #[doc = "Invalid APB"]
    #[inline(always)]
    pub const fn set_invldapb(&mut self, val: super::vals::Intencapt2Invldapb) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Local Jabber"]
    #[must_use]
    #[inline(always)]
    pub const fn locjab(&self) -> super::vals::Intencapt2Locjab {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Intencapt2Locjab::from_bits(val as u8)
    }
    #[doc = "Local Jabber"]
    #[inline(always)]
    pub const fn set_locjab(&mut self, val: super::vals::Intencapt2Locjab) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "Remote Jabber"]
    #[must_use]
    #[inline(always)]
    pub const fn remjab(&self) -> super::vals::Intencapt2Remjab {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Intencapt2Remjab::from_bits(val as u8)
    }
    #[doc = "Remote Jabber"]
    #[inline(always)]
    pub const fn set_remjab(&mut self, val: super::vals::Intencapt2Remjab) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "PIN Fault Monitor"]
    #[must_use]
    #[inline(always)]
    pub const fn pinfault(&self) -> super::vals::Intencapt2Pinfault {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Intencapt2Pinfault::from_bits(val as u8)
    }
    #[doc = "PIN Fault Monitor"]
    #[inline(always)]
    pub const fn set_pinfault(&mut self, val: super::vals::Intencapt2Pinfault) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "PLCA Diagnostics"]
    #[must_use]
    #[inline(always)]
    pub const fn plcadiag(&self) -> super::vals::Intencapt2Plcadiag {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Intencapt2Plcadiag::from_bits(val as u8)
    }
    #[doc = "PLCA Diagnostics"]
    #[inline(always)]
    pub const fn set_plcadiag(&mut self, val: super::vals::Intencapt2Plcadiag) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "SMI Access"]
    #[must_use]
    #[inline(always)]
    pub const fn smiaccess(&self) -> super::vals::Intencapt2Smiaccess {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Intencapt2Smiaccess::from_bits(val as u8)
    }
    #[doc = "SMI Access"]
    #[inline(always)]
    pub const fn set_smiaccess(&mut self, val: super::vals::Intencapt2Smiaccess) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "Local Wake"]
    #[must_use]
    #[inline(always)]
    pub const fn lclwk(&self) -> super::vals::Intencapt2Lclwk {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Intencapt2Lclwk::from_bits(val as u8)
    }
    #[doc = "Local Wake"]
    #[inline(always)]
    pub const fn set_lclwk(&mut self, val: super::vals::Intencapt2Lclwk) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "Suaspend Symbol Detects"]
    #[must_use]
    #[inline(always)]
    pub const fn sspdet(&self) -> super::vals::Intencapt2Sspdet {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Intencapt2Sspdet::from_bits(val as u8)
    }
    #[doc = "Suaspend Symbol Detects"]
    #[inline(always)]
    pub const fn set_sspdet(&mut self, val: super::vals::Intencapt2Sspdet) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "WUT Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn wutdet(&self) -> super::vals::Intencapt2Wutdet {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Intencapt2Wutdet::from_bits(val as u8)
    }
    #[doc = "WUT Detect"]
    #[inline(always)]
    pub const fn set_wutdet(&mut self, val: super::vals::Intencapt2Wutdet) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "WUP transmission completed Write 1 clears bits."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdone(&self) -> super::vals::Intencapt2Wupdone {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Intencapt2Wupdone::from_bits(val as u8)
    }
    #[doc = "WUP transmission completed Write 1 clears bits."]
    #[inline(always)]
    pub const fn set_wupdone(&mut self, val: super::vals::Intencapt2Wupdone) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "APB Parity"]
    #[must_use]
    #[inline(always)]
    pub const fn apbparity(&self) -> super::vals::Intencapt2Apbparity {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Intencapt2Apbparity::from_bits(val as u8)
    }
    #[doc = "APB Parity"]
    #[inline(always)]
    pub const fn set_apbparity(&mut self, val: super::vals::Intencapt2Apbparity) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
}
impl Default for Intencapt2 {
    #[inline(always)]
    fn default() -> Intencapt2 {
        Intencapt2(0)
    }
}
impl core::fmt::Debug for Intencapt2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intencapt2")
            .field("physcol", &self.physcol())
            .field("plcarec", &self.plcarec())
            .field("plcastat", &self.plcastat())
            .field("modestat", &self.modestat())
            .field("invldapb", &self.invldapb())
            .field("locjab", &self.locjab())
            .field("remjab", &self.remjab())
            .field("pinfault", &self.pinfault())
            .field("plcadiag", &self.plcadiag())
            .field("smiaccess", &self.smiaccess())
            .field("lclwk", &self.lclwk())
            .field("sspdet", &self.sspdet())
            .field("wutdet", &self.wutdet())
            .field("wupdone", &self.wupdone())
            .field("apbparity", &self.apbparity())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intencapt2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intencapt2 {{ physcol: {:?}, plcarec: {:?}, plcastat: {:?}, modestat: {:?}, invldapb: {:?}, locjab: {:?}, remjab: {:?}, pinfault: {:?}, plcadiag: {:?}, smiaccess: {:?}, lclwk: {:?}, sspdet: {:?}, wutdet: {:?}, wupdone: {:?}, apbparity: {:?} }}",
            self.physcol(),
            self.plcarec(),
            self.plcastat(),
            self.modestat(),
            self.invldapb(),
            self.locjab(),
            self.remjab(),
            self.pinfault(),
            self.plcadiag(),
            self.smiaccess(),
            self.lclwk(),
            self.sspdet(),
            self.wutdet(),
            self.wupdone(),
            self.apbparity()
        )
    }
}
#[doc = "Interrupt Enable Clear1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenclr1(pub u16);
impl Intenclr1 {
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn physcol(&self) -> super::vals::Intenclr1Physcol {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Intenclr1Physcol::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_physcol(&mut self, val: super::vals::Intenclr1Physcol) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn plcarec(&self) -> super::vals::Intenclr1Plcarec {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Intenclr1Plcarec::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_plcarec(&mut self, val: super::vals::Intenclr1Plcarec) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn plcastat(&self) -> super::vals::Intenclr1Plcastat {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Intenclr1Plcastat::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_plcastat(&mut self, val: super::vals::Intenclr1Plcastat) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn modestat(&self) -> super::vals::Intenclr1Modestat {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Intenclr1Modestat::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_modestat(&mut self, val: super::vals::Intenclr1Modestat) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn invldapb(&self) -> super::vals::Intenclr1Invldapb {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Intenclr1Invldapb::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_invldapb(&mut self, val: super::vals::Intenclr1Invldapb) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn locjab(&self) -> super::vals::Intenclr1Locjab {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Intenclr1Locjab::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_locjab(&mut self, val: super::vals::Intenclr1Locjab) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn remjab(&self) -> super::vals::Intenclr1Remjab {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Intenclr1Remjab::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_remjab(&mut self, val: super::vals::Intenclr1Remjab) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn pinfault(&self) -> super::vals::Intenclr1Pinfault {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Intenclr1Pinfault::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_pinfault(&mut self, val: super::vals::Intenclr1Pinfault) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn plcadiag(&self) -> super::vals::Intenclr1Plcadiag {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Intenclr1Plcadiag::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_plcadiag(&mut self, val: super::vals::Intenclr1Plcadiag) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn smiaccess(&self) -> super::vals::Intenclr1Smiaccess {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Intenclr1Smiaccess::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_smiaccess(&mut self, val: super::vals::Intenclr1Smiaccess) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn lclwk(&self) -> super::vals::Intenclr1Lclwk {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Intenclr1Lclwk::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_lclwk(&mut self, val: super::vals::Intenclr1Lclwk) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn sspdet(&self) -> super::vals::Intenclr1Sspdet {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Intenclr1Sspdet::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_sspdet(&mut self, val: super::vals::Intenclr1Sspdet) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn wutdet(&self) -> super::vals::Intenclr1Wutdet {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Intenclr1Wutdet::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_wutdet(&mut self, val: super::vals::Intenclr1Wutdet) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn wupdone(&self) -> super::vals::Intenclr1Wupdone {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Intenclr1Wupdone::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_wupdone(&mut self, val: super::vals::Intenclr1Wupdone) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn apbparity(&self) -> super::vals::Intenclr1Apbparity {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Intenclr1Apbparity::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_apbparity(&mut self, val: super::vals::Intenclr1Apbparity) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
}
impl Default for Intenclr1 {
    #[inline(always)]
    fn default() -> Intenclr1 {
        Intenclr1(0)
    }
}
impl core::fmt::Debug for Intenclr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intenclr1")
            .field("physcol", &self.physcol())
            .field("plcarec", &self.plcarec())
            .field("plcastat", &self.plcastat())
            .field("modestat", &self.modestat())
            .field("invldapb", &self.invldapb())
            .field("locjab", &self.locjab())
            .field("remjab", &self.remjab())
            .field("pinfault", &self.pinfault())
            .field("plcadiag", &self.plcadiag())
            .field("smiaccess", &self.smiaccess())
            .field("lclwk", &self.lclwk())
            .field("sspdet", &self.sspdet())
            .field("wutdet", &self.wutdet())
            .field("wupdone", &self.wupdone())
            .field("apbparity", &self.apbparity())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intenclr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intenclr1 {{ physcol: {:?}, plcarec: {:?}, plcastat: {:?}, modestat: {:?}, invldapb: {:?}, locjab: {:?}, remjab: {:?}, pinfault: {:?}, plcadiag: {:?}, smiaccess: {:?}, lclwk: {:?}, sspdet: {:?}, wutdet: {:?}, wupdone: {:?}, apbparity: {:?} }}",
            self.physcol(),
            self.plcarec(),
            self.plcastat(),
            self.modestat(),
            self.invldapb(),
            self.locjab(),
            self.remjab(),
            self.pinfault(),
            self.plcadiag(),
            self.smiaccess(),
            self.lclwk(),
            self.sspdet(),
            self.wutdet(),
            self.wupdone(),
            self.apbparity()
        )
    }
}
#[doc = "Interrupt Enable Clear2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenclr2(pub u16);
impl Intenclr2 {
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn physcol(&self) -> super::vals::Intenclr2Physcol {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Intenclr2Physcol::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_physcol(&mut self, val: super::vals::Intenclr2Physcol) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn plcarec(&self) -> super::vals::Intenclr2Plcarec {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Intenclr2Plcarec::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_plcarec(&mut self, val: super::vals::Intenclr2Plcarec) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn plcastat(&self) -> super::vals::Intenclr2Plcastat {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Intenclr2Plcastat::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_plcastat(&mut self, val: super::vals::Intenclr2Plcastat) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn modestat(&self) -> super::vals::Intenclr2Modestat {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Intenclr2Modestat::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_modestat(&mut self, val: super::vals::Intenclr2Modestat) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn invldapb(&self) -> super::vals::Intenclr2Invldapb {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Intenclr2Invldapb::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_invldapb(&mut self, val: super::vals::Intenclr2Invldapb) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn locjab(&self) -> super::vals::Intenclr2Locjab {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Intenclr2Locjab::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_locjab(&mut self, val: super::vals::Intenclr2Locjab) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn remjab(&self) -> super::vals::Intenclr2Remjab {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Intenclr2Remjab::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_remjab(&mut self, val: super::vals::Intenclr2Remjab) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn pinfault(&self) -> super::vals::Intenclr2Pinfault {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Intenclr2Pinfault::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_pinfault(&mut self, val: super::vals::Intenclr2Pinfault) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn plcadiag(&self) -> super::vals::Intenclr2Plcadiag {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Intenclr2Plcadiag::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_plcadiag(&mut self, val: super::vals::Intenclr2Plcadiag) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn smiaccess(&self) -> super::vals::Intenclr2Smiaccess {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Intenclr2Smiaccess::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_smiaccess(&mut self, val: super::vals::Intenclr2Smiaccess) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn lclwk(&self) -> super::vals::Intenclr2Lclwk {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Intenclr2Lclwk::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_lclwk(&mut self, val: super::vals::Intenclr2Lclwk) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn sspdet(&self) -> super::vals::Intenclr2Sspdet {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Intenclr2Sspdet::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_sspdet(&mut self, val: super::vals::Intenclr2Sspdet) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn wutdet(&self) -> super::vals::Intenclr2Wutdet {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Intenclr2Wutdet::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_wutdet(&mut self, val: super::vals::Intenclr2Wutdet) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn wupdone(&self) -> super::vals::Intenclr2Wupdone {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Intenclr2Wupdone::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_wupdone(&mut self, val: super::vals::Intenclr2Wupdone) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn apbparity(&self) -> super::vals::Intenclr2Apbparity {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Intenclr2Apbparity::from_bits(val as u8)
    }
    #[doc = "write 1 disables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_apbparity(&mut self, val: super::vals::Intenclr2Apbparity) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
}
impl Default for Intenclr2 {
    #[inline(always)]
    fn default() -> Intenclr2 {
        Intenclr2(0)
    }
}
impl core::fmt::Debug for Intenclr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intenclr2")
            .field("physcol", &self.physcol())
            .field("plcarec", &self.plcarec())
            .field("plcastat", &self.plcastat())
            .field("modestat", &self.modestat())
            .field("invldapb", &self.invldapb())
            .field("locjab", &self.locjab())
            .field("remjab", &self.remjab())
            .field("pinfault", &self.pinfault())
            .field("plcadiag", &self.plcadiag())
            .field("smiaccess", &self.smiaccess())
            .field("lclwk", &self.lclwk())
            .field("sspdet", &self.sspdet())
            .field("wutdet", &self.wutdet())
            .field("wupdone", &self.wupdone())
            .field("apbparity", &self.apbparity())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intenclr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intenclr2 {{ physcol: {:?}, plcarec: {:?}, plcastat: {:?}, modestat: {:?}, invldapb: {:?}, locjab: {:?}, remjab: {:?}, pinfault: {:?}, plcadiag: {:?}, smiaccess: {:?}, lclwk: {:?}, sspdet: {:?}, wutdet: {:?}, wupdone: {:?}, apbparity: {:?} }}",
            self.physcol(),
            self.plcarec(),
            self.plcastat(),
            self.modestat(),
            self.invldapb(),
            self.locjab(),
            self.remjab(),
            self.pinfault(),
            self.plcadiag(),
            self.smiaccess(),
            self.lclwk(),
            self.sspdet(),
            self.wutdet(),
            self.wupdone(),
            self.apbparity()
        )
    }
}
#[doc = "Interrupt Enable Set1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenset1(pub u16);
impl Intenset1 {
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn physcol(&self) -> super::vals::Intenset1Physcol {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Intenset1Physcol::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_physcol(&mut self, val: super::vals::Intenset1Physcol) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn plcarec(&self) -> super::vals::Intenset1Plcarec {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Intenset1Plcarec::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_plcarec(&mut self, val: super::vals::Intenset1Plcarec) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn plcastat(&self) -> super::vals::Intenset1Plcastat {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Intenset1Plcastat::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_plcastat(&mut self, val: super::vals::Intenset1Plcastat) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Mode status"]
    #[must_use]
    #[inline(always)]
    pub const fn modestat(&self) -> super::vals::Intenset1Modestat {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Intenset1Modestat::from_bits(val as u8)
    }
    #[doc = "Mode status"]
    #[inline(always)]
    pub const fn set_modestat(&mut self, val: super::vals::Intenset1Modestat) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn invldapb(&self) -> super::vals::Intenset1Invldapb {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Intenset1Invldapb::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_invldapb(&mut self, val: super::vals::Intenset1Invldapb) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn locjab(&self) -> super::vals::Intenset1Locjab {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Intenset1Locjab::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_locjab(&mut self, val: super::vals::Intenset1Locjab) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn remjab(&self) -> super::vals::Intenset1Remjab {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Intenset1Remjab::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_remjab(&mut self, val: super::vals::Intenset1Remjab) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn pinfault(&self) -> super::vals::Intenset1Pinfault {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Intenset1Pinfault::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_pinfault(&mut self, val: super::vals::Intenset1Pinfault) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn plcadiag(&self) -> super::vals::Intenset1Plcadiag {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Intenset1Plcadiag::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_plcadiag(&mut self, val: super::vals::Intenset1Plcadiag) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn smiaccess(&self) -> super::vals::Intenset1Smiaccess {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Intenset1Smiaccess::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_smiaccess(&mut self, val: super::vals::Intenset1Smiaccess) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn lclwk(&self) -> super::vals::Intenset1Lclwk {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Intenset1Lclwk::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_lclwk(&mut self, val: super::vals::Intenset1Lclwk) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn sspdet(&self) -> super::vals::Intenset1Sspdet {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Intenset1Sspdet::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_sspdet(&mut self, val: super::vals::Intenset1Sspdet) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn wutdet(&self) -> super::vals::Intenset1Wutdet {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Intenset1Wutdet::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_wutdet(&mut self, val: super::vals::Intenset1Wutdet) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn wupdone(&self) -> super::vals::Intenset1Wupdone {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Intenset1Wupdone::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_wupdone(&mut self, val: super::vals::Intenset1Wupdone) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn apbparity(&self) -> super::vals::Intenset1Apbparity {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Intenset1Apbparity::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_apbparity(&mut self, val: super::vals::Intenset1Apbparity) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
}
impl Default for Intenset1 {
    #[inline(always)]
    fn default() -> Intenset1 {
        Intenset1(0)
    }
}
impl core::fmt::Debug for Intenset1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intenset1")
            .field("physcol", &self.physcol())
            .field("plcarec", &self.plcarec())
            .field("plcastat", &self.plcastat())
            .field("modestat", &self.modestat())
            .field("invldapb", &self.invldapb())
            .field("locjab", &self.locjab())
            .field("remjab", &self.remjab())
            .field("pinfault", &self.pinfault())
            .field("plcadiag", &self.plcadiag())
            .field("smiaccess", &self.smiaccess())
            .field("lclwk", &self.lclwk())
            .field("sspdet", &self.sspdet())
            .field("wutdet", &self.wutdet())
            .field("wupdone", &self.wupdone())
            .field("apbparity", &self.apbparity())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intenset1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intenset1 {{ physcol: {:?}, plcarec: {:?}, plcastat: {:?}, modestat: {:?}, invldapb: {:?}, locjab: {:?}, remjab: {:?}, pinfault: {:?}, plcadiag: {:?}, smiaccess: {:?}, lclwk: {:?}, sspdet: {:?}, wutdet: {:?}, wupdone: {:?}, apbparity: {:?} }}",
            self.physcol(),
            self.plcarec(),
            self.plcastat(),
            self.modestat(),
            self.invldapb(),
            self.locjab(),
            self.remjab(),
            self.pinfault(),
            self.plcadiag(),
            self.smiaccess(),
            self.lclwk(),
            self.sspdet(),
            self.wutdet(),
            self.wupdone(),
            self.apbparity()
        )
    }
}
#[doc = "Interrupt Enable Set1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenset2(pub u16);
impl Intenset2 {
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn physcol(&self) -> super::vals::Intenset2Physcol {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Intenset2Physcol::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_physcol(&mut self, val: super::vals::Intenset2Physcol) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn plcarec(&self) -> super::vals::Intenset2Plcarec {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Intenset2Plcarec::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_plcarec(&mut self, val: super::vals::Intenset2Plcarec) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn plcastat(&self) -> super::vals::Intenset2Plcastat {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Intenset2Plcastat::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_plcastat(&mut self, val: super::vals::Intenset2Plcastat) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Mode status"]
    #[must_use]
    #[inline(always)]
    pub const fn modestat(&self) -> super::vals::Intenset2Modestat {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Intenset2Modestat::from_bits(val as u8)
    }
    #[doc = "Mode status"]
    #[inline(always)]
    pub const fn set_modestat(&mut self, val: super::vals::Intenset2Modestat) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn invldapb(&self) -> super::vals::Intenset2Invldapb {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Intenset2Invldapb::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_invldapb(&mut self, val: super::vals::Intenset2Invldapb) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn locjab(&self) -> super::vals::Intenset2Locjab {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Intenset2Locjab::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_locjab(&mut self, val: super::vals::Intenset2Locjab) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn remjab(&self) -> super::vals::Intenset2Remjab {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Intenset2Remjab::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_remjab(&mut self, val: super::vals::Intenset2Remjab) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn pinfault(&self) -> super::vals::Intenset2Pinfault {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Intenset2Pinfault::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_pinfault(&mut self, val: super::vals::Intenset2Pinfault) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn plcadiag(&self) -> super::vals::Intenset2Plcadiag {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Intenset2Plcadiag::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_plcadiag(&mut self, val: super::vals::Intenset2Plcadiag) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn smiaccess(&self) -> super::vals::Intenset2Smiaccess {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Intenset2Smiaccess::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_smiaccess(&mut self, val: super::vals::Intenset2Smiaccess) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn lclwk(&self) -> super::vals::Intenset2Lclwk {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Intenset2Lclwk::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_lclwk(&mut self, val: super::vals::Intenset2Lclwk) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn sspdet(&self) -> super::vals::Intenset2Sspdet {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Intenset2Sspdet::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_sspdet(&mut self, val: super::vals::Intenset2Sspdet) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn wutdet(&self) -> super::vals::Intenset2Wutdet {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Intenset2Wutdet::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_wutdet(&mut self, val: super::vals::Intenset2Wutdet) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn wupdone(&self) -> super::vals::Intenset2Wupdone {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Intenset2Wupdone::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_wupdone(&mut self, val: super::vals::Intenset2Wupdone) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn apbparity(&self) -> super::vals::Intenset2Apbparity {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Intenset2Apbparity::from_bits(val as u8)
    }
    #[doc = "write 1 enables corresponding interrupt"]
    #[inline(always)]
    pub const fn set_apbparity(&mut self, val: super::vals::Intenset2Apbparity) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
}
impl Default for Intenset2 {
    #[inline(always)]
    fn default() -> Intenset2 {
        Intenset2(0)
    }
}
impl core::fmt::Debug for Intenset2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intenset2")
            .field("physcol", &self.physcol())
            .field("plcarec", &self.plcarec())
            .field("plcastat", &self.plcastat())
            .field("modestat", &self.modestat())
            .field("invldapb", &self.invldapb())
            .field("locjab", &self.locjab())
            .field("remjab", &self.remjab())
            .field("pinfault", &self.pinfault())
            .field("plcadiag", &self.plcadiag())
            .field("smiaccess", &self.smiaccess())
            .field("lclwk", &self.lclwk())
            .field("sspdet", &self.sspdet())
            .field("wutdet", &self.wutdet())
            .field("wupdone", &self.wupdone())
            .field("apbparity", &self.apbparity())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intenset2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intenset2 {{ physcol: {:?}, plcarec: {:?}, plcastat: {:?}, modestat: {:?}, invldapb: {:?}, locjab: {:?}, remjab: {:?}, pinfault: {:?}, plcadiag: {:?}, smiaccess: {:?}, lclwk: {:?}, sspdet: {:?}, wutdet: {:?}, wupdone: {:?}, apbparity: {:?} }}",
            self.physcol(),
            self.plcarec(),
            self.plcastat(),
            self.modestat(),
            self.invldapb(),
            self.locjab(),
            self.remjab(),
            self.pinfault(),
            self.plcadiag(),
            self.smiaccess(),
            self.lclwk(),
            self.sspdet(),
            self.wutdet(),
            self.wupdone(),
            self.apbparity()
        )
    }
}
#[doc = "Interrupt Status1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstat1(pub u16);
impl Intstat1 {
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn physcol(&self) -> super::vals::Intstat1Physcol {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Intstat1Physcol::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_physcol(&mut self, val: super::vals::Intstat1Physcol) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn plcarec(&self) -> super::vals::Intstat1Plcarec {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Intstat1Plcarec::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_plcarec(&mut self, val: super::vals::Intstat1Plcarec) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn plcastat(&self) -> super::vals::Intstat1Plcastat {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Intstat1Plcastat::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_plcastat(&mut self, val: super::vals::Intstat1Plcastat) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn modestat(&self) -> super::vals::Intstat1Modestat {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Intstat1Modestat::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_modestat(&mut self, val: super::vals::Intstat1Modestat) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn invldapb(&self) -> super::vals::Intstat1Invldapb {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Intstat1Invldapb::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_invldapb(&mut self, val: super::vals::Intstat1Invldapb) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn locjab(&self) -> super::vals::Intstat1Locjab {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Intstat1Locjab::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_locjab(&mut self, val: super::vals::Intstat1Locjab) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn remjab(&self) -> super::vals::Intstat1Remjab {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Intstat1Remjab::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_remjab(&mut self, val: super::vals::Intstat1Remjab) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn pinfault(&self) -> super::vals::Intstat1Pinfault {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Intstat1Pinfault::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_pinfault(&mut self, val: super::vals::Intstat1Pinfault) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn plcadiag(&self) -> super::vals::Intstat1Plcadiag {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Intstat1Plcadiag::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_plcadiag(&mut self, val: super::vals::Intstat1Plcadiag) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn smiaccess(&self) -> super::vals::Intstat1Smiaccess {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Intstat1Smiaccess::from_bits(val as u8)
    }
    #[doc = "status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_smiaccess(&mut self, val: super::vals::Intstat1Smiaccess) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn lclwk(&self) -> super::vals::Intstat1Lclwk {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Intstat1Lclwk::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_lclwk(&mut self, val: super::vals::Intstat1Lclwk) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn sspdet(&self) -> super::vals::Intstat1Sspdet {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Intstat1Sspdet::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_sspdet(&mut self, val: super::vals::Intstat1Sspdet) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn wutdet(&self) -> super::vals::Intstat1Wutdet {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Intstat1Wutdet::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_wutdet(&mut self, val: super::vals::Intstat1Wutdet) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn wupdone(&self) -> super::vals::Intstat1Wupdone {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Intstat1Wupdone::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_wupdone(&mut self, val: super::vals::Intstat1Wupdone) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn apbparity(&self) -> super::vals::Intstat1Apbparity {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Intstat1Apbparity::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_apbparity(&mut self, val: super::vals::Intstat1Apbparity) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
}
impl Default for Intstat1 {
    #[inline(always)]
    fn default() -> Intstat1 {
        Intstat1(0)
    }
}
impl core::fmt::Debug for Intstat1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intstat1")
            .field("physcol", &self.physcol())
            .field("plcarec", &self.plcarec())
            .field("plcastat", &self.plcastat())
            .field("modestat", &self.modestat())
            .field("invldapb", &self.invldapb())
            .field("locjab", &self.locjab())
            .field("remjab", &self.remjab())
            .field("pinfault", &self.pinfault())
            .field("plcadiag", &self.plcadiag())
            .field("smiaccess", &self.smiaccess())
            .field("lclwk", &self.lclwk())
            .field("sspdet", &self.sspdet())
            .field("wutdet", &self.wutdet())
            .field("wupdone", &self.wupdone())
            .field("apbparity", &self.apbparity())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intstat1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intstat1 {{ physcol: {:?}, plcarec: {:?}, plcastat: {:?}, modestat: {:?}, invldapb: {:?}, locjab: {:?}, remjab: {:?}, pinfault: {:?}, plcadiag: {:?}, smiaccess: {:?}, lclwk: {:?}, sspdet: {:?}, wutdet: {:?}, wupdone: {:?}, apbparity: {:?} }}",
            self.physcol(),
            self.plcarec(),
            self.plcastat(),
            self.modestat(),
            self.invldapb(),
            self.locjab(),
            self.remjab(),
            self.pinfault(),
            self.plcadiag(),
            self.smiaccess(),
            self.lclwk(),
            self.sspdet(),
            self.wutdet(),
            self.wupdone(),
            self.apbparity()
        )
    }
}
#[doc = "Interrupt Status2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstat2(pub u16);
impl Intstat2 {
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn physcol(&self) -> super::vals::Intstat2Physcol {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Intstat2Physcol::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_physcol(&mut self, val: super::vals::Intstat2Physcol) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn plcarec(&self) -> super::vals::Intstat2Plcarec {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Intstat2Plcarec::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_plcarec(&mut self, val: super::vals::Intstat2Plcarec) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn plcastat(&self) -> super::vals::Intstat2Plcastat {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Intstat2Plcastat::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_plcastat(&mut self, val: super::vals::Intstat2Plcastat) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn modestat(&self) -> super::vals::Intstat2Modestat {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Intstat2Modestat::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_modestat(&mut self, val: super::vals::Intstat2Modestat) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn invldapb(&self) -> super::vals::Intstat2Invldapb {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Intstat2Invldapb::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_invldapb(&mut self, val: super::vals::Intstat2Invldapb) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn locjab(&self) -> super::vals::Intstat2Locjab {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Intstat2Locjab::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_locjab(&mut self, val: super::vals::Intstat2Locjab) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn remjab(&self) -> super::vals::Intstat2Remjab {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Intstat2Remjab::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_remjab(&mut self, val: super::vals::Intstat2Remjab) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn pinfault(&self) -> super::vals::Intstat2Pinfault {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Intstat2Pinfault::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_pinfault(&mut self, val: super::vals::Intstat2Pinfault) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn plcadiag(&self) -> super::vals::Intstat2Plcadiag {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Intstat2Plcadiag::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_plcadiag(&mut self, val: super::vals::Intstat2Plcadiag) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn smiaccess(&self) -> super::vals::Intstat2Smiaccess {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Intstat2Smiaccess::from_bits(val as u8)
    }
    #[doc = "status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_smiaccess(&mut self, val: super::vals::Intstat2Smiaccess) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn lclwk(&self) -> super::vals::Intstat2Lclwk {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Intstat2Lclwk::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_lclwk(&mut self, val: super::vals::Intstat2Lclwk) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn sspdet(&self) -> super::vals::Intstat2Sspdet {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Intstat2Sspdet::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_sspdet(&mut self, val: super::vals::Intstat2Sspdet) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn wutdet(&self) -> super::vals::Intstat2Wutdet {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Intstat2Wutdet::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_wutdet(&mut self, val: super::vals::Intstat2Wutdet) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn wupdone(&self) -> super::vals::Intstat2Wupdone {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Intstat2Wupdone::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_wupdone(&mut self, val: super::vals::Intstat2Wupdone) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[must_use]
    #[inline(always)]
    pub const fn apbparity(&self) -> super::vals::Intstat2Apbparity {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Intstat2Apbparity::from_bits(val as u8)
    }
    #[doc = "Status of corresponding interrupt after enable stage"]
    #[inline(always)]
    pub const fn set_apbparity(&mut self, val: super::vals::Intstat2Apbparity) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
}
impl Default for Intstat2 {
    #[inline(always)]
    fn default() -> Intstat2 {
        Intstat2(0)
    }
}
impl core::fmt::Debug for Intstat2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intstat2")
            .field("physcol", &self.physcol())
            .field("plcarec", &self.plcarec())
            .field("plcastat", &self.plcastat())
            .field("modestat", &self.modestat())
            .field("invldapb", &self.invldapb())
            .field("locjab", &self.locjab())
            .field("remjab", &self.remjab())
            .field("pinfault", &self.pinfault())
            .field("plcadiag", &self.plcadiag())
            .field("smiaccess", &self.smiaccess())
            .field("lclwk", &self.lclwk())
            .field("sspdet", &self.sspdet())
            .field("wutdet", &self.wutdet())
            .field("wupdone", &self.wupdone())
            .field("apbparity", &self.apbparity())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intstat2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intstat2 {{ physcol: {:?}, plcarec: {:?}, plcastat: {:?}, modestat: {:?}, invldapb: {:?}, locjab: {:?}, remjab: {:?}, pinfault: {:?}, plcadiag: {:?}, smiaccess: {:?}, lclwk: {:?}, sspdet: {:?}, wutdet: {:?}, wupdone: {:?}, apbparity: {:?} }}",
            self.physcol(),
            self.plcarec(),
            self.plcastat(),
            self.modestat(),
            self.invldapb(),
            self.locjab(),
            self.remjab(),
            self.pinfault(),
            self.plcadiag(),
            self.smiaccess(),
            self.lclwk(),
            self.sspdet(),
            self.wutdet(),
            self.wupdone(),
            self.apbparity()
        )
    }
}
#[doc = "Mode Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Modecfg(pub u16);
impl Modecfg {
    #[doc = "Enable wake-up to WAITINIT mode by local-wake-event"]
    #[must_use]
    #[inline(always)]
    pub const fn lclwkena(&self) -> super::vals::Lclwkena {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lclwkena::from_bits(val as u8)
    }
    #[doc = "Enable wake-up to WAITINIT mode by local-wake-event"]
    #[inline(always)]
    pub const fn set_lclwkena(&mut self, val: super::vals::Lclwkena) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Enable wake-up to WAITINIT mode by remote-wake-event"]
    #[must_use]
    #[inline(always)]
    pub const fn remwkena(&self) -> super::vals::Remwkena {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Remwkena::from_bits(val as u8)
    }
    #[doc = "Enable wake-up to WAITINIT mode by remote-wake-event"]
    #[inline(always)]
    pub const fn set_remwkena(&mut self, val: super::vals::Remwkena) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Enable wake-up to WAITINIT mode by SUSPEND detection"]
    #[must_use]
    #[inline(always)]
    pub const fn sspwkena(&self) -> super::vals::Sspwkena {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sspwkena::from_bits(val as u8)
    }
    #[doc = "Enable wake-up to WAITINIT mode by SUSPEND detection"]
    #[inline(always)]
    pub const fn set_sspwkena(&mut self, val: super::vals::Sspwkena) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
}
impl Default for Modecfg {
    #[inline(always)]
    fn default() -> Modecfg {
        Modecfg(0)
    }
}
impl core::fmt::Debug for Modecfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Modecfg")
            .field("lclwkena", &self.lclwkena())
            .field("remwkena", &self.remwkena())
            .field("sspwkena", &self.sspwkena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Modecfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Modecfg {{ lclwkena: {:?}, remwkena: {:?}, sspwkena: {:?} }}",
            self.lclwkena(),
            self.remwkena(),
            self.sspwkena()
        )
    }
}
#[doc = "MODE Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Modectrl(pub u16);
impl Modectrl {
    #[doc = "Set back to NONE when mode controller has entered the selected mode"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd(&self) -> super::vals::Cmd {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Cmd::from_bits(val as u8)
    }
    #[doc = "Set back to NONE when mode controller has entered the selected mode"]
    #[inline(always)]
    pub const fn set_cmd(&mut self, val: super::vals::Cmd) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u16) & 0x07) << 0usize);
    }
}
impl Default for Modectrl {
    #[inline(always)]
    fn default() -> Modectrl {
        Modectrl(0)
    }
}
impl core::fmt::Debug for Modectrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Modectrl")
            .field("cmd", &self.cmd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Modectrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Modectrl {{ cmd: {:?} }}", self.cmd())
    }
}
#[doc = "Mode Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Modestat(pub u16);
impl Modestat {
    #[doc = "Status power-mode sequencer"]
    #[must_use]
    #[inline(always)]
    pub const fn stat(&self) -> super::vals::ModestatStat {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::ModestatStat::from_bits(val as u8)
    }
    #[doc = "Status power-mode sequencer"]
    #[inline(always)]
    pub const fn set_stat(&mut self, val: super::vals::ModestatStat) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
    #[doc = "Silent Time-out condition"]
    #[must_use]
    #[inline(always)]
    pub const fn silentto(&self) -> super::vals::Silentto {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Silentto::from_bits(val as u8)
    }
    #[doc = "Silent Time-out condition"]
    #[inline(always)]
    pub const fn set_silentto(&mut self, val: super::vals::Silentto) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
}
impl Default for Modestat {
    #[inline(always)]
    fn default() -> Modestat {
        Modestat(0)
    }
}
impl core::fmt::Debug for Modestat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Modestat")
            .field("stat", &self.stat())
            .field("silentto", &self.silentto())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Modestat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Modestat {{ stat: {:?}, silentto: {:?} }}",
            self.stat(),
            self.silentto()
        )
    }
}
#[doc = "Mask Control PLCADIAG1 Flag"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mskplcadiag1(pub u16);
impl Mskplcadiag1 {
    #[doc = "Mask control for BCNBFTO flag"]
    #[must_use]
    #[inline(always)]
    pub const fn bcnbfto(&self) -> super::vals::Mskplcadiag1Bcnbfto {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Mskplcadiag1Bcnbfto::from_bits(val as u8)
    }
    #[doc = "Mask control for BCNBFTO flag"]
    #[inline(always)]
    pub const fn set_bcnbfto(&mut self, val: super::vals::Mskplcadiag1Bcnbfto) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Mask control for UNEXPB flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unexpb(&self) -> super::vals::Mskplcadiag1Unexpb {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Mskplcadiag1Unexpb::from_bits(val as u8)
    }
    #[doc = "Mask control for UNEXPB flag"]
    #[inline(always)]
    pub const fn set_unexpb(&mut self, val: super::vals::Mskplcadiag1Unexpb) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Mask control for RXINTO flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rxinto(&self) -> super::vals::Mskplcadiag1Rxinto {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Mskplcadiag1Rxinto::from_bits(val as u8)
    }
    #[doc = "Mask control for RXINTO flag"]
    #[inline(always)]
    pub const fn set_rxinto(&mut self, val: super::vals::Mskplcadiag1Rxinto) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
}
impl Default for Mskplcadiag1 {
    #[inline(always)]
    fn default() -> Mskplcadiag1 {
        Mskplcadiag1(0)
    }
}
impl core::fmt::Debug for Mskplcadiag1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mskplcadiag1")
            .field("bcnbfto", &self.bcnbfto())
            .field("unexpb", &self.unexpb())
            .field("rxinto", &self.rxinto())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mskplcadiag1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mskplcadiag1 {{ bcnbfto: {:?}, unexpb: {:?}, rxinto: {:?} }}",
            self.bcnbfto(),
            self.unexpb(),
            self.rxinto()
        )
    }
}
#[doc = "Mask Control PLCADIAG2 Flags"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mskplcadiag2(pub u16);
impl Mskplcadiag2 {
    #[doc = "Early Beacon Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn earlybcn(&self) -> super::vals::Mskplcadiag2Earlybcn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Mskplcadiag2Earlybcn::from_bits(val as u8)
    }
    #[doc = "Early Beacon Flag"]
    #[inline(always)]
    pub const fn set_earlybcn(&mut self, val: super::vals::Mskplcadiag2Earlybcn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Late Beacon"]
    #[must_use]
    #[inline(always)]
    pub const fn latebcn(&self) -> super::vals::Mskplcadiag2Latebcn {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Mskplcadiag2Latebcn::from_bits(val as u8)
    }
    #[doc = "Late Beacon"]
    #[inline(always)]
    pub const fn set_latebcn(&mut self, val: super::vals::Mskplcadiag2Latebcn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "NO RX Beacon"]
    #[must_use]
    #[inline(always)]
    pub const fn norxbcn(&self) -> super::vals::Mskplcadiag2Norxbcn {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Mskplcadiag2Norxbcn::from_bits(val as u8)
    }
    #[doc = "NO RX Beacon"]
    #[inline(always)]
    pub const fn set_norxbcn(&mut self, val: super::vals::Mskplcadiag2Norxbcn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "UNDEF State"]
    #[must_use]
    #[inline(always)]
    pub const fn undefstate(&self) -> super::vals::Mskplcadiag2Undefstate {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mskplcadiag2Undefstate::from_bits(val as u8)
    }
    #[doc = "UNDEF State"]
    #[inline(always)]
    pub const fn set_undefstate(&mut self, val: super::vals::Mskplcadiag2Undefstate) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
}
impl Default for Mskplcadiag2 {
    #[inline(always)]
    fn default() -> Mskplcadiag2 {
        Mskplcadiag2(0)
    }
}
impl core::fmt::Debug for Mskplcadiag2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mskplcadiag2")
            .field("earlybcn", &self.earlybcn())
            .field("latebcn", &self.latebcn())
            .field("norxbcn", &self.norxbcn())
            .field("undefstate", &self.undefstate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mskplcadiag2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mskplcadiag2 {{ earlybcn: {:?}, latebcn: {:?}, norxbcn: {:?}, undefstate: {:?} }}",
            self.earlybcn(),
            self.latebcn(),
            self.norxbcn(),
            self.undefstate()
        )
    }
}
#[doc = "Mask Control TXCDIAG Flag"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msktxcdiag(pub u16);
impl Msktxcdiag {
    #[doc = "RX Low Fail"]
    #[must_use]
    #[inline(always)]
    pub const fn rxlowfail(&self) -> super::vals::MsktxcdiagRxlowfail {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MsktxcdiagRxlowfail::from_bits(val as u8)
    }
    #[doc = "RX Low Fail"]
    #[inline(always)]
    pub const fn set_rxlowfail(&mut self, val: super::vals::MsktxcdiagRxlowfail) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "RX High Fail"]
    #[must_use]
    #[inline(always)]
    pub const fn rxhghfail(&self) -> super::vals::MsktxcdiagRxhghfail {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::MsktxcdiagRxhghfail::from_bits(val as u8)
    }
    #[doc = "RX High Fail"]
    #[inline(always)]
    pub const fn set_rxhghfail(&mut self, val: super::vals::MsktxcdiagRxhghfail) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "ED Low Fail"]
    #[must_use]
    #[inline(always)]
    pub const fn edlowfail(&self) -> super::vals::MsktxcdiagEdlowfail {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::MsktxcdiagEdlowfail::from_bits(val as u8)
    }
    #[doc = "ED Low Fail"]
    #[inline(always)]
    pub const fn set_edlowfail(&mut self, val: super::vals::MsktxcdiagEdlowfail) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "ED High Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn edhghfail(&self) -> super::vals::MsktxcdiagEdhghfail {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::MsktxcdiagEdhghfail::from_bits(val as u8)
    }
    #[doc = "ED High Flag"]
    #[inline(always)]
    pub const fn set_edhghfail(&mut self, val: super::vals::MsktxcdiagEdhghfail) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "Low Power Fail"]
    #[must_use]
    #[inline(always)]
    pub const fn lpwrfail(&self) -> super::vals::MsktxcdiagLpwrfail {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::MsktxcdiagLpwrfail::from_bits(val as u8)
    }
    #[doc = "Low Power Fail"]
    #[inline(always)]
    pub const fn set_lpwrfail(&mut self, val: super::vals::MsktxcdiagLpwrfail) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
}
impl Default for Msktxcdiag {
    #[inline(always)]
    fn default() -> Msktxcdiag {
        Msktxcdiag(0)
    }
}
impl core::fmt::Debug for Msktxcdiag {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Msktxcdiag")
            .field("rxlowfail", &self.rxlowfail())
            .field("rxhghfail", &self.rxhghfail())
            .field("edlowfail", &self.edlowfail())
            .field("edhghfail", &self.edhghfail())
            .field("lpwrfail", &self.lpwrfail())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Msktxcdiag {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Msktxcdiag {{ rxlowfail: {:?}, rxhghfail: {:?}, edlowfail: {:?}, edhghfail: {:?}, lpwrfail: {:?} }}",
            self.rxlowfail(),
            self.rxhghfail(),
            self.edlowfail(),
            self.edhghfail(),
            self.lpwrfail()
        )
    }
}
#[doc = "PCS Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcsctrl(pub u16);
impl Pcsctrl {
    #[doc = "Duplex"]
    #[must_use]
    #[inline(always)]
    pub const fn duplx(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Duplex"]
    #[inline(always)]
    pub const fn set_duplx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Loop"]
    #[must_use]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::PcsctrlLoop {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::PcsctrlLoop::from_bits(val as u8)
    }
    #[doc = "Loop"]
    #[inline(always)]
    pub const fn set_loop_(&mut self, val: super::vals::PcsctrlLoop) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn rst(&self) -> super::vals::Rst {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Rst::from_bits(val as u8)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub const fn set_rst(&mut self, val: super::vals::Rst) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for Pcsctrl {
    #[inline(always)]
    fn default() -> Pcsctrl {
        Pcsctrl(0)
    }
}
impl core::fmt::Debug for Pcsctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcsctrl")
            .field("duplx", &self.duplx())
            .field("loop_", &self.loop_())
            .field("rst", &self.rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcsctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcsctrl {{ duplx: {=bool:?}, loop_: {:?}, rst: {:?} }}",
            self.duplx(),
            self.loop_(),
            self.rst()
        )
    }
}
#[doc = "PCS Remote Jabber Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcsdiag1(pub u16);
impl Pcsdiag1 {
    #[doc = "Remote Jabber Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn remjabcnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Remote Jabber Counter"]
    #[inline(always)]
    pub const fn set_remjabcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Pcsdiag1 {
    #[inline(always)]
    fn default() -> Pcsdiag1 {
        Pcsdiag1(0)
    }
}
impl core::fmt::Debug for Pcsdiag1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcsdiag1")
            .field("remjabcnt", &self.remjabcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcsdiag1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pcsdiag1 {{ remjabcnt: {=u16:?} }}", self.remjabcnt())
    }
}
#[doc = "PCS Physical Collisions Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcsdiag2(pub u16);
impl Pcsdiag2 {
    #[doc = "Physical Collisions Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn phycolcnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Physical Collisions Counter"]
    #[inline(always)]
    pub const fn set_phycolcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Pcsdiag2 {
    #[inline(always)]
    fn default() -> Pcsdiag2 {
        Pcsdiag2(0)
    }
}
impl core::fmt::Debug for Pcsdiag2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcsdiag2")
            .field("phycolcnt", &self.phycolcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcsdiag2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pcsdiag2 {{ phycolcnt: {=u16:?} }}", self.phycolcnt())
    }
}
#[doc = "PCS Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcsstat(pub u16);
impl Pcsstat {
    #[doc = "Jabber condition"]
    #[must_use]
    #[inline(always)]
    pub const fn jab(&self) -> super::vals::Jab {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Jab::from_bits(val as u8)
    }
    #[doc = "Jabber condition"]
    #[inline(always)]
    pub const fn set_jab(&mut self, val: super::vals::Jab) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
}
impl Default for Pcsstat {
    #[inline(always)]
    fn default() -> Pcsstat {
        Pcsstat(0)
    }
}
impl core::fmt::Debug for Pcsstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcsstat").field("jab", &self.jab()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcsstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pcsstat {{ jab: {:?} }}", self.jab())
    }
}
#[doc = "PHY control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Phyctrl(pub u16);
impl Phyctrl {
    #[doc = "Speed Selection1"]
    #[must_use]
    #[inline(always)]
    pub const fn spd1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Speed Selection1"]
    #[inline(always)]
    pub const fn set_spd1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Collision Test"]
    #[must_use]
    #[inline(always)]
    pub const fn ctest(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Collision Test"]
    #[inline(always)]
    pub const fn set_ctest(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Duplex Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn duplx(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Duplex Mode"]
    #[inline(always)]
    pub const fn set_duplx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Speed Selection0"]
    #[must_use]
    #[inline(always)]
    pub const fn spd0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Speed Selection0"]
    #[inline(always)]
    pub const fn set_spd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "RESET"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "RESET"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Phyctrl {
    #[inline(always)]
    fn default() -> Phyctrl {
        Phyctrl(0)
    }
}
impl core::fmt::Debug for Phyctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Phyctrl")
            .field("spd1", &self.spd1())
            .field("ctest", &self.ctest())
            .field("duplx", &self.duplx())
            .field("spd0", &self.spd0())
            .field("reset", &self.reset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Phyctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Phyctrl {{ spd1: {=bool:?}, ctest: {=bool:?}, duplx: {=bool:?}, spd0: {=bool:?}, reset: {=bool:?} }}",
            self.spd1(),
            self.ctest(),
            self.duplx(),
            self.spd0(),
            self.reset()
        )
    }
}
#[doc = "PHY Identifier0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Phyid0(pub u16);
impl Phyid0 {
    #[doc = "Organizationally Unique Idendifier"]
    #[must_use]
    #[inline(always)]
    pub const fn oui_03_18(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Organizationally Unique Idendifier"]
    #[inline(always)]
    pub const fn set_oui_03_18(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Phyid0 {
    #[inline(always)]
    fn default() -> Phyid0 {
        Phyid0(0)
    }
}
impl core::fmt::Debug for Phyid0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Phyid0")
            .field("oui_03_18", &self.oui_03_18())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Phyid0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Phyid0 {{ oui_03_18: {=u16:?} }}", self.oui_03_18())
    }
}
#[doc = "PHY Identifier1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Phyid1(pub u16);
impl Phyid1 {
    #[doc = "IC Model Revision"]
    #[must_use]
    #[inline(always)]
    pub const fn icrevision(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "IC Model Revision"]
    #[inline(always)]
    pub const fn set_icrevision(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "IC Model Identifier"]
    #[must_use]
    #[inline(always)]
    pub const fn icmodel(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x3f;
        val as u8
    }
    #[doc = "IC Model Identifier"]
    #[inline(always)]
    pub const fn set_icmodel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 4usize)) | (((val as u16) & 0x3f) << 4usize);
    }
    #[doc = "Organizationally Unique Idendifier"]
    #[must_use]
    #[inline(always)]
    pub const fn oui_19_24(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "Organizationally Unique Idendifier"]
    #[inline(always)]
    pub const fn set_oui_19_24(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u16) & 0x3f) << 10usize);
    }
}
impl Default for Phyid1 {
    #[inline(always)]
    fn default() -> Phyid1 {
        Phyid1(0)
    }
}
impl core::fmt::Debug for Phyid1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Phyid1")
            .field("icrevision", &self.icrevision())
            .field("icmodel", &self.icmodel())
            .field("oui_19_24", &self.oui_19_24())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Phyid1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Phyid1 {{ icrevision: {=u8:?}, icmodel: {=u8:?}, oui_19_24: {=u8:?} }}",
            self.icrevision(),
            self.icmodel(),
            self.oui_19_24()
        )
    }
}
#[doc = "PHY status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Phystat(pub u16);
impl Phystat {
    #[doc = "Local Jabber"]
    #[must_use]
    #[inline(always)]
    pub const fn ljab(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Local Jabber"]
    #[inline(always)]
    pub const fn set_ljab(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Remote Jabber"]
    #[must_use]
    #[inline(always)]
    pub const fn rjab(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Remote Jabber"]
    #[inline(always)]
    pub const fn set_rjab(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
}
impl Default for Phystat {
    #[inline(always)]
    fn default() -> Phystat {
        Phystat(0)
    }
}
impl core::fmt::Debug for Phystat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Phystat")
            .field("ljab", &self.ljab())
            .field("rjab", &self.rjab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Phystat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Phystat {{ ljab: {=bool:?}, rjab: {=bool:?} }}",
            self.ljab(),
            self.rjab()
        )
    }
}
#[doc = "PLCA Burst Mode Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plcaburst(pub u16);
impl Plcaburst {
    #[doc = "Burst Timer"]
    #[must_use]
    #[inline(always)]
    pub const fn btmr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Burst Timer"]
    #[inline(always)]
    pub const fn set_btmr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Maximum Burst Count"]
    #[must_use]
    #[inline(always)]
    pub const fn maxbc(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Maximum Burst Count"]
    #[inline(always)]
    pub const fn set_maxbc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Plcaburst {
    #[inline(always)]
    fn default() -> Plcaburst {
        Plcaburst(0)
    }
}
impl core::fmt::Debug for Plcaburst {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Plcaburst")
            .field("btmr", &self.btmr())
            .field("maxbc", &self.maxbc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Plcaburst {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Plcaburst {{ btmr: {=u8:?}, maxbc: {=u8:?} }}",
            self.btmr(),
            self.maxbc()
        )
    }
}
#[doc = "PLCA Control0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plcactrl0(pub u16);
impl Plcactrl0 {
    #[doc = "Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn rst(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub const fn set_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Plcactrl0 {
    #[inline(always)]
    fn default() -> Plcactrl0 {
        Plcactrl0(0)
    }
}
impl core::fmt::Debug for Plcactrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Plcactrl0")
            .field("rst", &self.rst())
            .field("en", &self.en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Plcactrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Plcactrl0 {{ rst: {=bool:?}, en: {=bool:?} }}",
            self.rst(),
            self.en()
        )
    }
}
#[doc = "PLCA Control1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plcactrl1(pub u16);
impl Plcactrl1 {
    #[doc = "Identifier"]
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Identifier"]
    #[inline(always)]
    pub const fn set_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "PLCA Node count"]
    #[must_use]
    #[inline(always)]
    pub const fn ncnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "PLCA Node count"]
    #[inline(always)]
    pub const fn set_ncnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Plcactrl1 {
    #[inline(always)]
    fn default() -> Plcactrl1 {
        Plcactrl1(0)
    }
}
impl core::fmt::Debug for Plcactrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Plcactrl1")
            .field("id", &self.id())
            .field("ncnt", &self.ncnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Plcactrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Plcactrl1 {{ id: {=u8:?}, ncnt: {=u8:?} }}",
            self.id(),
            self.ncnt()
        )
    }
}
#[doc = "PLCA TC14 Adv diagnostics"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plcadiag1(pub u16);
impl Plcadiag1 {
    #[doc = "Beacon Consecutive"]
    #[must_use]
    #[inline(always)]
    pub const fn bcnbfto(&self) -> super::vals::Plcadiag1Bcnbfto {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Plcadiag1Bcnbfto::from_bits(val as u8)
    }
    #[doc = "Beacon Consecutive"]
    #[inline(always)]
    pub const fn set_bcnbfto(&mut self, val: super::vals::Plcadiag1Bcnbfto) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Unexpected Beacon"]
    #[must_use]
    #[inline(always)]
    pub const fn unexpb(&self) -> super::vals::Plcadiag1Unexpb {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Plcadiag1Unexpb::from_bits(val as u8)
    }
    #[doc = "Unexpected Beacon"]
    #[inline(always)]
    pub const fn set_unexpb(&mut self, val: super::vals::Plcadiag1Unexpb) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Indicates Reception"]
    #[must_use]
    #[inline(always)]
    pub const fn rxinto(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates Reception"]
    #[inline(always)]
    pub const fn set_rxinto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
}
impl Default for Plcadiag1 {
    #[inline(always)]
    fn default() -> Plcadiag1 {
        Plcadiag1(0)
    }
}
impl core::fmt::Debug for Plcadiag1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Plcadiag1")
            .field("bcnbfto", &self.bcnbfto())
            .field("unexpb", &self.unexpb())
            .field("rxinto", &self.rxinto())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Plcadiag1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Plcadiag1 {{ bcnbfto: {:?}, unexpb: {:?}, rxinto: {=bool:?} }}",
            self.bcnbfto(),
            self.unexpb(),
            self.rxinto()
        )
    }
}
#[doc = "PLCA NXP prop diagnostics"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plcadiag2(pub u16);
impl Plcadiag2 {
    #[doc = "Early Beacon"]
    #[must_use]
    #[inline(always)]
    pub const fn earlybcn(&self) -> super::vals::Plcadiag2Earlybcn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Plcadiag2Earlybcn::from_bits(val as u8)
    }
    #[doc = "Early Beacon"]
    #[inline(always)]
    pub const fn set_earlybcn(&mut self, val: super::vals::Plcadiag2Earlybcn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Late Beacon"]
    #[must_use]
    #[inline(always)]
    pub const fn latebcn(&self) -> super::vals::Plcadiag2Latebcn {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Plcadiag2Latebcn::from_bits(val as u8)
    }
    #[doc = "Late Beacon"]
    #[inline(always)]
    pub const fn set_latebcn(&mut self, val: super::vals::Plcadiag2Latebcn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "No Beacon Received"]
    #[must_use]
    #[inline(always)]
    pub const fn norxbcn(&self) -> super::vals::Plcadiag2Norxbcn {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Plcadiag2Norxbcn::from_bits(val as u8)
    }
    #[doc = "No Beacon Received"]
    #[inline(always)]
    pub const fn set_norxbcn(&mut self, val: super::vals::Plcadiag2Norxbcn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Undefined State"]
    #[must_use]
    #[inline(always)]
    pub const fn undefstate(&self) -> super::vals::Plcadiag2Undefstate {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Plcadiag2Undefstate::from_bits(val as u8)
    }
    #[doc = "Undefined State"]
    #[inline(always)]
    pub const fn set_undefstate(&mut self, val: super::vals::Plcadiag2Undefstate) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
}
impl Default for Plcadiag2 {
    #[inline(always)]
    fn default() -> Plcadiag2 {
        Plcadiag2(0)
    }
}
impl core::fmt::Debug for Plcadiag2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Plcadiag2")
            .field("earlybcn", &self.earlybcn())
            .field("latebcn", &self.latebcn())
            .field("norxbcn", &self.norxbcn())
            .field("undefstate", &self.undefstate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Plcadiag2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Plcadiag2 {{ earlybcn: {:?}, latebcn: {:?}, norxbcn: {:?}, undefstate: {:?} }}",
            self.earlybcn(),
            self.latebcn(),
            self.norxbcn(),
            self.undefstate()
        )
    }
}
#[doc = "PLCA Beacon Counter Diagnostics"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plcadiag3(pub u16);
impl Plcadiag3 {
    #[doc = "RX Beacon Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn rxbcntcnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "RX Beacon Counter"]
    #[inline(always)]
    pub const fn set_rxbcntcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Plcadiag3 {
    #[inline(always)]
    fn default() -> Plcadiag3 {
        Plcadiag3(0)
    }
}
impl core::fmt::Debug for Plcadiag3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Plcadiag3")
            .field("rxbcntcnt", &self.rxbcntcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Plcadiag3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Plcadiag3 {{ rxbcntcnt: {=u16:?} }}", self.rxbcntcnt())
    }
}
#[doc = "PLCA TO Counter Diagnostics"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plcadiag4(pub u16);
impl Plcadiag4 {
    #[doc = "Transmit Oppurtunity Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn tocnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transmit Oppurtunity Counter"]
    #[inline(always)]
    pub const fn set_tocnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Plcadiag4 {
    #[inline(always)]
    fn default() -> Plcadiag4 {
        Plcadiag4(0)
    }
}
impl core::fmt::Debug for Plcadiag4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Plcadiag4")
            .field("tocnt", &self.tocnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Plcadiag4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Plcadiag4 {{ tocnt: {=u16:?} }}", self.tocnt())
    }
}
#[doc = "PLCA Identification and Version"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plcaidver(pub u16);
impl Plcaidver {
    #[doc = "Version"]
    #[must_use]
    #[inline(always)]
    pub const fn ver(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Version"]
    #[inline(always)]
    pub const fn set_ver(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "OA memory map identifier"]
    #[must_use]
    #[inline(always)]
    pub const fn idm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "OA memory map identifier"]
    #[inline(always)]
    pub const fn set_idm(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Plcaidver {
    #[inline(always)]
    fn default() -> Plcaidver {
        Plcaidver(0)
    }
}
impl core::fmt::Debug for Plcaidver {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Plcaidver")
            .field("ver", &self.ver())
            .field("idm", &self.idm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Plcaidver {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Plcaidver {{ ver: {=u8:?}, idm: {=u8:?} }}",
            self.ver(),
            self.idm()
        )
    }
}
#[doc = "PLCA Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plcarecst(pub u16);
impl Plcarecst {
    #[doc = "Recovering Status"]
    #[must_use]
    #[inline(always)]
    pub const fn recst(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Recovering Status"]
    #[inline(always)]
    pub const fn set_recst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Plcarecst {
    #[inline(always)]
    fn default() -> Plcarecst {
        Plcarecst(0)
    }
}
impl core::fmt::Debug for Plcarecst {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Plcarecst")
            .field("recst", &self.recst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Plcarecst {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Plcarecst {{ recst: {=bool:?} }}", self.recst())
    }
}
#[doc = "PLCA Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plcastat(pub u16);
impl Plcastat {
    #[doc = "PLCA Node status"]
    #[must_use]
    #[inline(always)]
    pub const fn pst(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PLCA Node status"]
    #[inline(always)]
    pub const fn set_pst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Plcastat {
    #[inline(always)]
    fn default() -> Plcastat {
        Plcastat(0)
    }
}
impl core::fmt::Debug for Plcastat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Plcastat")
            .field("pst", &self.pst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Plcastat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Plcastat {{ pst: {=bool:?} }}", self.pst())
    }
}
#[doc = "PLCA Transmit Opportunity Timer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plcatotmr(pub u16);
impl Plcatotmr {
    #[doc = "PLCA Transmit Opportunity"]
    #[must_use]
    #[inline(always)]
    pub const fn totmr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "PLCA Transmit Opportunity"]
    #[inline(always)]
    pub const fn set_totmr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for Plcatotmr {
    #[inline(always)]
    fn default() -> Plcatotmr {
        Plcatotmr(0)
    }
}
impl core::fmt::Debug for Plcatotmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Plcatotmr")
            .field("totmr", &self.totmr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Plcatotmr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Plcatotmr {{ totmr: {=u8:?} }}", self.totmr())
    }
}
#[doc = "PMA CTRL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmactrl(pub u16);
impl Pmactrl {
    #[doc = "Loop"]
    #[must_use]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::PmactrlLoop {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PmactrlLoop::from_bits(val as u8)
    }
    #[doc = "Loop"]
    #[inline(always)]
    pub const fn set_loop_(&mut self, val: super::vals::PmactrlLoop) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Multidrop Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn mult(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Multidrop Configuration"]
    #[inline(always)]
    pub const fn set_mult(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Transmission Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn txdis(&self) -> super::vals::Txdis {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Txdis::from_bits(val as u8)
    }
    #[doc = "Transmission Disable"]
    #[inline(always)]
    pub const fn set_txdis(&mut self, val: super::vals::Txdis) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn rst(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub const fn set_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Pmactrl {
    #[inline(always)]
    fn default() -> Pmactrl {
        Pmactrl(0)
    }
}
impl core::fmt::Debug for Pmactrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmactrl")
            .field("loop_", &self.loop_())
            .field("mult", &self.mult())
            .field("txdis", &self.txdis())
            .field("rst", &self.rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmactrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pmactrl {{ loop_: {:?}, mult: {=bool:?}, txdis: {:?}, rst: {=bool:?} }}",
            self.loop_(),
            self.mult(),
            self.txdis(),
            self.rst()
        )
    }
}
#[doc = "PMA/PMD Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmapmdctrl(pub u16);
impl Pmapmdctrl {
    #[doc = "Type"]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> super::vals::Type {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Type::from_bits(val as u8)
    }
    #[doc = "Type"]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: super::vals::Type) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
}
impl Default for Pmapmdctrl {
    #[inline(always)]
    fn default() -> Pmapmdctrl {
        Pmapmdctrl(0)
    }
}
impl core::fmt::Debug for Pmapmdctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmapmdctrl")
            .field("type_", &self.type_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmapmdctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pmapmdctrl {{ type_: {:?} }}", self.type_())
    }
}
#[doc = "PMA Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmastat(pub u16);
impl Pmastat {
    #[doc = "RJAB Alias"]
    #[must_use]
    #[inline(always)]
    pub const fn rjab(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RJAB Alias"]
    #[inline(always)]
    pub const fn set_rjab(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Receive Fault Advertises"]
    #[must_use]
    #[inline(always)]
    pub const fn rflta(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Fault Advertises"]
    #[inline(always)]
    pub const fn set_rflta(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Multidrop Advertises"]
    #[must_use]
    #[inline(always)]
    pub const fn multa(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Multidrop Advertises"]
    #[inline(always)]
    pub const fn set_multa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Low Power Advertise"]
    #[must_use]
    #[inline(always)]
    pub const fn lpwa(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Low Power Advertise"]
    #[inline(always)]
    pub const fn set_lpwa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Loopback Advertise"]
    #[must_use]
    #[inline(always)]
    pub const fn loopa(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Loopback Advertise"]
    #[inline(always)]
    pub const fn set_loopa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
}
impl Default for Pmastat {
    #[inline(always)]
    fn default() -> Pmastat {
        Pmastat(0)
    }
}
impl core::fmt::Debug for Pmastat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmastat")
            .field("rjab", &self.rjab())
            .field("rflta", &self.rflta())
            .field("multa", &self.multa())
            .field("lpwa", &self.lpwa())
            .field("loopa", &self.loopa())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmastat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pmastat {{ rjab: {=bool:?}, rflta: {=bool:?}, multa: {=bool:?}, lpwa: {=bool:?}, loopa: {=bool:?} }}",
            self.rjab(),
            self.rflta(),
            self.multa(),
            self.lpwa(),
            self.loopa()
        )
    }
}
#[doc = "PMA Test Mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmatm(pub u16);
impl Pmatm {
    #[doc = "Pattern"]
    #[must_use]
    #[inline(always)]
    pub const fn pattern(&self) -> super::vals::Pattern {
        let val = (self.0 >> 13usize) & 0x07;
        super::vals::Pattern::from_bits(val as u8)
    }
    #[doc = "Pattern"]
    #[inline(always)]
    pub const fn set_pattern(&mut self, val: super::vals::Pattern) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u16) & 0x07) << 13usize);
    }
}
impl Default for Pmatm {
    #[inline(always)]
    fn default() -> Pmatm {
        Pmatm(0)
    }
}
impl core::fmt::Debug for Pmatm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmatm")
            .field("pattern", &self.pattern())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmatm {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pmatm {{ pattern: {:?} }}", self.pattern())
    }
}
#[doc = "SMI Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smicfg(pub u16);
impl Smicfg {
    #[doc = "Speed"]
    #[must_use]
    #[inline(always)]
    pub const fn speed(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "Speed"]
    #[inline(always)]
    pub const fn set_speed(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u16) & 0x3f) << 1usize);
    }
    #[doc = "Disable SMI Preamble"]
    #[must_use]
    #[inline(always)]
    pub const fn dispre(&self) -> super::vals::Dispre {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Dispre::from_bits(val as u8)
    }
    #[doc = "Disable SMI Preamble"]
    #[inline(always)]
    pub const fn set_dispre(&mut self, val: super::vals::Dispre) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Hold"]
    #[must_use]
    #[inline(always)]
    pub const fn hold(&self) -> super::vals::Hold {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Hold::from_bits(val as u8)
    }
    #[doc = "Hold"]
    #[inline(always)]
    pub const fn set_hold(&mut self, val: super::vals::Hold) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u16) & 0x07) << 8usize);
    }
}
impl Default for Smicfg {
    #[inline(always)]
    fn default() -> Smicfg {
        Smicfg(0)
    }
}
impl core::fmt::Debug for Smicfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smicfg")
            .field("speed", &self.speed())
            .field("dispre", &self.dispre())
            .field("hold", &self.hold())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smicfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smicfg {{ speed: {=u8:?}, dispre: {:?}, hold: {:?} }}",
            self.speed(),
            self.dispre(),
            self.hold()
        )
    }
}
#[doc = "SMI Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smictrl(pub u16);
impl Smictrl {
    #[doc = "Turn Around"]
    #[must_use]
    #[inline(always)]
    pub const fn ta(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Turn Around"]
    #[inline(always)]
    pub const fn set_ta(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u16) & 0x03) << 0usize);
    }
    #[doc = "Register Address"]
    #[must_use]
    #[inline(always)]
    pub const fn ra(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x1f;
        val as u8
    }
    #[doc = "Register Address"]
    #[inline(always)]
    pub const fn set_ra(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 2usize)) | (((val as u16) & 0x1f) << 2usize);
    }
    #[doc = "PHY Address"]
    #[must_use]
    #[inline(always)]
    pub const fn pa(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x1f;
        val as u8
    }
    #[doc = "PHY Address"]
    #[inline(always)]
    pub const fn set_pa(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 7usize)) | (((val as u16) & 0x1f) << 7usize);
    }
    #[doc = "Operation"]
    #[must_use]
    #[inline(always)]
    pub const fn op(&self) -> super::vals::Op {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Op::from_bits(val as u8)
    }
    #[doc = "Operation"]
    #[inline(always)]
    pub const fn set_op(&mut self, val: super::vals::Op) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Start"]
    #[must_use]
    #[inline(always)]
    pub const fn st(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Start"]
    #[inline(always)]
    pub const fn set_st(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u16) & 0x03) << 14usize);
    }
}
impl Default for Smictrl {
    #[inline(always)]
    fn default() -> Smictrl {
        Smictrl(0)
    }
}
impl core::fmt::Debug for Smictrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smictrl")
            .field("ta", &self.ta())
            .field("ra", &self.ra())
            .field("pa", &self.pa())
            .field("op", &self.op())
            .field("st", &self.st())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smictrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smictrl {{ ta: {=u8:?}, ra: {=u8:?}, pa: {=u8:?}, op: {:?}, st: {=u8:?} }}",
            self.ta(),
            self.ra(),
            self.pa(),
            self.op(),
            self.st()
        )
    }
}
#[doc = "SMI Frame DATA"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smidata(pub u16);
impl Smidata {
    #[doc = "Frame Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Frame Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Smidata {
    #[inline(always)]
    fn default() -> Smidata {
        Smidata(0)
    }
}
impl core::fmt::Debug for Smidata {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smidata")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smidata {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smidata {{ data: {=u16:?} }}", self.data())
    }
}
#[doc = "SMI Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smistat(pub u16);
impl Smistat {
    #[doc = "Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn ready(&self) -> super::vals::Ready {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ready::from_bits(val as u8)
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub const fn set_ready(&mut self, val: super::vals::Ready) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Error Code"]
    #[must_use]
    #[inline(always)]
    pub const fn errcode(&self) -> super::vals::SmistatErrcode {
        let val = (self.0 >> 1usize) & 0x07;
        super::vals::SmistatErrcode::from_bits(val as u8)
    }
    #[doc = "Error Code"]
    #[inline(always)]
    pub const fn set_errcode(&mut self, val: super::vals::SmistatErrcode) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u16) & 0x07) << 1usize);
    }
}
impl Default for Smistat {
    #[inline(always)]
    fn default() -> Smistat {
        Smistat(0)
    }
}
impl core::fmt::Debug for Smistat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smistat")
            .field("ready", &self.ready())
            .field("errcode", &self.errcode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smistat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smistat {{ ready: {:?}, errcode: {:?} }}",
            self.ready(),
            self.errcode()
        )
    }
}
#[doc = "Transceiver Diagnostics"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txcdiag(pub u16);
impl Txcdiag {
    #[doc = "RX Low Pin Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn rxlowfail(&self) -> super::vals::TxcdiagRxlowfail {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::TxcdiagRxlowfail::from_bits(val as u8)
    }
    #[doc = "RX Low Pin Detect"]
    #[inline(always)]
    pub const fn set_rxlowfail(&mut self, val: super::vals::TxcdiagRxlowfail) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "RX High Pin Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn rxhghfail(&self) -> super::vals::TxcdiagRxhghfail {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::TxcdiagRxhghfail::from_bits(val as u8)
    }
    #[doc = "RX High Pin Detect"]
    #[inline(always)]
    pub const fn set_rxhghfail(&mut self, val: super::vals::TxcdiagRxhghfail) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Stuck Low ED Pin"]
    #[must_use]
    #[inline(always)]
    pub const fn edlowfail(&self) -> super::vals::TxcdiagEdlowfail {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::TxcdiagEdlowfail::from_bits(val as u8)
    }
    #[doc = "Stuck Low ED Pin"]
    #[inline(always)]
    pub const fn set_edlowfail(&mut self, val: super::vals::TxcdiagEdlowfail) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Stuck High ED Pin"]
    #[must_use]
    #[inline(always)]
    pub const fn edhghfail(&self) -> super::vals::TxcdiagEdhghfail {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::TxcdiagEdhghfail::from_bits(val as u8)
    }
    #[doc = "Stuck High ED Pin"]
    #[inline(always)]
    pub const fn set_edhghfail(&mut self, val: super::vals::TxcdiagEdhghfail) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "Low Power Failure"]
    #[must_use]
    #[inline(always)]
    pub const fn lpwrfail(&self) -> super::vals::TxcdiagLpwrfail {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::TxcdiagLpwrfail::from_bits(val as u8)
    }
    #[doc = "Low Power Failure"]
    #[inline(always)]
    pub const fn set_lpwrfail(&mut self, val: super::vals::TxcdiagLpwrfail) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
}
impl Default for Txcdiag {
    #[inline(always)]
    fn default() -> Txcdiag {
        Txcdiag(0)
    }
}
impl core::fmt::Debug for Txcdiag {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txcdiag")
            .field("rxlowfail", &self.rxlowfail())
            .field("rxhghfail", &self.rxhghfail())
            .field("edlowfail", &self.edlowfail())
            .field("edhghfail", &self.edhghfail())
            .field("lpwrfail", &self.lpwrfail())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txcdiag {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Txcdiag {{ rxlowfail: {:?}, rxhghfail: {:?}, edlowfail: {:?}, edhghfail: {:?}, lpwrfail: {:?} }}",
            self.rxlowfail(),
            self.rxhghfail(),
            self.edlowfail(),
            self.edhghfail(),
            self.lpwrfail()
        )
    }
}
#[doc = "Version Information"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Version(pub u16);
impl Version {
    #[doc = "Customer Version"]
    #[must_use]
    #[inline(always)]
    pub const fn custver(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Customer Version"]
    #[inline(always)]
    pub const fn set_custver(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "Minor Version"]
    #[must_use]
    #[inline(always)]
    pub const fn minorver(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version"]
    #[inline(always)]
    pub const fn set_minorver(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val as u16) & 0xff) << 4usize);
    }
    #[doc = "Major Version"]
    #[must_use]
    #[inline(always)]
    pub const fn majorver(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Major Version"]
    #[inline(always)]
    pub const fn set_majorver(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u16) & 0x0f) << 12usize);
    }
}
impl Default for Version {
    #[inline(always)]
    fn default() -> Version {
        Version(0)
    }
}
impl core::fmt::Debug for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Version")
            .field("custver", &self.custver())
            .field("minorver", &self.minorver())
            .field("majorver", &self.majorver())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Version {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Version {{ custver: {=u8:?}, minorver: {=u8:?}, majorver: {=u8:?} }}",
            self.custver(),
            self.minorver(),
            self.majorver()
        )
    }
}
#[doc = "Wake Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wkcfg(pub u16);
impl Wkcfg {
    #[doc = "Enable wake forwarding of a remote-wake-event"]
    #[must_use]
    #[inline(always)]
    pub const fn remkwkfdw(&self) -> super::vals::Remkwkfdw {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Remkwkfdw::from_bits(val as u8)
    }
    #[doc = "Enable wake forwarding of a remote-wake-event"]
    #[inline(always)]
    pub const fn set_remkwkfdw(&mut self, val: super::vals::Remkwkfdw) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
}
impl Default for Wkcfg {
    #[inline(always)]
    fn default() -> Wkcfg {
        Wkcfg(0)
    }
}
impl core::fmt::Debug for Wkcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wkcfg")
            .field("remkwkfdw", &self.remkwkfdw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wkcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wkcfg {{ remkwkfdw: {:?} }}", self.remkwkfdw())
    }
}
#[doc = "Wake Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wkctrl(pub u16);
impl Wkctrl {
    #[doc = "Command to generate a wake-forward pulse"]
    #[must_use]
    #[inline(always)]
    pub const fn remwkfdw(&self) -> super::vals::Remwkfdw {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Remwkfdw::from_bits(val as u8)
    }
    #[doc = "Command to generate a wake-forward pulse"]
    #[inline(always)]
    pub const fn set_remwkfdw(&mut self, val: super::vals::Remwkfdw) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
}
impl Default for Wkctrl {
    #[inline(always)]
    fn default() -> Wkctrl {
        Wkctrl(0)
    }
}
impl core::fmt::Debug for Wkctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wkctrl")
            .field("remwkfdw", &self.remwkfdw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wkctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wkctrl {{ remwkfdw: {:?} }}", self.remwkfdw())
    }
}
#[doc = "Wakeup Confguration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wupcfg(pub u16);
impl Wupcfg {
    #[doc = "Enable WUP transmission by local-wake-event"]
    #[must_use]
    #[inline(always)]
    pub const fn lclwkwup(&self) -> super::vals::Lclwkwup {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lclwkwup::from_bits(val as u8)
    }
    #[doc = "Enable WUP transmission by local-wake-event"]
    #[inline(always)]
    pub const fn set_lclwkwup(&mut self, val: super::vals::Lclwkwup) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Wakeup Timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn wuptimeout(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Wakeup Timeout"]
    #[inline(always)]
    pub const fn set_wuptimeout(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Wupcfg {
    #[inline(always)]
    fn default() -> Wupcfg {
        Wupcfg(0)
    }
}
impl core::fmt::Debug for Wupcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wupcfg")
            .field("lclwkwup", &self.lclwkwup())
            .field("wuptimeout", &self.wuptimeout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wupcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wupcfg {{ lclwkwup: {:?}, wuptimeout: {=u8:?} }}",
            self.lclwkwup(),
            self.wuptimeout()
        )
    }
}
#[doc = "Wake Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wupctrl(pub u16);
impl Wupctrl {
    #[doc = "Trigger the WUP sequencer transmission"]
    #[must_use]
    #[inline(always)]
    pub const fn wupcmd(&self) -> super::vals::Wupcmd {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Wupcmd::from_bits(val as u8)
    }
    #[doc = "Trigger the WUP sequencer transmission"]
    #[inline(always)]
    pub const fn set_wupcmd(&mut self, val: super::vals::Wupcmd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Abort the WUP sequencer when pending."]
    #[must_use]
    #[inline(always)]
    pub const fn abortcmd(&self) -> super::vals::Abortcmd {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Abortcmd::from_bits(val as u8)
    }
    #[doc = "Abort the WUP sequencer when pending."]
    #[inline(always)]
    pub const fn set_abortcmd(&mut self, val: super::vals::Abortcmd) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
}
impl Default for Wupctrl {
    #[inline(always)]
    fn default() -> Wupctrl {
        Wupctrl(0)
    }
}
impl core::fmt::Debug for Wupctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wupctrl")
            .field("wupcmd", &self.wupcmd())
            .field("abortcmd", &self.abortcmd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wupctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wupctrl {{ wupcmd: {:?}, abortcmd: {:?} }}",
            self.wupcmd(),
            self.abortcmd()
        )
    }
}
#[doc = "Wakeup Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wupstat(pub u16);
impl Wupstat {
    #[doc = "Status"]
    #[must_use]
    #[inline(always)]
    pub const fn stat(&self) -> super::vals::WupstatStat {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::WupstatStat::from_bits(val as u8)
    }
    #[doc = "Status"]
    #[inline(always)]
    pub const fn set_stat(&mut self, val: super::vals::WupstatStat) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Error code"]
    #[must_use]
    #[inline(always)]
    pub const fn errcode(&self) -> super::vals::WupstatErrcode {
        let val = (self.0 >> 2usize) & 0x07;
        super::vals::WupstatErrcode::from_bits(val as u8)
    }
    #[doc = "Error code"]
    #[inline(always)]
    pub const fn set_errcode(&mut self, val: super::vals::WupstatErrcode) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val.to_bits() as u16) & 0x07) << 2usize);
    }
}
impl Default for Wupstat {
    #[inline(always)]
    fn default() -> Wupstat {
        Wupstat(0)
    }
}
impl core::fmt::Debug for Wupstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wupstat")
            .field("stat", &self.stat())
            .field("errcode", &self.errcode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wupstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wupstat {{ stat: {:?}, errcode: {:?} }}",
            self.stat(),
            self.errcode()
        )
    }
}
