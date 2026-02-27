#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Basic implementation"]
    pub const FEATURE0: Self = Self(0x0);
}
impl Feature {
    pub const fn from_bits(val: u16) -> Feature {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Feature {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("FEATURE0"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "FEATURE0"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Feature {
    #[inline(always)]
    fn from(val: u16) -> Feature {
        Feature::from_bits(val)
    }
}
impl From<Feature> for u16 {
    #[inline(always)]
    fn from(val: Feature) -> u16 {
        Feature::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe0 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe0 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe0 {
        Gpwe0::from_bits(val)
    }
}
impl From<Gpwe0> for u8 {
    #[inline(always)]
    fn from(val: Gpwe0) -> u8 {
        Gpwe0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe1 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe1 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe1 {
        Gpwe1::from_bits(val)
    }
}
impl From<Gpwe1> for u8 {
    #[inline(always)]
    fn from(val: Gpwe1) -> u8 {
        Gpwe1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe10 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe10 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe10 {
        Gpwe10::from_bits(val)
    }
}
impl From<Gpwe10> for u8 {
    #[inline(always)]
    fn from(val: Gpwe10) -> u8 {
        Gpwe10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe11 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe11 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe11 {
        Gpwe11::from_bits(val)
    }
}
impl From<Gpwe11> for u8 {
    #[inline(always)]
    fn from(val: Gpwe11) -> u8 {
        Gpwe11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe12 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe12 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe12 {
        Gpwe12::from_bits(val)
    }
}
impl From<Gpwe12> for u8 {
    #[inline(always)]
    fn from(val: Gpwe12) -> u8 {
        Gpwe12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe13 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe13 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe13 {
        Gpwe13::from_bits(val)
    }
}
impl From<Gpwe13> for u8 {
    #[inline(always)]
    fn from(val: Gpwe13) -> u8 {
        Gpwe13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe14 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe14 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe14 {
        Gpwe14::from_bits(val)
    }
}
impl From<Gpwe14> for u8 {
    #[inline(always)]
    fn from(val: Gpwe14) -> u8 {
        Gpwe14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe15 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe15 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe15 {
        Gpwe15::from_bits(val)
    }
}
impl From<Gpwe15> for u8 {
    #[inline(always)]
    fn from(val: Gpwe15) -> u8 {
        Gpwe15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe16 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe16 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe16 {
        Gpwe16::from_bits(val)
    }
}
impl From<Gpwe16> for u8 {
    #[inline(always)]
    fn from(val: Gpwe16) -> u8 {
        Gpwe16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe17 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe17 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe17 {
        Gpwe17::from_bits(val)
    }
}
impl From<Gpwe17> for u8 {
    #[inline(always)]
    fn from(val: Gpwe17) -> u8 {
        Gpwe17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe18 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe18 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe18 {
        Gpwe18::from_bits(val)
    }
}
impl From<Gpwe18> for u8 {
    #[inline(always)]
    fn from(val: Gpwe18) -> u8 {
        Gpwe18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe19 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe19 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe19 {
        Gpwe19::from_bits(val)
    }
}
impl From<Gpwe19> for u8 {
    #[inline(always)]
    fn from(val: Gpwe19) -> u8 {
        Gpwe19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe2 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe2 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe2 {
        Gpwe2::from_bits(val)
    }
}
impl From<Gpwe2> for u8 {
    #[inline(always)]
    fn from(val: Gpwe2) -> u8 {
        Gpwe2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe20 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe20 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe20 {
        Gpwe20::from_bits(val)
    }
}
impl From<Gpwe20> for u8 {
    #[inline(always)]
    fn from(val: Gpwe20) -> u8 {
        Gpwe20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe21 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe21 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe21 {
        Gpwe21::from_bits(val)
    }
}
impl From<Gpwe21> for u8 {
    #[inline(always)]
    fn from(val: Gpwe21) -> u8 {
        Gpwe21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe22 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe22 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe22 {
        Gpwe22::from_bits(val)
    }
}
impl From<Gpwe22> for u8 {
    #[inline(always)]
    fn from(val: Gpwe22) -> u8 {
        Gpwe22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe23 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe23 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe23 {
        Gpwe23::from_bits(val)
    }
}
impl From<Gpwe23> for u8 {
    #[inline(always)]
    fn from(val: Gpwe23) -> u8 {
        Gpwe23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe24 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe24 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe24 {
        Gpwe24::from_bits(val)
    }
}
impl From<Gpwe24> for u8 {
    #[inline(always)]
    fn from(val: Gpwe24) -> u8 {
        Gpwe24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe25 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe25 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe25 {
        Gpwe25::from_bits(val)
    }
}
impl From<Gpwe25> for u8 {
    #[inline(always)]
    fn from(val: Gpwe25) -> u8 {
        Gpwe25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe26 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe26 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe26 {
        Gpwe26::from_bits(val)
    }
}
impl From<Gpwe26> for u8 {
    #[inline(always)]
    fn from(val: Gpwe26) -> u8 {
        Gpwe26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe27 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe27 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe27 {
        Gpwe27::from_bits(val)
    }
}
impl From<Gpwe27> for u8 {
    #[inline(always)]
    fn from(val: Gpwe27) -> u8 {
        Gpwe27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe28 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe28 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe28 {
        Gpwe28::from_bits(val)
    }
}
impl From<Gpwe28> for u8 {
    #[inline(always)]
    fn from(val: Gpwe28) -> u8 {
        Gpwe28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe29 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe29 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe29 {
        Gpwe29::from_bits(val)
    }
}
impl From<Gpwe29> for u8 {
    #[inline(always)]
    fn from(val: Gpwe29) -> u8 {
        Gpwe29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe3 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe3 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe3 {
        Gpwe3::from_bits(val)
    }
}
impl From<Gpwe3> for u8 {
    #[inline(always)]
    fn from(val: Gpwe3) -> u8 {
        Gpwe3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe30 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe30 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe30 {
        Gpwe30::from_bits(val)
    }
}
impl From<Gpwe30> for u8 {
    #[inline(always)]
    fn from(val: Gpwe30) -> u8 {
        Gpwe30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe31 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe31 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe31 {
        Gpwe31::from_bits(val)
    }
}
impl From<Gpwe31> for u8 {
    #[inline(always)]
    fn from(val: Gpwe31) -> u8 {
        Gpwe31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe4 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe4 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe4 {
        Gpwe4::from_bits(val)
    }
}
impl From<Gpwe4> for u8 {
    #[inline(always)]
    fn from(val: Gpwe4) -> u8 {
        Gpwe4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe5 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe5 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe5 {
        Gpwe5::from_bits(val)
    }
}
impl From<Gpwe5> for u8 {
    #[inline(always)]
    fn from(val: Gpwe5) -> u8 {
        Gpwe5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe6 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe6 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe6 {
        Gpwe6::from_bits(val)
    }
}
impl From<Gpwe6> for u8 {
    #[inline(always)]
    fn from(val: Gpwe6) -> u8 {
        Gpwe6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe7 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe7 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe7 {
        Gpwe7::from_bits(val)
    }
}
impl From<Gpwe7> for u8 {
    #[inline(always)]
    fn from(val: Gpwe7) -> u8 {
        Gpwe7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe8 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe8 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe8 {
        Gpwe8::from_bits(val)
    }
}
impl From<Gpwe8> for u8 {
    #[inline(always)]
    fn from(val: Gpwe8) -> u8 {
        Gpwe8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpwe9 {
    #[doc = "Not updated"]
    GPWE0 = 0x0,
    #[doc = "Updated"]
    GPWE1 = 0x01,
}
impl Gpwe9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpwe9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpwe9 {
    #[inline(always)]
    fn from(val: u8) -> Gpwe9 {
        Gpwe9::from_bits(val)
    }
}
impl From<Gpwe9> for u8 {
    #[inline(always)]
    fn from(val: Gpwe9) -> u8 {
        Gpwe9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr0Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Dse {
        Pcr0Dse::from_bits(val)
    }
}
impl From<Pcr0Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Dse) -> u8 {
        Pcr0Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr0Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Ibe {
        Pcr0Ibe::from_bits(val)
    }
}
impl From<Pcr0Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Ibe) -> u8 {
        Pcr0Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr0Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Inv {
        Pcr0Inv::from_bits(val)
    }
}
impl From<Pcr0Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Inv) -> u8 {
        Pcr0Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr0Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Lk {
        Pcr0Lk::from_bits(val)
    }
}
impl From<Pcr0Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Lk) -> u8 {
        Pcr0Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr0Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Mux {
        Pcr0Mux::from_bits(val)
    }
}
impl From<Pcr0Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Mux) -> u8 {
        Pcr0Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr0Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Ode {
        Pcr0Ode::from_bits(val)
    }
}
impl From<Pcr0Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Ode) -> u8 {
        Pcr0Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr0Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Pe {
        Pcr0Pe::from_bits(val)
    }
}
impl From<Pcr0Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Pe) -> u8 {
        Pcr0Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Pfe {
    #[doc = "Disables"]
    PFE0 = 0x0,
    #[doc = "Enables"]
    PFE1 = 0x01,
}
impl Pcr0Pfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Pfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Pfe {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Pfe {
        Pcr0Pfe::from_bits(val)
    }
}
impl From<Pcr0Pfe> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Pfe) -> u8 {
        Pcr0Pfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr0Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Ps {
        Pcr0Ps::from_bits(val)
    }
}
impl From<Pcr0Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Ps) -> u8 {
        Pcr0Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr0Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr0Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr0Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr0Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr0Sre {
        Pcr0Sre::from_bits(val)
    }
}
impl From<Pcr0Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr0Sre) -> u8 {
        Pcr0Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr10Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr10Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr10Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr10Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr10Dse {
        Pcr10Dse::from_bits(val)
    }
}
impl From<Pcr10Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr10Dse) -> u8 {
        Pcr10Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr10Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr10Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr10Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr10Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr10Ibe {
        Pcr10Ibe::from_bits(val)
    }
}
impl From<Pcr10Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr10Ibe) -> u8 {
        Pcr10Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr10Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr10Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr10Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr10Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr10Inv {
        Pcr10Inv::from_bits(val)
    }
}
impl From<Pcr10Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr10Inv) -> u8 {
        Pcr10Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr10Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr10Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr10Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr10Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr10Lk {
        Pcr10Lk::from_bits(val)
    }
}
impl From<Pcr10Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr10Lk) -> u8 {
        Pcr10Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr10Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr10Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr10Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr10Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr10Mux {
        Pcr10Mux::from_bits(val)
    }
}
impl From<Pcr10Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr10Mux) -> u8 {
        Pcr10Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr10Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr10Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr10Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr10Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr10Ode {
        Pcr10Ode::from_bits(val)
    }
}
impl From<Pcr10Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr10Ode) -> u8 {
        Pcr10Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr10Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr10Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr10Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr10Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr10Pe {
        Pcr10Pe::from_bits(val)
    }
}
impl From<Pcr10Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr10Pe) -> u8 {
        Pcr10Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr10Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr10Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr10Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr10Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr10Ps {
        Pcr10Ps::from_bits(val)
    }
}
impl From<Pcr10Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr10Ps) -> u8 {
        Pcr10Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr10Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr10Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr10Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr10Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr10Sre {
        Pcr10Sre::from_bits(val)
    }
}
impl From<Pcr10Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr10Sre) -> u8 {
        Pcr10Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr11Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr11Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr11Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr11Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr11Dse {
        Pcr11Dse::from_bits(val)
    }
}
impl From<Pcr11Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr11Dse) -> u8 {
        Pcr11Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr11Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr11Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr11Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr11Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr11Ibe {
        Pcr11Ibe::from_bits(val)
    }
}
impl From<Pcr11Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr11Ibe) -> u8 {
        Pcr11Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr11Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr11Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr11Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr11Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr11Inv {
        Pcr11Inv::from_bits(val)
    }
}
impl From<Pcr11Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr11Inv) -> u8 {
        Pcr11Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr11Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr11Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr11Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr11Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr11Lk {
        Pcr11Lk::from_bits(val)
    }
}
impl From<Pcr11Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr11Lk) -> u8 {
        Pcr11Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr11Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr11Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr11Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr11Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr11Mux {
        Pcr11Mux::from_bits(val)
    }
}
impl From<Pcr11Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr11Mux) -> u8 {
        Pcr11Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr11Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr11Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr11Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr11Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr11Ode {
        Pcr11Ode::from_bits(val)
    }
}
impl From<Pcr11Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr11Ode) -> u8 {
        Pcr11Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr11Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr11Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr11Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr11Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr11Pe {
        Pcr11Pe::from_bits(val)
    }
}
impl From<Pcr11Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr11Pe) -> u8 {
        Pcr11Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr11Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr11Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr11Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr11Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr11Ps {
        Pcr11Ps::from_bits(val)
    }
}
impl From<Pcr11Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr11Ps) -> u8 {
        Pcr11Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr11Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr11Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr11Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr11Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr11Sre {
        Pcr11Sre::from_bits(val)
    }
}
impl From<Pcr11Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr11Sre) -> u8 {
        Pcr11Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr12Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr12Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr12Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr12Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr12Dse {
        Pcr12Dse::from_bits(val)
    }
}
impl From<Pcr12Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr12Dse) -> u8 {
        Pcr12Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr12Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr12Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr12Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr12Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr12Ibe {
        Pcr12Ibe::from_bits(val)
    }
}
impl From<Pcr12Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr12Ibe) -> u8 {
        Pcr12Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr12Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr12Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr12Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr12Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr12Inv {
        Pcr12Inv::from_bits(val)
    }
}
impl From<Pcr12Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr12Inv) -> u8 {
        Pcr12Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr12Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr12Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr12Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr12Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr12Lk {
        Pcr12Lk::from_bits(val)
    }
}
impl From<Pcr12Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr12Lk) -> u8 {
        Pcr12Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr12Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr12Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr12Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr12Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr12Mux {
        Pcr12Mux::from_bits(val)
    }
}
impl From<Pcr12Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr12Mux) -> u8 {
        Pcr12Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr12Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr12Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr12Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr12Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr12Ode {
        Pcr12Ode::from_bits(val)
    }
}
impl From<Pcr12Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr12Ode) -> u8 {
        Pcr12Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr12Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr12Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr12Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr12Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr12Pe {
        Pcr12Pe::from_bits(val)
    }
}
impl From<Pcr12Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr12Pe) -> u8 {
        Pcr12Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr12Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr12Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr12Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr12Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr12Ps {
        Pcr12Ps::from_bits(val)
    }
}
impl From<Pcr12Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr12Ps) -> u8 {
        Pcr12Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr12Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr12Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr12Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr12Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr12Sre {
        Pcr12Sre::from_bits(val)
    }
}
impl From<Pcr12Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr12Sre) -> u8 {
        Pcr12Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr13Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr13Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr13Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr13Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr13Dse {
        Pcr13Dse::from_bits(val)
    }
}
impl From<Pcr13Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr13Dse) -> u8 {
        Pcr13Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr13Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr13Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr13Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr13Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr13Ibe {
        Pcr13Ibe::from_bits(val)
    }
}
impl From<Pcr13Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr13Ibe) -> u8 {
        Pcr13Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr13Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr13Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr13Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr13Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr13Inv {
        Pcr13Inv::from_bits(val)
    }
}
impl From<Pcr13Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr13Inv) -> u8 {
        Pcr13Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr13Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr13Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr13Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr13Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr13Lk {
        Pcr13Lk::from_bits(val)
    }
}
impl From<Pcr13Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr13Lk) -> u8 {
        Pcr13Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr13Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr13Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr13Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr13Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr13Mux {
        Pcr13Mux::from_bits(val)
    }
}
impl From<Pcr13Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr13Mux) -> u8 {
        Pcr13Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr13Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr13Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr13Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr13Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr13Ode {
        Pcr13Ode::from_bits(val)
    }
}
impl From<Pcr13Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr13Ode) -> u8 {
        Pcr13Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr13Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr13Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr13Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr13Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr13Pe {
        Pcr13Pe::from_bits(val)
    }
}
impl From<Pcr13Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr13Pe) -> u8 {
        Pcr13Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr13Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr13Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr13Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr13Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr13Ps {
        Pcr13Ps::from_bits(val)
    }
}
impl From<Pcr13Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr13Ps) -> u8 {
        Pcr13Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr13Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr13Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr13Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr13Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr13Sre {
        Pcr13Sre::from_bits(val)
    }
}
impl From<Pcr13Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr13Sre) -> u8 {
        Pcr13Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr14Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr14Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr14Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr14Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr14Dse {
        Pcr14Dse::from_bits(val)
    }
}
impl From<Pcr14Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr14Dse) -> u8 {
        Pcr14Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr14Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr14Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr14Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr14Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr14Ibe {
        Pcr14Ibe::from_bits(val)
    }
}
impl From<Pcr14Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr14Ibe) -> u8 {
        Pcr14Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr14Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr14Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr14Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr14Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr14Inv {
        Pcr14Inv::from_bits(val)
    }
}
impl From<Pcr14Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr14Inv) -> u8 {
        Pcr14Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr14Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr14Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr14Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr14Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr14Lk {
        Pcr14Lk::from_bits(val)
    }
}
impl From<Pcr14Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr14Lk) -> u8 {
        Pcr14Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr14Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr14Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr14Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr14Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr14Mux {
        Pcr14Mux::from_bits(val)
    }
}
impl From<Pcr14Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr14Mux) -> u8 {
        Pcr14Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr14Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr14Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr14Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr14Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr14Ode {
        Pcr14Ode::from_bits(val)
    }
}
impl From<Pcr14Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr14Ode) -> u8 {
        Pcr14Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr14Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr14Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr14Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr14Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr14Pe {
        Pcr14Pe::from_bits(val)
    }
}
impl From<Pcr14Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr14Pe) -> u8 {
        Pcr14Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr14Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr14Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr14Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr14Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr14Ps {
        Pcr14Ps::from_bits(val)
    }
}
impl From<Pcr14Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr14Ps) -> u8 {
        Pcr14Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr14Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr14Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr14Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr14Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr14Sre {
        Pcr14Sre::from_bits(val)
    }
}
impl From<Pcr14Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr14Sre) -> u8 {
        Pcr14Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr15Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr15Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr15Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr15Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr15Dse {
        Pcr15Dse::from_bits(val)
    }
}
impl From<Pcr15Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr15Dse) -> u8 {
        Pcr15Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr15Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr15Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr15Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr15Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr15Ibe {
        Pcr15Ibe::from_bits(val)
    }
}
impl From<Pcr15Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr15Ibe) -> u8 {
        Pcr15Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr15Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr15Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr15Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr15Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr15Inv {
        Pcr15Inv::from_bits(val)
    }
}
impl From<Pcr15Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr15Inv) -> u8 {
        Pcr15Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr15Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr15Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr15Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr15Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr15Lk {
        Pcr15Lk::from_bits(val)
    }
}
impl From<Pcr15Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr15Lk) -> u8 {
        Pcr15Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr15Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr15Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr15Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr15Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr15Mux {
        Pcr15Mux::from_bits(val)
    }
}
impl From<Pcr15Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr15Mux) -> u8 {
        Pcr15Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr15Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr15Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr15Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr15Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr15Ode {
        Pcr15Ode::from_bits(val)
    }
}
impl From<Pcr15Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr15Ode) -> u8 {
        Pcr15Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr15Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr15Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr15Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr15Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr15Pe {
        Pcr15Pe::from_bits(val)
    }
}
impl From<Pcr15Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr15Pe) -> u8 {
        Pcr15Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr15Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr15Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr15Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr15Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr15Ps {
        Pcr15Ps::from_bits(val)
    }
}
impl From<Pcr15Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr15Ps) -> u8 {
        Pcr15Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr15Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr15Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr15Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr15Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr15Sre {
        Pcr15Sre::from_bits(val)
    }
}
impl From<Pcr15Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr15Sre) -> u8 {
        Pcr15Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr16Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Dse {
        Pcr16Dse::from_bits(val)
    }
}
impl From<Pcr16Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Dse) -> u8 {
        Pcr16Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr16Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Ibe {
        Pcr16Ibe::from_bits(val)
    }
}
impl From<Pcr16Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Ibe) -> u8 {
        Pcr16Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr16Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Inv {
        Pcr16Inv::from_bits(val)
    }
}
impl From<Pcr16Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Inv) -> u8 {
        Pcr16Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr16Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Lk {
        Pcr16Lk::from_bits(val)
    }
}
impl From<Pcr16Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Lk) -> u8 {
        Pcr16Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr16Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Mux {
        Pcr16Mux::from_bits(val)
    }
}
impl From<Pcr16Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Mux) -> u8 {
        Pcr16Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr16Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Ode {
        Pcr16Ode::from_bits(val)
    }
}
impl From<Pcr16Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Ode) -> u8 {
        Pcr16Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr16Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Pe {
        Pcr16Pe::from_bits(val)
    }
}
impl From<Pcr16Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Pe) -> u8 {
        Pcr16Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr16Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Ps {
        Pcr16Ps::from_bits(val)
    }
}
impl From<Pcr16Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Ps) -> u8 {
        Pcr16Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Pv {
    #[doc = "Low"]
    PV0 = 0x0,
    #[doc = "High"]
    PV1 = 0x01,
}
impl Pcr16Pv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Pv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Pv {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Pv {
        Pcr16Pv::from_bits(val)
    }
}
impl From<Pcr16Pv> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Pv) -> u8 {
        Pcr16Pv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr16Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr16Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr16Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr16Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr16Sre {
        Pcr16Sre::from_bits(val)
    }
}
impl From<Pcr16Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr16Sre) -> u8 {
        Pcr16Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr17Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr17Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr17Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr17Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr17Dse {
        Pcr17Dse::from_bits(val)
    }
}
impl From<Pcr17Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr17Dse) -> u8 {
        Pcr17Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr17Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr17Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr17Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr17Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr17Ibe {
        Pcr17Ibe::from_bits(val)
    }
}
impl From<Pcr17Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr17Ibe) -> u8 {
        Pcr17Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr17Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr17Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr17Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr17Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr17Inv {
        Pcr17Inv::from_bits(val)
    }
}
impl From<Pcr17Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr17Inv) -> u8 {
        Pcr17Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr17Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr17Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr17Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr17Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr17Lk {
        Pcr17Lk::from_bits(val)
    }
}
impl From<Pcr17Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr17Lk) -> u8 {
        Pcr17Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr17Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr17Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr17Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr17Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr17Mux {
        Pcr17Mux::from_bits(val)
    }
}
impl From<Pcr17Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr17Mux) -> u8 {
        Pcr17Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr17Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr17Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr17Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr17Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr17Ode {
        Pcr17Ode::from_bits(val)
    }
}
impl From<Pcr17Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr17Ode) -> u8 {
        Pcr17Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr17Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr17Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr17Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr17Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr17Pe {
        Pcr17Pe::from_bits(val)
    }
}
impl From<Pcr17Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr17Pe) -> u8 {
        Pcr17Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr17Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr17Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr17Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr17Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr17Ps {
        Pcr17Ps::from_bits(val)
    }
}
impl From<Pcr17Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr17Ps) -> u8 {
        Pcr17Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr17Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr17Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr17Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr17Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr17Sre {
        Pcr17Sre::from_bits(val)
    }
}
impl From<Pcr17Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr17Sre) -> u8 {
        Pcr17Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr18Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr18Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr18Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr18Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr18Dse {
        Pcr18Dse::from_bits(val)
    }
}
impl From<Pcr18Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr18Dse) -> u8 {
        Pcr18Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr18Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr18Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr18Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr18Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr18Ibe {
        Pcr18Ibe::from_bits(val)
    }
}
impl From<Pcr18Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr18Ibe) -> u8 {
        Pcr18Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr18Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr18Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr18Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr18Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr18Inv {
        Pcr18Inv::from_bits(val)
    }
}
impl From<Pcr18Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr18Inv) -> u8 {
        Pcr18Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr18Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr18Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr18Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr18Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr18Lk {
        Pcr18Lk::from_bits(val)
    }
}
impl From<Pcr18Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr18Lk) -> u8 {
        Pcr18Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr18Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr18Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr18Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr18Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr18Mux {
        Pcr18Mux::from_bits(val)
    }
}
impl From<Pcr18Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr18Mux) -> u8 {
        Pcr18Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr18Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr18Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr18Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr18Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr18Ode {
        Pcr18Ode::from_bits(val)
    }
}
impl From<Pcr18Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr18Ode) -> u8 {
        Pcr18Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr18Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr18Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr18Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr18Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr18Pe {
        Pcr18Pe::from_bits(val)
    }
}
impl From<Pcr18Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr18Pe) -> u8 {
        Pcr18Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr18Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr18Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr18Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr18Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr18Ps {
        Pcr18Ps::from_bits(val)
    }
}
impl From<Pcr18Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr18Ps) -> u8 {
        Pcr18Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr18Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr18Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr18Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr18Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr18Sre {
        Pcr18Sre::from_bits(val)
    }
}
impl From<Pcr18Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr18Sre) -> u8 {
        Pcr18Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr19Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr19Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr19Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr19Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr19Dse {
        Pcr19Dse::from_bits(val)
    }
}
impl From<Pcr19Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr19Dse) -> u8 {
        Pcr19Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr19Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr19Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr19Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr19Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr19Ibe {
        Pcr19Ibe::from_bits(val)
    }
}
impl From<Pcr19Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr19Ibe) -> u8 {
        Pcr19Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr19Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr19Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr19Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr19Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr19Inv {
        Pcr19Inv::from_bits(val)
    }
}
impl From<Pcr19Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr19Inv) -> u8 {
        Pcr19Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr19Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr19Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr19Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr19Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr19Lk {
        Pcr19Lk::from_bits(val)
    }
}
impl From<Pcr19Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr19Lk) -> u8 {
        Pcr19Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr19Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr19Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr19Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr19Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr19Mux {
        Pcr19Mux::from_bits(val)
    }
}
impl From<Pcr19Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr19Mux) -> u8 {
        Pcr19Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr19Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr19Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr19Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr19Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr19Ode {
        Pcr19Ode::from_bits(val)
    }
}
impl From<Pcr19Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr19Ode) -> u8 {
        Pcr19Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr19Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr19Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr19Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr19Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr19Pe {
        Pcr19Pe::from_bits(val)
    }
}
impl From<Pcr19Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr19Pe) -> u8 {
        Pcr19Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr19Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr19Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr19Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr19Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr19Ps {
        Pcr19Ps::from_bits(val)
    }
}
impl From<Pcr19Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr19Ps) -> u8 {
        Pcr19Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr19Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr19Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr19Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr19Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr19Sre {
        Pcr19Sre::from_bits(val)
    }
}
impl From<Pcr19Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr19Sre) -> u8 {
        Pcr19Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr1Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Dse {
        Pcr1Dse::from_bits(val)
    }
}
impl From<Pcr1Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Dse) -> u8 {
        Pcr1Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr1Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Ibe {
        Pcr1Ibe::from_bits(val)
    }
}
impl From<Pcr1Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Ibe) -> u8 {
        Pcr1Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr1Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Inv {
        Pcr1Inv::from_bits(val)
    }
}
impl From<Pcr1Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Inv) -> u8 {
        Pcr1Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr1Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Lk {
        Pcr1Lk::from_bits(val)
    }
}
impl From<Pcr1Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Lk) -> u8 {
        Pcr1Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr1Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Mux {
        Pcr1Mux::from_bits(val)
    }
}
impl From<Pcr1Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Mux) -> u8 {
        Pcr1Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr1Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Ode {
        Pcr1Ode::from_bits(val)
    }
}
impl From<Pcr1Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Ode) -> u8 {
        Pcr1Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr1Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Pe {
        Pcr1Pe::from_bits(val)
    }
}
impl From<Pcr1Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Pe) -> u8 {
        Pcr1Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Pfe {
    #[doc = "Disables"]
    PFE0 = 0x0,
    #[doc = "Enables"]
    PFE1 = 0x01,
}
impl Pcr1Pfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Pfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Pfe {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Pfe {
        Pcr1Pfe::from_bits(val)
    }
}
impl From<Pcr1Pfe> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Pfe) -> u8 {
        Pcr1Pfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr1Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Ps {
        Pcr1Ps::from_bits(val)
    }
}
impl From<Pcr1Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Ps) -> u8 {
        Pcr1Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr1Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr1Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr1Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr1Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr1Sre {
        Pcr1Sre::from_bits(val)
    }
}
impl From<Pcr1Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr1Sre) -> u8 {
        Pcr1Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr29Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr29Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr29Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr29Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr29Dse {
        Pcr29Dse::from_bits(val)
    }
}
impl From<Pcr29Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr29Dse) -> u8 {
        Pcr29Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr29Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr29Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr29Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr29Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr29Ibe {
        Pcr29Ibe::from_bits(val)
    }
}
impl From<Pcr29Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr29Ibe) -> u8 {
        Pcr29Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr29Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr29Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr29Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr29Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr29Inv {
        Pcr29Inv::from_bits(val)
    }
}
impl From<Pcr29Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr29Inv) -> u8 {
        Pcr29Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr29Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr29Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr29Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr29Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr29Lk {
        Pcr29Lk::from_bits(val)
    }
}
impl From<Pcr29Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr29Lk) -> u8 {
        Pcr29Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr29Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
}
impl Pcr29Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr29Mux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr29Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr29Mux {
        Pcr29Mux::from_bits(val)
    }
}
impl From<Pcr29Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr29Mux) -> u8 {
        Pcr29Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr29Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr29Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr29Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr29Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr29Ode {
        Pcr29Ode::from_bits(val)
    }
}
impl From<Pcr29Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr29Ode) -> u8 {
        Pcr29Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr29Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr29Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr29Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr29Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr29Pe {
        Pcr29Pe::from_bits(val)
    }
}
impl From<Pcr29Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr29Pe) -> u8 {
        Pcr29Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr29Pfe {
    #[doc = "Disables"]
    PFE0 = 0x0,
    #[doc = "Enables"]
    PFE1 = 0x01,
}
impl Pcr29Pfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr29Pfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr29Pfe {
    #[inline(always)]
    fn from(val: u8) -> Pcr29Pfe {
        Pcr29Pfe::from_bits(val)
    }
}
impl From<Pcr29Pfe> for u8 {
    #[inline(always)]
    fn from(val: Pcr29Pfe) -> u8 {
        Pcr29Pfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr29Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr29Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr29Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr29Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr29Ps {
        Pcr29Ps::from_bits(val)
    }
}
impl From<Pcr29Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr29Ps) -> u8 {
        Pcr29Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr29Pv {
    #[doc = "Low"]
    PV0 = 0x0,
    #[doc = "High"]
    PV1 = 0x01,
}
impl Pcr29Pv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr29Pv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr29Pv {
    #[inline(always)]
    fn from(val: u8) -> Pcr29Pv {
        Pcr29Pv::from_bits(val)
    }
}
impl From<Pcr29Pv> for u8 {
    #[inline(always)]
    fn from(val: Pcr29Pv) -> u8 {
        Pcr29Pv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr29Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr29Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr29Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr29Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr29Sre {
        Pcr29Sre::from_bits(val)
    }
}
impl From<Pcr29Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr29Sre) -> u8 {
        Pcr29Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr2Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Dse {
        Pcr2Dse::from_bits(val)
    }
}
impl From<Pcr2Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Dse) -> u8 {
        Pcr2Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr2Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Ibe {
        Pcr2Ibe::from_bits(val)
    }
}
impl From<Pcr2Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Ibe) -> u8 {
        Pcr2Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr2Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Inv {
        Pcr2Inv::from_bits(val)
    }
}
impl From<Pcr2Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Inv) -> u8 {
        Pcr2Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr2Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Lk {
        Pcr2Lk::from_bits(val)
    }
}
impl From<Pcr2Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Lk) -> u8 {
        Pcr2Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr2Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Mux {
        Pcr2Mux::from_bits(val)
    }
}
impl From<Pcr2Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Mux) -> u8 {
        Pcr2Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr2Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Ode {
        Pcr2Ode::from_bits(val)
    }
}
impl From<Pcr2Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Ode) -> u8 {
        Pcr2Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr2Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Pe {
        Pcr2Pe::from_bits(val)
    }
}
impl From<Pcr2Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Pe) -> u8 {
        Pcr2Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr2Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Ps {
        Pcr2Ps::from_bits(val)
    }
}
impl From<Pcr2Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Ps) -> u8 {
        Pcr2Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr2Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr2Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr2Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr2Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr2Sre {
        Pcr2Sre::from_bits(val)
    }
}
impl From<Pcr2Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr2Sre) -> u8 {
        Pcr2Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr30Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr30Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr30Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr30Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr30Dse {
        Pcr30Dse::from_bits(val)
    }
}
impl From<Pcr30Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr30Dse) -> u8 {
        Pcr30Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr30Dse1 {
    #[doc = "Normal"]
    DSE10 = 0x0,
    #[doc = "Double"]
    DSE11 = 0x01,
}
impl Pcr30Dse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr30Dse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr30Dse1 {
    #[inline(always)]
    fn from(val: u8) -> Pcr30Dse1 {
        Pcr30Dse1::from_bits(val)
    }
}
impl From<Pcr30Dse1> for u8 {
    #[inline(always)]
    fn from(val: Pcr30Dse1) -> u8 {
        Pcr30Dse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr30Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr30Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr30Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr30Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr30Ibe {
        Pcr30Ibe::from_bits(val)
    }
}
impl From<Pcr30Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr30Ibe) -> u8 {
        Pcr30Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr30Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr30Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr30Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr30Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr30Inv {
        Pcr30Inv::from_bits(val)
    }
}
impl From<Pcr30Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr30Inv) -> u8 {
        Pcr30Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr30Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr30Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr30Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr30Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr30Lk {
        Pcr30Lk::from_bits(val)
    }
}
impl From<Pcr30Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr30Lk) -> u8 {
        Pcr30Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr30Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr30Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr30Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr30Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr30Mux {
        Pcr30Mux::from_bits(val)
    }
}
impl From<Pcr30Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr30Mux) -> u8 {
        Pcr30Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr30Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr30Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr30Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr30Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr30Ode {
        Pcr30Ode::from_bits(val)
    }
}
impl From<Pcr30Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr30Ode) -> u8 {
        Pcr30Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr30Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr30Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr30Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr30Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr30Pe {
        Pcr30Pe::from_bits(val)
    }
}
impl From<Pcr30Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr30Pe) -> u8 {
        Pcr30Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr30Pfe {
    #[doc = "Disables"]
    PFE0 = 0x0,
    #[doc = "Enables"]
    PFE1 = 0x01,
}
impl Pcr30Pfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr30Pfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr30Pfe {
    #[inline(always)]
    fn from(val: u8) -> Pcr30Pfe {
        Pcr30Pfe::from_bits(val)
    }
}
impl From<Pcr30Pfe> for u8 {
    #[inline(always)]
    fn from(val: Pcr30Pfe) -> u8 {
        Pcr30Pfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr30Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr30Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr30Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr30Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr30Ps {
        Pcr30Ps::from_bits(val)
    }
}
impl From<Pcr30Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr30Ps) -> u8 {
        Pcr30Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr30Pv {
    #[doc = "Low"]
    PV0 = 0x0,
    #[doc = "High"]
    PV1 = 0x01,
}
impl Pcr30Pv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr30Pv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr30Pv {
    #[inline(always)]
    fn from(val: u8) -> Pcr30Pv {
        Pcr30Pv::from_bits(val)
    }
}
impl From<Pcr30Pv> for u8 {
    #[inline(always)]
    fn from(val: Pcr30Pv) -> u8 {
        Pcr30Pv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr30Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr30Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr30Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr30Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr30Sre {
        Pcr30Sre::from_bits(val)
    }
}
impl From<Pcr30Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr30Sre) -> u8 {
        Pcr30Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr31Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr31Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr31Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr31Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr31Dse {
        Pcr31Dse::from_bits(val)
    }
}
impl From<Pcr31Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr31Dse) -> u8 {
        Pcr31Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr31Dse1 {
    #[doc = "Normal"]
    DSE10 = 0x0,
    #[doc = "Double"]
    DSE11 = 0x01,
}
impl Pcr31Dse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr31Dse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr31Dse1 {
    #[inline(always)]
    fn from(val: u8) -> Pcr31Dse1 {
        Pcr31Dse1::from_bits(val)
    }
}
impl From<Pcr31Dse1> for u8 {
    #[inline(always)]
    fn from(val: Pcr31Dse1) -> u8 {
        Pcr31Dse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr31Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr31Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr31Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr31Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr31Ibe {
        Pcr31Ibe::from_bits(val)
    }
}
impl From<Pcr31Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr31Ibe) -> u8 {
        Pcr31Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr31Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr31Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr31Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr31Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr31Inv {
        Pcr31Inv::from_bits(val)
    }
}
impl From<Pcr31Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr31Inv) -> u8 {
        Pcr31Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr31Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr31Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr31Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr31Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr31Lk {
        Pcr31Lk::from_bits(val)
    }
}
impl From<Pcr31Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr31Lk) -> u8 {
        Pcr31Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr31Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr31Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr31Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr31Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr31Mux {
        Pcr31Mux::from_bits(val)
    }
}
impl From<Pcr31Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr31Mux) -> u8 {
        Pcr31Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr31Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr31Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr31Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr31Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr31Ode {
        Pcr31Ode::from_bits(val)
    }
}
impl From<Pcr31Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr31Ode) -> u8 {
        Pcr31Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr31Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr31Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr31Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr31Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr31Pe {
        Pcr31Pe::from_bits(val)
    }
}
impl From<Pcr31Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr31Pe) -> u8 {
        Pcr31Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr31Pfe {
    #[doc = "Disables"]
    PFE0 = 0x0,
    #[doc = "Enables"]
    PFE1 = 0x01,
}
impl Pcr31Pfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr31Pfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr31Pfe {
    #[inline(always)]
    fn from(val: u8) -> Pcr31Pfe {
        Pcr31Pfe::from_bits(val)
    }
}
impl From<Pcr31Pfe> for u8 {
    #[inline(always)]
    fn from(val: Pcr31Pfe) -> u8 {
        Pcr31Pfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr31Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr31Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr31Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr31Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr31Ps {
        Pcr31Ps::from_bits(val)
    }
}
impl From<Pcr31Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr31Ps) -> u8 {
        Pcr31Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr31Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr31Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr31Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr31Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr31Sre {
        Pcr31Sre::from_bits(val)
    }
}
impl From<Pcr31Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr31Sre) -> u8 {
        Pcr31Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr3Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Dse {
        Pcr3Dse::from_bits(val)
    }
}
impl From<Pcr3Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Dse) -> u8 {
        Pcr3Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr3Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Ibe {
        Pcr3Ibe::from_bits(val)
    }
}
impl From<Pcr3Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Ibe) -> u8 {
        Pcr3Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr3Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Inv {
        Pcr3Inv::from_bits(val)
    }
}
impl From<Pcr3Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Inv) -> u8 {
        Pcr3Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr3Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Lk {
        Pcr3Lk::from_bits(val)
    }
}
impl From<Pcr3Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Lk) -> u8 {
        Pcr3Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr3Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Mux {
        Pcr3Mux::from_bits(val)
    }
}
impl From<Pcr3Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Mux) -> u8 {
        Pcr3Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr3Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Ode {
        Pcr3Ode::from_bits(val)
    }
}
impl From<Pcr3Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Ode) -> u8 {
        Pcr3Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr3Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Pe {
        Pcr3Pe::from_bits(val)
    }
}
impl From<Pcr3Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Pe) -> u8 {
        Pcr3Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr3Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Ps {
        Pcr3Ps::from_bits(val)
    }
}
impl From<Pcr3Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Ps) -> u8 {
        Pcr3Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr3Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr3Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr3Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr3Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr3Sre {
        Pcr3Sre::from_bits(val)
    }
}
impl From<Pcr3Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr3Sre) -> u8 {
        Pcr3Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr4Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Dse {
        Pcr4Dse::from_bits(val)
    }
}
impl From<Pcr4Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Dse) -> u8 {
        Pcr4Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr4Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Ibe {
        Pcr4Ibe::from_bits(val)
    }
}
impl From<Pcr4Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Ibe) -> u8 {
        Pcr4Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr4Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Inv {
        Pcr4Inv::from_bits(val)
    }
}
impl From<Pcr4Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Inv) -> u8 {
        Pcr4Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr4Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Lk {
        Pcr4Lk::from_bits(val)
    }
}
impl From<Pcr4Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Lk) -> u8 {
        Pcr4Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr4Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Mux {
        Pcr4Mux::from_bits(val)
    }
}
impl From<Pcr4Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Mux) -> u8 {
        Pcr4Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr4Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Ode {
        Pcr4Ode::from_bits(val)
    }
}
impl From<Pcr4Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Ode) -> u8 {
        Pcr4Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr4Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Pe {
        Pcr4Pe::from_bits(val)
    }
}
impl From<Pcr4Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Pe) -> u8 {
        Pcr4Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr4Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Ps {
        Pcr4Ps::from_bits(val)
    }
}
impl From<Pcr4Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Ps) -> u8 {
        Pcr4Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr4Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr4Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr4Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr4Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr4Sre {
        Pcr4Sre::from_bits(val)
    }
}
impl From<Pcr4Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr4Sre) -> u8 {
        Pcr4Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr5Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Dse {
        Pcr5Dse::from_bits(val)
    }
}
impl From<Pcr5Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Dse) -> u8 {
        Pcr5Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr5Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Ibe {
        Pcr5Ibe::from_bits(val)
    }
}
impl From<Pcr5Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Ibe) -> u8 {
        Pcr5Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr5Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Inv {
        Pcr5Inv::from_bits(val)
    }
}
impl From<Pcr5Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Inv) -> u8 {
        Pcr5Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr5Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Lk {
        Pcr5Lk::from_bits(val)
    }
}
impl From<Pcr5Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Lk) -> u8 {
        Pcr5Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr5Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Mux {
        Pcr5Mux::from_bits(val)
    }
}
impl From<Pcr5Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Mux) -> u8 {
        Pcr5Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr5Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Ode {
        Pcr5Ode::from_bits(val)
    }
}
impl From<Pcr5Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Ode) -> u8 {
        Pcr5Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr5Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Pe {
        Pcr5Pe::from_bits(val)
    }
}
impl From<Pcr5Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Pe) -> u8 {
        Pcr5Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr5Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Ps {
        Pcr5Ps::from_bits(val)
    }
}
impl From<Pcr5Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Ps) -> u8 {
        Pcr5Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr5Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr5Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr5Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr5Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr5Sre {
        Pcr5Sre::from_bits(val)
    }
}
impl From<Pcr5Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr5Sre) -> u8 {
        Pcr5Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr6Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Dse {
        Pcr6Dse::from_bits(val)
    }
}
impl From<Pcr6Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Dse) -> u8 {
        Pcr6Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr6Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Ibe {
        Pcr6Ibe::from_bits(val)
    }
}
impl From<Pcr6Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Ibe) -> u8 {
        Pcr6Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr6Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Inv {
        Pcr6Inv::from_bits(val)
    }
}
impl From<Pcr6Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Inv) -> u8 {
        Pcr6Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr6Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Lk {
        Pcr6Lk::from_bits(val)
    }
}
impl From<Pcr6Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Lk) -> u8 {
        Pcr6Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr6Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Mux {
        Pcr6Mux::from_bits(val)
    }
}
impl From<Pcr6Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Mux) -> u8 {
        Pcr6Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr6Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Ode {
        Pcr6Ode::from_bits(val)
    }
}
impl From<Pcr6Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Ode) -> u8 {
        Pcr6Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr6Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Pe {
        Pcr6Pe::from_bits(val)
    }
}
impl From<Pcr6Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Pe) -> u8 {
        Pcr6Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr6Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Ps {
        Pcr6Ps::from_bits(val)
    }
}
impl From<Pcr6Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Ps) -> u8 {
        Pcr6Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr6Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr6Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr6Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr6Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr6Sre {
        Pcr6Sre::from_bits(val)
    }
}
impl From<Pcr6Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr6Sre) -> u8 {
        Pcr6Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr7Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Dse {
        Pcr7Dse::from_bits(val)
    }
}
impl From<Pcr7Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Dse) -> u8 {
        Pcr7Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr7Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Ibe {
        Pcr7Ibe::from_bits(val)
    }
}
impl From<Pcr7Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Ibe) -> u8 {
        Pcr7Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr7Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Inv {
        Pcr7Inv::from_bits(val)
    }
}
impl From<Pcr7Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Inv) -> u8 {
        Pcr7Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr7Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Lk {
        Pcr7Lk::from_bits(val)
    }
}
impl From<Pcr7Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Lk) -> u8 {
        Pcr7Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr7Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Mux {
        Pcr7Mux::from_bits(val)
    }
}
impl From<Pcr7Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Mux) -> u8 {
        Pcr7Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr7Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Ode {
        Pcr7Ode::from_bits(val)
    }
}
impl From<Pcr7Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Ode) -> u8 {
        Pcr7Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr7Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Pe {
        Pcr7Pe::from_bits(val)
    }
}
impl From<Pcr7Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Pe) -> u8 {
        Pcr7Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr7Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Ps {
        Pcr7Ps::from_bits(val)
    }
}
impl From<Pcr7Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Ps) -> u8 {
        Pcr7Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr7Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr7Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr7Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr7Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr7Sre {
        Pcr7Sre::from_bits(val)
    }
}
impl From<Pcr7Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr7Sre) -> u8 {
        Pcr7Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr8Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Dse {
        Pcr8Dse::from_bits(val)
    }
}
impl From<Pcr8Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Dse) -> u8 {
        Pcr8Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr8Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Ibe {
        Pcr8Ibe::from_bits(val)
    }
}
impl From<Pcr8Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Ibe) -> u8 {
        Pcr8Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr8Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Inv {
        Pcr8Inv::from_bits(val)
    }
}
impl From<Pcr8Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Inv) -> u8 {
        Pcr8Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr8Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Lk {
        Pcr8Lk::from_bits(val)
    }
}
impl From<Pcr8Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Lk) -> u8 {
        Pcr8Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr8Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Mux {
        Pcr8Mux::from_bits(val)
    }
}
impl From<Pcr8Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Mux) -> u8 {
        Pcr8Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr8Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Ode {
        Pcr8Ode::from_bits(val)
    }
}
impl From<Pcr8Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Ode) -> u8 {
        Pcr8Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr8Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Pe {
        Pcr8Pe::from_bits(val)
    }
}
impl From<Pcr8Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Pe) -> u8 {
        Pcr8Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Pfe {
    #[doc = "Disables"]
    PFE0 = 0x0,
    #[doc = "Enables"]
    PFE1 = 0x01,
}
impl Pcr8Pfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Pfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Pfe {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Pfe {
        Pcr8Pfe::from_bits(val)
    }
}
impl From<Pcr8Pfe> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Pfe) -> u8 {
        Pcr8Pfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr8Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Ps {
        Pcr8Ps::from_bits(val)
    }
}
impl From<Pcr8Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Ps) -> u8 {
        Pcr8Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Pv {
    #[doc = "Low"]
    PV0 = 0x0,
    #[doc = "High"]
    PV1 = 0x01,
}
impl Pcr8Pv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Pv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Pv {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Pv {
        Pcr8Pv::from_bits(val)
    }
}
impl From<Pcr8Pv> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Pv) -> u8 {
        Pcr8Pv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr8Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr8Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr8Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr8Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr8Sre {
        Pcr8Sre::from_bits(val)
    }
}
impl From<Pcr8Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr8Sre) -> u8 {
        Pcr8Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Dse {
    #[doc = "Low"]
    DSE0 = 0x0,
    #[doc = "High"]
    DSE1 = 0x01,
}
impl Pcr9Dse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Dse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Dse {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Dse {
        Pcr9Dse::from_bits(val)
    }
}
impl From<Pcr9Dse> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Dse) -> u8 {
        Pcr9Dse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Ibe {
    #[doc = "Disables"]
    IBE0 = 0x0,
    #[doc = "Enables"]
    IBE1 = 0x01,
}
impl Pcr9Ibe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Ibe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Ibe {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Ibe {
        Pcr9Ibe::from_bits(val)
    }
}
impl From<Pcr9Ibe> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Ibe) -> u8 {
        Pcr9Ibe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Inv {
    #[doc = "Does not invert"]
    INV0 = 0x0,
    #[doc = "Inverts"]
    INV1 = 0x01,
}
impl Pcr9Inv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Inv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Inv {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Inv {
        Pcr9Inv::from_bits(val)
    }
}
impl From<Pcr9Inv> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Inv) -> u8 {
        Pcr9Inv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Lk {
    #[doc = "Does not lock"]
    LK0 = 0x0,
    #[doc = "Locks"]
    LK1 = 0x01,
}
impl Pcr9Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Lk {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Lk {
        Pcr9Lk::from_bits(val)
    }
}
impl From<Pcr9Lk> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Lk) -> u8 {
        Pcr9Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Mux {
    #[doc = "Alternative 0 (GPIO)"]
    MUX00 = 0x0,
    #[doc = "Alternative 1 (chip-specific)"]
    MUX01 = 0x01,
    #[doc = "Alternative 2 (chip-specific)"]
    MUX10 = 0x02,
    #[doc = "Alternative 3 (chip-specific)"]
    MUX11 = 0x03,
    #[doc = "Alternative 4 (chip-specific)"]
    MUX100 = 0x04,
    #[doc = "Alternative 5 (chip-specific)"]
    MUX101 = 0x05,
    #[doc = "Alternative 6 (chip-specific)"]
    MUX110 = 0x06,
    #[doc = "Alternative 7 (chip-specific)"]
    MUX111 = 0x07,
    #[doc = "Alternative 8 (chip-specific)"]
    MUX1000 = 0x08,
    #[doc = "Alternative 9 (chip-specific)"]
    MUX1001 = 0x09,
    #[doc = "Alternative 10 (chip-specific)"]
    MUX1010 = 0x0a,
    #[doc = "Alternative 11 (chip-specific)"]
    MUX1011 = 0x0b,
    #[doc = "Alternative 12 (chip-specific)"]
    MUX1100 = 0x0c,
    #[doc = "Alternative 13 (chip-specific)"]
    MUX1101 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pcr9Mux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Mux {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Mux {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Mux {
        Pcr9Mux::from_bits(val)
    }
}
impl From<Pcr9Mux> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Mux) -> u8 {
        Pcr9Mux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Ode {
    #[doc = "Disables"]
    ODE0 = 0x0,
    #[doc = "Enables"]
    ODE1 = 0x01,
}
impl Pcr9Ode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Ode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Ode {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Ode {
        Pcr9Ode::from_bits(val)
    }
}
impl From<Pcr9Ode> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Ode) -> u8 {
        Pcr9Ode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Pe {
    #[doc = "Disables"]
    PE0 = 0x0,
    #[doc = "Enables"]
    PE1 = 0x01,
}
impl Pcr9Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Pe {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Pe {
        Pcr9Pe::from_bits(val)
    }
}
impl From<Pcr9Pe> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Pe) -> u8 {
        Pcr9Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Pfe {
    #[doc = "Disables"]
    PFE0 = 0x0,
    #[doc = "Enables"]
    PFE1 = 0x01,
}
impl Pcr9Pfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Pfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Pfe {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Pfe {
        Pcr9Pfe::from_bits(val)
    }
}
impl From<Pcr9Pfe> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Pfe) -> u8 {
        Pcr9Pfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Ps {
    #[doc = "Enables internal pulldown resistor"]
    PS0 = 0x0,
    #[doc = "Enables internal pullup resistor"]
    PS1 = 0x01,
}
impl Pcr9Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Ps {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Ps {
        Pcr9Ps::from_bits(val)
    }
}
impl From<Pcr9Ps> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Ps) -> u8 {
        Pcr9Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcr9Sre {
    #[doc = "Fast"]
    SRE0 = 0x0,
    #[doc = "Slow"]
    SRE1 = 0x01,
}
impl Pcr9Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcr9Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcr9Sre {
    #[inline(always)]
    fn from(val: u8) -> Pcr9Sre {
        Pcr9Sre::from_bits(val)
    }
}
impl From<Pcr9Sre> for u8 {
    #[inline(always)]
    fn from(val: Pcr9Sre) -> u8 {
        Pcr9Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Range {
    #[doc = "1.71 V-3.6 V"]
    RANGE0 = 0x0,
    #[doc = "2.70 V-3.6 V"]
    RANGE1 = 0x01,
}
impl Range {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Range {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Range {
    #[inline(always)]
    fn from(val: u8) -> Range {
        Range::from_bits(val)
    }
}
impl From<Range> for u8 {
    #[inline(always)]
    fn from(val: Range) -> u8 {
        Range::to_bits(val)
    }
}
