#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sr {
    #[doc = "Stop Receive"]
    STOP = 0x0,
    #[doc = "Start Receive"]
    START = 0x01,
}
impl Sr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sr {
    #[inline(always)]
    fn from(val: u8) -> Sr {
        Sr::from_bits(val)
    }
}
impl From<Sr> for u8 {
    #[inline(always)]
    fn from(val: Sr) -> u8 {
        Sr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum St {
    #[doc = "Stop Transmission Command"]
    STOP = 0x0,
    #[doc = "Start Transmission Command"]
    START = 0x01,
}
impl St {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> St {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for St {
    #[inline(always)]
    fn from(val: u8) -> St {
        St::from_bits(val)
    }
}
impl From<St> for u8 {
    #[inline(always)]
    fn from(val: St) -> u8 {
        St::to_bits(val)
    }
}
