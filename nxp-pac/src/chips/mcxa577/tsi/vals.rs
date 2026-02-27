#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BaseNochargeNum {
    #[doc = "The SSC output bit 1's basic period will be 1 clock cycle of system clock"]
    BASE_NOCHARGE_NUM_0 = 0x0,
    #[doc = "The SSC output bit 1's basic period will be 2 clock cycles of system clock"]
    BASE_NOCHARGE_NUM_1 = 0x01,
    #[doc = "The SSC output bit 1's basic period will be 3 clock cycles of system clock"]
    BASE_NOCHARGE_NUM_2 = 0x02,
    #[doc = "The SSC output bit 1's basic period will be 4 clock cycles of system clock"]
    BASE_NOCHARGE_NUM_3 = 0x03,
    #[doc = "The SSC output bit 1's basic period will be 5 clock cycles of system clock"]
    BASE_NOCHARGE_NUM_4 = 0x04,
    #[doc = "The SSC output bit 1's basic period will be 6 clock cycles of system clock"]
    BASE_NOCHARGE_NUM_5 = 0x05,
    #[doc = "The SSC output bit 1's basic period will be 7 clock cycles of system clock"]
    BASE_NOCHARGE_NUM_6 = 0x06,
    #[doc = "The SSC output bit 1's basic period will be 8 clock cycles of system clock"]
    BASE_NOCHARGE_NUM_7 = 0x07,
    #[doc = "The SSC output bit 1's basic period will be 9 clock cycles of system clock"]
    BASE_NOCHARGE_NUM_8 = 0x08,
    #[doc = "The SSC output bit 1's basic period will be 10 clock cycles of system clock"]
    BASE_NOCHARGE_NUM_9 = 0x09,
    #[doc = "The SSC output bit 1's basic period will be 11 clock cycles of system clock"]
    BASE_NOCHARGE_NUM_10 = 0x0a,
    #[doc = "The SSC output bit 1's basic period will be 12 clock cycles of system clock"]
    BASE_NOCHARGE_NUM_11 = 0x0b,
    #[doc = "The SSC output bit 1's basic period will be 13 clock cycles of system clock"]
    BASE_NOCHARGE_NUM_12 = 0x0c,
    #[doc = "The SSC output bit 1's basic period will be 14 clock cycles of system clock"]
    BASE_NOCHARGE_NUM_13 = 0x0d,
    #[doc = "The SSC output bit 1's basic period will be 15 clock cycles of system clock"]
    BASE_NOCHARGE_NUM_14 = 0x0e,
    #[doc = "The SSC output bit 1's basic period will be 16 clock cycles of system clock"]
    BASE_NOCHARGE_NUM_15 = 0x0f,
}
impl BaseNochargeNum {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BaseNochargeNum {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BaseNochargeNum {
    #[inline(always)]
    fn from(val: u8) -> BaseNochargeNum {
        BaseNochargeNum::from_bits(val)
    }
}
impl From<BaseNochargeNum> for u8 {
    #[inline(always)]
    fn from(val: BaseNochargeNum) -> u8 {
        BaseNochargeNum::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BaseTraceDebounce {
    #[doc = "The current counter value take 0 ratio"]
    BASE_TRACE_DEBOUNCE_0 = 0x0,
    #[doc = "The current counter value take 1/16 ratio"]
    BASE_TRACE_DEBOUNCE_1 = 0x01,
    #[doc = "The current counter value take 2/16 ratio"]
    BASE_TRACE_DEBOUNCE_2 = 0x02,
    #[doc = "The current counter value take 3/16 ratio"]
    BASE_TRACE_DEBOUNCE_3 = 0x03,
    #[doc = "The current counter value take 4/16 ratio"]
    BASE_TRACE_DEBOUNCE_4 = 0x04,
    #[doc = "The current counter value take 5/16 ratio"]
    BASE_TRACE_DEBOUNCE_5 = 0x05,
    #[doc = "The current counter value take 6/16 ratio"]
    BASE_TRACE_DEBOUNCE_6 = 0x06,
    #[doc = "The current counter value take 7/16 ratio"]
    BASE_TRACE_DEBOUNCE_7 = 0x07,
    #[doc = "The current counter value take 8/16 ratio"]
    BASE_TRACE_DEBOUNCE_8 = 0x08,
    #[doc = "The current counter value take 9/16 ratio"]
    BASE_TRACE_DEBOUNCE_9 = 0x09,
    #[doc = "The current counter value take 10/16 ratio"]
    BASE_TRACE_DEBOUNCE_10 = 0x0a,
    #[doc = "The current counter value take 11/16 ratio"]
    BASE_TRACE_DEBOUNCE_11 = 0x0b,
    #[doc = "The current counter value take 12/16 ratio"]
    BASE_TRACE_DEBOUNCE_12 = 0x0c,
    #[doc = "The current counter value take 13/16 ratio"]
    BASE_TRACE_DEBOUNCE_13 = 0x0d,
    #[doc = "The current counter value take 14/16 ratio"]
    BASE_TRACE_DEBOUNCE_14 = 0x0e,
    #[doc = "The current counter value take 15/16 ratio"]
    BASE_TRACE_DEBOUNCE_15 = 0x0f,
}
impl BaseTraceDebounce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BaseTraceDebounce {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BaseTraceDebounce {
    #[inline(always)]
    fn from(val: u8) -> BaseTraceDebounce {
        BaseTraceDebounce::from_bits(val)
    }
}
impl From<BaseTraceDebounce> for u8 {
    #[inline(always)]
    fn from(val: BaseTraceDebounce) -> u8 {
        BaseTraceDebounce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CapBank148ff {
    #[doc = "enable another 148 pF in cap bank"]
    CAP_BANK_148FF_0 = 0x0,
    #[doc = "enable another 296 pF in cap bank"]
    CAP_BANK_148FF_1 = 0x01,
    #[doc = "enable another 444 pF in cap bank"]
    CAP_BANK_148FF_2 = 0x02,
    #[doc = "enable another 592 pF in cap bank"]
    CAP_BANK_148FF_3 = 0x03,
    #[doc = "enable another 740 pF in cap bank"]
    CAP_BANK_148FF_4 = 0x04,
    #[doc = "enable another 888 pF in cap bank"]
    CAP_BANK_148FF_5 = 0x05,
    #[doc = "enable another 1036 pF in cap bank"]
    CAP_BANK_148FF_6 = 0x06,
    #[doc = "enable another 1184 pF in cap bank"]
    CAP_BANK_148FF_7 = 0x07,
}
impl CapBank148ff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CapBank148ff {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CapBank148ff {
    #[inline(always)]
    fn from(val: u8) -> CapBank148ff {
        CapBank148ff::from_bits(val)
    }
}
impl From<CapBank148ff> for u8 {
    #[inline(always)]
    fn from(val: CapBank148ff) -> u8 {
        CapBank148ff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CapBank2500ff {
    #[doc = "enable another 2.5 pF in cap bank"]
    CAP_BANK_2500FF_0 = 0x0,
    #[doc = "enable another 5.0 pF in cap bank"]
    CAP_BANK_2500FF_1 = 0x01,
    #[doc = "enable another 7.5 pF in cap bank"]
    CAP_BANK_2500FF_2 = 0x02,
    #[doc = "enable another 10 pF in cap bank"]
    CAP_BANK_2500FF_3 = 0x03,
    #[doc = "enable another 12.5 pF in cap bank"]
    CAP_BANK_2500FF_4 = 0x04,
    #[doc = "enable another 15 pF in cap bank"]
    CAP_BANK_2500FF_5 = 0x05,
    #[doc = "enable another 17.5 pF in cap bank"]
    CAP_BANK_2500FF_6 = 0x06,
    #[doc = "enable another 20 pF in cap bank"]
    CAP_BANK_2500FF_7 = 0x07,
}
impl CapBank2500ff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CapBank2500ff {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CapBank2500ff {
    #[inline(always)]
    fn from(val: u8) -> CapBank2500ff {
        CapBank2500ff::from_bits(val)
    }
}
impl From<CapBank2500ff> for u8 {
    #[inline(always)]
    fn from(val: CapBank2500ff) -> u8 {
        CapBank2500ff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CapBank312ff {
    #[doc = "enable another 0.3125 pF"]
    CAP_BANK_312FF_0 = 0x0,
    #[doc = "enable another 0.625 pF"]
    CAP_BANK_312FF_1 = 0x01,
    #[doc = "enable another 0.936 pF"]
    CAP_BANK_312FF_2 = 0x02,
    #[doc = "enable another 1.25 pF"]
    CAP_BANK_312FF_3 = 0x03,
    #[doc = "enable another 1.56 pF"]
    CAP_BANK_312FF_4 = 0x04,
    #[doc = "enable another 1.87 pF"]
    CAP_BANK_312FF_5 = 0x05,
    #[doc = "enable another 2.19 pF"]
    CAP_BANK_312FF_6 = 0x06,
    #[doc = "enable another 2.5 pF"]
    CAP_BANK_312FF_7 = 0x07,
}
impl CapBank312ff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CapBank312ff {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CapBank312ff {
    #[inline(always)]
    fn from(val: u8) -> CapBank312ff {
        CapBank312ff::from_bits(val)
    }
}
impl From<CapBank312ff> for u8 {
    #[inline(always)]
    fn from(val: CapBank312ff) -> u8 {
        CapBank312ff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CapBankEn {
    #[doc = "Disable cap bank function"]
    CAP_BANK_EN_0 = 0x0,
    #[doc = "Enable cap bank function"]
    CAP_BANK_EN_1 = 0x01,
}
impl CapBankEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CapBankEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CapBankEn {
    #[inline(always)]
    fn from(val: u8) -> CapBankEn {
        CapBankEn::from_bits(val)
    }
}
impl From<CapBankEn> for u8 {
    #[inline(always)]
    fn from(val: CapBankEn) -> u8 {
        CapBankEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChargeNum {
    #[doc = "The SSC output bit 0's period will be 1 clock cycle of system clock"]
    CHARGE_NUM_0 = 0x0,
    #[doc = "The SSC output bit 0's period will be 2 clock cycles of system clock"]
    CHARGE_NUM_1 = 0x01,
    #[doc = "The SSC output bit 0's period will be 3 clock cycles of system clock"]
    CHARGE_NUM_2 = 0x02,
    #[doc = "The SSC output bit 0's period will be 4 clock cycles of system clock"]
    CHARGE_NUM_3 = 0x03,
    #[doc = "The SSC output bit 0's period will be 5 clock cycles of system clock"]
    CHARGE_NUM_4 = 0x04,
    #[doc = "The SSC output bit 0's period will be 6 clock cycles of system clock"]
    CHARGE_NUM_5 = 0x05,
    #[doc = "The SSC output bit 0's period will be 7 clock cycles of system clock"]
    CHARGE_NUM_6 = 0x06,
    #[doc = "The SSC output bit 0's period will be 8 clock cycles of system clock"]
    CHARGE_NUM_7 = 0x07,
    #[doc = "The SSC output bit 0's period will be 9 clock cycles of system clock"]
    CHARGE_NUM_8 = 0x08,
    #[doc = "The SSC output bit 0's period will be 10 clock cycles of system clock"]
    CHARGE_NUM_9 = 0x09,
    #[doc = "The SSC output bit 0's period will be 11 clock cycles of system clock"]
    CHARGE_NUM_10 = 0x0a,
    #[doc = "The SSC output bit 0's period will be 12 clock cycles of system clock"]
    CHARGE_NUM_11 = 0x0b,
    #[doc = "The SSC output bit 0's period will be 13 clock cycles of system clock"]
    CHARGE_NUM_12 = 0x0c,
    #[doc = "The SSC output bit 0's period will be 14 clock cycles of system clock"]
    CHARGE_NUM_13 = 0x0d,
    #[doc = "The SSC output bit 0's period will be 15 clock cycles of system clock"]
    CHARGE_NUM_14 = 0x0e,
    #[doc = "The SSC output bit 0's period will be 16 clock cycles of system clock"]
    CHARGE_NUM_15 = 0x0f,
}
impl ChargeNum {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChargeNum {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChargeNum {
    #[inline(always)]
    fn from(val: u8) -> ChargeNum {
        ChargeNum::from_bits(val)
    }
}
impl From<ChargeNum> for u8 {
    #[inline(always)]
    fn from(val: ChargeNum) -> u8 {
        ChargeNum::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkSel {
    #[doc = "Select 32K clock to use"]
    CLK_SEL_0 = 0x0,
    #[doc = "Select 16K clock to use"]
    CLK_SEL_1 = 0x01,
}
impl ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkSel {
    #[inline(always)]
    fn from(val: u8) -> ClkSel {
        ClkSel::from_bits(val)
    }
}
impl From<ClkSel> for u8 {
    #[inline(always)]
    fn from(val: ClkSel) -> u8 {
        ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConfigMode {
    #[doc = "self-cap mode, this address is TSI_SEL_CONFIG"]
    MODE_0 = 0x0,
    #[doc = "mutual-cap mode, this address is TSI_MUL_CONFIG"]
    MODE_1 = 0x01,
}
impl ConfigMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ConfigMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ConfigMode {
    #[inline(always)]
    fn from(val: u8) -> ConfigMode {
        ConfigMode::from_bits(val)
    }
}
impl From<ConfigMode> for u8 {
    #[inline(always)]
    fn from(val: ConfigMode) -> u8 {
        ConfigMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConfigMutualMode {
    #[doc = "self-cap mode, this address is TSI_CONFIG for self-cap"]
    MODE_0 = 0x0,
    #[doc = "mutual-cap mode, this address is TSI_CONFIG for mutual-cap"]
    MODE_1 = 0x01,
}
impl ConfigMutualMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ConfigMutualMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ConfigMutualMode {
    #[inline(always)]
    fn from(val: u8) -> ConfigMutualMode {
        ConfigMutualMode::from_bits(val)
    }
}
impl From<ConfigMutualMode> for u8 {
    #[inline(always)]
    fn from(val: ConfigMutualMode) -> u8 {
        ConfigMutualMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CountingClkHigh {
    #[doc = "Using default process inside SSC"]
    COUNTING_CLK_HIGH_0 = 0x0,
    #[doc = "Follow the counting signal from AIP front end"]
    COUNTING_CLK_HIGH_1 = 0x01,
}
impl CountingClkHigh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CountingClkHigh {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CountingClkHigh {
    #[inline(always)]
    fn from(val: u8) -> CountingClkHigh {
        CountingClkHigh::from_bits(val)
    }
}
impl From<CountingClkHigh> for u8 {
    #[inline(always)]
    fn from(val: CountingClkHigh) -> u8 {
        CountingClkHigh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebounceCounterConfigured {
    #[doc = "1 out of range event can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_0 = 0x0,
    #[doc = "2 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_1 = 0x01,
    #[doc = "3 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_2 = 0x02,
    #[doc = "4 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_3 = 0x03,
    #[doc = "5 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_4 = 0x04,
    #[doc = "6 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_5 = 0x05,
    #[doc = "7 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_6 = 0x06,
    #[doc = "8 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_7 = 0x07,
    #[doc = "9 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_8 = 0x08,
    #[doc = "10 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_9 = 0x09,
    #[doc = "11 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_10 = 0x0a,
    #[doc = "12 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_11 = 0x0b,
    #[doc = "13 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_12 = 0x0c,
    #[doc = "14 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_13 = 0x0d,
    #[doc = "15 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_14 = 0x0e,
    #[doc = "16 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_15 = 0x0f,
    #[doc = "17 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_16 = 0x10,
    #[doc = "18 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_17 = 0x11,
    #[doc = "19 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_18 = 0x12,
    #[doc = "20 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_19 = 0x13,
    #[doc = "21 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_20 = 0x14,
    #[doc = "22 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_21 = 0x15,
    #[doc = "23 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_22 = 0x16,
    #[doc = "24 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_23 = 0x17,
    #[doc = "25 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_24 = 0x18,
    #[doc = "26 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_25 = 0x19,
    #[doc = "27 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_26 = 0x1a,
    #[doc = "28 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_27 = 0x1b,
    #[doc = "29 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_28 = 0x1c,
    #[doc = "30 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_29 = 0x1d,
    #[doc = "31 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_30 = 0x1e,
    #[doc = "32 out of range events can generate interrupt"]
    DEBOUNCE_COUNTER_CONFIGURED_31 = 0x1f,
}
impl DebounceCounterConfigured {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebounceCounterConfigured {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebounceCounterConfigured {
    #[inline(always)]
    fn from(val: u8) -> DebounceCounterConfigured {
        DebounceCounterConfigured::from_bits(val)
    }
}
impl From<DebounceCounterConfigured> for u8 {
    #[inline(always)]
    fn from(val: DebounceCounterConfigured) -> u8 {
        DebounceCounterConfigured::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaenEos {
    #[doc = "Disable DMA request at End of Scan event"]
    DMAEN_EOS_0 = 0x0,
    #[doc = "Enable DMA request at End of event"]
    DMAEN_EOS_1 = 0x01,
}
impl DmaenEos {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaenEos {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaenEos {
    #[inline(always)]
    fn from(val: u8) -> DmaenEos {
        DmaenEos::from_bits(val)
    }
}
impl From<DmaenEos> for u8 {
    #[inline(always)]
    fn from(val: DmaenEos) -> u8 {
        DmaenEos::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaenOutrg {
    #[doc = "Disable DMA request at Out-of-Range event"]
    DMAEN_OUTRG_0 = 0x0,
    #[doc = "Enable DMA request at Out-of-Range event"]
    DMAEN_OUTRG_1 = 0x01,
}
impl DmaenOutrg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaenOutrg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaenOutrg {
    #[inline(always)]
    fn from(val: u8) -> DmaenOutrg {
        DmaenOutrg::from_bits(val)
    }
}
impl From<DmaenOutrg> for u8 {
    #[inline(always)]
    fn from(val: DmaenOutrg) -> u8 {
        DmaenOutrg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DpdModeEnable {
    #[doc = "configuration information in 3v domain logic is latched"]
    DPD_MODE_ENABLE_0 = 0x0,
    #[doc = "configuration information in 3v domain logic is updating from configuration registers simultaneously"]
    DPD_MODE_ENABLE_1 = 0x01,
}
impl DpdModeEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DpdModeEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DpdModeEnable {
    #[inline(always)]
    fn from(val: u8) -> DpdModeEnable {
        DpdModeEnable::from_bits(val)
    }
}
impl From<DpdModeEnable> for u8 {
    #[inline(always)]
    fn from(val: DpdModeEnable) -> u8 {
        DpdModeEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DpdOutrgf {
    #[doc = "the counter value is not out of range"]
    DPD_OUTRGF_0 = 0x0,
    #[doc = "the counter value is out of range"]
    DPD_OUTRGF_1 = 0x01,
}
impl DpdOutrgf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DpdOutrgf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DpdOutrgf {
    #[inline(always)]
    fn from(val: u8) -> DpdOutrgf {
        DpdOutrgf::from_bits(val)
    }
}
impl From<DpdOutrgf> for u8 {
    #[inline(always)]
    fn from(val: DpdOutrgf) -> u8 {
        DpdOutrgf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DpdTestEn {
    #[doc = "IP is not entering dpd test mode"]
    DPD_TEST_EN_0 = 0x0,
    #[doc = "IP is entering dpd test mode"]
    DPD_TEST_EN_1 = 0x01,
}
impl DpdTestEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DpdTestEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DpdTestEn {
    #[inline(always)]
    fn from(val: u8) -> DpdTestEn {
        DpdTestEn::from_bits(val)
    }
}
impl From<DpdTestEn> for u8 {
    #[inline(always)]
    fn from(val: DpdTestEn) -> u8 {
        DpdTestEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DpdTestEosf {
    #[doc = "scan is not done in DPD test mode"]
    DPD_TEST_EOSF_0 = 0x0,
    #[doc = "scan is done in DPD test mode"]
    DPD_TEST_EOSF_1 = 0x01,
}
impl DpdTestEosf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DpdTestEosf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DpdTestEosf {
    #[inline(always)]
    fn from(val: u8) -> DpdTestEosf {
        DpdTestEosf::from_bits(val)
    }
}
impl From<DpdTestEosf> for u8 {
    #[inline(always)]
    fn from(val: DpdTestEosf) -> u8 {
        DpdTestEosf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dvolt {
    #[doc = "Vm=0.6 V, Vp=1.73 V"]
    DVOLT_0 = 0x0,
    #[doc = "Vm=0.6 V, Vp=1.82 V"]
    DVOLT_1 = 0x01,
    #[doc = "Vm=0.6 V, Vp=1.89 V"]
    DVOLT_2 = 0x02,
    #[doc = "Vm=0.6 V, Vp=1.98 V"]
    DVOLT_3 = 0x03,
    #[doc = "Vm=0.6 V, Vp=2.06 V"]
    DVOLT_4 = 0x04,
    #[doc = "Vm=0.6 V, Vp=2.14 V"]
    DVOLT_5 = 0x05,
    #[doc = "Vm=0.6 V, Vp=2.23 V"]
    DVOLT_6 = 0x06,
    #[doc = "Vm=0.6 V, Vp=2.31 V"]
    DVOLT_7 = 0x07,
    #[doc = "Vm=0.6 V, Vp=2.39 V"]
    DVOLT_8 = 0x08,
    #[doc = "Vm=0.6 V, Vp=2.47 V"]
    DVOLT_9 = 0x09,
    #[doc = "Vm=0.6 V, Vp=2.56 V"]
    DVOLT_10 = 0x0a,
    #[doc = "Vm=0.6 V, Vp=2.64 V"]
    DVOLT_11 = 0x0b,
    #[doc = "Vm=0.6 V, Vp=2.72 V"]
    DVOLT_12 = 0x0c,
    #[doc = "Vm=0.6 V, Vp=2.81 V"]
    DVOLT_13 = 0x0d,
    #[doc = "Vm=0.6 V, Vp=2.88 V"]
    DVOLT_14 = 0x0e,
    #[doc = "Vm=0.6 V, Vp=2.97 V"]
    DVOLT_15 = 0x0f,
}
impl Dvolt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dvolt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dvolt {
    #[inline(always)]
    fn from(val: u8) -> Dvolt {
        Dvolt::from_bits(val)
    }
}
impl From<Dvolt> for u8 {
    #[inline(always)]
    fn from(val: Dvolt) -> u8 {
        Dvolt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eosf {
    #[doc = "scan is not done"]
    EOSF_0 = 0x0,
    #[doc = "scan is done"]
    EOSF_1 = 0x01,
}
impl Eosf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eosf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eosf {
    #[inline(always)]
    fn from(val: u8) -> Eosf {
        Eosf::from_bits(val)
    }
}
impl From<Eosf> for u8 {
    #[inline(always)]
    fn from(val: Eosf) -> u8 {
        Eosf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Esor {
    #[doc = "End of Scan interrupt is disable"]
    ESOR_0 = 0x0,
    #[doc = "End of Scan interrupt is enabled"]
    ESOR_1 = 0x01,
}
impl Esor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Esor {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Esor {
    #[inline(always)]
    fn from(val: u8) -> Esor {
        Esor::from_bits(val)
    }
}
impl From<Esor> for u8 {
    #[inline(always)]
    fn from(val: Esor) -> u8 {
        Esor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilterTrim {
    #[doc = "filter bypass"]
    FILTER_TRIM_0 = 0x0,
    #[doc = "5.3ns"]
    FILTER_TRIM_1 = 0x01,
    #[doc = "21.1ns"]
    FILTER_TRIM_2 = 0x02,
    #[doc = "25ns"]
    FILTER_TRIM_3 = 0x03,
}
impl FilterTrim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FilterTrim {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FilterTrim {
    #[inline(always)]
    fn from(val: u8) -> FilterTrim {
        FilterTrim::from_bits(val)
    }
}
impl From<FilterTrim> for u8 {
    #[inline(always)]
    fn from(val: FilterTrim) -> u8 {
        FilterTrim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MCntEn {
    #[doc = "disable, the counter work in all scan process"]
    M_CNT_EN_0 = 0x0,
    #[doc = "enable, the counter only work when the capacitor does the integration"]
    M_CNT_EN_1 = 0x01,
}
impl MCntEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MCntEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MCntEn {
    #[inline(always)]
    fn from(val: u8) -> MCntEn {
        MCntEn::from_bits(val)
    }
}
impl From<MCntEn> for u8 {
    #[inline(always)]
    fn from(val: MCntEn) -> u8 {
        MCntEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MMode {
    #[doc = "sense both pos and neg side"]
    M_MODE_0 = 0x0,
    #[doc = "sense pos side only"]
    M_MODE_1 = 0x01,
    #[doc = "sense neg side only"]
    M_MODE_2 = 0x02,
    #[doc = "not allowed"]
    M_MODE_3 = 0x03,
}
impl MMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MMode {
    #[inline(always)]
    fn from(val: u8) -> MMode {
        MMode::from_bits(val)
    }
}
impl From<MMode> for u8 {
    #[inline(always)]
    fn from(val: MMode) -> u8 {
        MMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MNmirror {
    #[doc = "m=1"]
    M_NMIRROR_0 = 0x0,
    #[doc = "m=2"]
    M_NMIRROR_1 = 0x01,
    #[doc = "m=3"]
    M_NMIRROR_2 = 0x02,
    #[doc = "m=4 (default)"]
    M_NMIRROR_3 = 0x03,
}
impl MNmirror {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MNmirror {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MNmirror {
    #[inline(always)]
    fn from(val: u8) -> MNmirror {
        MNmirror::from_bits(val)
    }
}
impl From<MNmirror> for u8 {
    #[inline(always)]
    fn from(val: MNmirror) -> u8 {
        MNmirror::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MPmirrorl {
    #[doc = "m=4"]
    M_PMIRRORL_0 = 0x0,
    #[doc = "m=8"]
    M_PMIRRORL_1 = 0x01,
    #[doc = "m=12"]
    M_PMIRRORL_2 = 0x02,
    #[doc = "m=16 (default)"]
    M_PMIRRORL_3 = 0x03,
    #[doc = "m=20"]
    M_PMIRRORL_4 = 0x04,
    #[doc = "m=24"]
    M_PMIRRORL_5 = 0x05,
    #[doc = "m=28"]
    M_PMIRRORL_6 = 0x06,
    #[doc = "m=32"]
    M_PMIRRORL_7 = 0x07,
}
impl MPmirrorl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MPmirrorl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MPmirrorl {
    #[inline(always)]
    fn from(val: u8) -> MPmirrorl {
        MPmirrorl::from_bits(val)
    }
}
impl From<MPmirrorl> for u8 {
    #[inline(always)]
    fn from(val: MPmirrorl) -> u8 {
        MPmirrorl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MPmirrorr {
    #[doc = "m=1"]
    M_PMIRRORR_0 = 0x0,
    #[doc = "m=2"]
    M_PMIRRORR_1 = 0x01,
    #[doc = "m=3"]
    M_PMIRRORR_2 = 0x02,
    #[doc = "m=4 (default)"]
    M_PMIRRORR_3 = 0x03,
}
impl MPmirrorr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MPmirrorr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MPmirrorr {
    #[inline(always)]
    fn from(val: u8) -> MPmirrorr {
        MPmirrorr::from_bits(val)
    }
}
impl From<MPmirrorr> for u8 {
    #[inline(always)]
    fn from(val: MPmirrorr) -> u8 {
        MPmirrorr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MPreCurrent {
    #[doc = "1 uA"]
    M_PRE_CURRENT_0 = 0x0,
    #[doc = "2 uA"]
    M_PRE_CURRENT_1 = 0x01,
    #[doc = "3 uA"]
    M_PRE_CURRENT_2 = 0x02,
    #[doc = "4 uA"]
    M_PRE_CURRENT_3 = 0x03,
    #[doc = "5 uA"]
    M_PRE_CURRENT_4 = 0x04,
    #[doc = "6 uA"]
    M_PRE_CURRENT_5 = 0x05,
    #[doc = "7 uA"]
    M_PRE_CURRENT_6 = 0x06,
    #[doc = "8 uA"]
    M_PRE_CURRENT_7 = 0x07,
}
impl MPreCurrent {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MPreCurrent {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MPreCurrent {
    #[inline(always)]
    fn from(val: u8) -> MPreCurrent {
        MPreCurrent::from_bits(val)
    }
}
impl From<MPreCurrent> for u8 {
    #[inline(always)]
    fn from(val: MPreCurrent) -> u8 {
        MPreCurrent::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MPreRes {
    #[doc = "1 kohm"]
    M_PRE_RES_0 = 0x0,
    #[doc = "2 kohm"]
    M_PRE_RES_1 = 0x01,
    #[doc = "3 kohm"]
    M_PRE_RES_2 = 0x02,
    #[doc = "4 kohm"]
    M_PRE_RES_3 = 0x03,
    #[doc = "5 kohm"]
    M_PRE_RES_4 = 0x04,
    #[doc = "6 kohm"]
    M_PRE_RES_5 = 0x05,
    #[doc = "7 kohm"]
    M_PRE_RES_6 = 0x06,
    #[doc = "8 kohm"]
    M_PRE_RES_7 = 0x07,
}
impl MPreRes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MPreRes {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MPreRes {
    #[inline(always)]
    fn from(val: u8) -> MPreRes {
        MPreRes::from_bits(val)
    }
}
impl From<MPreRes> for u8 {
    #[inline(always)]
    fn from(val: MPreRes) -> u8 {
        MPreRes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MSenBoost {
    #[doc = "0 u"]
    M_SEN_BOOST_0 = 0x0,
    #[doc = "2 u"]
    M_SEN_BOOST_1 = 0x01,
    #[doc = "4 u"]
    M_SEN_BOOST_2 = 0x02,
    #[doc = "6 u"]
    M_SEN_BOOST_3 = 0x03,
    #[doc = "8 u"]
    M_SEN_BOOST_4 = 0x04,
    #[doc = "10 u"]
    M_SEN_BOOST_5 = 0x05,
    #[doc = "12 u"]
    M_SEN_BOOST_6 = 0x06,
    #[doc = "14 u"]
    M_SEN_BOOST_7 = 0x07,
    #[doc = "16 u"]
    M_SEN_BOOST_8 = 0x08,
    #[doc = "18 u"]
    M_SEN_BOOST_9 = 0x09,
    #[doc = "20 u"]
    M_SEN_BOOST_10 = 0x0a,
    #[doc = "22 u"]
    M_SEN_BOOST_11 = 0x0b,
    #[doc = "24 u"]
    M_SEN_BOOST_12 = 0x0c,
    #[doc = "26 u"]
    M_SEN_BOOST_13 = 0x0d,
    #[doc = "28 u"]
    M_SEN_BOOST_14 = 0x0e,
    #[doc = "30 u"]
    M_SEN_BOOST_15 = 0x0f,
    #[doc = "32 u"]
    M_SEN_BOOST_16 = 0x10,
    #[doc = "34 u"]
    M_SEN_BOOST_17 = 0x11,
    #[doc = "36 u"]
    M_SEN_BOOST_18 = 0x12,
    #[doc = "38 u"]
    M_SEN_BOOST_19 = 0x13,
    #[doc = "40 u"]
    M_SEN_BOOST_20 = 0x14,
    #[doc = "42 u"]
    M_SEN_BOOST_21 = 0x15,
    #[doc = "44 u"]
    M_SEN_BOOST_22 = 0x16,
    #[doc = "46 u"]
    M_SEN_BOOST_23 = 0x17,
    #[doc = "48 u"]
    M_SEN_BOOST_24 = 0x18,
    #[doc = "50 u"]
    M_SEN_BOOST_25 = 0x19,
    #[doc = "52 u"]
    M_SEN_BOOST_26 = 0x1a,
    #[doc = "54 u"]
    M_SEN_BOOST_27 = 0x1b,
    #[doc = "56 u"]
    M_SEN_BOOST_28 = 0x1c,
    #[doc = "58 u"]
    M_SEN_BOOST_29 = 0x1d,
    #[doc = "60 u"]
    M_SEN_BOOST_30 = 0x1e,
    #[doc = "62 u"]
    M_SEN_BOOST_31 = 0x1f,
}
impl MSenBoost {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MSenBoost {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MSenBoost {
    #[inline(always)]
    fn from(val: u8) -> MSenBoost {
        MSenBoost::from_bits(val)
    }
}
impl From<MSenBoost> for u8 {
    #[inline(always)]
    fn from(val: MSenBoost) -> u8 {
        MSenBoost::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MSenRes {
    #[doc = "10 k"]
    M_SEN_RES_0 = 0x0,
    #[doc = "10 k + (2.5/3) k (just for auto-calibration)"]
    M_SEN_RES_1 = 0x01,
    #[doc = "12.5 k (default)"]
    M_SEN_RES_2 = 0x02,
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
    #[doc = "25 k"]
    M_SEN_RES_14 = 0x0e,
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
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl MSenRes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MSenRes {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MSenRes {
    #[inline(always)]
    fn from(val: u8) -> MSenRes {
        MSenRes::from_bits(val)
    }
}
impl From<MSenRes> for u8 {
    #[inline(always)]
    fn from(val: MSenRes) -> u8 {
        MSenRes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MTrimCap {
    #[doc = "0 pF (default)"]
    M_TRIM_CAP_0 = 0x0,
    #[doc = "10 pF"]
    M_TRIM_CAP_1 = 0x01,
    #[doc = "10 pF"]
    M_TRIM_CAP_2 = 0x02,
    #[doc = "20 pF"]
    M_TRIM_CAP_3 = 0x03,
}
impl MTrimCap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MTrimCap {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MTrimCap {
    #[inline(always)]
    fn from(val: u8) -> MTrimCap {
        MTrimCap::from_bits(val)
    }
}
impl From<MTrimCap> for u8 {
    #[inline(always)]
    fn from(val: MTrimCap) -> u8 {
        MTrimCap::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MTxPdEn {
    #[doc = "disable"]
    M_TX_PD_EN_0 = 0x0,
    #[doc = "enable (default)"]
    M_TX_PD_EN_1 = 0x01,
}
impl MTxPdEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MTxPdEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MTxPdEn {
    #[inline(always)]
    fn from(val: u8) -> MTxPdEn {
        MTxPdEn::from_bits(val)
    }
}
impl From<MTxPdEn> for u8 {
    #[inline(always)]
    fn from(val: MTxPdEn) -> u8 {
        MTxPdEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MVpreChoose {
    #[doc = "Internal 1.2 V voltage (default)"]
    M_VPRE_CHOOSE_0 = 0x0,
    #[doc = "External 1.2 V voltage from PMC"]
    M_VPRE_CHOOSE_1 = 0x01,
}
impl MVpreChoose {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MVpreChoose {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MVpreChoose {
    #[inline(always)]
    fn from(val: u8) -> MVpreChoose {
        MVpreChoose::from_bits(val)
    }
}
impl From<MVpreChoose> for u8 {
    #[inline(always)]
    fn from(val: MVpreChoose) -> u8 {
        MVpreChoose::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MoveNochargeMin {
    #[doc = "The SSC output bit 1's min period will be (1 + TSI_SSC0\\[BASE_ NOCHARGE_NUM\\]) clock cycle of system clock"]
    MOVE_NOCHARGE_MIN_0 = 0x0,
    #[doc = "The SSC output bit 1's min period will be (2 + TSI_SSC0\\[BASE_ NOCHARGE_NUM\\]) clock cycles of system clock"]
    MOVE_NOCHARGE_MIN_1 = 0x01,
    #[doc = "The SSC output bit 1's min period will be (3 + TSI_SSC0\\[BASE_ NOCHARGE_NUM\\]) clock cycles of system clock"]
    MOVE_NOCHARGE_MIN_2 = 0x02,
    #[doc = "The SSC output bit 1's min period will be (4 + TSI_SSC0\\[BASE_ NOCHARGE_NUM\\]) clock cycles of system clock"]
    MOVE_NOCHARGE_MIN_3 = 0x03,
    #[doc = "The SSC output bit 1's min period will be (5 + TSI_SSC0\\[BASE_ NOCHARGE_NUM\\]) clock cycles of system clock"]
    MOVE_NOCHARGE_MIN_4 = 0x04,
    #[doc = "The SSC output bit 1's min period will be (6 + TSI_SSC0\\[BASE_ NOCHARGE_NUM\\]) clock cycles of system clock"]
    MOVE_NOCHARGE_MIN_5 = 0x05,
    #[doc = "The SSC output bit 1's min period will be (7 + TSI_SSC0\\[BASE_ NOCHARGE_NUM\\]) clock cycles of system clock"]
    MOVE_NOCHARGE_MIN_6 = 0x06,
    #[doc = "The SSC output bit 1's min period will be (8 + TSI_SSC0\\[BASE_ NOCHARGE_NUM\\]) clock cycles of system clock"]
    MOVE_NOCHARGE_MIN_7 = 0x07,
    #[doc = "The SSC output bit 1's min period will be (9 + TSI_SSC0\\[BASE_ NOCHARGE_NUM\\]) clock cycles of system clock"]
    MOVE_NOCHARGE_MIN_8 = 0x08,
    #[doc = "The SSC output bit 1's min period will be (10 + TSI_SSC0\\[BASE_ NOCHARGE_NUM\\]) clock cycles of system clock"]
    MOVE_NOCHARGE_MIN_9 = 0x09,
    #[doc = "The SSC output bit 1's min period will be (11 + TSI_SSC0\\[BASE_ NOCHARGE_NUM\\]) clock cycles of system clock"]
    MOVE_NOCHARGE_MIN_10 = 0x0a,
    #[doc = "The SSC output bit 1's min period will be (12 + TSI_SSC0\\[BASE_ NOCHARGE_NUM\\]) clock cycles of system clock"]
    MOVE_NOCHARGE_MIN_11 = 0x0b,
    #[doc = "The SSC output bit 1's min period will be (13 + TSI_SSC0\\[BASE_ NOCHARGE_NUM\\]) clock cycles of system clock"]
    MOVE_NOCHARGE_MIN_12 = 0x0c,
    #[doc = "The SSC output bit 1's min period will be (14 + TSI_SSC0\\[BASE_ NOCHARGE_NUM\\]) clock cycles of system clock"]
    MOVE_NOCHARGE_MIN_13 = 0x0d,
    #[doc = "The SSC output bit 1's min period will be (15 + TSI_SSC0\\[BASE_ NOCHARGE_NUM\\]) clock cycles of system clock"]
    MOVE_NOCHARGE_MIN_14 = 0x0e,
    #[doc = "The SSC output bit 1's min period will be (16 + TSI_SSC0\\[BASE_ NOCHARGE_NUM\\]) clock cycles of system clock"]
    MOVE_NOCHARGE_MIN_15 = 0x0f,
}
impl MoveNochargeMin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MoveNochargeMin {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MoveNochargeMin {
    #[inline(always)]
    fn from(val: u8) -> MoveNochargeMin {
        MoveNochargeMin::from_bits(val)
    }
}
impl From<MoveNochargeMin> for u8 {
    #[inline(always)]
    fn from(val: MoveNochargeMin) -> u8 {
        MoveNochargeMin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MoveRepeatNum {
    #[doc = "The up_down counter will be updated for every sample-charge cycle"]
    MOVE_REPEAT_NUM_0 = 0x0,
    #[doc = "The up_down counter will be updated for every 2 sample-charge cycles"]
    MOVE_REPEAT_NUM_1 = 0x01,
    #[doc = "The up_down counter will be updated for every 3 sample-charge cycles"]
    MOVE_REPEAT_NUM_2 = 0x02,
    #[doc = "The up_down counter will be updated for every 4 sample-charge cycles"]
    MOVE_REPEAT_NUM_3 = 0x03,
    #[doc = "The up_down counter will be updated for every 5 sample-charge cycles"]
    MOVE_REPEAT_NUM_4 = 0x04,
    #[doc = "The up_down counter will be updated for every 6 sample-charge cycles"]
    MOVE_REPEAT_NUM_5 = 0x05,
    #[doc = "The up_down counter will be updated for every 7 sample-charge cycles"]
    MOVE_REPEAT_NUM_6 = 0x06,
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
impl MoveRepeatNum {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MoveRepeatNum {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MoveRepeatNum {
    #[inline(always)]
    fn from(val: u8) -> MoveRepeatNum {
        MoveRepeatNum::from_bits(val)
    }
}
impl From<MoveRepeatNum> for u8 {
    #[inline(always)]
    fn from(val: MoveRepeatNum) -> u8 {
        MoveRepeatNum::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MoveStepsNum {
    #[doc = "The added value for up-down counter is 0"]
    MOVE_STEPS_NUM_0 = 0x0,
    #[doc = "The added value for up-down counter is 1"]
    MOVE_STEPS_NUM_1 = 0x01,
    #[doc = "The added value for up-down counter is 2"]
    MOVE_STEPS_NUM_2 = 0x02,
    #[doc = "The added value for up-down counter is 3"]
    MOVE_STEPS_NUM_3 = 0x03,
    #[doc = "The added value for up-down counter is 4"]
    MOVE_STEPS_NUM_4 = 0x04,
    #[doc = "The added value for up-down counter is 5"]
    MOVE_STEPS_NUM_5 = 0x05,
    #[doc = "The added value for up-down counter is 6"]
    MOVE_STEPS_NUM_6 = 0x06,
    #[doc = "The added value for up-down counter is 7"]
    MOVE_STEPS_NUM_7 = 0x07,
}
impl MoveStepsNum {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MoveStepsNum {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MoveStepsNum {
    #[inline(always)]
    fn from(val: u8) -> MoveStepsNum {
        MoveStepsNum::from_bits(val)
    }
}
impl From<MoveStepsNum> for u8 {
    #[inline(always)]
    fn from(val: MoveStepsNum) -> u8 {
        MoveStepsNum::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutrgEn {
    #[doc = "Out-of-range interrupt is disable"]
    OUTRG_EN_0 = 0x0,
    #[doc = "Out-of-range interrupt is enabled"]
    OUTRG_EN_1 = 0x01,
}
impl OutrgEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutrgEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutrgEn {
    #[inline(always)]
    fn from(val: u8) -> OutrgEn {
        OutrgEn::from_bits(val)
    }
}
impl From<OutrgEn> for u8 {
    #[inline(always)]
    fn from(val: OutrgEn) -> u8 {
        OutrgEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Outrgf {
    #[doc = "the counter value is not out of range"]
    OUTRGF_0 = 0x0,
    #[doc = "the counter value is out of range"]
    OUTRGF_1 = 0x01,
}
impl Outrgf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Outrgf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Outrgf {
    #[inline(always)]
    fn from(val: u8) -> Outrgf {
        Outrgf::from_bits(val)
    }
}
impl From<Outrgf> for u8 {
    #[inline(always)]
    fn from(val: Outrgf) -> u8 {
        Outrgf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Overrunf {
    #[doc = "there is trigger or ips bus writing during scan progress"]
    OVERRUNF_0 = 0x0,
    #[doc = "there is no trigger or ips bus writing during scan progress"]
    OVERRUNF_1 = 0x01,
}
impl Overrunf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Overrunf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Overrunf {
    #[inline(always)]
    fn from(val: u8) -> Overrunf {
        Overrunf::from_bits(val)
    }
}
impl From<Overrunf> for u8 {
    #[inline(always)]
    fn from(val: Overrunf) -> u8 {
        Overrunf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PrbsOutsel {
    #[doc = "NC"]
    PRBS_OUTSEL_0 = 0x0,
    #[doc = "NC"]
    PRBS_OUTSEL_1 = 0x01,
    #[doc = "The order of the primitive polynomial is 2"]
    PRBS_OUTSEL_2 = 0x02,
    #[doc = "The order of the primitive polynomial is 3"]
    PRBS_OUTSEL_3 = 0x03,
    #[doc = "The order of the primitive polynomial is 4"]
    PRBS_OUTSEL_4 = 0x04,
    #[doc = "The order of the primitive polynomial is 5"]
    PRBS_OUTSEL_5 = 0x05,
    #[doc = "The order of the primitive polynomial is 6"]
    PRBS_OUTSEL_6 = 0x06,
    #[doc = "The order of the primitive polynomial is 7"]
    PRBS_OUTSEL_7 = 0x07,
    #[doc = "The order of the primitive polynomial is 8"]
    PRBS_OUTSEL_8 = 0x08,
    #[doc = "The order of the primitive polynomial is 9"]
    PRBS_OUTSEL_9 = 0x09,
    #[doc = "The order of the primitive polynomial is 10"]
    PRBS_OUTSEL_10 = 0x0a,
    #[doc = "The order of the primitive polynomial is 11"]
    PRBS_OUTSEL_11 = 0x0b,
    #[doc = "The order of the primitive polynomial is 12"]
    PRBS_OUTSEL_12 = 0x0c,
    #[doc = "The order of the primitive polynomial is 13"]
    PRBS_OUTSEL_13 = 0x0d,
    #[doc = "The order of the primitive polynomial is 14"]
    PRBS_OUTSEL_14 = 0x0e,
    #[doc = "The order of the primitive polynomial is 15"]
    PRBS_OUTSEL_15 = 0x0f,
}
impl PrbsOutsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PrbsOutsel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PrbsOutsel {
    #[inline(always)]
    fn from(val: u8) -> PrbsOutsel {
        PrbsOutsel::from_bits(val)
    }
}
impl From<PrbsOutsel> for u8 {
    #[inline(always)]
    fn from(val: PrbsOutsel) -> u8 {
        PrbsOutsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PreChrgMode {
    #[doc = "mode 0"]
    PRE_CHRG_MODE_0 = 0x0,
    #[doc = "mode 1"]
    PRE_CHRG_MODE_1 = 0x01,
}
impl PreChrgMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PreChrgMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PreChrgMode {
    #[inline(always)]
    fn from(val: u8) -> PreChrgMode {
        PreChrgMode::from_bits(val)
    }
}
impl From<PreChrgMode> for u8 {
    #[inline(always)]
    fn from(val: PreChrgMode) -> u8 {
        PreChrgMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RunCtrl {
    #[doc = "gate analog switch clock"]
    RUN_CTRL_0 = 0x0,
    #[doc = "not gate analog switch clock"]
    RUN_CTRL_1 = 0x01,
}
impl RunCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RunCtrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RunCtrl {
    #[inline(always)]
    fn from(val: u8) -> RunCtrl {
        RunCtrl::from_bits(val)
    }
}
impl From<RunCtrl> for u8 {
    #[inline(always)]
    fn from(val: RunCtrl) -> u8 {
        RunCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SNoise {
    #[doc = "noise cancellation off (default)"]
    S_NOISE_0 = 0x0,
    #[doc = "noise cancellation on"]
    S_NOISE_1 = 0x01,
}
impl SNoise {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SNoise {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SNoise {
    #[inline(always)]
    fn from(val: u8) -> SNoise {
        SNoise::from_bits(val)
    }
}
impl From<SNoise> for u8 {
    #[inline(always)]
    fn from(val: SNoise) -> u8 {
        SNoise::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SSen {
    #[doc = "Sensitivity boost off (default)"]
    S_SEN_0 = 0x0,
    #[doc = "Sensitivity boost on"]
    S_SEN_1 = 0x01,
}
impl SSen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SSen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SSen {
    #[inline(always)]
    fn from(val: u8) -> SSen {
        SSen::from_bits(val)
    }
}
impl From<SSen> for u8 {
    #[inline(always)]
    fn from(val: SSen) -> u8 {
        SSen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SXch {
    #[doc = "1/16 (default)"]
    S_XCH_0 = 0x0,
    #[doc = "1/8"]
    S_XCH_1 = 0x01,
    #[doc = "1/4"]
    S_XCH_2 = 0x02,
    #[doc = "1/2"]
    S_XCH_3 = 0x03,
    #[doc = "1/1"]
    S_XCH_4 = 0x04,
    #[doc = "2/1"]
    S_XCH_5 = 0x05,
    #[doc = "4/1"]
    S_XCH_6 = 0x06,
    #[doc = "8/1"]
    S_XCH_7 = 0x07,
}
impl SXch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SXch {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SXch {
    #[inline(always)]
    fn from(val: u8) -> SXch {
        SXch::from_bits(val)
    }
}
impl From<SXch> for u8 {
    #[inline(always)]
    fn from(val: SXch) -> u8 {
        SXch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SXdn {
    #[doc = "1/16 (default)"]
    S_XDN_0 = 0x0,
    #[doc = "1/8"]
    S_XDN_1 = 0x01,
    #[doc = "1/4"]
    S_XDN_2 = 0x02,
    #[doc = "1/2"]
    S_XDN_3 = 0x03,
    #[doc = "NA"]
    S_XDN_4 = 0x04,
    #[doc = "NA"]
    S_XDN_5 = 0x05,
    #[doc = "NA"]
    S_XDN_6 = 0x06,
    #[doc = "NA"]
    S_XDN_7 = 0x07,
}
impl SXdn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SXdn {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SXdn {
    #[inline(always)]
    fn from(val: u8) -> SXdn {
        SXdn::from_bits(val)
    }
}
impl From<SXdn> for u8 {
    #[inline(always)]
    fn from(val: SXdn) -> u8 {
        SXdn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SXin {
    #[doc = "1/8"]
    S_XIN_0 = 0x0,
    #[doc = "1/4 (default)"]
    S_XIN_1 = 0x01,
}
impl SXin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SXin {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SXin {
    #[inline(always)]
    fn from(val: u8) -> SXin {
        SXin::from_bits(val)
    }
}
impl From<SXin> for u8 {
    #[inline(always)]
    fn from(val: SXin) -> u8 {
        SXin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SelfBoostMode {
    #[doc = "cap boost mode (default)"]
    SELF_BOOST_MODE_0 = 0x0,
    #[doc = "current boost mode"]
    SELF_BOOST_MODE_1 = 0x01,
}
impl SelfBoostMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SelfBoostMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SelfBoostMode {
    #[inline(always)]
    fn from(val: u8) -> SelfBoostMode {
        SelfBoostMode::from_bits(val)
    }
}
impl From<SelfBoostMode> for u8 {
    #[inline(always)]
    fn from(val: SelfBoostMode) -> u8 {
        SelfBoostMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SelfBoostScale {
    #[doc = "500 nA (default)"]
    SELF_BOOST_SCALE_0 = 0x0,
    #[doc = "1 uA"]
    SELF_BOOST_SCALE_1 = 0x01,
    #[doc = "1.5 uA"]
    SELF_BOOST_SCALE_2 = 0x02,
    #[doc = "2 uA"]
    SELF_BOOST_SCALE_3 = 0x03,
    #[doc = "2.5 uA"]
    SELF_BOOST_SCALE_4 = 0x04,
    #[doc = "3 uA"]
    SELF_BOOST_SCALE_5 = 0x05,
    #[doc = "3.5 uA"]
    SELF_BOOST_SCALE_6 = 0x06,
    #[doc = "4 uA"]
    SELF_BOOST_SCALE_7 = 0x07,
}
impl SelfBoostScale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SelfBoostScale {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SelfBoostScale {
    #[inline(always)]
    fn from(val: u8) -> SelfBoostScale {
        SelfBoostScale::from_bits(val)
    }
}
impl From<SelfBoostScale> for u8 {
    #[inline(always)]
    fn from(val: SelfBoostScale) -> u8 {
        SelfBoostScale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SelfBypassFlt {
    #[doc = "LPF is not bypassed (default)"]
    SELF_BYPASS_FLT_0 = 0x0,
    #[doc = "LPF is bypassed"]
    SELF_BYPASS_FLT_1 = 0x01,
}
impl SelfBypassFlt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SelfBypassFlt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SelfBypassFlt {
    #[inline(always)]
    fn from(val: u8) -> SelfBypassFlt {
        SelfBypassFlt::from_bits(val)
    }
}
impl From<SelfBypassFlt> for u8 {
    #[inline(always)]
    fn from(val: SelfBypassFlt) -> u8 {
        SelfBypassFlt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SincCutoff {
    #[doc = "div=0"]
    SINC_CUTOFF_0 = 0x0,
    #[doc = "div=2"]
    SINC_CUTOFF_1 = 0x01,
    #[doc = "div=4"]
    SINC_CUTOFF_2 = 0x02,
    #[doc = "div=8"]
    SINC_CUTOFF_3 = 0x03,
    #[doc = "div=16"]
    SINC_CUTOFF_4 = 0x04,
    #[doc = "div=32"]
    SINC_CUTOFF_5 = 0x05,
    #[doc = "div=64"]
    SINC_CUTOFF_6 = 0x06,
    #[doc = "div=128"]
    SINC_CUTOFF_7 = 0x07,
    #[doc = "NC"]
    SINC_CUTOFF_8 = 0x08,
    #[doc = "NC"]
    SINC_CUTOFF_9 = 0x09,
    #[doc = "NC"]
    SINC_CUTOFF_10 = 0x0a,
    #[doc = "NC"]
    SINC_CUTOFF_11 = 0x0b,
    #[doc = "NC"]
    SINC_CUTOFF_12 = 0x0c,
    #[doc = "NC"]
    SINC_CUTOFF_13 = 0x0d,
    #[doc = "NC"]
    SINC_CUTOFF_14 = 0x0e,
    #[doc = "NC"]
    SINC_CUTOFF_15 = 0x0f,
}
impl SincCutoff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SincCutoff {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SincCutoff {
    #[inline(always)]
    fn from(val: u8) -> SincCutoff {
        SincCutoff::from_bits(val)
    }
}
impl From<SincCutoff> for u8 {
    #[inline(always)]
    fn from(val: SincCutoff) -> u8 {
        SincCutoff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SincDecimation {
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 1 triger period"]
    SINC_DECIMATION_0 = 0x0,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 2 triger periods"]
    SINC_DECIMATION_1 = 0x01,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 3 triger periods"]
    SINC_DECIMATION_2 = 0x02,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 4 triger periods"]
    SINC_DECIMATION_3 = 0x03,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 5 triger periods"]
    SINC_DECIMATION_4 = 0x04,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 6 triger periods"]
    SINC_DECIMATION_5 = 0x05,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 7 triger periods"]
    SINC_DECIMATION_6 = 0x06,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 8 triger periods"]
    SINC_DECIMATION_7 = 0x07,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 9 triger periods"]
    SINC_DECIMATION_8 = 0x08,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 10 triger periods"]
    SINC_DECIMATION_9 = 0x09,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 11 triger periods"]
    SINC_DECIMATION_10 = 0x0a,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 12 triger periods"]
    SINC_DECIMATION_11 = 0x0b,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 13 triger periods"]
    SINC_DECIMATION_12 = 0x0c,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 14 triger periods"]
    SINC_DECIMATION_13 = 0x0d,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 15 triger periods"]
    SINC_DECIMATION_14 = 0x0e,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 16 triger periods"]
    SINC_DECIMATION_15 = 0x0f,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 17 triger periods"]
    SINC_DECIMATION_16 = 0x10,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 18 triger periods"]
    SINC_DECIMATION_17 = 0x11,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 19 triger periods"]
    SINC_DECIMATION_18 = 0x12,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 20 triger periods"]
    SINC_DECIMATION_19 = 0x13,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 21 triger periods"]
    SINC_DECIMATION_20 = 0x14,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 22 triger periods"]
    SINC_DECIMATION_21 = 0x15,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 23 triger periods"]
    SINC_DECIMATION_22 = 0x16,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 24 triger periods"]
    SINC_DECIMATION_23 = 0x17,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 25 triger periods"]
    SINC_DECIMATION_24 = 0x18,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 26 triger periods"]
    SINC_DECIMATION_25 = 0x19,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 27 triger periods"]
    SINC_DECIMATION_26 = 0x1a,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 28 triger periods"]
    SINC_DECIMATION_27 = 0x1b,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 29 triger periods"]
    SINC_DECIMATION_28 = 0x1c,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 30 triger periods"]
    SINC_DECIMATION_29 = 0x1d,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 31 triger periods"]
    SINC_DECIMATION_30 = 0x1e,
    #[doc = "The TSI_DATA\\[TSICNT\\] bits is the counter value of 32 triger periods"]
    SINC_DECIMATION_31 = 0x1f,
}
impl SincDecimation {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SincDecimation {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SincDecimation {
    #[inline(always)]
    fn from(val: u8) -> SincDecimation {
        SincDecimation::from_bits(val)
    }
}
impl From<SincDecimation> for u8 {
    #[inline(always)]
    fn from(val: SincDecimation) -> u8 {
        SincDecimation::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SincOrder {
    #[doc = "Using 1 order sinc filter"]
    SINC_ORDER_0 = 0x0,
    #[doc = "Using 2 order sinc filter"]
    SINC_ORDER_1 = 0x01,
}
impl SincOrder {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SincOrder {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SincOrder {
    #[inline(always)]
    fn from(val: u8) -> SincOrder {
        SincOrder::from_bits(val)
    }
}
impl From<SincOrder> for u8 {
    #[inline(always)]
    fn from(val: SincOrder) -> u8 {
        SincOrder::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SincOverflowFlag {
    #[doc = "The counter result has no overflow occurrence in the last scan process"]
    SINC_OVERFLOW_FLAG_0 = 0x0,
    #[doc = "The counter result has an overflow occurrence in the last scan process"]
    SINC_OVERFLOW_FLAG_1 = 0x01,
}
impl SincOverflowFlag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SincOverflowFlag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SincOverflowFlag {
    #[inline(always)]
    fn from(val: u8) -> SincOverflowFlag {
        SincOverflowFlag::from_bits(val)
    }
}
impl From<SincOverflowFlag> for u8 {
    #[inline(always)]
    fn from(val: SincOverflowFlag) -> u8 {
        SincOverflowFlag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SincValid {
    #[doc = "SINC filter is disabled"]
    SINC_VALID_0 = 0x0,
    #[doc = "SINC filter is enabled"]
    SINC_VALID_1 = 0x01,
}
impl SincValid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SincValid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SincValid {
    #[inline(always)]
    fn from(val: u8) -> SincValid {
        SincValid::from_bits(val)
    }
}
impl From<SincValid> for u8 {
    #[inline(always)]
    fn from(val: SincValid) -> u8 {
        SincValid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SscControlOut {
    #[doc = "SSC output value is 0"]
    SSC_CONTROL_OUT_0 = 0x0,
    #[doc = "SSC output value is 1"]
    SSC_CONTROL_OUT_1 = 0x01,
}
impl SscControlOut {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SscControlOut {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SscControlOut {
    #[inline(always)]
    fn from(val: u8) -> SscControlOut {
        SscControlOut::from_bits(val)
    }
}
impl From<SscControlOut> for u8 {
    #[inline(always)]
    fn from(val: SscControlOut) -> u8 {
        SscControlOut::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SscControlReverse {
    #[doc = "Keep the polarity of the SSC output bit"]
    SSC_CONTROL_REVERSE_0 = 0x0,
    #[doc = "Reverse the polarity of the SSC output bit"]
    SSC_CONTROL_REVERSE_1 = 0x01,
}
impl SscControlReverse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SscControlReverse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SscControlReverse {
    #[inline(always)]
    fn from(val: u8) -> SscControlReverse {
        SscControlReverse::from_bits(val)
    }
}
impl From<SscControlReverse> for u8 {
    #[inline(always)]
    fn from(val: SscControlReverse) -> u8 {
        SscControlReverse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SscMode {
    #[doc = "Using PRBS method generating SSC output bit"]
    SSC_MODE_0 = 0x0,
    #[doc = "Using up-down counter generating SSC output bit"]
    SSC_MODE_1 = 0x01,
    #[doc = "SSC function is disabled"]
    SSC_MODE_2 = 0x02,
    #[doc = "NC"]
    SSC_MODE_3 = 0x03,
}
impl SscMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SscMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SscMode {
    #[inline(always)]
    fn from(val: u8) -> SscMode {
        SscMode::from_bits(val)
    }
}
impl From<SscMode> for u8 {
    #[inline(always)]
    fn from(val: SscMode) -> u8 {
        SscMode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SscPrescaleNum(u16);
impl SscPrescaleNum {
    #[doc = "div1"]
    pub const SSC_PRESCALE_NUM_0: Self = Self(0x0);
    #[doc = "div2"]
    pub const SSC_PRESCALE_NUM_1: Self = Self(0x01);
    #[doc = "div4"]
    pub const SSC_PRESCALE_NUM_3: Self = Self(0x03);
    #[doc = "div8"]
    pub const SSC_PRESCALE_NUM_7: Self = Self(0x07);
    #[doc = "div16"]
    pub const SSC_PRESCALE_NUM_15: Self = Self(0x0f);
    #[doc = "div32"]
    pub const SSC_PRESCALE_NUM_31: Self = Self(0x1f);
    #[doc = "div64"]
    pub const SSC_PRESCALE_NUM_63: Self = Self(0x3f);
    #[doc = "div128"]
    pub const SSC_PRESCALE_NUM_127: Self = Self(0x7f);
    #[doc = "div256"]
    pub const SSC_PRESCALE_NUM_255: Self = Self(0xff);
}
impl SscPrescaleNum {
    pub const fn from_bits(val: u16) -> SscPrescaleNum {
        Self(val & 0x0fff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for SscPrescaleNum {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("SSC_PRESCALE_NUM_0"),
            0x01 => f.write_str("SSC_PRESCALE_NUM_1"),
            0x03 => f.write_str("SSC_PRESCALE_NUM_3"),
            0x07 => f.write_str("SSC_PRESCALE_NUM_7"),
            0x0f => f.write_str("SSC_PRESCALE_NUM_15"),
            0x1f => f.write_str("SSC_PRESCALE_NUM_31"),
            0x3f => f.write_str("SSC_PRESCALE_NUM_63"),
            0x7f => f.write_str("SSC_PRESCALE_NUM_127"),
            0xff => f.write_str("SSC_PRESCALE_NUM_255"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SscPrescaleNum {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "SSC_PRESCALE_NUM_0"),
            0x01 => defmt::write!(f, "SSC_PRESCALE_NUM_1"),
            0x03 => defmt::write!(f, "SSC_PRESCALE_NUM_3"),
            0x07 => defmt::write!(f, "SSC_PRESCALE_NUM_7"),
            0x0f => defmt::write!(f, "SSC_PRESCALE_NUM_15"),
            0x1f => defmt::write!(f, "SSC_PRESCALE_NUM_31"),
            0x3f => defmt::write!(f, "SSC_PRESCALE_NUM_63"),
            0x7f => defmt::write!(f, "SSC_PRESCALE_NUM_127"),
            0xff => defmt::write!(f, "SSC_PRESCALE_NUM_255"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for SscPrescaleNum {
    #[inline(always)]
    fn from(val: u16) -> SscPrescaleNum {
        SscPrescaleNum::from_bits(val)
    }
}
impl From<SscPrescaleNum> for u16 {
    #[inline(always)]
    fn from(val: SscPrescaleNum) -> u16 {
        SscPrescaleNum::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stm {
    #[doc = "Software trigger scan"]
    STM_0 = 0x0,
    #[doc = "Hardware trigger scan"]
    STM_1 = 0x01,
}
impl Stm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stm {
    #[inline(always)]
    fn from(val: u8) -> Stm {
        Stm::from_bits(val)
    }
}
impl From<Stm> for u8 {
    #[inline(always)]
    fn from(val: Stm) -> u8 {
        Stm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwitchEnable {
    #[doc = "SSC function is disabled"]
    SWITCH_ENABLE_0 = 0x0,
    #[doc = "SSC function is enabled"]
    SWITCH_ENABLE_1 = 0x01,
}
impl SwitchEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwitchEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwitchEnable {
    #[inline(always)]
    fn from(val: u8) -> SwitchEnable {
        SwitchEnable::from_bits(val)
    }
}
impl From<SwitchEnable> for u8 {
    #[inline(always)]
    fn from(val: SwitchEnable) -> u8 {
        SwitchEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swts {
    #[doc = "No effect"]
    SWTS_0 = 0x0,
    #[doc = "Takes effect"]
    SWTS_1 = 0x01,
}
impl Swts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swts {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swts {
    #[inline(always)]
    fn from(val: u8) -> Swts {
        Swts::from_bits(val)
    }
}
impl From<Swts> for u8 {
    #[inline(always)]
    fn from(val: Swts) -> u8 {
        Swts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ThesholdRatio {
    #[doc = "The thresholdh = baseline+counter/2, and thresholdl = baseline-counter/2"]
    THESHOLD_RATIO_0 = 0x0,
    #[doc = "The thresholdh = baseline+counter/4, and thresholdl = baseline-counter/4"]
    THESHOLD_RATIO_1 = 0x01,
    #[doc = "The thresholdh = baseline+counter/8, and thresholdl = baseline-counter/8"]
    THESHOLD_RATIO_2 = 0x02,
    #[doc = "The thresholdh = baseline+counter/16, and thresholdl = baseline-counter/16"]
    THESHOLD_RATIO_3 = 0x03,
    #[doc = "The thresholdh = baseline+counter/32, and thresholdl = baseline-counter/32"]
    THESHOLD_RATIO_4 = 0x04,
    #[doc = "The thresholdh = baseline+counter/64, and thresholdl = baseline-counter/64"]
    THESHOLD_RATIO_5 = 0x05,
    #[doc = "The thresholdh = baseline+counter/128, and thresholdl = baseline-counter/128"]
    THESHOLD_RATIO_6 = 0x06,
    #[doc = "The thresholdh = baseline+counter/256, and thresholdl = baseline-counter/256"]
    THESHOLD_RATIO_7 = 0x07,
}
impl ThesholdRatio {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ThesholdRatio {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ThesholdRatio {
    #[inline(always)]
    fn from(val: u8) -> ThesholdRatio {
        ThesholdRatio::from_bits(val)
    }
}
impl From<ThesholdRatio> for u8 {
    #[inline(always)]
    fn from(val: ThesholdRatio) -> u8 {
        ThesholdRatio::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ThresholdTraceEn {
    #[doc = "Disable threshold trace function"]
    THRESHOLD_TRACE_EN_0 = 0x0,
    #[doc = "Enable threshold trace function"]
    THRESHOLD_TRACE_EN_1 = 0x01,
}
impl ThresholdTraceEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ThresholdTraceEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ThresholdTraceEn {
    #[inline(always)]
    fn from(val: u8) -> ThresholdTraceEn {
        ThresholdTraceEn::from_bits(val)
    }
}
impl From<ThresholdTraceEn> for u8 {
    #[inline(always)]
    fn from(val: ThresholdTraceEn) -> u8 {
        ThresholdTraceEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrigClkDivider {
    #[doc = "No divider"]
    TRIG_CLK_DIVIDER_0 = 0x0,
    #[doc = "Divided by 2"]
    TRIG_CLK_DIVIDER_1 = 0x01,
    #[doc = "Divided by 3"]
    TRIG_CLK_DIVIDER_2 = 0x02,
    #[doc = "Divided by 4"]
    TRIG_CLK_DIVIDER_3 = 0x03,
    #[doc = "Divided by 5"]
    TRIG_CLK_DIVIDER_4 = 0x04,
    #[doc = "Divided by 6"]
    TRIG_CLK_DIVIDER_5 = 0x05,
    #[doc = "Divided by 7"]
    TRIG_CLK_DIVIDER_6 = 0x06,
    #[doc = "Divided by 8"]
    TRIG_CLK_DIVIDER_7 = 0x07,
    #[doc = "Divided by 9"]
    TRIG_CLK_DIVIDER_8 = 0x08,
    #[doc = "Divided by 10"]
    TRIG_CLK_DIVIDER_9 = 0x09,
    #[doc = "Divided by 11"]
    TRIG_CLK_DIVIDER_10 = 0x0a,
    #[doc = "Divided by 12"]
    TRIG_CLK_DIVIDER_11 = 0x0b,
    #[doc = "Divided by 13"]
    TRIG_CLK_DIVIDER_12 = 0x0c,
    #[doc = "Divided by 14"]
    TRIG_CLK_DIVIDER_13 = 0x0d,
    #[doc = "Divided by 15"]
    TRIG_CLK_DIVIDER_14 = 0x0e,
    #[doc = "Divided by 16"]
    TRIG_CLK_DIVIDER_15 = 0x0f,
    #[doc = "Divided by 17"]
    TRIG_CLK_DIVIDER_16 = 0x10,
    #[doc = "Divided by 18"]
    TRIG_CLK_DIVIDER_17 = 0x11,
    #[doc = "Divided by 19"]
    TRIG_CLK_DIVIDER_18 = 0x12,
    #[doc = "Divided by 20"]
    TRIG_CLK_DIVIDER_19 = 0x13,
    #[doc = "Divided by 21"]
    TRIG_CLK_DIVIDER_20 = 0x14,
    #[doc = "Divided by 22"]
    TRIG_CLK_DIVIDER_21 = 0x15,
    #[doc = "Divided by 23"]
    TRIG_CLK_DIVIDER_22 = 0x16,
    #[doc = "Divided by 24"]
    TRIG_CLK_DIVIDER_23 = 0x17,
    #[doc = "Divided by 25"]
    TRIG_CLK_DIVIDER_24 = 0x18,
    #[doc = "Divided by 26"]
    TRIG_CLK_DIVIDER_25 = 0x19,
    #[doc = "Divided by 27"]
    TRIG_CLK_DIVIDER_26 = 0x1a,
    #[doc = "Divided by 28"]
    TRIG_CLK_DIVIDER_27 = 0x1b,
    #[doc = "Divided by 29"]
    TRIG_CLK_DIVIDER_28 = 0x1c,
    #[doc = "Divided by 30"]
    TRIG_CLK_DIVIDER_29 = 0x1d,
    #[doc = "Divided by 31"]
    TRIG_CLK_DIVIDER_30 = 0x1e,
    #[doc = "Divided by 32"]
    TRIG_CLK_DIVIDER_31 = 0x1f,
}
impl TrigClkDivider {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrigClkDivider {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrigClkDivider {
    #[inline(always)]
    fn from(val: u8) -> TrigClkDivider {
        TrigClkDivider::from_bits(val)
    }
}
impl From<TrigClkDivider> for u8 {
    #[inline(always)]
    fn from(val: TrigClkDivider) -> u8 {
        TrigClkDivider::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrigClkSel {
    #[doc = "32 KHz clock"]
    TRIG_CLK_SEL_0 = 0x0,
    #[doc = "clksoc"]
    TRIG_CLK_SEL_1 = 0x01,
}
impl TrigClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrigClkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrigClkSel {
    #[inline(always)]
    fn from(val: u8) -> TrigClkSel {
        TrigClkSel::from_bits(val)
    }
}
impl From<TrigClkSel> for u8 {
    #[inline(always)]
    fn from(val: TrigClkSel) -> u8 {
        TrigClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrigEn {
    #[doc = "disabled"]
    TRIG_EN_0 = 0x0,
    #[doc = "enabled"]
    TRIG_EN_1 = 0x01,
}
impl TrigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrigEn {
    #[inline(always)]
    fn from(val: u8) -> TrigEn {
        TrigEn::from_bits(val)
    }
}
impl From<TrigEn> for u8 {
    #[inline(always)]
    fn from(val: TrigEn) -> u8 {
        TrigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TsiAnaTestSetting {
    #[doc = "Normal operation mode"]
    TSI_ANA_TEST_SETTING_0 = 0x0,
    #[doc = "Test mode 1, Vref test"]
    TSI_ANA_TEST_SETTING_1 = 0x01,
    #[doc = "Test mode 2, Vprechrg voltage test"]
    TSI_ANA_TEST_SETTING_2 = 0x02,
    #[doc = "Test mode 3, Comparator test"]
    TSI_ANA_TEST_SETTING_3 = 0x03,
    #[doc = "Test mode 4, Precharge function test"]
    TSI_ANA_TEST_SETTING_4 = 0x04,
    #[doc = "Test mode 5, 4uA bias test"]
    TSI_ANA_TEST_SETTING_5 = 0x05,
    #[doc = "Test mode 6, 2uA bias test"]
    TSI_ANA_TEST_SETTING_6 = 0x06,
    #[doc = "Test mode 7, Function monitor test"]
    TSI_ANA_TEST_SETTING_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl TsiAnaTestSetting {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TsiAnaTestSetting {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TsiAnaTestSetting {
    #[inline(always)]
    fn from(val: u8) -> TsiAnaTestSetting {
        TsiAnaTestSetting::from_bits(val)
    }
}
impl From<TsiAnaTestSetting> for u8 {
    #[inline(always)]
    fn from(val: TsiAnaTestSetting) -> u8 {
        TsiAnaTestSetting::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TsiTestEn {
    #[doc = "Test mode is disabled"]
    TSI_TEST_EN_0 = 0x0,
    #[doc = "Test mode is enabled"]
    TSI_TEST_EN_1 = 0x01,
}
impl TsiTestEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TsiTestEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TsiTestEn {
    #[inline(always)]
    fn from(val: u8) -> TsiTestEn {
        TsiTestEn::from_bits(val)
    }
}
impl From<TsiTestEn> for u8 {
    #[inline(always)]
    fn from(val: TsiTestEn) -> u8 {
        TsiTestEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TsicntOverflowFlag {
    #[doc = "The DATA\\[TSICNT\\] value has no overflow occurrence in the last scan process"]
    TSICNT_OVERFLOW_FLAG_0 = 0x0,
    #[doc = "The DATA\\[TSICNT\\] value has an overflow occurrence in the last scan process"]
    TSICNT_OVERFLOW_FLAG_1 = 0x01,
}
impl TsicntOverflowFlag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TsicntOverflowFlag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TsicntOverflowFlag {
    #[inline(always)]
    fn from(val: u8) -> TsicntOverflowFlag {
        TsicntOverflowFlag::from_bits(val)
    }
}
impl From<TsicntOverflowFlag> for u8 {
    #[inline(always)]
    fn from(val: TsicntOverflowFlag) -> u8 {
        TsicntOverflowFlag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsien {
    #[doc = "TSI module disabled"]
    TSIEN_0 = 0x0,
    #[doc = "TSI module enabled"]
    TSIEN_1 = 0x01,
}
impl Tsien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsien {
    #[inline(always)]
    fn from(val: u8) -> Tsien {
        Tsien::from_bits(val)
    }
}
impl From<Tsien> for u8 {
    #[inline(always)]
    fn from(val: Tsien) -> u8 {
        Tsien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TsienDpd {
    #[doc = "3v domain logic is disabled"]
    TSIEN_DPD_0 = 0x0,
    #[doc = "3v domain logic is enabled"]
    TSIEN_DPD_1 = 0x01,
}
impl TsienDpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TsienDpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TsienDpd {
    #[inline(always)]
    fn from(val: u8) -> TsienDpd {
        TsienDpd::from_bits(val)
    }
}
impl From<TsienDpd> for u8 {
    #[inline(always)]
    fn from(val: TsienDpd) -> u8 {
        TsienDpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XinScaleDown {
    #[doc = "1 (default)"]
    XIN_SCALE_DOWN_0 = 0x0,
    #[doc = "1/2"]
    XIN_SCALE_DOWN_1 = 0x01,
    #[doc = "1/3"]
    XIN_SCALE_DOWN_2 = 0x02,
    #[doc = "1/4"]
    XIN_SCALE_DOWN_3 = 0x03,
    #[doc = "1/5"]
    XIN_SCALE_DOWN_4 = 0x04,
    #[doc = "1/6"]
    XIN_SCALE_DOWN_5 = 0x05,
    #[doc = "1/7"]
    XIN_SCALE_DOWN_6 = 0x06,
    #[doc = "1/8"]
    XIN_SCALE_DOWN_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl XinScaleDown {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XinScaleDown {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XinScaleDown {
    #[inline(always)]
    fn from(val: u8) -> XinScaleDown {
        XinScaleDown::from_bits(val)
    }
}
impl From<XinScaleDown> for u8 {
    #[inline(always)]
    fn from(val: XinScaleDown) -> u8 {
        XinScaleDown::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XinScaleDownTrim {
    #[doc = "1 (default)"]
    XIN_SCALE_DOWN_TRIM_0 = 0x0,
    #[doc = "1/2"]
    XIN_SCALE_DOWN_TRIM_1 = 0x01,
    #[doc = "1/3"]
    XIN_SCALE_DOWN_TRIM_2 = 0x02,
    #[doc = "1/4"]
    XIN_SCALE_DOWN_TRIM_3 = 0x03,
    #[doc = "1/5"]
    XIN_SCALE_DOWN_TRIM_4 = 0x04,
    #[doc = "1/6"]
    XIN_SCALE_DOWN_TRIM_5 = 0x05,
    #[doc = "1/7"]
    XIN_SCALE_DOWN_TRIM_6 = 0x06,
    #[doc = "1/8"]
    XIN_SCALE_DOWN_TRIM_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl XinScaleDownTrim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XinScaleDownTrim {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XinScaleDownTrim {
    #[inline(always)]
    fn from(val: u8) -> XinScaleDownTrim {
        XinScaleDownTrim::from_bits(val)
    }
}
impl From<XinScaleDownTrim> for u8 {
    #[inline(always)]
    fn from(val: XinScaleDownTrim) -> u8 {
        XinScaleDownTrim::to_bits(val)
    }
}
