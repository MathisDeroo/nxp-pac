#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Da {
    #[doc = "Weighted Round-Robin with Rx:Tx or Tx:Rx"]
    WRR = 0x0,
    #[doc = "Fixed Priority"]
    FP = 0x01,
}
impl Da {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Da {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Da {
    #[inline(always)]
    fn from(val: u8) -> Da {
        Da::from_bits(val)
    }
}
impl From<Da> for u8 {
    #[inline(always)]
    fn from(val: Da) -> u8 {
        Da::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intm {
    #[doc = "See above description"]
    MODE0 = 0x0,
    #[doc = "See above description"]
    MODE1 = 0x01,
    #[doc = "See above description"]
    MODE2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Intm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intm {
    #[inline(always)]
    fn from(val: u8) -> Intm {
        Intm::from_bits(val)
    }
}
impl From<Intm> for u8 {
    #[inline(always)]
    fn from(val: Intm) -> u8 {
        Intm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pr {
    #[doc = "The priority ratio is 1:1"]
    R_1_1 = 0x0,
    #[doc = "The priority ratio is 2:1"]
    R_2_1 = 0x01,
    #[doc = "The priority ratio is 3:1"]
    R_3_1 = 0x02,
    #[doc = "The priority ratio is 4:1"]
    R_4_1 = 0x03,
    #[doc = "The priority ratio is 5:1"]
    R_5_1 = 0x04,
    #[doc = "The priority ratio is 6:1"]
    R_6_1 = 0x05,
    #[doc = "The priority ratio is 7:1"]
    R_7_1 = 0x06,
    #[doc = "The priority ratio is 8:1"]
    R_8_1 = 0x07,
}
impl Pr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pr {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pr {
    #[inline(always)]
    fn from(val: u8) -> Pr {
        Pr::from_bits(val)
    }
}
impl From<Pr> for u8 {
    #[inline(always)]
    fn from(val: Pr) -> u8 {
        Pr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rps0 {
    #[doc = "Stopped (Reset or Stop Receive Command issued)"]
    STOP = 0x0,
    #[doc = "Running (Fetching Rx Transfer Descriptor)"]
    RUN_FRTD = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Running (Waiting for Rx packet)"]
    RUN_WRP = 0x03,
    #[doc = "Suspended (Rx Descriptor Unavailable)"]
    SUSPND = 0x04,
    #[doc = "Running (Closing the Rx Descriptor)"]
    RUN_CRD = 0x05,
    #[doc = "Timestamp write state"]
    TSTMP = 0x06,
    #[doc = "Running (Transferring the received packet data from the Rx buffer to the system memory)"]
    RUN_TRP = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Rps0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rps0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rps0 {
    #[inline(always)]
    fn from(val: u8) -> Rps0 {
        Rps0::from_bits(val)
    }
}
impl From<Rps0> for u8 {
    #[inline(always)]
    fn from(val: Rps0) -> u8 {
        Rps0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tps0 {
    #[doc = "Stopped (Reset or Stop Transmit Command issued)"]
    STOP = 0x0,
    #[doc = "Running (Fetching Tx Transfer Descriptor)"]
    RUN_FTTD = 0x01,
    #[doc = "Running (Waiting for status)"]
    RUN_WS = 0x02,
    #[doc = "Running (Reading Data from system memory buffer and queuing it to the Tx buffer (Tx FIFO))"]
    RUN_RDS = 0x03,
    #[doc = "Timestamp write state"]
    TSTMP_WS = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Suspended (Tx Descriptor Unavailable or Tx Buffer Underflow)"]
    SUSPND = 0x06,
    #[doc = "Running (Closing Tx Descriptor)"]
    RUN_CTD = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Tps0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tps0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tps0 {
    #[inline(always)]
    fn from(val: u8) -> Tps0 {
        Tps0::from_bits(val)
    }
}
impl From<Tps0> for u8 {
    #[inline(always)]
    fn from(val: Tps0) -> u8 {
        Tps0::to_bits(val)
    }
}
