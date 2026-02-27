#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Size {
    #[doc = "8-bit"]
    EIGHT_BIT = 0x0,
    #[doc = "16-bit"]
    SIXTEEN_BIT = 0x01,
    #[doc = "32-bit"]
    THIRTYTWO_BIT = 0x02,
    #[doc = "64-bit"]
    SIXTYFOUR_BIT = 0x03,
    #[doc = "16-byte"]
    SIXTEEN_BYTE = 0x04,
    #[doc = "32-byte"]
    THIRTYTWO_BYTE = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Size {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Size {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Size {
    #[inline(always)]
    fn from(val: u8) -> Size {
        Size::from_bits(val)
    }
}
impl From<Size> for u8 {
    #[inline(always)]
    fn from(val: Size) -> u8 {
        Size::to_bits(val)
    }
}
