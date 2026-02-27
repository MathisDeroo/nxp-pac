#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crl {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Crl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crl {
    #[inline(always)]
    fn from(val: u8) -> Crl {
        Crl::from_bits(val)
    }
}
impl From<Crl> for u8 {
    #[inline(always)]
    fn from(val: Crl) -> u8 {
        Crl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crr {
    #[doc = "Reads to the Control register are ignored."]
    IGNORED = 0x0,
    #[doc = "Reads to the Control register complete as normal."]
    NORMAL = 0x01,
}
impl Crr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crr {
    #[inline(always)]
    fn from(val: u8) -> Crr {
        Crr::from_bits(val)
    }
}
impl From<Crr> for u8 {
    #[inline(always)]
    fn from(val: Crr) -> u8 {
        Crr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crw {
    #[doc = "Writes to the Control register are ignored."]
    IGNORED = 0x0,
    #[doc = "Writes to the Control register complete as normal."]
    NORMAL = 0x01,
}
impl Crw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crw {
    #[inline(always)]
    fn from(val: u8) -> Crw {
        Crw::from_bits(val)
    }
}
impl From<Crw> for u8 {
    #[inline(always)]
    fn from(val: Crw) -> u8 {
        Crw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Distam {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Automatically disables the prescaler after tamper detection"]
    AUTO_DIS = 0x01,
}
impl Distam {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Distam {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Distam {
    #[inline(always)]
    fn from(val: u8) -> Distam {
        Distam::from_bits(val)
    }
}
impl From<Distam> for u8 {
    #[inline(always)]
    fn from(val: Distam) -> u8 {
        Distam::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfe {
    #[doc = "Bypasses"]
    BYPASS = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Gfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfe {
    #[inline(always)]
    fn from(val: u8) -> Gfe {
        Gfe::from_bits(val)
    }
}
impl From<Gfe> for u8 {
    #[inline(always)]
    fn from(val: Gfe) -> u8 {
        Gfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfl0 {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Gfl0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfl0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfl0 {
    #[inline(always)]
    fn from(val: u8) -> Gfl0 {
        Gfl0::from_bits(val)
    }
}
impl From<Gfl0> for u8 {
    #[inline(always)]
    fn from(val: Gfl0) -> u8 {
        Gfl0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfl1 {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Gfl1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfl1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfl1 {
    #[inline(always)]
    fn from(val: u8) -> Gfl1 {
        Gfl1::from_bits(val)
    }
}
impl From<Gfl1> for u8 {
    #[inline(always)]
    fn from(val: Gfl1) -> u8 {
        Gfl1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfl2 {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Gfl2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfl2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfl2 {
    #[inline(always)]
    fn from(val: u8) -> Gfl2 {
        Gfl2::from_bits(val)
    }
}
impl From<Gfl2> for u8 {
    #[inline(always)]
    fn from(val: Gfl2) -> u8 {
        Gfl2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfl3 {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Gfl3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfl3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfl3 {
    #[inline(always)]
    fn from(val: u8) -> Gfl3 {
        Gfl3::from_bits(val)
    }
}
impl From<Gfl3> for u8 {
    #[inline(always)]
    fn from(val: Gfl3) -> u8 {
        Gfl3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfl4 {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Gfl4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfl4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfl4 {
    #[inline(always)]
    fn from(val: u8) -> Gfl4 {
        Gfl4::from_bits(val)
    }
}
impl From<Gfl4> for u8 {
    #[inline(always)]
    fn from(val: Gfl4) -> u8 {
        Gfl4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfl5 {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Gfl5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfl5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfl5 {
    #[inline(always)]
    fn from(val: u8) -> Gfl5 {
        Gfl5::from_bits(val)
    }
}
impl From<Gfl5> for u8 {
    #[inline(always)]
    fn from(val: Gfl5) -> u8 {
        Gfl5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfl6 {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Gfl6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfl6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfl6 {
    #[inline(always)]
    fn from(val: u8) -> Gfl6 {
        Gfl6::from_bits(val)
    }
}
impl From<Gfl6> for u8 {
    #[inline(always)]
    fn from(val: Gfl6) -> u8 {
        Gfl6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfl7 {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Gfl7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfl7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfl7 {
    #[inline(always)]
    fn from(val: u8) -> Gfl7 {
        Gfl7::from_bits(val)
    }
}
impl From<Gfl7> for u8 {
    #[inline(always)]
    fn from(val: Gfl7) -> u8 {
        Gfl7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfp {
    #[doc = "512 Hz prescaler clock"]
    FREQ_512_HZ = 0x0,
    #[doc = "32.768 kHz clock"]
    FREQ_32_KHZ = 0x01,
}
impl Gfp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfp {
    #[inline(always)]
    fn from(val: u8) -> Gfp {
        Gfp::from_bits(val)
    }
}
impl From<Gfp> for u8 {
    #[inline(always)]
    fn from(val: Gfp) -> u8 {
        Gfp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfr0 {
    #[doc = "Reads are ignored"]
    IGNORED = 0x0,
    #[doc = "Reads complete as normal"]
    NORMAL = 0x01,
}
impl Gfr0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfr0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfr0 {
    #[inline(always)]
    fn from(val: u8) -> Gfr0 {
        Gfr0::from_bits(val)
    }
}
impl From<Gfr0> for u8 {
    #[inline(always)]
    fn from(val: Gfr0) -> u8 {
        Gfr0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfr1 {
    #[doc = "Reads are ignored"]
    IGNORED = 0x0,
    #[doc = "Reads complete as normal"]
    NORMAL = 0x01,
}
impl Gfr1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfr1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfr1 {
    #[inline(always)]
    fn from(val: u8) -> Gfr1 {
        Gfr1::from_bits(val)
    }
}
impl From<Gfr1> for u8 {
    #[inline(always)]
    fn from(val: Gfr1) -> u8 {
        Gfr1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfr2 {
    #[doc = "Reads are ignored"]
    IGNORED = 0x0,
    #[doc = "Reads complete as normal"]
    NORMAL = 0x01,
}
impl Gfr2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfr2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfr2 {
    #[inline(always)]
    fn from(val: u8) -> Gfr2 {
        Gfr2::from_bits(val)
    }
}
impl From<Gfr2> for u8 {
    #[inline(always)]
    fn from(val: Gfr2) -> u8 {
        Gfr2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfr3 {
    #[doc = "Reads are ignored"]
    IGNORED = 0x0,
    #[doc = "Reads complete as normal"]
    NORMAL = 0x01,
}
impl Gfr3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfr3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfr3 {
    #[inline(always)]
    fn from(val: u8) -> Gfr3 {
        Gfr3::from_bits(val)
    }
}
impl From<Gfr3> for u8 {
    #[inline(always)]
    fn from(val: Gfr3) -> u8 {
        Gfr3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfr4 {
    #[doc = "Reads are ignored"]
    IGNORED = 0x0,
    #[doc = "Reads complete as normal"]
    NORMAL = 0x01,
}
impl Gfr4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfr4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfr4 {
    #[inline(always)]
    fn from(val: u8) -> Gfr4 {
        Gfr4::from_bits(val)
    }
}
impl From<Gfr4> for u8 {
    #[inline(always)]
    fn from(val: Gfr4) -> u8 {
        Gfr4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfr5 {
    #[doc = "Reads are ignored"]
    IGNORED = 0x0,
    #[doc = "Reads complete as normal"]
    NORMAL = 0x01,
}
impl Gfr5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfr5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfr5 {
    #[inline(always)]
    fn from(val: u8) -> Gfr5 {
        Gfr5::from_bits(val)
    }
}
impl From<Gfr5> for u8 {
    #[inline(always)]
    fn from(val: Gfr5) -> u8 {
        Gfr5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfr6 {
    #[doc = "Reads are ignored"]
    IGNORED = 0x0,
    #[doc = "Reads complete as normal"]
    NORMAL = 0x01,
}
impl Gfr6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfr6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfr6 {
    #[inline(always)]
    fn from(val: u8) -> Gfr6 {
        Gfr6::from_bits(val)
    }
}
impl From<Gfr6> for u8 {
    #[inline(always)]
    fn from(val: Gfr6) -> u8 {
        Gfr6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfr7 {
    #[doc = "Reads are ignored"]
    IGNORED = 0x0,
    #[doc = "Reads complete as normal"]
    NORMAL = 0x01,
}
impl Gfr7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfr7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfr7 {
    #[inline(always)]
    fn from(val: u8) -> Gfr7 {
        Gfr7::from_bits(val)
    }
}
impl From<Gfr7> for u8 {
    #[inline(always)]
    fn from(val: Gfr7) -> u8 {
        Gfr7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfw0 {
    #[doc = "Writes are ignored"]
    IGNORED = 0x0,
    #[doc = "Writes complete as normal"]
    NORMAL = 0x01,
}
impl Gfw0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfw0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfw0 {
    #[inline(always)]
    fn from(val: u8) -> Gfw0 {
        Gfw0::from_bits(val)
    }
}
impl From<Gfw0> for u8 {
    #[inline(always)]
    fn from(val: Gfw0) -> u8 {
        Gfw0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfw1 {
    #[doc = "Writes are ignored"]
    IGNORED = 0x0,
    #[doc = "Writes complete as normal"]
    NORMAL = 0x01,
}
impl Gfw1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfw1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfw1 {
    #[inline(always)]
    fn from(val: u8) -> Gfw1 {
        Gfw1::from_bits(val)
    }
}
impl From<Gfw1> for u8 {
    #[inline(always)]
    fn from(val: Gfw1) -> u8 {
        Gfw1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfw2 {
    #[doc = "Writes are ignored"]
    IGNORED = 0x0,
    #[doc = "Writes complete as normal"]
    NORMAL = 0x01,
}
impl Gfw2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfw2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfw2 {
    #[inline(always)]
    fn from(val: u8) -> Gfw2 {
        Gfw2::from_bits(val)
    }
}
impl From<Gfw2> for u8 {
    #[inline(always)]
    fn from(val: Gfw2) -> u8 {
        Gfw2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfw3 {
    #[doc = "Writes are ignored"]
    IGNORED = 0x0,
    #[doc = "Writes complete as normal"]
    NORMAL = 0x01,
}
impl Gfw3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfw3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfw3 {
    #[inline(always)]
    fn from(val: u8) -> Gfw3 {
        Gfw3::from_bits(val)
    }
}
impl From<Gfw3> for u8 {
    #[inline(always)]
    fn from(val: Gfw3) -> u8 {
        Gfw3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfw4 {
    #[doc = "Writes are ignored"]
    IGNORED = 0x0,
    #[doc = "Writes complete as normal"]
    NORMAL = 0x01,
}
impl Gfw4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfw4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfw4 {
    #[inline(always)]
    fn from(val: u8) -> Gfw4 {
        Gfw4::from_bits(val)
    }
}
impl From<Gfw4> for u8 {
    #[inline(always)]
    fn from(val: Gfw4) -> u8 {
        Gfw4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfw5 {
    #[doc = "Writes are ignored"]
    IGNORED = 0x0,
    #[doc = "Writes complete as normal"]
    NORMAL = 0x01,
}
impl Gfw5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfw5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfw5 {
    #[inline(always)]
    fn from(val: u8) -> Gfw5 {
        Gfw5::from_bits(val)
    }
}
impl From<Gfw5> for u8 {
    #[inline(always)]
    fn from(val: Gfw5) -> u8 {
        Gfw5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfw6 {
    #[doc = "Writes are ignored"]
    IGNORED = 0x0,
    #[doc = "Writes complete as normal"]
    NORMAL = 0x01,
}
impl Gfw6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfw6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfw6 {
    #[inline(always)]
    fn from(val: u8) -> Gfw6 {
        Gfw6::from_bits(val)
    }
}
impl From<Gfw6> for u8 {
    #[inline(always)]
    fn from(val: Gfw6) -> u8 {
        Gfw6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gfw7 {
    #[doc = "Writes are ignored"]
    IGNORED = 0x0,
    #[doc = "Writes complete as normal"]
    NORMAL = 0x01,
}
impl Gfw7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gfw7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gfw7 {
    #[inline(always)]
    fn from(val: u8) -> Gfw7 {
        Gfw7::from_bits(val)
    }
}
impl From<Gfw7> for u8 {
    #[inline(always)]
    fn from(val: Gfw7) -> u8 {
        Gfw7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iel {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Iel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iel {
    #[inline(always)]
    fn from(val: u8) -> Iel {
        Iel::from_bits(val)
    }
}
impl From<Iel> for u8 {
    #[inline(always)]
    fn from(val: Iel) -> u8 {
        Iel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ier {
    #[doc = "Reads to the Interrupt enable register are ignored."]
    IGNORED = 0x0,
    #[doc = "Reads to the Interrupt enable register complete as normal."]
    NORMAL = 0x01,
}
impl Ier {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ier {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ier {
    #[inline(always)]
    fn from(val: u8) -> Ier {
        Ier::from_bits(val)
    }
}
impl From<Ier> for u8 {
    #[inline(always)]
    fn from(val: Ier) -> u8 {
        Ier::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iew {
    #[doc = "Writes to the Interrupt enable register are ignored."]
    IGNORED = 0x0,
    #[doc = "Writes to the Interrupt enable register complete as normal."]
    NORMAL = 0x01,
}
impl Iew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iew {
    #[inline(always)]
    fn from(val: u8) -> Iew {
        Iew::from_bits(val)
    }
}
impl From<Iew> for u8 {
    #[inline(always)]
    fn from(val: Iew) -> u8 {
        Iew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lrl {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Lrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lrl {
    #[inline(always)]
    fn from(val: u8) -> Lrl {
        Lrl::from_bits(val)
    }
}
impl From<Lrl> for u8 {
    #[inline(always)]
    fn from(val: Lrl) -> u8 {
        Lrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lrr {
    #[doc = "Reads to the Lock register are ignored."]
    IGNORED = 0x0,
    #[doc = "Reads to the Lock register complete as normal."]
    NORMAL = 0x01,
}
impl Lrr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lrr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lrr {
    #[inline(always)]
    fn from(val: u8) -> Lrr {
        Lrr::from_bits(val)
    }
}
impl From<Lrr> for u8 {
    #[inline(always)]
    fn from(val: Lrr) -> u8 {
        Lrr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lrw {
    #[doc = "Writes to the Lock register are ignored."]
    IGNORED = 0x0,
    #[doc = "Writes to the Lock register complete as normal."]
    NORMAL = 0x01,
}
impl Lrw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lrw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lrw {
    #[inline(always)]
    fn from(val: u8) -> Lrw {
        Lrw::from_bits(val)
    }
}
impl From<Lrw> for u8 {
    #[inline(always)]
    fn from(val: Lrw) -> u8 {
        Lrw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ppl {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Ppl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ppl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ppl {
    #[inline(always)]
    fn from(val: u8) -> Ppl {
        Ppl::from_bits(val)
    }
}
impl From<Ppl> for u8 {
    #[inline(always)]
    fn from(val: Ppl) -> u8 {
        Ppl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ppr {
    #[doc = "Reads to the pin polarity register are ignored."]
    IGNORED = 0x0,
    #[doc = "Reads to the pin polarity register complete as normal."]
    NORMAL = 0x01,
}
impl Ppr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ppr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ppr {
    #[inline(always)]
    fn from(val: u8) -> Ppr {
        Ppr::from_bits(val)
    }
}
impl From<Ppr> for u8 {
    #[inline(always)]
    fn from(val: Ppr) -> u8 {
        Ppr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ppw {
    #[doc = "Writes to the pin polarity register are ignored."]
    IGNORED = 0x0,
    #[doc = "Writes to the pin polarity register complete as normal."]
    NORMAL = 0x01,
}
impl Ppw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ppw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ppw {
    #[inline(always)]
    fn from(val: u8) -> Ppw {
        Ppw::from_bits(val)
    }
}
impl From<Ppw> for u8 {
    #[inline(always)]
    fn from(val: Ppw) -> u8 {
        Ppw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Srl {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Srl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srl {
    #[inline(always)]
    fn from(val: u8) -> Srl {
        Srl::from_bits(val)
    }
}
impl From<Srl> for u8 {
    #[inline(always)]
    fn from(val: Srl) -> u8 {
        Srl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Srr {
    #[doc = "Reads to the Status Register are ignored."]
    IGNORED = 0x0,
    #[doc = "Reads to the Status Register complete as normal."]
    NORMAL = 0x01,
}
impl Srr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srr {
    #[inline(always)]
    fn from(val: u8) -> Srr {
        Srr::from_bits(val)
    }
}
impl From<Srr> for u8 {
    #[inline(always)]
    fn from(val: Srr) -> u8 {
        Srr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Srw {
    #[doc = "Writes to the Status Register are ignored."]
    IGNORED = 0x0,
    #[doc = "Writes to the Status Register complete as normal."]
    NORMAL = 0x01,
}
impl Srw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srw {
    #[inline(always)]
    fn from(val: u8) -> Srw {
        Srw::from_bits(val)
    }
}
impl From<Srw> for u8 {
    #[inline(always)]
    fn from(val: Srw) -> u8 {
        Srw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swr {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Perform a software reset"]
    SW_RESET = 0x01,
}
impl Swr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swr {
    #[inline(always)]
    fn from(val: u8) -> Swr {
        Swr::from_bits(val)
    }
}
impl From<Swr> for u8 {
    #[inline(always)]
    fn from(val: Swr) -> u8 {
        Swr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tel {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Tel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tel {
    #[inline(always)]
    fn from(val: u8) -> Tel {
        Tel::from_bits(val)
    }
}
impl From<Tel> for u8 {
    #[inline(always)]
    fn from(val: Tel) -> u8 {
        Tel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ter {
    #[doc = "Reads to the tamper enable register are ignored."]
    IGNORED = 0x0,
    #[doc = "Reads to the tamper enable register complete as normal."]
    NORMAL = 0x01,
}
impl Ter {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ter {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ter {
    #[inline(always)]
    fn from(val: u8) -> Ter {
        Ter::from_bits(val)
    }
}
impl From<Ter> for u8 {
    #[inline(always)]
    fn from(val: Ter) -> u8 {
        Ter::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tew {
    #[doc = "Writes to the tamper enable register are ignored."]
    IGNORED = 0x0,
    #[doc = "Writes to the tamper enable register complete as normal."]
    NORMAL = 0x01,
}
impl Tew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tew {
    #[inline(always)]
    fn from(val: u8) -> Tew {
        Tew::from_bits(val)
    }
}
impl From<Tew> for u8 {
    #[inline(always)]
    fn from(val: Tew) -> u8 {
        Tew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tps {
    #[doc = "Asserts"]
    ASSERT = 0x0,
    #[doc = "Negates"]
    NEGATE = 0x01,
}
impl Tps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tps {
    #[inline(always)]
    fn from(val: u8) -> Tps {
        Tps::from_bits(val)
    }
}
impl From<Tps> for u8 {
    #[inline(always)]
    fn from(val: Tps) -> u8 {
        Tps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpsf {
    #[doc = "Every 8 cycles"]
    CYCLES_8 = 0x0,
    #[doc = "Every 32 cycles"]
    CYCLES_32 = 0x01,
    #[doc = "Every 128 cycles"]
    CYCLES_128 = 0x02,
    #[doc = "Every 512 cycles"]
    CYCLES_512 = 0x03,
}
impl Tpsf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpsf {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpsf {
    #[inline(always)]
    fn from(val: u8) -> Tpsf {
        Tpsf::from_bits(val)
    }
}
impl From<Tpsf> for u8 {
    #[inline(always)]
    fn from(val: Tpsf) -> u8 {
        Tpsf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpsw {
    #[doc = "Continuous monitoring, pin sampling disabled"]
    DISABLE = 0x0,
    #[doc = "2 cycles for pull enable and 1 cycle for input buffer enable"]
    CYCLES_2 = 0x01,
    #[doc = "4 cycles for pull enable and 2 cycles for input buffer enable"]
    CYCLES_4 = 0x02,
    #[doc = "8 cycles for pull enable and 4 cycles for input buffer enable"]
    CYCLES_8 = 0x03,
}
impl Tpsw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpsw {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpsw {
    #[inline(always)]
    fn from(val: u8) -> Tpsw {
        Tpsw::from_bits(val)
    }
}
impl From<Tpsw> for u8 {
    #[inline(always)]
    fn from(val: Tpsw) -> u8 {
        Tpsw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsl {
    #[doc = "Locked and writes are ignored"]
    LOCK = 0x0,
    #[doc = "Not locked and writes complete as normal"]
    NOT_LOCK = 0x01,
}
impl Tsl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsl {
    #[inline(always)]
    fn from(val: u8) -> Tsl {
        Tsl::from_bits(val)
    }
}
impl From<Tsl> for u8 {
    #[inline(always)]
    fn from(val: Tsl) -> u8 {
        Tsl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsrr {
    #[doc = "Reads to the tamper seconds register are ignored."]
    IGNORED = 0x0,
    #[doc = "Reads to the tamper seconds register complete as normal."]
    NORMAL = 0x01,
}
impl Tsrr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsrr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsrr {
    #[inline(always)]
    fn from(val: u8) -> Tsrr {
        Tsrr::from_bits(val)
    }
}
impl From<Tsrr> for u8 {
    #[inline(always)]
    fn from(val: Tsrr) -> u8 {
        Tsrr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsrw {
    #[doc = "Writes to the Tamper Seconds register are ignored."]
    IGNORED = 0x0,
    #[doc = "Writes to the Tamper Seconds register complete as normal."]
    NORMAL = 0x01,
}
impl Tsrw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsrw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsrw {
    #[inline(always)]
    fn from(val: u8) -> Tsrw {
        Tsrw::from_bits(val)
    }
}
impl From<Tsrw> for u8 {
    #[inline(always)]
    fn from(val: Tsrw) -> u8 {
        Tsrw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Um {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Allows the clearing of interrupts"]
    CLEAR_INTS = 0x01,
}
impl Um {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Um {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Um {
    #[inline(always)]
    fn from(val: u8) -> Um {
        Um::from_bits(val)
    }
}
impl From<Um> for u8 {
    #[inline(always)]
    fn from(val: Um) -> u8 {
        Um::to_bits(val)
    }
}
