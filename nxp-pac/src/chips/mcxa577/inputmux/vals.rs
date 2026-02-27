#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcTrigTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected"]
    VAL1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected"]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected"]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected"]
    VAL4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected"]
    VAL5 = 0x05,
    #[doc = "CMP0_OUT input is selected"]
    VAL6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT0 input is selected"]
    VAL9 = 0x09,
    #[doc = "CTimer0_MAT1 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CTimer1_MAT0 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CTimer1_MAT1 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CTimer2_MAT0 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CTimer2_MAT1 input is selected"]
    VAL14 = 0x0e,
    #[doc = "LPTMR0 input is selected"]
    VAL15 = 0x0f,
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
    #[doc = "GPIO0 Pin Event Trig 0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected"]
    VAL27 = 0x1b,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL28 = 0x1c,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL29 = 0x1d,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected"]
    VAL30 = 0x1e,
    #[doc = "WUU"]
    VAL31 = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    VAL37 = 0x25,
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    VAL38 = 0x26,
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    VAL39 = 0x27,
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    VAL40 = 0x28,
    #[doc = "CTimer3_MAT0 input is selected"]
    VAL41 = 0x29,
    #[doc = "CTimer3_MAT1 input is selected"]
    VAL42 = 0x2a,
    #[doc = "CTimer4_MAT0 input is selected"]
    VAL43 = 0x2b,
    #[doc = "CTimer4_MAT1 input is selected"]
    VAL44 = 0x2c,
    #[doc = "FlexIO0 CH0 input is selected"]
    VAL45 = 0x2d,
    #[doc = "FlexIO0 CH1 input is selected"]
    VAL46 = 0x2e,
    #[doc = "FlexIO0 CH2 input is selected"]
    VAL47 = 0x2f,
    #[doc = "FlexIO0 CH3 input is selected"]
    VAL48 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    #[doc = "GPIO0 Pin Event Trig 1 input is selected"]
    VAL58 = 0x3a,
    #[doc = "GPIO1 Pin Event Trig 1 input is selected"]
    VAL59 = 0x3b,
    #[doc = "GPIO2 Pin Event Trig 1 input is seleected"]
    VAL60 = 0x3c,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL61 = 0x3d,
    #[doc = "GPIO4 Pin Event Trig 1 input is selected"]
    VAL62 = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl AdcTrigTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcTrigTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcTrigTrigin {
    #[inline(always)]
    fn from(val: u8) -> AdcTrigTrigin {
        AdcTrigTrigin::from_bits(val)
    }
}
impl From<AdcTrigTrigin> for u8 {
    #[inline(always)]
    fn from(val: AdcTrigTrigin) -> u8 {
        AdcTrigTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AoiInputInp {
    _RESERVED_0 = 0x0,
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    VAL1 = 0x01,
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    VAL2 = 0x02,
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    VAL3 = 0x03,
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    VAL4 = 0x04,
    #[doc = "CMP0_OUT input is selected"]
    VAL5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "CTimer0_MAT0 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTimer0_MAT1 input is selected"]
    VAL9 = 0x09,
    #[doc = "CTimer0_MAT2 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CTimer0_MAT3 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CTimer1_MAT0 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CTimer1_MAT1 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CTimer1_MAT2 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CTimer1_MAT3 input is selected"]
    VAL15 = 0x0f,
    #[doc = "CTimer2_MAT0 input is selected"]
    VAL16 = 0x10,
    #[doc = "CTimer2_MAT1 input is selected"]
    VAL17 = 0x11,
    #[doc = "CTimer2_MAT2 input is selected"]
    VAL18 = 0x12,
    #[doc = "CTimer2_MAT3 input is selected"]
    VAL19 = 0x13,
    #[doc = "LPTMR0 input is selected"]
    VAL20 = 0x14,
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
    #[doc = "TRIG_IN0 input is selected"]
    VAL35 = 0x23,
    #[doc = "TRIG_IN1 input is selected"]
    VAL36 = 0x24,
    #[doc = "TRIG_IN2 input is selected"]
    VAL37 = 0x25,
    #[doc = "TRIG_IN3 input is selected"]
    VAL38 = 0x26,
    #[doc = "TRIG_IN4 input is selected"]
    VAL39 = 0x27,
    #[doc = "TRIG_IN5 input is selected"]
    VAL40 = 0x28,
    #[doc = "TRIG_IN6 input is selected"]
    VAL41 = 0x29,
    #[doc = "TRIG_IN7 input is selected"]
    VAL42 = 0x2a,
    #[doc = "TRIG_IN8 input is selected"]
    VAL43 = 0x2b,
    #[doc = "TRIG_IN9 input is selected"]
    VAL44 = 0x2c,
    #[doc = "TRIG_IN10 input is selected"]
    VAL45 = 0x2d,
    #[doc = "TRIG_IN11 input is selected"]
    VAL46 = 0x2e,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected"]
    VAL47 = 0x2f,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected"]
    VAL48 = 0x30,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL49 = 0x31,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL50 = 0x32,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected"]
    VAL51 = 0x33,
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    VAL52 = 0x34,
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    VAL53 = 0x35,
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    VAL54 = 0x36,
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    VAL55 = 0x37,
    #[doc = "CTimer3_MAT0 input is selected"]
    VAL56 = 0x38,
    #[doc = "CTimer3_MAT1 input is selected"]
    VAL57 = 0x39,
    #[doc = "CTimer3_MAT2 input is selected"]
    VAL58 = 0x3a,
    #[doc = "CTimer3_MAT3 input is selected"]
    VAL59 = 0x3b,
    #[doc = "CTimer4_MAT0 input is selected"]
    VAL60 = 0x3c,
    #[doc = "CTimer4_MAT1 input is selected"]
    VAL61 = 0x3d,
    #[doc = "CTimer4_MAT2 input is selected"]
    VAL62 = 0x3e,
    #[doc = "CTimer4_MAT3 input is selected"]
    VAL63 = 0x3f,
    #[doc = "FlexIO CH0 input is selected"]
    VAL64 = 0x40,
    #[doc = "FlexIO CH1 input is selected"]
    VAL65 = 0x41,
    #[doc = "FlexIO CH2 input is selected"]
    VAL66 = 0x42,
    #[doc = "FlexIO CH3 input is selected"]
    VAL67 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    #[doc = "GPIO0 Pin Event Trig 1 input is selected"]
    VAL97 = 0x61,
    #[doc = "GPIO1 Pin Event Trig 1 input is selected"]
    VAL98 = 0x62,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL99 = 0x63,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL100 = 0x64,
    #[doc = "GPIO4 Pin Event Trig 1 input is selected"]
    VAL101 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl AoiInputInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AoiInputInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AoiInputInp {
    #[inline(always)]
    fn from(val: u8) -> AoiInputInp {
        AoiInputInp::from_bits(val)
    }
}
impl From<AoiInputInp> for u8 {
    #[inline(always)]
    fn from(val: AoiInputInp) -> u8 {
        AoiInputInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmpTrigTrigin {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected"]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected"]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected"]
    VAL4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected"]
    VAL5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "CTimer0_MAT0 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected"]
    VAL9 = 0x09,
    #[doc = "CTimer1_MAT0"]
    VAL10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CTimer2_MAT0 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected"]
    VAL13 = 0x0d,
    #[doc = "LPTMR0 input is selected"]
    VAL14 = 0x0e,
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
    #[doc = "GPIO0 Pin Event Trig 0 input is selected"]
    VAL25 = 0x19,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL27 = 0x1b,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL28 = 0x1c,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected"]
    VAL29 = 0x1d,
    #[doc = "WUU input is selected"]
    VAL30 = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    #[doc = "CTimer3_MAT0 input is selected"]
    VAL39 = 0x27,
    #[doc = "CTimer3_MAT1 input is selected"]
    VAL40 = 0x28,
    #[doc = "CTimer4_MAT0 input is selected"]
    VAL41 = 0x29,
    #[doc = "CTimer4_MAT1 input is selected"]
    VAL42 = 0x2a,
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
    #[doc = "GPIO0 Pin Event Trig 1 input is selected"]
    VAL56 = 0x38,
    #[doc = "GPIO1 Pin Event Trig 1 input is selected"]
    VAL57 = 0x39,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL58 = 0x3a,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL59 = 0x3b,
    #[doc = "GPIO4 Pin Event Trig 1 input is selected"]
    VAL60 = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl CmpTrigTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmpTrigTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmpTrigTrigin {
    #[inline(always)]
    fn from(val: u8) -> CmpTrigTrigin {
        CmpTrigTrigin::from_bits(val)
    }
}
impl From<CmpTrigTrigin> for u8 {
    #[inline(always)]
    fn from(val: CmpTrigTrigin) -> u8 {
        CmpTrigTrigin::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ctimer0capInp(u8);
impl Ctimer0capInp {
    #[doc = "CT_INP0 input is selected"]
    pub const VAL1: Self = Self(0x01);
    #[doc = "CT_INP1 input is selected"]
    pub const VAL2: Self = Self(0x02);
    #[doc = "CT_INP2 input is selected"]
    pub const VAL3: Self = Self(0x03);
    #[doc = "CT_INP3 input is selected"]
    pub const VAL4: Self = Self(0x04);
    #[doc = "CT_INP4 input is selected"]
    pub const VAL5: Self = Self(0x05);
    #[doc = "CT_INP5 input is selected"]
    pub const VAL6: Self = Self(0x06);
    #[doc = "CT_INP6 input is selected"]
    pub const VAL7: Self = Self(0x07);
    #[doc = "CT_INP7 input is selected"]
    pub const VAL8: Self = Self(0x08);
    #[doc = "CT_INP8 input is selected"]
    pub const VAL9: Self = Self(0x09);
    #[doc = "CT_INP9 input is selected"]
    pub const VAL10: Self = Self(0x0a);
    #[doc = "CT_INP10 input is selected"]
    pub const VAL11: Self = Self(0x0b);
    #[doc = "CT_INP11 input is selected"]
    pub const VAL12: Self = Self(0x0c);
    #[doc = "CT_INP12 input is selected"]
    pub const VAL13: Self = Self(0x0d);
    #[doc = "CT_INP13 input is selected"]
    pub const VAL14: Self = Self(0x0e);
    #[doc = "CT_INP14 input is selected"]
    pub const VAL15: Self = Self(0x0f);
    #[doc = "CT_INP15 input is selected"]
    pub const VAL16: Self = Self(0x10);
    #[doc = "CT_INP16 input is selected"]
    pub const VAL17: Self = Self(0x11);
    #[doc = "CT_INP17 input is selected"]
    pub const VAL18: Self = Self(0x12);
    #[doc = "CT_INP18 input is selected"]
    pub const VAL19: Self = Self(0x13);
    #[doc = "CT_INP19 input is selected"]
    pub const VAL20: Self = Self(0x14);
    #[doc = "AOI0_OUT0 input is selected"]
    pub const VAL22: Self = Self(0x16);
    #[doc = "AOI0_OUT1 input is selected"]
    pub const VAL23: Self = Self(0x17);
    #[doc = "AOI0_OUT2 input is selected"]
    pub const VAL24: Self = Self(0x18);
    #[doc = "AOI0_OUT3 input is selected"]
    pub const VAL25: Self = Self(0x19);
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    pub const VAL26: Self = Self(0x1a);
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    pub const VAL27: Self = Self(0x1b);
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    pub const VAL28: Self = Self(0x1c);
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    pub const VAL29: Self = Self(0x1d);
    #[doc = "CMP0_OUT is selected"]
    pub const VAL30: Self = Self(0x1e);
    #[doc = "CTimer1_MAT1 input is selected"]
    pub const VAL33: Self = Self(0x21);
    #[doc = "CTimer1_MAT2 input is selected"]
    pub const VAL34: Self = Self(0x22);
    #[doc = "CTimer1_MAT3 input is selected"]
    pub const VAL35: Self = Self(0x23);
    #[doc = "CTimer2_MAT1 input is selected"]
    pub const VAL36: Self = Self(0x24);
    #[doc = "CTimer2_MAT2 input is selected"]
    pub const VAL37: Self = Self(0x25);
    #[doc = "CTimer2_MAT3 input is selected"]
    pub const VAL38: Self = Self(0x26);
    #[doc = "LPI2C0 Master End of Packet input is selected"]
    pub const VAL48: Self = Self(0x30);
    #[doc = "LPI2C0 Slave End of Packet input is selected"]
    pub const VAL49: Self = Self(0x31);
    #[doc = "LPI2C1 Master End of Packet input is selected"]
    pub const VAL50: Self = Self(0x32);
    #[doc = "LPI2C1 Slave End of Packet input is selected"]
    pub const VAL51: Self = Self(0x33);
    #[doc = "LPSPI0 End of Frame input is selected"]
    pub const VAL52: Self = Self(0x34);
    #[doc = "LPSPI0 Received Data Word input is selected"]
    pub const VAL53: Self = Self(0x35);
    #[doc = "LPSPI1 End of Frame input is selected"]
    pub const VAL54: Self = Self(0x36);
    #[doc = "LPSPI1 Received Data Word input is selected"]
    pub const VAL55: Self = Self(0x37);
    #[doc = "LPUART0 Received Data Word input is selected"]
    pub const VAL56: Self = Self(0x38);
    #[doc = "LPUART0 Transmitted Data Word input is selected"]
    pub const VAL57: Self = Self(0x39);
    #[doc = "LPUART0 Receive Line Idle input is selected"]
    pub const VAL58: Self = Self(0x3a);
    #[doc = "LPUART1 Received Data Word input is selected"]
    pub const VAL59: Self = Self(0x3b);
    #[doc = "LPUART1 Transmitted Data Word input is selected"]
    pub const VAL60: Self = Self(0x3c);
    #[doc = "LPUART1 Receive Line Idle input is selected"]
    pub const VAL61: Self = Self(0x3d);
    #[doc = "LPUART2 Received Data Word input is selected"]
    pub const VAL62: Self = Self(0x3e);
    #[doc = "LPUART2 Transmitted Data Word input is selected"]
    pub const VAL63: Self = Self(0x3f);
    #[doc = "LPUART2 Receive Line Idle input is selected"]
    pub const VAL64: Self = Self(0x40);
    #[doc = "LPUART3 Received Data Word input is selected"]
    pub const VAL65: Self = Self(0x41);
    #[doc = "LPUART3 Transmitted Data Word input is selected"]
    pub const VAL66: Self = Self(0x42);
    #[doc = "LPUART3 Receive Line Idle input is selected"]
    pub const VAL67: Self = Self(0x43);
    #[doc = "LPUART4 Received Data Word input is selected"]
    pub const VAL68: Self = Self(0x44);
    #[doc = "LPUART4 Transmitted Data Word input is selected"]
    pub const VAL69: Self = Self(0x45);
    #[doc = "LPUART4 Receive Line Idle input is selected"]
    pub const VAL70: Self = Self(0x46);
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    pub const VAL75: Self = Self(0x4b);
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    pub const VAL76: Self = Self(0x4c);
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    pub const VAL77: Self = Self(0x4d);
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    pub const VAL78: Self = Self(0x4e);
    #[doc = "CTimer3_MAT1 input is selected"]
    pub const VAL79: Self = Self(0x4f);
    #[doc = "CTimer3_MAT2 input is selected"]
    pub const VAL80: Self = Self(0x50);
    #[doc = "CTimer3_MAT3 input is selected"]
    pub const VAL81: Self = Self(0x51);
    #[doc = "CTimer4_MAT1 input is selected"]
    pub const VAL82: Self = Self(0x52);
    #[doc = "CTimer4_MAT2 input is selected"]
    pub const VAL83: Self = Self(0x53);
    #[doc = "CTimer4_MAT3 input is selected"]
    pub const VAL84: Self = Self(0x54);
    #[doc = "LPI2C2 Master End of Packet input is selected"]
    pub const VAL94: Self = Self(0x5e);
    #[doc = "LPI2C2 Slave End of Packet input is selected"]
    pub const VAL95: Self = Self(0x5f);
    #[doc = "LPI2C3 Master End of Packet input is selected"]
    pub const VAL96: Self = Self(0x60);
    #[doc = "LPI2C3 Slave End of Packet input is selected"]
    pub const VAL97: Self = Self(0x61);
    #[doc = "LPUART5 Received Data Word input is selected"]
    pub const VAL98: Self = Self(0x62);
    #[doc = "LPUART5 Transmitted Data Word input is selected"]
    pub const VAL99: Self = Self(0x63);
    #[doc = "LPUART5 Receive Line Idle input is selected"]
    pub const VAL100: Self = Self(0x64);
    #[doc = "TRIG_IN0 input is selected"]
    pub const VAL113: Self = Self(0x71);
    #[doc = "TRIG_IN1 input is selected"]
    pub const VAL114: Self = Self(0x72);
    #[doc = "TRIG_IN2 input is selected"]
    pub const VAL115: Self = Self(0x73);
    #[doc = "TRIG_IN3 input is selected"]
    pub const VAL116: Self = Self(0x74);
    #[doc = "TRIG_IN4 input is selected"]
    pub const VAL117: Self = Self(0x75);
    #[doc = "TRIG_IN5 input is selected"]
    pub const VAL118: Self = Self(0x76);
    #[doc = "TRIG_IN6 input is selected"]
    pub const VAL119: Self = Self(0x77);
    #[doc = "TRIG_IN7 input is selected"]
    pub const VAL120: Self = Self(0x78);
    #[doc = "TRIG_IN8 input is selected"]
    pub const VAL121: Self = Self(0x79);
    #[doc = "TRIG_IN9 input is selected"]
    pub const VAL122: Self = Self(0x7a);
    #[doc = "TRIG_IN10 input is selected"]
    pub const VAL123: Self = Self(0x7b);
    #[doc = "TRIG_IN11 input is selected"]
    pub const VAL124: Self = Self(0x7c);
    #[doc = "USB1 Start of Frame input is selected"]
    pub const VAL125: Self = Self(0x7d);
    #[doc = "LPSPI2 End of Frame input is selected"]
    pub const VAL126: Self = Self(0x7e);
    #[doc = "LPSPI2 Received Data Word input is selected"]
    pub const VAL127: Self = Self(0x7f);
    #[doc = "LPSPI3 End of Frame input is selected"]
    pub const VAL128: Self = Self(0x80);
    #[doc = "LPSPI3 Received Data Word input is selected"]
    pub const VAL129: Self = Self(0x81);
    #[doc = "LPSPI4 End of Frame input is selected"]
    pub const VAL130: Self = Self(0x82);
    #[doc = "LPSPI4 Received Data Word input is selected"]
    pub const VAL131: Self = Self(0x83);
    #[doc = "LPSPI5 End of Frame input is selected"]
    pub const VAL132: Self = Self(0x84);
    #[doc = "LPSPI5 Received Data Word input is selected"]
    pub const VAL133: Self = Self(0x85);
}
impl Ctimer0capInp {
    pub const fn from_bits(val: u8) -> Ctimer0capInp {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ctimer0capInp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("VAL1"),
            0x02 => f.write_str("VAL2"),
            0x03 => f.write_str("VAL3"),
            0x04 => f.write_str("VAL4"),
            0x05 => f.write_str("VAL5"),
            0x06 => f.write_str("VAL6"),
            0x07 => f.write_str("VAL7"),
            0x08 => f.write_str("VAL8"),
            0x09 => f.write_str("VAL9"),
            0x0a => f.write_str("VAL10"),
            0x0b => f.write_str("VAL11"),
            0x0c => f.write_str("VAL12"),
            0x0d => f.write_str("VAL13"),
            0x0e => f.write_str("VAL14"),
            0x0f => f.write_str("VAL15"),
            0x10 => f.write_str("VAL16"),
            0x11 => f.write_str("VAL17"),
            0x12 => f.write_str("VAL18"),
            0x13 => f.write_str("VAL19"),
            0x14 => f.write_str("VAL20"),
            0x16 => f.write_str("VAL22"),
            0x17 => f.write_str("VAL23"),
            0x18 => f.write_str("VAL24"),
            0x19 => f.write_str("VAL25"),
            0x1a => f.write_str("VAL26"),
            0x1b => f.write_str("VAL27"),
            0x1c => f.write_str("VAL28"),
            0x1d => f.write_str("VAL29"),
            0x1e => f.write_str("VAL30"),
            0x21 => f.write_str("VAL33"),
            0x22 => f.write_str("VAL34"),
            0x23 => f.write_str("VAL35"),
            0x24 => f.write_str("VAL36"),
            0x25 => f.write_str("VAL37"),
            0x26 => f.write_str("VAL38"),
            0x30 => f.write_str("VAL48"),
            0x31 => f.write_str("VAL49"),
            0x32 => f.write_str("VAL50"),
            0x33 => f.write_str("VAL51"),
            0x34 => f.write_str("VAL52"),
            0x35 => f.write_str("VAL53"),
            0x36 => f.write_str("VAL54"),
            0x37 => f.write_str("VAL55"),
            0x38 => f.write_str("VAL56"),
            0x39 => f.write_str("VAL57"),
            0x3a => f.write_str("VAL58"),
            0x3b => f.write_str("VAL59"),
            0x3c => f.write_str("VAL60"),
            0x3d => f.write_str("VAL61"),
            0x3e => f.write_str("VAL62"),
            0x3f => f.write_str("VAL63"),
            0x40 => f.write_str("VAL64"),
            0x41 => f.write_str("VAL65"),
            0x42 => f.write_str("VAL66"),
            0x43 => f.write_str("VAL67"),
            0x44 => f.write_str("VAL68"),
            0x45 => f.write_str("VAL69"),
            0x46 => f.write_str("VAL70"),
            0x4b => f.write_str("VAL75"),
            0x4c => f.write_str("VAL76"),
            0x4d => f.write_str("VAL77"),
            0x4e => f.write_str("VAL78"),
            0x4f => f.write_str("VAL79"),
            0x50 => f.write_str("VAL80"),
            0x51 => f.write_str("VAL81"),
            0x52 => f.write_str("VAL82"),
            0x53 => f.write_str("VAL83"),
            0x54 => f.write_str("VAL84"),
            0x5e => f.write_str("VAL94"),
            0x5f => f.write_str("VAL95"),
            0x60 => f.write_str("VAL96"),
            0x61 => f.write_str("VAL97"),
            0x62 => f.write_str("VAL98"),
            0x63 => f.write_str("VAL99"),
            0x64 => f.write_str("VAL100"),
            0x71 => f.write_str("VAL113"),
            0x72 => f.write_str("VAL114"),
            0x73 => f.write_str("VAL115"),
            0x74 => f.write_str("VAL116"),
            0x75 => f.write_str("VAL117"),
            0x76 => f.write_str("VAL118"),
            0x77 => f.write_str("VAL119"),
            0x78 => f.write_str("VAL120"),
            0x79 => f.write_str("VAL121"),
            0x7a => f.write_str("VAL122"),
            0x7b => f.write_str("VAL123"),
            0x7c => f.write_str("VAL124"),
            0x7d => f.write_str("VAL125"),
            0x7e => f.write_str("VAL126"),
            0x7f => f.write_str("VAL127"),
            0x80 => f.write_str("VAL128"),
            0x81 => f.write_str("VAL129"),
            0x82 => f.write_str("VAL130"),
            0x83 => f.write_str("VAL131"),
            0x84 => f.write_str("VAL132"),
            0x85 => f.write_str("VAL133"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer0capInp {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "VAL1"),
            0x02 => defmt::write!(f, "VAL2"),
            0x03 => defmt::write!(f, "VAL3"),
            0x04 => defmt::write!(f, "VAL4"),
            0x05 => defmt::write!(f, "VAL5"),
            0x06 => defmt::write!(f, "VAL6"),
            0x07 => defmt::write!(f, "VAL7"),
            0x08 => defmt::write!(f, "VAL8"),
            0x09 => defmt::write!(f, "VAL9"),
            0x0a => defmt::write!(f, "VAL10"),
            0x0b => defmt::write!(f, "VAL11"),
            0x0c => defmt::write!(f, "VAL12"),
            0x0d => defmt::write!(f, "VAL13"),
            0x0e => defmt::write!(f, "VAL14"),
            0x0f => defmt::write!(f, "VAL15"),
            0x10 => defmt::write!(f, "VAL16"),
            0x11 => defmt::write!(f, "VAL17"),
            0x12 => defmt::write!(f, "VAL18"),
            0x13 => defmt::write!(f, "VAL19"),
            0x14 => defmt::write!(f, "VAL20"),
            0x16 => defmt::write!(f, "VAL22"),
            0x17 => defmt::write!(f, "VAL23"),
            0x18 => defmt::write!(f, "VAL24"),
            0x19 => defmt::write!(f, "VAL25"),
            0x1a => defmt::write!(f, "VAL26"),
            0x1b => defmt::write!(f, "VAL27"),
            0x1c => defmt::write!(f, "VAL28"),
            0x1d => defmt::write!(f, "VAL29"),
            0x1e => defmt::write!(f, "VAL30"),
            0x21 => defmt::write!(f, "VAL33"),
            0x22 => defmt::write!(f, "VAL34"),
            0x23 => defmt::write!(f, "VAL35"),
            0x24 => defmt::write!(f, "VAL36"),
            0x25 => defmt::write!(f, "VAL37"),
            0x26 => defmt::write!(f, "VAL38"),
            0x30 => defmt::write!(f, "VAL48"),
            0x31 => defmt::write!(f, "VAL49"),
            0x32 => defmt::write!(f, "VAL50"),
            0x33 => defmt::write!(f, "VAL51"),
            0x34 => defmt::write!(f, "VAL52"),
            0x35 => defmt::write!(f, "VAL53"),
            0x36 => defmt::write!(f, "VAL54"),
            0x37 => defmt::write!(f, "VAL55"),
            0x38 => defmt::write!(f, "VAL56"),
            0x39 => defmt::write!(f, "VAL57"),
            0x3a => defmt::write!(f, "VAL58"),
            0x3b => defmt::write!(f, "VAL59"),
            0x3c => defmt::write!(f, "VAL60"),
            0x3d => defmt::write!(f, "VAL61"),
            0x3e => defmt::write!(f, "VAL62"),
            0x3f => defmt::write!(f, "VAL63"),
            0x40 => defmt::write!(f, "VAL64"),
            0x41 => defmt::write!(f, "VAL65"),
            0x42 => defmt::write!(f, "VAL66"),
            0x43 => defmt::write!(f, "VAL67"),
            0x44 => defmt::write!(f, "VAL68"),
            0x45 => defmt::write!(f, "VAL69"),
            0x46 => defmt::write!(f, "VAL70"),
            0x4b => defmt::write!(f, "VAL75"),
            0x4c => defmt::write!(f, "VAL76"),
            0x4d => defmt::write!(f, "VAL77"),
            0x4e => defmt::write!(f, "VAL78"),
            0x4f => defmt::write!(f, "VAL79"),
            0x50 => defmt::write!(f, "VAL80"),
            0x51 => defmt::write!(f, "VAL81"),
            0x52 => defmt::write!(f, "VAL82"),
            0x53 => defmt::write!(f, "VAL83"),
            0x54 => defmt::write!(f, "VAL84"),
            0x5e => defmt::write!(f, "VAL94"),
            0x5f => defmt::write!(f, "VAL95"),
            0x60 => defmt::write!(f, "VAL96"),
            0x61 => defmt::write!(f, "VAL97"),
            0x62 => defmt::write!(f, "VAL98"),
            0x63 => defmt::write!(f, "VAL99"),
            0x64 => defmt::write!(f, "VAL100"),
            0x71 => defmt::write!(f, "VAL113"),
            0x72 => defmt::write!(f, "VAL114"),
            0x73 => defmt::write!(f, "VAL115"),
            0x74 => defmt::write!(f, "VAL116"),
            0x75 => defmt::write!(f, "VAL117"),
            0x76 => defmt::write!(f, "VAL118"),
            0x77 => defmt::write!(f, "VAL119"),
            0x78 => defmt::write!(f, "VAL120"),
            0x79 => defmt::write!(f, "VAL121"),
            0x7a => defmt::write!(f, "VAL122"),
            0x7b => defmt::write!(f, "VAL123"),
            0x7c => defmt::write!(f, "VAL124"),
            0x7d => defmt::write!(f, "VAL125"),
            0x7e => defmt::write!(f, "VAL126"),
            0x7f => defmt::write!(f, "VAL127"),
            0x80 => defmt::write!(f, "VAL128"),
            0x81 => defmt::write!(f, "VAL129"),
            0x82 => defmt::write!(f, "VAL130"),
            0x83 => defmt::write!(f, "VAL131"),
            0x84 => defmt::write!(f, "VAL132"),
            0x85 => defmt::write!(f, "VAL133"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ctimer0capInp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer0capInp {
        Ctimer0capInp::from_bits(val)
    }
}
impl From<Ctimer0capInp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer0capInp) -> u8 {
        Ctimer0capInp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ctimer1capInp(u8);
impl Ctimer1capInp {
    #[doc = "CT_INP0 input is selected"]
    pub const VAL1: Self = Self(0x01);
    #[doc = "CT_INP1 input is selected"]
    pub const VAL2: Self = Self(0x02);
    #[doc = "CT_INP2 input is selected"]
    pub const VAL3: Self = Self(0x03);
    #[doc = "CT_INP3 input is selected"]
    pub const VAL4: Self = Self(0x04);
    #[doc = "CT_INP4 input is selected"]
    pub const VAL5: Self = Self(0x05);
    #[doc = "CT_INP5 input is selected"]
    pub const VAL6: Self = Self(0x06);
    #[doc = "CT_INP6 input is selected"]
    pub const VAL7: Self = Self(0x07);
    #[doc = "CT_INP7 input is selected"]
    pub const VAL8: Self = Self(0x08);
    #[doc = "CT_INP8 input is selected"]
    pub const VAL9: Self = Self(0x09);
    #[doc = "CT_INP9 input is selected"]
    pub const VAL10: Self = Self(0x0a);
    #[doc = "CT_INP10 input is selected"]
    pub const VAL11: Self = Self(0x0b);
    #[doc = "CT_INP11 input is selected"]
    pub const VAL12: Self = Self(0x0c);
    #[doc = "CT_INP12 input is selected"]
    pub const VAL13: Self = Self(0x0d);
    #[doc = "CT_INP13 input is selected"]
    pub const VAL14: Self = Self(0x0e);
    #[doc = "CT_INP14 input is selected"]
    pub const VAL15: Self = Self(0x0f);
    #[doc = "CT_INP15 input is selected"]
    pub const VAL16: Self = Self(0x10);
    #[doc = "CT_INP16 input is selected"]
    pub const VAL17: Self = Self(0x11);
    #[doc = "CT_INP17 input is selected"]
    pub const VAL18: Self = Self(0x12);
    #[doc = "CT_INP18 input is selected"]
    pub const VAL19: Self = Self(0x13);
    #[doc = "CT_INP19 input is selected"]
    pub const VAL20: Self = Self(0x14);
    #[doc = "AOI0_OUT0 input is selected"]
    pub const VAL22: Self = Self(0x16);
    #[doc = "AOI0_OUT1 input is selected"]
    pub const VAL23: Self = Self(0x17);
    #[doc = "AOI0_OUT2 input is selected"]
    pub const VAL24: Self = Self(0x18);
    #[doc = "AOI0_OUT3 input is selected"]
    pub const VAL25: Self = Self(0x19);
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    pub const VAL26: Self = Self(0x1a);
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    pub const VAL27: Self = Self(0x1b);
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    pub const VAL28: Self = Self(0x1c);
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    pub const VAL29: Self = Self(0x1d);
    #[doc = "CMP0_OUT input is selected"]
    pub const VAL30: Self = Self(0x1e);
    #[doc = "CTimer0_MAT1 input is selected"]
    pub const VAL33: Self = Self(0x21);
    #[doc = "CTimer0_MAT2 input is selected"]
    pub const VAL34: Self = Self(0x22);
    #[doc = "CTimer0_MAT3 input is selected"]
    pub const VAL35: Self = Self(0x23);
    #[doc = "CTimer2_MAT1 input is selected"]
    pub const VAL36: Self = Self(0x24);
    #[doc = "CTimer2_MAT2 input is selected"]
    pub const VAL37: Self = Self(0x25);
    #[doc = "CTimer2_MAT3 input is selected"]
    pub const VAL38: Self = Self(0x26);
    #[doc = "LPI2C0 Master End of Packet input is selected"]
    pub const VAL48: Self = Self(0x30);
    #[doc = "LPI2C0 Slave End of Packet input is selected"]
    pub const VAL49: Self = Self(0x31);
    #[doc = "LPI2C1 Master End of Packet input is selected"]
    pub const VAL50: Self = Self(0x32);
    #[doc = "LPI2C1 Slave End of Packet input is selected"]
    pub const VAL51: Self = Self(0x33);
    #[doc = "LPSPI0 End of Frame input is selected"]
    pub const VAL52: Self = Self(0x34);
    #[doc = "LPSPI0 Received Data Word input is selected"]
    pub const VAL53: Self = Self(0x35);
    #[doc = "LPSPI1 End of Frame input is selected"]
    pub const VAL54: Self = Self(0x36);
    #[doc = "LPSPI1 Received Data Word input is selected"]
    pub const VAL55: Self = Self(0x37);
    #[doc = "LPUART0 Received Data Word input is selected"]
    pub const VAL56: Self = Self(0x38);
    #[doc = "LPUART0 Transmitted Data Word input is selected"]
    pub const VAL57: Self = Self(0x39);
    #[doc = "LPUART0 Receive Line Idle input is selected"]
    pub const VAL58: Self = Self(0x3a);
    #[doc = "LPUART1 Received Data Word input is selected"]
    pub const VAL59: Self = Self(0x3b);
    #[doc = "LPUART1 Transmitted Data Word input is selected"]
    pub const VAL60: Self = Self(0x3c);
    #[doc = "LPUART1 Receive Line Idle input is selected"]
    pub const VAL61: Self = Self(0x3d);
    #[doc = "LPUART2 Received Data Word input is selected"]
    pub const VAL62: Self = Self(0x3e);
    #[doc = "LPUART2 Transmitted Data Word input is selected"]
    pub const VAL63: Self = Self(0x3f);
    #[doc = "LPUART2 Receive Line Idle input is selected"]
    pub const VAL64: Self = Self(0x40);
    #[doc = "LPUART3 Received Data Word input is selected"]
    pub const VAL65: Self = Self(0x41);
    #[doc = "LPUART3 Transmitted Data Word input is selected"]
    pub const VAL66: Self = Self(0x42);
    #[doc = "LPUART3 Receive Line Idle input is selected"]
    pub const VAL67: Self = Self(0x43);
    #[doc = "LPUART4 Received Data Word input is selected"]
    pub const VAL68: Self = Self(0x44);
    #[doc = "LPUART4 Transmitted Data Word input is selected"]
    pub const VAL69: Self = Self(0x45);
    #[doc = "LPUART4 Receive Line Idle input is selected"]
    pub const VAL70: Self = Self(0x46);
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    pub const VAL75: Self = Self(0x4b);
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    pub const VAL76: Self = Self(0x4c);
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    pub const VAL77: Self = Self(0x4d);
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    pub const VAL78: Self = Self(0x4e);
    #[doc = "CTimer3_MAT1 is selected"]
    pub const VAL79: Self = Self(0x4f);
    #[doc = "CTimer3_MAT2 input is selected"]
    pub const VAL80: Self = Self(0x50);
    #[doc = "CTimer3_MAT3 input is selected"]
    pub const VAL81: Self = Self(0x51);
    #[doc = "CTimer4_MAT1 input is selected"]
    pub const VAL82: Self = Self(0x52);
    #[doc = "CTimer4_MAT2 input is selected"]
    pub const VAL83: Self = Self(0x53);
    #[doc = "CTimer4_MAT3 input is selected"]
    pub const VAL84: Self = Self(0x54);
    #[doc = "LPI2C2 Master End of Packet input is selected"]
    pub const VAL94: Self = Self(0x5e);
    #[doc = "LPI2C2 Slave End of Packet input is selected"]
    pub const VAL95: Self = Self(0x5f);
    #[doc = "LPI2C3 Master End of Packet input is selected"]
    pub const VAL96: Self = Self(0x60);
    #[doc = "LPI2C3 Slave End of Packet input is selected"]
    pub const VAL97: Self = Self(0x61);
    #[doc = "LPUART5 Received Data Word input is selected"]
    pub const VAL98: Self = Self(0x62);
    #[doc = "LPUART5 Transmitted Data Word input is selected"]
    pub const VAL99: Self = Self(0x63);
    #[doc = "LPUART5 Receive Line Idle input is selected"]
    pub const VAL100: Self = Self(0x64);
    #[doc = "TRIG_IN0 input is selected"]
    pub const VAL113: Self = Self(0x71);
    #[doc = "TRIG_IN1 input is selected"]
    pub const VAL114: Self = Self(0x72);
    #[doc = "TRIG_IN2 input is selected"]
    pub const VAL115: Self = Self(0x73);
    #[doc = "TRIG_IN3 input is selected"]
    pub const VAL116: Self = Self(0x74);
    #[doc = "TRIG_IN4 input is selected"]
    pub const VAL117: Self = Self(0x75);
    #[doc = "TRIG_IN5 input is selected"]
    pub const VAL118: Self = Self(0x76);
    #[doc = "TRIG_IN6 input is selected"]
    pub const VAL119: Self = Self(0x77);
    #[doc = "TRIG_IN7 input is selected"]
    pub const VAL120: Self = Self(0x78);
    #[doc = "TRIG_IN8 input is selected"]
    pub const VAL121: Self = Self(0x79);
    #[doc = "TRIG_IN9 input is selected"]
    pub const VAL122: Self = Self(0x7a);
    #[doc = "TRIG_IN10 input is selected"]
    pub const VAL123: Self = Self(0x7b);
    #[doc = "TRIG_IN11 input is selected"]
    pub const VAL124: Self = Self(0x7c);
    #[doc = "USB1 Start of Frame input is selected"]
    pub const VAL125: Self = Self(0x7d);
    #[doc = "LPSPI2 End of Frame input is selected"]
    pub const VAL126: Self = Self(0x7e);
    #[doc = "LPSPI2 Received Data Word input is selected"]
    pub const VAL127: Self = Self(0x7f);
    #[doc = "LPSPI3 End of Frame is selected"]
    pub const VAL128: Self = Self(0x80);
    #[doc = "LPSPI3 Received Data Word input is selected"]
    pub const VAL129: Self = Self(0x81);
    #[doc = "LPSPI4 End of Frame input is selected"]
    pub const VAL130: Self = Self(0x82);
    #[doc = "LPSPI4 Received Data Word input is selected"]
    pub const VAL131: Self = Self(0x83);
    #[doc = "LPSPI5 End of Frame input is selected"]
    pub const VAL132: Self = Self(0x84);
    #[doc = "LPSPI5 Received Data Word input is selected"]
    pub const VAL133: Self = Self(0x85);
}
impl Ctimer1capInp {
    pub const fn from_bits(val: u8) -> Ctimer1capInp {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ctimer1capInp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("VAL1"),
            0x02 => f.write_str("VAL2"),
            0x03 => f.write_str("VAL3"),
            0x04 => f.write_str("VAL4"),
            0x05 => f.write_str("VAL5"),
            0x06 => f.write_str("VAL6"),
            0x07 => f.write_str("VAL7"),
            0x08 => f.write_str("VAL8"),
            0x09 => f.write_str("VAL9"),
            0x0a => f.write_str("VAL10"),
            0x0b => f.write_str("VAL11"),
            0x0c => f.write_str("VAL12"),
            0x0d => f.write_str("VAL13"),
            0x0e => f.write_str("VAL14"),
            0x0f => f.write_str("VAL15"),
            0x10 => f.write_str("VAL16"),
            0x11 => f.write_str("VAL17"),
            0x12 => f.write_str("VAL18"),
            0x13 => f.write_str("VAL19"),
            0x14 => f.write_str("VAL20"),
            0x16 => f.write_str("VAL22"),
            0x17 => f.write_str("VAL23"),
            0x18 => f.write_str("VAL24"),
            0x19 => f.write_str("VAL25"),
            0x1a => f.write_str("VAL26"),
            0x1b => f.write_str("VAL27"),
            0x1c => f.write_str("VAL28"),
            0x1d => f.write_str("VAL29"),
            0x1e => f.write_str("VAL30"),
            0x21 => f.write_str("VAL33"),
            0x22 => f.write_str("VAL34"),
            0x23 => f.write_str("VAL35"),
            0x24 => f.write_str("VAL36"),
            0x25 => f.write_str("VAL37"),
            0x26 => f.write_str("VAL38"),
            0x30 => f.write_str("VAL48"),
            0x31 => f.write_str("VAL49"),
            0x32 => f.write_str("VAL50"),
            0x33 => f.write_str("VAL51"),
            0x34 => f.write_str("VAL52"),
            0x35 => f.write_str("VAL53"),
            0x36 => f.write_str("VAL54"),
            0x37 => f.write_str("VAL55"),
            0x38 => f.write_str("VAL56"),
            0x39 => f.write_str("VAL57"),
            0x3a => f.write_str("VAL58"),
            0x3b => f.write_str("VAL59"),
            0x3c => f.write_str("VAL60"),
            0x3d => f.write_str("VAL61"),
            0x3e => f.write_str("VAL62"),
            0x3f => f.write_str("VAL63"),
            0x40 => f.write_str("VAL64"),
            0x41 => f.write_str("VAL65"),
            0x42 => f.write_str("VAL66"),
            0x43 => f.write_str("VAL67"),
            0x44 => f.write_str("VAL68"),
            0x45 => f.write_str("VAL69"),
            0x46 => f.write_str("VAL70"),
            0x4b => f.write_str("VAL75"),
            0x4c => f.write_str("VAL76"),
            0x4d => f.write_str("VAL77"),
            0x4e => f.write_str("VAL78"),
            0x4f => f.write_str("VAL79"),
            0x50 => f.write_str("VAL80"),
            0x51 => f.write_str("VAL81"),
            0x52 => f.write_str("VAL82"),
            0x53 => f.write_str("VAL83"),
            0x54 => f.write_str("VAL84"),
            0x5e => f.write_str("VAL94"),
            0x5f => f.write_str("VAL95"),
            0x60 => f.write_str("VAL96"),
            0x61 => f.write_str("VAL97"),
            0x62 => f.write_str("VAL98"),
            0x63 => f.write_str("VAL99"),
            0x64 => f.write_str("VAL100"),
            0x71 => f.write_str("VAL113"),
            0x72 => f.write_str("VAL114"),
            0x73 => f.write_str("VAL115"),
            0x74 => f.write_str("VAL116"),
            0x75 => f.write_str("VAL117"),
            0x76 => f.write_str("VAL118"),
            0x77 => f.write_str("VAL119"),
            0x78 => f.write_str("VAL120"),
            0x79 => f.write_str("VAL121"),
            0x7a => f.write_str("VAL122"),
            0x7b => f.write_str("VAL123"),
            0x7c => f.write_str("VAL124"),
            0x7d => f.write_str("VAL125"),
            0x7e => f.write_str("VAL126"),
            0x7f => f.write_str("VAL127"),
            0x80 => f.write_str("VAL128"),
            0x81 => f.write_str("VAL129"),
            0x82 => f.write_str("VAL130"),
            0x83 => f.write_str("VAL131"),
            0x84 => f.write_str("VAL132"),
            0x85 => f.write_str("VAL133"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer1capInp {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "VAL1"),
            0x02 => defmt::write!(f, "VAL2"),
            0x03 => defmt::write!(f, "VAL3"),
            0x04 => defmt::write!(f, "VAL4"),
            0x05 => defmt::write!(f, "VAL5"),
            0x06 => defmt::write!(f, "VAL6"),
            0x07 => defmt::write!(f, "VAL7"),
            0x08 => defmt::write!(f, "VAL8"),
            0x09 => defmt::write!(f, "VAL9"),
            0x0a => defmt::write!(f, "VAL10"),
            0x0b => defmt::write!(f, "VAL11"),
            0x0c => defmt::write!(f, "VAL12"),
            0x0d => defmt::write!(f, "VAL13"),
            0x0e => defmt::write!(f, "VAL14"),
            0x0f => defmt::write!(f, "VAL15"),
            0x10 => defmt::write!(f, "VAL16"),
            0x11 => defmt::write!(f, "VAL17"),
            0x12 => defmt::write!(f, "VAL18"),
            0x13 => defmt::write!(f, "VAL19"),
            0x14 => defmt::write!(f, "VAL20"),
            0x16 => defmt::write!(f, "VAL22"),
            0x17 => defmt::write!(f, "VAL23"),
            0x18 => defmt::write!(f, "VAL24"),
            0x19 => defmt::write!(f, "VAL25"),
            0x1a => defmt::write!(f, "VAL26"),
            0x1b => defmt::write!(f, "VAL27"),
            0x1c => defmt::write!(f, "VAL28"),
            0x1d => defmt::write!(f, "VAL29"),
            0x1e => defmt::write!(f, "VAL30"),
            0x21 => defmt::write!(f, "VAL33"),
            0x22 => defmt::write!(f, "VAL34"),
            0x23 => defmt::write!(f, "VAL35"),
            0x24 => defmt::write!(f, "VAL36"),
            0x25 => defmt::write!(f, "VAL37"),
            0x26 => defmt::write!(f, "VAL38"),
            0x30 => defmt::write!(f, "VAL48"),
            0x31 => defmt::write!(f, "VAL49"),
            0x32 => defmt::write!(f, "VAL50"),
            0x33 => defmt::write!(f, "VAL51"),
            0x34 => defmt::write!(f, "VAL52"),
            0x35 => defmt::write!(f, "VAL53"),
            0x36 => defmt::write!(f, "VAL54"),
            0x37 => defmt::write!(f, "VAL55"),
            0x38 => defmt::write!(f, "VAL56"),
            0x39 => defmt::write!(f, "VAL57"),
            0x3a => defmt::write!(f, "VAL58"),
            0x3b => defmt::write!(f, "VAL59"),
            0x3c => defmt::write!(f, "VAL60"),
            0x3d => defmt::write!(f, "VAL61"),
            0x3e => defmt::write!(f, "VAL62"),
            0x3f => defmt::write!(f, "VAL63"),
            0x40 => defmt::write!(f, "VAL64"),
            0x41 => defmt::write!(f, "VAL65"),
            0x42 => defmt::write!(f, "VAL66"),
            0x43 => defmt::write!(f, "VAL67"),
            0x44 => defmt::write!(f, "VAL68"),
            0x45 => defmt::write!(f, "VAL69"),
            0x46 => defmt::write!(f, "VAL70"),
            0x4b => defmt::write!(f, "VAL75"),
            0x4c => defmt::write!(f, "VAL76"),
            0x4d => defmt::write!(f, "VAL77"),
            0x4e => defmt::write!(f, "VAL78"),
            0x4f => defmt::write!(f, "VAL79"),
            0x50 => defmt::write!(f, "VAL80"),
            0x51 => defmt::write!(f, "VAL81"),
            0x52 => defmt::write!(f, "VAL82"),
            0x53 => defmt::write!(f, "VAL83"),
            0x54 => defmt::write!(f, "VAL84"),
            0x5e => defmt::write!(f, "VAL94"),
            0x5f => defmt::write!(f, "VAL95"),
            0x60 => defmt::write!(f, "VAL96"),
            0x61 => defmt::write!(f, "VAL97"),
            0x62 => defmt::write!(f, "VAL98"),
            0x63 => defmt::write!(f, "VAL99"),
            0x64 => defmt::write!(f, "VAL100"),
            0x71 => defmt::write!(f, "VAL113"),
            0x72 => defmt::write!(f, "VAL114"),
            0x73 => defmt::write!(f, "VAL115"),
            0x74 => defmt::write!(f, "VAL116"),
            0x75 => defmt::write!(f, "VAL117"),
            0x76 => defmt::write!(f, "VAL118"),
            0x77 => defmt::write!(f, "VAL119"),
            0x78 => defmt::write!(f, "VAL120"),
            0x79 => defmt::write!(f, "VAL121"),
            0x7a => defmt::write!(f, "VAL122"),
            0x7b => defmt::write!(f, "VAL123"),
            0x7c => defmt::write!(f, "VAL124"),
            0x7d => defmt::write!(f, "VAL125"),
            0x7e => defmt::write!(f, "VAL126"),
            0x7f => defmt::write!(f, "VAL127"),
            0x80 => defmt::write!(f, "VAL128"),
            0x81 => defmt::write!(f, "VAL129"),
            0x82 => defmt::write!(f, "VAL130"),
            0x83 => defmt::write!(f, "VAL131"),
            0x84 => defmt::write!(f, "VAL132"),
            0x85 => defmt::write!(f, "VAL133"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ctimer1capInp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer1capInp {
        Ctimer1capInp::from_bits(val)
    }
}
impl From<Ctimer1capInp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer1capInp) -> u8 {
        Ctimer1capInp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ctimer2capInp(u8);
impl Ctimer2capInp {
    #[doc = "CT_INP0 input is selected"]
    pub const VAL1: Self = Self(0x01);
    #[doc = "CT_INP1 input is selected"]
    pub const VAL2: Self = Self(0x02);
    #[doc = "CT_INP2 input is selected"]
    pub const VAL3: Self = Self(0x03);
    #[doc = "CT_INP3 input is selected"]
    pub const VAL4: Self = Self(0x04);
    #[doc = "CT_INP4 input is selected"]
    pub const VAL5: Self = Self(0x05);
    #[doc = "CT_INP5 input is selected"]
    pub const VAL6: Self = Self(0x06);
    #[doc = "CT_INP6 input is selected"]
    pub const VAL7: Self = Self(0x07);
    #[doc = "CT_INP7 input is selected"]
    pub const VAL8: Self = Self(0x08);
    #[doc = "CT_INP8 input is selected"]
    pub const VAL9: Self = Self(0x09);
    #[doc = "CT_INP9 input is selected"]
    pub const VAL10: Self = Self(0x0a);
    #[doc = "CT_INP10 input is selected"]
    pub const VAL11: Self = Self(0x0b);
    #[doc = "CT_INP11 input is selected"]
    pub const VAL12: Self = Self(0x0c);
    #[doc = "CT_INP12 input is selected"]
    pub const VAL13: Self = Self(0x0d);
    #[doc = "CT_INP13 input is selected"]
    pub const VAL14: Self = Self(0x0e);
    #[doc = "CT_INP14 input is selected"]
    pub const VAL15: Self = Self(0x0f);
    #[doc = "CT_INP15 input is selected"]
    pub const VAL16: Self = Self(0x10);
    #[doc = "CT_INP16 input is selected"]
    pub const VAL17: Self = Self(0x11);
    #[doc = "CT_INP17 input is selected"]
    pub const VAL18: Self = Self(0x12);
    #[doc = "CT_INP18 input is selected"]
    pub const VAL19: Self = Self(0x13);
    #[doc = "CT_INP19 input is selected"]
    pub const VAL20: Self = Self(0x14);
    #[doc = "AOI0_OUT0 input is selected"]
    pub const VAL22: Self = Self(0x16);
    #[doc = "AOI0_OUT1 input is selected"]
    pub const VAL23: Self = Self(0x17);
    #[doc = "AOI0_OUT2 input is selected"]
    pub const VAL24: Self = Self(0x18);
    #[doc = "AOI0_OUT3 input is selected"]
    pub const VAL25: Self = Self(0x19);
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    pub const VAL26: Self = Self(0x1a);
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    pub const VAL27: Self = Self(0x1b);
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    pub const VAL28: Self = Self(0x1c);
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    pub const VAL29: Self = Self(0x1d);
    #[doc = "CMP0_OUT is selected"]
    pub const VAL30: Self = Self(0x1e);
    #[doc = "CTimer0_MAT1 input is selected"]
    pub const VAL33: Self = Self(0x21);
    #[doc = "CTimer0_MAT2 input is selected"]
    pub const VAL34: Self = Self(0x22);
    #[doc = "CTimer0_MAT3 input is selected"]
    pub const VAL35: Self = Self(0x23);
    #[doc = "CTimer1_MAT1 input is selected"]
    pub const VAL36: Self = Self(0x24);
    #[doc = "CTimer1_MAT2 input is selected"]
    pub const VAL37: Self = Self(0x25);
    #[doc = "CTimer1_MAT3 input is selected"]
    pub const VAL38: Self = Self(0x26);
    #[doc = "LPI2C0 Master End of Packet input is selected"]
    pub const VAL48: Self = Self(0x30);
    #[doc = "LPI2C0 Slave End of Packet input is selected"]
    pub const VAL49: Self = Self(0x31);
    #[doc = "LPI2C1 Master End of Packet input is selected"]
    pub const VAL50: Self = Self(0x32);
    #[doc = "LPI2C1 Slave End of Packet input is selected"]
    pub const VAL51: Self = Self(0x33);
    #[doc = "LPSPI0 End of Frame input is selected"]
    pub const VAL52: Self = Self(0x34);
    #[doc = "LPSPI0 Received Data Word input is selected"]
    pub const VAL53: Self = Self(0x35);
    #[doc = "LPSPI1 End of Frame input is selected"]
    pub const VAL54: Self = Self(0x36);
    #[doc = "LPSPI1 Received Data Word input is selected"]
    pub const VAL55: Self = Self(0x37);
    #[doc = "LPUART0 Received Data Word input is selected"]
    pub const VAL56: Self = Self(0x38);
    #[doc = "LPUART0 Transmitted Data Word input is selected"]
    pub const VAL57: Self = Self(0x39);
    #[doc = "LPUART0 Receive Line Idle input is selected"]
    pub const VAL58: Self = Self(0x3a);
    #[doc = "LPUART1 Received Data Word input is selected"]
    pub const VAL59: Self = Self(0x3b);
    #[doc = "LPUART1 Transmitted Data Word input is selected"]
    pub const VAL60: Self = Self(0x3c);
    #[doc = "LPUART1 Receive Line Idle input is selected"]
    pub const VAL61: Self = Self(0x3d);
    #[doc = "LPUART2 Received Data Word input is selected"]
    pub const VAL62: Self = Self(0x3e);
    #[doc = "LPUART2 Transmitted Data Word input is selected"]
    pub const VAL63: Self = Self(0x3f);
    #[doc = "LPUART2 Receive Line Idle input is selected"]
    pub const VAL64: Self = Self(0x40);
    #[doc = "LPUART3 Received Data Word input is selected"]
    pub const VAL65: Self = Self(0x41);
    #[doc = "LPUART3 Transmitted Data Word input is selected"]
    pub const VAL66: Self = Self(0x42);
    #[doc = "LPUART3 Receive Line Idle input is selected"]
    pub const VAL67: Self = Self(0x43);
    #[doc = "LPUART4 Received Data Word input is selected"]
    pub const VAL68: Self = Self(0x44);
    #[doc = "LPUART4 Transmitted Data Word input is selected"]
    pub const VAL69: Self = Self(0x45);
    #[doc = "LPUART4 Receive Line Idle input is selected"]
    pub const VAL70: Self = Self(0x46);
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    pub const VAL75: Self = Self(0x4b);
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    pub const VAL76: Self = Self(0x4c);
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    pub const VAL77: Self = Self(0x4d);
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    pub const VAL78: Self = Self(0x4e);
    #[doc = "CTimer3_MAT1 input is selected"]
    pub const VAL79: Self = Self(0x4f);
    #[doc = "CTimer3_MAT2 input is selected"]
    pub const VAL80: Self = Self(0x50);
    #[doc = "CTimer3_MAT3 input is selected"]
    pub const VAL81: Self = Self(0x51);
    #[doc = "CTimer4_MAT1 input is selected"]
    pub const VAL82: Self = Self(0x52);
    #[doc = "CTimer4_MAT2 input is selected"]
    pub const VAL83: Self = Self(0x53);
    #[doc = "CTimer4_MAT3 input is selected"]
    pub const VAL84: Self = Self(0x54);
    #[doc = "LPI2C2 Master End of Packet input is selected"]
    pub const VAL94: Self = Self(0x5e);
    #[doc = "LPI2C2 Slave End of Packet input is selected"]
    pub const VAL95: Self = Self(0x5f);
    #[doc = "LPI2C3 Master End of Packet input is selected"]
    pub const VAL96: Self = Self(0x60);
    #[doc = "LPI2C3 Slave End of Packet input is selected"]
    pub const VAL97: Self = Self(0x61);
    #[doc = "LPUART5 Received Data Word input is selected"]
    pub const VAL98: Self = Self(0x62);
    #[doc = "LPUART5 Transmitted Data Word input is selected"]
    pub const VAL99: Self = Self(0x63);
    #[doc = "LPUART5 Receive Line Idle input is selected"]
    pub const VAL100: Self = Self(0x64);
    #[doc = "TRIG_IN0 input is selected"]
    pub const VAL113: Self = Self(0x71);
    #[doc = "TRIG_IN1 input is selected"]
    pub const VAL114: Self = Self(0x72);
    #[doc = "TRIG_IN2 input is selected"]
    pub const VAL115: Self = Self(0x73);
    #[doc = "TRIG_IN3 input is selected"]
    pub const VAL116: Self = Self(0x74);
    #[doc = "TRIG_IN4 input is selected"]
    pub const VAL117: Self = Self(0x75);
    #[doc = "TRIG_IN5 input is selected"]
    pub const VAL118: Self = Self(0x76);
    #[doc = "TRIG_IN6 input is selected"]
    pub const VAL119: Self = Self(0x77);
    #[doc = "TRIG_IN7 input is selected"]
    pub const VAL120: Self = Self(0x78);
    #[doc = "TRIG_IN8 input is selected"]
    pub const VAL121: Self = Self(0x79);
    #[doc = "TRIG_IN9 input is selected"]
    pub const VAL122: Self = Self(0x7a);
    #[doc = "TRIG_IN10 input is selected"]
    pub const VAL123: Self = Self(0x7b);
    #[doc = "TRIG_IN11 input is selected"]
    pub const VAL124: Self = Self(0x7c);
    #[doc = "USB1 Start of Frame input is selected"]
    pub const VAL125: Self = Self(0x7d);
    #[doc = "LPSPI2 End of Frame input is selected"]
    pub const VAL126: Self = Self(0x7e);
    #[doc = "LPSPI2 Received Data Word input is selected"]
    pub const VAL127: Self = Self(0x7f);
    #[doc = "LPSPI3 End of Frame input is selected"]
    pub const VAL128: Self = Self(0x80);
    #[doc = "LPSPI3 Received Data Word input is selected"]
    pub const VAL129: Self = Self(0x81);
    #[doc = "LPSPI4 End of Frame input is selected"]
    pub const VAL130: Self = Self(0x82);
    #[doc = "LPSPI4 Received Data Word input is selected"]
    pub const VAL131: Self = Self(0x83);
    #[doc = "LPSPI5 End of Frame input is selected"]
    pub const VAL132: Self = Self(0x84);
    #[doc = "LPSPI5 Received Data Word input is selected"]
    pub const VAL133: Self = Self(0x85);
}
impl Ctimer2capInp {
    pub const fn from_bits(val: u8) -> Ctimer2capInp {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ctimer2capInp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("VAL1"),
            0x02 => f.write_str("VAL2"),
            0x03 => f.write_str("VAL3"),
            0x04 => f.write_str("VAL4"),
            0x05 => f.write_str("VAL5"),
            0x06 => f.write_str("VAL6"),
            0x07 => f.write_str("VAL7"),
            0x08 => f.write_str("VAL8"),
            0x09 => f.write_str("VAL9"),
            0x0a => f.write_str("VAL10"),
            0x0b => f.write_str("VAL11"),
            0x0c => f.write_str("VAL12"),
            0x0d => f.write_str("VAL13"),
            0x0e => f.write_str("VAL14"),
            0x0f => f.write_str("VAL15"),
            0x10 => f.write_str("VAL16"),
            0x11 => f.write_str("VAL17"),
            0x12 => f.write_str("VAL18"),
            0x13 => f.write_str("VAL19"),
            0x14 => f.write_str("VAL20"),
            0x16 => f.write_str("VAL22"),
            0x17 => f.write_str("VAL23"),
            0x18 => f.write_str("VAL24"),
            0x19 => f.write_str("VAL25"),
            0x1a => f.write_str("VAL26"),
            0x1b => f.write_str("VAL27"),
            0x1c => f.write_str("VAL28"),
            0x1d => f.write_str("VAL29"),
            0x1e => f.write_str("VAL30"),
            0x21 => f.write_str("VAL33"),
            0x22 => f.write_str("VAL34"),
            0x23 => f.write_str("VAL35"),
            0x24 => f.write_str("VAL36"),
            0x25 => f.write_str("VAL37"),
            0x26 => f.write_str("VAL38"),
            0x30 => f.write_str("VAL48"),
            0x31 => f.write_str("VAL49"),
            0x32 => f.write_str("VAL50"),
            0x33 => f.write_str("VAL51"),
            0x34 => f.write_str("VAL52"),
            0x35 => f.write_str("VAL53"),
            0x36 => f.write_str("VAL54"),
            0x37 => f.write_str("VAL55"),
            0x38 => f.write_str("VAL56"),
            0x39 => f.write_str("VAL57"),
            0x3a => f.write_str("VAL58"),
            0x3b => f.write_str("VAL59"),
            0x3c => f.write_str("VAL60"),
            0x3d => f.write_str("VAL61"),
            0x3e => f.write_str("VAL62"),
            0x3f => f.write_str("VAL63"),
            0x40 => f.write_str("VAL64"),
            0x41 => f.write_str("VAL65"),
            0x42 => f.write_str("VAL66"),
            0x43 => f.write_str("VAL67"),
            0x44 => f.write_str("VAL68"),
            0x45 => f.write_str("VAL69"),
            0x46 => f.write_str("VAL70"),
            0x4b => f.write_str("VAL75"),
            0x4c => f.write_str("VAL76"),
            0x4d => f.write_str("VAL77"),
            0x4e => f.write_str("VAL78"),
            0x4f => f.write_str("VAL79"),
            0x50 => f.write_str("VAL80"),
            0x51 => f.write_str("VAL81"),
            0x52 => f.write_str("VAL82"),
            0x53 => f.write_str("VAL83"),
            0x54 => f.write_str("VAL84"),
            0x5e => f.write_str("VAL94"),
            0x5f => f.write_str("VAL95"),
            0x60 => f.write_str("VAL96"),
            0x61 => f.write_str("VAL97"),
            0x62 => f.write_str("VAL98"),
            0x63 => f.write_str("VAL99"),
            0x64 => f.write_str("VAL100"),
            0x71 => f.write_str("VAL113"),
            0x72 => f.write_str("VAL114"),
            0x73 => f.write_str("VAL115"),
            0x74 => f.write_str("VAL116"),
            0x75 => f.write_str("VAL117"),
            0x76 => f.write_str("VAL118"),
            0x77 => f.write_str("VAL119"),
            0x78 => f.write_str("VAL120"),
            0x79 => f.write_str("VAL121"),
            0x7a => f.write_str("VAL122"),
            0x7b => f.write_str("VAL123"),
            0x7c => f.write_str("VAL124"),
            0x7d => f.write_str("VAL125"),
            0x7e => f.write_str("VAL126"),
            0x7f => f.write_str("VAL127"),
            0x80 => f.write_str("VAL128"),
            0x81 => f.write_str("VAL129"),
            0x82 => f.write_str("VAL130"),
            0x83 => f.write_str("VAL131"),
            0x84 => f.write_str("VAL132"),
            0x85 => f.write_str("VAL133"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer2capInp {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "VAL1"),
            0x02 => defmt::write!(f, "VAL2"),
            0x03 => defmt::write!(f, "VAL3"),
            0x04 => defmt::write!(f, "VAL4"),
            0x05 => defmt::write!(f, "VAL5"),
            0x06 => defmt::write!(f, "VAL6"),
            0x07 => defmt::write!(f, "VAL7"),
            0x08 => defmt::write!(f, "VAL8"),
            0x09 => defmt::write!(f, "VAL9"),
            0x0a => defmt::write!(f, "VAL10"),
            0x0b => defmt::write!(f, "VAL11"),
            0x0c => defmt::write!(f, "VAL12"),
            0x0d => defmt::write!(f, "VAL13"),
            0x0e => defmt::write!(f, "VAL14"),
            0x0f => defmt::write!(f, "VAL15"),
            0x10 => defmt::write!(f, "VAL16"),
            0x11 => defmt::write!(f, "VAL17"),
            0x12 => defmt::write!(f, "VAL18"),
            0x13 => defmt::write!(f, "VAL19"),
            0x14 => defmt::write!(f, "VAL20"),
            0x16 => defmt::write!(f, "VAL22"),
            0x17 => defmt::write!(f, "VAL23"),
            0x18 => defmt::write!(f, "VAL24"),
            0x19 => defmt::write!(f, "VAL25"),
            0x1a => defmt::write!(f, "VAL26"),
            0x1b => defmt::write!(f, "VAL27"),
            0x1c => defmt::write!(f, "VAL28"),
            0x1d => defmt::write!(f, "VAL29"),
            0x1e => defmt::write!(f, "VAL30"),
            0x21 => defmt::write!(f, "VAL33"),
            0x22 => defmt::write!(f, "VAL34"),
            0x23 => defmt::write!(f, "VAL35"),
            0x24 => defmt::write!(f, "VAL36"),
            0x25 => defmt::write!(f, "VAL37"),
            0x26 => defmt::write!(f, "VAL38"),
            0x30 => defmt::write!(f, "VAL48"),
            0x31 => defmt::write!(f, "VAL49"),
            0x32 => defmt::write!(f, "VAL50"),
            0x33 => defmt::write!(f, "VAL51"),
            0x34 => defmt::write!(f, "VAL52"),
            0x35 => defmt::write!(f, "VAL53"),
            0x36 => defmt::write!(f, "VAL54"),
            0x37 => defmt::write!(f, "VAL55"),
            0x38 => defmt::write!(f, "VAL56"),
            0x39 => defmt::write!(f, "VAL57"),
            0x3a => defmt::write!(f, "VAL58"),
            0x3b => defmt::write!(f, "VAL59"),
            0x3c => defmt::write!(f, "VAL60"),
            0x3d => defmt::write!(f, "VAL61"),
            0x3e => defmt::write!(f, "VAL62"),
            0x3f => defmt::write!(f, "VAL63"),
            0x40 => defmt::write!(f, "VAL64"),
            0x41 => defmt::write!(f, "VAL65"),
            0x42 => defmt::write!(f, "VAL66"),
            0x43 => defmt::write!(f, "VAL67"),
            0x44 => defmt::write!(f, "VAL68"),
            0x45 => defmt::write!(f, "VAL69"),
            0x46 => defmt::write!(f, "VAL70"),
            0x4b => defmt::write!(f, "VAL75"),
            0x4c => defmt::write!(f, "VAL76"),
            0x4d => defmt::write!(f, "VAL77"),
            0x4e => defmt::write!(f, "VAL78"),
            0x4f => defmt::write!(f, "VAL79"),
            0x50 => defmt::write!(f, "VAL80"),
            0x51 => defmt::write!(f, "VAL81"),
            0x52 => defmt::write!(f, "VAL82"),
            0x53 => defmt::write!(f, "VAL83"),
            0x54 => defmt::write!(f, "VAL84"),
            0x5e => defmt::write!(f, "VAL94"),
            0x5f => defmt::write!(f, "VAL95"),
            0x60 => defmt::write!(f, "VAL96"),
            0x61 => defmt::write!(f, "VAL97"),
            0x62 => defmt::write!(f, "VAL98"),
            0x63 => defmt::write!(f, "VAL99"),
            0x64 => defmt::write!(f, "VAL100"),
            0x71 => defmt::write!(f, "VAL113"),
            0x72 => defmt::write!(f, "VAL114"),
            0x73 => defmt::write!(f, "VAL115"),
            0x74 => defmt::write!(f, "VAL116"),
            0x75 => defmt::write!(f, "VAL117"),
            0x76 => defmt::write!(f, "VAL118"),
            0x77 => defmt::write!(f, "VAL119"),
            0x78 => defmt::write!(f, "VAL120"),
            0x79 => defmt::write!(f, "VAL121"),
            0x7a => defmt::write!(f, "VAL122"),
            0x7b => defmt::write!(f, "VAL123"),
            0x7c => defmt::write!(f, "VAL124"),
            0x7d => defmt::write!(f, "VAL125"),
            0x7e => defmt::write!(f, "VAL126"),
            0x7f => defmt::write!(f, "VAL127"),
            0x80 => defmt::write!(f, "VAL128"),
            0x81 => defmt::write!(f, "VAL129"),
            0x82 => defmt::write!(f, "VAL130"),
            0x83 => defmt::write!(f, "VAL131"),
            0x84 => defmt::write!(f, "VAL132"),
            0x85 => defmt::write!(f, "VAL133"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ctimer2capInp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer2capInp {
        Ctimer2capInp::from_bits(val)
    }
}
impl From<Ctimer2capInp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer2capInp) -> u8 {
        Ctimer2capInp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ctimer3capInp(u8);
impl Ctimer3capInp {
    #[doc = "CT_INP0 input is selected"]
    pub const VAL1: Self = Self(0x01);
    #[doc = "CT_INP1 input is selected"]
    pub const VAL2: Self = Self(0x02);
    #[doc = "CT_INP2 input is selected"]
    pub const VAL3: Self = Self(0x03);
    #[doc = "CT_INP3 input is selected"]
    pub const VAL4: Self = Self(0x04);
    #[doc = "CT_INP4 input is selected"]
    pub const VAL5: Self = Self(0x05);
    #[doc = "CT_INP5 input is selected"]
    pub const VAL6: Self = Self(0x06);
    #[doc = "CT_INP6 input is selected"]
    pub const VAL7: Self = Self(0x07);
    #[doc = "CT_INP7 input is selected"]
    pub const VAL8: Self = Self(0x08);
    #[doc = "CT_INP8 input is selected"]
    pub const VAL9: Self = Self(0x09);
    #[doc = "CT_INP9 input is selected"]
    pub const VAL10: Self = Self(0x0a);
    #[doc = "CT_INP10 input is selected"]
    pub const VAL11: Self = Self(0x0b);
    #[doc = "CT_INP11 input is selected"]
    pub const VAL12: Self = Self(0x0c);
    #[doc = "CT_INP12 input is selected"]
    pub const VAL13: Self = Self(0x0d);
    #[doc = "CT_INP13 input is selected"]
    pub const VAL14: Self = Self(0x0e);
    #[doc = "CT_INP14 input is selected"]
    pub const VAL15: Self = Self(0x0f);
    #[doc = "CT_INP15 input is selected"]
    pub const VAL16: Self = Self(0x10);
    #[doc = "CT_INP16 input is selected"]
    pub const VAL17: Self = Self(0x11);
    #[doc = "CT_INP17 input is selected"]
    pub const VAL18: Self = Self(0x12);
    #[doc = "CT_INP18 input is selected"]
    pub const VAL19: Self = Self(0x13);
    #[doc = "CT_INP19 input is selected"]
    pub const VAL20: Self = Self(0x14);
    #[doc = "AOI0_OUT0 input is selected"]
    pub const VAL22: Self = Self(0x16);
    #[doc = "AOI0_OUT1 input is selected"]
    pub const VAL23: Self = Self(0x17);
    #[doc = "AOI0_OUT2 input is selected"]
    pub const VAL24: Self = Self(0x18);
    #[doc = "AOI0_OUT3 input is selected"]
    pub const VAL25: Self = Self(0x19);
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    pub const VAL26: Self = Self(0x1a);
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    pub const VAL27: Self = Self(0x1b);
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    pub const VAL28: Self = Self(0x1c);
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    pub const VAL29: Self = Self(0x1d);
    #[doc = "CMP0_OUT input is selected"]
    pub const VAL30: Self = Self(0x1e);
    #[doc = "CTimer0_MAT1 input is selected"]
    pub const VAL33: Self = Self(0x21);
    #[doc = "CTimer0_MAT2 input is selected"]
    pub const VAL34: Self = Self(0x22);
    #[doc = "CTimer0_MAT3 input is selected"]
    pub const VAL35: Self = Self(0x23);
    #[doc = "CTimer1_MAT1 input is selected"]
    pub const VAL36: Self = Self(0x24);
    #[doc = "CTimer1_MAT2 input is selected"]
    pub const VAL37: Self = Self(0x25);
    #[doc = "CTimer1_MAT3 input is selected"]
    pub const VAL38: Self = Self(0x26);
    #[doc = "LPI2C0 Master End of Packet input is selected"]
    pub const VAL48: Self = Self(0x30);
    #[doc = "LPI2C0 Slave End of Packet input is selected"]
    pub const VAL49: Self = Self(0x31);
    #[doc = "LPI2C1 Master End of Packet input is selected"]
    pub const VAL50: Self = Self(0x32);
    #[doc = "LPI2C1 Slave End of Packet input is selected"]
    pub const VAL51: Self = Self(0x33);
    #[doc = "LPSPI0 End of Frame input is selected"]
    pub const VAL52: Self = Self(0x34);
    #[doc = "LPSPI0 Received Data Word input is selected"]
    pub const VAL53: Self = Self(0x35);
    #[doc = "LPSPI1 End of Frame input is selected"]
    pub const VAL54: Self = Self(0x36);
    #[doc = "LPSPI1 Received Data Word input is selected"]
    pub const VAL55: Self = Self(0x37);
    #[doc = "LPUART0 Received Data Word input is selected"]
    pub const VAL56: Self = Self(0x38);
    #[doc = "LPUART0 Transmitted Data Word input is selected"]
    pub const VAL57: Self = Self(0x39);
    #[doc = "LPUART0 Receive Line Idle input is selected"]
    pub const VAL58: Self = Self(0x3a);
    #[doc = "LPUART1 Received Data Word input is selected"]
    pub const VAL59: Self = Self(0x3b);
    #[doc = "LPUART1 Transmitted Data Word input is selected"]
    pub const VAL60: Self = Self(0x3c);
    #[doc = "LPUART1 Receive Line Idle input is selected"]
    pub const VAL61: Self = Self(0x3d);
    #[doc = "LPUART2 Received Data Word input is selected"]
    pub const VAL62: Self = Self(0x3e);
    #[doc = "LPUART2 Transmitted Data Word input is selected"]
    pub const VAL63: Self = Self(0x3f);
    #[doc = "LPUART2 Receive Line Idle input is selected"]
    pub const VAL64: Self = Self(0x40);
    #[doc = "LPUART3 Received Data Word input is selected"]
    pub const VAL65: Self = Self(0x41);
    #[doc = "LPUART3 Transmitted Data Word input is selected"]
    pub const VAL66: Self = Self(0x42);
    #[doc = "LPUART3 Receive Line Idle input is selected"]
    pub const VAL67: Self = Self(0x43);
    #[doc = "LPUART4 Received Data Word input is selected"]
    pub const VAL68: Self = Self(0x44);
    #[doc = "LPUART4 Transmitted Data Word input is selected"]
    pub const VAL69: Self = Self(0x45);
    #[doc = "LPUART4 Receive Line Idle input is selected"]
    pub const VAL70: Self = Self(0x46);
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    pub const VAL75: Self = Self(0x4b);
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    pub const VAL76: Self = Self(0x4c);
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    pub const VAL77: Self = Self(0x4d);
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    pub const VAL78: Self = Self(0x4e);
    #[doc = "CTimer2_MAT1 input is selected"]
    pub const VAL79: Self = Self(0x4f);
    #[doc = "CTimer2_MAT2 input is selected"]
    pub const VAL80: Self = Self(0x50);
    #[doc = "CTimer2_MAT3 input is selected"]
    pub const VAL81: Self = Self(0x51);
    #[doc = "CTimer4_MAT1 input is selected"]
    pub const VAL82: Self = Self(0x52);
    #[doc = "CTimer4_MAT2 input is selected"]
    pub const VAL83: Self = Self(0x53);
    #[doc = "CTimer4_MAT3 input is selected"]
    pub const VAL84: Self = Self(0x54);
    #[doc = "LPI2C2 Master End of Packet input is selected"]
    pub const VAL94: Self = Self(0x5e);
    #[doc = "LPI2C2 Slave End of Packet input is selected"]
    pub const VAL95: Self = Self(0x5f);
    #[doc = "LPI2C3 Master End of Packet input is selected"]
    pub const VAL96: Self = Self(0x60);
    #[doc = "LPI2C3 Slave End of Packet input is selected"]
    pub const VAL97: Self = Self(0x61);
    #[doc = "LPUART5 Received Data Word input is selected"]
    pub const VAL98: Self = Self(0x62);
    #[doc = "LPUART5 Transmitted Data Word input is selected"]
    pub const VAL99: Self = Self(0x63);
    #[doc = "LPUART5 Receive Line Idle input is selected"]
    pub const VAL100: Self = Self(0x64);
    #[doc = "TMPR_OUT0 input is selected"]
    pub const VAL102: Self = Self(0x66);
    #[doc = "TMPR_OUT1 input is selected"]
    pub const VAL103: Self = Self(0x67);
    #[doc = "TRIG_IN0 input is selected"]
    pub const VAL113: Self = Self(0x71);
    #[doc = "TRIG_IN1 input is selected"]
    pub const VAL114: Self = Self(0x72);
    #[doc = "TRIG_IN2 input is selected"]
    pub const VAL115: Self = Self(0x73);
    #[doc = "TRIG_IN3 input is selected"]
    pub const VAL116: Self = Self(0x74);
    #[doc = "TRIG_IN4 input is selected"]
    pub const VAL117: Self = Self(0x75);
    #[doc = "TRIG_IN5 input is selected"]
    pub const VAL118: Self = Self(0x76);
    #[doc = "TRIG_IN6 input is selected"]
    pub const VAL119: Self = Self(0x77);
    #[doc = "TRIG_IN7 input is selected"]
    pub const VAL120: Self = Self(0x78);
    #[doc = "TRIG_IN8 input is selected"]
    pub const VAL121: Self = Self(0x79);
    #[doc = "TRIG_IN9 input is selected"]
    pub const VAL122: Self = Self(0x7a);
    #[doc = "TRIG_IN10 input is selected"]
    pub const VAL123: Self = Self(0x7b);
    #[doc = "TRIG_IN11 input is selected"]
    pub const VAL124: Self = Self(0x7c);
    #[doc = "USB1 Start of Frame input is selected"]
    pub const VAL125: Self = Self(0x7d);
    #[doc = "LPSPI2 End of Frame input is selected"]
    pub const VAL126: Self = Self(0x7e);
    #[doc = "LPSPI2 Received Data Word input is selected"]
    pub const VAL127: Self = Self(0x7f);
    #[doc = "LPSPI3 End of Frame input is selected"]
    pub const VAL128: Self = Self(0x80);
    #[doc = "LPSPI3 Received Data Word input is selected"]
    pub const VAL129: Self = Self(0x81);
    #[doc = "LPSPI4 End of Frame input is selected"]
    pub const VAL130: Self = Self(0x82);
    #[doc = "LPSPI4 Received Data Word input is selected"]
    pub const VAL131: Self = Self(0x83);
    #[doc = "LPSPI5 End of Frame input is selected"]
    pub const VAL132: Self = Self(0x84);
    #[doc = "LPSPI5 Received Data Word input is selected"]
    pub const VAL133: Self = Self(0x85);
}
impl Ctimer3capInp {
    pub const fn from_bits(val: u8) -> Ctimer3capInp {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ctimer3capInp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("VAL1"),
            0x02 => f.write_str("VAL2"),
            0x03 => f.write_str("VAL3"),
            0x04 => f.write_str("VAL4"),
            0x05 => f.write_str("VAL5"),
            0x06 => f.write_str("VAL6"),
            0x07 => f.write_str("VAL7"),
            0x08 => f.write_str("VAL8"),
            0x09 => f.write_str("VAL9"),
            0x0a => f.write_str("VAL10"),
            0x0b => f.write_str("VAL11"),
            0x0c => f.write_str("VAL12"),
            0x0d => f.write_str("VAL13"),
            0x0e => f.write_str("VAL14"),
            0x0f => f.write_str("VAL15"),
            0x10 => f.write_str("VAL16"),
            0x11 => f.write_str("VAL17"),
            0x12 => f.write_str("VAL18"),
            0x13 => f.write_str("VAL19"),
            0x14 => f.write_str("VAL20"),
            0x16 => f.write_str("VAL22"),
            0x17 => f.write_str("VAL23"),
            0x18 => f.write_str("VAL24"),
            0x19 => f.write_str("VAL25"),
            0x1a => f.write_str("VAL26"),
            0x1b => f.write_str("VAL27"),
            0x1c => f.write_str("VAL28"),
            0x1d => f.write_str("VAL29"),
            0x1e => f.write_str("VAL30"),
            0x21 => f.write_str("VAL33"),
            0x22 => f.write_str("VAL34"),
            0x23 => f.write_str("VAL35"),
            0x24 => f.write_str("VAL36"),
            0x25 => f.write_str("VAL37"),
            0x26 => f.write_str("VAL38"),
            0x30 => f.write_str("VAL48"),
            0x31 => f.write_str("VAL49"),
            0x32 => f.write_str("VAL50"),
            0x33 => f.write_str("VAL51"),
            0x34 => f.write_str("VAL52"),
            0x35 => f.write_str("VAL53"),
            0x36 => f.write_str("VAL54"),
            0x37 => f.write_str("VAL55"),
            0x38 => f.write_str("VAL56"),
            0x39 => f.write_str("VAL57"),
            0x3a => f.write_str("VAL58"),
            0x3b => f.write_str("VAL59"),
            0x3c => f.write_str("VAL60"),
            0x3d => f.write_str("VAL61"),
            0x3e => f.write_str("VAL62"),
            0x3f => f.write_str("VAL63"),
            0x40 => f.write_str("VAL64"),
            0x41 => f.write_str("VAL65"),
            0x42 => f.write_str("VAL66"),
            0x43 => f.write_str("VAL67"),
            0x44 => f.write_str("VAL68"),
            0x45 => f.write_str("VAL69"),
            0x46 => f.write_str("VAL70"),
            0x4b => f.write_str("VAL75"),
            0x4c => f.write_str("VAL76"),
            0x4d => f.write_str("VAL77"),
            0x4e => f.write_str("VAL78"),
            0x4f => f.write_str("VAL79"),
            0x50 => f.write_str("VAL80"),
            0x51 => f.write_str("VAL81"),
            0x52 => f.write_str("VAL82"),
            0x53 => f.write_str("VAL83"),
            0x54 => f.write_str("VAL84"),
            0x5e => f.write_str("VAL94"),
            0x5f => f.write_str("VAL95"),
            0x60 => f.write_str("VAL96"),
            0x61 => f.write_str("VAL97"),
            0x62 => f.write_str("VAL98"),
            0x63 => f.write_str("VAL99"),
            0x64 => f.write_str("VAL100"),
            0x66 => f.write_str("VAL102"),
            0x67 => f.write_str("VAL103"),
            0x71 => f.write_str("VAL113"),
            0x72 => f.write_str("VAL114"),
            0x73 => f.write_str("VAL115"),
            0x74 => f.write_str("VAL116"),
            0x75 => f.write_str("VAL117"),
            0x76 => f.write_str("VAL118"),
            0x77 => f.write_str("VAL119"),
            0x78 => f.write_str("VAL120"),
            0x79 => f.write_str("VAL121"),
            0x7a => f.write_str("VAL122"),
            0x7b => f.write_str("VAL123"),
            0x7c => f.write_str("VAL124"),
            0x7d => f.write_str("VAL125"),
            0x7e => f.write_str("VAL126"),
            0x7f => f.write_str("VAL127"),
            0x80 => f.write_str("VAL128"),
            0x81 => f.write_str("VAL129"),
            0x82 => f.write_str("VAL130"),
            0x83 => f.write_str("VAL131"),
            0x84 => f.write_str("VAL132"),
            0x85 => f.write_str("VAL133"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer3capInp {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "VAL1"),
            0x02 => defmt::write!(f, "VAL2"),
            0x03 => defmt::write!(f, "VAL3"),
            0x04 => defmt::write!(f, "VAL4"),
            0x05 => defmt::write!(f, "VAL5"),
            0x06 => defmt::write!(f, "VAL6"),
            0x07 => defmt::write!(f, "VAL7"),
            0x08 => defmt::write!(f, "VAL8"),
            0x09 => defmt::write!(f, "VAL9"),
            0x0a => defmt::write!(f, "VAL10"),
            0x0b => defmt::write!(f, "VAL11"),
            0x0c => defmt::write!(f, "VAL12"),
            0x0d => defmt::write!(f, "VAL13"),
            0x0e => defmt::write!(f, "VAL14"),
            0x0f => defmt::write!(f, "VAL15"),
            0x10 => defmt::write!(f, "VAL16"),
            0x11 => defmt::write!(f, "VAL17"),
            0x12 => defmt::write!(f, "VAL18"),
            0x13 => defmt::write!(f, "VAL19"),
            0x14 => defmt::write!(f, "VAL20"),
            0x16 => defmt::write!(f, "VAL22"),
            0x17 => defmt::write!(f, "VAL23"),
            0x18 => defmt::write!(f, "VAL24"),
            0x19 => defmt::write!(f, "VAL25"),
            0x1a => defmt::write!(f, "VAL26"),
            0x1b => defmt::write!(f, "VAL27"),
            0x1c => defmt::write!(f, "VAL28"),
            0x1d => defmt::write!(f, "VAL29"),
            0x1e => defmt::write!(f, "VAL30"),
            0x21 => defmt::write!(f, "VAL33"),
            0x22 => defmt::write!(f, "VAL34"),
            0x23 => defmt::write!(f, "VAL35"),
            0x24 => defmt::write!(f, "VAL36"),
            0x25 => defmt::write!(f, "VAL37"),
            0x26 => defmt::write!(f, "VAL38"),
            0x30 => defmt::write!(f, "VAL48"),
            0x31 => defmt::write!(f, "VAL49"),
            0x32 => defmt::write!(f, "VAL50"),
            0x33 => defmt::write!(f, "VAL51"),
            0x34 => defmt::write!(f, "VAL52"),
            0x35 => defmt::write!(f, "VAL53"),
            0x36 => defmt::write!(f, "VAL54"),
            0x37 => defmt::write!(f, "VAL55"),
            0x38 => defmt::write!(f, "VAL56"),
            0x39 => defmt::write!(f, "VAL57"),
            0x3a => defmt::write!(f, "VAL58"),
            0x3b => defmt::write!(f, "VAL59"),
            0x3c => defmt::write!(f, "VAL60"),
            0x3d => defmt::write!(f, "VAL61"),
            0x3e => defmt::write!(f, "VAL62"),
            0x3f => defmt::write!(f, "VAL63"),
            0x40 => defmt::write!(f, "VAL64"),
            0x41 => defmt::write!(f, "VAL65"),
            0x42 => defmt::write!(f, "VAL66"),
            0x43 => defmt::write!(f, "VAL67"),
            0x44 => defmt::write!(f, "VAL68"),
            0x45 => defmt::write!(f, "VAL69"),
            0x46 => defmt::write!(f, "VAL70"),
            0x4b => defmt::write!(f, "VAL75"),
            0x4c => defmt::write!(f, "VAL76"),
            0x4d => defmt::write!(f, "VAL77"),
            0x4e => defmt::write!(f, "VAL78"),
            0x4f => defmt::write!(f, "VAL79"),
            0x50 => defmt::write!(f, "VAL80"),
            0x51 => defmt::write!(f, "VAL81"),
            0x52 => defmt::write!(f, "VAL82"),
            0x53 => defmt::write!(f, "VAL83"),
            0x54 => defmt::write!(f, "VAL84"),
            0x5e => defmt::write!(f, "VAL94"),
            0x5f => defmt::write!(f, "VAL95"),
            0x60 => defmt::write!(f, "VAL96"),
            0x61 => defmt::write!(f, "VAL97"),
            0x62 => defmt::write!(f, "VAL98"),
            0x63 => defmt::write!(f, "VAL99"),
            0x64 => defmt::write!(f, "VAL100"),
            0x66 => defmt::write!(f, "VAL102"),
            0x67 => defmt::write!(f, "VAL103"),
            0x71 => defmt::write!(f, "VAL113"),
            0x72 => defmt::write!(f, "VAL114"),
            0x73 => defmt::write!(f, "VAL115"),
            0x74 => defmt::write!(f, "VAL116"),
            0x75 => defmt::write!(f, "VAL117"),
            0x76 => defmt::write!(f, "VAL118"),
            0x77 => defmt::write!(f, "VAL119"),
            0x78 => defmt::write!(f, "VAL120"),
            0x79 => defmt::write!(f, "VAL121"),
            0x7a => defmt::write!(f, "VAL122"),
            0x7b => defmt::write!(f, "VAL123"),
            0x7c => defmt::write!(f, "VAL124"),
            0x7d => defmt::write!(f, "VAL125"),
            0x7e => defmt::write!(f, "VAL126"),
            0x7f => defmt::write!(f, "VAL127"),
            0x80 => defmt::write!(f, "VAL128"),
            0x81 => defmt::write!(f, "VAL129"),
            0x82 => defmt::write!(f, "VAL130"),
            0x83 => defmt::write!(f, "VAL131"),
            0x84 => defmt::write!(f, "VAL132"),
            0x85 => defmt::write!(f, "VAL133"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ctimer3capInp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer3capInp {
        Ctimer3capInp::from_bits(val)
    }
}
impl From<Ctimer3capInp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer3capInp) -> u8 {
        Ctimer3capInp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ctimer4capInp(u8);
impl Ctimer4capInp {
    #[doc = "CT_INP0 input is selected"]
    pub const VAL1: Self = Self(0x01);
    #[doc = "CT_INP1 input is selected"]
    pub const VAL2: Self = Self(0x02);
    #[doc = "CT_INP2 input is selected"]
    pub const VAL3: Self = Self(0x03);
    #[doc = "CT_INP3 input is selected"]
    pub const VAL4: Self = Self(0x04);
    #[doc = "CT_INP4 input is selected"]
    pub const VAL5: Self = Self(0x05);
    #[doc = "CT_INP5 input is selected"]
    pub const VAL6: Self = Self(0x06);
    #[doc = "CT_INP6 input is selected"]
    pub const VAL7: Self = Self(0x07);
    #[doc = "CT_INP7 input is selected"]
    pub const VAL8: Self = Self(0x08);
    #[doc = "CT_INP8 input is selected"]
    pub const VAL9: Self = Self(0x09);
    #[doc = "CT_INP9 input is selected"]
    pub const VAL10: Self = Self(0x0a);
    #[doc = "CT_INP10 input is selected"]
    pub const VAL11: Self = Self(0x0b);
    #[doc = "CT_INP11 input is selected"]
    pub const VAL12: Self = Self(0x0c);
    #[doc = "CT_INP12 input is selected"]
    pub const VAL13: Self = Self(0x0d);
    #[doc = "CT_INP13 input is selected"]
    pub const VAL14: Self = Self(0x0e);
    #[doc = "CT_INP14 input is selected"]
    pub const VAL15: Self = Self(0x0f);
    #[doc = "CT_INP15 input is selected"]
    pub const VAL16: Self = Self(0x10);
    #[doc = "CT_INP16 input is selected"]
    pub const VAL17: Self = Self(0x11);
    #[doc = "CT_INP17 input is selected"]
    pub const VAL18: Self = Self(0x12);
    #[doc = "CT_INP18 input is selected"]
    pub const VAL19: Self = Self(0x13);
    #[doc = "CT_INP19 input is selected"]
    pub const VAL20: Self = Self(0x14);
    #[doc = "AOI0_OUT0 input is selected"]
    pub const VAL22: Self = Self(0x16);
    #[doc = "AOI0_OUT1 input is selected"]
    pub const VAL23: Self = Self(0x17);
    #[doc = "AOI0_OUT2 input is selected"]
    pub const VAL24: Self = Self(0x18);
    #[doc = "AOI0_OUT3 input is selected"]
    pub const VAL25: Self = Self(0x19);
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    pub const VAL26: Self = Self(0x1a);
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    pub const VAL27: Self = Self(0x1b);
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    pub const VAL28: Self = Self(0x1c);
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    pub const VAL29: Self = Self(0x1d);
    #[doc = "CMP0_OUT is selected"]
    pub const VAL30: Self = Self(0x1e);
    #[doc = "CTimer0_MAT1 input is selected"]
    pub const VAL33: Self = Self(0x21);
    #[doc = "CTimer0_MAT2 input is selected"]
    pub const VAL34: Self = Self(0x22);
    #[doc = "CTimer0_MAT3 input is selected"]
    pub const VAL35: Self = Self(0x23);
    #[doc = "CTimer1_MAT1 input is selected"]
    pub const VAL36: Self = Self(0x24);
    #[doc = "CTimer1_MAT2 input is selected"]
    pub const VAL37: Self = Self(0x25);
    #[doc = "CTimer1_MAT3 input is selected"]
    pub const VAL38: Self = Self(0x26);
    #[doc = "LPI2C0 Master End of Packet input is selected"]
    pub const VAL48: Self = Self(0x30);
    #[doc = "LPI2C0 Slave End of Packet input is selected"]
    pub const VAL49: Self = Self(0x31);
    #[doc = "LPI2C1 Master End of Packet input is selected"]
    pub const VAL50: Self = Self(0x32);
    #[doc = "LPI2C1 Slave End of Packet input is selected"]
    pub const VAL51: Self = Self(0x33);
    #[doc = "LPSPI0 End of Frame input is selected"]
    pub const VAL52: Self = Self(0x34);
    #[doc = "LPSPI0 Received Data Word input is selected"]
    pub const VAL53: Self = Self(0x35);
    #[doc = "LPSPI1 End of Frame input is selected"]
    pub const VAL54: Self = Self(0x36);
    #[doc = "LPSPI1 Received Data Word input is selected"]
    pub const VAL55: Self = Self(0x37);
    #[doc = "LPUART0 Received Data Word input is selected"]
    pub const VAL56: Self = Self(0x38);
    #[doc = "LPUART0 Transmitted Data Word input is selected"]
    pub const VAL57: Self = Self(0x39);
    #[doc = "LPUART0 Receive Line Idle input is selected"]
    pub const VAL58: Self = Self(0x3a);
    #[doc = "LPUART1 Received Data Word input is selected"]
    pub const VAL59: Self = Self(0x3b);
    #[doc = "LPUART1 Transmitted Data Word input is selected"]
    pub const VAL60: Self = Self(0x3c);
    #[doc = "LPUART1 Receive Line Idle input is selected"]
    pub const VAL61: Self = Self(0x3d);
    #[doc = "LPUART2 Received Data Word input is selected"]
    pub const VAL62: Self = Self(0x3e);
    #[doc = "LPUART2 Transmitted Data Word input is selected"]
    pub const VAL63: Self = Self(0x3f);
    #[doc = "LPUART2 Receive Line Idle input is selected"]
    pub const VAL64: Self = Self(0x40);
    #[doc = "LPUART3 Received Data Word input is selected"]
    pub const VAL65: Self = Self(0x41);
    #[doc = "LPUART3 Transmitted Data Word input is selected"]
    pub const VAL66: Self = Self(0x42);
    #[doc = "LPUART3 Receive Line Idle input is selected"]
    pub const VAL67: Self = Self(0x43);
    #[doc = "LPUART4 Received Data Word input is selected"]
    pub const VAL68: Self = Self(0x44);
    #[doc = "LPUART4 Transmitted Data Word input is selected"]
    pub const VAL69: Self = Self(0x45);
    #[doc = "LPUART4 Receive Line Idle input is selected"]
    pub const VAL70: Self = Self(0x46);
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    pub const VAL75: Self = Self(0x4b);
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    pub const VAL76: Self = Self(0x4c);
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    pub const VAL77: Self = Self(0x4d);
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    pub const VAL78: Self = Self(0x4e);
    #[doc = "CTimer2_MAT1 input is selected"]
    pub const VAL79: Self = Self(0x4f);
    #[doc = "CTimer2_MAT2 input is selected"]
    pub const VAL80: Self = Self(0x50);
    #[doc = "CTimer2_MAT3 input is selected"]
    pub const VAL81: Self = Self(0x51);
    #[doc = "CTimer3_MAT1 input is selected"]
    pub const VAL82: Self = Self(0x52);
    #[doc = "CTimer3_MAT2 input is selected"]
    pub const VAL83: Self = Self(0x53);
    #[doc = "CTimer3_MAT3 input is selected"]
    pub const VAL84: Self = Self(0x54);
    #[doc = "LPI2C2 Master End of Packet input is selected"]
    pub const VAL94: Self = Self(0x5e);
    #[doc = "LPI2C2 Slave End of Packet input is selected"]
    pub const VAL95: Self = Self(0x5f);
    #[doc = "LPI2C3 Master End of Packet input is selected"]
    pub const VAL96: Self = Self(0x60);
    #[doc = "LPI2C3 Slave End of Packet input is selected"]
    pub const VAL97: Self = Self(0x61);
    #[doc = "LPUART5 Received Data Word input is selected"]
    pub const VAL98: Self = Self(0x62);
    #[doc = "LPUART5 Transmitted Data Word input is selected"]
    pub const VAL99: Self = Self(0x63);
    #[doc = "LPUART5 Receive Line Idle input is selected"]
    pub const VAL100: Self = Self(0x64);
    #[doc = "TMPR_OUT0 input is selected"]
    pub const VAL102: Self = Self(0x66);
    #[doc = "TMPR_OUT1 input is selected"]
    pub const VAL103: Self = Self(0x67);
    #[doc = "TRIG_IN0 input is selected"]
    pub const VAL113: Self = Self(0x71);
    #[doc = "TRIG_IN1 input is selected"]
    pub const VAL114: Self = Self(0x72);
    #[doc = "TRIG_IN2 input is selected"]
    pub const VAL115: Self = Self(0x73);
    #[doc = "TRIG_IN3 input is selected"]
    pub const VAL116: Self = Self(0x74);
    #[doc = "TRIG_IN4 input is selected"]
    pub const VAL117: Self = Self(0x75);
    #[doc = "TRIG_IN5 input is selected"]
    pub const VAL118: Self = Self(0x76);
    #[doc = "TRIG_IN6 input is selected"]
    pub const VAL119: Self = Self(0x77);
    #[doc = "TRIG_IN7 input is selected"]
    pub const VAL120: Self = Self(0x78);
    #[doc = "TRIG_IN8 input is selected"]
    pub const VAL121: Self = Self(0x79);
    #[doc = "TRIG_IN9 input is selected"]
    pub const VAL122: Self = Self(0x7a);
    #[doc = "TRIG_IN10 input is selected"]
    pub const VAL123: Self = Self(0x7b);
    #[doc = "TRIG_IN11 input is selected"]
    pub const VAL124: Self = Self(0x7c);
    #[doc = "USB1 Start of Frame input is selected"]
    pub const VAL125: Self = Self(0x7d);
    #[doc = "LPSPI2 End of Frame input is selected"]
    pub const VAL126: Self = Self(0x7e);
    #[doc = "LPSPI2 Received Data Word input is selected"]
    pub const VAL127: Self = Self(0x7f);
    #[doc = "LPSPI3 End of Frame input is selected"]
    pub const VAL128: Self = Self(0x80);
    #[doc = "LPSPI3 Received Data Word input is selected"]
    pub const VAL129: Self = Self(0x81);
    #[doc = "LPSPI4 End of Frame input is selected"]
    pub const VAL130: Self = Self(0x82);
    #[doc = "LPSPI4 Received Data Word input is selected"]
    pub const VAL131: Self = Self(0x83);
    #[doc = "LPSPI5 End of Frame input is selected"]
    pub const VAL132: Self = Self(0x84);
    #[doc = "LPSPI5 Received Data Word input is selected"]
    pub const VAL133: Self = Self(0x85);
}
impl Ctimer4capInp {
    pub const fn from_bits(val: u8) -> Ctimer4capInp {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ctimer4capInp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("VAL1"),
            0x02 => f.write_str("VAL2"),
            0x03 => f.write_str("VAL3"),
            0x04 => f.write_str("VAL4"),
            0x05 => f.write_str("VAL5"),
            0x06 => f.write_str("VAL6"),
            0x07 => f.write_str("VAL7"),
            0x08 => f.write_str("VAL8"),
            0x09 => f.write_str("VAL9"),
            0x0a => f.write_str("VAL10"),
            0x0b => f.write_str("VAL11"),
            0x0c => f.write_str("VAL12"),
            0x0d => f.write_str("VAL13"),
            0x0e => f.write_str("VAL14"),
            0x0f => f.write_str("VAL15"),
            0x10 => f.write_str("VAL16"),
            0x11 => f.write_str("VAL17"),
            0x12 => f.write_str("VAL18"),
            0x13 => f.write_str("VAL19"),
            0x14 => f.write_str("VAL20"),
            0x16 => f.write_str("VAL22"),
            0x17 => f.write_str("VAL23"),
            0x18 => f.write_str("VAL24"),
            0x19 => f.write_str("VAL25"),
            0x1a => f.write_str("VAL26"),
            0x1b => f.write_str("VAL27"),
            0x1c => f.write_str("VAL28"),
            0x1d => f.write_str("VAL29"),
            0x1e => f.write_str("VAL30"),
            0x21 => f.write_str("VAL33"),
            0x22 => f.write_str("VAL34"),
            0x23 => f.write_str("VAL35"),
            0x24 => f.write_str("VAL36"),
            0x25 => f.write_str("VAL37"),
            0x26 => f.write_str("VAL38"),
            0x30 => f.write_str("VAL48"),
            0x31 => f.write_str("VAL49"),
            0x32 => f.write_str("VAL50"),
            0x33 => f.write_str("VAL51"),
            0x34 => f.write_str("VAL52"),
            0x35 => f.write_str("VAL53"),
            0x36 => f.write_str("VAL54"),
            0x37 => f.write_str("VAL55"),
            0x38 => f.write_str("VAL56"),
            0x39 => f.write_str("VAL57"),
            0x3a => f.write_str("VAL58"),
            0x3b => f.write_str("VAL59"),
            0x3c => f.write_str("VAL60"),
            0x3d => f.write_str("VAL61"),
            0x3e => f.write_str("VAL62"),
            0x3f => f.write_str("VAL63"),
            0x40 => f.write_str("VAL64"),
            0x41 => f.write_str("VAL65"),
            0x42 => f.write_str("VAL66"),
            0x43 => f.write_str("VAL67"),
            0x44 => f.write_str("VAL68"),
            0x45 => f.write_str("VAL69"),
            0x46 => f.write_str("VAL70"),
            0x4b => f.write_str("VAL75"),
            0x4c => f.write_str("VAL76"),
            0x4d => f.write_str("VAL77"),
            0x4e => f.write_str("VAL78"),
            0x4f => f.write_str("VAL79"),
            0x50 => f.write_str("VAL80"),
            0x51 => f.write_str("VAL81"),
            0x52 => f.write_str("VAL82"),
            0x53 => f.write_str("VAL83"),
            0x54 => f.write_str("VAL84"),
            0x5e => f.write_str("VAL94"),
            0x5f => f.write_str("VAL95"),
            0x60 => f.write_str("VAL96"),
            0x61 => f.write_str("VAL97"),
            0x62 => f.write_str("VAL98"),
            0x63 => f.write_str("VAL99"),
            0x64 => f.write_str("VAL100"),
            0x66 => f.write_str("VAL102"),
            0x67 => f.write_str("VAL103"),
            0x71 => f.write_str("VAL113"),
            0x72 => f.write_str("VAL114"),
            0x73 => f.write_str("VAL115"),
            0x74 => f.write_str("VAL116"),
            0x75 => f.write_str("VAL117"),
            0x76 => f.write_str("VAL118"),
            0x77 => f.write_str("VAL119"),
            0x78 => f.write_str("VAL120"),
            0x79 => f.write_str("VAL121"),
            0x7a => f.write_str("VAL122"),
            0x7b => f.write_str("VAL123"),
            0x7c => f.write_str("VAL124"),
            0x7d => f.write_str("VAL125"),
            0x7e => f.write_str("VAL126"),
            0x7f => f.write_str("VAL127"),
            0x80 => f.write_str("VAL128"),
            0x81 => f.write_str("VAL129"),
            0x82 => f.write_str("VAL130"),
            0x83 => f.write_str("VAL131"),
            0x84 => f.write_str("VAL132"),
            0x85 => f.write_str("VAL133"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer4capInp {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "VAL1"),
            0x02 => defmt::write!(f, "VAL2"),
            0x03 => defmt::write!(f, "VAL3"),
            0x04 => defmt::write!(f, "VAL4"),
            0x05 => defmt::write!(f, "VAL5"),
            0x06 => defmt::write!(f, "VAL6"),
            0x07 => defmt::write!(f, "VAL7"),
            0x08 => defmt::write!(f, "VAL8"),
            0x09 => defmt::write!(f, "VAL9"),
            0x0a => defmt::write!(f, "VAL10"),
            0x0b => defmt::write!(f, "VAL11"),
            0x0c => defmt::write!(f, "VAL12"),
            0x0d => defmt::write!(f, "VAL13"),
            0x0e => defmt::write!(f, "VAL14"),
            0x0f => defmt::write!(f, "VAL15"),
            0x10 => defmt::write!(f, "VAL16"),
            0x11 => defmt::write!(f, "VAL17"),
            0x12 => defmt::write!(f, "VAL18"),
            0x13 => defmt::write!(f, "VAL19"),
            0x14 => defmt::write!(f, "VAL20"),
            0x16 => defmt::write!(f, "VAL22"),
            0x17 => defmt::write!(f, "VAL23"),
            0x18 => defmt::write!(f, "VAL24"),
            0x19 => defmt::write!(f, "VAL25"),
            0x1a => defmt::write!(f, "VAL26"),
            0x1b => defmt::write!(f, "VAL27"),
            0x1c => defmt::write!(f, "VAL28"),
            0x1d => defmt::write!(f, "VAL29"),
            0x1e => defmt::write!(f, "VAL30"),
            0x21 => defmt::write!(f, "VAL33"),
            0x22 => defmt::write!(f, "VAL34"),
            0x23 => defmt::write!(f, "VAL35"),
            0x24 => defmt::write!(f, "VAL36"),
            0x25 => defmt::write!(f, "VAL37"),
            0x26 => defmt::write!(f, "VAL38"),
            0x30 => defmt::write!(f, "VAL48"),
            0x31 => defmt::write!(f, "VAL49"),
            0x32 => defmt::write!(f, "VAL50"),
            0x33 => defmt::write!(f, "VAL51"),
            0x34 => defmt::write!(f, "VAL52"),
            0x35 => defmt::write!(f, "VAL53"),
            0x36 => defmt::write!(f, "VAL54"),
            0x37 => defmt::write!(f, "VAL55"),
            0x38 => defmt::write!(f, "VAL56"),
            0x39 => defmt::write!(f, "VAL57"),
            0x3a => defmt::write!(f, "VAL58"),
            0x3b => defmt::write!(f, "VAL59"),
            0x3c => defmt::write!(f, "VAL60"),
            0x3d => defmt::write!(f, "VAL61"),
            0x3e => defmt::write!(f, "VAL62"),
            0x3f => defmt::write!(f, "VAL63"),
            0x40 => defmt::write!(f, "VAL64"),
            0x41 => defmt::write!(f, "VAL65"),
            0x42 => defmt::write!(f, "VAL66"),
            0x43 => defmt::write!(f, "VAL67"),
            0x44 => defmt::write!(f, "VAL68"),
            0x45 => defmt::write!(f, "VAL69"),
            0x46 => defmt::write!(f, "VAL70"),
            0x4b => defmt::write!(f, "VAL75"),
            0x4c => defmt::write!(f, "VAL76"),
            0x4d => defmt::write!(f, "VAL77"),
            0x4e => defmt::write!(f, "VAL78"),
            0x4f => defmt::write!(f, "VAL79"),
            0x50 => defmt::write!(f, "VAL80"),
            0x51 => defmt::write!(f, "VAL81"),
            0x52 => defmt::write!(f, "VAL82"),
            0x53 => defmt::write!(f, "VAL83"),
            0x54 => defmt::write!(f, "VAL84"),
            0x5e => defmt::write!(f, "VAL94"),
            0x5f => defmt::write!(f, "VAL95"),
            0x60 => defmt::write!(f, "VAL96"),
            0x61 => defmt::write!(f, "VAL97"),
            0x62 => defmt::write!(f, "VAL98"),
            0x63 => defmt::write!(f, "VAL99"),
            0x64 => defmt::write!(f, "VAL100"),
            0x66 => defmt::write!(f, "VAL102"),
            0x67 => defmt::write!(f, "VAL103"),
            0x71 => defmt::write!(f, "VAL113"),
            0x72 => defmt::write!(f, "VAL114"),
            0x73 => defmt::write!(f, "VAL115"),
            0x74 => defmt::write!(f, "VAL116"),
            0x75 => defmt::write!(f, "VAL117"),
            0x76 => defmt::write!(f, "VAL118"),
            0x77 => defmt::write!(f, "VAL119"),
            0x78 => defmt::write!(f, "VAL120"),
            0x79 => defmt::write!(f, "VAL121"),
            0x7a => defmt::write!(f, "VAL122"),
            0x7b => defmt::write!(f, "VAL123"),
            0x7c => defmt::write!(f, "VAL124"),
            0x7d => defmt::write!(f, "VAL125"),
            0x7e => defmt::write!(f, "VAL126"),
            0x7f => defmt::write!(f, "VAL127"),
            0x80 => defmt::write!(f, "VAL128"),
            0x81 => defmt::write!(f, "VAL129"),
            0x82 => defmt::write!(f, "VAL130"),
            0x83 => defmt::write!(f, "VAL131"),
            0x84 => defmt::write!(f, "VAL132"),
            0x85 => defmt::write!(f, "VAL133"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ctimer4capInp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer4capInp {
        Ctimer4capInp::from_bits(val)
    }
}
impl From<Ctimer4capInp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer4capInp) -> u8 {
        Ctimer4capInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DacTrigTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV"]
    VAL1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected"]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected"]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected"]
    VAL4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected"]
    VAL5 = 0x05,
    #[doc = "CMP0_OUT input is selected"]
    VAL6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT0 input is selected"]
    VAL9 = 0x09,
    #[doc = "CTimer0_MAT1 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CTimer1_MAT0 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CTimer1_MAT1 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CTimer2_MAT0 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CTimer2_MAT1 input is selected"]
    VAL14 = 0x0e,
    #[doc = "LPTMR0 input is selected"]
    VAL15 = 0x0f,
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
    #[doc = "GPIO0 Pin Event Trig 0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected"]
    VAL27 = 0x1b,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL28 = 0x1c,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL29 = 0x1d,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected"]
    VAL30 = 0x1e,
    #[doc = "WUU input is selected"]
    VAL31 = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    VAL37 = 0x25,
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    VAL38 = 0x26,
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    VAL39 = 0x27,
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    VAL40 = 0x28,
    #[doc = "CTimer3_MAT0 input is selected"]
    VAL41 = 0x29,
    #[doc = "CTimer3_MAT1 input is selected"]
    VAL42 = 0x2a,
    #[doc = "CTimer4_MAT0 input is selected"]
    VAL43 = 0x2b,
    #[doc = "CTimer4_MAT1 input is selected"]
    VAL44 = 0x2c,
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
    #[doc = "GPIO0 Pin Event Trig 1 input is selected"]
    VAL62 = 0x3e,
    #[doc = "GPIO1 Pin Event Trig 1 input is selected"]
    VAL63 = 0x3f,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL64 = 0x40,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL65 = 0x41,
    #[doc = "GPIO4 Pin Event Trig 1 input is selected"]
    VAL66 = 0x42,
    _RESERVED_43 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl DacTrigTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DacTrigTrigin {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DacTrigTrigin {
    #[inline(always)]
    fn from(val: u8) -> DacTrigTrigin {
        DacTrigTrigin::from_bits(val)
    }
}
impl From<DacTrigTrigin> for u8 {
    #[inline(always)]
    fn from(val: DacTrigTrigin) -> u8 {
        DacTrigTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetTrigInInp {
    _RESERVED_0 = 0x0,
    #[doc = "10BASE-T1S Rx Event Output is selected"]
    VAL1 = 0x01,
    #[doc = "10BASE-T1S Tx Event Output is selected"]
    VAL2 = 0x02,
    #[doc = "TRIG_IN0 input is selected"]
    VAL3 = 0x03,
    #[doc = "TRIG_IN1 input is selected"]
    VAL4 = 0x04,
    #[doc = "TRIG_IN2 input is selected"]
    VAL5 = 0x05,
    #[doc = "TRIG_IN3 input is selected"]
    VAL6 = 0x06,
    #[doc = "TRIG_IN4 input is selected"]
    VAL7 = 0x07,
    #[doc = "TRIG_IN5 input is selected"]
    VAL8 = 0x08,
    #[doc = "TRIG_IN6 input is selected"]
    VAL9 = 0x09,
    #[doc = "TRIG_IN7 input is selected"]
    VAL10 = 0x0a,
    #[doc = "TRIG_IN8 input is selected"]
    VAL11 = 0x0b,
    #[doc = "TRIG_IN9 input is selected"]
    VAL12 = 0x0c,
    #[doc = "TRIG_IN10 input is selected"]
    VAL13 = 0x0d,
    #[doc = "TRIG_IN11 input is selected"]
    VAL14 = 0x0e,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected"]
    VAL15 = 0x0f,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected"]
    VAL16 = 0x10,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL17 = 0x11,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL18 = 0x12,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected"]
    VAL19 = 0x13,
    #[doc = "GPIO0 Pin Event Trig 1 input is selected"]
    VAL20 = 0x14,
    #[doc = "GPIO1 Pin Event Trig 1 input is selected"]
    VAL21 = 0x15,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL22 = 0x16,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL23 = 0x17,
    #[doc = "GPIO4 Pin Event Trig 1 input is selected"]
    VAL24 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl EnetTrigInInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetTrigInInp {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetTrigInInp {
    #[inline(always)]
    fn from(val: u8) -> EnetTrigInInp {
        EnetTrigInInp::from_bits(val)
    }
}
impl From<EnetTrigInInp> for u8 {
    #[inline(always)]
    fn from(val: EnetTrigInInp) -> u8 {
        EnetTrigInInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexioTrigInp {
    _RESERVED_0 = 0x0,
    #[doc = "AOI0_OUT0 input is selected"]
    VAL1 = 0x01,
    #[doc = "AOI0_OUT1 input is selected"]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT2 input is selected"]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT3 input is selected"]
    VAL4 = 0x04,
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    VAL5 = 0x05,
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    VAL6 = 0x06,
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    VAL7 = 0x07,
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    VAL8 = 0x08,
    #[doc = "CMP0_OUT input is selected"]
    VAL9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "CTimer0_MAT1 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CTimer0_MAT2 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CTimer1_MAT1 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CTimer1_MAT2 input is selected"]
    VAL15 = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "CTimer2_MAT2 input is selected"]
    VAL17 = 0x11,
    #[doc = "LPTMR0 input is selected"]
    VAL18 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "TRIG_IN0 input is selected"]
    VAL24 = 0x18,
    #[doc = "TRIG_IN1 input is selected"]
    VAL25 = 0x19,
    #[doc = "TRIG_IN2 input is selected"]
    VAL26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected"]
    VAL27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected"]
    VAL28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected"]
    VAL29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected"]
    VAL30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected"]
    VAL31 = 0x1f,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected"]
    VAL32 = 0x20,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected"]
    VAL33 = 0x21,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL34 = 0x22,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL35 = 0x23,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected"]
    VAL36 = 0x24,
    #[doc = "WUU input is selected"]
    VAL37 = 0x25,
    #[doc = "LPI2C0 Master End of Packet"]
    VAL38 = 0x26,
    #[doc = "LPI2C0 Slave End of Packet"]
    VAL39 = 0x27,
    #[doc = "LPI2C1 Master End of Packet"]
    VAL40 = 0x28,
    #[doc = "LPI2C1 Slave End of Packet"]
    VAL41 = 0x29,
    #[doc = "LPSPI0 End of Frame"]
    VAL42 = 0x2a,
    #[doc = "LPSPI0 Received Data Word"]
    VAL43 = 0x2b,
    #[doc = "LPSPI1 End of Frame"]
    VAL44 = 0x2c,
    #[doc = "LPSPI1 Received Data Word"]
    VAL45 = 0x2d,
    #[doc = "LPUART0 Received Data Word"]
    VAL46 = 0x2e,
    #[doc = "LPUART0 Transmitted Data Word"]
    VAL47 = 0x2f,
    #[doc = "LPUART0 Receive Line Idle"]
    VAL48 = 0x30,
    #[doc = "LPUART1 Received Data Word"]
    VAL49 = 0x31,
    #[doc = "LPUART1 Transmitted Data Word"]
    VAL50 = 0x32,
    #[doc = "LPUART1 Receive Line Idle"]
    VAL51 = 0x33,
    #[doc = "LPUART2 Received Data Word"]
    VAL52 = 0x34,
    #[doc = "LPUART2 Transmitted Data Word"]
    VAL53 = 0x35,
    #[doc = "LPUART2 Receive Line Idle"]
    VAL54 = 0x36,
    #[doc = "LPUART3 Received Data Word"]
    VAL55 = 0x37,
    #[doc = "LPUART3 Transmitted Data Word"]
    VAL56 = 0x38,
    #[doc = "LPUART3 Receive Line Idle"]
    VAL57 = 0x39,
    #[doc = "LPUART4 Received Data Word"]
    VAL58 = 0x3a,
    #[doc = "LPUART4 Transmitted Data Word"]
    VAL59 = 0x3b,
    #[doc = "LPUART4 Receive Line Idle"]
    VAL60 = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
    _RESERVED_40 = 0x40,
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    VAL65 = 0x41,
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    VAL66 = 0x42,
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    VAL67 = 0x43,
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    VAL68 = 0x44,
    #[doc = "CTimer3_MAT2 input is selected"]
    VAL69 = 0x45,
    #[doc = "CTimer3_MAT3 input is selected"]
    VAL70 = 0x46,
    #[doc = "CTimer4_MAT2 input is selected"]
    VAL71 = 0x47,
    #[doc = "CTimer4_MAT3 input is selected"]
    VAL72 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    #[doc = "LPI2C2 Master End of Packet"]
    VAL77 = 0x4d,
    #[doc = "LPI2C2 Slave End of Packet"]
    VAL78 = 0x4e,
    #[doc = "LPI2C3 Master End of Packet"]
    VAL79 = 0x4f,
    #[doc = "LPI2C3 Slave End of Packet"]
    VAL80 = 0x50,
    #[doc = "LPSPI2 End of Frame input is selected"]
    VAL81 = 0x51,
    #[doc = "LPSPI2 Received Data Word input is selected"]
    VAL82 = 0x52,
    #[doc = "LPSPI3 End of Frame input is selected"]
    VAL83 = 0x53,
    #[doc = "LPSPI3 Received Data Word input is selected"]
    VAL84 = 0x54,
    #[doc = "GPIO0 Pin Event Trig 1 input is selected"]
    VAL85 = 0x55,
    #[doc = "GPIO1 Pin Event Trig 1 input is selected"]
    VAL86 = 0x56,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL87 = 0x57,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL88 = 0x58,
    #[doc = "GPIO4 Pin Event Trig 1 input is selected"]
    VAL89 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl FlexioTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexioTrigInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexioTrigInp {
    #[inline(always)]
    fn from(val: u8) -> FlexioTrigInp {
        FlexioTrigInp::from_bits(val)
    }
}
impl From<FlexioTrigInp> for u8 {
    #[inline(always)]
    fn from(val: FlexioTrigInp) -> u8 {
        FlexioTrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqmeasRefInp {
    _RESERVED_0 = 0x0,
    #[doc = "clk_in input is selected"]
    VAL1 = 0x01,
    #[doc = "FRO_OSC_12M input is selected"]
    VAL2 = 0x02,
    #[doc = "fro_hf_div input is selected"]
    VAL3 = 0x03,
    #[doc = "OSC32K\\[1\\] input is selected"]
    VAL4 = 0x04,
    #[doc = "clk_16k\\[1\\] input is selected"]
    VAL5 = 0x05,
    #[doc = "SLOW_CLK input is selected"]
    VAL6 = 0x06,
    #[doc = "FREQME_CLK_IN0 input is selected"]
    VAL7 = 0x07,
    #[doc = "FREQME_CLK_IN1 input is selected input is selected"]
    VAL8 = 0x08,
    #[doc = "AOI0_OUT0 input is selected"]
    VAL9 = 0x09,
    #[doc = "AOI0_OUT1"]
    VAL10 = 0x0a,
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
    #[doc = "pll1_clk_div input is selected"]
    VAL31 = 0x1f,
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
impl FreqmeasRefInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqmeasRefInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FreqmeasRefInp {
    #[inline(always)]
    fn from(val: u8) -> FreqmeasRefInp {
        FreqmeasRefInp::from_bits(val)
    }
}
impl From<FreqmeasRefInp> for u8 {
    #[inline(always)]
    fn from(val: FreqmeasRefInp) -> u8 {
        FreqmeasRefInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqmeasTarInp {
    _RESERVED_0 = 0x0,
    #[doc = "clk_in input is selected"]
    VAL1 = 0x01,
    #[doc = "FRO_OSC_12M input is selected"]
    VAL2 = 0x02,
    #[doc = "fro_hf_div input is selected"]
    VAL3 = 0x03,
    #[doc = "OSC32K\\[1\\] input is selected"]
    VAL4 = 0x04,
    #[doc = "clk_16k\\[1\\] input is selected"]
    VAL5 = 0x05,
    #[doc = "SLOW_CLK input is selected"]
    VAL6 = 0x06,
    #[doc = "FREQME_CLK_IN0 input is selected"]
    VAL7 = 0x07,
    #[doc = "FREQME_CLK_IN1 input is selected input is selected"]
    VAL8 = 0x08,
    #[doc = "AOI0_OUT0 input is selected"]
    VAL9 = 0x09,
    #[doc = "AOI0_OUT1"]
    VAL10 = 0x0a,
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
    #[doc = "pll1_clk_div input is selected"]
    VAL31 = 0x1f,
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
impl FreqmeasTarInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqmeasTarInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FreqmeasTarInp {
    #[inline(always)]
    fn from(val: u8) -> FreqmeasTarInp {
        FreqmeasTarInp::from_bits(val)
    }
}
impl From<FreqmeasTarInp> for u8 {
    #[inline(always)]
    fn from(val: FreqmeasTarInp) -> u8 {
        FreqmeasTarInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2cTrigInp {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected"]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected"]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected"]
    VAL4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected"]
    VAL5 = 0x05,
    #[doc = "CMP0_OUT input is selected"]
    VAL6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT0 input is selected"]
    VAL9 = 0x09,
    #[doc = "CTimer0_MAT1 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CTimer1_MAT0 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CTimer1_MAT1 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CTimer2_MAT0 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CTimer2_MAT1 input is selected"]
    VAL14 = 0x0e,
    #[doc = "LPTMR0 input is selected"]
    VAL15 = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "TRIG_IN0 input is selected"]
    VAL17 = 0x11,
    #[doc = "TRIG_IN1 input is selected"]
    VAL18 = 0x12,
    #[doc = "TRIG_IN2 input is selected"]
    VAL19 = 0x13,
    #[doc = "TRIG_IN3 input is selected"]
    VAL20 = 0x14,
    #[doc = "TRIG_IN4 input is selected"]
    VAL21 = 0x15,
    #[doc = "TRIG_IN5 input is selected"]
    VAL22 = 0x16,
    #[doc = "TRIG_IN6 input is selected"]
    VAL23 = 0x17,
    #[doc = "TRIG_IN7 input is selected"]
    VAL24 = 0x18,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected"]
    VAL25 = 0x19,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL27 = 0x1b,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL28 = 0x1c,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected"]
    VAL29 = 0x1d,
    #[doc = "WUU input is selected"]
    VAL30 = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    #[doc = "CTimer3_MAT2 input is selected"]
    VAL35 = 0x23,
    #[doc = "CTimer3_MAT3 input is selected"]
    VAL36 = 0x24,
    #[doc = "CTimer4_MAT2 input is selected"]
    VAL37 = 0x25,
    #[doc = "CTimer4_MAT3 input is selected"]
    VAL38 = 0x26,
    #[doc = "FlexIO0 CH0 input is selected"]
    VAL39 = 0x27,
    #[doc = "FlexIO0 CH1 input is selected"]
    VAL40 = 0x28,
    #[doc = "FlexIO0 CH2 input is selected"]
    VAL41 = 0x29,
    #[doc = "FlexIO0 CH3 input is selected"]
    VAL42 = 0x2a,
    #[doc = "GPIO0 Pin Event Trig 1 input is selected"]
    VAL43 = 0x2b,
    #[doc = "GPIO1 Pin Event Trig 1 input is selected"]
    VAL44 = 0x2c,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL45 = 0x2d,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL46 = 0x2e,
    #[doc = "GPIO4 Pin Event Trig 1 input is selected"]
    VAL47 = 0x2f,
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
impl Lpi2cTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2cTrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2cTrigInp {
    #[inline(always)]
    fn from(val: u8) -> Lpi2cTrigInp {
        Lpi2cTrigInp::from_bits(val)
    }
}
impl From<Lpi2cTrigInp> for u8 {
    #[inline(always)]
    fn from(val: Lpi2cTrigInp) -> u8 {
        Lpi2cTrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpspiTrigInp {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected"]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected"]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected"]
    VAL4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected"]
    VAL5 = 0x05,
    #[doc = "CMP0_OUT input is selected"]
    VAL6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT1 input is selected"]
    VAL9 = 0x09,
    #[doc = "CTimer0_MAT2 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CTimer1_MAT1 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CTimer1_MAT2 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CTimer2_MAT1 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CTimer2_MAT2 input is selected"]
    VAL14 = 0x0e,
    #[doc = "LPTMR0 input is selected"]
    VAL15 = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "TRIG_IN0 input is selected"]
    VAL17 = 0x11,
    #[doc = "TRIG_IN1 input is selected"]
    VAL18 = 0x12,
    #[doc = "TRIG_IN2 input is selected"]
    VAL19 = 0x13,
    #[doc = "TRIG_IN3 input is selected"]
    VAL20 = 0x14,
    #[doc = "TRIG_IN4 input is selected"]
    VAL21 = 0x15,
    #[doc = "TRIG_IN5 input is selected"]
    VAL22 = 0x16,
    #[doc = "TRIG_IN6 input is selected"]
    VAL23 = 0x17,
    #[doc = "TRIG_IN7 input is selected"]
    VAL24 = 0x18,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected"]
    VAL25 = 0x19,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected"]
    VAL26 = 0x1a,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL27 = 0x1b,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL28 = 0x1c,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected"]
    VAL29 = 0x1d,
    #[doc = "WUU input is selected"]
    VAL30 = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    #[doc = "CTimer3_MAT2 inputs is selected"]
    VAL35 = 0x23,
    #[doc = "CTimer3_MAT3 input is selected"]
    VAL36 = 0x24,
    #[doc = "CTimer4_MAT2 input is selected"]
    VAL37 = 0x25,
    #[doc = "CTimer4_MAT3 input is selected"]
    VAL38 = 0x26,
    #[doc = "FlexIO0 CH0 input is selected"]
    VAL39 = 0x27,
    #[doc = "FlexIO0 CH1 input is selected"]
    VAL40 = 0x28,
    #[doc = "FlexIO0 CH2 input is selected"]
    VAL41 = 0x29,
    #[doc = "FlexIO0 CH3 input is selected"]
    VAL42 = 0x2a,
    #[doc = "GPIO0 Pin Event Trig 1 input is selected"]
    VAL43 = 0x2b,
    #[doc = "GPIO1 Pin Event Trig 1 input is selected"]
    VAL44 = 0x2c,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL45 = 0x2d,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL46 = 0x2e,
    #[doc = "GPIO4 Pin Event Trig 1 input is selected"]
    VAL47 = 0x2f,
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
impl LpspiTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpspiTrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpspiTrigInp {
    #[inline(always)]
    fn from(val: u8) -> LpspiTrigInp {
        LpspiTrigInp::from_bits(val)
    }
}
impl From<LpspiTrigInp> for u8 {
    #[inline(always)]
    fn from(val: LpspiTrigInp) -> u8 {
        LpspiTrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpuartInp {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected"]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected"]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected"]
    VAL4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected"]
    VAL5 = 0x05,
    #[doc = "CMP0_OUT input is selected"]
    VAL6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected"]
    VAL9 = 0x09,
    #[doc = "CTimer0_MAT3 input is selected"]
    VAL10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected"]
    VAL11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected"]
    VAL14 = 0x0e,
    #[doc = "LPTMR0 input is selected"]
    VAL15 = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "TRIG_IN0 input is selected"]
    VAL17 = 0x11,
    #[doc = "TRIG_IN1 input is selected"]
    VAL18 = 0x12,
    #[doc = "TRIG_IN2 input is selected"]
    VAL19 = 0x13,
    #[doc = "TRIG_IN3 input is selected"]
    VAL20 = 0x14,
    #[doc = "TRIG_IN4 input is selected"]
    VAL21 = 0x15,
    #[doc = "TRIG_IN5 input is selected"]
    VAL22 = 0x16,
    #[doc = "TRIG_IN6 input is selected"]
    VAL23 = 0x17,
    #[doc = "TRIG_IN7 input is selected"]
    VAL24 = 0x18,
    #[doc = "TRIG_IN8 input is selected"]
    VAL25 = 0x19,
    #[doc = "TRIG_IN9 input is selected"]
    VAL26 = 0x1a,
    #[doc = "TRIG_IN10 input is selected"]
    VAL27 = 0x1b,
    #[doc = "TRIG_IN11 input is selected"]
    VAL28 = 0x1c,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected"]
    VAL29 = 0x1d,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected"]
    VAL30 = 0x1e,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL31 = 0x1f,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL32 = 0x20,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected"]
    VAL33 = 0x21,
    #[doc = "WUU selected"]
    VAL34 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    #[doc = "CTimer3_MAT2 input is selected"]
    VAL40 = 0x28,
    #[doc = "CTimer3_MAT3 input is selected"]
    VAL41 = 0x29,
    #[doc = "CTimer4_MAT2 input is selected"]
    VAL42 = 0x2a,
    #[doc = "CTimer4_MAT3 input is selected"]
    VAL43 = 0x2b,
    #[doc = "FlexIO0 CH0 input is selected"]
    VAL44 = 0x2c,
    #[doc = "FlexIO0 CH1 input is selected"]
    VAL45 = 0x2d,
    #[doc = "FlexIO0 CH2 input is selected"]
    VAL46 = 0x2e,
    #[doc = "FlexIO0 CH3 input is selected"]
    VAL47 = 0x2f,
    #[doc = "GPIO0 Pin Event Trig 1 input is selected"]
    VAL48 = 0x30,
    #[doc = "GPIO1 Pin Event Trig 1 input is selected"]
    VAL49 = 0x31,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL50 = 0x32,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL51 = 0x33,
    #[doc = "GPIO4 Pin Event Trig 1 input is selected"]
    VAL52 = 0x34,
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
impl LpuartInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpuartInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpuartInp {
    #[inline(always)]
    fn from(val: u8) -> LpuartInp {
        LpuartInp::from_bits(val)
    }
}
impl From<LpuartInp> for u8 {
    #[inline(always)]
    fn from(val: LpuartInp) -> u8 {
        LpuartInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmartDmaTrigInp {
    _RESERVED_0 = 0x0,
    #[doc = "GPIO P0_16 input is selected"]
    VAL1 = 0x01,
    #[doc = "GPIO P0_17 input is selected"]
    VAL2 = 0x02,
    #[doc = "GPIO P1_8 input is selected"]
    VAL3 = 0x03,
    #[doc = "GPIO P1_9 input is selected"]
    VAL4 = 0x04,
    #[doc = "GPIO P1_10 input is selected"]
    VAL5 = 0x05,
    #[doc = "GPIO P1_11 input is selected"]
    VAL6 = 0x06,
    #[doc = "GPIO P1_12 input is selected"]
    VAL7 = 0x07,
    #[doc = "GPIO P1_13 input is selected"]
    VAL8 = 0x08,
    #[doc = "GPIO P2_0 input is selected"]
    VAL9 = 0x09,
    #[doc = "GPIO P2_1 input is selected"]
    VAL10 = 0x0a,
    #[doc = "GPIO P2_2 input is selected"]
    VAL11 = 0x0b,
    #[doc = "GPIO P2_3 input is selected"]
    VAL12 = 0x0c,
    #[doc = "GPIO P2_6 input is selected"]
    VAL13 = 0x0d,
    #[doc = "GPIO P3_8 input is selected"]
    VAL14 = 0x0e,
    #[doc = "GPIO P3_9 input is selected"]
    VAL15 = 0x0f,
    #[doc = "GPIO P3_10 input is selected"]
    VAL16 = 0x10,
    #[doc = "GPIO P3_11 input is selected"]
    VAL17 = 0x11,
    #[doc = "GPIO P3_12 input is seclected"]
    VAL18 = 0x12,
    #[doc = "GPIO0 Pin Event Trig input is selected"]
    VAL19 = 0x13,
    #[doc = "GPIO1 Pin Event Trig input is selected"]
    VAL20 = 0x14,
    #[doc = "GPIO2 Pin Event Trig input is selected"]
    VAL21 = 0x15,
    #[doc = "GPIO3 Pin Event Trig input is selected"]
    VAL22 = 0x16,
    #[doc = "GPIO4 Pin Event Trig input is selected"]
    VAL23 = 0x17,
    #[doc = "ARM_TXEV input is selected"]
    VAL24 = 0x18,
    #[doc = "AOI0_OUT0 input is selected"]
    VAL25 = 0x19,
    _RESERVED_1a = 0x1a,
    #[doc = "DMA_IRQ input is selected"]
    VAL27 = 0x1b,
    _RESERVED_1c = 0x1c,
    #[doc = "WUU_IRQ input is selected"]
    VAL29 = 0x1d,
    #[doc = "CTimer0_MAT2 input is selected"]
    VAL30 = 0x1e,
    #[doc = "CTimer0_MAT3 input is selected"]
    VAL31 = 0x1f,
    #[doc = "CTimer1_MAT2 input is selected"]
    VAL32 = 0x20,
    #[doc = "CTimer1_MAT3 input is selected"]
    VAL33 = 0x21,
    #[doc = "CTimer2_MAT2 input is selected"]
    VAL34 = 0x22,
    #[doc = "CTimer2_MAT3 input is selected"]
    VAL35 = 0x23,
    #[doc = "CTimer3_MAT2 input is selected"]
    VAL36 = 0x24,
    #[doc = "CTimer3_MAT3 input is selected"]
    VAL37 = 0x25,
    #[doc = "CTimer4_MAT2 input is selected"]
    VAL38 = 0x26,
    #[doc = "CTimer4_MAT3 input is selected"]
    VAL39 = 0x27,
    #[doc = "OSTIMER_IRQ input is selected"]
    VAL40 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    #[doc = "RTC_Alarm_IRQ input is selected"]
    VAL45 = 0x2d,
    _RESERVED_2e = 0x2e,
    #[doc = "uTICK_IRQ input is selected"]
    VAL47 = 0x2f,
    #[doc = "WDT_IRQ input is selected"]
    VAL48 = 0x30,
    #[doc = "Wakeup_Timer_IRQ input is selected"]
    VAL49 = 0x31,
    #[doc = "CAN0_IRQ input is selected"]
    VAL50 = 0x32,
    #[doc = "CAN1_IRQ input is selected"]
    VAL51 = 0x33,
    #[doc = "FlexIO0_IRQ input is selected"]
    VAL52 = 0x34,
    #[doc = "FlexIO0_Shifer0_DMA_Req input is selected"]
    VAL53 = 0x35,
    #[doc = "FlexIO0_Shifer1_DMA_Req input is selected"]
    VAL54 = 0x36,
    #[doc = "FlexIO0_Shifer2_DMA_Req input is selected"]
    VAL55 = 0x37,
    #[doc = "FlexIO0_Shifer3_DMA_Req input is selected"]
    VAL56 = 0x38,
    #[doc = "I3C0_IRQ input is selected"]
    VAL57 = 0x39,
    #[doc = "LPI2C0_IRQ input is selected"]
    VAL58 = 0x3a,
    #[doc = "LPI2C1_IRQ input is selected"]
    VAL59 = 0x3b,
    #[doc = "LPSPI0_IRQ input is selected"]
    VAL60 = 0x3c,
    #[doc = "LPSPI1_IRQ input is selected"]
    VAL61 = 0x3d,
    #[doc = "LPUART0_IRQ input is selected"]
    VAL62 = 0x3e,
    #[doc = "LPUART1_IRQ input is selected"]
    VAL63 = 0x3f,
    #[doc = "LPUART2_IRQ input is selected"]
    VAL64 = 0x40,
    #[doc = "LPUART3_IRQ input is selected"]
    VAL65 = 0x41,
    _RESERVED_42 = 0x42,
    #[doc = "USB1 Start of Frame input is selected"]
    VAL67 = 0x43,
    #[doc = "ADC0_IRQ input is selected"]
    VAL68 = 0x44,
    #[doc = "ADC1_IRQ input is selected"]
    VAL69 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    #[doc = "CMP0_IRQ input is selected"]
    VAL72 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    #[doc = "CMP0_OUT input is selected"]
    VAL75 = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    #[doc = "DAC0_IRQ input is selected"]
    VAL78 = 0x4e,
    _RESERVED_4f = 0x4f,
    #[doc = "DMA1_IRQ input is selected"]
    VAL80 = 0x50,
    #[doc = "DAC1_IRQ input is selected"]
    VAL81 = 0x51,
    #[doc = "TSI0_End_of_Scan_IRQ input is selected"]
    VAL82 = 0x52,
    #[doc = "TSI0_Out_of_Range_IRQ input is selected"]
    VAL83 = 0x53,
    #[doc = "ENET QOS IRQ input is selected"]
    VAL84 = 0x54,
    #[doc = "10BASE_T1S IRQ input is selected"]
    VAL85 = 0x55,
    #[doc = "ERM Interrupt input is selected"]
    VAL86 = 0x56,
    #[doc = "TMPR_OUT0 input is selected"]
    VAL87 = 0x57,
    #[doc = "TMPR_OUT1 input is selected"]
    VAL88 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl SmartDmaTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmartDmaTrigInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmartDmaTrigInp {
    #[inline(always)]
    fn from(val: u8) -> SmartDmaTrigInp {
        SmartDmaTrigInp::from_bits(val)
    }
}
impl From<SmartDmaTrigInp> for u8 {
    #[inline(always)]
    fn from(val: SmartDmaTrigInp) -> u8 {
        SmartDmaTrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum T1sWkupInp {
    _RESERVED_0 = 0x0,
    #[doc = "AOI0_OUT0 input is selected"]
    VAL1 = 0x01,
    #[doc = "AOI0_OUT1 input is selected"]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT2 input is selected"]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT3 input is selected"]
    VAL4 = 0x04,
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    VAL5 = 0x05,
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    VAL6 = 0x06,
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    VAL7 = 0x07,
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    VAL8 = 0x08,
    #[doc = "CMP0_OUT input is selected"]
    VAL9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "CTimer0_MAT1 input is selected"]
    VAL12 = 0x0c,
    #[doc = "CTimer0_MAT2 input is selected"]
    VAL13 = 0x0d,
    #[doc = "CTimer1_MAT1 input is selected"]
    VAL14 = 0x0e,
    #[doc = "CTimer1_MAT2 input is selected"]
    VAL15 = 0x0f,
    #[doc = "CTimer2_MAT1 input is selected"]
    VAL16 = 0x10,
    #[doc = "CTimer2_MAT2 input is selected"]
    VAL17 = 0x11,
    #[doc = "LPTMR0 input is seclected"]
    VAL18 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "TRIG_IN0 input is selected"]
    VAL24 = 0x18,
    #[doc = "TRIG_IN1 input is selected"]
    VAL25 = 0x19,
    #[doc = "TRIG_IN2 input is selected"]
    VAL26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected"]
    VAL27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected"]
    VAL28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected"]
    VAL29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected"]
    VAL30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected"]
    VAL31 = 0x1f,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected"]
    VAL32 = 0x20,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected"]
    VAL33 = 0x21,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
    VAL34 = 0x22,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
    VAL35 = 0x23,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected"]
    VAL36 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    #[doc = "LPSPI0 Received Data Word input is selected"]
    VAL43 = 0x2b,
    _RESERVED_2c = 0x2c,
    #[doc = "LPSPI1 Received Data Word input is selected"]
    VAL45 = 0x2d,
    #[doc = "LPUART0 Received Data Word input is selected"]
    VAL46 = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "LPUART1 Received Data Word input is selected"]
    VAL49 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    #[doc = "LPUART2 Received Data Word input is selected"]
    VAL52 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    #[doc = "LPUART3 Received Data Word input is selected"]
    VAL55 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    #[doc = "LPUART4 Received Data Word input is selected"]
    VAL58 = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
    _RESERVED_40 = 0x40,
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    VAL65 = 0x41,
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    VAL66 = 0x42,
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    VAL67 = 0x43,
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    VAL68 = 0x44,
    #[doc = "CTimer3_MAT2 input is selected"]
    VAL69 = 0x45,
    #[doc = "CTimer3_MAT3 input is selected"]
    VAL70 = 0x46,
    #[doc = "CTimer4_MAT2 input is selected"]
    VAL71 = 0x47,
    #[doc = "CTimer4_MAT3 input is selected"]
    VAL72 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    #[doc = "LPSPI2 Received Data Word input is selected"]
    VAL82 = 0x52,
    _RESERVED_53 = 0x53,
    #[doc = "LPSPI3 Received Data Word input is selected"]
    VAL84 = 0x54,
    #[doc = "GPIO0 Pin Event Trig 1 input is selected"]
    VAL85 = 0x55,
    #[doc = "GPIO1 Pin Event Trig 1 input is selected"]
    VAL86 = 0x56,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
    VAL87 = 0x57,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
    VAL88 = 0x58,
    #[doc = "GPIO4 Pin Event Trig 1 input is selected"]
    VAL89 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl T1sWkupInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> T1sWkupInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for T1sWkupInp {
    #[inline(always)]
    fn from(val: u8) -> T1sWkupInp {
        T1sWkupInp::from_bits(val)
    }
}
impl From<T1sWkupInp> for u8 {
    #[inline(always)]
    fn from(val: T1sWkupInp) -> u8 {
        T1sWkupInp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Timer0trigInp(u8);
impl Timer0trigInp {
    #[doc = "CT_INP0 input is selected"]
    pub const VAL1: Self = Self(0x01);
    #[doc = "CT_INP1 input is selected"]
    pub const VAL2: Self = Self(0x02);
    #[doc = "CT_INP2 input is selected"]
    pub const VAL3: Self = Self(0x03);
    #[doc = "CT_INP3 input is selected"]
    pub const VAL4: Self = Self(0x04);
    #[doc = "CT_INP4 input is selected"]
    pub const VAL5: Self = Self(0x05);
    #[doc = "CT_INP5 input is selected"]
    pub const VAL6: Self = Self(0x06);
    #[doc = "CT_INP6 input is selected"]
    pub const VAL7: Self = Self(0x07);
    #[doc = "CT_INP7 input is selected"]
    pub const VAL8: Self = Self(0x08);
    #[doc = "CT_INP8 input is selected"]
    pub const VAL9: Self = Self(0x09);
    #[doc = "CT_INP9 input is selected"]
    pub const VAL10: Self = Self(0x0a);
    #[doc = "CT_INP10 input is selected"]
    pub const VAL11: Self = Self(0x0b);
    #[doc = "CT_INP11 input is selected"]
    pub const VAL12: Self = Self(0x0c);
    #[doc = "CT_INP12 input is selected"]
    pub const VAL13: Self = Self(0x0d);
    #[doc = "CT_INP13 input is selected"]
    pub const VAL14: Self = Self(0x0e);
    #[doc = "CT_INP14 input is selected"]
    pub const VAL15: Self = Self(0x0f);
    #[doc = "CT_INP15 input is selected"]
    pub const VAL16: Self = Self(0x10);
    #[doc = "CT_INP16 input is selected"]
    pub const VAL17: Self = Self(0x11);
    #[doc = "CT_INP17 input is selected"]
    pub const VAL18: Self = Self(0x12);
    #[doc = "CT_INP18 input is selected"]
    pub const VAL19: Self = Self(0x13);
    #[doc = "CT_INP19 input is selected"]
    pub const VAL20: Self = Self(0x14);
    #[doc = "AOI0_OUT0 input is selected"]
    pub const VAL22: Self = Self(0x16);
    #[doc = "AOI0_OUT1 input is selected"]
    pub const VAL23: Self = Self(0x17);
    #[doc = "AOI0_OUT2 input is selected"]
    pub const VAL24: Self = Self(0x18);
    #[doc = "AOI0_OUT3 input is selected"]
    pub const VAL25: Self = Self(0x19);
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    pub const VAL26: Self = Self(0x1a);
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    pub const VAL27: Self = Self(0x1b);
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    pub const VAL28: Self = Self(0x1c);
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    pub const VAL29: Self = Self(0x1d);
    #[doc = "CMP0_OUT is selected"]
    pub const VAL30: Self = Self(0x1e);
    #[doc = "CTimer1_MAT1 input is selected"]
    pub const VAL33: Self = Self(0x21);
    #[doc = "CTimer1_MAT2 input is selected"]
    pub const VAL34: Self = Self(0x22);
    #[doc = "CTimer1_MAT3 input is selected"]
    pub const VAL35: Self = Self(0x23);
    #[doc = "CTimer2_MAT1 input is selected"]
    pub const VAL36: Self = Self(0x24);
    #[doc = "CTimer2_MAT2 input is selected"]
    pub const VAL37: Self = Self(0x25);
    #[doc = "CTimer2_MAT3 input is selected"]
    pub const VAL38: Self = Self(0x26);
    #[doc = "LPI2C0 Master End of Packet input is selected"]
    pub const VAL48: Self = Self(0x30);
    #[doc = "LPI2C0 Slave End of Packet input is selected"]
    pub const VAL49: Self = Self(0x31);
    #[doc = "LPI2C1 Master End of Packet input is selected"]
    pub const VAL50: Self = Self(0x32);
    #[doc = "LPI2C1 Slave End of Packet input is selected"]
    pub const VAL51: Self = Self(0x33);
    #[doc = "LPSPI0 End of Frame input is selected"]
    pub const VAL52: Self = Self(0x34);
    #[doc = "LPSPI0 Received Data Word input is selected"]
    pub const VAL53: Self = Self(0x35);
    #[doc = "LPSPI1 End of Frame input is selected"]
    pub const VAL54: Self = Self(0x36);
    #[doc = "LPSPI1 Received Data Word input is selected"]
    pub const VAL55: Self = Self(0x37);
    #[doc = "LPUART0 Received Data Word input is selected"]
    pub const VAL56: Self = Self(0x38);
    #[doc = "LPUART0 Transmitted Data Word input is selected"]
    pub const VAL57: Self = Self(0x39);
    #[doc = "LPUART0 Receive Line Idle input is selected"]
    pub const VAL58: Self = Self(0x3a);
    #[doc = "LPUART1 Received Data Word input is selected"]
    pub const VAL59: Self = Self(0x3b);
    #[doc = "LPUART1 Transmitted Data Word input is selected"]
    pub const VAL60: Self = Self(0x3c);
    #[doc = "LPUART1 Receive Line Idle input is selected"]
    pub const VAL61: Self = Self(0x3d);
    #[doc = "LPUART2 Received Data Word input is selected"]
    pub const VAL62: Self = Self(0x3e);
    #[doc = "LPUART2 Transmitted Data Word input is selected"]
    pub const VAL63: Self = Self(0x3f);
    #[doc = "LPUART2 Receive Line Idle input is selected"]
    pub const VAL64: Self = Self(0x40);
    #[doc = "LPUART3 Received Data Word input is selected"]
    pub const VAL65: Self = Self(0x41);
    #[doc = "LPUART3 Transmitted Data Word input is selected"]
    pub const VAL66: Self = Self(0x42);
    #[doc = "LPUART3 Receive Line Idle input is selected"]
    pub const VAL67: Self = Self(0x43);
    #[doc = "LPUART4 Received Data Word input is selected"]
    pub const VAL68: Self = Self(0x44);
    #[doc = "LPUART4 Transmitted Data Word input is selected"]
    pub const VAL69: Self = Self(0x45);
    #[doc = "LPUART4 Receive Line Idle input is selected"]
    pub const VAL70: Self = Self(0x46);
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    pub const VAL75: Self = Self(0x4b);
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    pub const VAL76: Self = Self(0x4c);
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    pub const VAL77: Self = Self(0x4d);
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    pub const VAL78: Self = Self(0x4e);
    #[doc = "CTimer3_MAT1 input is selected"]
    pub const VAL79: Self = Self(0x4f);
    #[doc = "CTimer3_MAT2 input is selected"]
    pub const VAL80: Self = Self(0x50);
    #[doc = "CTimer3_MAT3 input is selected"]
    pub const VAL81: Self = Self(0x51);
    #[doc = "CTimer4_MAT1 input is selected"]
    pub const VAL82: Self = Self(0x52);
    #[doc = "CTimer4_MAT2 input is selected"]
    pub const VAL83: Self = Self(0x53);
    #[doc = "CTimer4_MAT3 input is selected"]
    pub const VAL84: Self = Self(0x54);
    #[doc = "LPI2C2 Master End of Packet input is selected"]
    pub const VAL94: Self = Self(0x5e);
    #[doc = "LPI2C2 Slave End of Packet input is selected"]
    pub const VAL95: Self = Self(0x5f);
    #[doc = "LPI2C3 Master End of Packet input is selected"]
    pub const VAL96: Self = Self(0x60);
    #[doc = "LPI2C3 Slave End of Packet input is selected"]
    pub const VAL97: Self = Self(0x61);
    #[doc = "LPUART5 Received Data Word input is selected"]
    pub const VAL98: Self = Self(0x62);
    #[doc = "LPUART5 Transmitted Data Word input is selected"]
    pub const VAL99: Self = Self(0x63);
    #[doc = "LPUART5 Receive Line Idle input is selected"]
    pub const VAL100: Self = Self(0x64);
    #[doc = "TRIG_IN0 input is selected"]
    pub const VAL113: Self = Self(0x71);
    #[doc = "TRIG_IN1 input is selected"]
    pub const VAL114: Self = Self(0x72);
    #[doc = "TRIG_IN2 input is selected"]
    pub const VAL115: Self = Self(0x73);
    #[doc = "TRIG_IN3 input is selected"]
    pub const VAL116: Self = Self(0x74);
    #[doc = "TRIG_IN4 input is selected"]
    pub const VAL117: Self = Self(0x75);
    #[doc = "TRIG_IN5 input is selected"]
    pub const VAL118: Self = Self(0x76);
    #[doc = "TRIG_IN6 input is selected"]
    pub const VAL119: Self = Self(0x77);
    #[doc = "TRIG_IN7 input is selected"]
    pub const VAL120: Self = Self(0x78);
    #[doc = "TRIG_IN8 input is selected"]
    pub const VAL121: Self = Self(0x79);
    #[doc = "TRIG_IN9 input is selected"]
    pub const VAL122: Self = Self(0x7a);
    #[doc = "TRIG_IN10 input is selected"]
    pub const VAL123: Self = Self(0x7b);
    #[doc = "TRIG_IN11 input is selected"]
    pub const VAL124: Self = Self(0x7c);
    #[doc = "USB1 Start of Frame input is selected"]
    pub const VAL125: Self = Self(0x7d);
    #[doc = "LPSPI2 End of Frame input is selected"]
    pub const VAL126: Self = Self(0x7e);
    #[doc = "LPSPI2 Received Data Word input is selected"]
    pub const VAL127: Self = Self(0x7f);
    #[doc = "LPSPI3 End of Frame input is selected"]
    pub const VAL128: Self = Self(0x80);
    #[doc = "LPSPI3 Received Data Word input is selected"]
    pub const VAL129: Self = Self(0x81);
    #[doc = "LPSPI4 End of Frame input is selected"]
    pub const VAL130: Self = Self(0x82);
    #[doc = "LPSPI4 Received Data Word input is selected"]
    pub const VAL131: Self = Self(0x83);
    #[doc = "LPSPI5 End of Frame input is selected"]
    pub const VAL132: Self = Self(0x84);
    #[doc = "LPSPI5 Received Data Word input is selected"]
    pub const VAL133: Self = Self(0x85);
}
impl Timer0trigInp {
    pub const fn from_bits(val: u8) -> Timer0trigInp {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Timer0trigInp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("VAL1"),
            0x02 => f.write_str("VAL2"),
            0x03 => f.write_str("VAL3"),
            0x04 => f.write_str("VAL4"),
            0x05 => f.write_str("VAL5"),
            0x06 => f.write_str("VAL6"),
            0x07 => f.write_str("VAL7"),
            0x08 => f.write_str("VAL8"),
            0x09 => f.write_str("VAL9"),
            0x0a => f.write_str("VAL10"),
            0x0b => f.write_str("VAL11"),
            0x0c => f.write_str("VAL12"),
            0x0d => f.write_str("VAL13"),
            0x0e => f.write_str("VAL14"),
            0x0f => f.write_str("VAL15"),
            0x10 => f.write_str("VAL16"),
            0x11 => f.write_str("VAL17"),
            0x12 => f.write_str("VAL18"),
            0x13 => f.write_str("VAL19"),
            0x14 => f.write_str("VAL20"),
            0x16 => f.write_str("VAL22"),
            0x17 => f.write_str("VAL23"),
            0x18 => f.write_str("VAL24"),
            0x19 => f.write_str("VAL25"),
            0x1a => f.write_str("VAL26"),
            0x1b => f.write_str("VAL27"),
            0x1c => f.write_str("VAL28"),
            0x1d => f.write_str("VAL29"),
            0x1e => f.write_str("VAL30"),
            0x21 => f.write_str("VAL33"),
            0x22 => f.write_str("VAL34"),
            0x23 => f.write_str("VAL35"),
            0x24 => f.write_str("VAL36"),
            0x25 => f.write_str("VAL37"),
            0x26 => f.write_str("VAL38"),
            0x30 => f.write_str("VAL48"),
            0x31 => f.write_str("VAL49"),
            0x32 => f.write_str("VAL50"),
            0x33 => f.write_str("VAL51"),
            0x34 => f.write_str("VAL52"),
            0x35 => f.write_str("VAL53"),
            0x36 => f.write_str("VAL54"),
            0x37 => f.write_str("VAL55"),
            0x38 => f.write_str("VAL56"),
            0x39 => f.write_str("VAL57"),
            0x3a => f.write_str("VAL58"),
            0x3b => f.write_str("VAL59"),
            0x3c => f.write_str("VAL60"),
            0x3d => f.write_str("VAL61"),
            0x3e => f.write_str("VAL62"),
            0x3f => f.write_str("VAL63"),
            0x40 => f.write_str("VAL64"),
            0x41 => f.write_str("VAL65"),
            0x42 => f.write_str("VAL66"),
            0x43 => f.write_str("VAL67"),
            0x44 => f.write_str("VAL68"),
            0x45 => f.write_str("VAL69"),
            0x46 => f.write_str("VAL70"),
            0x4b => f.write_str("VAL75"),
            0x4c => f.write_str("VAL76"),
            0x4d => f.write_str("VAL77"),
            0x4e => f.write_str("VAL78"),
            0x4f => f.write_str("VAL79"),
            0x50 => f.write_str("VAL80"),
            0x51 => f.write_str("VAL81"),
            0x52 => f.write_str("VAL82"),
            0x53 => f.write_str("VAL83"),
            0x54 => f.write_str("VAL84"),
            0x5e => f.write_str("VAL94"),
            0x5f => f.write_str("VAL95"),
            0x60 => f.write_str("VAL96"),
            0x61 => f.write_str("VAL97"),
            0x62 => f.write_str("VAL98"),
            0x63 => f.write_str("VAL99"),
            0x64 => f.write_str("VAL100"),
            0x71 => f.write_str("VAL113"),
            0x72 => f.write_str("VAL114"),
            0x73 => f.write_str("VAL115"),
            0x74 => f.write_str("VAL116"),
            0x75 => f.write_str("VAL117"),
            0x76 => f.write_str("VAL118"),
            0x77 => f.write_str("VAL119"),
            0x78 => f.write_str("VAL120"),
            0x79 => f.write_str("VAL121"),
            0x7a => f.write_str("VAL122"),
            0x7b => f.write_str("VAL123"),
            0x7c => f.write_str("VAL124"),
            0x7d => f.write_str("VAL125"),
            0x7e => f.write_str("VAL126"),
            0x7f => f.write_str("VAL127"),
            0x80 => f.write_str("VAL128"),
            0x81 => f.write_str("VAL129"),
            0x82 => f.write_str("VAL130"),
            0x83 => f.write_str("VAL131"),
            0x84 => f.write_str("VAL132"),
            0x85 => f.write_str("VAL133"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer0trigInp {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "VAL1"),
            0x02 => defmt::write!(f, "VAL2"),
            0x03 => defmt::write!(f, "VAL3"),
            0x04 => defmt::write!(f, "VAL4"),
            0x05 => defmt::write!(f, "VAL5"),
            0x06 => defmt::write!(f, "VAL6"),
            0x07 => defmt::write!(f, "VAL7"),
            0x08 => defmt::write!(f, "VAL8"),
            0x09 => defmt::write!(f, "VAL9"),
            0x0a => defmt::write!(f, "VAL10"),
            0x0b => defmt::write!(f, "VAL11"),
            0x0c => defmt::write!(f, "VAL12"),
            0x0d => defmt::write!(f, "VAL13"),
            0x0e => defmt::write!(f, "VAL14"),
            0x0f => defmt::write!(f, "VAL15"),
            0x10 => defmt::write!(f, "VAL16"),
            0x11 => defmt::write!(f, "VAL17"),
            0x12 => defmt::write!(f, "VAL18"),
            0x13 => defmt::write!(f, "VAL19"),
            0x14 => defmt::write!(f, "VAL20"),
            0x16 => defmt::write!(f, "VAL22"),
            0x17 => defmt::write!(f, "VAL23"),
            0x18 => defmt::write!(f, "VAL24"),
            0x19 => defmt::write!(f, "VAL25"),
            0x1a => defmt::write!(f, "VAL26"),
            0x1b => defmt::write!(f, "VAL27"),
            0x1c => defmt::write!(f, "VAL28"),
            0x1d => defmt::write!(f, "VAL29"),
            0x1e => defmt::write!(f, "VAL30"),
            0x21 => defmt::write!(f, "VAL33"),
            0x22 => defmt::write!(f, "VAL34"),
            0x23 => defmt::write!(f, "VAL35"),
            0x24 => defmt::write!(f, "VAL36"),
            0x25 => defmt::write!(f, "VAL37"),
            0x26 => defmt::write!(f, "VAL38"),
            0x30 => defmt::write!(f, "VAL48"),
            0x31 => defmt::write!(f, "VAL49"),
            0x32 => defmt::write!(f, "VAL50"),
            0x33 => defmt::write!(f, "VAL51"),
            0x34 => defmt::write!(f, "VAL52"),
            0x35 => defmt::write!(f, "VAL53"),
            0x36 => defmt::write!(f, "VAL54"),
            0x37 => defmt::write!(f, "VAL55"),
            0x38 => defmt::write!(f, "VAL56"),
            0x39 => defmt::write!(f, "VAL57"),
            0x3a => defmt::write!(f, "VAL58"),
            0x3b => defmt::write!(f, "VAL59"),
            0x3c => defmt::write!(f, "VAL60"),
            0x3d => defmt::write!(f, "VAL61"),
            0x3e => defmt::write!(f, "VAL62"),
            0x3f => defmt::write!(f, "VAL63"),
            0x40 => defmt::write!(f, "VAL64"),
            0x41 => defmt::write!(f, "VAL65"),
            0x42 => defmt::write!(f, "VAL66"),
            0x43 => defmt::write!(f, "VAL67"),
            0x44 => defmt::write!(f, "VAL68"),
            0x45 => defmt::write!(f, "VAL69"),
            0x46 => defmt::write!(f, "VAL70"),
            0x4b => defmt::write!(f, "VAL75"),
            0x4c => defmt::write!(f, "VAL76"),
            0x4d => defmt::write!(f, "VAL77"),
            0x4e => defmt::write!(f, "VAL78"),
            0x4f => defmt::write!(f, "VAL79"),
            0x50 => defmt::write!(f, "VAL80"),
            0x51 => defmt::write!(f, "VAL81"),
            0x52 => defmt::write!(f, "VAL82"),
            0x53 => defmt::write!(f, "VAL83"),
            0x54 => defmt::write!(f, "VAL84"),
            0x5e => defmt::write!(f, "VAL94"),
            0x5f => defmt::write!(f, "VAL95"),
            0x60 => defmt::write!(f, "VAL96"),
            0x61 => defmt::write!(f, "VAL97"),
            0x62 => defmt::write!(f, "VAL98"),
            0x63 => defmt::write!(f, "VAL99"),
            0x64 => defmt::write!(f, "VAL100"),
            0x71 => defmt::write!(f, "VAL113"),
            0x72 => defmt::write!(f, "VAL114"),
            0x73 => defmt::write!(f, "VAL115"),
            0x74 => defmt::write!(f, "VAL116"),
            0x75 => defmt::write!(f, "VAL117"),
            0x76 => defmt::write!(f, "VAL118"),
            0x77 => defmt::write!(f, "VAL119"),
            0x78 => defmt::write!(f, "VAL120"),
            0x79 => defmt::write!(f, "VAL121"),
            0x7a => defmt::write!(f, "VAL122"),
            0x7b => defmt::write!(f, "VAL123"),
            0x7c => defmt::write!(f, "VAL124"),
            0x7d => defmt::write!(f, "VAL125"),
            0x7e => defmt::write!(f, "VAL126"),
            0x7f => defmt::write!(f, "VAL127"),
            0x80 => defmt::write!(f, "VAL128"),
            0x81 => defmt::write!(f, "VAL129"),
            0x82 => defmt::write!(f, "VAL130"),
            0x83 => defmt::write!(f, "VAL131"),
            0x84 => defmt::write!(f, "VAL132"),
            0x85 => defmt::write!(f, "VAL133"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Timer0trigInp {
    #[inline(always)]
    fn from(val: u8) -> Timer0trigInp {
        Timer0trigInp::from_bits(val)
    }
}
impl From<Timer0trigInp> for u8 {
    #[inline(always)]
    fn from(val: Timer0trigInp) -> u8 {
        Timer0trigInp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Timer1trigInp(u8);
impl Timer1trigInp {
    #[doc = "CT_INP0 input is selected"]
    pub const VAL1: Self = Self(0x01);
    #[doc = "CT_INP1 input is selected"]
    pub const VAL2: Self = Self(0x02);
    #[doc = "CT_INP2 input is selected"]
    pub const VAL3: Self = Self(0x03);
    #[doc = "CT_INP3 input is selected"]
    pub const VAL4: Self = Self(0x04);
    #[doc = "CT_INP4 input is selected"]
    pub const VAL5: Self = Self(0x05);
    #[doc = "CT_INP5 input is selected"]
    pub const VAL6: Self = Self(0x06);
    #[doc = "CT_INP6 input is selected"]
    pub const VAL7: Self = Self(0x07);
    #[doc = "CT_INP7 input is selected"]
    pub const VAL8: Self = Self(0x08);
    #[doc = "CT_INP8 input is selected"]
    pub const VAL9: Self = Self(0x09);
    #[doc = "CT_INP9 input is selected"]
    pub const VAL10: Self = Self(0x0a);
    #[doc = "CT_INP10 input is selected"]
    pub const VAL11: Self = Self(0x0b);
    #[doc = "CT_INP11 input is selected"]
    pub const VAL12: Self = Self(0x0c);
    #[doc = "CT_INP12 input is selected"]
    pub const VAL13: Self = Self(0x0d);
    #[doc = "CT_INP13 input is selected"]
    pub const VAL14: Self = Self(0x0e);
    #[doc = "CT_INP14 input is selected"]
    pub const VAL15: Self = Self(0x0f);
    #[doc = "CT_INP15 input is selected"]
    pub const VAL16: Self = Self(0x10);
    #[doc = "CT_INP16 input is selected"]
    pub const VAL17: Self = Self(0x11);
    #[doc = "CT_INP17 input is selected"]
    pub const VAL18: Self = Self(0x12);
    #[doc = "CT_INP18 input is selected"]
    pub const VAL19: Self = Self(0x13);
    #[doc = "CT_INP19 input is selected"]
    pub const VAL20: Self = Self(0x14);
    #[doc = "AOI0_OUT0 input is selected"]
    pub const VAL22: Self = Self(0x16);
    #[doc = "AOI0_OUT1 input is selected"]
    pub const VAL23: Self = Self(0x17);
    #[doc = "AOI0_OUT2 input is selected"]
    pub const VAL24: Self = Self(0x18);
    #[doc = "AOI0_OUT3 input is selected"]
    pub const VAL25: Self = Self(0x19);
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    pub const VAL26: Self = Self(0x1a);
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    pub const VAL27: Self = Self(0x1b);
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    pub const VAL28: Self = Self(0x1c);
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    pub const VAL29: Self = Self(0x1d);
    #[doc = "CMP0_OUT input is selected"]
    pub const VAL30: Self = Self(0x1e);
    #[doc = "CTimer0_MAT1 input is selected"]
    pub const VAL33: Self = Self(0x21);
    #[doc = "CTimer0_MAT2 input is selected"]
    pub const VAL34: Self = Self(0x22);
    #[doc = "CTimer0_MAT3 input is selected"]
    pub const VAL35: Self = Self(0x23);
    #[doc = "CTimer2_MAT1 input is selected"]
    pub const VAL36: Self = Self(0x24);
    #[doc = "CTimer2_MAT2 input is selected"]
    pub const VAL37: Self = Self(0x25);
    #[doc = "CTimer2_MAT3 input is selected"]
    pub const VAL38: Self = Self(0x26);
    #[doc = "LPI2C0 Master End of Packet input is selected"]
    pub const VAL48: Self = Self(0x30);
    #[doc = "LPI2C0 Slave End of Packet input is selected"]
    pub const VAL49: Self = Self(0x31);
    #[doc = "LPI2C1 Master End of Packet input is selected"]
    pub const VAL50: Self = Self(0x32);
    #[doc = "LPI2C1 Slave End of Packet input is selected"]
    pub const VAL51: Self = Self(0x33);
    #[doc = "LPSPI0 End of Frame input is selected"]
    pub const VAL52: Self = Self(0x34);
    #[doc = "LPSPI0 Received Data Word input is selected"]
    pub const VAL53: Self = Self(0x35);
    #[doc = "LPSPI1 End of Frame input is selected"]
    pub const VAL54: Self = Self(0x36);
    #[doc = "LPSPI1 Received Data Word input is selected"]
    pub const VAL55: Self = Self(0x37);
    #[doc = "LPUART0 Received Data Word input is selected"]
    pub const VAL56: Self = Self(0x38);
    #[doc = "LPUART0 Transmitted Data Word input is selected"]
    pub const VAL57: Self = Self(0x39);
    #[doc = "LPUART0 Receive Line Idle input is selected"]
    pub const VAL58: Self = Self(0x3a);
    #[doc = "LPUART1 Received Data Word input is selected"]
    pub const VAL59: Self = Self(0x3b);
    #[doc = "LPUART1 Transmitted Data Word input is selected"]
    pub const VAL60: Self = Self(0x3c);
    #[doc = "LPUART1 Receive Line Idle input is selected"]
    pub const VAL61: Self = Self(0x3d);
    #[doc = "LPUART2 Received Data Word input is selected"]
    pub const VAL62: Self = Self(0x3e);
    #[doc = "LPUART2 Transmitted Data Word input is selected"]
    pub const VAL63: Self = Self(0x3f);
    #[doc = "LPUART2 Receive Line Idle input is selected"]
    pub const VAL64: Self = Self(0x40);
    #[doc = "LPUART3 Received Data Word input is selected"]
    pub const VAL65: Self = Self(0x41);
    #[doc = "LPUART3 Transmitted Data Word input is selected"]
    pub const VAL66: Self = Self(0x42);
    #[doc = "LPUART3 Receive Line Idle input is selected"]
    pub const VAL67: Self = Self(0x43);
    #[doc = "LPUART4 Received Data Word input is selected"]
    pub const VAL68: Self = Self(0x44);
    #[doc = "LPUART4 Transmitted Data Word input is selected"]
    pub const VAL69: Self = Self(0x45);
    #[doc = "LPUART4 Receive Line Idle input is selected"]
    pub const VAL70: Self = Self(0x46);
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    pub const VAL75: Self = Self(0x4b);
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    pub const VAL76: Self = Self(0x4c);
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    pub const VAL77: Self = Self(0x4d);
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    pub const VAL78: Self = Self(0x4e);
    #[doc = "CTimer3_MAT1 is selected"]
    pub const VAL79: Self = Self(0x4f);
    #[doc = "CTimer3_MAT2 input is selected"]
    pub const VAL80: Self = Self(0x50);
    #[doc = "CTimer3_MAT3 input is selected"]
    pub const VAL81: Self = Self(0x51);
    #[doc = "CTimer4_MAT1 input is selected"]
    pub const VAL82: Self = Self(0x52);
    #[doc = "CTimer4_MAT2 input is selected"]
    pub const VAL83: Self = Self(0x53);
    #[doc = "CTimer4_MAT3 input is selected"]
    pub const VAL84: Self = Self(0x54);
    #[doc = "LPI2C2 Master End of Packet is selected"]
    pub const VAL94: Self = Self(0x5e);
    #[doc = "LPI2C2 Slave End of Packet input is selected"]
    pub const VAL95: Self = Self(0x5f);
    #[doc = "LPI2C3 Master End of Packet input is selected"]
    pub const VAL96: Self = Self(0x60);
    #[doc = "LPI2C3 Slave End of Packet input is selected"]
    pub const VAL97: Self = Self(0x61);
    #[doc = "LPUART5 Received Data Word input is selected"]
    pub const VAL98: Self = Self(0x62);
    #[doc = "LPUART5 Transmitted Data Word input is selected"]
    pub const VAL99: Self = Self(0x63);
    #[doc = "LPUART5 Receive Line Idle input is selected"]
    pub const VAL100: Self = Self(0x64);
    #[doc = "TRIG_IN0 input is selected"]
    pub const VAL113: Self = Self(0x71);
    #[doc = "TRIG_IN1 input is selected"]
    pub const VAL114: Self = Self(0x72);
    #[doc = "TRIG_IN2 input is selected"]
    pub const VAL115: Self = Self(0x73);
    #[doc = "TRIG_IN3 input is selected"]
    pub const VAL116: Self = Self(0x74);
    #[doc = "TRIG_IN4 input is selected"]
    pub const VAL117: Self = Self(0x75);
    #[doc = "TRIG_IN5 input is selected"]
    pub const VAL118: Self = Self(0x76);
    #[doc = "TRIG_IN6 input is selected"]
    pub const VAL119: Self = Self(0x77);
    #[doc = "TRIG_IN7 input is selected"]
    pub const VAL120: Self = Self(0x78);
    #[doc = "TRIG_IN8 input is selected"]
    pub const VAL121: Self = Self(0x79);
    #[doc = "TRIG_IN9 input is selected"]
    pub const VAL122: Self = Self(0x7a);
    #[doc = "TRIG_IN10 input is selected"]
    pub const VAL123: Self = Self(0x7b);
    #[doc = "TRIG_IN11 input is selected"]
    pub const VAL124: Self = Self(0x7c);
    #[doc = "USB1 Start of Frame input is selected"]
    pub const VAL125: Self = Self(0x7d);
    #[doc = "LPSPI2 End of Frame input is selected"]
    pub const VAL126: Self = Self(0x7e);
    #[doc = "LPSPI2 Received Data Word input is selected"]
    pub const VAL127: Self = Self(0x7f);
    #[doc = "LPSPI3 End of Frame input is selected"]
    pub const VAL128: Self = Self(0x80);
    #[doc = "LPSPI3 Received Data Word input is selected"]
    pub const VAL129: Self = Self(0x81);
    #[doc = "LPSPI4 End of Frame input is selected"]
    pub const VAL130: Self = Self(0x82);
    #[doc = "LPSPI4 Received Data Word input is selected"]
    pub const VAL131: Self = Self(0x83);
    #[doc = "LPSPI5 End of Frame input is selected"]
    pub const VAL132: Self = Self(0x84);
    #[doc = "LPSPI5 Received Data Word input is selected"]
    pub const VAL133: Self = Self(0x85);
}
impl Timer1trigInp {
    pub const fn from_bits(val: u8) -> Timer1trigInp {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Timer1trigInp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("VAL1"),
            0x02 => f.write_str("VAL2"),
            0x03 => f.write_str("VAL3"),
            0x04 => f.write_str("VAL4"),
            0x05 => f.write_str("VAL5"),
            0x06 => f.write_str("VAL6"),
            0x07 => f.write_str("VAL7"),
            0x08 => f.write_str("VAL8"),
            0x09 => f.write_str("VAL9"),
            0x0a => f.write_str("VAL10"),
            0x0b => f.write_str("VAL11"),
            0x0c => f.write_str("VAL12"),
            0x0d => f.write_str("VAL13"),
            0x0e => f.write_str("VAL14"),
            0x0f => f.write_str("VAL15"),
            0x10 => f.write_str("VAL16"),
            0x11 => f.write_str("VAL17"),
            0x12 => f.write_str("VAL18"),
            0x13 => f.write_str("VAL19"),
            0x14 => f.write_str("VAL20"),
            0x16 => f.write_str("VAL22"),
            0x17 => f.write_str("VAL23"),
            0x18 => f.write_str("VAL24"),
            0x19 => f.write_str("VAL25"),
            0x1a => f.write_str("VAL26"),
            0x1b => f.write_str("VAL27"),
            0x1c => f.write_str("VAL28"),
            0x1d => f.write_str("VAL29"),
            0x1e => f.write_str("VAL30"),
            0x21 => f.write_str("VAL33"),
            0x22 => f.write_str("VAL34"),
            0x23 => f.write_str("VAL35"),
            0x24 => f.write_str("VAL36"),
            0x25 => f.write_str("VAL37"),
            0x26 => f.write_str("VAL38"),
            0x30 => f.write_str("VAL48"),
            0x31 => f.write_str("VAL49"),
            0x32 => f.write_str("VAL50"),
            0x33 => f.write_str("VAL51"),
            0x34 => f.write_str("VAL52"),
            0x35 => f.write_str("VAL53"),
            0x36 => f.write_str("VAL54"),
            0x37 => f.write_str("VAL55"),
            0x38 => f.write_str("VAL56"),
            0x39 => f.write_str("VAL57"),
            0x3a => f.write_str("VAL58"),
            0x3b => f.write_str("VAL59"),
            0x3c => f.write_str("VAL60"),
            0x3d => f.write_str("VAL61"),
            0x3e => f.write_str("VAL62"),
            0x3f => f.write_str("VAL63"),
            0x40 => f.write_str("VAL64"),
            0x41 => f.write_str("VAL65"),
            0x42 => f.write_str("VAL66"),
            0x43 => f.write_str("VAL67"),
            0x44 => f.write_str("VAL68"),
            0x45 => f.write_str("VAL69"),
            0x46 => f.write_str("VAL70"),
            0x4b => f.write_str("VAL75"),
            0x4c => f.write_str("VAL76"),
            0x4d => f.write_str("VAL77"),
            0x4e => f.write_str("VAL78"),
            0x4f => f.write_str("VAL79"),
            0x50 => f.write_str("VAL80"),
            0x51 => f.write_str("VAL81"),
            0x52 => f.write_str("VAL82"),
            0x53 => f.write_str("VAL83"),
            0x54 => f.write_str("VAL84"),
            0x5e => f.write_str("VAL94"),
            0x5f => f.write_str("VAL95"),
            0x60 => f.write_str("VAL96"),
            0x61 => f.write_str("VAL97"),
            0x62 => f.write_str("VAL98"),
            0x63 => f.write_str("VAL99"),
            0x64 => f.write_str("VAL100"),
            0x71 => f.write_str("VAL113"),
            0x72 => f.write_str("VAL114"),
            0x73 => f.write_str("VAL115"),
            0x74 => f.write_str("VAL116"),
            0x75 => f.write_str("VAL117"),
            0x76 => f.write_str("VAL118"),
            0x77 => f.write_str("VAL119"),
            0x78 => f.write_str("VAL120"),
            0x79 => f.write_str("VAL121"),
            0x7a => f.write_str("VAL122"),
            0x7b => f.write_str("VAL123"),
            0x7c => f.write_str("VAL124"),
            0x7d => f.write_str("VAL125"),
            0x7e => f.write_str("VAL126"),
            0x7f => f.write_str("VAL127"),
            0x80 => f.write_str("VAL128"),
            0x81 => f.write_str("VAL129"),
            0x82 => f.write_str("VAL130"),
            0x83 => f.write_str("VAL131"),
            0x84 => f.write_str("VAL132"),
            0x85 => f.write_str("VAL133"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer1trigInp {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "VAL1"),
            0x02 => defmt::write!(f, "VAL2"),
            0x03 => defmt::write!(f, "VAL3"),
            0x04 => defmt::write!(f, "VAL4"),
            0x05 => defmt::write!(f, "VAL5"),
            0x06 => defmt::write!(f, "VAL6"),
            0x07 => defmt::write!(f, "VAL7"),
            0x08 => defmt::write!(f, "VAL8"),
            0x09 => defmt::write!(f, "VAL9"),
            0x0a => defmt::write!(f, "VAL10"),
            0x0b => defmt::write!(f, "VAL11"),
            0x0c => defmt::write!(f, "VAL12"),
            0x0d => defmt::write!(f, "VAL13"),
            0x0e => defmt::write!(f, "VAL14"),
            0x0f => defmt::write!(f, "VAL15"),
            0x10 => defmt::write!(f, "VAL16"),
            0x11 => defmt::write!(f, "VAL17"),
            0x12 => defmt::write!(f, "VAL18"),
            0x13 => defmt::write!(f, "VAL19"),
            0x14 => defmt::write!(f, "VAL20"),
            0x16 => defmt::write!(f, "VAL22"),
            0x17 => defmt::write!(f, "VAL23"),
            0x18 => defmt::write!(f, "VAL24"),
            0x19 => defmt::write!(f, "VAL25"),
            0x1a => defmt::write!(f, "VAL26"),
            0x1b => defmt::write!(f, "VAL27"),
            0x1c => defmt::write!(f, "VAL28"),
            0x1d => defmt::write!(f, "VAL29"),
            0x1e => defmt::write!(f, "VAL30"),
            0x21 => defmt::write!(f, "VAL33"),
            0x22 => defmt::write!(f, "VAL34"),
            0x23 => defmt::write!(f, "VAL35"),
            0x24 => defmt::write!(f, "VAL36"),
            0x25 => defmt::write!(f, "VAL37"),
            0x26 => defmt::write!(f, "VAL38"),
            0x30 => defmt::write!(f, "VAL48"),
            0x31 => defmt::write!(f, "VAL49"),
            0x32 => defmt::write!(f, "VAL50"),
            0x33 => defmt::write!(f, "VAL51"),
            0x34 => defmt::write!(f, "VAL52"),
            0x35 => defmt::write!(f, "VAL53"),
            0x36 => defmt::write!(f, "VAL54"),
            0x37 => defmt::write!(f, "VAL55"),
            0x38 => defmt::write!(f, "VAL56"),
            0x39 => defmt::write!(f, "VAL57"),
            0x3a => defmt::write!(f, "VAL58"),
            0x3b => defmt::write!(f, "VAL59"),
            0x3c => defmt::write!(f, "VAL60"),
            0x3d => defmt::write!(f, "VAL61"),
            0x3e => defmt::write!(f, "VAL62"),
            0x3f => defmt::write!(f, "VAL63"),
            0x40 => defmt::write!(f, "VAL64"),
            0x41 => defmt::write!(f, "VAL65"),
            0x42 => defmt::write!(f, "VAL66"),
            0x43 => defmt::write!(f, "VAL67"),
            0x44 => defmt::write!(f, "VAL68"),
            0x45 => defmt::write!(f, "VAL69"),
            0x46 => defmt::write!(f, "VAL70"),
            0x4b => defmt::write!(f, "VAL75"),
            0x4c => defmt::write!(f, "VAL76"),
            0x4d => defmt::write!(f, "VAL77"),
            0x4e => defmt::write!(f, "VAL78"),
            0x4f => defmt::write!(f, "VAL79"),
            0x50 => defmt::write!(f, "VAL80"),
            0x51 => defmt::write!(f, "VAL81"),
            0x52 => defmt::write!(f, "VAL82"),
            0x53 => defmt::write!(f, "VAL83"),
            0x54 => defmt::write!(f, "VAL84"),
            0x5e => defmt::write!(f, "VAL94"),
            0x5f => defmt::write!(f, "VAL95"),
            0x60 => defmt::write!(f, "VAL96"),
            0x61 => defmt::write!(f, "VAL97"),
            0x62 => defmt::write!(f, "VAL98"),
            0x63 => defmt::write!(f, "VAL99"),
            0x64 => defmt::write!(f, "VAL100"),
            0x71 => defmt::write!(f, "VAL113"),
            0x72 => defmt::write!(f, "VAL114"),
            0x73 => defmt::write!(f, "VAL115"),
            0x74 => defmt::write!(f, "VAL116"),
            0x75 => defmt::write!(f, "VAL117"),
            0x76 => defmt::write!(f, "VAL118"),
            0x77 => defmt::write!(f, "VAL119"),
            0x78 => defmt::write!(f, "VAL120"),
            0x79 => defmt::write!(f, "VAL121"),
            0x7a => defmt::write!(f, "VAL122"),
            0x7b => defmt::write!(f, "VAL123"),
            0x7c => defmt::write!(f, "VAL124"),
            0x7d => defmt::write!(f, "VAL125"),
            0x7e => defmt::write!(f, "VAL126"),
            0x7f => defmt::write!(f, "VAL127"),
            0x80 => defmt::write!(f, "VAL128"),
            0x81 => defmt::write!(f, "VAL129"),
            0x82 => defmt::write!(f, "VAL130"),
            0x83 => defmt::write!(f, "VAL131"),
            0x84 => defmt::write!(f, "VAL132"),
            0x85 => defmt::write!(f, "VAL133"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Timer1trigInp {
    #[inline(always)]
    fn from(val: u8) -> Timer1trigInp {
        Timer1trigInp::from_bits(val)
    }
}
impl From<Timer1trigInp> for u8 {
    #[inline(always)]
    fn from(val: Timer1trigInp) -> u8 {
        Timer1trigInp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Timer2trigInp(u8);
impl Timer2trigInp {
    #[doc = "CT_INP0 input is selected"]
    pub const VAL1: Self = Self(0x01);
    #[doc = "CT_INP1 input is selected"]
    pub const VAL2: Self = Self(0x02);
    #[doc = "CT_INP2 input is selected"]
    pub const VAL3: Self = Self(0x03);
    #[doc = "CT_INP3 input is selected"]
    pub const VAL4: Self = Self(0x04);
    #[doc = "CT_INP4 input is selected"]
    pub const VAL5: Self = Self(0x05);
    #[doc = "CT_INP5 input is selected"]
    pub const VAL6: Self = Self(0x06);
    #[doc = "CT_INP6 input is selected"]
    pub const VAL7: Self = Self(0x07);
    #[doc = "CT_INP7 input is selected"]
    pub const VAL8: Self = Self(0x08);
    #[doc = "CT_INP8 input is selected"]
    pub const VAL9: Self = Self(0x09);
    #[doc = "CT_INP9 input is selected"]
    pub const VAL10: Self = Self(0x0a);
    #[doc = "CT_INP10 input is selected"]
    pub const VAL11: Self = Self(0x0b);
    #[doc = "CT_INP11 input is selected"]
    pub const VAL12: Self = Self(0x0c);
    #[doc = "CT_INP12 input is selected"]
    pub const VAL13: Self = Self(0x0d);
    #[doc = "CT_INP13 input is selected"]
    pub const VAL14: Self = Self(0x0e);
    #[doc = "CT_INP14 input is selected"]
    pub const VAL15: Self = Self(0x0f);
    #[doc = "CT_INP15 input is selected"]
    pub const VAL16: Self = Self(0x10);
    #[doc = "CT_INP16 input is selected"]
    pub const VAL17: Self = Self(0x11);
    #[doc = "CT_INP17 input is selected"]
    pub const VAL18: Self = Self(0x12);
    #[doc = "CT_INP18 input is selected"]
    pub const VAL19: Self = Self(0x13);
    #[doc = "CT_INP19 input is selected"]
    pub const VAL20: Self = Self(0x14);
    #[doc = "AOI0_OUT0 input is selected"]
    pub const VAL22: Self = Self(0x16);
    #[doc = "AOI0_OUT1 input is selected"]
    pub const VAL23: Self = Self(0x17);
    #[doc = "AOI0_OUT2 input is selected"]
    pub const VAL24: Self = Self(0x18);
    #[doc = "AOI0_OUT3 input is selected"]
    pub const VAL25: Self = Self(0x19);
    #[doc = "ADC0_tcomp\\[0\\]"]
    pub const VAL26: Self = Self(0x1a);
    #[doc = "ADC0_tcomp\\[1\\]"]
    pub const VAL27: Self = Self(0x1b);
    #[doc = "ADC0_tcomp\\[2\\]"]
    pub const VAL28: Self = Self(0x1c);
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    pub const VAL29: Self = Self(0x1d);
    #[doc = "CMP0_OUT is selected"]
    pub const VAL30: Self = Self(0x1e);
    #[doc = "CTimer0_MAT1 input is selected"]
    pub const VAL33: Self = Self(0x21);
    #[doc = "CTimer0_MAT2 input is selected"]
    pub const VAL34: Self = Self(0x22);
    #[doc = "CTimer0_MAT3 input is selected"]
    pub const VAL35: Self = Self(0x23);
    #[doc = "CTimer1_MAT1 input is selected"]
    pub const VAL36: Self = Self(0x24);
    #[doc = "CTimer1_MAT2 input is selected"]
    pub const VAL37: Self = Self(0x25);
    #[doc = "CTimer1_MAT3 input is selected"]
    pub const VAL38: Self = Self(0x26);
    #[doc = "LPI2C0 Master End of Packet input is selected"]
    pub const VAL48: Self = Self(0x30);
    #[doc = "LPI2C0 Slave End of Packet input is selected"]
    pub const VAL49: Self = Self(0x31);
    #[doc = "LPI2C1 Master End of Packet input is selected"]
    pub const VAL50: Self = Self(0x32);
    #[doc = "LPI2C1 Slave End of Packet input is selected"]
    pub const VAL51: Self = Self(0x33);
    #[doc = "LPSPI0 End of Frame input is selected"]
    pub const VAL52: Self = Self(0x34);
    #[doc = "LPSPI0 Received Data Word input is selected"]
    pub const VAL53: Self = Self(0x35);
    #[doc = "LPSPI1 End of Frame input is selected"]
    pub const VAL54: Self = Self(0x36);
    #[doc = "LPSPI1 Received Data Word input is selected"]
    pub const VAL55: Self = Self(0x37);
    #[doc = "LPUART0 Received Data Word input is selected"]
    pub const VAL56: Self = Self(0x38);
    #[doc = "LPUART0 Transmitted Data Word input is selected"]
    pub const VAL57: Self = Self(0x39);
    #[doc = "LPUART0 Receive Line Idle input is selected"]
    pub const VAL58: Self = Self(0x3a);
    #[doc = "LPUART1 Received Data Word input is selected"]
    pub const VAL59: Self = Self(0x3b);
    #[doc = "LPUART1 Transmitted Data Word input is selected"]
    pub const VAL60: Self = Self(0x3c);
    #[doc = "LPUART1 Receive Line Idle input is selected"]
    pub const VAL61: Self = Self(0x3d);
    #[doc = "LPUART2 Received Data Word input is selected"]
    pub const VAL62: Self = Self(0x3e);
    #[doc = "LPUART2 Transmitted Data Word input is selected"]
    pub const VAL63: Self = Self(0x3f);
    #[doc = "LPUART2 Receive Line Idle input is selected"]
    pub const VAL64: Self = Self(0x40);
    #[doc = "LPUART3 Received Data Word input is selected"]
    pub const VAL65: Self = Self(0x41);
    #[doc = "LPUART3 Transmitted Data Word input is selected"]
    pub const VAL66: Self = Self(0x42);
    #[doc = "LPUART3 Receive Line Idle input is selected"]
    pub const VAL67: Self = Self(0x43);
    #[doc = "LPUART4 Received Data Word input is selected"]
    pub const VAL68: Self = Self(0x44);
    #[doc = "LPUART4 Transmitted Data Word input is selected"]
    pub const VAL69: Self = Self(0x45);
    #[doc = "LPUART4 Receive Line Idle input is selected"]
    pub const VAL70: Self = Self(0x46);
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    pub const VAL75: Self = Self(0x4b);
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    pub const VAL76: Self = Self(0x4c);
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    pub const VAL77: Self = Self(0x4d);
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    pub const VAL78: Self = Self(0x4e);
    #[doc = "CTimer3_MAT1 input is selected"]
    pub const VAL79: Self = Self(0x4f);
    #[doc = "CTimer3_MAT2 is selected"]
    pub const VAL80: Self = Self(0x50);
    #[doc = "CTimer3_MAT3 input is selected"]
    pub const VAL81: Self = Self(0x51);
    #[doc = "CTimer4_MAT1 input is selected"]
    pub const VAL82: Self = Self(0x52);
    #[doc = "CTimer4_MAT2 input is selected"]
    pub const VAL83: Self = Self(0x53);
    #[doc = "CTimer4_MAT3 input is selected"]
    pub const VAL84: Self = Self(0x54);
    #[doc = "LPI2C2 Master End of Packet input is selected"]
    pub const VAL94: Self = Self(0x5e);
    #[doc = "LPI2C2 Slave End of Packet input is selected"]
    pub const VAL95: Self = Self(0x5f);
    #[doc = "LPI2C3 Master End of Packet input is selected"]
    pub const VAL96: Self = Self(0x60);
    #[doc = "LPI2C3 Slave End of Packet input is selected"]
    pub const VAL97: Self = Self(0x61);
    #[doc = "LPUART5 Received Data Word input is selected"]
    pub const VAL98: Self = Self(0x62);
    #[doc = "LPUART5 Transmitted Data Word input is selected"]
    pub const VAL99: Self = Self(0x63);
    #[doc = "LPUART5 Receive Line Idle input is selected"]
    pub const VAL100: Self = Self(0x64);
    #[doc = "TRIG_IN0 input is selected"]
    pub const VAL113: Self = Self(0x71);
    #[doc = "TRIG_IN1 input is selected"]
    pub const VAL114: Self = Self(0x72);
    #[doc = "TRIG_IN2 input is selected"]
    pub const VAL115: Self = Self(0x73);
    #[doc = "TRIG_IN3 input is selected"]
    pub const VAL116: Self = Self(0x74);
    #[doc = "TRIG_IN4 input is selected"]
    pub const VAL117: Self = Self(0x75);
    #[doc = "TRIG_IN5 input is selected"]
    pub const VAL118: Self = Self(0x76);
    #[doc = "TRIG_IN6 input is selected"]
    pub const VAL119: Self = Self(0x77);
    #[doc = "TRIG_IN7 input is selected"]
    pub const VAL120: Self = Self(0x78);
    #[doc = "TRIG_IN8 input is selected"]
    pub const VAL121: Self = Self(0x79);
    #[doc = "TRIG_IN9 input is selected"]
    pub const VAL122: Self = Self(0x7a);
    #[doc = "TRIG_IN10 input is selected"]
    pub const VAL123: Self = Self(0x7b);
    #[doc = "TRIG_IN11 input is selected"]
    pub const VAL124: Self = Self(0x7c);
    #[doc = "USB1 Start of Frame input is selected"]
    pub const VAL125: Self = Self(0x7d);
    #[doc = "LPSPI2 End of Frame is selected"]
    pub const VAL126: Self = Self(0x7e);
    #[doc = "LPSPI2 Received Data Word input is selected"]
    pub const VAL127: Self = Self(0x7f);
    #[doc = "LPSPI3 End of Frame input is selected"]
    pub const VAL128: Self = Self(0x80);
    #[doc = "LPSPI3 Received Data Word input is selected"]
    pub const VAL129: Self = Self(0x81);
    #[doc = "LPSPI4 End of Frame input is selected"]
    pub const VAL130: Self = Self(0x82);
    #[doc = "LPSPI4 Received Data Word input is selected"]
    pub const VAL131: Self = Self(0x83);
    #[doc = "LPSPI5 End of Frame input is selected"]
    pub const VAL132: Self = Self(0x84);
    #[doc = "LPSPI5 Received Data Word input is selected"]
    pub const VAL133: Self = Self(0x85);
}
impl Timer2trigInp {
    pub const fn from_bits(val: u8) -> Timer2trigInp {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Timer2trigInp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("VAL1"),
            0x02 => f.write_str("VAL2"),
            0x03 => f.write_str("VAL3"),
            0x04 => f.write_str("VAL4"),
            0x05 => f.write_str("VAL5"),
            0x06 => f.write_str("VAL6"),
            0x07 => f.write_str("VAL7"),
            0x08 => f.write_str("VAL8"),
            0x09 => f.write_str("VAL9"),
            0x0a => f.write_str("VAL10"),
            0x0b => f.write_str("VAL11"),
            0x0c => f.write_str("VAL12"),
            0x0d => f.write_str("VAL13"),
            0x0e => f.write_str("VAL14"),
            0x0f => f.write_str("VAL15"),
            0x10 => f.write_str("VAL16"),
            0x11 => f.write_str("VAL17"),
            0x12 => f.write_str("VAL18"),
            0x13 => f.write_str("VAL19"),
            0x14 => f.write_str("VAL20"),
            0x16 => f.write_str("VAL22"),
            0x17 => f.write_str("VAL23"),
            0x18 => f.write_str("VAL24"),
            0x19 => f.write_str("VAL25"),
            0x1a => f.write_str("VAL26"),
            0x1b => f.write_str("VAL27"),
            0x1c => f.write_str("VAL28"),
            0x1d => f.write_str("VAL29"),
            0x1e => f.write_str("VAL30"),
            0x21 => f.write_str("VAL33"),
            0x22 => f.write_str("VAL34"),
            0x23 => f.write_str("VAL35"),
            0x24 => f.write_str("VAL36"),
            0x25 => f.write_str("VAL37"),
            0x26 => f.write_str("VAL38"),
            0x30 => f.write_str("VAL48"),
            0x31 => f.write_str("VAL49"),
            0x32 => f.write_str("VAL50"),
            0x33 => f.write_str("VAL51"),
            0x34 => f.write_str("VAL52"),
            0x35 => f.write_str("VAL53"),
            0x36 => f.write_str("VAL54"),
            0x37 => f.write_str("VAL55"),
            0x38 => f.write_str("VAL56"),
            0x39 => f.write_str("VAL57"),
            0x3a => f.write_str("VAL58"),
            0x3b => f.write_str("VAL59"),
            0x3c => f.write_str("VAL60"),
            0x3d => f.write_str("VAL61"),
            0x3e => f.write_str("VAL62"),
            0x3f => f.write_str("VAL63"),
            0x40 => f.write_str("VAL64"),
            0x41 => f.write_str("VAL65"),
            0x42 => f.write_str("VAL66"),
            0x43 => f.write_str("VAL67"),
            0x44 => f.write_str("VAL68"),
            0x45 => f.write_str("VAL69"),
            0x46 => f.write_str("VAL70"),
            0x4b => f.write_str("VAL75"),
            0x4c => f.write_str("VAL76"),
            0x4d => f.write_str("VAL77"),
            0x4e => f.write_str("VAL78"),
            0x4f => f.write_str("VAL79"),
            0x50 => f.write_str("VAL80"),
            0x51 => f.write_str("VAL81"),
            0x52 => f.write_str("VAL82"),
            0x53 => f.write_str("VAL83"),
            0x54 => f.write_str("VAL84"),
            0x5e => f.write_str("VAL94"),
            0x5f => f.write_str("VAL95"),
            0x60 => f.write_str("VAL96"),
            0x61 => f.write_str("VAL97"),
            0x62 => f.write_str("VAL98"),
            0x63 => f.write_str("VAL99"),
            0x64 => f.write_str("VAL100"),
            0x71 => f.write_str("VAL113"),
            0x72 => f.write_str("VAL114"),
            0x73 => f.write_str("VAL115"),
            0x74 => f.write_str("VAL116"),
            0x75 => f.write_str("VAL117"),
            0x76 => f.write_str("VAL118"),
            0x77 => f.write_str("VAL119"),
            0x78 => f.write_str("VAL120"),
            0x79 => f.write_str("VAL121"),
            0x7a => f.write_str("VAL122"),
            0x7b => f.write_str("VAL123"),
            0x7c => f.write_str("VAL124"),
            0x7d => f.write_str("VAL125"),
            0x7e => f.write_str("VAL126"),
            0x7f => f.write_str("VAL127"),
            0x80 => f.write_str("VAL128"),
            0x81 => f.write_str("VAL129"),
            0x82 => f.write_str("VAL130"),
            0x83 => f.write_str("VAL131"),
            0x84 => f.write_str("VAL132"),
            0x85 => f.write_str("VAL133"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer2trigInp {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "VAL1"),
            0x02 => defmt::write!(f, "VAL2"),
            0x03 => defmt::write!(f, "VAL3"),
            0x04 => defmt::write!(f, "VAL4"),
            0x05 => defmt::write!(f, "VAL5"),
            0x06 => defmt::write!(f, "VAL6"),
            0x07 => defmt::write!(f, "VAL7"),
            0x08 => defmt::write!(f, "VAL8"),
            0x09 => defmt::write!(f, "VAL9"),
            0x0a => defmt::write!(f, "VAL10"),
            0x0b => defmt::write!(f, "VAL11"),
            0x0c => defmt::write!(f, "VAL12"),
            0x0d => defmt::write!(f, "VAL13"),
            0x0e => defmt::write!(f, "VAL14"),
            0x0f => defmt::write!(f, "VAL15"),
            0x10 => defmt::write!(f, "VAL16"),
            0x11 => defmt::write!(f, "VAL17"),
            0x12 => defmt::write!(f, "VAL18"),
            0x13 => defmt::write!(f, "VAL19"),
            0x14 => defmt::write!(f, "VAL20"),
            0x16 => defmt::write!(f, "VAL22"),
            0x17 => defmt::write!(f, "VAL23"),
            0x18 => defmt::write!(f, "VAL24"),
            0x19 => defmt::write!(f, "VAL25"),
            0x1a => defmt::write!(f, "VAL26"),
            0x1b => defmt::write!(f, "VAL27"),
            0x1c => defmt::write!(f, "VAL28"),
            0x1d => defmt::write!(f, "VAL29"),
            0x1e => defmt::write!(f, "VAL30"),
            0x21 => defmt::write!(f, "VAL33"),
            0x22 => defmt::write!(f, "VAL34"),
            0x23 => defmt::write!(f, "VAL35"),
            0x24 => defmt::write!(f, "VAL36"),
            0x25 => defmt::write!(f, "VAL37"),
            0x26 => defmt::write!(f, "VAL38"),
            0x30 => defmt::write!(f, "VAL48"),
            0x31 => defmt::write!(f, "VAL49"),
            0x32 => defmt::write!(f, "VAL50"),
            0x33 => defmt::write!(f, "VAL51"),
            0x34 => defmt::write!(f, "VAL52"),
            0x35 => defmt::write!(f, "VAL53"),
            0x36 => defmt::write!(f, "VAL54"),
            0x37 => defmt::write!(f, "VAL55"),
            0x38 => defmt::write!(f, "VAL56"),
            0x39 => defmt::write!(f, "VAL57"),
            0x3a => defmt::write!(f, "VAL58"),
            0x3b => defmt::write!(f, "VAL59"),
            0x3c => defmt::write!(f, "VAL60"),
            0x3d => defmt::write!(f, "VAL61"),
            0x3e => defmt::write!(f, "VAL62"),
            0x3f => defmt::write!(f, "VAL63"),
            0x40 => defmt::write!(f, "VAL64"),
            0x41 => defmt::write!(f, "VAL65"),
            0x42 => defmt::write!(f, "VAL66"),
            0x43 => defmt::write!(f, "VAL67"),
            0x44 => defmt::write!(f, "VAL68"),
            0x45 => defmt::write!(f, "VAL69"),
            0x46 => defmt::write!(f, "VAL70"),
            0x4b => defmt::write!(f, "VAL75"),
            0x4c => defmt::write!(f, "VAL76"),
            0x4d => defmt::write!(f, "VAL77"),
            0x4e => defmt::write!(f, "VAL78"),
            0x4f => defmt::write!(f, "VAL79"),
            0x50 => defmt::write!(f, "VAL80"),
            0x51 => defmt::write!(f, "VAL81"),
            0x52 => defmt::write!(f, "VAL82"),
            0x53 => defmt::write!(f, "VAL83"),
            0x54 => defmt::write!(f, "VAL84"),
            0x5e => defmt::write!(f, "VAL94"),
            0x5f => defmt::write!(f, "VAL95"),
            0x60 => defmt::write!(f, "VAL96"),
            0x61 => defmt::write!(f, "VAL97"),
            0x62 => defmt::write!(f, "VAL98"),
            0x63 => defmt::write!(f, "VAL99"),
            0x64 => defmt::write!(f, "VAL100"),
            0x71 => defmt::write!(f, "VAL113"),
            0x72 => defmt::write!(f, "VAL114"),
            0x73 => defmt::write!(f, "VAL115"),
            0x74 => defmt::write!(f, "VAL116"),
            0x75 => defmt::write!(f, "VAL117"),
            0x76 => defmt::write!(f, "VAL118"),
            0x77 => defmt::write!(f, "VAL119"),
            0x78 => defmt::write!(f, "VAL120"),
            0x79 => defmt::write!(f, "VAL121"),
            0x7a => defmt::write!(f, "VAL122"),
            0x7b => defmt::write!(f, "VAL123"),
            0x7c => defmt::write!(f, "VAL124"),
            0x7d => defmt::write!(f, "VAL125"),
            0x7e => defmt::write!(f, "VAL126"),
            0x7f => defmt::write!(f, "VAL127"),
            0x80 => defmt::write!(f, "VAL128"),
            0x81 => defmt::write!(f, "VAL129"),
            0x82 => defmt::write!(f, "VAL130"),
            0x83 => defmt::write!(f, "VAL131"),
            0x84 => defmt::write!(f, "VAL132"),
            0x85 => defmt::write!(f, "VAL133"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Timer2trigInp {
    #[inline(always)]
    fn from(val: u8) -> Timer2trigInp {
        Timer2trigInp::from_bits(val)
    }
}
impl From<Timer2trigInp> for u8 {
    #[inline(always)]
    fn from(val: Timer2trigInp) -> u8 {
        Timer2trigInp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Timer3trigInp(u8);
impl Timer3trigInp {
    #[doc = "CT_INP0 input is selected"]
    pub const VAL1: Self = Self(0x01);
    #[doc = "CT_INP1 input is selected"]
    pub const VAL2: Self = Self(0x02);
    #[doc = "CT_INP2 input is selected"]
    pub const VAL3: Self = Self(0x03);
    #[doc = "CT_INP3 input is selected"]
    pub const VAL4: Self = Self(0x04);
    #[doc = "CT_INP4 input is selected"]
    pub const VAL5: Self = Self(0x05);
    #[doc = "CT_INP5 input is selected"]
    pub const VAL6: Self = Self(0x06);
    #[doc = "CT_INP6 input is selected"]
    pub const VAL7: Self = Self(0x07);
    #[doc = "CT_INP7 input is selected"]
    pub const VAL8: Self = Self(0x08);
    #[doc = "CT_INP8 input is selected"]
    pub const VAL9: Self = Self(0x09);
    #[doc = "CT_INP9 input is selected"]
    pub const VAL10: Self = Self(0x0a);
    #[doc = "CT_INP10 input is selected"]
    pub const VAL11: Self = Self(0x0b);
    #[doc = "CT_INP11 input is selected"]
    pub const VAL12: Self = Self(0x0c);
    #[doc = "CT_INP12 input is selected"]
    pub const VAL13: Self = Self(0x0d);
    #[doc = "CT_INP13 input is selected"]
    pub const VAL14: Self = Self(0x0e);
    #[doc = "CT_INP14 input is selected"]
    pub const VAL15: Self = Self(0x0f);
    #[doc = "CT_INP15 input is selected"]
    pub const VAL16: Self = Self(0x10);
    #[doc = "CT_INP16 input is selected"]
    pub const VAL17: Self = Self(0x11);
    #[doc = "CT_INP17 input is selected"]
    pub const VAL18: Self = Self(0x12);
    #[doc = "CT_INP18 input is selected"]
    pub const VAL19: Self = Self(0x13);
    #[doc = "CT_INP19 input is selected"]
    pub const VAL20: Self = Self(0x14);
    #[doc = "AOI0_OUT0 input is selected"]
    pub const VAL22: Self = Self(0x16);
    #[doc = "AOI0_OUT1 input is selected"]
    pub const VAL23: Self = Self(0x17);
    #[doc = "AOI0_OUT2 input is selected"]
    pub const VAL24: Self = Self(0x18);
    #[doc = "AOI0_OUT3 input is selected"]
    pub const VAL25: Self = Self(0x19);
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    pub const VAL26: Self = Self(0x1a);
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    pub const VAL27: Self = Self(0x1b);
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    pub const VAL28: Self = Self(0x1c);
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    pub const VAL29: Self = Self(0x1d);
    #[doc = "CMP0_OUT input is selected"]
    pub const VAL30: Self = Self(0x1e);
    #[doc = "CTimer0_MAT1 input is selected"]
    pub const VAL33: Self = Self(0x21);
    #[doc = "CTimer0_MAT2 input is selected"]
    pub const VAL34: Self = Self(0x22);
    #[doc = "CTimer0_MAT3 input is selected"]
    pub const VAL35: Self = Self(0x23);
    #[doc = "CTimer1_MAT1 input is selected"]
    pub const VAL36: Self = Self(0x24);
    #[doc = "CTimer1_MAT2 input is selected"]
    pub const VAL37: Self = Self(0x25);
    #[doc = "CTimer1_MAT3 input is selected"]
    pub const VAL38: Self = Self(0x26);
    #[doc = "LPI2C0 Master End of Packet input is selected"]
    pub const VAL48: Self = Self(0x30);
    #[doc = "LPI2C0 Slave End of Packet input is selected"]
    pub const VAL49: Self = Self(0x31);
    #[doc = "LPI2C1 Master End of Packet input is selected"]
    pub const VAL50: Self = Self(0x32);
    #[doc = "LPI2C1 Slave End of Packet input is selected"]
    pub const VAL51: Self = Self(0x33);
    #[doc = "LPSPI0 End of Frame input is selected"]
    pub const VAL52: Self = Self(0x34);
    #[doc = "LPSPI0 Received Data Word input is selected"]
    pub const VAL53: Self = Self(0x35);
    #[doc = "LPSPI1 End of Frame input is selected"]
    pub const VAL54: Self = Self(0x36);
    #[doc = "LPSPI1 Received Data Word input is selected"]
    pub const VAL55: Self = Self(0x37);
    #[doc = "LPUART0 Received Data Word input is selected"]
    pub const VAL56: Self = Self(0x38);
    #[doc = "LPUART0 Transmitted Data Word input is selected"]
    pub const VAL57: Self = Self(0x39);
    #[doc = "LPUART0 Receive Line Idle input is selected"]
    pub const VAL58: Self = Self(0x3a);
    #[doc = "LPUART1 Received Data Word input is selected"]
    pub const VAL59: Self = Self(0x3b);
    #[doc = "LPUART1 Transmitted Data Word input is selected"]
    pub const VAL60: Self = Self(0x3c);
    #[doc = "LPUART1 Receive Line Idle input is selected"]
    pub const VAL61: Self = Self(0x3d);
    #[doc = "LPUART2 Received Data Word input is selected"]
    pub const VAL62: Self = Self(0x3e);
    #[doc = "LPUART2 Transmitted Data Word input is selected"]
    pub const VAL63: Self = Self(0x3f);
    #[doc = "LPUART2 Receive Line Idle input is selected"]
    pub const VAL64: Self = Self(0x40);
    #[doc = "LPUART3 Received Data Word input is selected"]
    pub const VAL65: Self = Self(0x41);
    #[doc = "LPUART3 Transmitted Data Word input is selected"]
    pub const VAL66: Self = Self(0x42);
    #[doc = "LPUART3 Receive Line Idle input is selected"]
    pub const VAL67: Self = Self(0x43);
    #[doc = "LPUART4 Received Data Word input is selected"]
    pub const VAL68: Self = Self(0x44);
    #[doc = "LPUART4 Transmitted Data Word is selected"]
    pub const VAL69: Self = Self(0x45);
    #[doc = "LPUART4 Receive Line Idle input is selected"]
    pub const VAL70: Self = Self(0x46);
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    pub const VAL75: Self = Self(0x4b);
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    pub const VAL76: Self = Self(0x4c);
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    pub const VAL77: Self = Self(0x4d);
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    pub const VAL78: Self = Self(0x4e);
    #[doc = "CTimer2_MAT1 input is selected"]
    pub const VAL79: Self = Self(0x4f);
    #[doc = "CTimer2_MAT2 input is selected"]
    pub const VAL80: Self = Self(0x50);
    #[doc = "CTimer2_MAT3 input is selected"]
    pub const VAL81: Self = Self(0x51);
    #[doc = "CTimer4_MAT1 input is selected"]
    pub const VAL82: Self = Self(0x52);
    #[doc = "CTimer4_MAT2 input is selected"]
    pub const VAL83: Self = Self(0x53);
    #[doc = "CTimer4_MAT3 input is selected"]
    pub const VAL84: Self = Self(0x54);
    #[doc = "LPI2C2 Master End of Packet input is selected"]
    pub const VAL94: Self = Self(0x5e);
    #[doc = "LPI2C2 Slave End of Packet input is selected"]
    pub const VAL95: Self = Self(0x5f);
    #[doc = "LPI2C3 Master End of Packet input is selected"]
    pub const VAL96: Self = Self(0x60);
    #[doc = "LPI2C3 Slave End of Packet input is selected"]
    pub const VAL97: Self = Self(0x61);
    #[doc = "LPUART5 Received Data Word input is selected"]
    pub const VAL98: Self = Self(0x62);
    #[doc = "LPUART5 Transmitted Data Word input is selected"]
    pub const VAL99: Self = Self(0x63);
    #[doc = "LPUART5 Receive Line Idle input is selected"]
    pub const VAL100: Self = Self(0x64);
    #[doc = "TMPR_OUT0 input is selected"]
    pub const VAL102: Self = Self(0x66);
    #[doc = "TMPR_OUT1 input is selected"]
    pub const VAL103: Self = Self(0x67);
    #[doc = "TRIG_IN0 input is selected"]
    pub const VAL113: Self = Self(0x71);
    #[doc = "TRIG_IN1 input is selected"]
    pub const VAL114: Self = Self(0x72);
    #[doc = "TRIG_IN2 input is selected"]
    pub const VAL115: Self = Self(0x73);
    #[doc = "TRIG_IN3 input is selected"]
    pub const VAL116: Self = Self(0x74);
    #[doc = "TRIG_IN4 input is selected"]
    pub const VAL117: Self = Self(0x75);
    #[doc = "TRIG_IN5 input is selected"]
    pub const VAL118: Self = Self(0x76);
    #[doc = "TRIG_IN6 input is selected"]
    pub const VAL119: Self = Self(0x77);
    #[doc = "TRIG_IN7 input is selected"]
    pub const VAL120: Self = Self(0x78);
    #[doc = "TRIG_IN8 input is selected"]
    pub const VAL121: Self = Self(0x79);
    #[doc = "TRIG_IN9 input is selected"]
    pub const VAL122: Self = Self(0x7a);
    #[doc = "TRIG_IN10 input is selected"]
    pub const VAL123: Self = Self(0x7b);
    #[doc = "TRIG_IN11 input is selected"]
    pub const VAL124: Self = Self(0x7c);
    #[doc = "USB1 Start of Frame input is selected"]
    pub const VAL125: Self = Self(0x7d);
    #[doc = "LPSPI2 End of Frame input is selected"]
    pub const VAL126: Self = Self(0x7e);
    #[doc = "LPSPI2 Received Data Word input is selected"]
    pub const VAL127: Self = Self(0x7f);
    #[doc = "LPSPI3 End of Frame input is selected"]
    pub const VAL128: Self = Self(0x80);
    #[doc = "LPSPI3 Received Data Word input is selected"]
    pub const VAL129: Self = Self(0x81);
    #[doc = "LPSPI4 End of Frame input is selected"]
    pub const VAL130: Self = Self(0x82);
    #[doc = "LPSPI4 Received Data Word input is selected"]
    pub const VAL131: Self = Self(0x83);
    #[doc = "LPSPI5 End of Frame input is selected"]
    pub const VAL132: Self = Self(0x84);
    #[doc = "LPSPI5 Received Data Word input is selected"]
    pub const VAL133: Self = Self(0x85);
}
impl Timer3trigInp {
    pub const fn from_bits(val: u8) -> Timer3trigInp {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Timer3trigInp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("VAL1"),
            0x02 => f.write_str("VAL2"),
            0x03 => f.write_str("VAL3"),
            0x04 => f.write_str("VAL4"),
            0x05 => f.write_str("VAL5"),
            0x06 => f.write_str("VAL6"),
            0x07 => f.write_str("VAL7"),
            0x08 => f.write_str("VAL8"),
            0x09 => f.write_str("VAL9"),
            0x0a => f.write_str("VAL10"),
            0x0b => f.write_str("VAL11"),
            0x0c => f.write_str("VAL12"),
            0x0d => f.write_str("VAL13"),
            0x0e => f.write_str("VAL14"),
            0x0f => f.write_str("VAL15"),
            0x10 => f.write_str("VAL16"),
            0x11 => f.write_str("VAL17"),
            0x12 => f.write_str("VAL18"),
            0x13 => f.write_str("VAL19"),
            0x14 => f.write_str("VAL20"),
            0x16 => f.write_str("VAL22"),
            0x17 => f.write_str("VAL23"),
            0x18 => f.write_str("VAL24"),
            0x19 => f.write_str("VAL25"),
            0x1a => f.write_str("VAL26"),
            0x1b => f.write_str("VAL27"),
            0x1c => f.write_str("VAL28"),
            0x1d => f.write_str("VAL29"),
            0x1e => f.write_str("VAL30"),
            0x21 => f.write_str("VAL33"),
            0x22 => f.write_str("VAL34"),
            0x23 => f.write_str("VAL35"),
            0x24 => f.write_str("VAL36"),
            0x25 => f.write_str("VAL37"),
            0x26 => f.write_str("VAL38"),
            0x30 => f.write_str("VAL48"),
            0x31 => f.write_str("VAL49"),
            0x32 => f.write_str("VAL50"),
            0x33 => f.write_str("VAL51"),
            0x34 => f.write_str("VAL52"),
            0x35 => f.write_str("VAL53"),
            0x36 => f.write_str("VAL54"),
            0x37 => f.write_str("VAL55"),
            0x38 => f.write_str("VAL56"),
            0x39 => f.write_str("VAL57"),
            0x3a => f.write_str("VAL58"),
            0x3b => f.write_str("VAL59"),
            0x3c => f.write_str("VAL60"),
            0x3d => f.write_str("VAL61"),
            0x3e => f.write_str("VAL62"),
            0x3f => f.write_str("VAL63"),
            0x40 => f.write_str("VAL64"),
            0x41 => f.write_str("VAL65"),
            0x42 => f.write_str("VAL66"),
            0x43 => f.write_str("VAL67"),
            0x44 => f.write_str("VAL68"),
            0x45 => f.write_str("VAL69"),
            0x46 => f.write_str("VAL70"),
            0x4b => f.write_str("VAL75"),
            0x4c => f.write_str("VAL76"),
            0x4d => f.write_str("VAL77"),
            0x4e => f.write_str("VAL78"),
            0x4f => f.write_str("VAL79"),
            0x50 => f.write_str("VAL80"),
            0x51 => f.write_str("VAL81"),
            0x52 => f.write_str("VAL82"),
            0x53 => f.write_str("VAL83"),
            0x54 => f.write_str("VAL84"),
            0x5e => f.write_str("VAL94"),
            0x5f => f.write_str("VAL95"),
            0x60 => f.write_str("VAL96"),
            0x61 => f.write_str("VAL97"),
            0x62 => f.write_str("VAL98"),
            0x63 => f.write_str("VAL99"),
            0x64 => f.write_str("VAL100"),
            0x66 => f.write_str("VAL102"),
            0x67 => f.write_str("VAL103"),
            0x71 => f.write_str("VAL113"),
            0x72 => f.write_str("VAL114"),
            0x73 => f.write_str("VAL115"),
            0x74 => f.write_str("VAL116"),
            0x75 => f.write_str("VAL117"),
            0x76 => f.write_str("VAL118"),
            0x77 => f.write_str("VAL119"),
            0x78 => f.write_str("VAL120"),
            0x79 => f.write_str("VAL121"),
            0x7a => f.write_str("VAL122"),
            0x7b => f.write_str("VAL123"),
            0x7c => f.write_str("VAL124"),
            0x7d => f.write_str("VAL125"),
            0x7e => f.write_str("VAL126"),
            0x7f => f.write_str("VAL127"),
            0x80 => f.write_str("VAL128"),
            0x81 => f.write_str("VAL129"),
            0x82 => f.write_str("VAL130"),
            0x83 => f.write_str("VAL131"),
            0x84 => f.write_str("VAL132"),
            0x85 => f.write_str("VAL133"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer3trigInp {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "VAL1"),
            0x02 => defmt::write!(f, "VAL2"),
            0x03 => defmt::write!(f, "VAL3"),
            0x04 => defmt::write!(f, "VAL4"),
            0x05 => defmt::write!(f, "VAL5"),
            0x06 => defmt::write!(f, "VAL6"),
            0x07 => defmt::write!(f, "VAL7"),
            0x08 => defmt::write!(f, "VAL8"),
            0x09 => defmt::write!(f, "VAL9"),
            0x0a => defmt::write!(f, "VAL10"),
            0x0b => defmt::write!(f, "VAL11"),
            0x0c => defmt::write!(f, "VAL12"),
            0x0d => defmt::write!(f, "VAL13"),
            0x0e => defmt::write!(f, "VAL14"),
            0x0f => defmt::write!(f, "VAL15"),
            0x10 => defmt::write!(f, "VAL16"),
            0x11 => defmt::write!(f, "VAL17"),
            0x12 => defmt::write!(f, "VAL18"),
            0x13 => defmt::write!(f, "VAL19"),
            0x14 => defmt::write!(f, "VAL20"),
            0x16 => defmt::write!(f, "VAL22"),
            0x17 => defmt::write!(f, "VAL23"),
            0x18 => defmt::write!(f, "VAL24"),
            0x19 => defmt::write!(f, "VAL25"),
            0x1a => defmt::write!(f, "VAL26"),
            0x1b => defmt::write!(f, "VAL27"),
            0x1c => defmt::write!(f, "VAL28"),
            0x1d => defmt::write!(f, "VAL29"),
            0x1e => defmt::write!(f, "VAL30"),
            0x21 => defmt::write!(f, "VAL33"),
            0x22 => defmt::write!(f, "VAL34"),
            0x23 => defmt::write!(f, "VAL35"),
            0x24 => defmt::write!(f, "VAL36"),
            0x25 => defmt::write!(f, "VAL37"),
            0x26 => defmt::write!(f, "VAL38"),
            0x30 => defmt::write!(f, "VAL48"),
            0x31 => defmt::write!(f, "VAL49"),
            0x32 => defmt::write!(f, "VAL50"),
            0x33 => defmt::write!(f, "VAL51"),
            0x34 => defmt::write!(f, "VAL52"),
            0x35 => defmt::write!(f, "VAL53"),
            0x36 => defmt::write!(f, "VAL54"),
            0x37 => defmt::write!(f, "VAL55"),
            0x38 => defmt::write!(f, "VAL56"),
            0x39 => defmt::write!(f, "VAL57"),
            0x3a => defmt::write!(f, "VAL58"),
            0x3b => defmt::write!(f, "VAL59"),
            0x3c => defmt::write!(f, "VAL60"),
            0x3d => defmt::write!(f, "VAL61"),
            0x3e => defmt::write!(f, "VAL62"),
            0x3f => defmt::write!(f, "VAL63"),
            0x40 => defmt::write!(f, "VAL64"),
            0x41 => defmt::write!(f, "VAL65"),
            0x42 => defmt::write!(f, "VAL66"),
            0x43 => defmt::write!(f, "VAL67"),
            0x44 => defmt::write!(f, "VAL68"),
            0x45 => defmt::write!(f, "VAL69"),
            0x46 => defmt::write!(f, "VAL70"),
            0x4b => defmt::write!(f, "VAL75"),
            0x4c => defmt::write!(f, "VAL76"),
            0x4d => defmt::write!(f, "VAL77"),
            0x4e => defmt::write!(f, "VAL78"),
            0x4f => defmt::write!(f, "VAL79"),
            0x50 => defmt::write!(f, "VAL80"),
            0x51 => defmt::write!(f, "VAL81"),
            0x52 => defmt::write!(f, "VAL82"),
            0x53 => defmt::write!(f, "VAL83"),
            0x54 => defmt::write!(f, "VAL84"),
            0x5e => defmt::write!(f, "VAL94"),
            0x5f => defmt::write!(f, "VAL95"),
            0x60 => defmt::write!(f, "VAL96"),
            0x61 => defmt::write!(f, "VAL97"),
            0x62 => defmt::write!(f, "VAL98"),
            0x63 => defmt::write!(f, "VAL99"),
            0x64 => defmt::write!(f, "VAL100"),
            0x66 => defmt::write!(f, "VAL102"),
            0x67 => defmt::write!(f, "VAL103"),
            0x71 => defmt::write!(f, "VAL113"),
            0x72 => defmt::write!(f, "VAL114"),
            0x73 => defmt::write!(f, "VAL115"),
            0x74 => defmt::write!(f, "VAL116"),
            0x75 => defmt::write!(f, "VAL117"),
            0x76 => defmt::write!(f, "VAL118"),
            0x77 => defmt::write!(f, "VAL119"),
            0x78 => defmt::write!(f, "VAL120"),
            0x79 => defmt::write!(f, "VAL121"),
            0x7a => defmt::write!(f, "VAL122"),
            0x7b => defmt::write!(f, "VAL123"),
            0x7c => defmt::write!(f, "VAL124"),
            0x7d => defmt::write!(f, "VAL125"),
            0x7e => defmt::write!(f, "VAL126"),
            0x7f => defmt::write!(f, "VAL127"),
            0x80 => defmt::write!(f, "VAL128"),
            0x81 => defmt::write!(f, "VAL129"),
            0x82 => defmt::write!(f, "VAL130"),
            0x83 => defmt::write!(f, "VAL131"),
            0x84 => defmt::write!(f, "VAL132"),
            0x85 => defmt::write!(f, "VAL133"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Timer3trigInp {
    #[inline(always)]
    fn from(val: u8) -> Timer3trigInp {
        Timer3trigInp::from_bits(val)
    }
}
impl From<Timer3trigInp> for u8 {
    #[inline(always)]
    fn from(val: Timer3trigInp) -> u8 {
        Timer3trigInp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Timer4trigInp(u8);
impl Timer4trigInp {
    #[doc = "CT_INP0 input is selected"]
    pub const VAL1: Self = Self(0x01);
    #[doc = "CT_INP1 input is selected"]
    pub const VAL2: Self = Self(0x02);
    #[doc = "CT_INP2 input is selected"]
    pub const VAL3: Self = Self(0x03);
    #[doc = "CT_INP3 input is selected"]
    pub const VAL4: Self = Self(0x04);
    #[doc = "CT_INP4 input is selected"]
    pub const VAL5: Self = Self(0x05);
    #[doc = "CT_INP5 input is selected"]
    pub const VAL6: Self = Self(0x06);
    #[doc = "CT_INP6 input is selected"]
    pub const VAL7: Self = Self(0x07);
    #[doc = "CT_INP7 input is selected"]
    pub const VAL8: Self = Self(0x08);
    #[doc = "CT_INP8 input is selected"]
    pub const VAL9: Self = Self(0x09);
    #[doc = "CT_INP9 input is selected"]
    pub const VAL10: Self = Self(0x0a);
    #[doc = "CT_INP10 input is selected"]
    pub const VAL11: Self = Self(0x0b);
    #[doc = "CT_INP1 input is selected"]
    pub const VAL12: Self = Self(0x0c);
    #[doc = "CT_INP12 input is selected"]
    pub const VAL13: Self = Self(0x0d);
    #[doc = "CT_INP13 input is selected"]
    pub const VAL14: Self = Self(0x0e);
    #[doc = "CT_INP14 input is selected"]
    pub const VAL15: Self = Self(0x0f);
    #[doc = "CT_INP15 input is selected"]
    pub const VAL16: Self = Self(0x10);
    #[doc = "CT_INP16 input is selected"]
    pub const VAL17: Self = Self(0x11);
    #[doc = "CT_INP17 input is selected"]
    pub const VAL18: Self = Self(0x12);
    #[doc = "CT_INP18 input is selected"]
    pub const VAL19: Self = Self(0x13);
    #[doc = "CT_INP19 input is selected"]
    pub const VAL20: Self = Self(0x14);
    #[doc = "AOI0_OUT0 input is selected"]
    pub const VAL22: Self = Self(0x16);
    #[doc = "AOI0_OUT1 input is selected"]
    pub const VAL23: Self = Self(0x17);
    #[doc = "AOI0_OUT2 input is selected"]
    pub const VAL24: Self = Self(0x18);
    #[doc = "AOI0_OUT3 input is selected"]
    pub const VAL25: Self = Self(0x19);
    #[doc = "ADC0_tcomp\\[0\\] input is selected"]
    pub const VAL26: Self = Self(0x1a);
    #[doc = "ADC0_tcomp\\[1\\] input is selected"]
    pub const VAL27: Self = Self(0x1b);
    #[doc = "ADC0_tcomp\\[2\\] input is selected"]
    pub const VAL28: Self = Self(0x1c);
    #[doc = "ADC0_tcomp\\[3\\] input is selected"]
    pub const VAL29: Self = Self(0x1d);
    #[doc = "CMP0_OUT input is selected"]
    pub const VAL30: Self = Self(0x1e);
    #[doc = "CTimer0_MAT1 input is selected"]
    pub const VAL33: Self = Self(0x21);
    #[doc = "CTimer0_MAT2 input is selected"]
    pub const VAL34: Self = Self(0x22);
    #[doc = "CTimer0_MAT3 input is selected"]
    pub const VAL35: Self = Self(0x23);
    #[doc = "CTimer1_MAT1 input is selected"]
    pub const VAL36: Self = Self(0x24);
    #[doc = "CTimer1_MAT2 input is selected"]
    pub const VAL37: Self = Self(0x25);
    #[doc = "CTimer1_MAT3 input is selected"]
    pub const VAL38: Self = Self(0x26);
    #[doc = "LPI2C0 Master End of Packet input is selected"]
    pub const VAL48: Self = Self(0x30);
    #[doc = "LPI2C0 Slave End of Packet input is selected"]
    pub const VAL49: Self = Self(0x31);
    #[doc = "LPI2C1 Master End of Packet input is selected"]
    pub const VAL50: Self = Self(0x32);
    #[doc = "LPI2C1 Slave End of Packet input is selected"]
    pub const VAL51: Self = Self(0x33);
    #[doc = "LPSPI0 End of Frame input is selected"]
    pub const VAL52: Self = Self(0x34);
    #[doc = "LPSPI0 Received Data Word input is selected"]
    pub const VAL53: Self = Self(0x35);
    #[doc = "LPSPI1 End of Frame input is selected"]
    pub const VAL54: Self = Self(0x36);
    #[doc = "LPSPI1 Received Data Word input is selected"]
    pub const VAL55: Self = Self(0x37);
    #[doc = "LPUART0 Received Data Word input is selected"]
    pub const VAL56: Self = Self(0x38);
    #[doc = "LPUART0 Transmitted Data Word input is selected"]
    pub const VAL57: Self = Self(0x39);
    #[doc = "LPUART0 Receive Line Idle input is selected"]
    pub const VAL58: Self = Self(0x3a);
    #[doc = "LPUART1 Received Data Word input is selected"]
    pub const VAL59: Self = Self(0x3b);
    #[doc = "LPUART1 Transmitted Data Word input is selected"]
    pub const VAL60: Self = Self(0x3c);
    #[doc = "LPUART1 Receive Line Idle input is selected"]
    pub const VAL61: Self = Self(0x3d);
    #[doc = "LPUART2 Received Data Word input is selected"]
    pub const VAL62: Self = Self(0x3e);
    #[doc = "LPUART2 Transmitted Data Word input is selected"]
    pub const VAL63: Self = Self(0x3f);
    #[doc = "LPUART2 Receive Line Idle input is selected"]
    pub const VAL64: Self = Self(0x40);
    #[doc = "LPUART3 Received Data Word input is selected"]
    pub const VAL65: Self = Self(0x41);
    #[doc = "LPUART3 Transmitted Data Word input is selected"]
    pub const VAL66: Self = Self(0x42);
    #[doc = "LPUART3 Receive Line Idle input is selected"]
    pub const VAL67: Self = Self(0x43);
    #[doc = "LPUART4 Received Data Word input is selected"]
    pub const VAL68: Self = Self(0x44);
    #[doc = "LPUART4 Transmitted Data Word input is selected"]
    pub const VAL69: Self = Self(0x45);
    #[doc = "LPUART4 Receive Line Idle input is selected"]
    pub const VAL70: Self = Self(0x46);
    #[doc = "ADC1_tcomp\\[0\\] input is selected"]
    pub const VAL75: Self = Self(0x4b);
    #[doc = "ADC1_tcomp\\[1\\] input is selected"]
    pub const VAL76: Self = Self(0x4c);
    #[doc = "ADC1_tcomp\\[2\\] input is selected"]
    pub const VAL77: Self = Self(0x4d);
    #[doc = "ADC1_tcomp\\[3\\] input is selected"]
    pub const VAL78: Self = Self(0x4e);
    #[doc = "CTimer2_MAT1 input is selected"]
    pub const VAL79: Self = Self(0x4f);
    #[doc = "CTimer2_MAT2 input is selected"]
    pub const VAL80: Self = Self(0x50);
    #[doc = "CTimer2_MAT3 input is selected"]
    pub const VAL81: Self = Self(0x51);
    #[doc = "CTimer3_MAT1 input is selected"]
    pub const VAL82: Self = Self(0x52);
    #[doc = "CTimer3_MAT2 input is selected"]
    pub const VAL83: Self = Self(0x53);
    #[doc = "CTimer3_MAT3 input is selected"]
    pub const VAL84: Self = Self(0x54);
    #[doc = "LPI2C2 Master End of Packet input is selected"]
    pub const VAL94: Self = Self(0x5e);
    #[doc = "LPI2C2 Slave End of Packet input is selected"]
    pub const VAL95: Self = Self(0x5f);
    #[doc = "LPI2C3 Master End of Packet input is selected"]
    pub const VAL96: Self = Self(0x60);
    #[doc = "LPI2C3 Slave End of Packet input is selected"]
    pub const VAL97: Self = Self(0x61);
    #[doc = "LPUART5 Received Data Word input is selected"]
    pub const VAL98: Self = Self(0x62);
    #[doc = "LPUART5 Transmitted Data Word input is selected"]
    pub const VAL99: Self = Self(0x63);
    #[doc = "LPUART5 Receive Line Idle input is selected"]
    pub const VAL100: Self = Self(0x64);
    #[doc = "TMPR_OUT0 input is selected"]
    pub const VAL102: Self = Self(0x66);
    #[doc = "TMPR_OUT1 input is selected"]
    pub const VAL103: Self = Self(0x67);
    #[doc = "TRIG_IN0 input is selected"]
    pub const VAL113: Self = Self(0x71);
    #[doc = "TRIG_IN1 input is selected"]
    pub const VAL114: Self = Self(0x72);
    #[doc = "TRIG_IN2 input is selected"]
    pub const VAL115: Self = Self(0x73);
    #[doc = "TRIG_IN3 input is selected"]
    pub const VAL116: Self = Self(0x74);
    #[doc = "TRIG_IN4 is selected"]
    pub const VAL117: Self = Self(0x75);
    #[doc = "TRIG_IN5 is selected"]
    pub const VAL118: Self = Self(0x76);
    #[doc = "TRIG_IN6 input is selected"]
    pub const VAL119: Self = Self(0x77);
    #[doc = "TRIG_IN7 input is selected"]
    pub const VAL120: Self = Self(0x78);
    #[doc = "TRIG_IN8 input is selected"]
    pub const VAL121: Self = Self(0x79);
    #[doc = "TRIG_IN9 input is selected"]
    pub const VAL122: Self = Self(0x7a);
    #[doc = "TRIG_IN10 input is selected"]
    pub const VAL123: Self = Self(0x7b);
    #[doc = "TRIG_IN11 input is selected"]
    pub const VAL124: Self = Self(0x7c);
    #[doc = "USB1 Start of Frame input is selected"]
    pub const VAL125: Self = Self(0x7d);
    #[doc = "LPSPI2 End of Frame input is selected"]
    pub const VAL126: Self = Self(0x7e);
    #[doc = "LPSPI2 Received Data Word input is selected"]
    pub const VAL127: Self = Self(0x7f);
    #[doc = "LPSPI3 End of Frame input is selected"]
    pub const VAL128: Self = Self(0x80);
    #[doc = "LPSPI3 Received Data Word input is selected"]
    pub const VAL129: Self = Self(0x81);
    #[doc = "LPSPI4 End of Frame input is selected"]
    pub const VAL130: Self = Self(0x82);
    #[doc = "LPSPI4 Received Data Word input is selected"]
    pub const VAL131: Self = Self(0x83);
    #[doc = "LPSPI5 End of Frame input is selected"]
    pub const VAL132: Self = Self(0x84);
    #[doc = "LPSPI5 Received Data Word input is selected"]
    pub const VAL133: Self = Self(0x85);
}
impl Timer4trigInp {
    pub const fn from_bits(val: u8) -> Timer4trigInp {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Timer4trigInp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("VAL1"),
            0x02 => f.write_str("VAL2"),
            0x03 => f.write_str("VAL3"),
            0x04 => f.write_str("VAL4"),
            0x05 => f.write_str("VAL5"),
            0x06 => f.write_str("VAL6"),
            0x07 => f.write_str("VAL7"),
            0x08 => f.write_str("VAL8"),
            0x09 => f.write_str("VAL9"),
            0x0a => f.write_str("VAL10"),
            0x0b => f.write_str("VAL11"),
            0x0c => f.write_str("VAL12"),
            0x0d => f.write_str("VAL13"),
            0x0e => f.write_str("VAL14"),
            0x0f => f.write_str("VAL15"),
            0x10 => f.write_str("VAL16"),
            0x11 => f.write_str("VAL17"),
            0x12 => f.write_str("VAL18"),
            0x13 => f.write_str("VAL19"),
            0x14 => f.write_str("VAL20"),
            0x16 => f.write_str("VAL22"),
            0x17 => f.write_str("VAL23"),
            0x18 => f.write_str("VAL24"),
            0x19 => f.write_str("VAL25"),
            0x1a => f.write_str("VAL26"),
            0x1b => f.write_str("VAL27"),
            0x1c => f.write_str("VAL28"),
            0x1d => f.write_str("VAL29"),
            0x1e => f.write_str("VAL30"),
            0x21 => f.write_str("VAL33"),
            0x22 => f.write_str("VAL34"),
            0x23 => f.write_str("VAL35"),
            0x24 => f.write_str("VAL36"),
            0x25 => f.write_str("VAL37"),
            0x26 => f.write_str("VAL38"),
            0x30 => f.write_str("VAL48"),
            0x31 => f.write_str("VAL49"),
            0x32 => f.write_str("VAL50"),
            0x33 => f.write_str("VAL51"),
            0x34 => f.write_str("VAL52"),
            0x35 => f.write_str("VAL53"),
            0x36 => f.write_str("VAL54"),
            0x37 => f.write_str("VAL55"),
            0x38 => f.write_str("VAL56"),
            0x39 => f.write_str("VAL57"),
            0x3a => f.write_str("VAL58"),
            0x3b => f.write_str("VAL59"),
            0x3c => f.write_str("VAL60"),
            0x3d => f.write_str("VAL61"),
            0x3e => f.write_str("VAL62"),
            0x3f => f.write_str("VAL63"),
            0x40 => f.write_str("VAL64"),
            0x41 => f.write_str("VAL65"),
            0x42 => f.write_str("VAL66"),
            0x43 => f.write_str("VAL67"),
            0x44 => f.write_str("VAL68"),
            0x45 => f.write_str("VAL69"),
            0x46 => f.write_str("VAL70"),
            0x4b => f.write_str("VAL75"),
            0x4c => f.write_str("VAL76"),
            0x4d => f.write_str("VAL77"),
            0x4e => f.write_str("VAL78"),
            0x4f => f.write_str("VAL79"),
            0x50 => f.write_str("VAL80"),
            0x51 => f.write_str("VAL81"),
            0x52 => f.write_str("VAL82"),
            0x53 => f.write_str("VAL83"),
            0x54 => f.write_str("VAL84"),
            0x5e => f.write_str("VAL94"),
            0x5f => f.write_str("VAL95"),
            0x60 => f.write_str("VAL96"),
            0x61 => f.write_str("VAL97"),
            0x62 => f.write_str("VAL98"),
            0x63 => f.write_str("VAL99"),
            0x64 => f.write_str("VAL100"),
            0x66 => f.write_str("VAL102"),
            0x67 => f.write_str("VAL103"),
            0x71 => f.write_str("VAL113"),
            0x72 => f.write_str("VAL114"),
            0x73 => f.write_str("VAL115"),
            0x74 => f.write_str("VAL116"),
            0x75 => f.write_str("VAL117"),
            0x76 => f.write_str("VAL118"),
            0x77 => f.write_str("VAL119"),
            0x78 => f.write_str("VAL120"),
            0x79 => f.write_str("VAL121"),
            0x7a => f.write_str("VAL122"),
            0x7b => f.write_str("VAL123"),
            0x7c => f.write_str("VAL124"),
            0x7d => f.write_str("VAL125"),
            0x7e => f.write_str("VAL126"),
            0x7f => f.write_str("VAL127"),
            0x80 => f.write_str("VAL128"),
            0x81 => f.write_str("VAL129"),
            0x82 => f.write_str("VAL130"),
            0x83 => f.write_str("VAL131"),
            0x84 => f.write_str("VAL132"),
            0x85 => f.write_str("VAL133"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer4trigInp {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "VAL1"),
            0x02 => defmt::write!(f, "VAL2"),
            0x03 => defmt::write!(f, "VAL3"),
            0x04 => defmt::write!(f, "VAL4"),
            0x05 => defmt::write!(f, "VAL5"),
            0x06 => defmt::write!(f, "VAL6"),
            0x07 => defmt::write!(f, "VAL7"),
            0x08 => defmt::write!(f, "VAL8"),
            0x09 => defmt::write!(f, "VAL9"),
            0x0a => defmt::write!(f, "VAL10"),
            0x0b => defmt::write!(f, "VAL11"),
            0x0c => defmt::write!(f, "VAL12"),
            0x0d => defmt::write!(f, "VAL13"),
            0x0e => defmt::write!(f, "VAL14"),
            0x0f => defmt::write!(f, "VAL15"),
            0x10 => defmt::write!(f, "VAL16"),
            0x11 => defmt::write!(f, "VAL17"),
            0x12 => defmt::write!(f, "VAL18"),
            0x13 => defmt::write!(f, "VAL19"),
            0x14 => defmt::write!(f, "VAL20"),
            0x16 => defmt::write!(f, "VAL22"),
            0x17 => defmt::write!(f, "VAL23"),
            0x18 => defmt::write!(f, "VAL24"),
            0x19 => defmt::write!(f, "VAL25"),
            0x1a => defmt::write!(f, "VAL26"),
            0x1b => defmt::write!(f, "VAL27"),
            0x1c => defmt::write!(f, "VAL28"),
            0x1d => defmt::write!(f, "VAL29"),
            0x1e => defmt::write!(f, "VAL30"),
            0x21 => defmt::write!(f, "VAL33"),
            0x22 => defmt::write!(f, "VAL34"),
            0x23 => defmt::write!(f, "VAL35"),
            0x24 => defmt::write!(f, "VAL36"),
            0x25 => defmt::write!(f, "VAL37"),
            0x26 => defmt::write!(f, "VAL38"),
            0x30 => defmt::write!(f, "VAL48"),
            0x31 => defmt::write!(f, "VAL49"),
            0x32 => defmt::write!(f, "VAL50"),
            0x33 => defmt::write!(f, "VAL51"),
            0x34 => defmt::write!(f, "VAL52"),
            0x35 => defmt::write!(f, "VAL53"),
            0x36 => defmt::write!(f, "VAL54"),
            0x37 => defmt::write!(f, "VAL55"),
            0x38 => defmt::write!(f, "VAL56"),
            0x39 => defmt::write!(f, "VAL57"),
            0x3a => defmt::write!(f, "VAL58"),
            0x3b => defmt::write!(f, "VAL59"),
            0x3c => defmt::write!(f, "VAL60"),
            0x3d => defmt::write!(f, "VAL61"),
            0x3e => defmt::write!(f, "VAL62"),
            0x3f => defmt::write!(f, "VAL63"),
            0x40 => defmt::write!(f, "VAL64"),
            0x41 => defmt::write!(f, "VAL65"),
            0x42 => defmt::write!(f, "VAL66"),
            0x43 => defmt::write!(f, "VAL67"),
            0x44 => defmt::write!(f, "VAL68"),
            0x45 => defmt::write!(f, "VAL69"),
            0x46 => defmt::write!(f, "VAL70"),
            0x4b => defmt::write!(f, "VAL75"),
            0x4c => defmt::write!(f, "VAL76"),
            0x4d => defmt::write!(f, "VAL77"),
            0x4e => defmt::write!(f, "VAL78"),
            0x4f => defmt::write!(f, "VAL79"),
            0x50 => defmt::write!(f, "VAL80"),
            0x51 => defmt::write!(f, "VAL81"),
            0x52 => defmt::write!(f, "VAL82"),
            0x53 => defmt::write!(f, "VAL83"),
            0x54 => defmt::write!(f, "VAL84"),
            0x5e => defmt::write!(f, "VAL94"),
            0x5f => defmt::write!(f, "VAL95"),
            0x60 => defmt::write!(f, "VAL96"),
            0x61 => defmt::write!(f, "VAL97"),
            0x62 => defmt::write!(f, "VAL98"),
            0x63 => defmt::write!(f, "VAL99"),
            0x64 => defmt::write!(f, "VAL100"),
            0x66 => defmt::write!(f, "VAL102"),
            0x67 => defmt::write!(f, "VAL103"),
            0x71 => defmt::write!(f, "VAL113"),
            0x72 => defmt::write!(f, "VAL114"),
            0x73 => defmt::write!(f, "VAL115"),
            0x74 => defmt::write!(f, "VAL116"),
            0x75 => defmt::write!(f, "VAL117"),
            0x76 => defmt::write!(f, "VAL118"),
            0x77 => defmt::write!(f, "VAL119"),
            0x78 => defmt::write!(f, "VAL120"),
            0x79 => defmt::write!(f, "VAL121"),
            0x7a => defmt::write!(f, "VAL122"),
            0x7b => defmt::write!(f, "VAL123"),
            0x7c => defmt::write!(f, "VAL124"),
            0x7d => defmt::write!(f, "VAL125"),
            0x7e => defmt::write!(f, "VAL126"),
            0x7f => defmt::write!(f, "VAL127"),
            0x80 => defmt::write!(f, "VAL128"),
            0x81 => defmt::write!(f, "VAL129"),
            0x82 => defmt::write!(f, "VAL130"),
            0x83 => defmt::write!(f, "VAL131"),
            0x84 => defmt::write!(f, "VAL132"),
            0x85 => defmt::write!(f, "VAL133"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Timer4trigInp {
    #[inline(always)]
    fn from(val: u8) -> Timer4trigInp {
        Timer4trigInp::from_bits(val)
    }
}
impl From<Timer4trigInp> for u8 {
    #[inline(always)]
    fn from(val: Timer4trigInp) -> u8 {
        Timer4trigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrigOutInp {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected"]
    VAL2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected"]
    VAL3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected"]
    VAL4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected"]
    VAL5 = 0x05,
    #[doc = "CMP0_OUT input is selected"]
    VAL6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "LPUART0 ipp_do_lpuart_txd input is selected"]
    VAL9 = 0x09,
    #[doc = "LPUART1 ipp_do_lpuart_txd input is selected"]
    VAL10 = 0x0a,
    #[doc = "LPUART2 ipp_do_lpuart_txd input is selected"]
    VAL11 = 0x0b,
    #[doc = "LPUART3 ipp_do_lpuart_txd input is selected"]
    VAL12 = 0x0c,
    #[doc = "LPUART4 ipp_do_lpuart_txd input is selected"]
    VAL13 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    #[doc = "LPUART5 ipp_do_lpuart_txd input is selected"]
    VAL18 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    #[doc = "USB1 Start of Frame input is selected"]
    VAL21 = 0x15,
    #[doc = "ENET PPS Output input is selected"]
    VAL22 = 0x16,
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
impl TrigOutInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrigOutInp {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrigOutInp {
    #[inline(always)]
    fn from(val: u8) -> TrigOutInp {
        TrigOutInp::from_bits(val)
    }
}
impl From<TrigOutInp> for u8 {
    #[inline(always)]
    fn from(val: TrigOutInp) -> u8 {
        TrigOutInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsi0TrigInputInp {
    _RESERVED_0 = 0x0,
    #[doc = "CTimer0_MAT2 input is selected"]
    VAL1 = 0x01,
    #[doc = "CTimer0_MAT3 input is selected"]
    VAL2 = 0x02,
    #[doc = "CTimer1_MAT2 input is selected"]
    VAL3 = 0x03,
    #[doc = "CTimer1_MAT3 input is selected"]
    VAL4 = 0x04,
    #[doc = "CTimer2_MAT2 input is selected"]
    VAL5 = 0x05,
    #[doc = "CTimer2_MAT3 input is selected"]
    VAL6 = 0x06,
    #[doc = "CTimer3_MAT2 input is selected"]
    VAL7 = 0x07,
    #[doc = "CTimer3_MAT3 input is selected"]
    VAL8 = 0x08,
    #[doc = "CTimer4_MAT2 input is selected"]
    VAL9 = 0x09,
    #[doc = "CTimer4_MAT3 input is selected"]
    VAL10 = 0x0a,
    #[doc = "LPTMR0 input is selected"]
    VAL11 = 0x0b,
    #[doc = "WUU input is selected"]
    VAL12 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Tsi0TrigInputInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsi0TrigInputInp {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsi0TrigInputInp {
    #[inline(always)]
    fn from(val: u8) -> Tsi0TrigInputInp {
        Tsi0TrigInputInp::from_bits(val)
    }
}
impl From<Tsi0TrigInputInp> for u8 {
    #[inline(always)]
    fn from(val: Tsi0TrigInputInp) -> u8 {
        Tsi0TrigInputInp::to_bits(val)
    }
}
