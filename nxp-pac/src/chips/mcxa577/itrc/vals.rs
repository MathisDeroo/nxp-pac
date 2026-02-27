#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct In3225Status(u8);
impl In3225Status {
    #[doc = "Output not triggered."]
    pub const DISABLE: Self = Self(0x0);
    #[doc = "Output has been triggered."]
    pub const ENABLE: Self = Self(0x01);
}
impl In3225Status {
    pub const fn from_bits(val: u8) -> In3225Status {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for In3225Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("DISABLE"),
            0x01 => f.write_str("ENABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for In3225Status {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "DISABLE"),
            0x01 => defmt::write!(f, "ENABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for In3225Status {
    #[inline(always)]
    fn from(val: u8) -> In3225Status {
        In3225Status::from_bits(val)
    }
}
impl From<In3225Status> for u8 {
    #[inline(always)]
    fn from(val: In3225Status) -> u8 {
        In3225Status::to_bits(val)
    }
}
