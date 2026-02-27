#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilterBit0 {
    #[doc = "Filter after 7th bit for all Commands Op Codes that need to be filtered."]
    FILTER_BIT0_0 = 0x0,
    #[doc = "Filter after 8th bit for all Read/Write Op Codes that are filtered based on the address."]
    FILTER_BIT0_1 = 0x01,
}
impl FilterBit0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FilterBit0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FilterBit0 {
    #[inline(always)]
    fn from(val: u8) -> FilterBit0 {
        FilterBit0::from_bits(val)
    }
}
impl From<FilterBit0> for u8 {
    #[inline(always)]
    fn from(val: FilterBit0) -> u8 {
        FilterBit0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilterBit1 {
    #[doc = "Filter after 7th bit for all Commands Op Codes that need to be filtered."]
    FILTER_BIT3_0 = 0x0,
    #[doc = "Filter after 8th bit for all Read/Write Op Codes that are filtered based on the address."]
    FILTER_BIT3_1 = 0x01,
}
impl FilterBit1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FilterBit1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FilterBit1 {
    #[inline(always)]
    fn from(val: u8) -> FilterBit1 {
        FilterBit1::from_bits(val)
    }
}
impl From<FilterBit1> for u8 {
    #[inline(always)]
    fn from(val: FilterBit1) -> u8 {
        FilterBit1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilterBit2 {
    #[doc = "Filter after 7th bit for all Commands Op Codes that need to be filtered."]
    FILTER_BIT3_0 = 0x0,
    #[doc = "Filter after 8th bit for all Read/Write Op Codes that are filtered based on the address."]
    FILTER_BIT3_1 = 0x01,
}
impl FilterBit2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FilterBit2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FilterBit2 {
    #[inline(always)]
    fn from(val: u8) -> FilterBit2 {
        FilterBit2::from_bits(val)
    }
}
impl From<FilterBit2> for u8 {
    #[inline(always)]
    fn from(val: FilterBit2) -> u8 {
        FilterBit2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilterBit3 {
    #[doc = "Filter after 7th bit for all Commands Op Codes that need to be filtered."]
    FILTER_BIT3_0 = 0x0,
    #[doc = "Filter after 8th bit for all Read/Write Op Codes that are filtered based on the address."]
    FILTER_BIT3_1 = 0x01,
}
impl FilterBit3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FilterBit3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FilterBit3 {
    #[inline(always)]
    fn from(val: u8) -> FilterBit3 {
        FilterBit3::from_bits(val)
    }
}
impl From<FilterBit3> for u8 {
    #[inline(always)]
    fn from(val: FilterBit3) -> u8 {
        FilterBit3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0ActSel {
    #[doc = "Sets CS0 as the read location for the firmware image and sets CS1 as the write location for the FW image."]
    P0_ACT_SEL0 = 0x0,
    #[doc = "Sets CS1 as the read location for the firmware image and sets CS0 as the write location for the FW image."]
    P0_ACT_SEL1 = 0x01,
}
impl P0ActSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0ActSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0ActSel {
    #[inline(always)]
    fn from(val: u8) -> P0ActSel {
        P0ActSel::from_bits(val)
    }
}
impl From<P0ActSel> for u8 {
    #[inline(always)]
    fn from(val: P0ActSel) -> u8 {
        P0ActSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0BlkInt {
    #[doc = "Indicates normal operation"]
    P0_BLK_INT0 = 0x0,
    #[doc = "Indicates that a blocked opcode has been detected"]
    P0_BLK_INT1 = 0x01,
}
impl P0BlkInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0BlkInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0BlkInt {
    #[inline(always)]
    fn from(val: u8) -> P0BlkInt {
        P0BlkInt::from_bits(val)
    }
}
impl From<P0BlkInt> for u8 {
    #[inline(always)]
    fn from(val: P0BlkInt) -> u8 {
        P0BlkInt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0BlkMask {
    #[doc = "Indicates P0_BLK_INT is enabled."]
    P0_BLK_MASK0 = 0x0,
    #[doc = "Indicates P0_BLK_INT is disabled."]
    P0_BLK_MASK1 = 0x01,
}
impl P0BlkMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0BlkMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0BlkMask {
    #[inline(always)]
    fn from(val: u8) -> P0BlkMask {
        P0BlkMask::from_bits(val)
    }
}
impl From<P0BlkMask> for u8 {
    #[inline(always)]
    fn from(val: P0BlkMask) -> u8 {
        P0BlkMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0BypSel {
    #[doc = "CS0"]
    P0_BYP_SEL_0 = 0x0,
    #[doc = "CS1"]
    P0_BYP_SEL_1 = 0x01,
}
impl P0BypSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0BypSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0BypSel {
    #[inline(always)]
    fn from(val: u8) -> P0BypSel {
        P0BypSel::from_bits(val)
    }
}
impl From<P0BypSel> for u8 {
    #[inline(always)]
    fn from(val: P0BypSel) -> u8 {
        P0BypSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0ByteMdRst {
    #[doc = "3-byte mode after device reset."]
    P0_BYTE_MD_RST0 = 0x0,
    #[doc = "4-byte mode after device reset."]
    P0_BYTE_MD_RST1 = 0x01,
}
impl P0ByteMdRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0ByteMdRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0ByteMdRst {
    #[inline(always)]
    fn from(val: u8) -> P0ByteMdRst {
        P0ByteMdRst::from_bits(val)
    }
}
impl From<P0ByteMdRst> for u8 {
    #[inline(always)]
    fn from(val: P0ByteMdRst) -> u8 {
        P0ByteMdRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0ByteMode {
    #[doc = "3-Byte Address Mode"]
    P0_BYTE_MODE_0 = 0x0,
    #[doc = "4-Byte Address Mode"]
    P0_BYTE_MODE_1 = 0x01,
}
impl P0ByteMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0ByteMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0ByteMode {
    #[inline(always)]
    fn from(val: u8) -> P0ByteMode {
        P0ByteMode::from_bits(val)
    }
}
impl From<P0ByteMode> for u8 {
    #[inline(always)]
    fn from(val: P0ByteMode) -> u8 {
        P0ByteMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0ByteSel {
    #[doc = "Sets the filter to 3-byte address mode"]
    P0_BYTE_SEL0 = 0x0,
    #[doc = "Sets the filter to 4-byte address mode"]
    P0_BYTE_SEL1 = 0x01,
}
impl P0ByteSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0ByteSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0ByteSel {
    #[inline(always)]
    fn from(val: u8) -> P0ByteSel {
        P0ByteSel::from_bits(val)
    }
}
impl From<P0ByteSel> for u8 {
    #[inline(always)]
    fn from(val: P0ByteSel) -> u8 {
        P0ByteSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0ByteSelMd {
    #[doc = "Normal Operation. The Address byte mode is determined by the Op Code Command"]
    P0_BYTE_SEL_MD0 = 0x0,
    #[doc = "Sets the Byte Mode to 3-byte and gives control of the function to the P0_BYTE_SEL in the P0 Filter Control Register. In this mode, any changes to P0_BYTE_SEL are latched and the internal state of BYTE_SEL will retain this value when P0_BYTE_SEL_MD is cleared"]
    P0_BYTE_SEL_MD1 = 0x01,
}
impl P0ByteSelMd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0ByteSelMd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0ByteSelMd {
    #[inline(always)]
    fn from(val: u8) -> P0ByteSelMd {
        P0ByteSelMd::from_bits(val)
    }
}
impl From<P0ByteSelMd> for u8 {
    #[inline(always)]
    fn from(val: P0ByteSelMd) -> u8 {
        P0ByteSelMd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0BytemodeInt {
    #[doc = "Indicates normal operation"]
    P0_BYTEMODE_INT0 = 0x0,
    #[doc = "Indicates that a Bytemode change has been detected (OpCode E9 or B7)"]
    P0_BYTEMODE_INT1 = 0x01,
}
impl P0BytemodeInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0BytemodeInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0BytemodeInt {
    #[inline(always)]
    fn from(val: u8) -> P0BytemodeInt {
        P0BytemodeInt::from_bits(val)
    }
}
impl From<P0BytemodeInt> for u8 {
    #[inline(always)]
    fn from(val: P0BytemodeInt) -> u8 {
        P0BytemodeInt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0BytemodeMask {
    #[doc = "Indicates P0_BYTEMODE_INT is enabled."]
    P0_BYTEMODE_MASK0 = 0x0,
    #[doc = "Indicates P0_BYTEMODE_INT is disabled."]
    P0_BYTEMODE_MASK1 = 0x01,
}
impl P0BytemodeMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0BytemodeMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0BytemodeMask {
    #[inline(always)]
    fn from(val: u8) -> P0BytemodeMask {
        P0BytemodeMask::from_bits(val)
    }
}
impl From<P0BytemodeMask> for u8 {
    #[inline(always)]
    fn from(val: P0BytemodeMask) -> u8 {
        P0BytemodeMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0Dirty {
    #[doc = "Indicates normal operation. No Writes have been directed to the inactive Flash"]
    P0_DIRTY_0 = 0x0,
    #[doc = "Indicates that a write to the inactive Flash has been detected and the state of the flash should be considered \"dirty\"."]
    P0_DIRTY_1 = 0x01,
}
impl P0Dirty {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0Dirty {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0Dirty {
    #[inline(always)]
    fn from(val: u8) -> P0Dirty {
        P0Dirty::from_bits(val)
    }
}
impl From<P0Dirty> for u8 {
    #[inline(always)]
    fn from(val: P0Dirty) -> u8 {
        P0Dirty::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0DirtyInt {
    #[doc = "Indicates normal operation"]
    P0_DIRTY_INT0 = 0x0,
    #[doc = "Indicates that a write has occurred to the FW Code region of the inactive Flash and that flash is now considered corrupt."]
    P0_DIRTY_INT1 = 0x01,
}
impl P0DirtyInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0DirtyInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0DirtyInt {
    #[inline(always)]
    fn from(val: u8) -> P0DirtyInt {
        P0DirtyInt::from_bits(val)
    }
}
impl From<P0DirtyInt> for u8 {
    #[inline(always)]
    fn from(val: P0DirtyInt) -> u8 {
        P0DirtyInt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0DirtyMask {
    #[doc = "Indicates P0_DIRTY_INT interrupt is enabled."]
    P0_DIRTY_MASK0 = 0x0,
    #[doc = "Indicates P0_DIRTY_INT interrupt is disabled."]
    P0_DIRTY_MASK1 = 0x01,
}
impl P0DirtyMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0DirtyMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0DirtyMask {
    #[inline(always)]
    fn from(val: u8) -> P0DirtyMask {
        P0DirtyMask::from_bits(val)
    }
}
impl From<P0DirtyMask> for u8 {
    #[inline(always)]
    fn from(val: P0DirtyMask) -> u8 {
        P0DirtyMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0F8Int {
    #[doc = "Indicates normal operation"]
    P0_F8_INT0 = 0x0,
    #[doc = "Indicates an F8 command has been detected"]
    P0_F8_INT1 = 0x01,
}
impl P0F8Int {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0F8Int {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0F8Int {
    #[inline(always)]
    fn from(val: u8) -> P0F8Int {
        P0F8Int::from_bits(val)
    }
}
impl From<P0F8Int> for u8 {
    #[inline(always)]
    fn from(val: P0F8Int) -> u8 {
        P0F8Int::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0F8Mask {
    #[doc = "Indicates P0_F8_INT interrupt is enabled."]
    P0_F8_MASK0 = 0x0,
    #[doc = "Indicates P0_F8_INT interrupt is disabled."]
    P0_F8_MASK1 = 0x01,
}
impl P0F8Mask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0F8Mask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0F8Mask {
    #[inline(always)]
    fn from(val: u8) -> P0F8Mask {
        P0F8Mask::from_bits(val)
    }
}
impl From<P0F8Mask> for u8 {
    #[inline(always)]
    fn from(val: P0F8Mask) -> u8 {
        P0F8Mask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0FltEn {
    #[doc = "Filter Disabled. All CS Disable outputs are asserted so that host access to the flash devices is disabled. CS# inputs to the filter are disabled so that the filter state machine is locked in its current state. All other register controls can be accessed, but have no impact on the functionality of the filter while the filter is disabled."]
    FLT_DISABLE = 0x0,
    #[doc = "Filter Enabled. CS# inputs to the filter are enabled and the filter state machine controls the CS disable outputs. CS# inputs to the filter are enabled so that the filter state machine is active. All other register controls are functional. The filter is enabled by default at CPLD power on."]
    FLT_ENABLE = 0x01,
}
impl P0FltEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0FltEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0FltEn {
    #[inline(always)]
    fn from(val: u8) -> P0FltEn {
        P0FltEn::from_bits(val)
    }
}
impl From<P0FltEn> for u8 {
    #[inline(always)]
    fn from(val: P0FltEn) -> u8 {
        P0FltEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0MfgId {
    #[doc = "Macronix"]
    MACRONIX = 0x0,
    #[doc = "Windbond"]
    WINDBOND = 0x01,
    #[doc = "Micron"]
    MICRON = 0x02,
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
impl P0MfgId {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0MfgId {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0MfgId {
    #[inline(always)]
    fn from(val: u8) -> P0MfgId {
        P0MfgId::from_bits(val)
    }
}
impl From<P0MfgId> for u8 {
    #[inline(always)]
    fn from(val: P0MfgId) -> u8 {
        P0MfgId::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0PermByteMd {
    #[doc = "Normal Operation. Address byte mode can be changed as usual."]
    P0_PERM_BYTE__MD_0 = 0x0,
    #[doc = "Address byte mode cannot be changed."]
    P0_PERM_BYTE_MD_1 = 0x01,
}
impl P0PermByteMd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0PermByteMd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0PermByteMd {
    #[inline(always)]
    fn from(val: u8) -> P0PermByteMd {
        P0PermByteMd::from_bits(val)
    }
}
impl From<P0PermByteMd> for u8 {
    #[inline(always)]
    fn from(val: P0PermByteMd) -> u8 {
        P0PermByteMd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0SingleFlash {
    #[doc = "Sets flash memory organization to Dual Flash mode."]
    P0_SINGLE_FLASH0 = 0x0,
    #[doc = "Sets flash memory organization to Single Flash mode."]
    P0_SINGLE_FLASH1 = 0x01,
}
impl P0SingleFlash {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0SingleFlash {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0SingleFlash {
    #[inline(always)]
    fn from(val: u8) -> P0SingleFlash {
        P0SingleFlash::from_bits(val)
    }
}
impl From<P0SingleFlash> for u8 {
    #[inline(always)]
    fn from(val: P0SingleFlash) -> u8 {
        P0SingleFlash::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0SingleFlashAllowWrite {
    #[doc = "Normal operation. Writes are not allowed to unfiltered regions."]
    P0_SINGLE_FLASH_ALLOW_WRITE0 = 0x0,
    #[doc = "Writes are allowed to unfiltered regions."]
    P0_SINGLE_FLASH_ALLOW_WRITE1 = 0x01,
}
impl P0SingleFlashAllowWrite {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0SingleFlashAllowWrite {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0SingleFlashAllowWrite {
    #[inline(always)]
    fn from(val: u8) -> P0SingleFlashAllowWrite {
        P0SingleFlashAllowWrite::from_bits(val)
    }
}
impl From<P0SingleFlashAllowWrite> for u8 {
    #[inline(always)]
    fn from(val: P0SingleFlashAllowWrite) -> u8 {
        P0SingleFlashAllowWrite::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0SingleFlashChipSelect {
    #[doc = "Enables output to CS0 as the active chip."]
    P0_SINGLE_FLASH_CHIP_SELECT0 = 0x0,
    #[doc = "Enables output to CS1 as the active chip."]
    P0_SINGLE_FLASH_CHIP_SELECT1 = 0x01,
}
impl P0SingleFlashChipSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0SingleFlashChipSelect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0SingleFlashChipSelect {
    #[inline(always)]
    fn from(val: u8) -> P0SingleFlashChipSelect {
        P0SingleFlashChipSelect::from_bits(val)
    }
}
impl From<P0SingleFlashChipSelect> for u8 {
    #[inline(always)]
    fn from(val: P0SingleFlashChipSelect) -> u8 {
        P0SingleFlashChipSelect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0WeByteMd {
    #[doc = "Write Enable command is not required to switch address byte mode"]
    P0_WE_BYTE_MD0 = 0x0,
    #[doc = "Write Enable command is required prior to switching address byte mode"]
    P0_WE_BYTE_MD1 = 0x01,
}
impl P0WeByteMd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0WeByteMd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0WeByteMd {
    #[inline(always)]
    fn from(val: u8) -> P0WeByteMd {
        P0WeByteMd::from_bits(val)
    }
}
impl From<P0WeByteMd> for u8 {
    #[inline(always)]
    fn from(val: P0WeByteMd) -> u8 {
        P0WeByteMd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1ActSel {
    #[doc = "Sets CS0 as the read location for the firmware image and sets CS1 as the write location for the FW image."]
    P1_ACT_SEL0 = 0x0,
    #[doc = "Sets CS1 as the read location for the firmware image and sets CS0 as the write location for the FW image."]
    P1_ACT_SEL1 = 0x01,
}
impl P1ActSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1ActSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1ActSel {
    #[inline(always)]
    fn from(val: u8) -> P1ActSel {
        P1ActSel::from_bits(val)
    }
}
impl From<P1ActSel> for u8 {
    #[inline(always)]
    fn from(val: P1ActSel) -> u8 {
        P1ActSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1BlkInt {
    #[doc = "Indicates normal operation"]
    P1_BLK_INT0 = 0x0,
    #[doc = "Indicates that a blocked opcode has been detected"]
    P1_BLK_INT1 = 0x01,
}
impl P1BlkInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1BlkInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1BlkInt {
    #[inline(always)]
    fn from(val: u8) -> P1BlkInt {
        P1BlkInt::from_bits(val)
    }
}
impl From<P1BlkInt> for u8 {
    #[inline(always)]
    fn from(val: P1BlkInt) -> u8 {
        P1BlkInt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1BlkMask {
    #[doc = "Indicates P1_BLK_INT is enabled."]
    P1_BLK_MASK0 = 0x0,
    #[doc = "Indicates P1_BLK_INT is disabled."]
    P1_BLK_MASK1 = 0x01,
}
impl P1BlkMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1BlkMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1BlkMask {
    #[inline(always)]
    fn from(val: u8) -> P1BlkMask {
        P1BlkMask::from_bits(val)
    }
}
impl From<P1BlkMask> for u8 {
    #[inline(always)]
    fn from(val: P1BlkMask) -> u8 {
        P1BlkMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1BypSel {
    #[doc = "CS0"]
    P1_BYP_SEL_0 = 0x0,
    #[doc = "CS1"]
    P1_BYP_SEL_1 = 0x01,
}
impl P1BypSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1BypSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1BypSel {
    #[inline(always)]
    fn from(val: u8) -> P1BypSel {
        P1BypSel::from_bits(val)
    }
}
impl From<P1BypSel> for u8 {
    #[inline(always)]
    fn from(val: P1BypSel) -> u8 {
        P1BypSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1ByteMdRst {
    #[doc = "3-byte mode after device reset."]
    P1_BYTE_MD_RST0 = 0x0,
    #[doc = "4-byte mode after device reset."]
    P1_BYTE_MD_RST1 = 0x01,
}
impl P1ByteMdRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1ByteMdRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1ByteMdRst {
    #[inline(always)]
    fn from(val: u8) -> P1ByteMdRst {
        P1ByteMdRst::from_bits(val)
    }
}
impl From<P1ByteMdRst> for u8 {
    #[inline(always)]
    fn from(val: P1ByteMdRst) -> u8 {
        P1ByteMdRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1ByteMode {
    #[doc = "3-Byte Address Mode"]
    P1_BYTE_MODE_0 = 0x0,
    #[doc = "4-Byte Address Mode"]
    P1_BYTE_MODE_1 = 0x01,
}
impl P1ByteMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1ByteMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1ByteMode {
    #[inline(always)]
    fn from(val: u8) -> P1ByteMode {
        P1ByteMode::from_bits(val)
    }
}
impl From<P1ByteMode> for u8 {
    #[inline(always)]
    fn from(val: P1ByteMode) -> u8 {
        P1ByteMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1ByteSel {
    #[doc = "Sets the filter to 3-byte address mode."]
    P1_BYTE_SEL0 = 0x0,
    #[doc = "Sets the filter to 4-byte address mode."]
    P1_BYTE_SEL1 = 0x01,
}
impl P1ByteSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1ByteSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1ByteSel {
    #[inline(always)]
    fn from(val: u8) -> P1ByteSel {
        P1ByteSel::from_bits(val)
    }
}
impl From<P1ByteSel> for u8 {
    #[inline(always)]
    fn from(val: P1ByteSel) -> u8 {
        P1ByteSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1ByteSelMd {
    #[doc = "Normal Operation. The Address byte mode is determined by the Op Code Command."]
    P1_BYTE_SEL_MD0 = 0x0,
    #[doc = "Sets the Byte Mode to 3-byte and gives control of the function to the P1_BYTE_SEL in the P1 Filter Control Register. In this mode, any changes to P1_BYTE_SEL are latched and the internal state of P1_BYTE_SEL will retain this value when P1_BYTE_SEL_MD is cleared."]
    P1_BYTE_SEL_MD1 = 0x01,
}
impl P1ByteSelMd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1ByteSelMd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1ByteSelMd {
    #[inline(always)]
    fn from(val: u8) -> P1ByteSelMd {
        P1ByteSelMd::from_bits(val)
    }
}
impl From<P1ByteSelMd> for u8 {
    #[inline(always)]
    fn from(val: P1ByteSelMd) -> u8 {
        P1ByteSelMd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1BytemodeInt {
    #[doc = "Indicates normal operation"]
    P1_BYTEMODE_INT0 = 0x0,
    #[doc = "Indicates that a Bytemode change has been detected (OpCode E9 or B7)"]
    P1_BYTEMODE_INT1 = 0x01,
}
impl P1BytemodeInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1BytemodeInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1BytemodeInt {
    #[inline(always)]
    fn from(val: u8) -> P1BytemodeInt {
        P1BytemodeInt::from_bits(val)
    }
}
impl From<P1BytemodeInt> for u8 {
    #[inline(always)]
    fn from(val: P1BytemodeInt) -> u8 {
        P1BytemodeInt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1BytemodeMask {
    #[doc = "Indicates P1_BYTEMODE_INT is enabled."]
    P1_BYTEMODE_MASK0 = 0x0,
    #[doc = "Indicates P1_BYTEMODE_INT is disabled."]
    P1_BYTEMODE_MASK1 = 0x01,
}
impl P1BytemodeMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1BytemodeMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1BytemodeMask {
    #[inline(always)]
    fn from(val: u8) -> P1BytemodeMask {
        P1BytemodeMask::from_bits(val)
    }
}
impl From<P1BytemodeMask> for u8 {
    #[inline(always)]
    fn from(val: P1BytemodeMask) -> u8 {
        P1BytemodeMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1DirtyInt {
    #[doc = "Indicates normal operation"]
    P1_DIRTY_INT0 = 0x0,
    #[doc = "indicates that a write has occurred to the FW Code region of the inactive Flash and that flash is now considered corrupt"]
    P1_DIRTY_INT1 = 0x01,
}
impl P1DirtyInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1DirtyInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1DirtyInt {
    #[inline(always)]
    fn from(val: u8) -> P1DirtyInt {
        P1DirtyInt::from_bits(val)
    }
}
impl From<P1DirtyInt> for u8 {
    #[inline(always)]
    fn from(val: P1DirtyInt) -> u8 {
        P1DirtyInt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1DirtyMask {
    #[doc = "Indicates P1_DIRTY_INT interrupt is enabled."]
    P1_DIRTY_MASK0 = 0x0,
    #[doc = "Indicates P1_DIRTY_INT interrupt is disabled."]
    P1_DIRTY_MASK1 = 0x01,
}
impl P1DirtyMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1DirtyMask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1DirtyMask {
    #[inline(always)]
    fn from(val: u8) -> P1DirtyMask {
        P1DirtyMask::from_bits(val)
    }
}
impl From<P1DirtyMask> for u8 {
    #[inline(always)]
    fn from(val: P1DirtyMask) -> u8 {
        P1DirtyMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1F8Int {
    #[doc = "Indicates normal operation"]
    P1_F8_INT0 = 0x0,
    #[doc = "Indicates an F8 command has been detected"]
    P1_F8_INT1 = 0x01,
}
impl P1F8Int {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1F8Int {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1F8Int {
    #[inline(always)]
    fn from(val: u8) -> P1F8Int {
        P1F8Int::from_bits(val)
    }
}
impl From<P1F8Int> for u8 {
    #[inline(always)]
    fn from(val: P1F8Int) -> u8 {
        P1F8Int::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1FltEn {
    #[doc = "Filter Disabled. All CS Disable outputs are asserted so that host access to the flash devices is disabled. CS# inputs to the filter are disabled so that the filter state machine is locked in its current state. All register controls are functional."]
    FLT_DISABLE = 0x0,
    #[doc = "Filter Enabled. CS# inputs to the filter are enabled and the filter state machine controls the CS disable outputs. This bit takes priority over the P1_BYP_EN bit in the Test Register. Setting this bit enables filtering and enables communication to flash devices regardless of the state of P1_BYP_EN. All other register controls are functional while filtering is enabled. The filter is enabled by default at CPLD power on."]
    FLT_ENABLE = 0x01,
}
impl P1FltEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1FltEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1FltEn {
    #[inline(always)]
    fn from(val: u8) -> P1FltEn {
        P1FltEn::from_bits(val)
    }
}
impl From<P1FltEn> for u8 {
    #[inline(always)]
    fn from(val: P1FltEn) -> u8 {
        P1FltEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1MfgId {
    #[doc = "Macronix"]
    MACRONIX = 0x0,
    #[doc = "Windbond"]
    WINDBOND = 0x01,
    #[doc = "Micron"]
    MICRON = 0x02,
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
impl P1MfgId {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1MfgId {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1MfgId {
    #[inline(always)]
    fn from(val: u8) -> P1MfgId {
        P1MfgId::from_bits(val)
    }
}
impl From<P1MfgId> for u8 {
    #[inline(always)]
    fn from(val: P1MfgId) -> u8 {
        P1MfgId::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1PermByteMd {
    #[doc = "Normal Operation. Address byte mode can be changed as usual."]
    P1_PERM_BYTE__MD_0 = 0x0,
    #[doc = "Address byte mode cannot be changed."]
    P1_PERM_BYTE_MD_1 = 0x01,
}
impl P1PermByteMd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1PermByteMd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1PermByteMd {
    #[inline(always)]
    fn from(val: u8) -> P1PermByteMd {
        P1PermByteMd::from_bits(val)
    }
}
impl From<P1PermByteMd> for u8 {
    #[inline(always)]
    fn from(val: P1PermByteMd) -> u8 {
        P1PermByteMd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1SingleFlash {
    #[doc = "Sets flash memory organization to Dual Flash mode."]
    P1_SINGLE_FLASH0 = 0x0,
    #[doc = "Sets flash memory organization to Single Flash mode."]
    P1_SINGLE_FLASH1 = 0x01,
}
impl P1SingleFlash {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1SingleFlash {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1SingleFlash {
    #[inline(always)]
    fn from(val: u8) -> P1SingleFlash {
        P1SingleFlash::from_bits(val)
    }
}
impl From<P1SingleFlash> for u8 {
    #[inline(always)]
    fn from(val: P1SingleFlash) -> u8 {
        P1SingleFlash::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1SingleFlashAllowWrite {
    #[doc = "Normal operation. Writes are not allowed to unfiltered regions."]
    P1_SINGLE_FLASH_ALLOW_WRITE0 = 0x0,
    #[doc = "Writes are allowed to unfiltered regions."]
    P1_SINGLE_FLASH_ALLOW_WRITE1 = 0x01,
}
impl P1SingleFlashAllowWrite {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1SingleFlashAllowWrite {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1SingleFlashAllowWrite {
    #[inline(always)]
    fn from(val: u8) -> P1SingleFlashAllowWrite {
        P1SingleFlashAllowWrite::from_bits(val)
    }
}
impl From<P1SingleFlashAllowWrite> for u8 {
    #[inline(always)]
    fn from(val: P1SingleFlashAllowWrite) -> u8 {
        P1SingleFlashAllowWrite::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1SingleFlashChipSelect {
    #[doc = "Enables output to CS0 as the active chip."]
    P1_SINGLE_FLASH_CHIP_SELECT0 = 0x0,
    #[doc = "Enables output to CS1 as the active chip."]
    P1_SINGLE_FLASH_CHIP_SELECT1 = 0x01,
}
impl P1SingleFlashChipSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1SingleFlashChipSelect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1SingleFlashChipSelect {
    #[inline(always)]
    fn from(val: u8) -> P1SingleFlashChipSelect {
        P1SingleFlashChipSelect::from_bits(val)
    }
}
impl From<P1SingleFlashChipSelect> for u8 {
    #[inline(always)]
    fn from(val: P1SingleFlashChipSelect) -> u8 {
        P1SingleFlashChipSelect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1WeByteMd {
    #[doc = "Write Enable command is not required to switch address byte mode."]
    P1_WE_BYTE_MD0 = 0x0,
    #[doc = "Write Enable command is required prior to switching address byte mode."]
    P1_WE_BYTE_MD1 = 0x01,
}
impl P1WeByteMd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1WeByteMd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1WeByteMd {
    #[inline(always)]
    fn from(val: u8) -> P1WeByteMd {
        P1WeByteMd::from_bits(val)
    }
}
impl From<P1WeByteMd> for u8 {
    #[inline(always)]
    fn from(val: P1WeByteMd) -> u8 {
        P1WeByteMd::to_bits(val)
    }
}
