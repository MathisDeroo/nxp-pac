#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Actphysel {
    #[doc = "GMII or MII"]
    GMII_MII = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "B10T1S"]
    B10T1S = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Actphysel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Actphysel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Actphysel {
    #[inline(always)]
    fn from(val: u8) -> Actphysel {
        Actphysel::from_bits(val)
    }
}
impl From<Actphysel> for u8 {
    #[inline(always)]
    fn from(val: Actphysel) -> u8 {
        Actphysel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Addr64 {
    #[doc = "32"]
    M_32 = 0x0,
    #[doc = "40"]
    M_40 = 0x01,
    #[doc = "48"]
    M_48 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Addr64 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Addr64 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Addr64 {
    #[inline(always)]
    fn from(val: u8) -> Addr64 {
        Addr64::from_bits(val)
    }
}
impl From<Addr64> for u8 {
    #[inline(always)]
    fn from(val: Addr64) -> u8 {
        Addr64::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Addsub {
    #[doc = "Add time"]
    ADD = 0x0,
    #[doc = "Subtract time"]
    SUB = 0x01,
}
impl Addsub {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Addsub {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Addsub {
    #[inline(always)]
    fn from(val: u8) -> Addsub {
        Addsub::from_bits(val)
    }
}
impl From<Addsub> for u8 {
    #[inline(always)]
    fn from(val: Addsub) -> u8 {
        Addsub::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Asp {
    #[doc = "No Safety features selected"]
    NONE = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Asp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Asp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Asp {
    #[inline(always)]
    fn from(val: u8) -> Asp {
        Asp::from_bits(val)
    }
}
impl From<Asp> for u8 {
    #[inline(always)]
    fn from(val: Asp) -> u8 {
        Asp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Auxsnapnum {
    #[doc = "No auxiliary input"]
    NO_AUXI = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "2 auxiliary input"]
    M_2_AUXI = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Auxsnapnum {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Auxsnapnum {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Auxsnapnum {
    #[inline(always)]
    fn from(val: u8) -> Auxsnapnum {
        Auxsnapnum::from_bits(val)
    }
}
impl From<Auxsnapnum> for u8 {
    #[inline(always)]
    fn from(val: Auxsnapnum) -> u8 {
        Auxsnapnum::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bl {
    #[doc = "k = min(n,10)"]
    MIN_N_10 = 0x0,
    #[doc = "k = min(n,8)"]
    MIN_N_8 = 0x01,
    #[doc = "k = min(n,4)"]
    MIN_N_4 = 0x02,
    #[doc = "k = min(n,1)"]
    MIN_N_1 = 0x03,
}
impl Bl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bl {
    #[inline(always)]
    fn from(val: u8) -> Bl {
        Bl::from_bits(val)
    }
}
impl From<Bl> for u8 {
    #[inline(always)]
    fn from(val: Bl) -> u8 {
        Bl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbf {
    #[doc = "Enable Broadcast Packets"]
    ENABLE = 0x0,
    #[doc = "Disable Broadcast Packets"]
    DISABLE = 0x01,
}
impl Dbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbf {
    #[inline(always)]
    fn from(val: u8) -> Dbf {
        Dbf::from_bits(val)
    }
}
impl From<Dbf> for u8 {
    #[inline(always)]
    fn from(val: Dbf) -> u8 {
        Dbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dcrcc {
    #[doc = "CRC Checking is enabled"]
    ENABLE = 0x0,
    #[doc = "CRC Checking is disabled"]
    DISABLE = 0x01,
}
impl Dcrcc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dcrcc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dcrcc {
    #[inline(always)]
    fn from(val: u8) -> Dcrcc {
        Dcrcc::from_bits(val)
    }
}
impl From<Dcrcc> for u8 {
    #[inline(always)]
    fn from(val: Dcrcc) -> u8 {
        Dcrcc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dcrs {
    #[doc = "Enable Carrier Sense During Transmission"]
    ENABLE = 0x0,
    #[doc = "Disable Carrier Sense During Transmission"]
    DISABLE = 0x01,
}
impl Dcrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dcrs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dcrs {
    #[inline(always)]
    fn from(val: u8) -> Dcrs {
        Dcrs::from_bits(val)
    }
}
impl From<Dcrs> for u8 {
    #[inline(always)]
    fn from(val: Dcrs) -> u8 {
        Dcrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dm {
    #[doc = "Half-duplex mode"]
    HDUPLX = 0x0,
    #[doc = "Full-duplex mode"]
    FDUPLX = 0x01,
}
impl Dm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dm {
    #[inline(always)]
    fn from(val: u8) -> Dm {
        Dm::from_bits(val)
    }
}
impl From<Dm> for u8 {
    #[inline(always)]
    fn from(val: Dm) -> u8 {
        Dm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Do {
    #[doc = "Enable Receive Own"]
    ENABLE = 0x0,
    #[doc = "Disable Receive Own"]
    DISABLE = 0x01,
}
impl Do {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Do {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Do {
    #[inline(always)]
    fn from(val: u8) -> Do {
        Do::from_bits(val)
    }
}
impl From<Do> for u8 {
    #[inline(always)]
    fn from(val: Do) -> u8 {
        Do::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dovltc {
    #[doc = "VLAN Type Check is enabled"]
    ENABLE = 0x0,
    #[doc = "VLAN Type Check is disabled"]
    DISABLE = 0x01,
}
impl Dovltc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dovltc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dovltc {
    #[inline(always)]
    fn from(val: u8) -> Dovltc {
        Dovltc::from_bits(val)
    }
}
impl From<Dovltc> for u8 {
    #[inline(always)]
    fn from(val: Dovltc) -> u8 {
        Dovltc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dr {
    #[doc = "Enable Retry"]
    ENABLE = 0x0,
    #[doc = "Disable Retry"]
    DISABLE = 0x01,
}
impl Dr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dr {
    #[inline(always)]
    fn from(val: u8) -> Dr {
        Dr::from_bits(val)
    }
}
impl From<Dr> for u8 {
    #[inline(always)]
    fn from(val: Dr) -> u8 {
        Dr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dzpq {
    #[doc = "Zero-Quanta Pause packet generation is enabled"]
    ENABLE = 0x0,
    #[doc = "Zero-Quanta Pause packet generation is disabled"]
    DISABLE = 0x01,
}
impl Dzpq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dzpq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dzpq {
    #[inline(always)]
    fn from(val: u8) -> Dzpq {
        Dzpq::from_bits(val)
    }
}
impl From<Dzpq> for u8 {
    #[inline(always)]
    fn from(val: Dzpq) -> u8 {
        Dzpq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eivls {
    #[doc = "Do not strip"]
    DONOT = 0x0,
    #[doc = "Strip if VLAN filter passes"]
    IFPASS = 0x01,
    #[doc = "Strip if VLAN filter fails"]
    IFFAIL = 0x02,
    #[doc = "Always strip"]
    ALWAYS = 0x03,
}
impl Eivls {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eivls {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eivls {
    #[inline(always)]
    fn from(val: u8) -> Eivls {
        Eivls::from_bits(val)
    }
}
impl From<Eivls> for u8 {
    #[inline(always)]
    fn from(val: Eivls) -> u8 {
        Eivls::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Estdep {
    #[doc = "No Depth configured"]
    NODEPTH = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "256"]
    DEPTH256 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Estdep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Estdep {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Estdep {
    #[inline(always)]
    fn from(val: u8) -> Estdep {
        Estdep::from_bits(val)
    }
}
impl From<Estdep> for u8 {
    #[inline(always)]
    fn from(val: Estdep) -> u8 {
        Estdep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Estwid {
    #[doc = "Width not configured"]
    NOWIDTH = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "24"]
    WIDTH24 = 0x03,
}
impl Estwid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Estwid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Estwid {
    #[inline(always)]
    fn from(val: u8) -> Estwid {
        Estwid::from_bits(val)
    }
}
impl From<Estwid> for u8 {
    #[inline(always)]
    fn from(val: Estwid) -> u8 {
        Estwid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Evls {
    #[doc = "Do not strip"]
    DONOT = 0x0,
    #[doc = "Strip if VLAN filter passes"]
    IFPASS = 0x01,
    #[doc = "Strip if VLAN filter fails"]
    IFFAIL = 0x02,
    #[doc = "Always strip"]
    ALWAYS = 0x03,
}
impl Evls {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Evls {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Evls {
    #[inline(always)]
    fn from(val: u8) -> Evls {
        Evls::from_bits(val)
    }
}
impl From<Evls> for u8 {
    #[inline(always)]
    fn from(val: Evls) -> u8 {
        Evls::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fes {
    #[doc = "10 Mbps when PS bit is 1 and 1 Gbps when PS bit is 0"]
    M_10_1000M = 0x0,
    #[doc = "100 Mbps when PS bit is 1 and 2.5 Gbps when PS bit is 0"]
    M_100_2500M = 0x01,
}
impl Fes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fes {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fes {
    #[inline(always)]
    fn from(val: u8) -> Fes {
        Fes::from_bits(val)
    }
}
impl From<Fes> for u8 {
    #[inline(always)]
    fn from(val: Fes) -> u8 {
        Fes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Frpbs {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "256 Bytes"]
    M_256BYTES = 0x02,
    _RESERVED_3 = 0x03,
}
impl Frpbs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Frpbs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Frpbs {
    #[inline(always)]
    fn from(val: u8) -> Frpbs {
        Frpbs::from_bits(val)
    }
}
impl From<Frpbs> for u8 {
    #[inline(always)]
    fn from(val: Frpbs) -> u8 {
        Frpbs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Frpes {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "256 Entries"]
    M_256ENTR = 0x02,
    _RESERVED_3 = 0x03,
}
impl Frpes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Frpes {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Frpes {
    #[inline(always)]
    fn from(val: u8) -> Frpes {
        Frpes::from_bits(val)
    }
}
impl From<Frpes> for u8 {
    #[inline(always)]
    fn from(val: Frpes) -> u8 {
        Frpes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hashtblsz {
    #[doc = "No hash table"]
    NO_HT = 0x0,
    #[doc = "64"]
    M_64 = 0x01,
    #[doc = "128"]
    M_128 = 0x02,
    #[doc = "256"]
    M_256 = 0x03,
}
impl Hashtblsz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hashtblsz {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hashtblsz {
    #[inline(always)]
    fn from(val: u8) -> Hashtblsz {
        Hashtblsz::from_bits(val)
    }
}
impl From<Hashtblsz> for u8 {
    #[inline(always)]
    fn from(val: Hashtblsz) -> u8 {
        Hashtblsz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipg {
    #[doc = "96 bit times IPG"]
    IPG96 = 0x0,
    #[doc = "88 bit times IPG"]
    IPG88 = 0x01,
    #[doc = "80 bit times IPG"]
    IPG80 = 0x02,
    #[doc = "72 bit times IPG"]
    IPG72 = 0x03,
    #[doc = "64 bit times IPG"]
    IPG64 = 0x04,
    #[doc = "56 bit times IPG"]
    IPG56 = 0x05,
    #[doc = "48 bit times IPG"]
    IPG48 = 0x06,
    #[doc = "40 bit times IPG"]
    IPG40 = 0x07,
}
impl Ipg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipg {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipg {
    #[inline(always)]
    fn from(val: u8) -> Ipg {
        Ipg::from_bits(val)
    }
}
impl From<Ipg> for u8 {
    #[inline(always)]
    fn from(val: Ipg) -> u8 {
        Ipg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Jd {
    #[doc = "Jabber is enabled"]
    ENABLE = 0x0,
    #[doc = "Jabber is disabled"]
    DISABLE = 0x01,
}
impl Jd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Jd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Jd {
    #[inline(always)]
    fn from(val: u8) -> Jd {
        Jd::from_bits(val)
    }
}
impl From<Jd> for u8 {
    #[inline(always)]
    fn from(val: Jd) -> u8 {
        Jd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Jto {
    #[doc = "2 KB"]
    KB2 = 0x0,
    #[doc = "3 KB"]
    KB3 = 0x01,
    #[doc = "4 KB"]
    KB4 = 0x02,
    #[doc = "5 KB"]
    KB5 = 0x03,
    #[doc = "6 KB"]
    KB6 = 0x04,
    #[doc = "7 KB"]
    KB7 = 0x05,
    #[doc = "8 KB"]
    KB8 = 0x06,
    #[doc = "9 KB"]
    KB9 = 0x07,
    #[doc = "10 KB"]
    KB10 = 0x08,
    #[doc = "11 KB"]
    KB11 = 0x09,
    #[doc = "12 KB"]
    KB12 = 0x0a,
    #[doc = "13 KB"]
    KB13 = 0x0b,
    #[doc = "14 KB"]
    KB14 = 0x0c,
    #[doc = "15 KB"]
    KB15 = 0x0d,
    #[doc = "16383 Bytes"]
    B16383 = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Jto {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Jto {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Jto {
    #[inline(always)]
    fn from(val: u8) -> Jto {
        Jto::from_bits(val)
    }
}
impl From<Jto> for u8 {
    #[inline(always)]
    fn from(val: Jto) -> u8 {
        Jto::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum L3l4fnum {
    #[doc = "No L3 or L4 Filter"]
    NOFILT = 0x0,
    #[doc = "1 L3 or L4 Filter"]
    M_1FILT = 0x01,
    #[doc = "2 L3 or L4 Filters"]
    M_2FILT = 0x02,
    #[doc = "3 L3 or L4 Filters"]
    M_3FILT = 0x03,
    #[doc = "4 L3 or L4 Filters"]
    M_4FILT = 0x04,
    #[doc = "5 L3 or L4 Filters"]
    M_5FILT = 0x05,
    #[doc = "6 L3 or L4 Filters"]
    M_6FILT = 0x06,
    #[doc = "7 L3 or L4 Filters"]
    M_7FILT = 0x07,
    #[doc = "8 L3 or L4 Filters"]
    M_8FILT = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl L3l4fnum {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> L3l4fnum {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for L3l4fnum {
    #[inline(always)]
    fn from(val: u8) -> L3l4fnum {
        L3l4fnum::from_bits(val)
    }
}
impl From<L3l4fnum> for u8 {
    #[inline(always)]
    fn from(val: L3l4fnum) -> u8 {
        L3l4fnum::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MacInnerVlanInclCsvl {
    #[doc = "C-VLAN type (0x8100) is inserted"]
    _CVLAN = 0x0,
    #[doc = "S-VLAN type (0x88A8) is inserted"]
    _SVLAN = 0x01,
}
impl MacInnerVlanInclCsvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MacInnerVlanInclCsvl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MacInnerVlanInclCsvl {
    #[inline(always)]
    fn from(val: u8) -> MacInnerVlanInclCsvl {
        MacInnerVlanInclCsvl::from_bits(val)
    }
}
impl From<MacInnerVlanInclCsvl> for u8 {
    #[inline(always)]
    fn from(val: MacInnerVlanInclCsvl) -> u8 {
        MacInnerVlanInclCsvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MacInnerVlanInclVlc {
    #[doc = "No VLAN tag deletion, insertion, or replacement"]
    NO_TAG_DEL_INS_REP = 0x0,
    #[doc = "VLAN tag deletion"]
    TAG_DEL = 0x01,
    #[doc = "VLAN tag insertion"]
    TAG_INS = 0x02,
    #[doc = "VLAN tag replacement"]
    TAG_REP = 0x03,
}
impl MacInnerVlanInclVlc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MacInnerVlanInclVlc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MacInnerVlanInclVlc {
    #[inline(always)]
    fn from(val: u8) -> MacInnerVlanInclVlc {
        MacInnerVlanInclVlc::from_bits(val)
    }
}
impl From<MacInnerVlanInclVlc> for u8 {
    #[inline(always)]
    fn from(val: MacInnerVlanInclVlc) -> u8 {
        MacInnerVlanInclVlc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MacVlanInclCsvl {
    #[doc = "C-VLAN type (0x8100) is inserted or replaced"]
    _CVLAN = 0x0,
    #[doc = "S-VLAN type (0x88A8) is inserted or replaced"]
    _SVLAN = 0x01,
}
impl MacVlanInclCsvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MacVlanInclCsvl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MacVlanInclCsvl {
    #[inline(always)]
    fn from(val: u8) -> MacVlanInclCsvl {
        MacVlanInclCsvl::from_bits(val)
    }
}
impl From<MacVlanInclCsvl> for u8 {
    #[inline(always)]
    fn from(val: MacVlanInclCsvl) -> u8 {
        MacVlanInclCsvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MacVlanInclVlc {
    #[doc = "No VLAN tag deletion, insertion, or replacement"]
    NO_TAG_DEL_INS_REP = 0x0,
    #[doc = "VLAN tag deletion"]
    TAG_DEL = 0x01,
    #[doc = "VLAN tag insertion"]
    TAG_INS = 0x02,
    #[doc = "VLAN tag replacement"]
    TAG_REP = 0x03,
}
impl MacVlanInclVlc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MacVlanInclVlc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MacVlanInclVlc {
    #[inline(always)]
    fn from(val: u8) -> MacVlanInclVlc {
        MacVlanInclVlc::from_bits(val)
    }
}
impl From<MacVlanInclVlc> for u8 {
    #[inline(always)]
    fn from(val: MacVlanInclVlc) -> u8 {
        MacVlanInclVlc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nrvf {
    #[doc = "No Extended Rx VLAN Filters"]
    NO_ERVLAN = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Nrvf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nrvf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nrvf {
    #[inline(always)]
    fn from(val: u8) -> Nrvf {
        Nrvf::from_bits(val)
    }
}
impl From<Nrvf> for u8 {
    #[inline(always)]
    fn from(val: Nrvf) -> u8 {
        Nrvf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcf {
    #[doc = "MAC filters all control packets from reaching the application"]
    FLTR_ALL = 0x0,
    #[doc = "MAC forwards all control packets except Pause packets to the application even if they fail the Address filter"]
    FW_XCPT_PAU = 0x01,
    #[doc = "MAC forwards all control packets to the application even if they fail the Address filter"]
    FW_ALL = 0x02,
    #[doc = "MAC forwards the control packets that pass the Address filter"]
    FW_PASS = 0x03,
}
impl Pcf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcf {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcf {
    #[inline(always)]
    fn from(val: u8) -> Pcf {
        Pcf::from_bits(val)
    }
}
impl From<Pcf> for u8 {
    #[inline(always)]
    fn from(val: Pcf) -> u8 {
        Pcf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcsel {
    #[doc = "Policing counter feature is not selected"]
    INACTIVE = 0x0,
    #[doc = "16 Policing counters are selected"]
    PCNUM_16 = 0x01,
    #[doc = "32 Policing counters are selected"]
    PCNUM_32 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Pcsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcsel {
    #[inline(always)]
    fn from(val: u8) -> Pcsel {
        Pcsel::from_bits(val)
    }
}
impl From<Pcsel> for u8 {
    #[inline(always)]
    fn from(val: Pcsel) -> u8 {
        Pcsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Plt {
    #[doc = "Pause Time minus 4 Slot Times (PT -4 slot times)"]
    PT4 = 0x0,
    #[doc = "Pause Time minus 28 Slot Times (PT -28 slot times)"]
    PT28 = 0x01,
    #[doc = "Pause Time minus 36 Slot Times (PT -36 slot times)"]
    PT36 = 0x02,
    #[doc = "Pause Time minus 144 Slot Times (PT -144 slot times)"]
    PT144 = 0x03,
    #[doc = "Pause Time minus 256 Slot Times (PT -256 slot times)"]
    PT256 = 0x04,
    #[doc = "Pause Time minus 512 Slot Times (PT -512 slot times)"]
    PT512 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Plt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Plt {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Plt {
    #[inline(always)]
    fn from(val: u8) -> Plt {
        Plt::from_bits(val)
    }
}
impl From<Plt> for u8 {
    #[inline(always)]
    fn from(val: Plt) -> u8 {
        Plt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ppsoutnum {
    #[doc = "No PPS output"]
    NO_PPSO = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ppsoutnum {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ppsoutnum {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ppsoutnum {
    #[inline(always)]
    fn from(val: u8) -> Ppsoutnum {
        Ppsoutnum::from_bits(val)
    }
}
impl From<Ppsoutnum> for u8 {
    #[inline(always)]
    fn from(val: Ppsoutnum) -> u8 {
        Ppsoutnum::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prelen {
    #[doc = "7 bytes of preamble"]
    M_7BYTES = 0x0,
    #[doc = "5 bytes of preamble"]
    M_5BYTES = 0x01,
    #[doc = "3 bytes of preamble"]
    M_3BYTES = 0x02,
    _RESERVED_3 = 0x03,
}
impl Prelen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prelen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prelen {
    #[inline(always)]
    fn from(val: u8) -> Prelen {
        Prelen::from_bits(val)
    }
}
impl From<Prelen> for u8 {
    #[inline(always)]
    fn from(val: Prelen) -> u8 {
        Prelen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ps {
    #[doc = "For 1000 or 2500 Mbps operations"]
    M_1000_2500M = 0x0,
    #[doc = "For 10 or 100 Mbps operations"]
    M_10_100M = 0x01,
}
impl Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ps {
    #[inline(always)]
    fn from(val: u8) -> Ps {
        Ps::from_bits(val)
    }
}
impl From<Ps> for u8 {
    #[inline(always)]
    fn from(val: Ps) -> u8 {
        Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdcsz {
    #[doc = "Desc Cache not configured"]
    NO_DCACHE = 0x0,
    #[doc = "4"]
    M_1RDCSZ = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Rdcsz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdcsz {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdcsz {
    #[inline(always)]
    fn from(val: u8) -> Rdcsz {
        Rdcsz::from_bits(val)
    }
}
impl From<Rdcsz> for u8 {
    #[inline(always)]
    fn from(val: Rdcsz) -> u8 {
        Rdcsz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxchcnt {
    #[doc = "1 DMA Rx Channel"]
    D_1RXCH = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Rxchcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxchcnt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxchcnt {
    #[inline(always)]
    fn from(val: u8) -> Rxchcnt {
        Rxchcnt::from_bits(val)
    }
}
impl From<Rxchcnt> for u8 {
    #[inline(always)]
    fn from(val: Rxchcnt) -> u8 {
        Rxchcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxfifosize {
    #[doc = "128 bytes"]
    M_128B = 0x0,
    #[doc = "256 bytes"]
    M_256B = 0x01,
    #[doc = "512 bytes"]
    M_512B = 0x02,
    #[doc = "1024 bytes"]
    M_1024B = 0x03,
    #[doc = "2048 bytes"]
    M_2048B = 0x04,
    #[doc = "4096 bytes"]
    M_4096B = 0x05,
    #[doc = "8192 bytes"]
    M_8192B = 0x06,
    #[doc = "16384 bytes"]
    M_16384B = 0x07,
    #[doc = "32 KB"]
    M_32KB = 0x08,
    #[doc = "64 KB"]
    M_64KB = 0x09,
    #[doc = "128 KB"]
    M_128KB = 0x0a,
    #[doc = "256 KB"]
    M_256KB = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Rxfifosize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxfifosize {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxfifosize {
    #[inline(always)]
    fn from(val: u8) -> Rxfifosize {
        Rxfifosize::from_bits(val)
    }
}
impl From<Rxfifosize> for u8 {
    #[inline(always)]
    fn from(val: Rxfifosize) -> u8 {
        Rxfifosize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxqcnt {
    #[doc = "1 MTL Rx Queue"]
    M_1RXQ = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Rxqcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxqcnt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxqcnt {
    #[inline(always)]
    fn from(val: u8) -> Rxqcnt {
        Rxqcnt::from_bits(val)
    }
}
impl From<Rxqcnt> for u8 {
    #[inline(always)]
    fn from(val: Rxqcnt) -> u8 {
        Rxqcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sarc {
    #[doc = "mti_sa_ctrl_i and ati_sa_ctrl_i input signals control the SA field generation"]
    CTRL_SA_GEN = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Contents of MAC Addr-0 inserted in SA field"]
    ADD0_INSRT_IN_SA = 0x02,
    #[doc = "Contents of MAC Addr-0 replaces SA field"]
    ADD0_REPL_SA = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Contents of MAC Addr-1 inserted in SA field"]
    ADD1_INSRT_IN_SA = 0x06,
    #[doc = "Contents of MAC Addr-1 replaces SA field"]
    ADD1_REPL_SA = 0x07,
}
impl Sarc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sarc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sarc {
    #[inline(always)]
    fn from(val: u8) -> Sarc {
        Sarc::from_bits(val)
    }
}
impl From<Sarc> for u8 {
    #[inline(always)]
    fn from(val: Sarc) -> u8 {
        Sarc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tdcsz {
    #[doc = "Desc Cache not configured"]
    NO_DCACHE = 0x0,
    #[doc = "4"]
    M_1TDCSZ = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Tdcsz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tdcsz {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tdcsz {
    #[inline(always)]
    fn from(val: u8) -> Tdcsz {
        Tdcsz::from_bits(val)
    }
}
impl From<Tdcsz> for u8 {
    #[inline(always)]
    fn from(val: Tdcsz) -> u8 {
        Tdcsz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tfcsts {
    #[doc = "Idle state"]
    IDLE = 0x0,
    #[doc = "Waiting for one of the following: Status of the previous packet OR IPG or back off period to be over"]
    WAITING = 0x01,
    #[doc = "Generating and transmitting a Pause control packet (in full-duplex mode)"]
    GEN_TX_PAU = 0x02,
    #[doc = "Transferring input packet for transmission"]
    TRNSFR = 0x03,
}
impl Tfcsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tfcsts {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tfcsts {
    #[inline(always)]
    fn from(val: u8) -> Tfcsts {
        Tfcsts::from_bits(val)
    }
}
impl From<Tfcsts> for u8 {
    #[inline(always)]
    fn from(val: Tfcsts) -> u8 {
        Tfcsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tscfupdt {
    #[doc = "Coarse method is used to update system timestamp"]
    COARSE = 0x0,
    #[doc = "Fine method is used to update system timestamp"]
    FINE = 0x01,
}
impl Tscfupdt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tscfupdt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tscfupdt {
    #[inline(always)]
    fn from(val: u8) -> Tscfupdt {
        Tscfupdt::from_bits(val)
    }
}
impl From<Tscfupdt> for u8 {
    #[inline(always)]
    fn from(val: Tscfupdt) -> u8 {
        Tscfupdt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Tsss(u32);
impl Tsss {
    #[doc = "Power down is disabled"]
    pub const DISABLE: Self = Self(0x0);
    #[doc = "Power down is enabled"]
    pub const ENABLE: Self = Self(0x01);
}
impl Tsss {
    pub const fn from_bits(val: u32) -> Tsss {
        Self(val & 0x7fff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Tsss {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("DISABLE"),
            0x01 => f.write_str("ENABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsss {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "DISABLE"),
            0x01 => defmt::write!(f, "ENABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Tsss {
    #[inline(always)]
    fn from(val: u32) -> Tsss {
        Tsss::from_bits(val)
    }
}
impl From<Tsss> for u32 {
    #[inline(always)]
    fn from(val: Tsss) -> u32 {
        Tsss::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsstssel {
    _RESERVED_0 = 0x0,
    #[doc = "Internal"]
    INTRNL = 0x01,
    #[doc = "External"]
    EXTRNL = 0x02,
    #[doc = "Both"]
    BOTH = 0x03,
}
impl Tsstssel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsstssel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsstssel {
    #[inline(always)]
    fn from(val: u8) -> Tsstssel {
        Tsstssel::from_bits(val)
    }
}
impl From<Tsstssel> for u8 {
    #[inline(always)]
    fn from(val: Tsstssel) -> u8 {
        Tsstssel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txchcnt {
    #[doc = "1 DMA Tx Channel"]
    D_1TXCH = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Txchcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txchcnt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txchcnt {
    #[inline(always)]
    fn from(val: u8) -> Txchcnt {
        Txchcnt::from_bits(val)
    }
}
impl From<Txchcnt> for u8 {
    #[inline(always)]
    fn from(val: Txchcnt) -> u8 {
        Txchcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txfifosize {
    #[doc = "128 bytes"]
    M_128B = 0x0,
    #[doc = "256 bytes"]
    M_256B = 0x01,
    #[doc = "512 bytes"]
    M_512B = 0x02,
    #[doc = "1024 bytes"]
    M_1024B = 0x03,
    #[doc = "2048 bytes"]
    M_2048B = 0x04,
    #[doc = "4096 bytes"]
    M_4096B = 0x05,
    #[doc = "8192 bytes"]
    M_8192B = 0x06,
    #[doc = "16384 bytes"]
    M_16384B = 0x07,
    #[doc = "32 KB"]
    M_32KB = 0x08,
    #[doc = "64 KB"]
    M_64KB = 0x09,
    #[doc = "128 KB"]
    M_128KB = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Txfifosize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txfifosize {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txfifosize {
    #[inline(always)]
    fn from(val: u8) -> Txfifosize {
        Txfifosize::from_bits(val)
    }
}
impl From<Txfifosize> for u8 {
    #[inline(always)]
    fn from(val: Txfifosize) -> u8 {
        Txfifosize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txqcnt {
    #[doc = "1 MTL Tx Queue"]
    M_1TXQ = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Txqcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txqcnt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txqcnt {
    #[inline(always)]
    fn from(val: u8) -> Txqcnt {
        Txqcnt::from_bits(val)
    }
}
impl From<Txqcnt> for u8 {
    #[inline(always)]
    fn from(val: Txqcnt) -> u8 {
        Txqcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wd {
    #[doc = "Watchdog is enabled"]
    ENABLE = 0x0,
    #[doc = "Watchdog is disabled"]
    DISABLE = 0x01,
}
impl Wd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wd {
    #[inline(always)]
    fn from(val: u8) -> Wd {
        Wd::from_bits(val)
    }
}
impl From<Wd> for u8 {
    #[inline(always)]
    fn from(val: Wd) -> u8 {
        Wd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wto {
    #[doc = "2 KB"]
    KB2 = 0x0,
    #[doc = "3 KB"]
    KB3 = 0x01,
    #[doc = "4 KB"]
    KB4 = 0x02,
    #[doc = "5 KB"]
    KB5 = 0x03,
    #[doc = "6 KB"]
    KB6 = 0x04,
    #[doc = "7 KB"]
    KB7 = 0x05,
    #[doc = "8 KB"]
    KB8 = 0x06,
    #[doc = "9 KB"]
    KB9 = 0x07,
    #[doc = "10 KB"]
    KB10 = 0x08,
    #[doc = "11 KB"]
    KB11 = 0x09,
    #[doc = "12 KB"]
    KB12 = 0x0a,
    #[doc = "13 KB"]
    KB13 = 0x0b,
    #[doc = "14 KB"]
    KB14 = 0x0c,
    #[doc = "15 KB"]
    KB15 = 0x0d,
    #[doc = "16383 Bytes"]
    B16383 = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Wto {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wto {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wto {
    #[inline(always)]
    fn from(val: u8) -> Wto {
        Wto::from_bits(val)
    }
}
impl From<Wto> for u8 {
    #[inline(always)]
    fn from(val: Wto) -> u8 {
        Wto::to_bits(val)
    }
}
