#[doc = "ADC Trigger input connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcTrig(pub u32);
impl AdcTrig {
    #[doc = "ADC0 trigger inputs"]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::AdcTrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::AdcTrigTrigin::from_bits(val as u8)
    }
    #[doc = "ADC0 trigger inputs"]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::AdcTrigTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for AdcTrig {
    #[inline(always)]
    fn default() -> AdcTrig {
        AdcTrig(0)
    }
}
impl core::fmt::Debug for AdcTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcTrig")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AdcTrig {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "AOI0 trigger input connections 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AoiInput(pub u32);
impl AoiInput {
    #[doc = "AOI0 trigger input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::AoiInputInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::AoiInputInp::from_bits(val as u8)
    }
    #[doc = "AOI0 trigger input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::AoiInputInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for AoiInput {
    #[inline(always)]
    fn default() -> AoiInput {
        AoiInput(0)
    }
}
impl core::fmt::Debug for AoiInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AoiInput")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AoiInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AoiInput {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "CMP0 input connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmpTrig(pub u32);
impl CmpTrig {
    #[doc = "CMP0 input trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::CmpTrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::CmpTrigTrigin::from_bits(val as u8)
    }
    #[doc = "CMP0 input trigger"]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::CmpTrigTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for CmpTrig {
    #[inline(always)]
    fn default() -> CmpTrig {
        CmpTrig(0)
    }
}
impl core::fmt::Debug for CmpTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CmpTrig")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CmpTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CmpTrig {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "Capture select register for CTIMER inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer0cap(pub u32);
impl Ctimer0cap {
    #[doc = "Input number for CTIMER0"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer0capInp {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Ctimer0capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER0"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer0capInp) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Ctimer0cap {
    #[inline(always)]
    fn default() -> Ctimer0cap {
        Ctimer0cap(0)
    }
}
impl core::fmt::Debug for Ctimer0cap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer0cap")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer0cap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer0cap {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture select register for CTIMER inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer1cap(pub u32);
impl Ctimer1cap {
    #[doc = "Input number for CTIMER1"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer1capInp {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Ctimer1capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER1"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer1capInp) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Ctimer1cap {
    #[inline(always)]
    fn default() -> Ctimer1cap {
        Ctimer1cap(0)
    }
}
impl core::fmt::Debug for Ctimer1cap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer1cap")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer1cap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer1cap {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture select register for CTIMER inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer2cap(pub u32);
impl Ctimer2cap {
    #[doc = "Input number for CTIMER2"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer2capInp {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Ctimer2capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER2"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer2capInp) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Ctimer2cap {
    #[inline(always)]
    fn default() -> Ctimer2cap {
        Ctimer2cap(0)
    }
}
impl core::fmt::Debug for Ctimer2cap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer2cap")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer2cap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer2cap {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture select register for CTIMER inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer3cap(pub u32);
impl Ctimer3cap {
    #[doc = "Input number for CTIMER3"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer3capInp {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Ctimer3capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER3"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer3capInp) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Ctimer3cap {
    #[inline(always)]
    fn default() -> Ctimer3cap {
        Ctimer3cap(0)
    }
}
impl core::fmt::Debug for Ctimer3cap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer3cap")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer3cap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer3cap {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture select register for CTIMER inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer4cap(pub u32);
impl Ctimer4cap {
    #[doc = "Input number for CTIMER4"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer4capInp {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Ctimer4capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER4"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer4capInp) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Ctimer4cap {
    #[inline(always)]
    fn default() -> Ctimer4cap {
        Ctimer4cap(0)
    }
}
impl core::fmt::Debug for Ctimer4cap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer4cap")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer4cap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer4cap {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "DAC0 trigger"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacTrig(pub u32);
impl DacTrig {
    #[doc = "This register selects the DAC0 trigger inputs."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::DacTrigTrigin {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::DacTrigTrigin::from_bits(val as u8)
    }
    #[doc = "This register selects the DAC0 trigger inputs."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::DacTrigTrigin) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for DacTrig {
    #[inline(always)]
    fn default() -> DacTrig {
        DacTrig(0)
    }
}
impl core::fmt::Debug for DacTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacTrig")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DacTrig {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "ENET trigger input connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EnetTrigIn(pub u32);
impl EnetTrigIn {
    #[doc = "ENET trigger input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::EnetTrigInInp {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::EnetTrigInInp::from_bits(val as u8)
    }
    #[doc = "ENET trigger input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::EnetTrigInInp) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for EnetTrigIn {
    #[inline(always)]
    fn default() -> EnetTrigIn {
        EnetTrigIn(0)
    }
}
impl core::fmt::Debug for EnetTrigIn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EnetTrigIn")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EnetTrigIn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EnetTrigIn {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "FlexIO Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexioTrig(pub u32);
impl FlexioTrig {
    #[doc = "Input number for FlexIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::FlexioTrigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::FlexioTrigInp::from_bits(val as u8)
    }
    #[doc = "Input number for FlexIO0."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::FlexioTrigInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for FlexioTrig {
    #[inline(always)]
    fn default() -> FlexioTrig {
        FlexioTrig(0)
    }
}
impl core::fmt::Debug for FlexioTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexioTrig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexioTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexioTrig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Selection for frequency measurement reference clock"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FreqmeasRef(pub u32);
impl FreqmeasRef {
    #[doc = "Clock source number (binary value) for frequency measure function target clock."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::FreqmeasRefInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FreqmeasRefInp::from_bits(val as u8)
    }
    #[doc = "Clock source number (binary value) for frequency measure function target clock."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::FreqmeasRefInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for FreqmeasRef {
    #[inline(always)]
    fn default() -> FreqmeasRef {
        FreqmeasRef(0)
    }
}
impl core::fmt::Debug for FreqmeasRef {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FreqmeasRef")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FreqmeasRef {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FreqmeasRef {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Selection for frequency measurement reference clock"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FreqmeasTar(pub u32);
impl FreqmeasTar {
    #[doc = "Clock source number (binary value) for frequency measure function target clock."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::FreqmeasTarInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FreqmeasTarInp::from_bits(val as u8)
    }
    #[doc = "Clock source number (binary value) for frequency measure function target clock."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::FreqmeasTarInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for FreqmeasTar {
    #[inline(always)]
    fn default() -> FreqmeasTar {
        FreqmeasTar(0)
    }
}
impl core::fmt::Debug for FreqmeasTar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FreqmeasTar")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FreqmeasTar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FreqmeasTar {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "LPI2C0 trigger input connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2cTrig(pub u32);
impl Lpi2cTrig {
    #[doc = "LPI2C0 trigger input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Lpi2cTrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Lpi2cTrigInp::from_bits(val as u8)
    }
    #[doc = "LPI2C0 trigger input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Lpi2cTrigInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Lpi2cTrig {
    #[inline(always)]
    fn default() -> Lpi2cTrig {
        Lpi2cTrig(0)
    }
}
impl core::fmt::Debug for Lpi2cTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2cTrig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2cTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpi2cTrig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "LPSPI0 trigger input connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpspiTrig(pub u32);
impl LpspiTrig {
    #[doc = "LPSPI0 trigger input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::LpspiTrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::LpspiTrigInp::from_bits(val as u8)
    }
    #[doc = "LPSPI0 trigger input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::LpspiTrigInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for LpspiTrig {
    #[inline(always)]
    fn default() -> LpspiTrig {
        LpspiTrig(0)
    }
}
impl core::fmt::Debug for LpspiTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpspiTrig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpspiTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LpspiTrig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "LPUART0 trigger input connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart(pub u32);
impl Lpuart {
    #[doc = "LPUART0 trigger input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::LpuartInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::LpuartInp::from_bits(val as u8)
    }
    #[doc = "LPUART0 trigger input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::LpuartInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Lpuart {
    #[inline(always)]
    fn default() -> Lpuart {
        Lpuart(0)
    }
}
impl core::fmt::Debug for Lpuart {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart").field("inp", &self.inp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "SmartDMA Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmartDmaTrig(pub u32);
impl SmartDmaTrig {
    #[doc = "Input number for SmartDMA."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::SmartDmaTrigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::SmartDmaTrigInp::from_bits(val as u8)
    }
    #[doc = "Input number for SmartDMA."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::SmartDmaTrigInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for SmartDmaTrig {
    #[inline(always)]
    fn default() -> SmartDmaTrig {
        SmartDmaTrig(0)
    }
}
impl core::fmt::Debug for SmartDmaTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmartDmaTrig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmartDmaTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SmartDmaTrig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "10BASE-T1S Local Wake Up connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T1sWkup(pub u32);
impl T1sWkup {
    #[doc = "Input number for 10BASE-T1S Local Wake Up"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::T1sWkupInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::T1sWkupInp::from_bits(val as u8)
    }
    #[doc = "Input number for 10BASE-T1S Local Wake Up"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::T1sWkupInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for T1sWkup {
    #[inline(always)]
    fn default() -> T1sWkup {
        T1sWkup(0)
    }
}
impl core::fmt::Debug for T1sWkup {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T1sWkup").field("inp", &self.inp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for T1sWkup {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "T1sWkup {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Trigger register for TIMER0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer0trig(pub u32);
impl Timer0trig {
    #[doc = "Input number for CTIMER0"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Timer0trigInp {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Timer0trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER0"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Timer0trigInp) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Timer0trig {
    #[inline(always)]
    fn default() -> Timer0trig {
        Timer0trig(0)
    }
}
impl core::fmt::Debug for Timer0trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer0trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer0trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timer0trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Trigger register for TIMER1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer1trig(pub u32);
impl Timer1trig {
    #[doc = "Input number for CTIMER1"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Timer1trigInp {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Timer1trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER1"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Timer1trigInp) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Timer1trig {
    #[inline(always)]
    fn default() -> Timer1trig {
        Timer1trig(0)
    }
}
impl core::fmt::Debug for Timer1trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer1trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer1trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timer1trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Trigger register for TIMER2 inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2trig(pub u32);
impl Timer2trig {
    #[doc = "Input number for CTIMER2"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Timer2trigInp {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Timer2trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER2"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Timer2trigInp) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Timer2trig {
    #[inline(always)]
    fn default() -> Timer2trig {
        Timer2trig(0)
    }
}
impl core::fmt::Debug for Timer2trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer2trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer2trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timer2trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Trigger register for TIMER3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer3trig(pub u32);
impl Timer3trig {
    #[doc = "Input number for CTIMER3"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Timer3trigInp {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Timer3trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER3"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Timer3trigInp) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Timer3trig {
    #[inline(always)]
    fn default() -> Timer3trig {
        Timer3trig(0)
    }
}
impl core::fmt::Debug for Timer3trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer3trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer3trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timer3trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Trigger register for TIMER4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer4trig(pub u32);
impl Timer4trig {
    #[doc = "Input number for CTIMER4"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Timer4trigInp {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Timer4trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER4"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Timer4trigInp) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Timer4trig {
    #[inline(always)]
    fn default() -> Timer4trig {
        Timer4trig(0)
    }
}
impl core::fmt::Debug for Timer4trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer4trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer4trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timer4trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "EXT trigger connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrigOut(pub u32);
impl TrigOut {
    #[doc = "EXT trigger input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::TrigOutInp {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::TrigOutInp::from_bits(val as u8)
    }
    #[doc = "EXT trigger input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::TrigOutInp) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for TrigOut {
    #[inline(always)]
    fn default() -> TrigOut {
        TrigOut(0)
    }
}
impl core::fmt::Debug for TrigOut {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrigOut").field("inp", &self.inp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrigOut {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TrigOut {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "TSI0 trigger input connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsi0TrigInput(pub u32);
impl Tsi0TrigInput {
    #[doc = "TSI0 trigger input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Tsi0TrigInputInp {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Tsi0TrigInputInp::from_bits(val as u8)
    }
    #[doc = "TSI0 trigger input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Tsi0TrigInputInp) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Tsi0TrigInput {
    #[inline(always)]
    fn default() -> Tsi0TrigInput {
        Tsi0TrigInput(0)
    }
}
impl core::fmt::Debug for Tsi0TrigInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsi0TrigInput")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsi0TrigInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tsi0TrigInput {{ inp: {:?} }}", self.inp())
    }
}
