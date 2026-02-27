#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Abortcmd {
    #[doc = "NO_EFFECT"]
    REM_N = 0x0,
    #[doc = "Abort WUP-sequencer"]
    REM_TRIG = 0x01,
}
impl Abortcmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Abortcmd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Abortcmd {
    #[inline(always)]
    fn from(val: u8) -> Abortcmd {
        Abortcmd::from_bits(val)
    }
}
impl From<Abortcmd> for u8 {
    #[inline(always)]
    fn from(val: Abortcmd) -> u8 {
        Abortcmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Access {
    #[doc = "Disabled"]
    DI = 0x0,
    #[doc = "Enabled"]
    EN = 0x01,
}
impl Access {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Access {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Access {
    #[inline(always)]
    fn from(val: u8) -> Access {
        Access::from_bits(val)
    }
}
impl From<Access> for u8 {
    #[inline(always)]
    fn from(val: Access) -> u8 {
        Access::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmd {
    #[doc = "None"]
    NO = 0x0,
    #[doc = "Linkdown"]
    PEN = 0x01,
    #[doc = "Linkup"]
    LI = 0x02,
    #[doc = "TXCCFG"]
    TX = 0x03,
    #[doc = "Low-power"]
    LO = 0x04,
    #[doc = "TXCBIST"]
    TXCBIST = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Cmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmd {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmd {
    #[inline(always)]
    fn from(val: u8) -> Cmd {
        Cmd::from_bits(val)
    }
}
impl From<Cmd> for u8 {
    #[inline(always)]
    fn from(val: Cmd) -> u8 {
        Cmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dispre {
    #[doc = "Disabled"]
    DISPRE_HOLD_1 = 0x0,
    #[doc = "Enabled"]
    DISPRE_HOLD_2 = 0x01,
}
impl Dispre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dispre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dispre {
    #[inline(always)]
    fn from(val: u8) -> Dispre {
        Dispre::from_bits(val)
    }
}
impl From<Dispre> for u8 {
    #[inline(always)]
    fn from(val: Dispre) -> u8 {
        Dispre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hold {
    #[doc = "HOLD_1"]
    HOLD_1 = 0x0,
    #[doc = "HOLD_2"]
    HOLD_2 = 0x01,
    #[doc = "HOLD_3"]
    HOLD_3 = 0x02,
    #[doc = "HOLD_4"]
    HOLD_4 = 0x03,
    #[doc = "HOLD_5"]
    HOLD_5 = 0x04,
    #[doc = "6 PCLK cycles 6"]
    HOLD_6 = 0x05,
    #[doc = "HOLD_7"]
    HOLD_7 = 0x06,
    #[doc = "HOLD_8"]
    HOLD_8 = 0x07,
}
impl Hold {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hold {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hold {
    #[inline(always)]
    fn from(val: u8) -> Hold {
        Hold::from_bits(val)
    }
}
impl From<Hold> for u8 {
    #[inline(always)]
    fn from(val: Hold) -> u8 {
        Hold::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt1Apbparity {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt1Apbparity {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt1Apbparity {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt1Apbparity {
    #[inline(always)]
    fn from(val: u8) -> Intencapt1Apbparity {
        Intencapt1Apbparity::from_bits(val)
    }
}
impl From<Intencapt1Apbparity> for u8 {
    #[inline(always)]
    fn from(val: Intencapt1Apbparity) -> u8 {
        Intencapt1Apbparity::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt1Invldapb {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt1Invldapb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt1Invldapb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt1Invldapb {
    #[inline(always)]
    fn from(val: u8) -> Intencapt1Invldapb {
        Intencapt1Invldapb::from_bits(val)
    }
}
impl From<Intencapt1Invldapb> for u8 {
    #[inline(always)]
    fn from(val: Intencapt1Invldapb) -> u8 {
        Intencapt1Invldapb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt1Lclwk {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt1Lclwk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt1Lclwk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt1Lclwk {
    #[inline(always)]
    fn from(val: u8) -> Intencapt1Lclwk {
        Intencapt1Lclwk::from_bits(val)
    }
}
impl From<Intencapt1Lclwk> for u8 {
    #[inline(always)]
    fn from(val: Intencapt1Lclwk) -> u8 {
        Intencapt1Lclwk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt1Locjab {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt1Locjab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt1Locjab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt1Locjab {
    #[inline(always)]
    fn from(val: u8) -> Intencapt1Locjab {
        Intencapt1Locjab::from_bits(val)
    }
}
impl From<Intencapt1Locjab> for u8 {
    #[inline(always)]
    fn from(val: Intencapt1Locjab) -> u8 {
        Intencapt1Locjab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt1Modestat {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt1Modestat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt1Modestat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt1Modestat {
    #[inline(always)]
    fn from(val: u8) -> Intencapt1Modestat {
        Intencapt1Modestat::from_bits(val)
    }
}
impl From<Intencapt1Modestat> for u8 {
    #[inline(always)]
    fn from(val: Intencapt1Modestat) -> u8 {
        Intencapt1Modestat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt1Physcol {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt1Physcol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt1Physcol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt1Physcol {
    #[inline(always)]
    fn from(val: u8) -> Intencapt1Physcol {
        Intencapt1Physcol::from_bits(val)
    }
}
impl From<Intencapt1Physcol> for u8 {
    #[inline(always)]
    fn from(val: Intencapt1Physcol) -> u8 {
        Intencapt1Physcol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt1Pinfault {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt1Pinfault {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt1Pinfault {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt1Pinfault {
    #[inline(always)]
    fn from(val: u8) -> Intencapt1Pinfault {
        Intencapt1Pinfault::from_bits(val)
    }
}
impl From<Intencapt1Pinfault> for u8 {
    #[inline(always)]
    fn from(val: Intencapt1Pinfault) -> u8 {
        Intencapt1Pinfault::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt1Plcadiag {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt1Plcadiag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt1Plcadiag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt1Plcadiag {
    #[inline(always)]
    fn from(val: u8) -> Intencapt1Plcadiag {
        Intencapt1Plcadiag::from_bits(val)
    }
}
impl From<Intencapt1Plcadiag> for u8 {
    #[inline(always)]
    fn from(val: Intencapt1Plcadiag) -> u8 {
        Intencapt1Plcadiag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt1Plcarec {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt1Plcarec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt1Plcarec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt1Plcarec {
    #[inline(always)]
    fn from(val: u8) -> Intencapt1Plcarec {
        Intencapt1Plcarec::from_bits(val)
    }
}
impl From<Intencapt1Plcarec> for u8 {
    #[inline(always)]
    fn from(val: Intencapt1Plcarec) -> u8 {
        Intencapt1Plcarec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt1Plcastat {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt1Plcastat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt1Plcastat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt1Plcastat {
    #[inline(always)]
    fn from(val: u8) -> Intencapt1Plcastat {
        Intencapt1Plcastat::from_bits(val)
    }
}
impl From<Intencapt1Plcastat> for u8 {
    #[inline(always)]
    fn from(val: Intencapt1Plcastat) -> u8 {
        Intencapt1Plcastat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt1Remjab {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt1Remjab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt1Remjab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt1Remjab {
    #[inline(always)]
    fn from(val: u8) -> Intencapt1Remjab {
        Intencapt1Remjab::from_bits(val)
    }
}
impl From<Intencapt1Remjab> for u8 {
    #[inline(always)]
    fn from(val: Intencapt1Remjab) -> u8 {
        Intencapt1Remjab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt1Smiaccess {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt1Smiaccess {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt1Smiaccess {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt1Smiaccess {
    #[inline(always)]
    fn from(val: u8) -> Intencapt1Smiaccess {
        Intencapt1Smiaccess::from_bits(val)
    }
}
impl From<Intencapt1Smiaccess> for u8 {
    #[inline(always)]
    fn from(val: Intencapt1Smiaccess) -> u8 {
        Intencapt1Smiaccess::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt1Sspdet {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt1Sspdet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt1Sspdet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt1Sspdet {
    #[inline(always)]
    fn from(val: u8) -> Intencapt1Sspdet {
        Intencapt1Sspdet::from_bits(val)
    }
}
impl From<Intencapt1Sspdet> for u8 {
    #[inline(always)]
    fn from(val: Intencapt1Sspdet) -> u8 {
        Intencapt1Sspdet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt1Wupdone {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt1Wupdone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt1Wupdone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt1Wupdone {
    #[inline(always)]
    fn from(val: u8) -> Intencapt1Wupdone {
        Intencapt1Wupdone::from_bits(val)
    }
}
impl From<Intencapt1Wupdone> for u8 {
    #[inline(always)]
    fn from(val: Intencapt1Wupdone) -> u8 {
        Intencapt1Wupdone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt1Wutdet {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt1Wutdet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt1Wutdet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt1Wutdet {
    #[inline(always)]
    fn from(val: u8) -> Intencapt1Wutdet {
        Intencapt1Wutdet::from_bits(val)
    }
}
impl From<Intencapt1Wutdet> for u8 {
    #[inline(always)]
    fn from(val: Intencapt1Wutdet) -> u8 {
        Intencapt1Wutdet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt2Apbparity {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt2Apbparity {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt2Apbparity {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt2Apbparity {
    #[inline(always)]
    fn from(val: u8) -> Intencapt2Apbparity {
        Intencapt2Apbparity::from_bits(val)
    }
}
impl From<Intencapt2Apbparity> for u8 {
    #[inline(always)]
    fn from(val: Intencapt2Apbparity) -> u8 {
        Intencapt2Apbparity::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt2Invldapb {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt2Invldapb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt2Invldapb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt2Invldapb {
    #[inline(always)]
    fn from(val: u8) -> Intencapt2Invldapb {
        Intencapt2Invldapb::from_bits(val)
    }
}
impl From<Intencapt2Invldapb> for u8 {
    #[inline(always)]
    fn from(val: Intencapt2Invldapb) -> u8 {
        Intencapt2Invldapb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt2Lclwk {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt2Lclwk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt2Lclwk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt2Lclwk {
    #[inline(always)]
    fn from(val: u8) -> Intencapt2Lclwk {
        Intencapt2Lclwk::from_bits(val)
    }
}
impl From<Intencapt2Lclwk> for u8 {
    #[inline(always)]
    fn from(val: Intencapt2Lclwk) -> u8 {
        Intencapt2Lclwk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt2Locjab {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt2Locjab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt2Locjab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt2Locjab {
    #[inline(always)]
    fn from(val: u8) -> Intencapt2Locjab {
        Intencapt2Locjab::from_bits(val)
    }
}
impl From<Intencapt2Locjab> for u8 {
    #[inline(always)]
    fn from(val: Intencapt2Locjab) -> u8 {
        Intencapt2Locjab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt2Modestat {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt2Modestat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt2Modestat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt2Modestat {
    #[inline(always)]
    fn from(val: u8) -> Intencapt2Modestat {
        Intencapt2Modestat::from_bits(val)
    }
}
impl From<Intencapt2Modestat> for u8 {
    #[inline(always)]
    fn from(val: Intencapt2Modestat) -> u8 {
        Intencapt2Modestat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt2Physcol {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt2Physcol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt2Physcol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt2Physcol {
    #[inline(always)]
    fn from(val: u8) -> Intencapt2Physcol {
        Intencapt2Physcol::from_bits(val)
    }
}
impl From<Intencapt2Physcol> for u8 {
    #[inline(always)]
    fn from(val: Intencapt2Physcol) -> u8 {
        Intencapt2Physcol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt2Pinfault {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt2Pinfault {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt2Pinfault {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt2Pinfault {
    #[inline(always)]
    fn from(val: u8) -> Intencapt2Pinfault {
        Intencapt2Pinfault::from_bits(val)
    }
}
impl From<Intencapt2Pinfault> for u8 {
    #[inline(always)]
    fn from(val: Intencapt2Pinfault) -> u8 {
        Intencapt2Pinfault::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt2Plcadiag {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt2Plcadiag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt2Plcadiag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt2Plcadiag {
    #[inline(always)]
    fn from(val: u8) -> Intencapt2Plcadiag {
        Intencapt2Plcadiag::from_bits(val)
    }
}
impl From<Intencapt2Plcadiag> for u8 {
    #[inline(always)]
    fn from(val: Intencapt2Plcadiag) -> u8 {
        Intencapt2Plcadiag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt2Plcarec {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt2Plcarec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt2Plcarec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt2Plcarec {
    #[inline(always)]
    fn from(val: u8) -> Intencapt2Plcarec {
        Intencapt2Plcarec::from_bits(val)
    }
}
impl From<Intencapt2Plcarec> for u8 {
    #[inline(always)]
    fn from(val: Intencapt2Plcarec) -> u8 {
        Intencapt2Plcarec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt2Plcastat {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt2Plcastat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt2Plcastat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt2Plcastat {
    #[inline(always)]
    fn from(val: u8) -> Intencapt2Plcastat {
        Intencapt2Plcastat::from_bits(val)
    }
}
impl From<Intencapt2Plcastat> for u8 {
    #[inline(always)]
    fn from(val: Intencapt2Plcastat) -> u8 {
        Intencapt2Plcastat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt2Remjab {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt2Remjab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt2Remjab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt2Remjab {
    #[inline(always)]
    fn from(val: u8) -> Intencapt2Remjab {
        Intencapt2Remjab::from_bits(val)
    }
}
impl From<Intencapt2Remjab> for u8 {
    #[inline(always)]
    fn from(val: Intencapt2Remjab) -> u8 {
        Intencapt2Remjab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt2Smiaccess {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt2Smiaccess {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt2Smiaccess {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt2Smiaccess {
    #[inline(always)]
    fn from(val: u8) -> Intencapt2Smiaccess {
        Intencapt2Smiaccess::from_bits(val)
    }
}
impl From<Intencapt2Smiaccess> for u8 {
    #[inline(always)]
    fn from(val: Intencapt2Smiaccess) -> u8 {
        Intencapt2Smiaccess::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt2Sspdet {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt2Sspdet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt2Sspdet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt2Sspdet {
    #[inline(always)]
    fn from(val: u8) -> Intencapt2Sspdet {
        Intencapt2Sspdet::from_bits(val)
    }
}
impl From<Intencapt2Sspdet> for u8 {
    #[inline(always)]
    fn from(val: Intencapt2Sspdet) -> u8 {
        Intencapt2Sspdet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt2Wupdone {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt2Wupdone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt2Wupdone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt2Wupdone {
    #[inline(always)]
    fn from(val: u8) -> Intencapt2Wupdone {
        Intencapt2Wupdone::from_bits(val)
    }
}
impl From<Intencapt2Wupdone> for u8 {
    #[inline(always)]
    fn from(val: Intencapt2Wupdone) -> u8 {
        Intencapt2Wupdone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intencapt2Wutdet {
    #[doc = "NO Effect"]
    NE = 0x0,
    #[doc = "Clear"]
    CLEAR = 0x01,
}
impl Intencapt2Wutdet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intencapt2Wutdet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intencapt2Wutdet {
    #[inline(always)]
    fn from(val: u8) -> Intencapt2Wutdet {
        Intencapt2Wutdet::from_bits(val)
    }
}
impl From<Intencapt2Wutdet> for u8 {
    #[inline(always)]
    fn from(val: Intencapt2Wutdet) -> u8 {
        Intencapt2Wutdet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr1Apbparity {
    #[doc = "NO_EFFECT"]
    ADIS = 0x0,
    #[doc = "DISABLE"]
    AEN = 0x01,
}
impl Intenclr1Apbparity {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr1Apbparity {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr1Apbparity {
    #[inline(always)]
    fn from(val: u8) -> Intenclr1Apbparity {
        Intenclr1Apbparity::from_bits(val)
    }
}
impl From<Intenclr1Apbparity> for u8 {
    #[inline(always)]
    fn from(val: Intenclr1Apbparity) -> u8 {
        Intenclr1Apbparity::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr1Invldapb {
    #[doc = "NO_EFFECT"]
    PHYDIS = 0x0,
    #[doc = "DISABLE"]
    PHYEN = 0x01,
}
impl Intenclr1Invldapb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr1Invldapb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr1Invldapb {
    #[inline(always)]
    fn from(val: u8) -> Intenclr1Invldapb {
        Intenclr1Invldapb::from_bits(val)
    }
}
impl From<Intenclr1Invldapb> for u8 {
    #[inline(always)]
    fn from(val: Intenclr1Invldapb) -> u8 {
        Intenclr1Invldapb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr1Lclwk {
    #[doc = "NO_EFFECT"]
    LCDIS = 0x0,
    #[doc = "DISABLE"]
    LCEN = 0x01,
}
impl Intenclr1Lclwk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr1Lclwk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr1Lclwk {
    #[inline(always)]
    fn from(val: u8) -> Intenclr1Lclwk {
        Intenclr1Lclwk::from_bits(val)
    }
}
impl From<Intenclr1Lclwk> for u8 {
    #[inline(always)]
    fn from(val: Intenclr1Lclwk) -> u8 {
        Intenclr1Lclwk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr1Locjab {
    #[doc = "NO_EFFECT"]
    PHYDIS = 0x0,
    #[doc = "DISABLE"]
    PHYEN = 0x01,
}
impl Intenclr1Locjab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr1Locjab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr1Locjab {
    #[inline(always)]
    fn from(val: u8) -> Intenclr1Locjab {
        Intenclr1Locjab::from_bits(val)
    }
}
impl From<Intenclr1Locjab> for u8 {
    #[inline(always)]
    fn from(val: Intenclr1Locjab) -> u8 {
        Intenclr1Locjab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr1Modestat {
    #[doc = "NO_EFFECT"]
    PHYDIS = 0x0,
    #[doc = "DISABLE"]
    PHYEN = 0x01,
}
impl Intenclr1Modestat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr1Modestat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr1Modestat {
    #[inline(always)]
    fn from(val: u8) -> Intenclr1Modestat {
        Intenclr1Modestat::from_bits(val)
    }
}
impl From<Intenclr1Modestat> for u8 {
    #[inline(always)]
    fn from(val: Intenclr1Modestat) -> u8 {
        Intenclr1Modestat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr1Physcol {
    #[doc = "NO_EFFECT"]
    PHYDIS = 0x0,
    #[doc = "DISABLE"]
    PHYEN = 0x01,
}
impl Intenclr1Physcol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr1Physcol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr1Physcol {
    #[inline(always)]
    fn from(val: u8) -> Intenclr1Physcol {
        Intenclr1Physcol::from_bits(val)
    }
}
impl From<Intenclr1Physcol> for u8 {
    #[inline(always)]
    fn from(val: Intenclr1Physcol) -> u8 {
        Intenclr1Physcol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr1Pinfault {
    #[doc = "NO_EFFECT"]
    PNDIS = 0x0,
    #[doc = "DISABLE"]
    PNEN = 0x01,
}
impl Intenclr1Pinfault {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr1Pinfault {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr1Pinfault {
    #[inline(always)]
    fn from(val: u8) -> Intenclr1Pinfault {
        Intenclr1Pinfault::from_bits(val)
    }
}
impl From<Intenclr1Pinfault> for u8 {
    #[inline(always)]
    fn from(val: Intenclr1Pinfault) -> u8 {
        Intenclr1Pinfault::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr1Plcadiag {
    #[doc = "NO_EFFECT"]
    PHDIS = 0x0,
    #[doc = "DISABLE"]
    PHEN = 0x01,
}
impl Intenclr1Plcadiag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr1Plcadiag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr1Plcadiag {
    #[inline(always)]
    fn from(val: u8) -> Intenclr1Plcadiag {
        Intenclr1Plcadiag::from_bits(val)
    }
}
impl From<Intenclr1Plcadiag> for u8 {
    #[inline(always)]
    fn from(val: Intenclr1Plcadiag) -> u8 {
        Intenclr1Plcadiag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr1Plcarec {
    #[doc = "NO_EFFECT"]
    PHYDIS = 0x0,
    #[doc = "DISABLE"]
    PHYEN = 0x01,
}
impl Intenclr1Plcarec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr1Plcarec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr1Plcarec {
    #[inline(always)]
    fn from(val: u8) -> Intenclr1Plcarec {
        Intenclr1Plcarec::from_bits(val)
    }
}
impl From<Intenclr1Plcarec> for u8 {
    #[inline(always)]
    fn from(val: Intenclr1Plcarec) -> u8 {
        Intenclr1Plcarec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr1Plcastat {
    #[doc = "NO_EFFECT"]
    PHYDIS = 0x0,
    #[doc = "DISABLE"]
    PHYEN = 0x01,
}
impl Intenclr1Plcastat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr1Plcastat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr1Plcastat {
    #[inline(always)]
    fn from(val: u8) -> Intenclr1Plcastat {
        Intenclr1Plcastat::from_bits(val)
    }
}
impl From<Intenclr1Plcastat> for u8 {
    #[inline(always)]
    fn from(val: Intenclr1Plcastat) -> u8 {
        Intenclr1Plcastat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr1Remjab {
    #[doc = "NO_EFFECT"]
    REDIS = 0x0,
    #[doc = "DISABLE"]
    REEN = 0x01,
}
impl Intenclr1Remjab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr1Remjab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr1Remjab {
    #[inline(always)]
    fn from(val: u8) -> Intenclr1Remjab {
        Intenclr1Remjab::from_bits(val)
    }
}
impl From<Intenclr1Remjab> for u8 {
    #[inline(always)]
    fn from(val: Intenclr1Remjab) -> u8 {
        Intenclr1Remjab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr1Smiaccess {
    #[doc = "NO_EFFECT"]
    SMDIS = 0x0,
    #[doc = "DISABLE"]
    SMEN = 0x01,
}
impl Intenclr1Smiaccess {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr1Smiaccess {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr1Smiaccess {
    #[inline(always)]
    fn from(val: u8) -> Intenclr1Smiaccess {
        Intenclr1Smiaccess::from_bits(val)
    }
}
impl From<Intenclr1Smiaccess> for u8 {
    #[inline(always)]
    fn from(val: Intenclr1Smiaccess) -> u8 {
        Intenclr1Smiaccess::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr1Sspdet {
    #[doc = "NO_EFFECT"]
    SSPDIS = 0x0,
    #[doc = "DISABLE"]
    SSPEN = 0x01,
}
impl Intenclr1Sspdet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr1Sspdet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr1Sspdet {
    #[inline(always)]
    fn from(val: u8) -> Intenclr1Sspdet {
        Intenclr1Sspdet::from_bits(val)
    }
}
impl From<Intenclr1Sspdet> for u8 {
    #[inline(always)]
    fn from(val: Intenclr1Sspdet) -> u8 {
        Intenclr1Sspdet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr1Wupdone {
    #[doc = "NO_EFFECT"]
    WDIS = 0x0,
    #[doc = "DISABLE"]
    WEN = 0x01,
}
impl Intenclr1Wupdone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr1Wupdone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr1Wupdone {
    #[inline(always)]
    fn from(val: u8) -> Intenclr1Wupdone {
        Intenclr1Wupdone::from_bits(val)
    }
}
impl From<Intenclr1Wupdone> for u8 {
    #[inline(always)]
    fn from(val: Intenclr1Wupdone) -> u8 {
        Intenclr1Wupdone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr1Wutdet {
    #[doc = "NO_EFFECT"]
    WUDIS = 0x0,
    #[doc = "DISABLE"]
    WUEN = 0x01,
}
impl Intenclr1Wutdet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr1Wutdet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr1Wutdet {
    #[inline(always)]
    fn from(val: u8) -> Intenclr1Wutdet {
        Intenclr1Wutdet::from_bits(val)
    }
}
impl From<Intenclr1Wutdet> for u8 {
    #[inline(always)]
    fn from(val: Intenclr1Wutdet) -> u8 {
        Intenclr1Wutdet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr2Apbparity {
    #[doc = "NO_EFFECT"]
    ADIS = 0x0,
    #[doc = "DISABLE"]
    AEN = 0x01,
}
impl Intenclr2Apbparity {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr2Apbparity {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr2Apbparity {
    #[inline(always)]
    fn from(val: u8) -> Intenclr2Apbparity {
        Intenclr2Apbparity::from_bits(val)
    }
}
impl From<Intenclr2Apbparity> for u8 {
    #[inline(always)]
    fn from(val: Intenclr2Apbparity) -> u8 {
        Intenclr2Apbparity::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr2Invldapb {
    #[doc = "NO_EFFECT"]
    PHYDIS = 0x0,
    #[doc = "DISABLE"]
    PHYEN = 0x01,
}
impl Intenclr2Invldapb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr2Invldapb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr2Invldapb {
    #[inline(always)]
    fn from(val: u8) -> Intenclr2Invldapb {
        Intenclr2Invldapb::from_bits(val)
    }
}
impl From<Intenclr2Invldapb> for u8 {
    #[inline(always)]
    fn from(val: Intenclr2Invldapb) -> u8 {
        Intenclr2Invldapb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr2Lclwk {
    #[doc = "NO_EFFECT"]
    LCDIS = 0x0,
    #[doc = "DISABLE"]
    LCEN = 0x01,
}
impl Intenclr2Lclwk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr2Lclwk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr2Lclwk {
    #[inline(always)]
    fn from(val: u8) -> Intenclr2Lclwk {
        Intenclr2Lclwk::from_bits(val)
    }
}
impl From<Intenclr2Lclwk> for u8 {
    #[inline(always)]
    fn from(val: Intenclr2Lclwk) -> u8 {
        Intenclr2Lclwk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr2Locjab {
    #[doc = "NO_EFFECT"]
    PHYDIS = 0x0,
    #[doc = "DISABLE"]
    PHYEN = 0x01,
}
impl Intenclr2Locjab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr2Locjab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr2Locjab {
    #[inline(always)]
    fn from(val: u8) -> Intenclr2Locjab {
        Intenclr2Locjab::from_bits(val)
    }
}
impl From<Intenclr2Locjab> for u8 {
    #[inline(always)]
    fn from(val: Intenclr2Locjab) -> u8 {
        Intenclr2Locjab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr2Modestat {
    #[doc = "NO_EFFECT"]
    PHYDIS = 0x0,
    #[doc = "DISABLE"]
    PHYEN = 0x01,
}
impl Intenclr2Modestat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr2Modestat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr2Modestat {
    #[inline(always)]
    fn from(val: u8) -> Intenclr2Modestat {
        Intenclr2Modestat::from_bits(val)
    }
}
impl From<Intenclr2Modestat> for u8 {
    #[inline(always)]
    fn from(val: Intenclr2Modestat) -> u8 {
        Intenclr2Modestat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr2Physcol {
    #[doc = "NO_EFFECT"]
    PHYDIS = 0x0,
    #[doc = "DISABLE"]
    PHYEN = 0x01,
}
impl Intenclr2Physcol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr2Physcol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr2Physcol {
    #[inline(always)]
    fn from(val: u8) -> Intenclr2Physcol {
        Intenclr2Physcol::from_bits(val)
    }
}
impl From<Intenclr2Physcol> for u8 {
    #[inline(always)]
    fn from(val: Intenclr2Physcol) -> u8 {
        Intenclr2Physcol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr2Pinfault {
    #[doc = "NO_EFFECT"]
    PNDIS = 0x0,
    #[doc = "DISABLE"]
    PNEN = 0x01,
}
impl Intenclr2Pinfault {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr2Pinfault {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr2Pinfault {
    #[inline(always)]
    fn from(val: u8) -> Intenclr2Pinfault {
        Intenclr2Pinfault::from_bits(val)
    }
}
impl From<Intenclr2Pinfault> for u8 {
    #[inline(always)]
    fn from(val: Intenclr2Pinfault) -> u8 {
        Intenclr2Pinfault::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr2Plcadiag {
    #[doc = "NO_EFFECT"]
    PHDIS = 0x0,
    #[doc = "DISABLE"]
    PHEN = 0x01,
}
impl Intenclr2Plcadiag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr2Plcadiag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr2Plcadiag {
    #[inline(always)]
    fn from(val: u8) -> Intenclr2Plcadiag {
        Intenclr2Plcadiag::from_bits(val)
    }
}
impl From<Intenclr2Plcadiag> for u8 {
    #[inline(always)]
    fn from(val: Intenclr2Plcadiag) -> u8 {
        Intenclr2Plcadiag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr2Plcarec {
    #[doc = "NO_EFFECT"]
    PHYDIS = 0x0,
    #[doc = "DISABLE"]
    PHYEN = 0x01,
}
impl Intenclr2Plcarec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr2Plcarec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr2Plcarec {
    #[inline(always)]
    fn from(val: u8) -> Intenclr2Plcarec {
        Intenclr2Plcarec::from_bits(val)
    }
}
impl From<Intenclr2Plcarec> for u8 {
    #[inline(always)]
    fn from(val: Intenclr2Plcarec) -> u8 {
        Intenclr2Plcarec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr2Plcastat {
    #[doc = "NO_EFFECT"]
    PHYDIS = 0x0,
    #[doc = "DISABLE"]
    PHYEN = 0x01,
}
impl Intenclr2Plcastat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr2Plcastat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr2Plcastat {
    #[inline(always)]
    fn from(val: u8) -> Intenclr2Plcastat {
        Intenclr2Plcastat::from_bits(val)
    }
}
impl From<Intenclr2Plcastat> for u8 {
    #[inline(always)]
    fn from(val: Intenclr2Plcastat) -> u8 {
        Intenclr2Plcastat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr2Remjab {
    #[doc = "NO_EFFECT"]
    REDIS = 0x0,
    #[doc = "DISABLE"]
    REEN = 0x01,
}
impl Intenclr2Remjab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr2Remjab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr2Remjab {
    #[inline(always)]
    fn from(val: u8) -> Intenclr2Remjab {
        Intenclr2Remjab::from_bits(val)
    }
}
impl From<Intenclr2Remjab> for u8 {
    #[inline(always)]
    fn from(val: Intenclr2Remjab) -> u8 {
        Intenclr2Remjab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr2Smiaccess {
    #[doc = "NO_EFFECT"]
    SMDIS = 0x0,
    #[doc = "DISABLE"]
    SMEN = 0x01,
}
impl Intenclr2Smiaccess {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr2Smiaccess {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr2Smiaccess {
    #[inline(always)]
    fn from(val: u8) -> Intenclr2Smiaccess {
        Intenclr2Smiaccess::from_bits(val)
    }
}
impl From<Intenclr2Smiaccess> for u8 {
    #[inline(always)]
    fn from(val: Intenclr2Smiaccess) -> u8 {
        Intenclr2Smiaccess::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr2Sspdet {
    #[doc = "NO_EFFECT"]
    SSPDIS = 0x0,
    #[doc = "DISABLE"]
    SSPEN = 0x01,
}
impl Intenclr2Sspdet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr2Sspdet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr2Sspdet {
    #[inline(always)]
    fn from(val: u8) -> Intenclr2Sspdet {
        Intenclr2Sspdet::from_bits(val)
    }
}
impl From<Intenclr2Sspdet> for u8 {
    #[inline(always)]
    fn from(val: Intenclr2Sspdet) -> u8 {
        Intenclr2Sspdet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr2Wupdone {
    #[doc = "NO_EFFECT"]
    WDIS = 0x0,
    #[doc = "DISABLE"]
    WEN = 0x01,
}
impl Intenclr2Wupdone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr2Wupdone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr2Wupdone {
    #[inline(always)]
    fn from(val: u8) -> Intenclr2Wupdone {
        Intenclr2Wupdone::from_bits(val)
    }
}
impl From<Intenclr2Wupdone> for u8 {
    #[inline(always)]
    fn from(val: Intenclr2Wupdone) -> u8 {
        Intenclr2Wupdone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenclr2Wutdet {
    #[doc = "NO_EFFECT"]
    WUDIS = 0x0,
    #[doc = "DISABLE"]
    WUEN = 0x01,
}
impl Intenclr2Wutdet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenclr2Wutdet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenclr2Wutdet {
    #[inline(always)]
    fn from(val: u8) -> Intenclr2Wutdet {
        Intenclr2Wutdet::from_bits(val)
    }
}
impl From<Intenclr2Wutdet> for u8 {
    #[inline(always)]
    fn from(val: Intenclr2Wutdet) -> u8 {
        Intenclr2Wutdet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset1Apbparity {
    #[doc = "Disabled"]
    APBDIS = 0x0,
    #[doc = "ENABLED"]
    APBEN = 0x01,
}
impl Intenset1Apbparity {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset1Apbparity {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset1Apbparity {
    #[inline(always)]
    fn from(val: u8) -> Intenset1Apbparity {
        Intenset1Apbparity::from_bits(val)
    }
}
impl From<Intenset1Apbparity> for u8 {
    #[inline(always)]
    fn from(val: Intenset1Apbparity) -> u8 {
        Intenset1Apbparity::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset1Invldapb {
    #[doc = "Disabled"]
    INVLDIS = 0x0,
    #[doc = "ENABLED"]
    INVLEN = 0x01,
}
impl Intenset1Invldapb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset1Invldapb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset1Invldapb {
    #[inline(always)]
    fn from(val: u8) -> Intenset1Invldapb {
        Intenset1Invldapb::from_bits(val)
    }
}
impl From<Intenset1Invldapb> for u8 {
    #[inline(always)]
    fn from(val: Intenset1Invldapb) -> u8 {
        Intenset1Invldapb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset1Lclwk {
    #[doc = "Disabled"]
    LCLDIS = 0x0,
    #[doc = "ENABLED"]
    LCLEN = 0x01,
}
impl Intenset1Lclwk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset1Lclwk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset1Lclwk {
    #[inline(always)]
    fn from(val: u8) -> Intenset1Lclwk {
        Intenset1Lclwk::from_bits(val)
    }
}
impl From<Intenset1Lclwk> for u8 {
    #[inline(always)]
    fn from(val: Intenset1Lclwk) -> u8 {
        Intenset1Lclwk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset1Locjab {
    #[doc = "Disabled"]
    LOCDIS = 0x0,
    #[doc = "ENABLED"]
    LOCEN = 0x01,
}
impl Intenset1Locjab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset1Locjab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset1Locjab {
    #[inline(always)]
    fn from(val: u8) -> Intenset1Locjab {
        Intenset1Locjab::from_bits(val)
    }
}
impl From<Intenset1Locjab> for u8 {
    #[inline(always)]
    fn from(val: Intenset1Locjab) -> u8 {
        Intenset1Locjab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset1Modestat {
    #[doc = "Disabled"]
    MODEDIS = 0x0,
    #[doc = "ENABLED"]
    MODEEN = 0x01,
}
impl Intenset1Modestat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset1Modestat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset1Modestat {
    #[inline(always)]
    fn from(val: u8) -> Intenset1Modestat {
        Intenset1Modestat::from_bits(val)
    }
}
impl From<Intenset1Modestat> for u8 {
    #[inline(always)]
    fn from(val: Intenset1Modestat) -> u8 {
        Intenset1Modestat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset1Physcol {
    #[doc = "Disabled"]
    PHYDIS = 0x0,
    #[doc = "ENABLED"]
    PHYEN = 0x01,
}
impl Intenset1Physcol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset1Physcol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset1Physcol {
    #[inline(always)]
    fn from(val: u8) -> Intenset1Physcol {
        Intenset1Physcol::from_bits(val)
    }
}
impl From<Intenset1Physcol> for u8 {
    #[inline(always)]
    fn from(val: Intenset1Physcol) -> u8 {
        Intenset1Physcol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset1Pinfault {
    #[doc = "Disabled"]
    PINDIS = 0x0,
    #[doc = "ENABLED"]
    PINEN = 0x01,
}
impl Intenset1Pinfault {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset1Pinfault {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset1Pinfault {
    #[inline(always)]
    fn from(val: u8) -> Intenset1Pinfault {
        Intenset1Pinfault::from_bits(val)
    }
}
impl From<Intenset1Pinfault> for u8 {
    #[inline(always)]
    fn from(val: Intenset1Pinfault) -> u8 {
        Intenset1Pinfault::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset1Plcadiag {
    #[doc = "Disabled"]
    PLCADIS = 0x0,
    #[doc = "ENABLED"]
    PLCEN = 0x01,
}
impl Intenset1Plcadiag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset1Plcadiag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset1Plcadiag {
    #[inline(always)]
    fn from(val: u8) -> Intenset1Plcadiag {
        Intenset1Plcadiag::from_bits(val)
    }
}
impl From<Intenset1Plcadiag> for u8 {
    #[inline(always)]
    fn from(val: Intenset1Plcadiag) -> u8 {
        Intenset1Plcadiag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset1Plcarec {
    #[doc = "Disabled"]
    PLCDIS = 0x0,
    #[doc = "ENABLED"]
    PLCEN = 0x01,
}
impl Intenset1Plcarec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset1Plcarec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset1Plcarec {
    #[inline(always)]
    fn from(val: u8) -> Intenset1Plcarec {
        Intenset1Plcarec::from_bits(val)
    }
}
impl From<Intenset1Plcarec> for u8 {
    #[inline(always)]
    fn from(val: Intenset1Plcarec) -> u8 {
        Intenset1Plcarec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset1Plcastat {
    #[doc = "Disabled"]
    PLCDIS = 0x0,
    #[doc = "ENABLED"]
    PLCEN = 0x01,
}
impl Intenset1Plcastat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset1Plcastat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset1Plcastat {
    #[inline(always)]
    fn from(val: u8) -> Intenset1Plcastat {
        Intenset1Plcastat::from_bits(val)
    }
}
impl From<Intenset1Plcastat> for u8 {
    #[inline(always)]
    fn from(val: Intenset1Plcastat) -> u8 {
        Intenset1Plcastat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset1Remjab {
    #[doc = "Disabled"]
    REMDIS = 0x0,
    #[doc = "ENABLED"]
    REMEN = 0x01,
}
impl Intenset1Remjab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset1Remjab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset1Remjab {
    #[inline(always)]
    fn from(val: u8) -> Intenset1Remjab {
        Intenset1Remjab::from_bits(val)
    }
}
impl From<Intenset1Remjab> for u8 {
    #[inline(always)]
    fn from(val: Intenset1Remjab) -> u8 {
        Intenset1Remjab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset1Smiaccess {
    #[doc = "Disabled"]
    SMIDIS = 0x0,
    #[doc = "ENABLED"]
    SMIEN = 0x01,
}
impl Intenset1Smiaccess {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset1Smiaccess {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset1Smiaccess {
    #[inline(always)]
    fn from(val: u8) -> Intenset1Smiaccess {
        Intenset1Smiaccess::from_bits(val)
    }
}
impl From<Intenset1Smiaccess> for u8 {
    #[inline(always)]
    fn from(val: Intenset1Smiaccess) -> u8 {
        Intenset1Smiaccess::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset1Sspdet {
    #[doc = "Disabled"]
    SSPDIS = 0x0,
    #[doc = "ENABLED"]
    SSPEN = 0x01,
}
impl Intenset1Sspdet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset1Sspdet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset1Sspdet {
    #[inline(always)]
    fn from(val: u8) -> Intenset1Sspdet {
        Intenset1Sspdet::from_bits(val)
    }
}
impl From<Intenset1Sspdet> for u8 {
    #[inline(always)]
    fn from(val: Intenset1Sspdet) -> u8 {
        Intenset1Sspdet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset1Wupdone {
    #[doc = "Disabled"]
    WUPDIS = 0x0,
    #[doc = "ENABLED"]
    WUPEN = 0x01,
}
impl Intenset1Wupdone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset1Wupdone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset1Wupdone {
    #[inline(always)]
    fn from(val: u8) -> Intenset1Wupdone {
        Intenset1Wupdone::from_bits(val)
    }
}
impl From<Intenset1Wupdone> for u8 {
    #[inline(always)]
    fn from(val: Intenset1Wupdone) -> u8 {
        Intenset1Wupdone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset1Wutdet {
    #[doc = "Disabled"]
    WUTDIS = 0x0,
    #[doc = "ENABLED"]
    WUTEN = 0x01,
}
impl Intenset1Wutdet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset1Wutdet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset1Wutdet {
    #[inline(always)]
    fn from(val: u8) -> Intenset1Wutdet {
        Intenset1Wutdet::from_bits(val)
    }
}
impl From<Intenset1Wutdet> for u8 {
    #[inline(always)]
    fn from(val: Intenset1Wutdet) -> u8 {
        Intenset1Wutdet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset2Apbparity {
    #[doc = "Disabled"]
    APBDIS = 0x0,
    #[doc = "ENABLED"]
    APBEN = 0x01,
}
impl Intenset2Apbparity {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset2Apbparity {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset2Apbparity {
    #[inline(always)]
    fn from(val: u8) -> Intenset2Apbparity {
        Intenset2Apbparity::from_bits(val)
    }
}
impl From<Intenset2Apbparity> for u8 {
    #[inline(always)]
    fn from(val: Intenset2Apbparity) -> u8 {
        Intenset2Apbparity::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset2Invldapb {
    #[doc = "Disabled"]
    INVLDIS = 0x0,
    #[doc = "ENABLED"]
    INVLEN = 0x01,
}
impl Intenset2Invldapb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset2Invldapb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset2Invldapb {
    #[inline(always)]
    fn from(val: u8) -> Intenset2Invldapb {
        Intenset2Invldapb::from_bits(val)
    }
}
impl From<Intenset2Invldapb> for u8 {
    #[inline(always)]
    fn from(val: Intenset2Invldapb) -> u8 {
        Intenset2Invldapb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset2Lclwk {
    #[doc = "Disabled"]
    LCLDIS = 0x0,
    #[doc = "ENABLED"]
    LCLEN = 0x01,
}
impl Intenset2Lclwk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset2Lclwk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset2Lclwk {
    #[inline(always)]
    fn from(val: u8) -> Intenset2Lclwk {
        Intenset2Lclwk::from_bits(val)
    }
}
impl From<Intenset2Lclwk> for u8 {
    #[inline(always)]
    fn from(val: Intenset2Lclwk) -> u8 {
        Intenset2Lclwk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset2Locjab {
    #[doc = "Disabled"]
    LOCDIS = 0x0,
    #[doc = "ENABLED"]
    LOCEN = 0x01,
}
impl Intenset2Locjab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset2Locjab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset2Locjab {
    #[inline(always)]
    fn from(val: u8) -> Intenset2Locjab {
        Intenset2Locjab::from_bits(val)
    }
}
impl From<Intenset2Locjab> for u8 {
    #[inline(always)]
    fn from(val: Intenset2Locjab) -> u8 {
        Intenset2Locjab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset2Modestat {
    #[doc = "Disabled"]
    MODEDIS = 0x0,
    #[doc = "ENABLED"]
    MODEEN = 0x01,
}
impl Intenset2Modestat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset2Modestat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset2Modestat {
    #[inline(always)]
    fn from(val: u8) -> Intenset2Modestat {
        Intenset2Modestat::from_bits(val)
    }
}
impl From<Intenset2Modestat> for u8 {
    #[inline(always)]
    fn from(val: Intenset2Modestat) -> u8 {
        Intenset2Modestat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset2Physcol {
    #[doc = "Disabled"]
    PHYDIS = 0x0,
    #[doc = "ENABLED"]
    PHYEN = 0x01,
}
impl Intenset2Physcol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset2Physcol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset2Physcol {
    #[inline(always)]
    fn from(val: u8) -> Intenset2Physcol {
        Intenset2Physcol::from_bits(val)
    }
}
impl From<Intenset2Physcol> for u8 {
    #[inline(always)]
    fn from(val: Intenset2Physcol) -> u8 {
        Intenset2Physcol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset2Pinfault {
    #[doc = "Disabled"]
    PINDIS = 0x0,
    #[doc = "ENABLED"]
    PINEN = 0x01,
}
impl Intenset2Pinfault {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset2Pinfault {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset2Pinfault {
    #[inline(always)]
    fn from(val: u8) -> Intenset2Pinfault {
        Intenset2Pinfault::from_bits(val)
    }
}
impl From<Intenset2Pinfault> for u8 {
    #[inline(always)]
    fn from(val: Intenset2Pinfault) -> u8 {
        Intenset2Pinfault::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset2Plcadiag {
    #[doc = "Disabled"]
    PLCADIS = 0x0,
    #[doc = "ENABLED"]
    PLCEN = 0x01,
}
impl Intenset2Plcadiag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset2Plcadiag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset2Plcadiag {
    #[inline(always)]
    fn from(val: u8) -> Intenset2Plcadiag {
        Intenset2Plcadiag::from_bits(val)
    }
}
impl From<Intenset2Plcadiag> for u8 {
    #[inline(always)]
    fn from(val: Intenset2Plcadiag) -> u8 {
        Intenset2Plcadiag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset2Plcarec {
    #[doc = "Disabled"]
    PLCDIS = 0x0,
    #[doc = "ENABLED"]
    PLCEN = 0x01,
}
impl Intenset2Plcarec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset2Plcarec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset2Plcarec {
    #[inline(always)]
    fn from(val: u8) -> Intenset2Plcarec {
        Intenset2Plcarec::from_bits(val)
    }
}
impl From<Intenset2Plcarec> for u8 {
    #[inline(always)]
    fn from(val: Intenset2Plcarec) -> u8 {
        Intenset2Plcarec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset2Plcastat {
    #[doc = "Disabled"]
    PLCDIS = 0x0,
    #[doc = "ENABLED"]
    PLCEN = 0x01,
}
impl Intenset2Plcastat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset2Plcastat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset2Plcastat {
    #[inline(always)]
    fn from(val: u8) -> Intenset2Plcastat {
        Intenset2Plcastat::from_bits(val)
    }
}
impl From<Intenset2Plcastat> for u8 {
    #[inline(always)]
    fn from(val: Intenset2Plcastat) -> u8 {
        Intenset2Plcastat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset2Remjab {
    #[doc = "Disabled"]
    REMDIS = 0x0,
    #[doc = "ENABLED"]
    REMEN = 0x01,
}
impl Intenset2Remjab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset2Remjab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset2Remjab {
    #[inline(always)]
    fn from(val: u8) -> Intenset2Remjab {
        Intenset2Remjab::from_bits(val)
    }
}
impl From<Intenset2Remjab> for u8 {
    #[inline(always)]
    fn from(val: Intenset2Remjab) -> u8 {
        Intenset2Remjab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset2Smiaccess {
    #[doc = "Disabled"]
    SMIDIS = 0x0,
    #[doc = "ENABLED"]
    SMIEN = 0x01,
}
impl Intenset2Smiaccess {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset2Smiaccess {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset2Smiaccess {
    #[inline(always)]
    fn from(val: u8) -> Intenset2Smiaccess {
        Intenset2Smiaccess::from_bits(val)
    }
}
impl From<Intenset2Smiaccess> for u8 {
    #[inline(always)]
    fn from(val: Intenset2Smiaccess) -> u8 {
        Intenset2Smiaccess::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset2Sspdet {
    #[doc = "Disabled"]
    SSPDIS = 0x0,
    #[doc = "ENABLED"]
    SSPEN = 0x01,
}
impl Intenset2Sspdet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset2Sspdet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset2Sspdet {
    #[inline(always)]
    fn from(val: u8) -> Intenset2Sspdet {
        Intenset2Sspdet::from_bits(val)
    }
}
impl From<Intenset2Sspdet> for u8 {
    #[inline(always)]
    fn from(val: Intenset2Sspdet) -> u8 {
        Intenset2Sspdet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset2Wupdone {
    #[doc = "Disabled"]
    WUPDIS = 0x0,
    #[doc = "ENABLED"]
    WUPEN = 0x01,
}
impl Intenset2Wupdone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset2Wupdone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset2Wupdone {
    #[inline(always)]
    fn from(val: u8) -> Intenset2Wupdone {
        Intenset2Wupdone::from_bits(val)
    }
}
impl From<Intenset2Wupdone> for u8 {
    #[inline(always)]
    fn from(val: Intenset2Wupdone) -> u8 {
        Intenset2Wupdone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intenset2Wutdet {
    #[doc = "Disabled"]
    WUTDIS = 0x0,
    #[doc = "ENABLED"]
    WUTEN = 0x01,
}
impl Intenset2Wutdet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intenset2Wutdet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intenset2Wutdet {
    #[inline(always)]
    fn from(val: u8) -> Intenset2Wutdet {
        Intenset2Wutdet::from_bits(val)
    }
}
impl From<Intenset2Wutdet> for u8 {
    #[inline(always)]
    fn from(val: Intenset2Wutdet) -> u8 {
        Intenset2Wutdet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat1Apbparity {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat1Apbparity {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat1Apbparity {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat1Apbparity {
    #[inline(always)]
    fn from(val: u8) -> Intstat1Apbparity {
        Intstat1Apbparity::from_bits(val)
    }
}
impl From<Intstat1Apbparity> for u8 {
    #[inline(always)]
    fn from(val: Intstat1Apbparity) -> u8 {
        Intstat1Apbparity::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat1Invldapb {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat1Invldapb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat1Invldapb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat1Invldapb {
    #[inline(always)]
    fn from(val: u8) -> Intstat1Invldapb {
        Intstat1Invldapb::from_bits(val)
    }
}
impl From<Intstat1Invldapb> for u8 {
    #[inline(always)]
    fn from(val: Intstat1Invldapb) -> u8 {
        Intstat1Invldapb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat1Lclwk {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat1Lclwk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat1Lclwk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat1Lclwk {
    #[inline(always)]
    fn from(val: u8) -> Intstat1Lclwk {
        Intstat1Lclwk::from_bits(val)
    }
}
impl From<Intstat1Lclwk> for u8 {
    #[inline(always)]
    fn from(val: Intstat1Lclwk) -> u8 {
        Intstat1Lclwk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat1Locjab {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat1Locjab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat1Locjab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat1Locjab {
    #[inline(always)]
    fn from(val: u8) -> Intstat1Locjab {
        Intstat1Locjab::from_bits(val)
    }
}
impl From<Intstat1Locjab> for u8 {
    #[inline(always)]
    fn from(val: Intstat1Locjab) -> u8 {
        Intstat1Locjab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat1Modestat {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat1Modestat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat1Modestat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat1Modestat {
    #[inline(always)]
    fn from(val: u8) -> Intstat1Modestat {
        Intstat1Modestat::from_bits(val)
    }
}
impl From<Intstat1Modestat> for u8 {
    #[inline(always)]
    fn from(val: Intstat1Modestat) -> u8 {
        Intstat1Modestat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat1Physcol {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat1Physcol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat1Physcol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat1Physcol {
    #[inline(always)]
    fn from(val: u8) -> Intstat1Physcol {
        Intstat1Physcol::from_bits(val)
    }
}
impl From<Intstat1Physcol> for u8 {
    #[inline(always)]
    fn from(val: Intstat1Physcol) -> u8 {
        Intstat1Physcol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat1Pinfault {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat1Pinfault {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat1Pinfault {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat1Pinfault {
    #[inline(always)]
    fn from(val: u8) -> Intstat1Pinfault {
        Intstat1Pinfault::from_bits(val)
    }
}
impl From<Intstat1Pinfault> for u8 {
    #[inline(always)]
    fn from(val: Intstat1Pinfault) -> u8 {
        Intstat1Pinfault::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat1Plcadiag {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat1Plcadiag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat1Plcadiag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat1Plcadiag {
    #[inline(always)]
    fn from(val: u8) -> Intstat1Plcadiag {
        Intstat1Plcadiag::from_bits(val)
    }
}
impl From<Intstat1Plcadiag> for u8 {
    #[inline(always)]
    fn from(val: Intstat1Plcadiag) -> u8 {
        Intstat1Plcadiag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat1Plcarec {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat1Plcarec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat1Plcarec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat1Plcarec {
    #[inline(always)]
    fn from(val: u8) -> Intstat1Plcarec {
        Intstat1Plcarec::from_bits(val)
    }
}
impl From<Intstat1Plcarec> for u8 {
    #[inline(always)]
    fn from(val: Intstat1Plcarec) -> u8 {
        Intstat1Plcarec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat1Plcastat {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat1Plcastat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat1Plcastat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat1Plcastat {
    #[inline(always)]
    fn from(val: u8) -> Intstat1Plcastat {
        Intstat1Plcastat::from_bits(val)
    }
}
impl From<Intstat1Plcastat> for u8 {
    #[inline(always)]
    fn from(val: Intstat1Plcastat) -> u8 {
        Intstat1Plcastat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat1Remjab {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat1Remjab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat1Remjab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat1Remjab {
    #[inline(always)]
    fn from(val: u8) -> Intstat1Remjab {
        Intstat1Remjab::from_bits(val)
    }
}
impl From<Intstat1Remjab> for u8 {
    #[inline(always)]
    fn from(val: Intstat1Remjab) -> u8 {
        Intstat1Remjab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat1Smiaccess {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat1Smiaccess {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat1Smiaccess {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat1Smiaccess {
    #[inline(always)]
    fn from(val: u8) -> Intstat1Smiaccess {
        Intstat1Smiaccess::from_bits(val)
    }
}
impl From<Intstat1Smiaccess> for u8 {
    #[inline(always)]
    fn from(val: Intstat1Smiaccess) -> u8 {
        Intstat1Smiaccess::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat1Sspdet {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat1Sspdet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat1Sspdet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat1Sspdet {
    #[inline(always)]
    fn from(val: u8) -> Intstat1Sspdet {
        Intstat1Sspdet::from_bits(val)
    }
}
impl From<Intstat1Sspdet> for u8 {
    #[inline(always)]
    fn from(val: Intstat1Sspdet) -> u8 {
        Intstat1Sspdet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat1Wupdone {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat1Wupdone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat1Wupdone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat1Wupdone {
    #[inline(always)]
    fn from(val: u8) -> Intstat1Wupdone {
        Intstat1Wupdone::from_bits(val)
    }
}
impl From<Intstat1Wupdone> for u8 {
    #[inline(always)]
    fn from(val: Intstat1Wupdone) -> u8 {
        Intstat1Wupdone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat1Wutdet {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat1Wutdet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat1Wutdet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat1Wutdet {
    #[inline(always)]
    fn from(val: u8) -> Intstat1Wutdet {
        Intstat1Wutdet::from_bits(val)
    }
}
impl From<Intstat1Wutdet> for u8 {
    #[inline(always)]
    fn from(val: Intstat1Wutdet) -> u8 {
        Intstat1Wutdet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat2Apbparity {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat2Apbparity {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat2Apbparity {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat2Apbparity {
    #[inline(always)]
    fn from(val: u8) -> Intstat2Apbparity {
        Intstat2Apbparity::from_bits(val)
    }
}
impl From<Intstat2Apbparity> for u8 {
    #[inline(always)]
    fn from(val: Intstat2Apbparity) -> u8 {
        Intstat2Apbparity::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat2Invldapb {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat2Invldapb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat2Invldapb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat2Invldapb {
    #[inline(always)]
    fn from(val: u8) -> Intstat2Invldapb {
        Intstat2Invldapb::from_bits(val)
    }
}
impl From<Intstat2Invldapb> for u8 {
    #[inline(always)]
    fn from(val: Intstat2Invldapb) -> u8 {
        Intstat2Invldapb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat2Lclwk {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat2Lclwk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat2Lclwk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat2Lclwk {
    #[inline(always)]
    fn from(val: u8) -> Intstat2Lclwk {
        Intstat2Lclwk::from_bits(val)
    }
}
impl From<Intstat2Lclwk> for u8 {
    #[inline(always)]
    fn from(val: Intstat2Lclwk) -> u8 {
        Intstat2Lclwk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat2Locjab {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat2Locjab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat2Locjab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat2Locjab {
    #[inline(always)]
    fn from(val: u8) -> Intstat2Locjab {
        Intstat2Locjab::from_bits(val)
    }
}
impl From<Intstat2Locjab> for u8 {
    #[inline(always)]
    fn from(val: Intstat2Locjab) -> u8 {
        Intstat2Locjab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat2Modestat {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat2Modestat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat2Modestat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat2Modestat {
    #[inline(always)]
    fn from(val: u8) -> Intstat2Modestat {
        Intstat2Modestat::from_bits(val)
    }
}
impl From<Intstat2Modestat> for u8 {
    #[inline(always)]
    fn from(val: Intstat2Modestat) -> u8 {
        Intstat2Modestat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat2Physcol {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat2Physcol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat2Physcol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat2Physcol {
    #[inline(always)]
    fn from(val: u8) -> Intstat2Physcol {
        Intstat2Physcol::from_bits(val)
    }
}
impl From<Intstat2Physcol> for u8 {
    #[inline(always)]
    fn from(val: Intstat2Physcol) -> u8 {
        Intstat2Physcol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat2Pinfault {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat2Pinfault {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat2Pinfault {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat2Pinfault {
    #[inline(always)]
    fn from(val: u8) -> Intstat2Pinfault {
        Intstat2Pinfault::from_bits(val)
    }
}
impl From<Intstat2Pinfault> for u8 {
    #[inline(always)]
    fn from(val: Intstat2Pinfault) -> u8 {
        Intstat2Pinfault::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat2Plcadiag {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat2Plcadiag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat2Plcadiag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat2Plcadiag {
    #[inline(always)]
    fn from(val: u8) -> Intstat2Plcadiag {
        Intstat2Plcadiag::from_bits(val)
    }
}
impl From<Intstat2Plcadiag> for u8 {
    #[inline(always)]
    fn from(val: Intstat2Plcadiag) -> u8 {
        Intstat2Plcadiag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat2Plcarec {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat2Plcarec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat2Plcarec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat2Plcarec {
    #[inline(always)]
    fn from(val: u8) -> Intstat2Plcarec {
        Intstat2Plcarec::from_bits(val)
    }
}
impl From<Intstat2Plcarec> for u8 {
    #[inline(always)]
    fn from(val: Intstat2Plcarec) -> u8 {
        Intstat2Plcarec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat2Plcastat {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat2Plcastat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat2Plcastat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat2Plcastat {
    #[inline(always)]
    fn from(val: u8) -> Intstat2Plcastat {
        Intstat2Plcastat::from_bits(val)
    }
}
impl From<Intstat2Plcastat> for u8 {
    #[inline(always)]
    fn from(val: Intstat2Plcastat) -> u8 {
        Intstat2Plcastat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat2Remjab {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat2Remjab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat2Remjab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat2Remjab {
    #[inline(always)]
    fn from(val: u8) -> Intstat2Remjab {
        Intstat2Remjab::from_bits(val)
    }
}
impl From<Intstat2Remjab> for u8 {
    #[inline(always)]
    fn from(val: Intstat2Remjab) -> u8 {
        Intstat2Remjab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat2Smiaccess {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat2Smiaccess {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat2Smiaccess {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat2Smiaccess {
    #[inline(always)]
    fn from(val: u8) -> Intstat2Smiaccess {
        Intstat2Smiaccess::from_bits(val)
    }
}
impl From<Intstat2Smiaccess> for u8 {
    #[inline(always)]
    fn from(val: Intstat2Smiaccess) -> u8 {
        Intstat2Smiaccess::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat2Sspdet {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat2Sspdet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat2Sspdet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat2Sspdet {
    #[inline(always)]
    fn from(val: u8) -> Intstat2Sspdet {
        Intstat2Sspdet::from_bits(val)
    }
}
impl From<Intstat2Sspdet> for u8 {
    #[inline(always)]
    fn from(val: Intstat2Sspdet) -> u8 {
        Intstat2Sspdet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat2Wupdone {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat2Wupdone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat2Wupdone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat2Wupdone {
    #[inline(always)]
    fn from(val: u8) -> Intstat2Wupdone {
        Intstat2Wupdone::from_bits(val)
    }
}
impl From<Intstat2Wupdone> for u8 {
    #[inline(always)]
    fn from(val: Intstat2Wupdone) -> u8 {
        Intstat2Wupdone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intstat2Wutdet {
    #[doc = "Not Pending"]
    NP = 0x0,
    #[doc = "Pending"]
    PEN = 0x01,
}
impl Intstat2Wutdet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intstat2Wutdet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intstat2Wutdet {
    #[inline(always)]
    fn from(val: u8) -> Intstat2Wutdet {
        Intstat2Wutdet::from_bits(val)
    }
}
impl From<Intstat2Wutdet> for u8 {
    #[inline(always)]
    fn from(val: Intstat2Wutdet) -> u8 {
        Intstat2Wutdet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Jab {
    #[doc = "No"]
    JABNO = 0x0,
    #[doc = "Yes"]
    JABYES = 0x01,
}
impl Jab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Jab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Jab {
    #[inline(always)]
    fn from(val: u8) -> Jab {
        Jab::from_bits(val)
    }
}
impl From<Jab> for u8 {
    #[inline(always)]
    fn from(val: Jab) -> u8 {
        Jab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lclwkena {
    #[doc = "Disable"]
    SSDIS = 0x0,
    #[doc = "Enable"]
    SSEN = 0x01,
}
impl Lclwkena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lclwkena {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lclwkena {
    #[inline(always)]
    fn from(val: u8) -> Lclwkena {
        Lclwkena::from_bits(val)
    }
}
impl From<Lclwkena> for u8 {
    #[inline(always)]
    fn from(val: Lclwkena) -> u8 {
        Lclwkena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lclwkwup {
    #[doc = "Disabled"]
    WUPT_DIS = 0x0,
    #[doc = "Enabled"]
    WUPT_EN = 0x01,
}
impl Lclwkwup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lclwkwup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lclwkwup {
    #[inline(always)]
    fn from(val: u8) -> Lclwkwup {
        Lclwkwup::from_bits(val)
    }
}
impl From<Lclwkwup> for u8 {
    #[inline(always)]
    fn from(val: Lclwkwup) -> u8 {
        Lclwkwup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ModestatStat {
    #[doc = "PORST"]
    PORST = 0x0,
    #[doc = "WAITINIT"]
    WAIT = 0x01,
    #[doc = "Linkdown"]
    LINK = 0x02,
    #[doc = "Linkup"]
    LINKUP = 0x03,
    #[doc = "WAITCMDLP"]
    WAITCM = 0x04,
    #[doc = "Low-power"]
    LOW = 0x05,
    #[doc = "WAITCMDCFG"]
    WAI = 0x06,
    #[doc = "TXCCFG"]
    TXC = 0x07,
    #[doc = "TXCBIST"]
    TXCB = 0x08,
    #[doc = "WAITSSILENTLP"]
    WAITS = 0x09,
    #[doc = "WAITSILENTCFG"]
    WAITSI = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl ModestatStat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ModestatStat {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ModestatStat {
    #[inline(always)]
    fn from(val: u8) -> ModestatStat {
        ModestatStat::from_bits(val)
    }
}
impl From<ModestatStat> for u8 {
    #[inline(always)]
    fn from(val: ModestatStat) -> u8 {
        ModestatStat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mskplcadiag1Bcnbfto {
    #[doc = "Enabled"]
    EN2 = 0x0,
    #[doc = "Masked"]
    MASKED2 = 0x01,
}
impl Mskplcadiag1Bcnbfto {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mskplcadiag1Bcnbfto {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mskplcadiag1Bcnbfto {
    #[inline(always)]
    fn from(val: u8) -> Mskplcadiag1Bcnbfto {
        Mskplcadiag1Bcnbfto::from_bits(val)
    }
}
impl From<Mskplcadiag1Bcnbfto> for u8 {
    #[inline(always)]
    fn from(val: Mskplcadiag1Bcnbfto) -> u8 {
        Mskplcadiag1Bcnbfto::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mskplcadiag1Rxinto {
    #[doc = "Enabled"]
    EN = 0x0,
    #[doc = "Masked"]
    MASKED = 0x01,
}
impl Mskplcadiag1Rxinto {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mskplcadiag1Rxinto {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mskplcadiag1Rxinto {
    #[inline(always)]
    fn from(val: u8) -> Mskplcadiag1Rxinto {
        Mskplcadiag1Rxinto::from_bits(val)
    }
}
impl From<Mskplcadiag1Rxinto> for u8 {
    #[inline(always)]
    fn from(val: Mskplcadiag1Rxinto) -> u8 {
        Mskplcadiag1Rxinto::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mskplcadiag1Unexpb {
    #[doc = "Enabled"]
    EN2 = 0x0,
    #[doc = "Masked"]
    MASKED1 = 0x01,
}
impl Mskplcadiag1Unexpb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mskplcadiag1Unexpb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mskplcadiag1Unexpb {
    #[inline(always)]
    fn from(val: u8) -> Mskplcadiag1Unexpb {
        Mskplcadiag1Unexpb::from_bits(val)
    }
}
impl From<Mskplcadiag1Unexpb> for u8 {
    #[inline(always)]
    fn from(val: Mskplcadiag1Unexpb) -> u8 {
        Mskplcadiag1Unexpb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mskplcadiag2Earlybcn {
    #[doc = "Enabled"]
    EN7 = 0x0,
    #[doc = "Masked"]
    MASKED7 = 0x01,
}
impl Mskplcadiag2Earlybcn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mskplcadiag2Earlybcn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mskplcadiag2Earlybcn {
    #[inline(always)]
    fn from(val: u8) -> Mskplcadiag2Earlybcn {
        Mskplcadiag2Earlybcn::from_bits(val)
    }
}
impl From<Mskplcadiag2Earlybcn> for u8 {
    #[inline(always)]
    fn from(val: Mskplcadiag2Earlybcn) -> u8 {
        Mskplcadiag2Earlybcn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mskplcadiag2Latebcn {
    #[doc = "Enabled"]
    EN6 = 0x0,
    #[doc = "Masked"]
    MASKED6 = 0x01,
}
impl Mskplcadiag2Latebcn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mskplcadiag2Latebcn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mskplcadiag2Latebcn {
    #[inline(always)]
    fn from(val: u8) -> Mskplcadiag2Latebcn {
        Mskplcadiag2Latebcn::from_bits(val)
    }
}
impl From<Mskplcadiag2Latebcn> for u8 {
    #[inline(always)]
    fn from(val: Mskplcadiag2Latebcn) -> u8 {
        Mskplcadiag2Latebcn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mskplcadiag2Norxbcn {
    #[doc = "Enabled"]
    EN5 = 0x0,
    #[doc = "Masked"]
    MASKED5 = 0x01,
}
impl Mskplcadiag2Norxbcn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mskplcadiag2Norxbcn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mskplcadiag2Norxbcn {
    #[inline(always)]
    fn from(val: u8) -> Mskplcadiag2Norxbcn {
        Mskplcadiag2Norxbcn::from_bits(val)
    }
}
impl From<Mskplcadiag2Norxbcn> for u8 {
    #[inline(always)]
    fn from(val: Mskplcadiag2Norxbcn) -> u8 {
        Mskplcadiag2Norxbcn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mskplcadiag2Undefstate {
    #[doc = "Enabled"]
    EN2 = 0x0,
    #[doc = "Masked"]
    MASKED2 = 0x01,
}
impl Mskplcadiag2Undefstate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mskplcadiag2Undefstate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mskplcadiag2Undefstate {
    #[inline(always)]
    fn from(val: u8) -> Mskplcadiag2Undefstate {
        Mskplcadiag2Undefstate::from_bits(val)
    }
}
impl From<Mskplcadiag2Undefstate> for u8 {
    #[inline(always)]
    fn from(val: Mskplcadiag2Undefstate) -> u8 {
        Mskplcadiag2Undefstate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MsktxcdiagEdhghfail {
    #[doc = "Enabled"]
    EN10 = 0x0,
    #[doc = "Masked"]
    MASKED10 = 0x01,
}
impl MsktxcdiagEdhghfail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MsktxcdiagEdhghfail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MsktxcdiagEdhghfail {
    #[inline(always)]
    fn from(val: u8) -> MsktxcdiagEdhghfail {
        MsktxcdiagEdhghfail::from_bits(val)
    }
}
impl From<MsktxcdiagEdhghfail> for u8 {
    #[inline(always)]
    fn from(val: MsktxcdiagEdhghfail) -> u8 {
        MsktxcdiagEdhghfail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MsktxcdiagEdlowfail {
    #[doc = "Enabled"]
    EN11 = 0x0,
    #[doc = "Masked"]
    MASKED11 = 0x01,
}
impl MsktxcdiagEdlowfail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MsktxcdiagEdlowfail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MsktxcdiagEdlowfail {
    #[inline(always)]
    fn from(val: u8) -> MsktxcdiagEdlowfail {
        MsktxcdiagEdlowfail::from_bits(val)
    }
}
impl From<MsktxcdiagEdlowfail> for u8 {
    #[inline(always)]
    fn from(val: MsktxcdiagEdlowfail) -> u8 {
        MsktxcdiagEdlowfail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MsktxcdiagLpwrfail {
    #[doc = "Enabled"]
    EN9 = 0x0,
    #[doc = "Masked"]
    MASKED9 = 0x01,
}
impl MsktxcdiagLpwrfail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MsktxcdiagLpwrfail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MsktxcdiagLpwrfail {
    #[inline(always)]
    fn from(val: u8) -> MsktxcdiagLpwrfail {
        MsktxcdiagLpwrfail::from_bits(val)
    }
}
impl From<MsktxcdiagLpwrfail> for u8 {
    #[inline(always)]
    fn from(val: MsktxcdiagLpwrfail) -> u8 {
        MsktxcdiagLpwrfail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MsktxcdiagRxhghfail {
    #[doc = "Enabled"]
    EN12 = 0x0,
    #[doc = "Masked"]
    MASKED12 = 0x01,
}
impl MsktxcdiagRxhghfail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MsktxcdiagRxhghfail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MsktxcdiagRxhghfail {
    #[inline(always)]
    fn from(val: u8) -> MsktxcdiagRxhghfail {
        MsktxcdiagRxhghfail::from_bits(val)
    }
}
impl From<MsktxcdiagRxhghfail> for u8 {
    #[inline(always)]
    fn from(val: MsktxcdiagRxhghfail) -> u8 {
        MsktxcdiagRxhghfail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MsktxcdiagRxlowfail {
    #[doc = "Enabled"]
    EN13 = 0x0,
    #[doc = "Masked"]
    MASKED13 = 0x01,
}
impl MsktxcdiagRxlowfail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MsktxcdiagRxlowfail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MsktxcdiagRxlowfail {
    #[inline(always)]
    fn from(val: u8) -> MsktxcdiagRxlowfail {
        MsktxcdiagRxlowfail::from_bits(val)
    }
}
impl From<MsktxcdiagRxlowfail> for u8 {
    #[inline(always)]
    fn from(val: MsktxcdiagRxlowfail) -> u8 {
        MsktxcdiagRxlowfail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Op {
    #[doc = "WRITE_COMPERR"]
    WRITE_COMPERR = 0x0,
    #[doc = "WRITE"]
    WRITE = 0x01,
    #[doc = "READ"]
    READ = 0x02,
    #[doc = "READ_COMPERR"]
    READ_COMPERR = 0x03,
}
impl Op {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Op {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Op {
    #[inline(always)]
    fn from(val: u8) -> Op {
        Op::from_bits(val)
    }
}
impl From<Op> for u8 {
    #[inline(always)]
    fn from(val: Op) -> u8 {
        Op::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pattern {
    #[doc = "NONE"]
    NONE = 0x0,
    #[doc = "TM1"]
    TM1 = 0x01,
    #[doc = "TM2"]
    TM2 = 0x02,
    #[doc = "TM3"]
    TM3 = 0x03,
    #[doc = "TM4"]
    TM4 = 0x04,
    #[doc = "RES0"]
    RES0 = 0x05,
    #[doc = "RES1"]
    RES1 = 0x06,
    #[doc = "WUT"]
    WUT = 0x07,
}
impl Pattern {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pattern {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pattern {
    #[inline(always)]
    fn from(val: u8) -> Pattern {
        Pattern::from_bits(val)
    }
}
impl From<Pattern> for u8 {
    #[inline(always)]
    fn from(val: Pattern) -> u8 {
        Pattern::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcsctrlLoop {
    #[doc = "Disabled"]
    LD = 0x0,
    #[doc = "Enabled"]
    LE = 0x01,
}
impl PcsctrlLoop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcsctrlLoop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcsctrlLoop {
    #[inline(always)]
    fn from(val: u8) -> PcsctrlLoop {
        PcsctrlLoop::from_bits(val)
    }
}
impl From<PcsctrlLoop> for u8 {
    #[inline(always)]
    fn from(val: PcsctrlLoop) -> u8 {
        PcsctrlLoop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Plcadiag1Bcnbfto {
    #[doc = "NO"]
    NO3 = 0x0,
    #[doc = "Yes"]
    YES3 = 0x01,
}
impl Plcadiag1Bcnbfto {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Plcadiag1Bcnbfto {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Plcadiag1Bcnbfto {
    #[inline(always)]
    fn from(val: u8) -> Plcadiag1Bcnbfto {
        Plcadiag1Bcnbfto::from_bits(val)
    }
}
impl From<Plcadiag1Bcnbfto> for u8 {
    #[inline(always)]
    fn from(val: Plcadiag1Bcnbfto) -> u8 {
        Plcadiag1Bcnbfto::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Plcadiag1Unexpb {
    #[doc = "NO"]
    NO4 = 0x0,
    #[doc = "Yes"]
    YES4 = 0x01,
}
impl Plcadiag1Unexpb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Plcadiag1Unexpb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Plcadiag1Unexpb {
    #[inline(always)]
    fn from(val: u8) -> Plcadiag1Unexpb {
        Plcadiag1Unexpb::from_bits(val)
    }
}
impl From<Plcadiag1Unexpb> for u8 {
    #[inline(always)]
    fn from(val: Plcadiag1Unexpb) -> u8 {
        Plcadiag1Unexpb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Plcadiag2Earlybcn {
    #[doc = "NO"]
    NO8 = 0x0,
    #[doc = "Yes"]
    YES8 = 0x01,
}
impl Plcadiag2Earlybcn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Plcadiag2Earlybcn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Plcadiag2Earlybcn {
    #[inline(always)]
    fn from(val: u8) -> Plcadiag2Earlybcn {
        Plcadiag2Earlybcn::from_bits(val)
    }
}
impl From<Plcadiag2Earlybcn> for u8 {
    #[inline(always)]
    fn from(val: Plcadiag2Earlybcn) -> u8 {
        Plcadiag2Earlybcn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Plcadiag2Latebcn {
    #[doc = "NO"]
    NO7 = 0x0,
    #[doc = "Yes"]
    YES7 = 0x01,
}
impl Plcadiag2Latebcn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Plcadiag2Latebcn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Plcadiag2Latebcn {
    #[inline(always)]
    fn from(val: u8) -> Plcadiag2Latebcn {
        Plcadiag2Latebcn::from_bits(val)
    }
}
impl From<Plcadiag2Latebcn> for u8 {
    #[inline(always)]
    fn from(val: Plcadiag2Latebcn) -> u8 {
        Plcadiag2Latebcn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Plcadiag2Norxbcn {
    #[doc = "NO"]
    NO6 = 0x0,
    #[doc = "Yes"]
    YES6 = 0x01,
}
impl Plcadiag2Norxbcn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Plcadiag2Norxbcn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Plcadiag2Norxbcn {
    #[inline(always)]
    fn from(val: u8) -> Plcadiag2Norxbcn {
        Plcadiag2Norxbcn::from_bits(val)
    }
}
impl From<Plcadiag2Norxbcn> for u8 {
    #[inline(always)]
    fn from(val: Plcadiag2Norxbcn) -> u8 {
        Plcadiag2Norxbcn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Plcadiag2Undefstate {
    #[doc = "NO"]
    NO5 = 0x0,
    #[doc = "Yes"]
    YES5 = 0x01,
}
impl Plcadiag2Undefstate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Plcadiag2Undefstate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Plcadiag2Undefstate {
    #[inline(always)]
    fn from(val: u8) -> Plcadiag2Undefstate {
        Plcadiag2Undefstate::from_bits(val)
    }
}
impl From<Plcadiag2Undefstate> for u8 {
    #[inline(always)]
    fn from(val: Plcadiag2Undefstate) -> u8 {
        Plcadiag2Undefstate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PmactrlLoop {
    #[doc = "Disabled"]
    LOOPDIS = 0x0,
    #[doc = "Enabled"]
    LOOPEN = 0x01,
}
impl PmactrlLoop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PmactrlLoop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PmactrlLoop {
    #[inline(always)]
    fn from(val: u8) -> PmactrlLoop {
        PmactrlLoop::from_bits(val)
    }
}
impl From<PmactrlLoop> for u8 {
    #[inline(always)]
    fn from(val: PmactrlLoop) -> u8 {
        PmactrlLoop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ready {
    #[doc = "No"]
    READY = 0x0,
    #[doc = "Yes"]
    READY_NONTC14 = 0x01,
}
impl Ready {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ready {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ready {
    #[inline(always)]
    fn from(val: u8) -> Ready {
        Ready::from_bits(val)
    }
}
impl From<Ready> for u8 {
    #[inline(always)]
    fn from(val: Ready) -> u8 {
        Ready::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Remkwkfdw {
    #[doc = "DISABLED"]
    REM_NO = 0x0,
    #[doc = "ENABLED"]
    REM_YES = 0x01,
}
impl Remkwkfdw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Remkwkfdw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Remkwkfdw {
    #[inline(always)]
    fn from(val: u8) -> Remkwkfdw {
        Remkwkfdw::from_bits(val)
    }
}
impl From<Remkwkfdw> for u8 {
    #[inline(always)]
    fn from(val: Remkwkfdw) -> u8 {
        Remkwkfdw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Remwkena {
    #[doc = "Disable"]
    SSDIS = 0x0,
    #[doc = "Enable"]
    SSEN = 0x01,
}
impl Remwkena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Remwkena {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Remwkena {
    #[inline(always)]
    fn from(val: u8) -> Remwkena {
        Remwkena::from_bits(val)
    }
}
impl From<Remwkena> for u8 {
    #[inline(always)]
    fn from(val: Remwkena) -> u8 {
        Remwkena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Remwkfdw {
    #[doc = "NO_EFFECT"]
    REM_NO = 0x0,
    #[doc = "Trigger generation of a wake-forward pulse"]
    REM_YES = 0x01,
}
impl Remwkfdw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Remwkfdw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Remwkfdw {
    #[inline(always)]
    fn from(val: u8) -> Remwkfdw {
        Remwkfdw::from_bits(val)
    }
}
impl From<Remwkfdw> for u8 {
    #[inline(always)]
    fn from(val: Remwkfdw) -> u8 {
        Remwkfdw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rst {
    #[doc = "STAT_NO_RESET"]
    STAT_NO_RESET = 0x0,
    #[doc = "STAT_RESET"]
    STAT_RESET = 0x01,
}
impl Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rst {
    #[inline(always)]
    fn from(val: u8) -> Rst {
        Rst::from_bits(val)
    }
}
impl From<Rst> for u8 {
    #[inline(always)]
    fn from(val: Rst) -> u8 {
        Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Silentto {
    #[doc = "No"]
    SSDIS = 0x0,
    #[doc = "Yes"]
    SSEN = 0x01,
}
impl Silentto {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Silentto {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Silentto {
    #[inline(always)]
    fn from(val: u8) -> Silentto {
        Silentto::from_bits(val)
    }
}
impl From<Silentto> for u8 {
    #[inline(always)]
    fn from(val: Silentto) -> u8 {
        Silentto::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmistatErrcode {
    #[doc = "NONE"]
    NONE = 0x0,
    #[doc = "NONTC14"]
    NONTC14 = 0x01,
    #[doc = "Disabled"]
    E_DISABLED = 0x02,
    #[doc = "ABORTED"]
    EHOLD_2 = 0x03,
    #[doc = "INPRGS"]
    HOLD_2 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SmistatErrcode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmistatErrcode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmistatErrcode {
    #[inline(always)]
    fn from(val: u8) -> SmistatErrcode {
        SmistatErrcode::from_bits(val)
    }
}
impl From<SmistatErrcode> for u8 {
    #[inline(always)]
    fn from(val: SmistatErrcode) -> u8 {
        SmistatErrcode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sspwkena {
    #[doc = "Disable"]
    SSDIS = 0x0,
    #[doc = "Enable"]
    SSEN = 0x01,
}
impl Sspwkena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sspwkena {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sspwkena {
    #[inline(always)]
    fn from(val: u8) -> Sspwkena {
        Sspwkena::from_bits(val)
    }
}
impl From<Sspwkena> for u8 {
    #[inline(always)]
    fn from(val: Sspwkena) -> u8 {
        Sspwkena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxcdiagEdhghfail {
    #[doc = "NO"]
    NO11 = 0x0,
    #[doc = "Yes"]
    YES11 = 0x01,
}
impl TxcdiagEdhghfail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxcdiagEdhghfail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxcdiagEdhghfail {
    #[inline(always)]
    fn from(val: u8) -> TxcdiagEdhghfail {
        TxcdiagEdhghfail::from_bits(val)
    }
}
impl From<TxcdiagEdhghfail> for u8 {
    #[inline(always)]
    fn from(val: TxcdiagEdhghfail) -> u8 {
        TxcdiagEdhghfail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxcdiagEdlowfail {
    #[doc = "NO"]
    NO8 = 0x0,
    #[doc = "Yes"]
    YES8 = 0x01,
}
impl TxcdiagEdlowfail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxcdiagEdlowfail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxcdiagEdlowfail {
    #[inline(always)]
    fn from(val: u8) -> TxcdiagEdlowfail {
        TxcdiagEdlowfail::from_bits(val)
    }
}
impl From<TxcdiagEdlowfail> for u8 {
    #[inline(always)]
    fn from(val: TxcdiagEdlowfail) -> u8 {
        TxcdiagEdlowfail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxcdiagLpwrfail {
    #[doc = "NO"]
    NO10 = 0x0,
    #[doc = "Yes"]
    YES10 = 0x01,
}
impl TxcdiagLpwrfail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxcdiagLpwrfail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxcdiagLpwrfail {
    #[inline(always)]
    fn from(val: u8) -> TxcdiagLpwrfail {
        TxcdiagLpwrfail::from_bits(val)
    }
}
impl From<TxcdiagLpwrfail> for u8 {
    #[inline(always)]
    fn from(val: TxcdiagLpwrfail) -> u8 {
        TxcdiagLpwrfail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxcdiagRxhghfail {
    #[doc = "NO"]
    NO11 = 0x0,
    #[doc = "Yes"]
    YES11 = 0x01,
}
impl TxcdiagRxhghfail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxcdiagRxhghfail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxcdiagRxhghfail {
    #[inline(always)]
    fn from(val: u8) -> TxcdiagRxhghfail {
        TxcdiagRxhghfail::from_bits(val)
    }
}
impl From<TxcdiagRxhghfail> for u8 {
    #[inline(always)]
    fn from(val: TxcdiagRxhghfail) -> u8 {
        TxcdiagRxhghfail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxcdiagRxlowfail {
    #[doc = "NO"]
    NO12 = 0x0,
    #[doc = "Yes"]
    YES12 = 0x01,
}
impl TxcdiagRxlowfail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxcdiagRxlowfail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxcdiagRxlowfail {
    #[inline(always)]
    fn from(val: u8) -> TxcdiagRxlowfail {
        TxcdiagRxlowfail::from_bits(val)
    }
}
impl From<TxcdiagRxlowfail> for u8 {
    #[inline(always)]
    fn from(val: TxcdiagRxlowfail) -> u8 {
        TxcdiagRxlowfail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txdis {
    #[doc = "Disabled"]
    TXDIS = 0x0,
    #[doc = "Enabled"]
    TXEN = 0x01,
}
impl Txdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txdis {
    #[inline(always)]
    fn from(val: u8) -> Txdis {
        Txdis::from_bits(val)
    }
}
impl From<Txdis> for u8 {
    #[inline(always)]
    fn from(val: Txdis) -> u8 {
        Txdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Type {
    #[doc = "CBT1"]
    CBT1 = 0x0,
    #[doc = "MBT1"]
    MBT1 = 0x01,
    #[doc = "XBT1L"]
    XBT1L = 0x02,
    #[doc = "XBT1S"]
    XBT1S = 0x03,
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
impl Type {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Type {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Type {
    #[inline(always)]
    fn from(val: u8) -> Type {
        Type::from_bits(val)
    }
}
impl From<Type> for u8 {
    #[inline(always)]
    fn from(val: Type) -> u8 {
        Type::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupcmd {
    #[doc = "NO_EFFECT"]
    WUP_N = 0x0,
    #[doc = "Trigger WUP-sequencer"]
    WUP_TRIG = 0x01,
}
impl Wupcmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupcmd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupcmd {
    #[inline(always)]
    fn from(val: u8) -> Wupcmd {
        Wupcmd::from_bits(val)
    }
}
impl From<Wupcmd> for u8 {
    #[inline(always)]
    fn from(val: Wupcmd) -> u8 {
        Wupcmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WupstatErrcode {
    #[doc = "NONE"]
    NONE = 0x0,
    #[doc = "SWABORT"]
    SWABORT = 0x01,
    #[doc = "MODECHG"]
    MODECHG = 0x02,
    #[doc = "TOWM"]
    TOWM = 0x03,
    #[doc = "TOPD"]
    TOPD = 0x04,
    #[doc = "TOWP"]
    TOWP = 0x05,
    #[doc = "TOWA"]
    TOWA = 0x06,
    #[doc = "CORE"]
    CORE = 0x07,
}
impl WupstatErrcode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WupstatErrcode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WupstatErrcode {
    #[inline(always)]
    fn from(val: u8) -> WupstatErrcode {
        WupstatErrcode::from_bits(val)
    }
}
impl From<WupstatErrcode> for u8 {
    #[inline(always)]
    fn from(val: WupstatErrcode) -> u8 {
        WupstatErrcode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WupstatStat {
    #[doc = "IDLE"]
    IDLE = 0x0,
    #[doc = "WAITLINK"]
    HOLD_2 = 0x01,
    #[doc = "PENDING"]
    STPEND = 0x02,
    _RESERVED_3 = 0x03,
}
impl WupstatStat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WupstatStat {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WupstatStat {
    #[inline(always)]
    fn from(val: u8) -> WupstatStat {
        WupstatStat::from_bits(val)
    }
}
impl From<WupstatStat> for u8 {
    #[inline(always)]
    fn from(val: WupstatStat) -> u8 {
        WupstatStat::to_bits(val)
    }
}
