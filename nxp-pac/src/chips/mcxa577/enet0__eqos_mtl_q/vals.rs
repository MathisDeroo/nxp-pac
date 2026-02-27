#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisTcpEf {
    #[doc = "Dropping of TCP/IP Checksum Error Packets is enabled"]
    ENABLE = 0x0,
    #[doc = "Dropping of TCP/IP Checksum Error Packets is disabled"]
    DISABLE = 0x01,
}
impl DisTcpEf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisTcpEf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisTcpEf {
    #[inline(always)]
    fn from(val: u8) -> DisTcpEf {
        DisTcpEf::from_bits(val)
    }
}
impl From<DisTcpEf> for u8 {
    #[inline(always)]
    fn from(val: DisTcpEf) -> u8 {
        DisTcpEf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rrcsts {
    #[doc = "Idle state"]
    IDLE = 0x0,
    #[doc = "Reading packet data"]
    READ_DATA = 0x01,
    #[doc = "Reading packet status (or timestamp)"]
    READ_STS = 0x02,
    #[doc = "Flushing the packet data and status"]
    FLUSH = 0x03,
}
impl Rrcsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rrcsts {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rrcsts {
    #[inline(always)]
    fn from(val: u8) -> Rrcsts {
        Rrcsts::from_bits(val)
    }
}
impl From<Rrcsts> for u8 {
    #[inline(always)]
    fn from(val: Rrcsts) -> u8 {
        Rrcsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rtc {
    #[doc = "64"]
    M_64BYTE = 0x0,
    #[doc = "32"]
    M_32BYTE = 0x01,
    #[doc = "96"]
    M_96BYTE = 0x02,
    #[doc = "128"]
    M_128BYTE = 0x03,
}
impl Rtc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtc {
    #[inline(always)]
    fn from(val: u8) -> Rtc {
        Rtc::from_bits(val)
    }
}
impl From<Rtc> for u8 {
    #[inline(always)]
    fn from(val: Rtc) -> u8 {
        Rtc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxqsts {
    #[doc = "Rx Queue empty"]
    EMPTY = 0x0,
    #[doc = "Rx Queue fill-level below flow-control deactivate threshold"]
    BLW_THR = 0x01,
    #[doc = "Rx Queue fill-level above flow-control activate threshold"]
    ABV_THR = 0x02,
    #[doc = "Rx Queue full"]
    FULL = 0x03,
}
impl Rxqsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxqsts {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxqsts {
    #[inline(always)]
    fn from(val: u8) -> Rxqsts {
        Rxqsts::from_bits(val)
    }
}
impl From<Rxqsts> for u8 {
    #[inline(always)]
    fn from(val: Rxqsts) -> u8 {
        Rxqsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trcsts {
    #[doc = "Idle state"]
    IDLE = 0x0,
    #[doc = "Read state (transferring data to the MAC transmitter)"]
    READ = 0x01,
    #[doc = "Waiting for pending Tx Status from the MAC transmitter"]
    WAIT = 0x02,
    #[doc = "Flushing the Tx queue because of the Packet Abort request from the MAC"]
    FLUSH = 0x03,
}
impl Trcsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trcsts {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trcsts {
    #[inline(always)]
    fn from(val: u8) -> Trcsts {
        Trcsts::from_bits(val)
    }
}
impl From<Trcsts> for u8 {
    #[inline(always)]
    fn from(val: Trcsts) -> u8 {
        Trcsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ttc {
    #[doc = "32"]
    M_32BYTES = 0x0,
    #[doc = "64"]
    M_64BYTES = 0x01,
    #[doc = "96"]
    M_96BYTES = 0x02,
    #[doc = "128"]
    M_128BYTES = 0x03,
    #[doc = "192"]
    M_192BYTES = 0x04,
    #[doc = "256"]
    M_256BYTES = 0x05,
    #[doc = "384"]
    M_384BYTES = 0x06,
    #[doc = "512"]
    M_512BYTES = 0x07,
}
impl Ttc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ttc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ttc {
    #[inline(always)]
    fn from(val: u8) -> Ttc {
        Ttc::from_bits(val)
    }
}
impl From<Ttc> for u8 {
    #[inline(always)]
    fn from(val: Ttc) -> u8 {
        Ttc::to_bits(val)
    }
}
