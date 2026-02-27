#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aai {
    #[doc = "Interrupt did not occur"]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred"]
    INT_YES = 0x01,
}
impl Aai {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aai {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aai {
    #[inline(always)]
    fn from(val: u8) -> Aai {
        Aai::from_bits(val)
    }
}
impl From<Aai> for u8 {
    #[inline(always)]
    fn from(val: Aai) -> u8 {
        Aai::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbbrst {
    #[doc = "Incremental burst of unspecified length only"]
    INCR_BURST = 0x0,
    #[doc = "INCR4 burst, then single transfer"]
    INCR4_BURST = 0x01,
    #[doc = "INCR8 burst, INCR4 burst, then single transfer"]
    INCR8_BURST = 0x02,
    #[doc = "INCR16 burst, INCR8 burst, INCR4 burst, then single transfer"]
    INCR16_BURST = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "INCR4 burst, then incremental burst of unspecified length"]
    INCR4_UNSPEC = 0x05,
    #[doc = "INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
    INCR8_4_UNSPEC = 0x06,
    #[doc = "INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
    INCR16_8_4_UNSPEC = 0x07,
}
impl Ahbbrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbbrst {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbbrst {
    #[inline(always)]
    fn from(val: u8) -> Ahbbrst {
        Ahbbrst::from_bits(val)
    }
}
impl From<Ahbbrst> for u8 {
    #[inline(always)]
    fn from(val: Ahbbrst) -> u8 {
        Ahbbrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ase {
    #[doc = "Do not process the asynchronous schedule"]
    DONT_PROCESS_ASYNC = 0x0,
    #[doc = "Access the asynchronous schedule"]
    ACCESS_ASYNC = 0x01,
}
impl Ase {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ase {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ase {
    #[inline(always)]
    fn from(val: u8) -> Ase {
        Ase::from_bits(val)
    }
}
impl From<Ase> for u8 {
    #[inline(always)]
    fn from(val: Ase) -> u8 {
        Ase::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ccs {
    #[doc = "No device present or attached"]
    DEVICE_UNAVAILABLE = 0x0,
    #[doc = "Device present and attached"]
    DEVICE_AVAILABLE = 0x01,
}
impl Ccs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ccs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ccs {
    #[inline(always)]
    fn from(val: u8) -> Ccs {
        Ccs::from_bits(val)
    }
}
impl From<Ccs> for u8 {
    #[inline(always)]
    fn from(val: Ccs) -> u8 {
        Ccs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cf {
    #[doc = "Port routing to classic host controller"]
    PORT_ROUTING_CLASSIC_HOST = 0x0,
    #[doc = "Port routing to this host controller"]
    PORT_ROUTING_HOST = 0x01,
}
impl Cf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cf {
    #[inline(always)]
    fn from(val: u8) -> Cf {
        Cf::from_bits(val)
    }
}
impl From<Cf> for u8 {
    #[inline(always)]
    fn from(val: Cf) -> u8 {
        Cf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cm {
    #[doc = "Idle (default for host and device combination)"]
    IDL = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Device controller (default for device-only controller)"]
    DEVICE_CONTR = 0x02,
    #[doc = "Host controller (default for host-only controller)"]
    HOST_CONTR = 0x03,
}
impl Cm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cm {
    #[inline(always)]
    fn from(val: u8) -> Cm {
        Cm::from_bits(val)
    }
}
impl From<Cm> for u8 {
    #[inline(always)]
    fn from(val: Cm) -> u8 {
        Cm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Den {
    #[doc = "Not device capable"]
    NOT_DEN = 0x0,
    #[doc = "Device capable"]
    DEN = 0x01,
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
impl Den {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Den {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Den {
    #[inline(always)]
    fn from(val: u8) -> Den {
        Den::from_bits(val)
    }
}
impl From<Den> for u8 {
    #[inline(always)]
    fn from(val: Den) -> u8 {
        Den::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl1Rxi {
    #[doc = "Allow"]
    ENABLE = 0x0,
    #[doc = "Inhibit"]
    INHIBIT = 0x01,
}
impl Endptctrl1Rxi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl1Rxi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl1Rxi {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl1Rxi {
        Endptctrl1Rxi::from_bits(val)
    }
}
impl From<Endptctrl1Rxi> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl1Rxi) -> u8 {
        Endptctrl1Rxi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl1Rxt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl1Rxt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl1Rxt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl1Rxt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl1Rxt {
        Endptctrl1Rxt::from_bits(val)
    }
}
impl From<Endptctrl1Rxt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl1Rxt) -> u8 {
        Endptctrl1Rxt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl1Txi {
    #[doc = "Allow"]
    ENABLE = 0x0,
    #[doc = "Inhibit"]
    INHIBIT = 0x01,
}
impl Endptctrl1Txi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl1Txi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl1Txi {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl1Txi {
        Endptctrl1Txi::from_bits(val)
    }
}
impl From<Endptctrl1Txi> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl1Txi) -> u8 {
        Endptctrl1Txi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl1Txt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl1Txt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl1Txt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl1Txt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl1Txt {
        Endptctrl1Txt::from_bits(val)
    }
}
impl From<Endptctrl1Txt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl1Txt) -> u8 {
        Endptctrl1Txt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl2Rxi {
    #[doc = "Allow"]
    ENABLE = 0x0,
    #[doc = "Inhibit"]
    INHIBIT = 0x01,
}
impl Endptctrl2Rxi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl2Rxi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl2Rxi {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl2Rxi {
        Endptctrl2Rxi::from_bits(val)
    }
}
impl From<Endptctrl2Rxi> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl2Rxi) -> u8 {
        Endptctrl2Rxi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl2Rxt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl2Rxt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl2Rxt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl2Rxt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl2Rxt {
        Endptctrl2Rxt::from_bits(val)
    }
}
impl From<Endptctrl2Rxt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl2Rxt) -> u8 {
        Endptctrl2Rxt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl2Txi {
    #[doc = "Allow"]
    ENABLE = 0x0,
    #[doc = "Inhibit"]
    INHIBIT = 0x01,
}
impl Endptctrl2Txi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl2Txi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl2Txi {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl2Txi {
        Endptctrl2Txi::from_bits(val)
    }
}
impl From<Endptctrl2Txi> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl2Txi) -> u8 {
        Endptctrl2Txi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl2Txt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl2Txt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl2Txt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl2Txt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl2Txt {
        Endptctrl2Txt::from_bits(val)
    }
}
impl From<Endptctrl2Txt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl2Txt) -> u8 {
        Endptctrl2Txt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl3Rxi {
    #[doc = "Allow"]
    ENABLE = 0x0,
    #[doc = "Inhibit"]
    INHIBIT = 0x01,
}
impl Endptctrl3Rxi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl3Rxi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl3Rxi {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl3Rxi {
        Endptctrl3Rxi::from_bits(val)
    }
}
impl From<Endptctrl3Rxi> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl3Rxi) -> u8 {
        Endptctrl3Rxi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl3Rxt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl3Rxt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl3Rxt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl3Rxt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl3Rxt {
        Endptctrl3Rxt::from_bits(val)
    }
}
impl From<Endptctrl3Rxt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl3Rxt) -> u8 {
        Endptctrl3Rxt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl3Txi {
    #[doc = "Allow"]
    ENABLE = 0x0,
    #[doc = "Inhibit"]
    INHIBIT = 0x01,
}
impl Endptctrl3Txi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl3Txi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl3Txi {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl3Txi {
        Endptctrl3Txi::from_bits(val)
    }
}
impl From<Endptctrl3Txi> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl3Txi) -> u8 {
        Endptctrl3Txi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl3Txt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl3Txt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl3Txt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl3Txt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl3Txt {
        Endptctrl3Txt::from_bits(val)
    }
}
impl From<Endptctrl3Txt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl3Txt) -> u8 {
        Endptctrl3Txt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl4Rxt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl4Rxt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl4Rxt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl4Rxt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl4Rxt {
        Endptctrl4Rxt::from_bits(val)
    }
}
impl From<Endptctrl4Rxt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl4Rxt) -> u8 {
        Endptctrl4Rxt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl4Txi {
    #[doc = "Allow"]
    ENABLE = 0x0,
    #[doc = "Inhibit"]
    INHIBIT = 0x01,
}
impl Endptctrl4Txi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl4Txi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl4Txi {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl4Txi {
        Endptctrl4Txi::from_bits(val)
    }
}
impl From<Endptctrl4Txi> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl4Txi) -> u8 {
        Endptctrl4Txi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl4Txt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl4Txt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl4Txt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl4Txt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl4Txt {
        Endptctrl4Txt::from_bits(val)
    }
}
impl From<Endptctrl4Txt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl4Txt) -> u8 {
        Endptctrl4Txt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl5Rxi {
    #[doc = "Allow"]
    ENABLE = 0x0,
    #[doc = "Inhibit"]
    INHIBIT = 0x01,
}
impl Endptctrl5Rxi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl5Rxi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl5Rxi {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl5Rxi {
        Endptctrl5Rxi::from_bits(val)
    }
}
impl From<Endptctrl5Rxi> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl5Rxi) -> u8 {
        Endptctrl5Rxi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl5Rxt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl5Rxt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl5Rxt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl5Rxt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl5Rxt {
        Endptctrl5Rxt::from_bits(val)
    }
}
impl From<Endptctrl5Rxt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl5Rxt) -> u8 {
        Endptctrl5Rxt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl5Txi {
    #[doc = "Allow"]
    ENABLE = 0x0,
    #[doc = "Inhibit"]
    INHIBIT = 0x01,
}
impl Endptctrl5Txi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl5Txi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl5Txi {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl5Txi {
        Endptctrl5Txi::from_bits(val)
    }
}
impl From<Endptctrl5Txi> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl5Txi) -> u8 {
        Endptctrl5Txi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl5Txt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl5Txt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl5Txt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl5Txt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl5Txt {
        Endptctrl5Txt::from_bits(val)
    }
}
impl From<Endptctrl5Txt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl5Txt) -> u8 {
        Endptctrl5Txt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl6Rxi {
    #[doc = "Allow"]
    ENABLE = 0x0,
    #[doc = "Inhibit"]
    INHIBIT = 0x01,
}
impl Endptctrl6Rxi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl6Rxi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl6Rxi {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl6Rxi {
        Endptctrl6Rxi::from_bits(val)
    }
}
impl From<Endptctrl6Rxi> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl6Rxi) -> u8 {
        Endptctrl6Rxi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl6Rxt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl6Rxt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl6Rxt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl6Rxt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl6Rxt {
        Endptctrl6Rxt::from_bits(val)
    }
}
impl From<Endptctrl6Rxt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl6Rxt) -> u8 {
        Endptctrl6Rxt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl6Txi {
    #[doc = "Allow"]
    ENABLE = 0x0,
    #[doc = "Inhibit"]
    INHIBIT = 0x01,
}
impl Endptctrl6Txi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl6Txi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl6Txi {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl6Txi {
        Endptctrl6Txi::from_bits(val)
    }
}
impl From<Endptctrl6Txi> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl6Txi) -> u8 {
        Endptctrl6Txi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl6Txt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl6Txt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl6Txt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl6Txt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl6Txt {
        Endptctrl6Txt::from_bits(val)
    }
}
impl From<Endptctrl6Txt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl6Txt) -> u8 {
        Endptctrl6Txt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl7Rxi {
    #[doc = "Allow"]
    ENABLE = 0x0,
    #[doc = "Inhibit"]
    INHIBIT = 0x01,
}
impl Endptctrl7Rxi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl7Rxi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl7Rxi {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl7Rxi {
        Endptctrl7Rxi::from_bits(val)
    }
}
impl From<Endptctrl7Rxi> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl7Rxi) -> u8 {
        Endptctrl7Rxi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl7Rxt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl7Rxt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl7Rxt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl7Rxt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl7Rxt {
        Endptctrl7Rxt::from_bits(val)
    }
}
impl From<Endptctrl7Rxt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl7Rxt) -> u8 {
        Endptctrl7Rxt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl7Txi {
    #[doc = "Allow"]
    ENABLE = 0x0,
    #[doc = "Inhibit"]
    INHIBIT = 0x01,
}
impl Endptctrl7Txi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl7Txi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl7Txi {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl7Txi {
        Endptctrl7Txi::from_bits(val)
    }
}
impl From<Endptctrl7Txi> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl7Txi) -> u8 {
        Endptctrl7Txi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl7Txt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl7Txt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl7Txt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl7Txt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl7Txt {
        Endptctrl7Txt::from_bits(val)
    }
}
impl From<Endptctrl7Txt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl7Txt) -> u8 {
        Endptctrl7Txt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Endptsetupstat(u16);
impl Endptsetupstat {
    #[doc = "Not received"]
    pub const NOTREC: Self = Self(0x0);
    #[doc = "Received"]
    pub const RECVD: Self = Self(0x01);
}
impl Endptsetupstat {
    pub const fn from_bits(val: u16) -> Endptsetupstat {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Endptsetupstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NOTREC"),
            0x01 => f.write_str("RECVD"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Endptsetupstat {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NOTREC"),
            0x01 => defmt::write!(f, "RECVD"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Endptsetupstat {
    #[inline(always)]
    fn from(val: u16) -> Endptsetupstat {
        Endptsetupstat::from_bits(val)
    }
}
impl From<Endptsetupstat> for u16 {
    #[inline(always)]
    fn from(val: Endptsetupstat) -> u16 {
        Endptsetupstat::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Eprn(u8);
impl Eprn {
    #[doc = "No NACK"]
    pub const NONACK: Self = Self(0x0);
    #[doc = "NACK"]
    pub const NACK: Self = Self(0x01);
}
impl Eprn {
    pub const fn from_bits(val: u8) -> Eprn {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Eprn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONACK"),
            0x01 => f.write_str("NACK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eprn {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONACK"),
            0x01 => defmt::write!(f, "NACK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Eprn {
    #[inline(always)]
    fn from(val: u8) -> Eprn {
        Eprn::from_bits(val)
    }
}
impl From<Eprn> for u8 {
    #[inline(always)]
    fn from(val: Eprn) -> u8 {
        Eprn::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Eptn(u8);
impl Eptn {
    #[doc = "No NACK"]
    pub const NONACK: Self = Self(0x0);
    #[doc = "NACK"]
    pub const NACK: Self = Self(0x01);
}
impl Eptn {
    pub const fn from_bits(val: u8) -> Eptn {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Eptn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONACK"),
            0x01 => f.write_str("NACK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eptn {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONACK"),
            0x01 => defmt::write!(f, "NACK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Eptn {
    #[inline(always)]
    fn from(val: u8) -> Eptn {
        Eptn::from_bits(val)
    }
}
impl From<Eptn> for u8 {
    #[inline(always)]
    fn from(val: Eptn) -> u8 {
        Eptn::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Erce(u8);
impl Erce {
    #[doc = "Transmit incomplete"]
    pub const TRANSNOTCOMP: Self = Self(0x0);
    #[doc = "Transmit complete"]
    pub const TRANSCOMP: Self = Self(0x01);
}
impl Erce {
    pub const fn from_bits(val: u8) -> Erce {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Erce {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("TRANSNOTCOMP"),
            0x01 => f.write_str("TRANSCOMP"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Erce {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "TRANSNOTCOMP"),
            0x01 => defmt::write!(f, "TRANSCOMP"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Erce {
    #[inline(always)]
    fn from(val: u8) -> Erce {
        Erce::from_bits(val)
    }
}
impl From<Erce> for u8 {
    #[inline(always)]
    fn from(val: Erce) -> u8 {
        Erce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Es {
    #[doc = "Little endian (default)"]
    LITTLE_ENDIAN = 0x0,
    #[doc = "Big endian"]
    BIG_ENDIAN = 0x01,
}
impl Es {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Es {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Es {
    #[inline(always)]
    fn from(val: u8) -> Es {
        Es::from_bits(val)
    }
}
impl From<Es> for u8 {
    #[inline(always)]
    fn from(val: Es) -> u8 {
        Es::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Etce(u8);
impl Etce {
    #[doc = "Transmit incomplete"]
    pub const TRANSNOTCOMP: Self = Self(0x0);
    #[doc = "Transmit complete"]
    pub const TRANSCOMP: Self = Self(0x01);
}
impl Etce {
    pub const fn from_bits(val: u8) -> Etce {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Etce {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("TRANSNOTCOMP"),
            0x01 => f.write_str("TRANSCOMP"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etce {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "TRANSNOTCOMP"),
            0x01 => defmt::write!(f, "TRANSCOMP"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Etce {
    #[inline(always)]
    fn from(val: u8) -> Etce {
        Etce::from_bits(val)
    }
}
impl From<Etce> for u8 {
    #[inline(always)]
    fn from(val: Etce) -> u8 {
        Etce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fri {
    #[doc = "Frame list index did not roll over"]
    ROLL_NO = 0x0,
    #[doc = "Frame list index rolled over"]
    ROLL_YES = 0x01,
}
impl Fri {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fri {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fri {
    #[inline(always)]
    fn from(val: u8) -> Fri {
        Fri::from_bits(val)
    }
}
impl From<Fri> for u8 {
    #[inline(always)]
    fn from(val: Fri) -> u8 {
        Fri::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Frindex(u16);
impl Frindex {
    #[doc = "(1024) 12"]
    pub const FRINDEX_1024: Self = Self(0x0);
    #[doc = "(512) 11"]
    pub const FRINDEX_512: Self = Self(0x01);
    #[doc = "(256) 10"]
    pub const FRINDEX_256: Self = Self(0x02);
    #[doc = "(128) 9"]
    pub const FRINDEX_128: Self = Self(0x03);
    #[doc = "(64) 8"]
    pub const FRINDEX_64: Self = Self(0x04);
    #[doc = "(32) 7"]
    pub const FRINDEX_32: Self = Self(0x05);
    #[doc = "(16) 6"]
    pub const FRINDEX_16: Self = Self(0x06);
    #[doc = "(8) 5"]
    pub const FRINDEX_8: Self = Self(0x07);
}
impl Frindex {
    pub const fn from_bits(val: u16) -> Frindex {
        Self(val & 0x3fff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Frindex {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("FRINDEX_1024"),
            0x01 => f.write_str("FRINDEX_512"),
            0x02 => f.write_str("FRINDEX_256"),
            0x03 => f.write_str("FRINDEX_128"),
            0x04 => f.write_str("FRINDEX_64"),
            0x05 => f.write_str("FRINDEX_32"),
            0x06 => f.write_str("FRINDEX_16"),
            0x07 => f.write_str("FRINDEX_8"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frindex {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "FRINDEX_1024"),
            0x01 => defmt::write!(f, "FRINDEX_512"),
            0x02 => defmt::write!(f, "FRINDEX_256"),
            0x03 => defmt::write!(f, "FRINDEX_128"),
            0x04 => defmt::write!(f, "FRINDEX_64"),
            0x05 => defmt::write!(f, "FRINDEX_32"),
            0x06 => defmt::write!(f, "FRINDEX_16"),
            0x07 => defmt::write!(f, "FRINDEX_8"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Frindex {
    #[inline(always)]
    fn from(val: u16) -> Frindex {
        Frindex::from_bits(val)
    }
}
impl From<Frindex> for u16 {
    #[inline(always)]
    fn from(val: Frindex) -> u16 {
        Frindex::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gptimer0ctrlGptmode {
    #[doc = "One Shot mode"]
    ONE_SHOT = 0x0,
    #[doc = "Repeat mode"]
    REPEAT = 0x01,
}
impl Gptimer0ctrlGptmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gptimer0ctrlGptmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gptimer0ctrlGptmode {
    #[inline(always)]
    fn from(val: u8) -> Gptimer0ctrlGptmode {
        Gptimer0ctrlGptmode::from_bits(val)
    }
}
impl From<Gptimer0ctrlGptmode> for u8 {
    #[inline(always)]
    fn from(val: Gptimer0ctrlGptmode) -> u8 {
        Gptimer0ctrlGptmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gptimer0ctrlGptrst {
    #[doc = "No action"]
    NO_ACTION = 0x0,
    #[doc = "Load counter value"]
    LOAD_CNTR = 0x01,
}
impl Gptimer0ctrlGptrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gptimer0ctrlGptrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gptimer0ctrlGptrst {
    #[inline(always)]
    fn from(val: u8) -> Gptimer0ctrlGptrst {
        Gptimer0ctrlGptrst::from_bits(val)
    }
}
impl From<Gptimer0ctrlGptrst> for u8 {
    #[inline(always)]
    fn from(val: Gptimer0ctrlGptrst) -> u8 {
        Gptimer0ctrlGptrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gptimer0ctrlGptrun {
    #[doc = "Stopped counting"]
    STOP_CNTR = 0x0,
    #[doc = "Running"]
    RUN = 0x01,
}
impl Gptimer0ctrlGptrun {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gptimer0ctrlGptrun {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gptimer0ctrlGptrun {
    #[inline(always)]
    fn from(val: u8) -> Gptimer0ctrlGptrun {
        Gptimer0ctrlGptrun::from_bits(val)
    }
}
impl From<Gptimer0ctrlGptrun> for u8 {
    #[inline(always)]
    fn from(val: Gptimer0ctrlGptrun) -> u8 {
        Gptimer0ctrlGptrun::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gptimer1ctrlGptmode {
    #[doc = "One Shot mode"]
    ONE_SHOT = 0x0,
    #[doc = "Repeat mode"]
    REPEAT = 0x01,
}
impl Gptimer1ctrlGptmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gptimer1ctrlGptmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gptimer1ctrlGptmode {
    #[inline(always)]
    fn from(val: u8) -> Gptimer1ctrlGptmode {
        Gptimer1ctrlGptmode::from_bits(val)
    }
}
impl From<Gptimer1ctrlGptmode> for u8 {
    #[inline(always)]
    fn from(val: Gptimer1ctrlGptmode) -> u8 {
        Gptimer1ctrlGptmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gptimer1ctrlGptrst {
    #[doc = "No action"]
    NO_ACTION = 0x0,
    #[doc = "Load counter value"]
    LOAD_CNTR = 0x01,
}
impl Gptimer1ctrlGptrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gptimer1ctrlGptrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gptimer1ctrlGptrst {
    #[inline(always)]
    fn from(val: u8) -> Gptimer1ctrlGptrst {
        Gptimer1ctrlGptrst::from_bits(val)
    }
}
impl From<Gptimer1ctrlGptrst> for u8 {
    #[inline(always)]
    fn from(val: Gptimer1ctrlGptrst) -> u8 {
        Gptimer1ctrlGptrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gptimer1ctrlGptrun {
    #[doc = "Stopped counting"]
    STOP_CNTR = 0x0,
    #[doc = "Running"]
    RUN = 0x01,
}
impl Gptimer1ctrlGptrun {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gptimer1ctrlGptrun {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gptimer1ctrlGptrun {
    #[inline(always)]
    fn from(val: u8) -> Gptimer1ctrlGptrun {
        Gptimer1ctrlGptrun::from_bits(val)
    }
}
impl From<Gptimer1ctrlGptrun> for u8 {
    #[inline(always)]
    fn from(val: Gptimer1ctrlGptrun) -> u8 {
        Gptimer1ctrlGptrun::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Haar {
    #[doc = "Disable"]
    HAAR_0 = 0x0,
    #[doc = "Enable"]
    HAAR_1 = 0x01,
}
impl Haar {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Haar {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Haar {
    #[inline(always)]
    fn from(val: u8) -> Haar {
        Haar::from_bits(val)
    }
}
impl From<Haar> for u8 {
    #[inline(always)]
    fn from(val: Haar) -> u8 {
        Haar::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Haba {
    #[doc = "Disable"]
    HABA_0 = 0x0,
    #[doc = "Enable"]
    HABA_1 = 0x01,
}
impl Haba {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Haba {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Haba {
    #[inline(always)]
    fn from(val: u8) -> Haba {
        Haba::from_bits(val)
    }
}
impl From<Haba> for u8 {
    #[inline(always)]
    fn from(val: Haba) -> u8 {
        Haba::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hadp {
    #[doc = "Disable"]
    HADP_0 = 0x0,
    #[doc = "Enable"]
    HADP_1 = 0x01,
}
impl Hadp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hadp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hadp {
    #[inline(always)]
    fn from(val: u8) -> Hadp {
        Hadp::from_bits(val)
    }
}
impl From<Hadp> for u8 {
    #[inline(always)]
    fn from(val: Hadp) -> u8 {
        Hadp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HwdeviceDc {
    #[doc = "Not supported"]
    DEVICE_OP_DIS = 0x0,
    #[doc = "Supported"]
    DEVICE_OP_EN = 0x01,
}
impl HwdeviceDc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HwdeviceDc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HwdeviceDc {
    #[inline(always)]
    fn from(val: u8) -> HwdeviceDc {
        HwdeviceDc::from_bits(val)
    }
}
impl From<HwdeviceDc> for u8 {
    #[inline(always)]
    fn from(val: HwdeviceDc) -> u8 {
        HwdeviceDc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HwhostHc {
    #[doc = "Not supported"]
    HOST_OP_DIS = 0x0,
    #[doc = "Supported"]
    HOST_OP_EN = 0x01,
}
impl HwhostHc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HwhostHc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HwhostHc {
    #[inline(always)]
    fn from(val: u8) -> HwhostHc {
        HwhostHc::from_bits(val)
    }
}
impl From<HwhostHc> for u8 {
    #[inline(always)]
    fn from(val: HwhostHc) -> u8 {
        HwhostHc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Id {
    #[doc = "A device"]
    DEV_A = 0x0,
    #[doc = "B device"]
    DEV_B = 0x01,
}
impl Id {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Id {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Id {
    #[inline(always)]
    fn from(val: u8) -> Id {
        Id::from_bits(val)
    }
}
impl From<Id> for u8 {
    #[inline(always)]
    fn from(val: Id) -> u8 {
        Id::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Itc(u8);
impl Itc {
    #[doc = "Immediate (no threshold)"]
    pub const IMMEDIATE: Self = Self(0x0);
    #[doc = "1 microframe"]
    pub const MICROFRAME_1: Self = Self(0x01);
    #[doc = "2 microframes"]
    pub const MICROFRAME_2: Self = Self(0x02);
    #[doc = "4 microframes"]
    pub const MICROFRAME_4: Self = Self(0x04);
    #[doc = "8 microframes"]
    pub const MICROFRAME_8: Self = Self(0x08);
    #[doc = "16 microframes"]
    pub const MICROFRAME_16: Self = Self(0x10);
    #[doc = "32 microframes"]
    pub const MICROFRAME_32: Self = Self(0x20);
    #[doc = "64 microframes"]
    pub const MICROFRAME_64: Self = Self(0x40);
}
impl Itc {
    pub const fn from_bits(val: u8) -> Itc {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Itc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("IMMEDIATE"),
            0x01 => f.write_str("MICROFRAME_1"),
            0x02 => f.write_str("MICROFRAME_2"),
            0x04 => f.write_str("MICROFRAME_4"),
            0x08 => f.write_str("MICROFRAME_8"),
            0x10 => f.write_str("MICROFRAME_16"),
            0x20 => f.write_str("MICROFRAME_32"),
            0x40 => f.write_str("MICROFRAME_64"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Itc {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "IMMEDIATE"),
            0x01 => defmt::write!(f, "MICROFRAME_1"),
            0x02 => defmt::write!(f, "MICROFRAME_2"),
            0x04 => defmt::write!(f, "MICROFRAME_4"),
            0x08 => defmt::write!(f, "MICROFRAME_8"),
            0x10 => defmt::write!(f, "MICROFRAME_16"),
            0x20 => defmt::write!(f, "MICROFRAME_32"),
            0x40 => defmt::write!(f, "MICROFRAME_64"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Itc {
    #[inline(always)]
    fn from(val: u8) -> Itc {
        Itc::from_bits(val)
    }
}
impl From<Itc> for u8 {
    #[inline(always)]
    fn from(val: Itc) -> u8 {
        Itc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpm {
    #[doc = "Not supported"]
    LPM_NO = 0x0,
    #[doc = "Supported"]
    LPM_EN = 0x01,
}
impl Lpm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpm {
    #[inline(always)]
    fn from(val: u8) -> Lpm {
        Lpm::from_bits(val)
    }
}
impl From<Lpm> for u8 {
    #[inline(always)]
    fn from(val: Lpm) -> u8 {
        Lpm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmDevRcvdi {
    #[doc = "Interrupt did not occur"]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred"]
    INT_YES = 0x01,
}
impl LpmDevRcvdi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmDevRcvdi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmDevRcvdi {
    #[inline(always)]
    fn from(val: u8) -> LpmDevRcvdi {
        LpmDevRcvdi::from_bits(val)
    }
}
impl From<LpmDevRcvdi> for u8 {
    #[inline(always)]
    fn from(val: LpmDevRcvdi) -> u8 {
        LpmDevRcvdi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmHstCompi {
    #[doc = "Interrupt did not occur"]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred"]
    INT_YES = 0x01,
}
impl LpmHstCompi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmHstCompi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmHstCompi {
    #[inline(always)]
    fn from(val: u8) -> LpmHstCompi {
        LpmHstCompi::from_bits(val)
    }
}
impl From<LpmHstCompi> for u8 {
    #[inline(always)]
    fn from(val: LpmHstCompi) -> u8 {
        LpmHstCompi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmL1Entryi {
    #[doc = "Interrupt did not occur"]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred"]
    INT_YES = 0x01,
}
impl LpmL1Entryi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmL1Entryi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmL1Entryi {
    #[inline(always)]
    fn from(val: u8) -> LpmL1Entryi {
        LpmL1Entryi::from_bits(val)
    }
}
impl From<LpmL1Entryi> for u8 {
    #[inline(always)]
    fn from(val: LpmL1Entryi) -> u8 {
        LpmL1Entryi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmL1Exiti {
    #[doc = "Interrupt did not occur"]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred"]
    INT_YES = 0x01,
}
impl LpmL1Exiti {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmL1Exiti {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmL1Exiti {
    #[inline(always)]
    fn from(val: u8) -> LpmL1Exiti {
        LpmL1Exiti::from_bits(val)
    }
}
impl From<LpmL1Exiti> for u8 {
    #[inline(always)]
    fn from(val: LpmL1Exiti) -> u8 {
        LpmL1Exiti::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ls {
    #[doc = "SE0"]
    SE0 = 0x0,
    #[doc = "K-state"]
    K_STATE = 0x01,
    #[doc = "J-state"]
    J_STATE = 0x02,
    #[doc = "Undefined"]
    UNDEFINED = 0x03,
}
impl Ls {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ls {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ls {
    #[inline(always)]
    fn from(val: u8) -> Ls {
        Ls::from_bits(val)
    }
}
impl From<Ls> for u8 {
    #[inline(always)]
    fn from(val: Ls) -> u8 {
        Ls::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NCc {
    #[doc = "No internal companion controller exists"]
    NO_COMP_CONTROLLER = 0x0,
    #[doc = "Internal companion controllers exist"]
    COMP_CONTROLLER = 0x01,
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
impl NCc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NCc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NCc {
    #[inline(always)]
    fn from(val: u8) -> NCc {
        NCc::from_bits(val)
    }
}
impl From<NCc> for u8 {
    #[inline(always)]
    fn from(val: NCc) -> u8 {
        NCc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pci {
    #[doc = "Port change not detected"]
    DETECT_NO = 0x0,
    #[doc = "Port change detected"]
    DETECT_YES = 0x01,
}
impl Pci {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pci {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pci {
    #[inline(always)]
    fn from(val: u8) -> Pci {
        Pci::from_bits(val)
    }
}
impl From<Pci> for u8 {
    #[inline(always)]
    fn from(val: Pci) -> u8 {
        Pci::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pfsc {
    #[doc = "Normal operation"]
    NORMAL = 0x0,
    #[doc = "Forced to full speed"]
    FULL_SPEED = 0x01,
}
impl Pfsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pfsc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pfsc {
    #[inline(always)]
    fn from(val: u8) -> Pfsc {
        Pfsc::from_bits(val)
    }
}
impl From<Pfsc> for u8 {
    #[inline(always)]
    fn from(val: Pfsc) -> u8 {
        Pfsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Phcd {
    #[doc = "Enable"]
    PHY_CLK_EN = 0x0,
    #[doc = "Disable"]
    PHY_CLK_DIS = 0x01,
}
impl Phcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Phcd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Phcd {
    #[inline(always)]
    fn from(val: u8) -> Phcd {
        Phcd::from_bits(val)
    }
}
impl From<Phcd> for u8 {
    #[inline(always)]
    fn from(val: Phcd) -> u8 {
        Phcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Phym {
    #[doc = "UTMI/UMTI+"]
    UTMI = 0x0,
    #[doc = "ULPI DDR"]
    ULPI_DDR = 0x01,
    #[doc = "ULPI"]
    ULPI = 0x02,
    #[doc = "Serial only"]
    SERIAL = 0x03,
    #[doc = "Software programmable: reset to UTMI/UTMI+"]
    SW_RST_UTMI = 0x04,
    #[doc = "Software programmable: reset to ULPI DDR"]
    SW_RST_ULPI_DDR = 0x05,
    #[doc = "Software programmable: reset to ULPI"]
    SW_RST_ULPI = 0x06,
    #[doc = "Software programmable: reset to Serial"]
    SW_RST_SERIAL = 0x07,
    #[doc = "IC-USB"]
    ICUSB = 0x08,
    #[doc = "Software programmable: reset to IC-USB"]
    SW_RST_ICUSB = 0x09,
    #[doc = "HSIC"]
    HSIC = 0x0a,
    #[doc = "Software programmable: reset to HSIC"]
    SW_RST_HSIC = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Phym {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Phym {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Phym {
    #[inline(always)]
    fn from(val: u8) -> Phym {
        Phym::from_bits(val)
    }
}
impl From<Phym> for u8 {
    #[inline(always)]
    fn from(val: Phym) -> u8 {
        Phym::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Phyw {
    #[doc = "8-bit wide data bus (software nonprogrammable)"]
    DATA_BUS_8 = 0x0,
    #[doc = "16-bit wide data bus (software nonprogrammable)"]
    DATA_BUS_16 = 0x01,
    #[doc = "Reset to 8-bit wide data bus (software programmable)"]
    SW_RST_8 = 0x02,
    #[doc = "Reset to 16-bit wide data bus (software programmable)"]
    SW_RST_16 = 0x03,
}
impl Phyw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Phyw {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Phyw {
    #[inline(always)]
    fn from(val: u8) -> Phyw {
        Phyw::from_bits(val)
    }
}
impl From<Phyw> for u8 {
    #[inline(always)]
    fn from(val: Phyw) -> u8 {
        Phyw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pic {
    #[doc = "Port indicators are off"]
    PORT_INDICATOR_OFF = 0x0,
    #[doc = "Amber"]
    PORT_IND_AMBER = 0x01,
    #[doc = "Green"]
    PORT_IND_GREEN = 0x02,
    #[doc = "Undefined"]
    UNDEFINED = 0x03,
}
impl Pic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pic {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pic {
    #[inline(always)]
    fn from(val: u8) -> Pic {
        Pic::from_bits(val)
    }
}
impl From<Pic> for u8 {
    #[inline(always)]
    fn from(val: Pic) -> u8 {
        Pic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ppc {
    #[doc = "No port power switches"]
    NO_SWITCHES = 0x0,
    #[doc = "Port power switches exist"]
    PORT_SWITCHES = 0x01,
}
impl Ppc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ppc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ppc {
    #[inline(always)]
    fn from(val: u8) -> Ppc {
        Ppc::from_bits(val)
    }
}
impl From<Ppc> for u8 {
    #[inline(always)]
    fn from(val: Ppc) -> u8 {
        Ppc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pse {
    #[doc = "Do not process the periodic schedule"]
    DONT_PROCESS_PT = 0x0,
    #[doc = "Process the periodic schedule"]
    PROCESS_PT_PERIODICLISTBASE = 0x01,
}
impl Pse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pse {
    #[inline(always)]
    fn from(val: u8) -> Pse {
        Pse::from_bits(val)
    }
}
impl From<Pse> for u8 {
    #[inline(always)]
    fn from(val: Pse) -> u8 {
        Pse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pspd {
    #[doc = "FS"]
    FS = 0x0,
    #[doc = "LS"]
    LS = 0x01,
    #[doc = "HS"]
    HS = 0x02,
    #[doc = "Undefined"]
    UNDEFINED = 0x03,
}
impl Pspd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pspd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pspd {
    #[inline(always)]
    fn from(val: u8) -> Pspd {
        Pspd::from_bits(val)
    }
}
impl From<Pspd> for u8 {
    #[inline(always)]
    fn from(val: Pspd) -> u8 {
        Pspd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptc {
    #[doc = "TEST_MODE_DISABLE"]
    TST_MODE_DIS = 0x0,
    #[doc = "J_STATE"]
    J_STATE = 0x01,
    #[doc = "K_STATE"]
    K_STATE = 0x02,
    #[doc = "SE0 (host) or NAK (device)"]
    SE0 = 0x03,
    #[doc = "Packet"]
    PCKT = 0x04,
    #[doc = "FORCE_ENABLE_HS"]
    HS = 0x05,
    #[doc = "FORCE_ENABLE_FS"]
    FS = 0x06,
    #[doc = "FORCE_ENABLE_LS"]
    LS = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Ptc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptc {
    #[inline(always)]
    fn from(val: u8) -> Ptc {
        Ptc::from_bits(val)
    }
}
impl From<Ptc> for u8 {
    #[inline(always)]
    fn from(val: Ptc) -> u8 {
        Ptc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptw {
    #[doc = "8-bit UTMI interface (60 MHz)"]
    UTMI_8 = 0x0,
    #[doc = "16-bit UTMI interface (30 MHz)"]
    UTMI_16 = 0x01,
}
impl Ptw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptw {
    #[inline(always)]
    fn from(val: u8) -> Ptw {
        Ptw::from_bits(val)
    }
}
impl From<Ptw> for u8 {
    #[inline(always)]
    fn from(val: Ptw) -> u8 {
        Ptw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rs {
    #[doc = "Stopped executing"]
    STOP = 0x0,
    #[doc = "Running"]
    RUN = 0x01,
}
impl Rs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rs {
    #[inline(always)]
    fn from(val: u8) -> Rs {
        Rs::from_bits(val)
    }
}
impl From<Rs> for u8 {
    #[inline(always)]
    fn from(val: Rs) -> u8 {
        Rs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sei {
    #[doc = "Error response did not occur"]
    INT_NO = 0x0,
    #[doc = "Error response occurred"]
    INT_YES = 0x01,
}
impl Sei {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sei {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sei {
    #[inline(always)]
    fn from(val: u8) -> Sei {
        Sei::from_bits(val)
    }
}
impl From<Sei> for u8 {
    #[inline(always)]
    fn from(val: Sei) -> u8 {
        Sei::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sli {
    #[doc = "Did not enter Suspended state"]
    SUS_NO = 0x0,
    #[doc = "Entered Suspended state"]
    SUS_YES = 0x01,
}
impl Sli {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sli {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sli {
    #[inline(always)]
    fn from(val: u8) -> Sli {
        Sli::from_bits(val)
    }
}
impl From<Sli> for u8 {
    #[inline(always)]
    fn from(val: Sli) -> u8 {
        Sli::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slom {
    #[doc = "On (default)"]
    LOCKOUT_ON = 0x0,
    #[doc = "Off"]
    LOCKOUT_OFF = 0x01,
}
impl Slom {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slom {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slom {
    #[inline(always)]
    fn from(val: u8) -> Slom {
        Slom::from_bits(val)
    }
}
impl From<Slom> for u8 {
    #[inline(always)]
    fn from(val: Slom) -> u8 {
        Slom::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm {
    #[doc = "No serial engine; always use parallel signaling"]
    SERIAL_ENGINE_NO = 0x0,
    #[doc = "Serial engine present; always use serial signaling for FS and LS"]
    SERIAL_ENGINE_EN = 0x01,
    #[doc = "Software programmable; reset to use parallel signaling for FS and LS"]
    SW_RST_PARALLEL = 0x02,
    #[doc = "Software programmable; reset to use serial signaling for FS and LS"]
    SW_RST_SERIAL_ENG = 0x03,
}
impl Sm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm {
    #[inline(always)]
    fn from(val: u8) -> Sm {
        Sm::from_bits(val)
    }
}
impl From<Sm> for u8 {
    #[inline(always)]
    fn from(val: Sm) -> u8 {
        Sm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sri {
    #[doc = "SOF not received"]
    SOF_NO = 0x0,
    #[doc = "SOF received"]
    SOF_YES = 0x01,
}
impl Sri {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sri {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sri {
    #[inline(always)]
    fn from(val: u8) -> Sri {
        Sri::from_bits(val)
    }
}
impl From<Sri> for u8 {
    #[inline(always)]
    fn from(val: Sri) -> u8 {
        Sri::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ti0 {
    #[doc = "Interrupt did not occur"]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred"]
    INT_YES = 0x01,
}
impl Ti0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ti0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ti0 {
    #[inline(always)]
    fn from(val: u8) -> Ti0 {
        Ti0::from_bits(val)
    }
}
impl From<Ti0> for u8 {
    #[inline(always)]
    fn from(val: Ti0) -> u8 {
        Ti0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ti1 {
    #[doc = "Interrupt did not occur"]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred"]
    INT_YES = 0x01,
}
impl Ti1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ti1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ti1 {
    #[inline(always)]
    fn from(val: u8) -> Ti1 {
        Ti1::from_bits(val)
    }
}
impl From<Ti1> for u8 {
    #[inline(always)]
    fn from(val: Ti1) -> u8 {
        Ti1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uai {
    #[doc = "Interrupt did not occur"]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred"]
    INT_YES = 0x01,
}
impl Uai {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uai {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uai {
    #[inline(always)]
    fn from(val: u8) -> Uai {
        Uai::from_bits(val)
    }
}
impl From<Uai> for u8 {
    #[inline(always)]
    fn from(val: Uai) -> u8 {
        Uai::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uei {
    #[doc = "Interrupt did not occur"]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred"]
    INT_YES = 0x01,
}
impl Uei {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uei {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uei {
    #[inline(always)]
    fn from(val: u8) -> Uei {
        Uei::from_bits(val)
    }
}
impl From<Uei> for u8 {
    #[inline(always)]
    fn from(val: Uei) -> u8 {
        Uei::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ui {
    #[doc = "Interrupt did not occur"]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred"]
    INT_YES = 0x01,
}
impl Ui {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ui {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ui {
    #[inline(always)]
    fn from(val: u8) -> Ui {
        Ui::from_bits(val)
    }
}
impl From<Ui> for u8 {
    #[inline(always)]
    fn from(val: Ui) -> u8 {
        Ui::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ulpii {
    #[doc = "Event completion did not occur"]
    EVENT_NO = 0x0,
    #[doc = "Event completion occurred"]
    EVENT_YES = 0x01,
}
impl Ulpii {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ulpii {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ulpii {
    #[inline(always)]
    fn from(val: u8) -> Ulpii {
        Ulpii::from_bits(val)
    }
}
impl From<Ulpii> for u8 {
    #[inline(always)]
    fn from(val: Ulpii) -> u8 {
        Ulpii::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Upi {
    #[doc = "Interrupt did not occur"]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred"]
    INT_YES = 0x01,
}
impl Upi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Upi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Upi {
    #[inline(always)]
    fn from(val: u8) -> Upi {
        Upi::from_bits(val)
    }
}
impl From<Upi> for u8 {
    #[inline(always)]
    fn from(val: Upi) -> u8 {
        Upi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uri {
    #[doc = "USB reset not received"]
    USB_NO = 0x0,
    #[doc = "USB reset received"]
    USB_YES = 0x01,
}
impl Uri {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uri {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uri {
    #[inline(always)]
    fn from(val: u8) -> Uri {
        Uri::from_bits(val)
    }
}
impl From<Uri> for u8 {
    #[inline(always)]
    fn from(val: Uri) -> u8 {
        Uri::to_bits(val)
    }
}
