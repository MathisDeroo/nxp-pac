#[doc = "MAC Address0 High"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacAddress0High(pub u32);
impl MacAddress0High {
    #[doc = "MAC Address0\\[47:32\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn addrhi(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "MAC Address0\\[47:32\\]"]
    #[inline(always)]
    pub const fn set_addrhi(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Address Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ae(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Address Enable"]
    #[inline(always)]
    pub const fn set_ae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for MacAddress0High {
    #[inline(always)]
    fn default() -> MacAddress0High {
        MacAddress0High(0)
    }
}
impl core::fmt::Debug for MacAddress0High {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacAddress0High")
            .field("addrhi", &self.addrhi())
            .field("ae", &self.ae())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacAddress0High {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacAddress0High {{ addrhi: {=u16:?}, ae: {=bool:?} }}",
            self.addrhi(),
            self.ae()
        )
    }
}
#[doc = "MAC Address0 Low"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacAddress0Low(pub u32);
impl MacAddress0Low {
    #[doc = "MAC Address0\\[31:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn addrlo(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "MAC Address0\\[31:0\\]"]
    #[inline(always)]
    pub const fn set_addrlo(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MacAddress0Low {
    #[inline(always)]
    fn default() -> MacAddress0Low {
        MacAddress0Low(0)
    }
}
impl core::fmt::Debug for MacAddress0Low {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacAddress0Low")
            .field("addrlo", &self.addrlo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacAddress0Low {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacAddress0Low {{ addrlo: {=u32:?} }}", self.addrlo())
    }
}
#[doc = "Auxiliary Timestamp Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacAuxiliaryControl(pub u32);
impl MacAuxiliaryControl {
    #[doc = "Auxiliary Snapshot FIFO Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn atsfc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Auxiliary Snapshot FIFO Clear"]
    #[inline(always)]
    pub const fn set_atsfc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Auxiliary Snapshot 0 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn atsen0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Auxiliary Snapshot 0 Enable"]
    #[inline(always)]
    pub const fn set_atsen0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Auxiliary Snapshot 1 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn atsen1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Auxiliary Snapshot 1 Enable"]
    #[inline(always)]
    pub const fn set_atsen1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for MacAuxiliaryControl {
    #[inline(always)]
    fn default() -> MacAuxiliaryControl {
        MacAuxiliaryControl(0)
    }
}
impl core::fmt::Debug for MacAuxiliaryControl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacAuxiliaryControl")
            .field("atsfc", &self.atsfc())
            .field("atsen0", &self.atsen0())
            .field("atsen1", &self.atsen1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacAuxiliaryControl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacAuxiliaryControl {{ atsfc: {=bool:?}, atsen0: {=bool:?}, atsen1: {=bool:?} }}",
            self.atsfc(),
            self.atsen0(),
            self.atsen1()
        )
    }
}
#[doc = "Auxiliary Timestamp Nanoseconds"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacAuxiliaryTimestampNanoseconds(pub u32);
impl MacAuxiliaryTimestampNanoseconds {
    #[doc = "Auxiliary Timestamp"]
    #[must_use]
    #[inline(always)]
    pub const fn auxtslo(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Auxiliary Timestamp"]
    #[inline(always)]
    pub const fn set_auxtslo(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
}
impl Default for MacAuxiliaryTimestampNanoseconds {
    #[inline(always)]
    fn default() -> MacAuxiliaryTimestampNanoseconds {
        MacAuxiliaryTimestampNanoseconds(0)
    }
}
impl core::fmt::Debug for MacAuxiliaryTimestampNanoseconds {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacAuxiliaryTimestampNanoseconds")
            .field("auxtslo", &self.auxtslo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacAuxiliaryTimestampNanoseconds {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacAuxiliaryTimestampNanoseconds {{ auxtslo: {=u32:?} }}",
            self.auxtslo()
        )
    }
}
#[doc = "Auxiliary Timestamp Seconds"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacAuxiliaryTimestampSeconds(pub u32);
impl MacAuxiliaryTimestampSeconds {
    #[doc = "Auxiliary Timestamp"]
    #[must_use]
    #[inline(always)]
    pub const fn auxtshi(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Auxiliary Timestamp"]
    #[inline(always)]
    pub const fn set_auxtshi(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MacAuxiliaryTimestampSeconds {
    #[inline(always)]
    fn default() -> MacAuxiliaryTimestampSeconds {
        MacAuxiliaryTimestampSeconds(0)
    }
}
impl core::fmt::Debug for MacAuxiliaryTimestampSeconds {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacAuxiliaryTimestampSeconds")
            .field("auxtshi", &self.auxtshi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacAuxiliaryTimestampSeconds {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacAuxiliaryTimestampSeconds {{ auxtshi: {=u32:?} }}",
            self.auxtshi()
        )
    }
}
#[doc = "MAC Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacConfiguration(pub u32);
impl MacConfiguration {
    #[doc = "Receiver Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn re(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Enable"]
    #[inline(always)]
    pub const fn set_re(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmitter Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn te(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Enable"]
    #[inline(always)]
    pub const fn set_te(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Preamble Length for Transmit packets"]
    #[must_use]
    #[inline(always)]
    pub const fn prelen(&self) -> super::vals::Prelen {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Prelen::from_bits(val as u8)
    }
    #[doc = "Preamble Length for Transmit packets"]
    #[inline(always)]
    pub const fn set_prelen(&mut self, val: super::vals::Prelen) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Deferral Check"]
    #[must_use]
    #[inline(always)]
    pub const fn dc(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Deferral Check"]
    #[inline(always)]
    pub const fn set_dc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Back-Off Limit"]
    #[must_use]
    #[inline(always)]
    pub const fn bl(&self) -> super::vals::Bl {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Bl::from_bits(val as u8)
    }
    #[doc = "Back-Off Limit"]
    #[inline(always)]
    pub const fn set_bl(&mut self, val: super::vals::Bl) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Disable Retry"]
    #[must_use]
    #[inline(always)]
    pub const fn dr(&self) -> super::vals::Dr {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Dr::from_bits(val as u8)
    }
    #[doc = "Disable Retry"]
    #[inline(always)]
    pub const fn set_dr(&mut self, val: super::vals::Dr) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Disable Carrier Sense During Transmission"]
    #[must_use]
    #[inline(always)]
    pub const fn dcrs(&self) -> super::vals::Dcrs {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Dcrs::from_bits(val as u8)
    }
    #[doc = "Disable Carrier Sense During Transmission"]
    #[inline(always)]
    pub const fn set_dcrs(&mut self, val: super::vals::Dcrs) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Disable Receive Own"]
    #[must_use]
    #[inline(always)]
    pub const fn do_(&self) -> super::vals::Do {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Do::from_bits(val as u8)
    }
    #[doc = "Disable Receive Own"]
    #[inline(always)]
    pub const fn set_do_(&mut self, val: super::vals::Do) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable Carrier Sense Before Transmission in Full-Duplex Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ecrsfd(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Carrier Sense Before Transmission in Full-Duplex Mode"]
    #[inline(always)]
    pub const fn set_ecrsfd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Loopback Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lm(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Loopback Mode"]
    #[inline(always)]
    pub const fn set_lm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Duplex Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn dm(&self) -> super::vals::Dm {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Dm::from_bits(val as u8)
    }
    #[doc = "Duplex Mode"]
    #[inline(always)]
    pub const fn set_dm(&mut self, val: super::vals::Dm) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Speed"]
    #[must_use]
    #[inline(always)]
    pub const fn fes(&self) -> super::vals::Fes {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Fes::from_bits(val as u8)
    }
    #[doc = "Speed"]
    #[inline(always)]
    pub const fn set_fes(&mut self, val: super::vals::Fes) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Port Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Ps {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Ps::from_bits(val as u8)
    }
    #[doc = "Port Select"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Ps) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Jumbo Packet Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn je(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Jumbo Packet Enable"]
    #[inline(always)]
    pub const fn set_je(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Jabber Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn jd(&self) -> super::vals::Jd {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Jd::from_bits(val as u8)
    }
    #[doc = "Jabber Disable"]
    #[inline(always)]
    pub const fn set_jd(&mut self, val: super::vals::Jd) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Watchdog Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn wd(&self) -> super::vals::Wd {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Wd::from_bits(val as u8)
    }
    #[doc = "Watchdog Disable"]
    #[inline(always)]
    pub const fn set_wd(&mut self, val: super::vals::Wd) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Automatic Pad or CRC Stripping"]
    #[must_use]
    #[inline(always)]
    pub const fn acs(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic Pad or CRC Stripping"]
    #[inline(always)]
    pub const fn set_acs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "CRC stripping for Type packets"]
    #[must_use]
    #[inline(always)]
    pub const fn cst(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "CRC stripping for Type packets"]
    #[inline(always)]
    pub const fn set_cst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Support for 2K Packets"]
    #[must_use]
    #[inline(always)]
    pub const fn s2kp(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Support for 2K Packets"]
    #[inline(always)]
    pub const fn set_s2kp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Giant Packet Size Limit Control Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpslce(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Giant Packet Size Limit Control Enable"]
    #[inline(always)]
    pub const fn set_gpslce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Inter-Packet Gap"]
    #[must_use]
    #[inline(always)]
    pub const fn ipg(&self) -> super::vals::Ipg {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Ipg::from_bits(val as u8)
    }
    #[doc = "Inter-Packet Gap"]
    #[inline(always)]
    pub const fn set_ipg(&mut self, val: super::vals::Ipg) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "Checksum Offload"]
    #[must_use]
    #[inline(always)]
    pub const fn ipc(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Checksum Offload"]
    #[inline(always)]
    pub const fn set_ipc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Source Address Insertion or Replacement Control"]
    #[must_use]
    #[inline(always)]
    pub const fn sarc(&self) -> super::vals::Sarc {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Sarc::from_bits(val as u8)
    }
    #[doc = "Source Address Insertion or Replacement Control"]
    #[inline(always)]
    pub const fn set_sarc(&mut self, val: super::vals::Sarc) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
}
impl Default for MacConfiguration {
    #[inline(always)]
    fn default() -> MacConfiguration {
        MacConfiguration(0)
    }
}
impl core::fmt::Debug for MacConfiguration {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacConfiguration")
            .field("re", &self.re())
            .field("te", &self.te())
            .field("prelen", &self.prelen())
            .field("dc", &self.dc())
            .field("bl", &self.bl())
            .field("dr", &self.dr())
            .field("dcrs", &self.dcrs())
            .field("do_", &self.do_())
            .field("ecrsfd", &self.ecrsfd())
            .field("lm", &self.lm())
            .field("dm", &self.dm())
            .field("fes", &self.fes())
            .field("ps", &self.ps())
            .field("je", &self.je())
            .field("jd", &self.jd())
            .field("wd", &self.wd())
            .field("acs", &self.acs())
            .field("cst", &self.cst())
            .field("s2kp", &self.s2kp())
            .field("gpslce", &self.gpslce())
            .field("ipg", &self.ipg())
            .field("ipc", &self.ipc())
            .field("sarc", &self.sarc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacConfiguration {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacConfiguration {{ re: {=bool:?}, te: {=bool:?}, prelen: {:?}, dc: {=bool:?}, bl: {:?}, dr: {:?}, dcrs: {:?}, do_: {:?}, ecrsfd: {=bool:?}, lm: {=bool:?}, dm: {:?}, fes: {:?}, ps: {:?}, je: {=bool:?}, jd: {:?}, wd: {:?}, acs: {=bool:?}, cst: {=bool:?}, s2kp: {=bool:?}, gpslce: {=bool:?}, ipg: {:?}, ipc: {=bool:?}, sarc: {:?} }}",
            self.re(),
            self.te(),
            self.prelen(),
            self.dc(),
            self.bl(),
            self.dr(),
            self.dcrs(),
            self.do_(),
            self.ecrsfd(),
            self.lm(),
            self.dm(),
            self.fes(),
            self.ps(),
            self.je(),
            self.jd(),
            self.wd(),
            self.acs(),
            self.cst(),
            self.s2kp(),
            self.gpslce(),
            self.ipg(),
            self.ipc(),
            self.sarc()
        )
    }
}
#[doc = "CSR Software Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacCsrSwCtrl(pub u32);
impl MacCsrSwCtrl {
    #[doc = "Register Clear on Write 1 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rcwe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Register Clear on Write 1 Enable"]
    #[inline(always)]
    pub const fn set_rcwe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for MacCsrSwCtrl {
    #[inline(always)]
    fn default() -> MacCsrSwCtrl {
        MacCsrSwCtrl(0)
    }
}
impl core::fmt::Debug for MacCsrSwCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacCsrSwCtrl")
            .field("rcwe", &self.rcwe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacCsrSwCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacCsrSwCtrl {{ rcwe: {=bool:?} }}", self.rcwe())
    }
}
#[doc = "Debug"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacDebug(pub u32);
impl MacDebug {
    #[doc = "MAC GMII or MII Receive Protocol Engine Status"]
    #[must_use]
    #[inline(always)]
    pub const fn rpests(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "MAC GMII or MII Receive Protocol Engine Status"]
    #[inline(always)]
    pub const fn set_rpests(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "MAC Receive Packet Controller FIFO Status"]
    #[must_use]
    #[inline(always)]
    pub const fn rfcfcsts(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "MAC Receive Packet Controller FIFO Status"]
    #[inline(always)]
    pub const fn set_rfcfcsts(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "MAC GMII or MII Transmit Protocol Engine Status"]
    #[must_use]
    #[inline(always)]
    pub const fn tpests(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "MAC GMII or MII Transmit Protocol Engine Status"]
    #[inline(always)]
    pub const fn set_tpests(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "MAC Transmit Packet Controller Status"]
    #[must_use]
    #[inline(always)]
    pub const fn tfcsts(&self) -> super::vals::Tfcsts {
        let val = (self.0 >> 17usize) & 0x03;
        super::vals::Tfcsts::from_bits(val as u8)
    }
    #[doc = "MAC Transmit Packet Controller Status"]
    #[inline(always)]
    pub const fn set_tfcsts(&mut self, val: super::vals::Tfcsts) {
        self.0 = (self.0 & !(0x03 << 17usize)) | (((val.to_bits() as u32) & 0x03) << 17usize);
    }
}
impl Default for MacDebug {
    #[inline(always)]
    fn default() -> MacDebug {
        MacDebug(0)
    }
}
impl core::fmt::Debug for MacDebug {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacDebug")
            .field("rpests", &self.rpests())
            .field("rfcfcsts", &self.rfcfcsts())
            .field("tpests", &self.tpests())
            .field("tfcsts", &self.tfcsts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacDebug {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacDebug {{ rpests: {=bool:?}, rfcfcsts: {=u8:?}, tpests: {=bool:?}, tfcsts: {:?} }}",
            self.rpests(),
            self.rfcfcsts(),
            self.tpests(),
            self.tfcsts()
        )
    }
}
#[doc = "MAC Extended Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacExtConfiguration(pub u32);
impl MacExtConfiguration {
    #[doc = "Giant Packet Size Limit"]
    #[must_use]
    #[inline(always)]
    pub const fn gpsl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Giant Packet Size Limit"]
    #[inline(always)]
    pub const fn set_gpsl(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "Disable CRC Checking for Received Packets"]
    #[must_use]
    #[inline(always)]
    pub const fn dcrcc(&self) -> super::vals::Dcrcc {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Dcrcc::from_bits(val as u8)
    }
    #[doc = "Disable CRC Checking for Received Packets"]
    #[inline(always)]
    pub const fn set_dcrcc(&mut self, val: super::vals::Dcrcc) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Slow Protocol Detection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn spen(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Slow Protocol Detection Enable"]
    #[inline(always)]
    pub const fn set_spen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Unicast Slow Protocol Packet Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn usp(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Unicast Slow Protocol Packet Detect"]
    #[inline(always)]
    pub const fn set_usp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Extended Inter-Packet Gap Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn eipgen(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Extended Inter-Packet Gap Enable"]
    #[inline(always)]
    pub const fn set_eipgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Extended Inter-Packet Gap"]
    #[must_use]
    #[inline(always)]
    pub const fn eipg(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x1f;
        val as u8
    }
    #[doc = "Extended Inter-Packet Gap"]
    #[inline(always)]
    pub const fn set_eipg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
    }
}
impl Default for MacExtConfiguration {
    #[inline(always)]
    fn default() -> MacExtConfiguration {
        MacExtConfiguration(0)
    }
}
impl core::fmt::Debug for MacExtConfiguration {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacExtConfiguration")
            .field("gpsl", &self.gpsl())
            .field("dcrcc", &self.dcrcc())
            .field("spen", &self.spen())
            .field("usp", &self.usp())
            .field("eipgen", &self.eipgen())
            .field("eipg", &self.eipg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacExtConfiguration {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacExtConfiguration {{ gpsl: {=u16:?}, dcrcc: {:?}, spen: {=bool:?}, usp: {=bool:?}, eipgen: {=bool:?}, eipg: {=u8:?} }}",
            self.gpsl(),
            self.dcrcc(),
            self.spen(),
            self.usp(),
            self.eipgen(),
            self.eipg()
        )
    }
}
#[doc = "HW Features 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacHwFeature0(pub u32);
impl MacHwFeature0 {
    #[doc = "10 or 100 Mbps Support"]
    #[must_use]
    #[inline(always)]
    pub const fn miisel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "10 or 100 Mbps Support"]
    #[inline(always)]
    pub const fn set_miisel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1000 Mbps Support"]
    #[must_use]
    #[inline(always)]
    pub const fn gmiisel(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1000 Mbps Support"]
    #[inline(always)]
    pub const fn set_gmiisel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Half-duplex Support"]
    #[must_use]
    #[inline(always)]
    pub const fn hdsel(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Half-duplex Support"]
    #[inline(always)]
    pub const fn set_hdsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "PCS Registers (TBI, SGMII, or RTBI PHY interface)"]
    #[must_use]
    #[inline(always)]
    pub const fn pcssel(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "PCS Registers (TBI, SGMII, or RTBI PHY interface)"]
    #[inline(always)]
    pub const fn set_pcssel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "VLAN Hash Filter Selected"]
    #[must_use]
    #[inline(always)]
    pub const fn vlhash(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VLAN Hash Filter Selected"]
    #[inline(always)]
    pub const fn set_vlhash(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "SMA (MDIO) Interface"]
    #[must_use]
    #[inline(always)]
    pub const fn smasel(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "SMA (MDIO) Interface"]
    #[inline(always)]
    pub const fn set_smasel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "PMT Remote Wake-up Packet Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rwksel(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PMT Remote Wake-up Packet Enable"]
    #[inline(always)]
    pub const fn set_rwksel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "PMT Magic Packet Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mgksel(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PMT Magic Packet Enable"]
    #[inline(always)]
    pub const fn set_mgksel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "RMON Module Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mmcsel(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "RMON Module Enable"]
    #[inline(always)]
    pub const fn set_mmcsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "ARP Offload Enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn arpoffsel(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "ARP Offload Enabled"]
    #[inline(always)]
    pub const fn set_arpoffsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "IEEE 1588-2008 Timestamp Enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn tssel(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "IEEE 1588-2008 Timestamp Enabled"]
    #[inline(always)]
    pub const fn set_tssel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Energy Efficient Ethernet Enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn eeesel(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Energy Efficient Ethernet Enabled"]
    #[inline(always)]
    pub const fn set_eeesel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Transmit Checksum Offload Enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn txcoesel(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Checksum Offload Enabled"]
    #[inline(always)]
    pub const fn set_txcoesel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Receive Checksum Offload Enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn rxcoesel(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Checksum Offload Enabled"]
    #[inline(always)]
    pub const fn set_rxcoesel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "MAC Addresses 1-31 Selected"]
    #[must_use]
    #[inline(always)]
    pub const fn addmacadrsel(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x1f;
        val as u8
    }
    #[doc = "MAC Addresses 1-31 Selected"]
    #[inline(always)]
    pub const fn set_addmacadrsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
    }
    #[doc = "MAC Addresses 32-63 Selected"]
    #[must_use]
    #[inline(always)]
    pub const fn macadr32sel(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "MAC Addresses 32-63 Selected"]
    #[inline(always)]
    pub const fn set_macadr32sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "MAC Addresses 64-127 Selected"]
    #[must_use]
    #[inline(always)]
    pub const fn macadr64sel(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "MAC Addresses 64-127 Selected"]
    #[inline(always)]
    pub const fn set_macadr64sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Timestamp System Time Source"]
    #[must_use]
    #[inline(always)]
    pub const fn tsstssel(&self) -> super::vals::Tsstssel {
        let val = (self.0 >> 25usize) & 0x03;
        super::vals::Tsstssel::from_bits(val as u8)
    }
    #[doc = "Timestamp System Time Source"]
    #[inline(always)]
    pub const fn set_tsstssel(&mut self, val: super::vals::Tsstssel) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
    #[doc = "Source Address or VLAN Insertion Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn savlanins(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Source Address or VLAN Insertion Enable"]
    #[inline(always)]
    pub const fn set_savlanins(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Active PHY Selected"]
    #[must_use]
    #[inline(always)]
    pub const fn actphysel(&self) -> super::vals::Actphysel {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Actphysel::from_bits(val as u8)
    }
    #[doc = "Active PHY Selected"]
    #[inline(always)]
    pub const fn set_actphysel(&mut self, val: super::vals::Actphysel) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for MacHwFeature0 {
    #[inline(always)]
    fn default() -> MacHwFeature0 {
        MacHwFeature0(0)
    }
}
impl core::fmt::Debug for MacHwFeature0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacHwFeature0")
            .field("miisel", &self.miisel())
            .field("gmiisel", &self.gmiisel())
            .field("hdsel", &self.hdsel())
            .field("pcssel", &self.pcssel())
            .field("vlhash", &self.vlhash())
            .field("smasel", &self.smasel())
            .field("rwksel", &self.rwksel())
            .field("mgksel", &self.mgksel())
            .field("mmcsel", &self.mmcsel())
            .field("arpoffsel", &self.arpoffsel())
            .field("tssel", &self.tssel())
            .field("eeesel", &self.eeesel())
            .field("txcoesel", &self.txcoesel())
            .field("rxcoesel", &self.rxcoesel())
            .field("addmacadrsel", &self.addmacadrsel())
            .field("macadr32sel", &self.macadr32sel())
            .field("macadr64sel", &self.macadr64sel())
            .field("tsstssel", &self.tsstssel())
            .field("savlanins", &self.savlanins())
            .field("actphysel", &self.actphysel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacHwFeature0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacHwFeature0 {{ miisel: {=bool:?}, gmiisel: {=bool:?}, hdsel: {=bool:?}, pcssel: {=bool:?}, vlhash: {=bool:?}, smasel: {=bool:?}, rwksel: {=bool:?}, mgksel: {=bool:?}, mmcsel: {=bool:?}, arpoffsel: {=bool:?}, tssel: {=bool:?}, eeesel: {=bool:?}, txcoesel: {=bool:?}, rxcoesel: {=bool:?}, addmacadrsel: {=u8:?}, macadr32sel: {=bool:?}, macadr64sel: {=bool:?}, tsstssel: {:?}, savlanins: {=bool:?}, actphysel: {:?} }}",
            self.miisel(),
            self.gmiisel(),
            self.hdsel(),
            self.pcssel(),
            self.vlhash(),
            self.smasel(),
            self.rwksel(),
            self.mgksel(),
            self.mmcsel(),
            self.arpoffsel(),
            self.tssel(),
            self.eeesel(),
            self.txcoesel(),
            self.rxcoesel(),
            self.addmacadrsel(),
            self.macadr32sel(),
            self.macadr64sel(),
            self.tsstssel(),
            self.savlanins(),
            self.actphysel()
        )
    }
}
#[doc = "HW Features 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacHwFeature1(pub u32);
impl MacHwFeature1 {
    #[doc = "MTL Receive FIFO Size"]
    #[must_use]
    #[inline(always)]
    pub const fn rxfifosize(&self) -> super::vals::Rxfifosize {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Rxfifosize::from_bits(val as u8)
    }
    #[doc = "MTL Receive FIFO Size"]
    #[inline(always)]
    pub const fn set_rxfifosize(&mut self, val: super::vals::Rxfifosize) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Single Port RAM Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn spram(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Single Port RAM Enable"]
    #[inline(always)]
    pub const fn set_spram(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "MTL Transmit FIFO Size"]
    #[must_use]
    #[inline(always)]
    pub const fn txfifosize(&self) -> super::vals::Txfifosize {
        let val = (self.0 >> 6usize) & 0x1f;
        super::vals::Txfifosize::from_bits(val as u8)
    }
    #[doc = "MTL Transmit FIFO Size"]
    #[inline(always)]
    pub const fn set_txfifosize(&mut self, val: super::vals::Txfifosize) {
        self.0 = (self.0 & !(0x1f << 6usize)) | (((val.to_bits() as u32) & 0x1f) << 6usize);
    }
    #[doc = "One-Step Timestamping Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn osten(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "One-Step Timestamping Enable"]
    #[inline(always)]
    pub const fn set_osten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "PTP Offload Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ptoen(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PTP Offload Enable"]
    #[inline(always)]
    pub const fn set_ptoen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "IEEE 1588 High Word Register Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn advthword(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "IEEE 1588 High Word Register Enable"]
    #[inline(always)]
    pub const fn set_advthword(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Address Width."]
    #[must_use]
    #[inline(always)]
    pub const fn addr64(&self) -> super::vals::Addr64 {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Addr64::from_bits(val as u8)
    }
    #[doc = "Address Width."]
    #[inline(always)]
    pub const fn set_addr64(&mut self, val: super::vals::Addr64) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "DCB Feature Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dcben(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DCB Feature Enable"]
    #[inline(always)]
    pub const fn set_dcben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Split Header Feature Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sphen(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Split Header Feature Enable"]
    #[inline(always)]
    pub const fn set_sphen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TCP Segmentation Offload Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tsoen(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "TCP Segmentation Offload Enable"]
    #[inline(always)]
    pub const fn set_tsoen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DMA Debug Registers Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgmema(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Debug Registers Enable"]
    #[inline(always)]
    pub const fn set_dbgmema(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "AV Feature Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn avsel(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "AV Feature Enable"]
    #[inline(always)]
    pub const fn set_avsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Rx Side Only AV Feature Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ravsel(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Rx Side Only AV Feature Enable"]
    #[inline(always)]
    pub const fn set_ravsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "One Step for PTP over UDP/IP Feature Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pouost(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "One Step for PTP over UDP/IP Feature Enable"]
    #[inline(always)]
    pub const fn set_pouost(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Hash Table Size"]
    #[must_use]
    #[inline(always)]
    pub const fn hashtblsz(&self) -> super::vals::Hashtblsz {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Hashtblsz::from_bits(val as u8)
    }
    #[doc = "Hash Table Size"]
    #[inline(always)]
    pub const fn set_hashtblsz(&mut self, val: super::vals::Hashtblsz) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Total number of L3 or L4 Filters"]
    #[must_use]
    #[inline(always)]
    pub const fn l3l4fnum(&self) -> super::vals::L3l4fnum {
        let val = (self.0 >> 27usize) & 0x0f;
        super::vals::L3l4fnum::from_bits(val as u8)
    }
    #[doc = "Total number of L3 or L4 Filters"]
    #[inline(always)]
    pub const fn set_l3l4fnum(&mut self, val: super::vals::L3l4fnum) {
        self.0 = (self.0 & !(0x0f << 27usize)) | (((val.to_bits() as u32) & 0x0f) << 27usize);
    }
}
impl Default for MacHwFeature1 {
    #[inline(always)]
    fn default() -> MacHwFeature1 {
        MacHwFeature1(0)
    }
}
impl core::fmt::Debug for MacHwFeature1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacHwFeature1")
            .field("rxfifosize", &self.rxfifosize())
            .field("spram", &self.spram())
            .field("txfifosize", &self.txfifosize())
            .field("osten", &self.osten())
            .field("ptoen", &self.ptoen())
            .field("advthword", &self.advthword())
            .field("addr64", &self.addr64())
            .field("dcben", &self.dcben())
            .field("sphen", &self.sphen())
            .field("tsoen", &self.tsoen())
            .field("dbgmema", &self.dbgmema())
            .field("avsel", &self.avsel())
            .field("ravsel", &self.ravsel())
            .field("pouost", &self.pouost())
            .field("hashtblsz", &self.hashtblsz())
            .field("l3l4fnum", &self.l3l4fnum())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacHwFeature1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacHwFeature1 {{ rxfifosize: {:?}, spram: {=bool:?}, txfifosize: {:?}, osten: {=bool:?}, ptoen: {=bool:?}, advthword: {=bool:?}, addr64: {:?}, dcben: {=bool:?}, sphen: {=bool:?}, tsoen: {=bool:?}, dbgmema: {=bool:?}, avsel: {=bool:?}, ravsel: {=bool:?}, pouost: {=bool:?}, hashtblsz: {:?}, l3l4fnum: {:?} }}",
            self.rxfifosize(),
            self.spram(),
            self.txfifosize(),
            self.osten(),
            self.ptoen(),
            self.advthword(),
            self.addr64(),
            self.dcben(),
            self.sphen(),
            self.tsoen(),
            self.dbgmema(),
            self.avsel(),
            self.ravsel(),
            self.pouost(),
            self.hashtblsz(),
            self.l3l4fnum()
        )
    }
}
#[doc = "HW Features 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacHwFeature2(pub u32);
impl MacHwFeature2 {
    #[doc = "Number of MTL Receive Queues"]
    #[must_use]
    #[inline(always)]
    pub const fn rxqcnt(&self) -> super::vals::Rxqcnt {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Rxqcnt::from_bits(val as u8)
    }
    #[doc = "Number of MTL Receive Queues"]
    #[inline(always)]
    pub const fn set_rxqcnt(&mut self, val: super::vals::Rxqcnt) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Number of MTL Transmit Queues"]
    #[must_use]
    #[inline(always)]
    pub const fn txqcnt(&self) -> super::vals::Txqcnt {
        let val = (self.0 >> 6usize) & 0x0f;
        super::vals::Txqcnt::from_bits(val as u8)
    }
    #[doc = "Number of MTL Transmit Queues"]
    #[inline(always)]
    pub const fn set_txqcnt(&mut self, val: super::vals::Txqcnt) {
        self.0 = (self.0 & !(0x0f << 6usize)) | (((val.to_bits() as u32) & 0x0f) << 6usize);
    }
    #[doc = "Number of DMA Receive Channels"]
    #[must_use]
    #[inline(always)]
    pub const fn rxchcnt(&self) -> super::vals::Rxchcnt {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Rxchcnt::from_bits(val as u8)
    }
    #[doc = "Number of DMA Receive Channels"]
    #[inline(always)]
    pub const fn set_rxchcnt(&mut self, val: super::vals::Rxchcnt) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Rx DMA Descriptor Cache Size in terms of 16-bytes descriptors:"]
    #[must_use]
    #[inline(always)]
    pub const fn rdcsz(&self) -> super::vals::Rdcsz {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Rdcsz::from_bits(val as u8)
    }
    #[doc = "Rx DMA Descriptor Cache Size in terms of 16-bytes descriptors:"]
    #[inline(always)]
    pub const fn set_rdcsz(&mut self, val: super::vals::Rdcsz) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Number of DMA Transmit Channels"]
    #[must_use]
    #[inline(always)]
    pub const fn txchcnt(&self) -> super::vals::Txchcnt {
        let val = (self.0 >> 18usize) & 0x0f;
        super::vals::Txchcnt::from_bits(val as u8)
    }
    #[doc = "Number of DMA Transmit Channels"]
    #[inline(always)]
    pub const fn set_txchcnt(&mut self, val: super::vals::Txchcnt) {
        self.0 = (self.0 & !(0x0f << 18usize)) | (((val.to_bits() as u32) & 0x0f) << 18usize);
    }
    #[doc = "Tx DMA Descriptor Cache Size in terms of 16-bytes descriptors:"]
    #[must_use]
    #[inline(always)]
    pub const fn tdcsz(&self) -> super::vals::Tdcsz {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::Tdcsz::from_bits(val as u8)
    }
    #[doc = "Tx DMA Descriptor Cache Size in terms of 16-bytes descriptors:"]
    #[inline(always)]
    pub const fn set_tdcsz(&mut self, val: super::vals::Tdcsz) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "Number of PPS Outputs"]
    #[must_use]
    #[inline(always)]
    pub const fn ppsoutnum(&self) -> super::vals::Ppsoutnum {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Ppsoutnum::from_bits(val as u8)
    }
    #[doc = "Number of PPS Outputs"]
    #[inline(always)]
    pub const fn set_ppsoutnum(&mut self, val: super::vals::Ppsoutnum) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "Number of Auxiliary Snapshot Inputs"]
    #[must_use]
    #[inline(always)]
    pub const fn auxsnapnum(&self) -> super::vals::Auxsnapnum {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Auxsnapnum::from_bits(val as u8)
    }
    #[doc = "Number of Auxiliary Snapshot Inputs"]
    #[inline(always)]
    pub const fn set_auxsnapnum(&mut self, val: super::vals::Auxsnapnum) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
}
impl Default for MacHwFeature2 {
    #[inline(always)]
    fn default() -> MacHwFeature2 {
        MacHwFeature2(0)
    }
}
impl core::fmt::Debug for MacHwFeature2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacHwFeature2")
            .field("rxqcnt", &self.rxqcnt())
            .field("txqcnt", &self.txqcnt())
            .field("rxchcnt", &self.rxchcnt())
            .field("rdcsz", &self.rdcsz())
            .field("txchcnt", &self.txchcnt())
            .field("tdcsz", &self.tdcsz())
            .field("ppsoutnum", &self.ppsoutnum())
            .field("auxsnapnum", &self.auxsnapnum())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacHwFeature2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacHwFeature2 {{ rxqcnt: {:?}, txqcnt: {:?}, rxchcnt: {:?}, rdcsz: {:?}, txchcnt: {:?}, tdcsz: {:?}, ppsoutnum: {:?}, auxsnapnum: {:?} }}",
            self.rxqcnt(),
            self.txqcnt(),
            self.rxchcnt(),
            self.rdcsz(),
            self.txchcnt(),
            self.tdcsz(),
            self.ppsoutnum(),
            self.auxsnapnum()
        )
    }
}
#[doc = "HW Features 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacHwFeature3(pub u32);
impl MacHwFeature3 {
    #[doc = "Number of Extended VLAN Tag Filters Enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn nrvf(&self) -> super::vals::Nrvf {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Nrvf::from_bits(val as u8)
    }
    #[doc = "Number of Extended VLAN Tag Filters Enabled"]
    #[inline(always)]
    pub const fn set_nrvf(&mut self, val: super::vals::Nrvf) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Queue/Channel based VLAN tag insertion on Tx Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cbtisel(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Queue/Channel based VLAN tag insertion on Tx Enable"]
    #[inline(always)]
    pub const fn set_cbtisel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Double VLAN Tag Processing Selected"]
    #[must_use]
    #[inline(always)]
    pub const fn dvlan(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Double VLAN Tag Processing Selected"]
    #[inline(always)]
    pub const fn set_dvlan(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Broadcast/Multicast Packet Duplication"]
    #[must_use]
    #[inline(always)]
    pub const fn pdupsel(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Broadcast/Multicast Packet Duplication"]
    #[inline(always)]
    pub const fn set_pdupsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Flexible Receive Parser Selected"]
    #[must_use]
    #[inline(always)]
    pub const fn frpsel(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Flexible Receive Parser Selected"]
    #[inline(always)]
    pub const fn set_frpsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Flexible Receive Parser Buffer size"]
    #[must_use]
    #[inline(always)]
    pub const fn frpbs(&self) -> super::vals::Frpbs {
        let val = (self.0 >> 11usize) & 0x03;
        super::vals::Frpbs::from_bits(val as u8)
    }
    #[doc = "Flexible Receive Parser Buffer size"]
    #[inline(always)]
    pub const fn set_frpbs(&mut self, val: super::vals::Frpbs) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val.to_bits() as u32) & 0x03) << 11usize);
    }
    #[doc = "Flexible Receive Parser Table Entries size"]
    #[must_use]
    #[inline(always)]
    pub const fn frpes(&self) -> super::vals::Frpes {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Frpes::from_bits(val as u8)
    }
    #[doc = "Flexible Receive Parser Table Entries size"]
    #[inline(always)]
    pub const fn set_frpes(&mut self, val: super::vals::Frpes) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "Enhancements to Scheduled Traffic Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn estsel(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enhancements to Scheduled Traffic Enable"]
    #[inline(always)]
    pub const fn set_estsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Depth of the Gate Control List"]
    #[must_use]
    #[inline(always)]
    pub const fn estdep(&self) -> super::vals::Estdep {
        let val = (self.0 >> 17usize) & 0x07;
        super::vals::Estdep::from_bits(val as u8)
    }
    #[doc = "Depth of the Gate Control List"]
    #[inline(always)]
    pub const fn set_estdep(&mut self, val: super::vals::Estdep) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val.to_bits() as u32) & 0x07) << 17usize);
    }
    #[doc = "Width of the Time Interval field in the Gate Control List"]
    #[must_use]
    #[inline(always)]
    pub const fn estwid(&self) -> super::vals::Estwid {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Estwid::from_bits(val as u8)
    }
    #[doc = "Width of the Time Interval field in the Gate Control List"]
    #[inline(always)]
    pub const fn set_estwid(&mut self, val: super::vals::Estwid) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Frame Preemption Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fpesel(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Preemption Enable"]
    #[inline(always)]
    pub const fn set_fpesel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Time Based Scheduling Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tbssel(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Time Based Scheduling Enable"]
    #[inline(always)]
    pub const fn set_tbssel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Automotive Safety Package"]
    #[must_use]
    #[inline(always)]
    pub const fn asp(&self) -> super::vals::Asp {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Asp::from_bits(val as u8)
    }
    #[doc = "Automotive Safety Package"]
    #[inline(always)]
    pub const fn set_asp(&mut self, val: super::vals::Asp) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for MacHwFeature3 {
    #[inline(always)]
    fn default() -> MacHwFeature3 {
        MacHwFeature3(0)
    }
}
impl core::fmt::Debug for MacHwFeature3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacHwFeature3")
            .field("nrvf", &self.nrvf())
            .field("cbtisel", &self.cbtisel())
            .field("dvlan", &self.dvlan())
            .field("pdupsel", &self.pdupsel())
            .field("frpsel", &self.frpsel())
            .field("frpbs", &self.frpbs())
            .field("frpes", &self.frpes())
            .field("estsel", &self.estsel())
            .field("estdep", &self.estdep())
            .field("estwid", &self.estwid())
            .field("fpesel", &self.fpesel())
            .field("tbssel", &self.tbssel())
            .field("asp", &self.asp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacHwFeature3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacHwFeature3 {{ nrvf: {:?}, cbtisel: {=bool:?}, dvlan: {=bool:?}, pdupsel: {=bool:?}, frpsel: {=bool:?}, frpbs: {:?}, frpes: {:?}, estsel: {=bool:?}, estdep: {:?}, estwid: {:?}, fpesel: {=bool:?}, tbssel: {=bool:?}, asp: {:?} }}",
            self.nrvf(),
            self.cbtisel(),
            self.dvlan(),
            self.pdupsel(),
            self.frpsel(),
            self.frpbs(),
            self.frpes(),
            self.estsel(),
            self.estdep(),
            self.estwid(),
            self.fpesel(),
            self.tbssel(),
            self.asp()
        )
    }
}
#[doc = "HW Features 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacHwFeature4(pub u32);
impl MacHwFeature4 {
    #[doc = "Policing Counters Selected"]
    #[must_use]
    #[inline(always)]
    pub const fn pcsel(&self) -> super::vals::Pcsel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Pcsel::from_bits(val as u8)
    }
    #[doc = "Policing Counters Selected"]
    #[inline(always)]
    pub const fn set_pcsel(&mut self, val: super::vals::Pcsel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for MacHwFeature4 {
    #[inline(always)]
    fn default() -> MacHwFeature4 {
        MacHwFeature4(0)
    }
}
impl core::fmt::Debug for MacHwFeature4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacHwFeature4")
            .field("pcsel", &self.pcsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacHwFeature4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacHwFeature4 {{ pcsel: {:?} }}", self.pcsel())
    }
}
#[doc = "Inner VLAN Tag Inclusion or Replacement"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacInnerVlanIncl(pub u32);
impl MacInnerVlanIncl {
    #[doc = "VLAN Tag for Transmit Packets"]
    #[must_use]
    #[inline(always)]
    pub const fn vlt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "VLAN Tag for Transmit Packets"]
    #[inline(always)]
    pub const fn set_vlt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "VLAN Tag Control in Transmit Packets"]
    #[must_use]
    #[inline(always)]
    pub const fn vlc(&self) -> super::vals::MacInnerVlanInclVlc {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::MacInnerVlanInclVlc::from_bits(val as u8)
    }
    #[doc = "VLAN Tag Control in Transmit Packets"]
    #[inline(always)]
    pub const fn set_vlc(&mut self, val: super::vals::MacInnerVlanInclVlc) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "VLAN Priority Control"]
    #[must_use]
    #[inline(always)]
    pub const fn vlp(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "VLAN Priority Control"]
    #[inline(always)]
    pub const fn set_vlp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "C-VLAN or S-VLAN"]
    #[must_use]
    #[inline(always)]
    pub const fn csvl(&self) -> super::vals::MacInnerVlanInclCsvl {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::MacInnerVlanInclCsvl::from_bits(val as u8)
    }
    #[doc = "C-VLAN or S-VLAN"]
    #[inline(always)]
    pub const fn set_csvl(&mut self, val: super::vals::MacInnerVlanInclCsvl) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "VLAN Tag Input"]
    #[must_use]
    #[inline(always)]
    pub const fn vlti(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "VLAN Tag Input"]
    #[inline(always)]
    pub const fn set_vlti(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for MacInnerVlanIncl {
    #[inline(always)]
    fn default() -> MacInnerVlanIncl {
        MacInnerVlanIncl(0)
    }
}
impl core::fmt::Debug for MacInnerVlanIncl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacInnerVlanIncl")
            .field("vlt", &self.vlt())
            .field("vlc", &self.vlc())
            .field("vlp", &self.vlp())
            .field("csvl", &self.csvl())
            .field("vlti", &self.vlti())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacInnerVlanIncl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacInnerVlanIncl {{ vlt: {=u16:?}, vlc: {:?}, vlp: {=bool:?}, csvl: {:?}, vlti: {=bool:?} }}",
            self.vlt(),
            self.vlc(),
            self.vlp(),
            self.csvl(),
            self.vlti()
        )
    }
}
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacInterruptEnable(pub u32);
impl MacInterruptEnable {
    #[doc = "PHY Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn phyie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "PHY Interrupt Enable"]
    #[inline(always)]
    pub const fn set_phyie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "PMT Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pmtie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "PMT Interrupt Enable"]
    #[inline(always)]
    pub const fn set_pmtie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Timestamp Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tsie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tsie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Transmit Status Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn txstsie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Status Interrupt Enable"]
    #[inline(always)]
    pub const fn set_txstsie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Receive Status Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rxstsie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Status Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rxstsie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "MDIO Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mdioie(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "MDIO Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mdioie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for MacInterruptEnable {
    #[inline(always)]
    fn default() -> MacInterruptEnable {
        MacInterruptEnable(0)
    }
}
impl core::fmt::Debug for MacInterruptEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacInterruptEnable")
            .field("phyie", &self.phyie())
            .field("pmtie", &self.pmtie())
            .field("tsie", &self.tsie())
            .field("txstsie", &self.txstsie())
            .field("rxstsie", &self.rxstsie())
            .field("mdioie", &self.mdioie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacInterruptEnable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacInterruptEnable {{ phyie: {=bool:?}, pmtie: {=bool:?}, tsie: {=bool:?}, txstsie: {=bool:?}, rxstsie: {=bool:?}, mdioie: {=bool:?} }}",
            self.phyie(),
            self.pmtie(),
            self.tsie(),
            self.txstsie(),
            self.rxstsie(),
            self.mdioie()
        )
    }
}
#[doc = "Interrupt Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacInterruptStatus(pub u32);
impl MacInterruptStatus {
    #[doc = "PHY Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn phyis(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "PHY Interrupt"]
    #[inline(always)]
    pub const fn set_phyis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "PMT Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn pmtis(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "PMT Interrupt Status"]
    #[inline(always)]
    pub const fn set_pmtis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Timestamp Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn tsis(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp Interrupt Status"]
    #[inline(always)]
    pub const fn set_tsis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Transmit Status Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn txstsis(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Status Interrupt"]
    #[inline(always)]
    pub const fn set_txstsis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Receive Status Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn rxstsis(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Status Interrupt"]
    #[inline(always)]
    pub const fn set_rxstsis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "MDIO Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn mdiois(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "MDIO Interrupt Status"]
    #[inline(always)]
    pub const fn set_mdiois(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for MacInterruptStatus {
    #[inline(always)]
    fn default() -> MacInterruptStatus {
        MacInterruptStatus(0)
    }
}
impl core::fmt::Debug for MacInterruptStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacInterruptStatus")
            .field("phyis", &self.phyis())
            .field("pmtis", &self.pmtis())
            .field("tsis", &self.tsis())
            .field("txstsis", &self.txstsis())
            .field("rxstsis", &self.rxstsis())
            .field("mdiois", &self.mdiois())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacInterruptStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacInterruptStatus {{ phyis: {=bool:?}, pmtis: {=bool:?}, tsis: {=bool:?}, txstsis: {=bool:?}, rxstsis: {=bool:?}, mdiois: {=bool:?} }}",
            self.phyis(),
            self.pmtis(),
            self.tsis(),
            self.txstsis(),
            self.rxstsis(),
            self.mdiois()
        )
    }
}
#[doc = "MDIO Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacMdioAddress(pub u32);
impl MacMdioAddress {
    #[doc = "GMII Busy"]
    #[must_use]
    #[inline(always)]
    pub const fn gb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GMII Busy"]
    #[inline(always)]
    pub const fn set_gb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clause 45 PHY Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn c45e(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Clause 45 PHY Enable"]
    #[inline(always)]
    pub const fn set_c45e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "GMII Operation Command 0"]
    #[must_use]
    #[inline(always)]
    pub const fn goc_0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "GMII Operation Command 0"]
    #[inline(always)]
    pub const fn set_goc_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "GMII Operation Command 1"]
    #[must_use]
    #[inline(always)]
    pub const fn goc_1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "GMII Operation Command 1"]
    #[inline(always)]
    pub const fn set_goc_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Skip Address Packet"]
    #[must_use]
    #[inline(always)]
    pub const fn skap(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Skip Address Packet"]
    #[inline(always)]
    pub const fn set_skap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CSR Clock Range"]
    #[must_use]
    #[inline(always)]
    pub const fn cr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "CSR Clock Range"]
    #[inline(always)]
    pub const fn set_cr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Number of Trailing Clocks"]
    #[must_use]
    #[inline(always)]
    pub const fn ntc(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Number of Trailing Clocks"]
    #[inline(always)]
    pub const fn set_ntc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Register or Device Address"]
    #[must_use]
    #[inline(always)]
    pub const fn rda(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Register or Device Address"]
    #[inline(always)]
    pub const fn set_rda(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Physical Layer Address"]
    #[must_use]
    #[inline(always)]
    pub const fn pa(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x1f;
        val as u8
    }
    #[doc = "Physical Layer Address"]
    #[inline(always)]
    pub const fn set_pa(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 21usize)) | (((val as u32) & 0x1f) << 21usize);
    }
    #[doc = "Back-to-Back Transactions"]
    #[must_use]
    #[inline(always)]
    pub const fn btb(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Back-to-Back Transactions"]
    #[inline(always)]
    pub const fn set_btb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Preamble Suppression Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pse(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Preamble Suppression Enable."]
    #[inline(always)]
    pub const fn set_pse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for MacMdioAddress {
    #[inline(always)]
    fn default() -> MacMdioAddress {
        MacMdioAddress(0)
    }
}
impl core::fmt::Debug for MacMdioAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacMdioAddress")
            .field("gb", &self.gb())
            .field("c45e", &self.c45e())
            .field("goc_0", &self.goc_0())
            .field("goc_1", &self.goc_1())
            .field("skap", &self.skap())
            .field("cr", &self.cr())
            .field("ntc", &self.ntc())
            .field("rda", &self.rda())
            .field("pa", &self.pa())
            .field("btb", &self.btb())
            .field("pse", &self.pse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacMdioAddress {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacMdioAddress {{ gb: {=bool:?}, c45e: {=bool:?}, goc_0: {=bool:?}, goc_1: {=bool:?}, skap: {=bool:?}, cr: {=u8:?}, ntc: {=u8:?}, rda: {=u8:?}, pa: {=u8:?}, btb: {=bool:?}, pse: {=bool:?} }}",
            self.gb(),
            self.c45e(),
            self.goc_0(),
            self.goc_1(),
            self.skap(),
            self.cr(),
            self.ntc(),
            self.rda(),
            self.pa(),
            self.btb(),
            self.pse()
        )
    }
}
#[doc = "MDIO Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacMdioData(pub u32);
impl MacMdioData {
    #[doc = "GMII Data"]
    #[must_use]
    #[inline(always)]
    pub const fn gd(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "GMII Data"]
    #[inline(always)]
    pub const fn set_gd(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Register Address"]
    #[must_use]
    #[inline(always)]
    pub const fn ra(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Register Address"]
    #[inline(always)]
    pub const fn set_ra(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MacMdioData {
    #[inline(always)]
    fn default() -> MacMdioData {
        MacMdioData(0)
    }
}
impl core::fmt::Debug for MacMdioData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacMdioData")
            .field("gd", &self.gd())
            .field("ra", &self.ra())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacMdioData {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacMdioData {{ gd: {=u16:?}, ra: {=u16:?} }}",
            self.gd(),
            self.ra()
        )
    }
}
#[doc = "MAC Packet Filter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacPacketFilter(pub u32);
impl MacPacketFilter {
    #[doc = "Promiscuous Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Promiscuous Mode"]
    #[inline(always)]
    pub const fn set_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DA Inverse Filtering"]
    #[must_use]
    #[inline(always)]
    pub const fn daif(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DA Inverse Filtering"]
    #[inline(always)]
    pub const fn set_daif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Pass All Multicast"]
    #[must_use]
    #[inline(always)]
    pub const fn pm(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pass All Multicast"]
    #[inline(always)]
    pub const fn set_pm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Disable Broadcast Packets"]
    #[must_use]
    #[inline(always)]
    pub const fn dbf(&self) -> super::vals::Dbf {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Dbf::from_bits(val as u8)
    }
    #[doc = "Disable Broadcast Packets"]
    #[inline(always)]
    pub const fn set_dbf(&mut self, val: super::vals::Dbf) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Pass Control Packets"]
    #[must_use]
    #[inline(always)]
    pub const fn pcf(&self) -> super::vals::Pcf {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Pcf::from_bits(val as u8)
    }
    #[doc = "Pass Control Packets"]
    #[inline(always)]
    pub const fn set_pcf(&mut self, val: super::vals::Pcf) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "VLAN Tag Filter Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vtfe(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "VLAN Tag Filter Enable"]
    #[inline(always)]
    pub const fn set_vtfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Receive All"]
    #[must_use]
    #[inline(always)]
    pub const fn ra(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Receive All"]
    #[inline(always)]
    pub const fn set_ra(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for MacPacketFilter {
    #[inline(always)]
    fn default() -> MacPacketFilter {
        MacPacketFilter(0)
    }
}
impl core::fmt::Debug for MacPacketFilter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacPacketFilter")
            .field("pr", &self.pr())
            .field("daif", &self.daif())
            .field("pm", &self.pm())
            .field("dbf", &self.dbf())
            .field("pcf", &self.pcf())
            .field("vtfe", &self.vtfe())
            .field("ra", &self.ra())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacPacketFilter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacPacketFilter {{ pr: {=bool:?}, daif: {=bool:?}, pm: {=bool:?}, dbf: {:?}, pcf: {:?}, vtfe: {=bool:?}, ra: {=bool:?} }}",
            self.pr(),
            self.daif(),
            self.pm(),
            self.dbf(),
            self.pcf(),
            self.vtfe(),
            self.ra()
        )
    }
}
#[doc = "PMT Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacPmtControlStatus(pub u32);
impl MacPmtControlStatus {
    #[doc = "Power Down"]
    #[must_use]
    #[inline(always)]
    pub const fn pwrdwn(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down"]
    #[inline(always)]
    pub const fn set_pwrdwn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Magic Packet Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mgkpkten(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Magic Packet Enable"]
    #[inline(always)]
    pub const fn set_mgkpkten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Magic Packet Received"]
    #[must_use]
    #[inline(always)]
    pub const fn mgkprcvd(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Magic Packet Received"]
    #[inline(always)]
    pub const fn set_mgkprcvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for MacPmtControlStatus {
    #[inline(always)]
    fn default() -> MacPmtControlStatus {
        MacPmtControlStatus(0)
    }
}
impl core::fmt::Debug for MacPmtControlStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacPmtControlStatus")
            .field("pwrdwn", &self.pwrdwn())
            .field("mgkpkten", &self.mgkpkten())
            .field("mgkprcvd", &self.mgkprcvd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacPmtControlStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacPmtControlStatus {{ pwrdwn: {=bool:?}, mgkpkten: {=bool:?}, mgkprcvd: {=bool:?} }}",
            self.pwrdwn(),
            self.mgkpkten(),
            self.mgkprcvd()
        )
    }
}
#[doc = "PPS0 Target Time Nanoseconds"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacPps0TargetTimeNanoseconds(pub u32);
impl MacPps0TargetTimeNanoseconds {
    #[doc = "Target Time Low for PPS Register"]
    #[must_use]
    #[inline(always)]
    pub const fn ttsl0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Target Time Low for PPS Register"]
    #[inline(always)]
    pub const fn set_ttsl0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
}
impl Default for MacPps0TargetTimeNanoseconds {
    #[inline(always)]
    fn default() -> MacPps0TargetTimeNanoseconds {
        MacPps0TargetTimeNanoseconds(0)
    }
}
impl core::fmt::Debug for MacPps0TargetTimeNanoseconds {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacPps0TargetTimeNanoseconds")
            .field("ttsl0", &self.ttsl0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacPps0TargetTimeNanoseconds {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacPps0TargetTimeNanoseconds {{ ttsl0: {=u32:?} }}",
            self.ttsl0()
        )
    }
}
#[doc = "PPS Target Time Seconds"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacPps0TargetTimeSeconds(pub u32);
impl MacPps0TargetTimeSeconds {
    #[doc = "PPS Target Time Seconds Register"]
    #[must_use]
    #[inline(always)]
    pub const fn tstrh0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "PPS Target Time Seconds Register"]
    #[inline(always)]
    pub const fn set_tstrh0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MacPps0TargetTimeSeconds {
    #[inline(always)]
    fn default() -> MacPps0TargetTimeSeconds {
        MacPps0TargetTimeSeconds(0)
    }
}
impl core::fmt::Debug for MacPps0TargetTimeSeconds {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacPps0TargetTimeSeconds")
            .field("tstrh0", &self.tstrh0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacPps0TargetTimeSeconds {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacPps0TargetTimeSeconds {{ tstrh0: {=u32:?} }}",
            self.tstrh0()
        )
    }
}
#[doc = "PPS Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacPpsControl(pub u32);
impl MacPpsControl {
    #[doc = "Timestamp Sub Seconds"]
    #[must_use]
    #[inline(always)]
    pub const fn ppsctrl_ppscmd(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Timestamp Sub Seconds"]
    #[inline(always)]
    pub const fn set_ppsctrl_ppscmd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for MacPpsControl {
    #[inline(always)]
    fn default() -> MacPpsControl {
        MacPpsControl(0)
    }
}
impl core::fmt::Debug for MacPpsControl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacPpsControl")
            .field("ppsctrl_ppscmd", &self.ppsctrl_ppscmd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacPpsControl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacPpsControl {{ ppsctrl_ppscmd: {=u8:?} }}",
            self.ppsctrl_ppscmd()
        )
    }
}
#[doc = "Queue 0 Transmit Flow Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacQ0TxFlowCtrl(pub u32);
impl MacQ0TxFlowCtrl {
    #[doc = "Flow Control Busy or Backpressure Activate"]
    #[must_use]
    #[inline(always)]
    pub const fn fcb_bpa(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Flow Control Busy or Backpressure Activate"]
    #[inline(always)]
    pub const fn set_fcb_bpa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit Flow Control Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tfe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Flow Control Enable"]
    #[inline(always)]
    pub const fn set_tfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pause Low Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn plt(&self) -> super::vals::Plt {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Plt::from_bits(val as u8)
    }
    #[doc = "Pause Low Threshold"]
    #[inline(always)]
    pub const fn set_plt(&mut self, val: super::vals::Plt) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Disable Zero-Quanta Pause"]
    #[must_use]
    #[inline(always)]
    pub const fn dzpq(&self) -> super::vals::Dzpq {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Dzpq::from_bits(val as u8)
    }
    #[doc = "Disable Zero-Quanta Pause"]
    #[inline(always)]
    pub const fn set_dzpq(&mut self, val: super::vals::Dzpq) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Pause Time"]
    #[must_use]
    #[inline(always)]
    pub const fn pt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Pause Time"]
    #[inline(always)]
    pub const fn set_pt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MacQ0TxFlowCtrl {
    #[inline(always)]
    fn default() -> MacQ0TxFlowCtrl {
        MacQ0TxFlowCtrl(0)
    }
}
impl core::fmt::Debug for MacQ0TxFlowCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacQ0TxFlowCtrl")
            .field("fcb_bpa", &self.fcb_bpa())
            .field("tfe", &self.tfe())
            .field("plt", &self.plt())
            .field("dzpq", &self.dzpq())
            .field("pt", &self.pt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacQ0TxFlowCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacQ0TxFlowCtrl {{ fcb_bpa: {=bool:?}, tfe: {=bool:?}, plt: {:?}, dzpq: {:?}, pt: {=u16:?} }}",
            self.fcb_bpa(),
            self.tfe(),
            self.plt(),
            self.dzpq(),
            self.pt()
        )
    }
}
#[doc = "Receive Domain TIme Increment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacRxDomainTimeIncr(pub u32);
impl MacRxDomainTimeIncr {
    #[doc = "Receive Domain Time Increment Value in Nanoseconds."]
    #[must_use]
    #[inline(always)]
    pub const fn rxns(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Receive Domain Time Increment Value in Nanoseconds."]
    #[inline(always)]
    pub const fn set_rxns(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MacRxDomainTimeIncr {
    #[inline(always)]
    fn default() -> MacRxDomainTimeIncr {
        MacRxDomainTimeIncr(0)
    }
}
impl core::fmt::Debug for MacRxDomainTimeIncr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacRxDomainTimeIncr")
            .field("rxns", &self.rxns())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacRxDomainTimeIncr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacRxDomainTimeIncr {{ rxns: {=u16:?} }}", self.rxns())
    }
}
#[doc = "Receive Flow Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacRxFlowCtrl(pub u32);
impl MacRxFlowCtrl {
    #[doc = "Receive Flow Control Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rfe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Flow Control Enable"]
    #[inline(always)]
    pub const fn set_rfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Unicast Pause Packet Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn up(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Unicast Pause Packet Detect"]
    #[inline(always)]
    pub const fn set_up(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for MacRxFlowCtrl {
    #[inline(always)]
    fn default() -> MacRxFlowCtrl {
        MacRxFlowCtrl(0)
    }
}
impl core::fmt::Debug for MacRxFlowCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacRxFlowCtrl")
            .field("rfe", &self.rfe())
            .field("up", &self.up())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacRxFlowCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacRxFlowCtrl {{ rfe: {=bool:?}, up: {=bool:?} }}",
            self.rfe(),
            self.up()
        )
    }
}
#[doc = "Receive Transmit Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacRxTxStatus(pub u32);
impl MacRxTxStatus {
    #[doc = "Transmit Jabber Timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn tjt(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Jabber Timeout"]
    #[inline(always)]
    pub const fn set_tjt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "No Carrier"]
    #[must_use]
    #[inline(always)]
    pub const fn ncarr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "No Carrier"]
    #[inline(always)]
    pub const fn set_ncarr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Loss of Carrier"]
    #[must_use]
    #[inline(always)]
    pub const fn lcarr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Loss of Carrier"]
    #[inline(always)]
    pub const fn set_lcarr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Excessive Deferral"]
    #[must_use]
    #[inline(always)]
    pub const fn exdef(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Excessive Deferral"]
    #[inline(always)]
    pub const fn set_exdef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Late Collision"]
    #[must_use]
    #[inline(always)]
    pub const fn lcol(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Late Collision"]
    #[inline(always)]
    pub const fn set_lcol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Excessive Collisions"]
    #[must_use]
    #[inline(always)]
    pub const fn excol(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Excessive Collisions"]
    #[inline(always)]
    pub const fn set_excol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Receive Watchdog Timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn rwt(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Watchdog Timeout"]
    #[inline(always)]
    pub const fn set_rwt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for MacRxTxStatus {
    #[inline(always)]
    fn default() -> MacRxTxStatus {
        MacRxTxStatus(0)
    }
}
impl core::fmt::Debug for MacRxTxStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacRxTxStatus")
            .field("tjt", &self.tjt())
            .field("ncarr", &self.ncarr())
            .field("lcarr", &self.lcarr())
            .field("exdef", &self.exdef())
            .field("lcol", &self.lcol())
            .field("excol", &self.excol())
            .field("rwt", &self.rwt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacRxTxStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacRxTxStatus {{ tjt: {=bool:?}, ncarr: {=bool:?}, lcarr: {=bool:?}, exdef: {=bool:?}, lcol: {=bool:?}, excol: {=bool:?}, rwt: {=bool:?} }}",
            self.tjt(),
            self.ncarr(),
            self.lcarr(),
            self.exdef(),
            self.lcol(),
            self.excol(),
            self.rwt()
        )
    }
}
#[doc = "Sub Second Increment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacSubSecondIncrement(pub u32);
impl MacSubSecondIncrement {
    #[doc = "Sub-second Increment Value"]
    #[must_use]
    #[inline(always)]
    pub const fn ssinc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Sub-second Increment Value"]
    #[inline(always)]
    pub const fn set_ssinc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for MacSubSecondIncrement {
    #[inline(always)]
    fn default() -> MacSubSecondIncrement {
        MacSubSecondIncrement(0)
    }
}
impl core::fmt::Debug for MacSubSecondIncrement {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacSubSecondIncrement")
            .field("ssinc", &self.ssinc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacSubSecondIncrement {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacSubSecondIncrement {{ ssinc: {=u8:?} }}",
            self.ssinc()
        )
    }
}
#[doc = "System Time Nanoseconds"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacSystemTimeNanoseconds(pub u32);
impl MacSystemTimeNanoseconds {
    #[doc = "Timestamp Sub Seconds"]
    #[must_use]
    #[inline(always)]
    pub const fn tsss(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Timestamp Sub Seconds"]
    #[inline(always)]
    pub const fn set_tsss(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
}
impl Default for MacSystemTimeNanoseconds {
    #[inline(always)]
    fn default() -> MacSystemTimeNanoseconds {
        MacSystemTimeNanoseconds(0)
    }
}
impl core::fmt::Debug for MacSystemTimeNanoseconds {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacSystemTimeNanoseconds")
            .field("tsss", &self.tsss())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacSystemTimeNanoseconds {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacSystemTimeNanoseconds {{ tsss: {=u32:?} }}",
            self.tsss()
        )
    }
}
#[doc = "MAC System Time Nanoseconds Update"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacSystemTimeNanosecondsUpdate(pub u32);
impl MacSystemTimeNanosecondsUpdate {
    #[doc = "Timestamp Sub Seconds"]
    #[must_use]
    #[inline(always)]
    pub const fn tsss(&self) -> super::vals::Tsss {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        super::vals::Tsss::from_bits(val as u32)
    }
    #[doc = "Timestamp Sub Seconds"]
    #[inline(always)]
    pub const fn set_tsss(&mut self, val: super::vals::Tsss) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0x7fff_ffff) << 0usize);
    }
    #[doc = "Add or Subtract Time"]
    #[must_use]
    #[inline(always)]
    pub const fn addsub(&self) -> super::vals::Addsub {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Addsub::from_bits(val as u8)
    }
    #[doc = "Add or Subtract Time"]
    #[inline(always)]
    pub const fn set_addsub(&mut self, val: super::vals::Addsub) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MacSystemTimeNanosecondsUpdate {
    #[inline(always)]
    fn default() -> MacSystemTimeNanosecondsUpdate {
        MacSystemTimeNanosecondsUpdate(0)
    }
}
impl core::fmt::Debug for MacSystemTimeNanosecondsUpdate {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacSystemTimeNanosecondsUpdate")
            .field("tsss", &self.tsss())
            .field("addsub", &self.addsub())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacSystemTimeNanosecondsUpdate {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacSystemTimeNanosecondsUpdate {{ tsss: {:?}, addsub: {:?} }}",
            self.tsss(),
            self.addsub()
        )
    }
}
#[doc = "System Time Seconds"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacSystemTimeSeconds(pub u32);
impl MacSystemTimeSeconds {
    #[doc = "Timestamp Second"]
    #[must_use]
    #[inline(always)]
    pub const fn tss(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timestamp Second"]
    #[inline(always)]
    pub const fn set_tss(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MacSystemTimeSeconds {
    #[inline(always)]
    fn default() -> MacSystemTimeSeconds {
        MacSystemTimeSeconds(0)
    }
}
impl core::fmt::Debug for MacSystemTimeSeconds {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacSystemTimeSeconds")
            .field("tss", &self.tss())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacSystemTimeSeconds {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacSystemTimeSeconds {{ tss: {=u32:?} }}", self.tss())
    }
}
#[doc = "System Time Seconds Update"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacSystemTimeSecondsUpdate(pub u32);
impl MacSystemTimeSecondsUpdate {
    #[doc = "Timestamp Seconds"]
    #[must_use]
    #[inline(always)]
    pub const fn tss(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timestamp Seconds"]
    #[inline(always)]
    pub const fn set_tss(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MacSystemTimeSecondsUpdate {
    #[inline(always)]
    fn default() -> MacSystemTimeSecondsUpdate {
        MacSystemTimeSecondsUpdate(0)
    }
}
impl core::fmt::Debug for MacSystemTimeSecondsUpdate {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacSystemTimeSecondsUpdate")
            .field("tss", &self.tss())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacSystemTimeSecondsUpdate {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacSystemTimeSecondsUpdate {{ tss: {=u32:?} }}",
            self.tss()
        )
    }
}
#[doc = "Timestamp Addend"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacTimestampAddend(pub u32);
impl MacTimestampAddend {
    #[doc = "Timestamp Addend Register"]
    #[must_use]
    #[inline(always)]
    pub const fn tsar(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timestamp Addend Register"]
    #[inline(always)]
    pub const fn set_tsar(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MacTimestampAddend {
    #[inline(always)]
    fn default() -> MacTimestampAddend {
        MacTimestampAddend(0)
    }
}
impl core::fmt::Debug for MacTimestampAddend {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacTimestampAddend")
            .field("tsar", &self.tsar())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacTimestampAddend {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacTimestampAddend {{ tsar: {=u32:?} }}", self.tsar())
    }
}
#[doc = "Timestamp Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacTimestampControl(pub u32);
impl MacTimestampControl {
    #[doc = "Enable Timestamp"]
    #[must_use]
    #[inline(always)]
    pub const fn tsena(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Timestamp"]
    #[inline(always)]
    pub const fn set_tsena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Fine or Coarse Timestamp Update"]
    #[must_use]
    #[inline(always)]
    pub const fn tscfupdt(&self) -> super::vals::Tscfupdt {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Tscfupdt::from_bits(val as u8)
    }
    #[doc = "Fine or Coarse Timestamp Update"]
    #[inline(always)]
    pub const fn set_tscfupdt(&mut self, val: super::vals::Tscfupdt) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Initialize Timestamp"]
    #[must_use]
    #[inline(always)]
    pub const fn tsinit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Initialize Timestamp"]
    #[inline(always)]
    pub const fn set_tsinit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Update Timestamp"]
    #[must_use]
    #[inline(always)]
    pub const fn tsupdt(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Update Timestamp"]
    #[inline(always)]
    pub const fn set_tsupdt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable Timestamp Interrupt Trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn tstrig(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Timestamp Interrupt Trigger"]
    #[inline(always)]
    pub const fn set_tstrig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Update Addend Register"]
    #[must_use]
    #[inline(always)]
    pub const fn tsaddreg(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Update Addend Register"]
    #[inline(always)]
    pub const fn set_tsaddreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable Timestamp for All Packets"]
    #[must_use]
    #[inline(always)]
    pub const fn tsenall(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Timestamp for All Packets"]
    #[inline(always)]
    pub const fn set_tsenall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Timestamp Digital or Binary Rollover Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tsctrlssr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp Digital or Binary Rollover Control"]
    #[inline(always)]
    pub const fn set_tsctrlssr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Enable PTP Packet Processing for Version 2 Format"]
    #[must_use]
    #[inline(always)]
    pub const fn tsver2ena(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PTP Packet Processing for Version 2 Format"]
    #[inline(always)]
    pub const fn set_tsver2ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable Processing of PTP over Ethernet Packets"]
    #[must_use]
    #[inline(always)]
    pub const fn tsipena(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Processing of PTP over Ethernet Packets"]
    #[inline(always)]
    pub const fn set_tsipena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable Processing of PTP Packets Sent over IPv6-UDP"]
    #[must_use]
    #[inline(always)]
    pub const fn tsipv6ena(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Processing of PTP Packets Sent over IPv6-UDP"]
    #[inline(always)]
    pub const fn set_tsipv6ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable Processing of PTP Packets Sent over IPv4-UDP"]
    #[must_use]
    #[inline(always)]
    pub const fn tsipv4ena(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Processing of PTP Packets Sent over IPv4-UDP"]
    #[inline(always)]
    pub const fn set_tsipv4ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enable Timestamp Snapshot for Event Messages"]
    #[must_use]
    #[inline(always)]
    pub const fn tsevntena(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Timestamp Snapshot for Event Messages"]
    #[inline(always)]
    pub const fn set_tsevntena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enable Snapshot for Messages Relevant to Master"]
    #[must_use]
    #[inline(always)]
    pub const fn tsmstrena(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Snapshot for Messages Relevant to Master"]
    #[inline(always)]
    pub const fn set_tsmstrena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Select PTP packets for Taking Snapshots"]
    #[must_use]
    #[inline(always)]
    pub const fn snaptypsel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Select PTP packets for Taking Snapshots"]
    #[inline(always)]
    pub const fn set_snaptypsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Enable MAC Address for PTP Packet Filtering"]
    #[must_use]
    #[inline(always)]
    pub const fn tsenmacaddr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enable MAC Address for PTP Packet Filtering"]
    #[inline(always)]
    pub const fn set_tsenmacaddr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "External System Time Input"]
    #[must_use]
    #[inline(always)]
    pub const fn esti(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "External System Time Input"]
    #[inline(always)]
    pub const fn set_esti(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Transmit Timestamp Status Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn txtsstsm(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Timestamp Status Mode"]
    #[inline(always)]
    pub const fn set_txtsstsm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "AV 802.1AS Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn av8021asmen(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "AV 802.1AS Mode Enable"]
    #[inline(always)]
    pub const fn set_av8021asmen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for MacTimestampControl {
    #[inline(always)]
    fn default() -> MacTimestampControl {
        MacTimestampControl(0)
    }
}
impl core::fmt::Debug for MacTimestampControl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacTimestampControl")
            .field("tsena", &self.tsena())
            .field("tscfupdt", &self.tscfupdt())
            .field("tsinit", &self.tsinit())
            .field("tsupdt", &self.tsupdt())
            .field("tstrig", &self.tstrig())
            .field("tsaddreg", &self.tsaddreg())
            .field("tsenall", &self.tsenall())
            .field("tsctrlssr", &self.tsctrlssr())
            .field("tsver2ena", &self.tsver2ena())
            .field("tsipena", &self.tsipena())
            .field("tsipv6ena", &self.tsipv6ena())
            .field("tsipv4ena", &self.tsipv4ena())
            .field("tsevntena", &self.tsevntena())
            .field("tsmstrena", &self.tsmstrena())
            .field("snaptypsel", &self.snaptypsel())
            .field("tsenmacaddr", &self.tsenmacaddr())
            .field("esti", &self.esti())
            .field("txtsstsm", &self.txtsstsm())
            .field("av8021asmen", &self.av8021asmen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacTimestampControl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacTimestampControl {{ tsena: {=bool:?}, tscfupdt: {:?}, tsinit: {=bool:?}, tsupdt: {=bool:?}, tstrig: {=bool:?}, tsaddreg: {=bool:?}, tsenall: {=bool:?}, tsctrlssr: {=bool:?}, tsver2ena: {=bool:?}, tsipena: {=bool:?}, tsipv6ena: {=bool:?}, tsipv4ena: {=bool:?}, tsevntena: {=bool:?}, tsmstrena: {=bool:?}, snaptypsel: {=u8:?}, tsenmacaddr: {=bool:?}, esti: {=bool:?}, txtsstsm: {=bool:?}, av8021asmen: {=bool:?} }}",
            self.tsena(),
            self.tscfupdt(),
            self.tsinit(),
            self.tsupdt(),
            self.tstrig(),
            self.tsaddreg(),
            self.tsenall(),
            self.tsctrlssr(),
            self.tsver2ena(),
            self.tsipena(),
            self.tsipv6ena(),
            self.tsipv4ena(),
            self.tsevntena(),
            self.tsmstrena(),
            self.snaptypsel(),
            self.tsenmacaddr(),
            self.esti(),
            self.txtsstsm(),
            self.av8021asmen()
        )
    }
}
#[doc = "MAC Timestamp Egress Correction Nanosecond"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacTimestampEgressCorrNanosecond(pub u32);
impl MacTimestampEgressCorrNanosecond {
    #[doc = "Timestamp Egress Correction"]
    #[must_use]
    #[inline(always)]
    pub const fn tsec(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timestamp Egress Correction"]
    #[inline(always)]
    pub const fn set_tsec(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MacTimestampEgressCorrNanosecond {
    #[inline(always)]
    fn default() -> MacTimestampEgressCorrNanosecond {
        MacTimestampEgressCorrNanosecond(0)
    }
}
impl core::fmt::Debug for MacTimestampEgressCorrNanosecond {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacTimestampEgressCorrNanosecond")
            .field("tsec", &self.tsec())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacTimestampEgressCorrNanosecond {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacTimestampEgressCorrNanosecond {{ tsec: {=u32:?} }}",
            self.tsec()
        )
    }
}
#[doc = "MAC Timestamp Egress Latency"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacTimestampEgressLatency(pub u32);
impl MacTimestampEgressLatency {
    #[doc = "Egress Timestamp Latency, in sub-nanoseconds"]
    #[must_use]
    #[inline(always)]
    pub const fn etlsns(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Egress Timestamp Latency, in sub-nanoseconds"]
    #[inline(always)]
    pub const fn set_etlsns(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Egress Timestamp Latency, in nanoseconds"]
    #[must_use]
    #[inline(always)]
    pub const fn etlns(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Egress Timestamp Latency, in nanoseconds"]
    #[inline(always)]
    pub const fn set_etlns(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for MacTimestampEgressLatency {
    #[inline(always)]
    fn default() -> MacTimestampEgressLatency {
        MacTimestampEgressLatency(0)
    }
}
impl core::fmt::Debug for MacTimestampEgressLatency {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacTimestampEgressLatency")
            .field("etlsns", &self.etlsns())
            .field("etlns", &self.etlns())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacTimestampEgressLatency {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacTimestampEgressLatency {{ etlsns: {=u8:?}, etlns: {=u16:?} }}",
            self.etlsns(),
            self.etlns()
        )
    }
}
#[doc = "MAC Timestamp Ingress Correction Nanosecond"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacTimestampIngressCorrNanosecond(pub u32);
impl MacTimestampIngressCorrNanosecond {
    #[doc = "Timestamp Ingress Correction"]
    #[must_use]
    #[inline(always)]
    pub const fn tsic(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timestamp Ingress Correction"]
    #[inline(always)]
    pub const fn set_tsic(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MacTimestampIngressCorrNanosecond {
    #[inline(always)]
    fn default() -> MacTimestampIngressCorrNanosecond {
        MacTimestampIngressCorrNanosecond(0)
    }
}
impl core::fmt::Debug for MacTimestampIngressCorrNanosecond {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacTimestampIngressCorrNanosecond")
            .field("tsic", &self.tsic())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacTimestampIngressCorrNanosecond {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacTimestampIngressCorrNanosecond {{ tsic: {=u32:?} }}",
            self.tsic()
        )
    }
}
#[doc = "MAC Timestamp Ingress Latency"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacTimestampIngressLatency(pub u32);
impl MacTimestampIngressLatency {
    #[doc = "Ingress Timestamp Latency, in sub-nanoseconds"]
    #[must_use]
    #[inline(always)]
    pub const fn itlsns(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Ingress Timestamp Latency, in sub-nanoseconds"]
    #[inline(always)]
    pub const fn set_itlsns(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Ingress Timestamp Latency, in nanoseconds"]
    #[must_use]
    #[inline(always)]
    pub const fn itlns(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Ingress Timestamp Latency, in nanoseconds"]
    #[inline(always)]
    pub const fn set_itlns(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for MacTimestampIngressLatency {
    #[inline(always)]
    fn default() -> MacTimestampIngressLatency {
        MacTimestampIngressLatency(0)
    }
}
impl core::fmt::Debug for MacTimestampIngressLatency {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacTimestampIngressLatency")
            .field("itlsns", &self.itlsns())
            .field("itlns", &self.itlns())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacTimestampIngressLatency {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacTimestampIngressLatency {{ itlsns: {=u8:?}, itlns: {=u16:?} }}",
            self.itlsns(),
            self.itlns()
        )
    }
}
#[doc = "Timestamp Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacTimestampStatus(pub u32);
impl MacTimestampStatus {
    #[doc = "Timestamp Seconds Overflow"]
    #[must_use]
    #[inline(always)]
    pub const fn tssovf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp Seconds Overflow"]
    #[inline(always)]
    pub const fn set_tssovf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Timestamp Target Time Reached"]
    #[must_use]
    #[inline(always)]
    pub const fn tstargt0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp Target Time Reached"]
    #[inline(always)]
    pub const fn set_tstargt0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Auxiliary Timestamp Trigger Snapshot"]
    #[must_use]
    #[inline(always)]
    pub const fn auxtstrig(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Auxiliary Timestamp Trigger Snapshot"]
    #[inline(always)]
    pub const fn set_auxtstrig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Timestamp Target Time Error"]
    #[must_use]
    #[inline(always)]
    pub const fn tstrgterr0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp Target Time Error"]
    #[inline(always)]
    pub const fn set_tstrgterr0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Tx Timestamp Status Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn txtssis(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Timestamp Status Interrupt Status"]
    #[inline(always)]
    pub const fn set_txtssis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Auxiliary Timestamp Snapshot Trigger Identifier"]
    #[must_use]
    #[inline(always)]
    pub const fn atsstn(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Auxiliary Timestamp Snapshot Trigger Identifier"]
    #[inline(always)]
    pub const fn set_atsstn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Auxiliary Timestamp Snapshot Trigger Missed"]
    #[must_use]
    #[inline(always)]
    pub const fn atsstm(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Auxiliary Timestamp Snapshot Trigger Missed"]
    #[inline(always)]
    pub const fn set_atsstm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Number of Auxiliary Timestamp Snapshots"]
    #[must_use]
    #[inline(always)]
    pub const fn atsns(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of Auxiliary Timestamp Snapshots"]
    #[inline(always)]
    pub const fn set_atsns(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
    }
}
impl Default for MacTimestampStatus {
    #[inline(always)]
    fn default() -> MacTimestampStatus {
        MacTimestampStatus(0)
    }
}
impl core::fmt::Debug for MacTimestampStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacTimestampStatus")
            .field("tssovf", &self.tssovf())
            .field("tstargt0", &self.tstargt0())
            .field("auxtstrig", &self.auxtstrig())
            .field("tstrgterr0", &self.tstrgterr0())
            .field("txtssis", &self.txtssis())
            .field("atsstn", &self.atsstn())
            .field("atsstm", &self.atsstm())
            .field("atsns", &self.atsns())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacTimestampStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacTimestampStatus {{ tssovf: {=bool:?}, tstargt0: {=bool:?}, auxtstrig: {=bool:?}, tstrgterr0: {=bool:?}, txtssis: {=bool:?}, atsstn: {=u8:?}, atsstm: {=bool:?}, atsns: {=u8:?} }}",
            self.tssovf(),
            self.tstargt0(),
            self.auxtstrig(),
            self.tstrgterr0(),
            self.txtssis(),
            self.atsstn(),
            self.atsstm(),
            self.atsns()
        )
    }
}
#[doc = "Transmit Domain TIme Increment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacTxDomainTimeIncr(pub u32);
impl MacTxDomainTimeIncr {
    #[doc = "Transmit Domain Time Increment Value in Nanoseconds."]
    #[must_use]
    #[inline(always)]
    pub const fn txns(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Transmit Domain Time Increment Value in Nanoseconds."]
    #[inline(always)]
    pub const fn set_txns(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MacTxDomainTimeIncr {
    #[inline(always)]
    fn default() -> MacTxDomainTimeIncr {
        MacTxDomainTimeIncr(0)
    }
}
impl core::fmt::Debug for MacTxDomainTimeIncr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacTxDomainTimeIncr")
            .field("txns", &self.txns())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacTxDomainTimeIncr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacTxDomainTimeIncr {{ txns: {=u16:?} }}", self.txns())
    }
}
#[doc = "Transmit Timestamp Status Nanoseconds"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacTxTimestampStatusNanoseconds(pub u32);
impl MacTxTimestampStatusNanoseconds {
    #[doc = "Transmit Timestamp Status Low"]
    #[must_use]
    #[inline(always)]
    pub const fn txtsslo(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Transmit Timestamp Status Low"]
    #[inline(always)]
    pub const fn set_txtsslo(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
    #[doc = "Transmit Timestamp Status Missed"]
    #[must_use]
    #[inline(always)]
    pub const fn txtssmis(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Timestamp Status Missed"]
    #[inline(always)]
    pub const fn set_txtssmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for MacTxTimestampStatusNanoseconds {
    #[inline(always)]
    fn default() -> MacTxTimestampStatusNanoseconds {
        MacTxTimestampStatusNanoseconds(0)
    }
}
impl core::fmt::Debug for MacTxTimestampStatusNanoseconds {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacTxTimestampStatusNanoseconds")
            .field("txtsslo", &self.txtsslo())
            .field("txtssmis", &self.txtssmis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacTxTimestampStatusNanoseconds {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacTxTimestampStatusNanoseconds {{ txtsslo: {=u32:?}, txtssmis: {=bool:?} }}",
            self.txtsslo(),
            self.txtssmis()
        )
    }
}
#[doc = "Transmit Timestamp Status Seconds"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacTxTimestampStatusSeconds(pub u32);
impl MacTxTimestampStatusSeconds {
    #[doc = "Transmit Timestamp Status High"]
    #[must_use]
    #[inline(always)]
    pub const fn txtsshi(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmit Timestamp Status High"]
    #[inline(always)]
    pub const fn set_txtsshi(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MacTxTimestampStatusSeconds {
    #[inline(always)]
    fn default() -> MacTxTimestampStatusSeconds {
        MacTxTimestampStatusSeconds(0)
    }
}
impl core::fmt::Debug for MacTxTimestampStatusSeconds {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacTxTimestampStatusSeconds")
            .field("txtsshi", &self.txtsshi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacTxTimestampStatusSeconds {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacTxTimestampStatusSeconds {{ txtsshi: {=u32:?} }}",
            self.txtsshi()
        )
    }
}
#[doc = "Version"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacVersion(pub u32);
impl MacVersion {
    #[doc = "Synopsys-defined Version"]
    #[must_use]
    #[inline(always)]
    pub const fn snpsver(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Synopsys-defined Version"]
    #[inline(always)]
    pub const fn set_snpsver(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "User-defined Version (configured with coreConsultant)"]
    #[must_use]
    #[inline(always)]
    pub const fn userver(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "User-defined Version (configured with coreConsultant)"]
    #[inline(always)]
    pub const fn set_userver(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for MacVersion {
    #[inline(always)]
    fn default() -> MacVersion {
        MacVersion(0)
    }
}
impl core::fmt::Debug for MacVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacVersion")
            .field("snpsver", &self.snpsver())
            .field("userver", &self.userver())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacVersion {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacVersion {{ snpsver: {=u8:?}, userver: {=u8:?} }}",
            self.snpsver(),
            self.userver()
        )
    }
}
#[doc = "VLAN Tag Inclusion or Replacement"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacVlanIncl(pub u32);
impl MacVlanIncl {
    #[doc = "VLAN Tag for Transmit Packets"]
    #[must_use]
    #[inline(always)]
    pub const fn vlt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "VLAN Tag for Transmit Packets"]
    #[inline(always)]
    pub const fn set_vlt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "VLAN Tag Control in Transmit Packets"]
    #[must_use]
    #[inline(always)]
    pub const fn vlc(&self) -> super::vals::MacVlanInclVlc {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::MacVlanInclVlc::from_bits(val as u8)
    }
    #[doc = "VLAN Tag Control in Transmit Packets"]
    #[inline(always)]
    pub const fn set_vlc(&mut self, val: super::vals::MacVlanInclVlc) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "VLAN Priority Control"]
    #[must_use]
    #[inline(always)]
    pub const fn vlp(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "VLAN Priority Control"]
    #[inline(always)]
    pub const fn set_vlp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "C-VLAN or S-VLAN"]
    #[must_use]
    #[inline(always)]
    pub const fn csvl(&self) -> super::vals::MacVlanInclCsvl {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::MacVlanInclCsvl::from_bits(val as u8)
    }
    #[doc = "C-VLAN or S-VLAN"]
    #[inline(always)]
    pub const fn set_csvl(&mut self, val: super::vals::MacVlanInclCsvl) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "VLAN Tag Input"]
    #[must_use]
    #[inline(always)]
    pub const fn vlti(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "VLAN Tag Input"]
    #[inline(always)]
    pub const fn set_vlti(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for MacVlanIncl {
    #[inline(always)]
    fn default() -> MacVlanIncl {
        MacVlanIncl(0)
    }
}
impl core::fmt::Debug for MacVlanIncl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacVlanIncl")
            .field("vlt", &self.vlt())
            .field("vlc", &self.vlc())
            .field("vlp", &self.vlp())
            .field("csvl", &self.csvl())
            .field("vlti", &self.vlti())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacVlanIncl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacVlanIncl {{ vlt: {=u16:?}, vlc: {:?}, vlp: {=bool:?}, csvl: {:?}, vlti: {=bool:?} }}",
            self.vlt(),
            self.vlc(),
            self.vlp(),
            self.csvl(),
            self.vlti()
        )
    }
}
#[doc = "VLAN Tag"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacVlanTag(pub u32);
impl MacVlanTag {
    #[doc = "VLAN Tag Identifier for Receive Packets"]
    #[must_use]
    #[inline(always)]
    pub const fn vl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "VLAN Tag Identifier for Receive Packets"]
    #[inline(always)]
    pub const fn set_vl(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Enable 12-Bit VLAN Tag Comparison"]
    #[must_use]
    #[inline(always)]
    pub const fn etv(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable 12-Bit VLAN Tag Comparison"]
    #[inline(always)]
    pub const fn set_etv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "VLAN Tag Inverse Match Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vtim(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "VLAN Tag Inverse Match Enable"]
    #[inline(always)]
    pub const fn set_vtim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enable S-VLAN"]
    #[must_use]
    #[inline(always)]
    pub const fn esvl(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enable S-VLAN"]
    #[inline(always)]
    pub const fn set_esvl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enable Receive S-VLAN Match"]
    #[must_use]
    #[inline(always)]
    pub const fn ersvlm(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Receive S-VLAN Match"]
    #[inline(always)]
    pub const fn set_ersvlm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Disable VLAN Type Check"]
    #[must_use]
    #[inline(always)]
    pub const fn dovltc(&self) -> super::vals::Dovltc {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Dovltc::from_bits(val as u8)
    }
    #[doc = "Disable VLAN Type Check"]
    #[inline(always)]
    pub const fn set_dovltc(&mut self, val: super::vals::Dovltc) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable VLAN Tag Stripping on Receive"]
    #[must_use]
    #[inline(always)]
    pub const fn evls(&self) -> super::vals::Evls {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::Evls::from_bits(val as u8)
    }
    #[doc = "Enable VLAN Tag Stripping on Receive"]
    #[inline(always)]
    pub const fn set_evls(&mut self, val: super::vals::Evls) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "Enable VLAN Tag in Rx status"]
    #[must_use]
    #[inline(always)]
    pub const fn evlrxs(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enable VLAN Tag in Rx status"]
    #[inline(always)]
    pub const fn set_evlrxs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Enable Double VLAN Processing"]
    #[must_use]
    #[inline(always)]
    pub const fn edvlp(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Double VLAN Processing"]
    #[inline(always)]
    pub const fn set_edvlp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Enable Inner VLAN Tag"]
    #[must_use]
    #[inline(always)]
    pub const fn erivlt(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Inner VLAN Tag"]
    #[inline(always)]
    pub const fn set_erivlt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Enable Inner VLAN Tag Stripping on Receive"]
    #[must_use]
    #[inline(always)]
    pub const fn eivls(&self) -> super::vals::Eivls {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Eivls::from_bits(val as u8)
    }
    #[doc = "Enable Inner VLAN Tag Stripping on Receive"]
    #[inline(always)]
    pub const fn set_eivls(&mut self, val: super::vals::Eivls) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Enable Inner VLAN Tag in Rx Status"]
    #[must_use]
    #[inline(always)]
    pub const fn eivlrxs(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Inner VLAN Tag in Rx Status"]
    #[inline(always)]
    pub const fn set_eivlrxs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for MacVlanTag {
    #[inline(always)]
    fn default() -> MacVlanTag {
        MacVlanTag(0)
    }
}
impl core::fmt::Debug for MacVlanTag {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacVlanTag")
            .field("vl", &self.vl())
            .field("etv", &self.etv())
            .field("vtim", &self.vtim())
            .field("esvl", &self.esvl())
            .field("ersvlm", &self.ersvlm())
            .field("dovltc", &self.dovltc())
            .field("evls", &self.evls())
            .field("evlrxs", &self.evlrxs())
            .field("edvlp", &self.edvlp())
            .field("erivlt", &self.erivlt())
            .field("eivls", &self.eivls())
            .field("eivlrxs", &self.eivlrxs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacVlanTag {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacVlanTag {{ vl: {=u16:?}, etv: {=bool:?}, vtim: {=bool:?}, esvl: {=bool:?}, ersvlm: {=bool:?}, dovltc: {:?}, evls: {:?}, evlrxs: {=bool:?}, edvlp: {=bool:?}, erivlt: {=bool:?}, eivls: {:?}, eivlrxs: {=bool:?} }}",
            self.vl(),
            self.etv(),
            self.vtim(),
            self.esvl(),
            self.ersvlm(),
            self.dovltc(),
            self.evls(),
            self.evlrxs(),
            self.edvlp(),
            self.erivlt(),
            self.eivls(),
            self.eivlrxs()
        )
    }
}
#[doc = "Watchdog and Jabber Timeout"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacWdJbTimeout(pub u32);
impl MacWdJbTimeout {
    #[doc = "Watchdog Timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn wto(&self) -> super::vals::Wto {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Wto::from_bits(val as u8)
    }
    #[doc = "Watchdog Timeout"]
    #[inline(always)]
    pub const fn set_wto(&mut self, val: super::vals::Wto) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Programmable Watchdog Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pwe(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Programmable Watchdog Enable"]
    #[inline(always)]
    pub const fn set_pwe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Jabber Timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn jto(&self) -> super::vals::Jto {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Jto::from_bits(val as u8)
    }
    #[doc = "Jabber Timeout"]
    #[inline(always)]
    pub const fn set_jto(&mut self, val: super::vals::Jto) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Programmable Jabber Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pje(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Programmable Jabber Enable"]
    #[inline(always)]
    pub const fn set_pje(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for MacWdJbTimeout {
    #[inline(always)]
    fn default() -> MacWdJbTimeout {
        MacWdJbTimeout(0)
    }
}
impl core::fmt::Debug for MacWdJbTimeout {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacWdJbTimeout")
            .field("wto", &self.wto())
            .field("pwe", &self.pwe())
            .field("jto", &self.jto())
            .field("pje", &self.pje())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacWdJbTimeout {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacWdJbTimeout {{ wto: {:?}, pwe: {=bool:?}, jto: {:?}, pje: {=bool:?} }}",
            self.wto(),
            self.pwe(),
            self.jto(),
            self.pje()
        )
    }
}
