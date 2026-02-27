#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ActiveCfgBgmode {
    #[doc = "Bandgap disabled"]
    BGMODE0 = 0x0,
    #[doc = "Bandgap enabled, buffer disabled"]
    BGMODE01 = 0x01,
    #[doc = "Bandgap enabled, buffer enabled"]
    BGMODE10 = 0x02,
    _RESERVED_3 = 0x03,
}
impl ActiveCfgBgmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ActiveCfgBgmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ActiveCfgBgmode {
    #[inline(always)]
    fn from(val: u8) -> ActiveCfgBgmode {
        ActiveCfgBgmode::from_bits(val)
    }
}
impl From<ActiveCfgBgmode> for u8 {
    #[inline(always)]
    fn from(val: ActiveCfgBgmode) -> u8 {
        ActiveCfgBgmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ActiveCfgCoreldoVddDs {
    #[doc = "Low"]
    LOW = 0x0,
    #[doc = "Normal"]
    NORMAL = 0x01,
}
impl ActiveCfgCoreldoVddDs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ActiveCfgCoreldoVddDs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ActiveCfgCoreldoVddDs {
    #[inline(always)]
    fn from(val: u8) -> ActiveCfgCoreldoVddDs {
        ActiveCfgCoreldoVddDs::from_bits(val)
    }
}
impl From<ActiveCfgCoreldoVddDs> for u8 {
    #[inline(always)]
    fn from(val: ActiveCfgCoreldoVddDs) -> u8 {
        ActiveCfgCoreldoVddDs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ActiveCfgCoreldoVddLvl {
    _RESERVED_0 = 0x0,
    #[doc = "Regulate to mid voltage (1.0 V)"]
    MID = 0x01,
    #[doc = "Regulate to normal voltage (1.1 V)"]
    NORMAL = 0x02,
    #[doc = "Regulate to overdrive voltage (1.15 V)"]
    OVER = 0x03,
}
impl ActiveCfgCoreldoVddLvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ActiveCfgCoreldoVddLvl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ActiveCfgCoreldoVddLvl {
    #[inline(always)]
    fn from(val: u8) -> ActiveCfgCoreldoVddLvl {
        ActiveCfgCoreldoVddLvl::from_bits(val)
    }
}
impl From<ActiveCfgCoreldoVddLvl> for u8 {
    #[inline(always)]
    fn from(val: ActiveCfgCoreldoVddLvl) -> u8 {
        ActiveCfgCoreldoVddLvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CoreldoSpare {
    #[doc = "TBD."]
    DISABLED = 0x0,
    #[doc = "TBD."]
    ENABLED = 0x01,
    _RESERVED_2 = 0x02,
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
impl CoreldoSpare {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CoreldoSpare {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CoreldoSpare {
    #[inline(always)]
    fn from(val: u8) -> CoreldoSpare {
        CoreldoSpare::from_bits(val)
    }
}
impl From<CoreldoSpare> for u8 {
    #[inline(always)]
    fn from(val: CoreldoSpare) -> u8 {
        CoreldoSpare::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CorevddHvdStatus {
    #[doc = "Event not detected"]
    EVENT_NO = 0x0,
    #[doc = "Event detected"]
    EVENT_YES = 0x01,
}
impl CorevddHvdStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CorevddHvdStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CorevddHvdStatus {
    #[inline(always)]
    fn from(val: u8) -> CorevddHvdStatus {
        CorevddHvdStatus::from_bits(val)
    }
}
impl From<CorevddHvdStatus> for u8 {
    #[inline(always)]
    fn from(val: CorevddHvdStatus) -> u8 {
        CorevddHvdStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CorevddHvdf {
    #[doc = "Event not detected"]
    EVENT_NO = 0x0,
    #[doc = "Event detected"]
    EVENT_YES = 0x01,
}
impl CorevddHvdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CorevddHvdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CorevddHvdf {
    #[inline(always)]
    fn from(val: u8) -> CorevddHvdf {
        CorevddHvdf::from_bits(val)
    }
}
impl From<CorevddHvdf> for u8 {
    #[inline(always)]
    fn from(val: CorevddHvdf) -> u8 {
        CorevddHvdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CorevddLvdStatus {
    #[doc = "Event not detected"]
    EVENT_NO = 0x0,
    #[doc = "Event detected"]
    EVENT_YES = 0x01,
}
impl CorevddLvdStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CorevddLvdStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CorevddLvdStatus {
    #[inline(always)]
    fn from(val: u8) -> CorevddLvdStatus {
        CorevddLvdStatus::from_bits(val)
    }
}
impl From<CorevddLvdStatus> for u8 {
    #[inline(always)]
    fn from(val: CorevddLvdStatus) -> u8 {
        CorevddLvdStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CorevddLvdf {
    #[doc = "Event not detected"]
    EVENT_NO = 0x0,
    #[doc = "Event detected"]
    EVENT_YES = 0x01,
}
impl CorevddLvdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CorevddLvdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CorevddLvdf {
    #[inline(always)]
    fn from(val: u8) -> CorevddLvdf {
        CorevddLvdf::from_bits(val)
    }
}
impl From<CorevddLvdf> for u8 {
    #[inline(always)]
    fn from(val: CorevddLvdf) -> u8 {
        CorevddLvdf::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Standard features"]
    pub const STANDARD: Self = Self(0x0);
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
            0x0 => f.write_str("STANDARD"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "STANDARD"),
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
pub enum GlitchDetectScCntSelect {
    #[doc = "0"]
    BIT0 = 0x0,
    #[doc = "1"]
    BIT1 = 0x01,
    #[doc = "2"]
    BIT2 = 0x02,
    #[doc = "3"]
    BIT3 = 0x03,
}
impl GlitchDetectScCntSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GlitchDetectScCntSelect {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GlitchDetectScCntSelect {
    #[inline(always)]
    fn from(val: u8) -> GlitchDetectScCntSelect {
        GlitchDetectScCntSelect::from_bits(val)
    }
}
impl From<GlitchDetectScCntSelect> for u8 {
    #[inline(always)]
    fn from(val: GlitchDetectScCntSelect) -> u8 {
        GlitchDetectScCntSelect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GlitchVddaDetectScCntSelect {
    #[doc = "0"]
    BIT0 = 0x0,
    #[doc = "1"]
    BIT1 = 0x01,
    #[doc = "2"]
    BIT2 = 0x02,
    #[doc = "3"]
    BIT3 = 0x03,
}
impl GlitchVddaDetectScCntSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GlitchVddaDetectScCntSelect {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GlitchVddaDetectScCntSelect {
    #[inline(always)]
    fn from(val: u8) -> GlitchVddaDetectScCntSelect {
        GlitchVddaDetectScCntSelect::from_bits(val)
    }
}
impl From<GlitchVddaDetectScCntSelect> for u8 {
    #[inline(always)]
    fn from(val: GlitchVddaDetectScCntSelect) -> u8 {
        GlitchVddaDetectScCntSelect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HvddAmpDisable {
    #[doc = "HVDD Amplifier detection enabled."]
    ENABLED = 0x0,
    #[doc = "HVDD Amplifier detection disabled."]
    DISABLED = 0x01,
}
impl HvddAmpDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HvddAmpDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HvddAmpDisable {
    #[inline(always)]
    fn from(val: u8) -> HvddAmpDisable {
        HvddAmpDisable::from_bits(val)
    }
}
impl From<HvddAmpDisable> for u8 {
    #[inline(always)]
    fn from(val: HvddAmpDisable) -> u8 {
        HvddAmpDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpCfgBgmode {
    #[doc = "Bandgap disabled"]
    BGMODE0 = 0x0,
    #[doc = "Bandgap enabled, buffer disabled"]
    BGMODE01 = 0x01,
    #[doc = "Bandgap enabled, buffer enabled"]
    BGMODE10 = 0x02,
    _RESERVED_3 = 0x03,
}
impl LpCfgBgmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpCfgBgmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpCfgBgmode {
    #[inline(always)]
    fn from(val: u8) -> LpCfgBgmode {
        LpCfgBgmode::from_bits(val)
    }
}
impl From<LpCfgBgmode> for u8 {
    #[inline(always)]
    fn from(val: LpCfgBgmode) -> u8 {
        LpCfgBgmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpCfgCoreldoVddDs {
    #[doc = "Low"]
    LOW = 0x0,
    #[doc = "Normal"]
    NORMAL = 0x01,
}
impl LpCfgCoreldoVddDs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpCfgCoreldoVddDs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpCfgCoreldoVddDs {
    #[inline(always)]
    fn from(val: u8) -> LpCfgCoreldoVddDs {
        LpCfgCoreldoVddDs::from_bits(val)
    }
}
impl From<LpCfgCoreldoVddDs> for u8 {
    #[inline(always)]
    fn from(val: LpCfgCoreldoVddDs) -> u8 {
        LpCfgCoreldoVddDs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpCfgCoreldoVddLvl {
    _RESERVED_0 = 0x0,
    #[doc = "Mid voltage (1.0 V)"]
    MID = 0x01,
    #[doc = "Normal voltage (1.1 V)"]
    NORMAL = 0x02,
    #[doc = "Overdrive voltage (1.15 V)"]
    OVER = 0x03,
}
impl LpCfgCoreldoVddLvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpCfgCoreldoVddLvl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpCfgCoreldoVddLvl {
    #[inline(always)]
    fn from(val: u8) -> LpCfgCoreldoVddLvl {
        LpCfgCoreldoVddLvl::from_bits(val)
    }
}
impl From<LpCfgCoreldoVddLvl> for u8 {
    #[inline(always)]
    fn from(val: LpCfgCoreldoVddLvl) -> u8 {
        LpCfgCoreldoVddLvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpCfgGlitchDetectDisable {
    #[doc = "Enable"]
    ENABLE = 0x0,
    #[doc = "Disable"]
    DISABLE = 0x01,
}
impl LpCfgGlitchDetectDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpCfgGlitchDetectDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpCfgGlitchDetectDisable {
    #[inline(always)]
    fn from(val: u8) -> LpCfgGlitchDetectDisable {
        LpCfgGlitchDetectDisable::from_bits(val)
    }
}
impl From<LpCfgGlitchDetectDisable> for u8 {
    #[inline(always)]
    fn from(val: LpCfgGlitchDetectDisable) -> u8 {
        LpCfgGlitchDetectDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpMode {
    #[doc = "SLEEP with system clock running"]
    MODE0 = 0x0,
    #[doc = "DSLEEP with system clock off"]
    MODE1 = 0x01,
    #[doc = "PDOWN with system clock off"]
    MODE2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "DPDOWN with system clock off"]
    MODE8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl LpMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpMode {
    #[inline(always)]
    fn from(val: u8) -> LpMode {
        LpMode::from_bits(val)
    }
}
impl From<LpMode> for u8 {
    #[inline(always)]
    fn from(val: LpMode) -> u8 {
        LpMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpreqov {
    #[doc = "Not forced"]
    FORCE_NO = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Forced low (ignore LPREQPOL settings)"]
    FORCE_LOW = 0x02,
    #[doc = "Forced high (ignore LPREQPOL settings)"]
    FORCE_HIGH = 0x03,
}
impl Lpreqov {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpreqov {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpreqov {
    #[inline(always)]
    fn from(val: u8) -> Lpreqov {
        Lpreqov::from_bits(val)
    }
}
impl From<Lpreqov> for u8 {
    #[inline(always)]
    fn from(val: Lpreqov) -> u8 {
        Lpreqov::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpreqpol {
    #[doc = "High"]
    HIGH = 0x0,
    #[doc = "Low"]
    LOW = 0x01,
}
impl Lpreqpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpreqpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpreqpol {
    #[inline(always)]
    fn from(val: u8) -> Lpreqpol {
        Lpreqpol::from_bits(val)
    }
}
impl From<Lpreqpol> for u8 {
    #[inline(always)]
    fn from(val: Lpreqpol) -> u8 {
        Lpreqpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lvsel {
    #[doc = "Normal"]
    NORMAL = 0x0,
    #[doc = "Safe"]
    SAFE = 0x01,
}
impl Lvsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvsel {
    #[inline(always)]
    fn from(val: u8) -> Lvsel {
        Lvsel::from_bits(val)
    }
}
impl From<Lvsel> for u8 {
    #[inline(always)]
    fn from(val: Lvsel) -> u8 {
        Lvsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdLpReq {
    #[doc = "Did not request"]
    REQ_NO = 0x0,
    #[doc = "Requested"]
    REQ_YES = 0x01,
}
impl PdLpReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdLpReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdLpReq {
    #[inline(always)]
    fn from(val: u8) -> PdLpReq {
        PdLpReq::from_bits(val)
    }
}
impl From<PdLpReq> for u8 {
    #[inline(always)]
    fn from(val: PdLpReq) -> u8 {
        PdLpReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwrReqStatus {
    #[doc = "Did not request"]
    REQ_NO = 0x0,
    #[doc = "Requested"]
    REQ_YES = 0x01,
}
impl PwrReqStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwrReqStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwrReqStatus {
    #[inline(always)]
    fn from(val: u8) -> PwrReqStatus {
        PwrReqStatus::from_bits(val)
    }
}
impl From<PwrReqStatus> for u8 {
    #[inline(always)]
    fn from(val: PwrReqStatus) -> u8 {
        PwrReqStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegBusy {
    #[doc = "Not busy"]
    REG_BUSY_NO = 0x0,
    #[doc = "Busy"]
    REG_BUSY_YES = 0x01,
}
impl RegBusy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegBusy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegBusy {
    #[inline(always)]
    fn from(val: u8) -> RegBusy {
        RegBusy::from_bits(val)
    }
}
impl From<RegBusy> for u8 {
    #[inline(always)]
    fn from(val: RegBusy) -> u8 {
        RegBusy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpcLpMode {
    #[doc = "Sleep mode with system clock running"]
    MODE0 = 0x0,
    #[doc = "DSLEEP with system clock off"]
    MODE1 = 0x01,
    #[doc = "PDOWN with system clock off"]
    MODE2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "DPDOWN with system clock off"]
    MODE8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SpcLpMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpcLpMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpcLpMode {
    #[inline(always)]
    fn from(val: u8) -> SpcLpMode {
        SpcLpMode::from_bits(val)
    }
}
impl From<SpcLpMode> for u8 {
    #[inline(always)]
    fn from(val: SpcLpMode) -> u8 {
        SpcLpMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpcLpReq {
    #[doc = "SPC is in Active mode; the ACTIVE_CFG register has control"]
    ACTIVE = 0x0,
    #[doc = "All power domains requested low-power mode; SPC entered a low-power state; power-mode configuration based on the LP_CFG register"]
    LOW_POWER = 0x01,
}
impl SpcLpReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpcLpReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpcLpReq {
    #[inline(always)]
    fn from(val: u8) -> SpcLpReq {
        SpcLpReq::from_bits(val)
    }
}
impl From<SpcLpReq> for u8 {
    #[inline(always)]
    fn from(val: SpcLpReq) -> u8 {
        SpcLpReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysvddHvdStatus {
    #[doc = "Event not detected"]
    EVENT_NO = 0x0,
    #[doc = "Event detected"]
    EVENT_YES = 0x01,
}
impl SysvddHvdStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysvddHvdStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysvddHvdStatus {
    #[inline(always)]
    fn from(val: u8) -> SysvddHvdStatus {
        SysvddHvdStatus::from_bits(val)
    }
}
impl From<SysvddHvdStatus> for u8 {
    #[inline(always)]
    fn from(val: SysvddHvdStatus) -> u8 {
        SysvddHvdStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysvddHvdf {
    #[doc = "Event not detected"]
    EVENT_NO = 0x0,
    #[doc = "Event detected"]
    EVENT_YES = 0x01,
}
impl SysvddHvdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysvddHvdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysvddHvdf {
    #[inline(always)]
    fn from(val: u8) -> SysvddHvdf {
        SysvddHvdf::from_bits(val)
    }
}
impl From<SysvddHvdf> for u8 {
    #[inline(always)]
    fn from(val: SysvddHvdf) -> u8 {
        SysvddHvdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysvddLvdStatus {
    #[doc = "Event not detected"]
    EVENT_NO = 0x0,
    #[doc = "Event detected"]
    EVENT_YES = 0x01,
}
impl SysvddLvdStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysvddLvdStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysvddLvdStatus {
    #[inline(always)]
    fn from(val: u8) -> SysvddLvdStatus {
        SysvddLvdStatus::from_bits(val)
    }
}
impl From<SysvddLvdStatus> for u8 {
    #[inline(always)]
    fn from(val: SysvddLvdStatus) -> u8 {
        SysvddLvdStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysvddLvdf {
    #[doc = "Event not detected"]
    EVENT_NO = 0x0,
    #[doc = "Event detected"]
    EVENT_YES = 0x01,
}
impl SysvddLvdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysvddLvdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysvddLvdf {
    #[inline(always)]
    fn from(val: u8) -> SysvddLvdf {
        SysvddLvdf::from_bits(val)
    }
}
impl From<SysvddLvdf> for u8 {
    #[inline(always)]
    fn from(val: SysvddLvdf) -> u8 {
        SysvddLvdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VdCoreCfgLock {
    #[doc = "Allow"]
    ALLOW = 0x0,
    #[doc = "Deny"]
    DENY = 0x01,
}
impl VdCoreCfgLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VdCoreCfgLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VdCoreCfgLock {
    #[inline(always)]
    fn from(val: u8) -> VdCoreCfgLock {
        VdCoreCfgLock::from_bits(val)
    }
}
impl From<VdCoreCfgLock> for u8 {
    #[inline(always)]
    fn from(val: VdCoreCfgLock) -> u8 {
        VdCoreCfgLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VdSysCfgLock {
    #[doc = "Allow"]
    ALLOW = 0x0,
    #[doc = "Deny"]
    DENY = 0x01,
}
impl VdSysCfgLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VdSysCfgLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VdSysCfgLock {
    #[inline(always)]
    fn from(val: u8) -> VdSysCfgLock {
        VdSysCfgLock::from_bits(val)
    }
}
impl From<VdSysCfgLock> for u8 {
    #[inline(always)]
    fn from(val: VdSysCfgLock) -> u8 {
        VdSysCfgLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VddVdDisable {
    #[doc = "Enable"]
    ENABLE = 0x0,
    #[doc = "Disable"]
    DISABLE = 0x01,
}
impl VddVdDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VddVdDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VddVdDisable {
    #[inline(always)]
    fn from(val: u8) -> VddVdDisable {
        VddVdDisable::from_bits(val)
    }
}
impl From<VddVdDisable> for u8 {
    #[inline(always)]
    fn from(val: VddVdDisable) -> u8 {
        VddVdDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vsm {
    _RESERVED_0 = 0x0,
    #[doc = "SRAM configured for 1.0V"]
    SRAM1V0 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "SRAM configured for 1.2V"]
    SRAM1V2 = 0x03,
}
impl Vsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vsm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vsm {
    #[inline(always)]
    fn from(val: u8) -> Vsm {
        Vsm::from_bits(val)
    }
}
impl From<Vsm> for u8 {
    #[inline(always)]
    fn from(val: Vsm) -> u8 {
        Vsm::to_bits(val)
    }
}
