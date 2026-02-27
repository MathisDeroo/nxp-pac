#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcClkselMux {
    #[doc = "FRO_LF_DIV"]
    I0_CLKROOT_SIRC_DIV = 0x0,
    #[doc = "FRO_HF_GATED"]
    I1_CLKROOT_FIRC_GATED = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "CLK_IN"]
    I3_CLKROOT_SOSC = 0x03,
    #[doc = "USB_PFD_CLK"]
    I4_CLKROOT_USBPFD = 0x04,
    #[doc = "CLK_1M"]
    I5_CLKROOT_1M = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    I6_CLKROOT_SPLL_DIV = 0x06,
    _RESERVED_7 = 0x07,
}
impl AdcClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcClkselMux {
    #[inline(always)]
    fn from(val: u8) -> AdcClkselMux {
        AdcClkselMux::from_bits(val)
    }
}
impl From<AdcClkselMux> for u8 {
    #[inline(always)]
    fn from(val: AdcClkselMux) -> u8 {
        AdcClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkdivHalt {
    #[doc = "Divider clock is running"]
    ON = 0x0,
    #[doc = "Divider clock is stopped"]
    OFF = 0x01,
}
impl ClkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> ClkdivHalt {
        ClkdivHalt::from_bits(val)
    }
}
impl From<ClkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: ClkdivHalt) -> u8 {
        ClkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkdivReset {
    #[doc = "Divider isn't reset"]
    ON = 0x0,
    #[doc = "Divider is reset"]
    OFF = 0x01,
}
impl ClkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkdivReset {
    #[inline(always)]
    fn from(val: u8) -> ClkdivReset {
        ClkdivReset::from_bits(val)
    }
}
impl From<ClkdivReset> for u8 {
    #[inline(always)]
    fn from(val: ClkdivReset) -> u8 {
        ClkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkdivUnstab {
    #[doc = "Divider clock is stable"]
    ON = 0x0,
    #[doc = "Clock frequency isn't stable"]
    OFF = 0x01,
}
impl ClkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> ClkdivUnstab {
        ClkdivUnstab::from_bits(val)
    }
}
impl From<ClkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: ClkdivUnstab) -> u8 {
        ClkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkoutClkselMux {
    #[doc = "FRO_12M"]
    I0_CLKROOT_12M = 0x0,
    #[doc = "FRO_HF_DIV"]
    I1_CLKROOT_FIRC_DIV = 0x01,
    #[doc = "CLK_IN"]
    I2_CLKROOT_SOSC = 0x02,
    #[doc = "LP_OSC"]
    I3_CLKROOT_LPOSC = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "PLL1_CLK_DIV"]
    I5_CLKROOT_SPLL_DIV = 0x05,
    #[doc = "SLOW_CLK"]
    I6_CLKROOT_SLOW = 0x06,
    _RESERVED_7 = 0x07,
}
impl ClkoutClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkoutClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkoutClkselMux {
    #[inline(always)]
    fn from(val: u8) -> ClkoutClkselMux {
        ClkoutClkselMux::from_bits(val)
    }
}
impl From<ClkoutClkselMux> for u8 {
    #[inline(always)]
    fn from(val: ClkoutClkselMux) -> u8 {
        ClkoutClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtimerClkselMux {
    #[doc = "FRO_LF_DIV"]
    I0_CLKROOT_SIRC_DIV = 0x0,
    #[doc = "FRO_HF_GATED"]
    I1_CLKROOT_FIRC_GATED = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "CLK_IN"]
    I3_CLKROOT_SOSC = 0x03,
    #[doc = "LP_OSC"]
    I4_CLKROOT_LPOSC = 0x04,
    #[doc = "CLK_1M"]
    I5_CLKROOT_1M = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    I6_CLKROOT_SPLL_DIV = 0x06,
    _RESERVED_7 = 0x07,
}
impl CtimerClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtimerClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtimerClkselMux {
    #[inline(always)]
    fn from(val: u8) -> CtimerClkselMux {
        CtimerClkselMux::from_bits(val)
    }
}
impl From<CtimerClkselMux> for u8 {
    #[inline(always)]
    fn from(val: CtimerClkselMux) -> u8 {
        CtimerClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DacClkselMux {
    #[doc = "FRO_LF_DIV"]
    I0_CLKROOT_FUNC_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV"]
    I2_CLKROOT_FUNC_2 = 0x02,
    #[doc = "CLK_IN"]
    I3_CLKROOT_FUNC_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M"]
    I5_CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    I6_CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl DacClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DacClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DacClkselMux {
    #[inline(always)]
    fn from(val: u8) -> DacClkselMux {
        DacClkselMux::from_bits(val)
    }
}
impl From<DacClkselMux> for u8 {
    #[inline(always)]
    fn from(val: DacClkselMux) -> u8 {
        DacClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DbgTraceClkselMux {
    #[doc = "CPU_CLK"]
    I0_CLKROOT_CPU_ALIAS = 0x0,
    #[doc = "CLK_1M"]
    I1_CLKROOT_1M = 0x01,
    #[doc = "CLK_16K"]
    I2_CLKROOT_16K = 0x02,
    _RESERVED_3 = 0x03,
}
impl DbgTraceClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DbgTraceClkselMux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DbgTraceClkselMux {
    #[inline(always)]
    fn from(val: u8) -> DbgTraceClkselMux {
        DbgTraceClkselMux::from_bits(val)
    }
}
impl From<DbgTraceClkselMux> for u8 {
    #[inline(always)]
    fn from(val: DbgTraceClkselMux) -> u8 {
        DbgTraceClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum E158ClkselMux {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "CLK_IN"]
    I3_CLKROOT_SOSC = 0x03,
    #[doc = "ENET0_TX_CLK"]
    I4_IPP__ENET0__CLK_TX_I = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "PLL1_CLK"]
    I6_CLKROOT_SPLL = 0x06,
    _RESERVED_7 = 0x07,
}
impl E158ClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> E158ClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for E158ClkselMux {
    #[inline(always)]
    fn from(val: u8) -> E158ClkselMux {
        E158ClkselMux::from_bits(val)
    }
}
impl From<E158ClkselMux> for u8 {
    #[inline(always)]
    fn from(val: E158ClkselMux) -> u8 {
        E158ClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EspiClkselMux {
    _RESERVED_0 = 0x0,
    #[doc = "FRO_HF_GATED"]
    I1_CLKROOT_FIRC_GATED = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "CLK_IN"]
    I3_CLKROOT_SOSC = 0x03,
    #[doc = "USB_PLL_CLK"]
    I4_CLKROOT_USBPLL = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    I6_CLKROOT_SPLL_DIV = 0x06,
    _RESERVED_7 = 0x07,
}
impl EspiClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EspiClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EspiClkselMux {
    #[inline(always)]
    fn from(val: u8) -> EspiClkselMux {
        EspiClkselMux::from_bits(val)
    }
}
impl From<EspiClkselMux> for u8 {
    #[inline(always)]
    fn from(val: EspiClkselMux) -> u8 {
        EspiClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FclkClkselMux {
    #[doc = "FRO_LF_DIV"]
    I0_CLKROOT_FUNC_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV"]
    I2_CLKROOT_FUNC_2 = 0x02,
    #[doc = "CLK_IN"]
    I3_CLKROOT_FUNC_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M"]
    I5_CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    I6_CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl FclkClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FclkClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FclkClkselMux {
    #[inline(always)]
    fn from(val: u8) -> FclkClkselMux {
        FclkClkselMux::from_bits(val)
    }
}
impl From<FclkClkselMux> for u8 {
    #[inline(always)]
    fn from(val: FclkClkselMux) -> u8 {
        FclkClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexcanClkselMux {
    _RESERVED_0 = 0x0,
    #[doc = "FRO_HF_GATED"]
    I1_CLKROOT_FIRC_GATED = 0x01,
    #[doc = "FRO_HF_DIV"]
    I2_CLKROOT_FIRC_DIV = 0x02,
    #[doc = "CLK_IN"]
    I3_CLKROOT_SOSC = 0x03,
    #[doc = "USB_PLL_CLK"]
    I4_CLKROOT_USBPLL = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "PLL1_CLK"]
    I6_CLKROOT_SPLL = 0x06,
    _RESERVED_7 = 0x07,
}
impl FlexcanClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexcanClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexcanClkselMux {
    #[inline(always)]
    fn from(val: u8) -> FlexcanClkselMux {
        FlexcanClkselMux::from_bits(val)
    }
}
impl From<FlexcanClkselMux> for u8 {
    #[inline(always)]
    fn from(val: FlexcanClkselMux) -> u8 {
        FlexcanClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexioClkselMux {
    #[doc = "FRO_LF_DIV"]
    I0_CLKROOT_FUNC_0 = 0x0,
    #[doc = "FRO_HF_GATED"]
    I1_CLKROOT_FUNC_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "CLK_IN"]
    I3_CLKROOT_FUNC_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M"]
    I5_CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    I6_CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl FlexioClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexioClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexioClkselMux {
    #[inline(always)]
    fn from(val: u8) -> FlexioClkselMux {
        FlexioClkselMux::from_bits(val)
    }
}
impl From<FlexioClkselMux> for u8 {
    #[inline(always)]
    fn from(val: FlexioClkselMux) -> u8 {
        FlexioClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspiClkselMux {
    _RESERVED_0 = 0x0,
    #[doc = "FRO_HF_GATED"]
    I1_CLKROOT_FIRC_GATED = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "USB_PFD_CLK"]
    I4_CLKROOT_USBPFD = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "PLL1_CLK"]
    I6_CLKROOT_SPLL = 0x06,
    _RESERVED_7 = 0x07,
}
impl FlexspiClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspiClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspiClkselMux {
    #[inline(always)]
    fn from(val: u8) -> FlexspiClkselMux {
        FlexspiClkselMux::from_bits(val)
    }
}
impl From<FlexspiClkselMux> for u8 {
    #[inline(always)]
    fn from(val: FlexspiClkselMux) -> u8 {
        FlexspiClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2cClkselMux {
    #[doc = "FRO_LF_DIV"]
    I0_CLKROOT_FUNC_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV"]
    I2_CLKROOT_FUNC_2 = 0x02,
    #[doc = "CLK_IN"]
    I3_CLKROOT_FUNC_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M"]
    I5_CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    I6_CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Lpi2cClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2cClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2cClkselMux {
    #[inline(always)]
    fn from(val: u8) -> Lpi2cClkselMux {
        Lpi2cClkselMux::from_bits(val)
    }
}
impl From<Lpi2cClkselMux> for u8 {
    #[inline(always)]
    fn from(val: Lpi2cClkselMux) -> u8 {
        Lpi2cClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpspiClkselMux {
    #[doc = "FRO_LF_DIV"]
    I0_CLKROOT_FUNC_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV"]
    I2_CLKROOT_FUNC_2 = 0x02,
    #[doc = "CLK_IN"]
    I3_CLKROOT_FUNC_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M"]
    I5_CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    I6_CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LpspiClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpspiClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpspiClkselMux {
    #[inline(always)]
    fn from(val: u8) -> LpspiClkselMux {
        LpspiClkselMux::from_bits(val)
    }
}
impl From<LpspiClkselMux> for u8 {
    #[inline(always)]
    fn from(val: LpspiClkselMux) -> u8 {
        LpspiClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LptmrClkselMux {
    #[doc = "FRO_LF_DIV"]
    I0_CLKROOT_FUNC_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV"]
    I2_CLKROOT_FUNC_2 = 0x02,
    #[doc = "CLK_IN"]
    I3_CLKROOT_FUNC_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M"]
    I5_CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    I6_CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LptmrClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LptmrClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LptmrClkselMux {
    #[inline(always)]
    fn from(val: u8) -> LptmrClkselMux {
        LptmrClkselMux::from_bits(val)
    }
}
impl From<LptmrClkselMux> for u8 {
    #[inline(always)]
    fn from(val: LptmrClkselMux) -> u8 {
        LptmrClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpuartClkselMux {
    #[doc = "FRO_LF_DIV"]
    I0_CLKROOT_SIRC_DIV = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV"]
    I2_CLKROOT_FIRC_DIV = 0x02,
    #[doc = "CLK_IN"]
    I3_CLKROOT_SOSC = 0x03,
    #[doc = "LP_OSC"]
    I4_CLKROOT_LPOSC = 0x04,
    #[doc = "CLK_1M"]
    I5_CLKROOT_1M = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    I6_CLKROOT_SPLL_DIV = 0x06,
    _RESERVED_7 = 0x07,
}
impl LpuartClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpuartClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpuartClkselMux {
    #[inline(always)]
    fn from(val: u8) -> LpuartClkselMux {
        LpuartClkselMux::from_bits(val)
    }
}
impl From<LpuartClkselMux> for u8 {
    #[inline(always)]
    fn from(val: LpuartClkselMux) -> u8 {
        LpuartClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OstimerClkselMux {
    #[doc = "CLK_16K"]
    I0_CLKROOT_16K = 0x0,
    #[doc = "CLK_32K"]
    I1_CLKROOT_32K = 0x01,
    #[doc = "CLK_1M"]
    I2_CLKROOT_1M = 0x02,
    _RESERVED_3 = 0x03,
}
impl OstimerClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OstimerClkselMux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OstimerClkselMux {
    #[inline(always)]
    fn from(val: u8) -> OstimerClkselMux {
        OstimerClkselMux::from_bits(val)
    }
}
impl From<OstimerClkselMux> for u8 {
    #[inline(always)]
    fn from(val: OstimerClkselMux) -> u8 {
        OstimerClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhyClkselMux {
    _RESERVED_0 = 0x0,
    #[doc = "FRO_HF_GATED"]
    I1_CLKROOT_FIRC_GATED = 0x01,
    #[doc = "CLK_IN"]
    I2_CLKROOT_SOSC = 0x02,
    _RESERVED_3 = 0x03,
}
impl PhyClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhyClkselMux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhyClkselMux {
    #[inline(always)]
    fn from(val: u8) -> PhyClkselMux {
        PhyClkselMux::from_bits(val)
    }
}
impl From<PhyClkselMux> for u8 {
    #[inline(always)]
    fn from(val: PhyClkselMux) -> u8 {
        PhyClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RmiiClkselMux {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "CLK_IN"]
    I3_CLKROOT_SOSC = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "PLL1_CLK"]
    I6_CLKROOT_SPLL = 0x06,
    _RESERVED_7 = 0x07,
}
impl RmiiClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RmiiClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RmiiClkselMux {
    #[inline(always)]
    fn from(val: u8) -> RmiiClkselMux {
        RmiiClkselMux::from_bits(val)
    }
}
impl From<RmiiClkselMux> for u8 {
    #[inline(always)]
    fn from(val: RmiiClkselMux) -> u8 {
        RmiiClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RrClkselMux {
    #[doc = "FRO_LF_DIV"]
    I0_CLKROOT_FUNC_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV"]
    I2_CLKROOT_FUNC_2 = 0x02,
    #[doc = "CLK_IN"]
    I3_CLKROOT_FUNC_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M"]
    I5_CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    I6_CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl RrClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RrClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RrClkselMux {
    #[inline(always)]
    fn from(val: u8) -> RrClkselMux {
        RrClkselMux::from_bits(val)
    }
}
impl From<RrClkselMux> for u8 {
    #[inline(always)]
    fn from(val: RrClkselMux) -> u8 {
        RrClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SystickClkselMux {
    #[doc = "CPU_CLK"]
    I0_CLKROOT_CPU = 0x0,
    #[doc = "CLK_1M"]
    I1_CLKROOT_1M = 0x01,
    #[doc = "CLK_16K"]
    I2_CLKROOT_16K = 0x02,
    _RESERVED_3 = 0x03,
}
impl SystickClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SystickClkselMux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SystickClkselMux {
    #[inline(always)]
    fn from(val: u8) -> SystickClkselMux {
        SystickClkselMux::from_bits(val)
    }
}
impl From<SystickClkselMux> for u8 {
    #[inline(always)]
    fn from(val: SystickClkselMux) -> u8 {
        SystickClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum T1sClkselMux {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "CLK_IN"]
    I3_CLKROOT_SOSC = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "PLL1_CLK"]
    I6_CLKROOT_SPLL = 0x06,
    _RESERVED_7 = 0x07,
}
impl T1sClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> T1sClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for T1sClkselMux {
    #[inline(always)]
    fn from(val: u8) -> T1sClkselMux {
        T1sClkselMux::from_bits(val)
    }
}
impl From<T1sClkselMux> for u8 {
    #[inline(always)]
    fn from(val: T1sClkselMux) -> u8 {
        T1sClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TsiClkselMux {
    #[doc = "FRO_LF_DIV"]
    I0_CLKROOT_FUNC_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "FRO_HF_DIV"]
    I2_CLKROOT_FUNC_2 = 0x02,
    #[doc = "CLK_IN"]
    I3_CLKROOT_FUNC_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CLK_1M"]
    I5_CLKROOT_FUNC_5 = 0x05,
    #[doc = "PLL1_CLK_DIV"]
    I6_CLKROOT_FUNC_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl TsiClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TsiClkselMux {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TsiClkselMux {
    #[inline(always)]
    fn from(val: u8) -> TsiClkselMux {
        TsiClkselMux::from_bits(val)
    }
}
impl From<TsiClkselMux> for u8 {
    #[inline(always)]
    fn from(val: TsiClkselMux) -> u8 {
        TsiClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbClkselMux {
    #[doc = "CLK_32K"]
    I0_CLKROOT_32K = 0x0,
    #[doc = "CLK_1M"]
    I1_CLKROOT_1M = 0x01,
    #[doc = "CLK__usbhs0_phy__clk_xtal"]
    I2_CLK__USBHS0_PHY__CLK_XTAL = 0x02,
    _RESERVED_3 = 0x03,
}
impl UsbClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbClkselMux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbClkselMux {
    #[inline(always)]
    fn from(val: u8) -> UsbClkselMux {
        UsbClkselMux::from_bits(val)
    }
}
impl From<UsbClkselMux> for u8 {
    #[inline(always)]
    fn from(val: UsbClkselMux) -> u8 {
        UsbClkselMux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WwdtClkselMux {
    #[doc = "CLK_16K"]
    I0_CLKROOT_16K = 0x0,
    #[doc = "FRO_HF_DIV"]
    I1_CLKROOT_FIRC_DIV = 0x01,
    #[doc = "CLK_1M"]
    I2_CLKROOT_1M = 0x02,
    #[doc = "CLK_1M"]
    I3_CLKROOT_1M = 0x03,
}
impl WwdtClkselMux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WwdtClkselMux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WwdtClkselMux {
    #[inline(always)]
    fn from(val: u8) -> WwdtClkselMux {
        WwdtClkselMux::from_bits(val)
    }
}
impl From<WwdtClkselMux> for u8 {
    #[inline(always)]
    fn from(val: WwdtClkselMux) -> u8 {
        WwdtClkselMux::to_bits(val)
    }
}
