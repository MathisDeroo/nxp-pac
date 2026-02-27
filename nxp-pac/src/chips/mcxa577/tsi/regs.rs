#[doc = "TSI Baseline Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Baseline(pub u32);
impl Baseline {
    #[doc = "TSI_BASELINE"]
    #[must_use]
    #[inline(always)]
    pub const fn tsi_baseline(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "TSI_BASELINE"]
    #[inline(always)]
    pub const fn set_tsi_baseline(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "BASE_TRACE_ DEBOUNCE"]
    #[must_use]
    #[inline(always)]
    pub const fn base_trace_debounce(&self) -> super::vals::BaseTraceDebounce {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::BaseTraceDebounce::from_bits(val as u8)
    }
    #[doc = "BASE_TRACE_ DEBOUNCE"]
    #[inline(always)]
    pub const fn set_base_trace_debounce(&mut self, val: super::vals::BaseTraceDebounce) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "BASE_TRACE_EN"]
    #[must_use]
    #[inline(always)]
    pub const fn base_trace_en(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "BASE_TRACE_EN"]
    #[inline(always)]
    pub const fn set_base_trace_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "THESHOLD_RATIO"]
    #[must_use]
    #[inline(always)]
    pub const fn theshold_ratio(&self) -> super::vals::ThesholdRatio {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::ThesholdRatio::from_bits(val as u8)
    }
    #[doc = "THESHOLD_RATIO"]
    #[inline(always)]
    pub const fn set_theshold_ratio(&mut self, val: super::vals::ThesholdRatio) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "THRESHOLD_TRACE_EN"]
    #[must_use]
    #[inline(always)]
    pub const fn threshold_trace_en(&self) -> super::vals::ThresholdTraceEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ThresholdTraceEn::from_bits(val as u8)
    }
    #[doc = "THRESHOLD_TRACE_EN"]
    #[inline(always)]
    pub const fn set_threshold_trace_en(&mut self, val: super::vals::ThresholdTraceEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Baseline {
    #[inline(always)]
    fn default() -> Baseline {
        Baseline(0)
    }
}
impl core::fmt::Debug for Baseline {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Baseline")
            .field("tsi_baseline", &self.tsi_baseline())
            .field("base_trace_debounce", &self.base_trace_debounce())
            .field("base_trace_en", &self.base_trace_en())
            .field("theshold_ratio", &self.theshold_ratio())
            .field("threshold_trace_en", &self.threshold_trace_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Baseline {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Baseline {{ tsi_baseline: {=u16:?}, base_trace_debounce: {:?}, base_trace_en: {=bool:?}, theshold_ratio: {:?}, threshold_trace_en: {:?} }}",
            self.tsi_baseline(),
            self.base_trace_debounce(),
            self.base_trace_en(),
            self.theshold_ratio(),
            self.threshold_trace_en()
        )
    }
}
#[doc = "TSI CONFIG Register (TSI_CONFIG) for self-capacitor (CONFIG)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "MODE"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::ConfigMode {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ConfigMode::from_bits(val as u8)
    }
    #[doc = "MODE"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::ConfigMode) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "FILTER_TRIM"]
    #[must_use]
    #[inline(always)]
    pub const fn filter_trim(&self) -> super::vals::FilterTrim {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::FilterTrim::from_bits(val as u8)
    }
    #[doc = "FILTER_TRIM"]
    #[inline(always)]
    pub const fn set_filter_trim(&mut self, val: super::vals::FilterTrim) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "self_bypass_flt"]
    #[must_use]
    #[inline(always)]
    pub const fn self_bypass_flt(&self) -> super::vals::SelfBypassFlt {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::SelfBypassFlt::from_bits(val as u8)
    }
    #[doc = "self_bypass_flt"]
    #[inline(always)]
    pub const fn set_self_bypass_flt(&mut self, val: super::vals::SelfBypassFlt) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "xin_scale_down_trim"]
    #[must_use]
    #[inline(always)]
    pub const fn xin_scale_down_trim(&self) -> super::vals::XinScaleDownTrim {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::XinScaleDownTrim::from_bits(val as u8)
    }
    #[doc = "xin_scale_down_trim"]
    #[inline(always)]
    pub const fn set_xin_scale_down_trim(&mut self, val: super::vals::XinScaleDownTrim) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "xin_scale_down"]
    #[must_use]
    #[inline(always)]
    pub const fn xin_scale_down(&self) -> super::vals::XinScaleDown {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::XinScaleDown::from_bits(val as u8)
    }
    #[doc = "xin_scale_down"]
    #[inline(always)]
    pub const fn set_xin_scale_down(&mut self, val: super::vals::XinScaleDown) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "self_boost_scale"]
    #[must_use]
    #[inline(always)]
    pub const fn self_boost_scale(&self) -> super::vals::SelfBoostScale {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::SelfBoostScale::from_bits(val as u8)
    }
    #[doc = "self_boost_scale"]
    #[inline(always)]
    pub const fn set_self_boost_scale(&mut self, val: super::vals::SelfBoostScale) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "S_NOISE"]
    #[must_use]
    #[inline(always)]
    pub const fn s_noise(&self) -> super::vals::SNoise {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::SNoise::from_bits(val as u8)
    }
    #[doc = "S_NOISE"]
    #[inline(always)]
    pub const fn set_s_noise(&mut self, val: super::vals::SNoise) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "S_XCH"]
    #[must_use]
    #[inline(always)]
    pub const fn s_xch(&self) -> super::vals::SXch {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::SXch::from_bits(val as u8)
    }
    #[doc = "S_XCH"]
    #[inline(always)]
    pub const fn set_s_xch(&mut self, val: super::vals::SXch) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "S_XIN"]
    #[must_use]
    #[inline(always)]
    pub const fn s_xin(&self) -> super::vals::SXin {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::SXin::from_bits(val as u8)
    }
    #[doc = "S_XIN"]
    #[inline(always)]
    pub const fn set_s_xin(&mut self, val: super::vals::SXin) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "S_SEN"]
    #[must_use]
    #[inline(always)]
    pub const fn s_sen(&self) -> super::vals::SSen {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::SSen::from_bits(val as u8)
    }
    #[doc = "S_SEN"]
    #[inline(always)]
    pub const fn set_s_sen(&mut self, val: super::vals::SSen) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "S_XDN"]
    #[must_use]
    #[inline(always)]
    pub const fn s_xdn(&self) -> super::vals::SXdn {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::SXdn::from_bits(val as u8)
    }
    #[doc = "S_XDN"]
    #[inline(always)]
    pub const fn set_s_xdn(&mut self, val: super::vals::SXdn) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "self_boost_mode"]
    #[must_use]
    #[inline(always)]
    pub const fn self_boost_mode(&self) -> super::vals::SelfBoostMode {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SelfBoostMode::from_bits(val as u8)
    }
    #[doc = "self_boost_mode"]
    #[inline(always)]
    pub const fn set_self_boost_mode(&mut self, val: super::vals::SelfBoostMode) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
impl core::fmt::Debug for Config {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Config")
            .field("mode", &self.mode())
            .field("filter_trim", &self.filter_trim())
            .field("self_bypass_flt", &self.self_bypass_flt())
            .field("xin_scale_down_trim", &self.xin_scale_down_trim())
            .field("xin_scale_down", &self.xin_scale_down())
            .field("self_boost_scale", &self.self_boost_scale())
            .field("s_noise", &self.s_noise())
            .field("s_xch", &self.s_xch())
            .field("s_xin", &self.s_xin())
            .field("s_sen", &self.s_sen())
            .field("s_xdn", &self.s_xdn())
            .field("self_boost_mode", &self.self_boost_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Config {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Config {{ mode: {:?}, filter_trim: {:?}, self_bypass_flt: {:?}, xin_scale_down_trim: {:?}, xin_scale_down: {:?}, self_boost_scale: {:?}, s_noise: {:?}, s_xch: {:?}, s_xin: {:?}, s_sen: {:?}, s_xdn: {:?}, self_boost_mode: {:?} }}",
            self.mode(),
            self.filter_trim(),
            self.self_bypass_flt(),
            self.xin_scale_down_trim(),
            self.xin_scale_down(),
            self.self_boost_scale(),
            self.s_noise(),
            self.s_xch(),
            self.s_xin(),
            self.s_sen(),
            self.s_xdn(),
            self.self_boost_mode()
        )
    }
}
#[doc = "TSI CONFIG Register (TSI_CONFIG) for mutual-capacitor (CONFIG_MUTUAL)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConfigMutual(pub u32);
impl ConfigMutual {
    #[doc = "MODE"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::ConfigMutualMode {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ConfigMutualMode::from_bits(val as u8)
    }
    #[doc = "MODE"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::ConfigMutualMode) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "M_NMIRROR"]
    #[must_use]
    #[inline(always)]
    pub const fn m_nmirror(&self) -> super::vals::MNmirror {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::MNmirror::from_bits(val as u8)
    }
    #[doc = "M_NMIRROR"]
    #[inline(always)]
    pub const fn set_m_nmirror(&mut self, val: super::vals::MNmirror) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "M_PMIRRORR"]
    #[must_use]
    #[inline(always)]
    pub const fn m_pmirrorr(&self) -> super::vals::MPmirrorr {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::MPmirrorr::from_bits(val as u8)
    }
    #[doc = "M_PMIRRORR"]
    #[inline(always)]
    pub const fn set_m_pmirrorr(&mut self, val: super::vals::MPmirrorr) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "M_PMIRRORL"]
    #[must_use]
    #[inline(always)]
    pub const fn m_pmirrorl(&self) -> super::vals::MPmirrorl {
        let val = (self.0 >> 5usize) & 0x07;
        super::vals::MPmirrorl::from_bits(val as u8)
    }
    #[doc = "M_PMIRRORL"]
    #[inline(always)]
    pub const fn set_m_pmirrorl(&mut self, val: super::vals::MPmirrorl) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
    }
    #[doc = "M_CNT_EN"]
    #[must_use]
    #[inline(always)]
    pub const fn m_cnt_en(&self) -> super::vals::MCntEn {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::MCntEn::from_bits(val as u8)
    }
    #[doc = "M_CNT_EN"]
    #[inline(always)]
    pub const fn set_m_cnt_en(&mut self, val: super::vals::MCntEn) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "M_TX_PD_EN"]
    #[must_use]
    #[inline(always)]
    pub const fn m_tx_pd_en(&self) -> super::vals::MTxPdEn {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::MTxPdEn::from_bits(val as u8)
    }
    #[doc = "M_TX_PD_EN"]
    #[inline(always)]
    pub const fn set_m_tx_pd_en(&mut self, val: super::vals::MTxPdEn) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "M_SEN_BOOST"]
    #[must_use]
    #[inline(always)]
    pub const fn m_sen_boost(&self) -> super::vals::MSenBoost {
        let val = (self.0 >> 18usize) & 0x1f;
        super::vals::MSenBoost::from_bits(val as u8)
    }
    #[doc = "M_SEN_BOOST"]
    #[inline(always)]
    pub const fn set_m_sen_boost(&mut self, val: super::vals::MSenBoost) {
        self.0 = (self.0 & !(0x1f << 18usize)) | (((val.to_bits() as u32) & 0x1f) << 18usize);
    }
    #[doc = "PRE_CHRG_MODE"]
    #[must_use]
    #[inline(always)]
    pub const fn pre_chrg_mode(&self) -> super::vals::PreChrgMode {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::PreChrgMode::from_bits(val as u8)
    }
    #[doc = "PRE_CHRG_MODE"]
    #[inline(always)]
    pub const fn set_pre_chrg_mode(&mut self, val: super::vals::PreChrgMode) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "M_PRE_RES"]
    #[must_use]
    #[inline(always)]
    pub const fn m_pre_res(&self) -> super::vals::MPreRes {
        let val = (self.0 >> 26usize) & 0x07;
        super::vals::MPreRes::from_bits(val as u8)
    }
    #[doc = "M_PRE_RES"]
    #[inline(always)]
    pub const fn set_m_pre_res(&mut self, val: super::vals::MPreRes) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val.to_bits() as u32) & 0x07) << 26usize);
    }
    #[doc = "M_PRE_CURRENT"]
    #[must_use]
    #[inline(always)]
    pub const fn m_pre_current(&self) -> super::vals::MPreCurrent {
        let val = (self.0 >> 29usize) & 0x07;
        super::vals::MPreCurrent::from_bits(val as u8)
    }
    #[doc = "M_PRE_CURRENT"]
    #[inline(always)]
    pub const fn set_m_pre_current(&mut self, val: super::vals::MPreCurrent) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val.to_bits() as u32) & 0x07) << 29usize);
    }
}
impl Default for ConfigMutual {
    #[inline(always)]
    fn default() -> ConfigMutual {
        ConfigMutual(0)
    }
}
impl core::fmt::Debug for ConfigMutual {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ConfigMutual")
            .field("mode", &self.mode())
            .field("m_nmirror", &self.m_nmirror())
            .field("m_pmirrorr", &self.m_pmirrorr())
            .field("m_pmirrorl", &self.m_pmirrorl())
            .field("m_cnt_en", &self.m_cnt_en())
            .field("m_tx_pd_en", &self.m_tx_pd_en())
            .field("m_sen_boost", &self.m_sen_boost())
            .field("pre_chrg_mode", &self.pre_chrg_mode())
            .field("m_pre_res", &self.m_pre_res())
            .field("m_pre_current", &self.m_pre_current())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ConfigMutual {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ConfigMutual {{ mode: {:?}, m_nmirror: {:?}, m_pmirrorr: {:?}, m_pmirrorl: {:?}, m_cnt_en: {:?}, m_tx_pd_en: {:?}, m_sen_boost: {:?}, pre_chrg_mode: {:?}, m_pre_res: {:?}, m_pre_current: {:?} }}",
            self.mode(),
            self.m_nmirror(),
            self.m_pmirrorr(),
            self.m_pmirrorl(),
            self.m_cnt_en(),
            self.m_tx_pd_en(),
            self.m_sen_boost(),
            self.pre_chrg_mode(),
            self.m_pre_res(),
            self.m_pre_current()
        )
    }
}
#[doc = "TSI Data and Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Data(pub u32);
impl Data {
    #[doc = "TSI Conversion Counter Value"]
    #[must_use]
    #[inline(always)]
    pub const fn tsicnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "TSI Conversion Counter Value"]
    #[inline(always)]
    pub const fn set_tsicnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "TSI Counter Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tsicnt_overflow_flag(&self) -> super::vals::TsicntOverflowFlag {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::TsicntOverflowFlag::from_bits(val as u8)
    }
    #[doc = "TSI Counter Overflow Flag"]
    #[inline(always)]
    pub const fn set_tsicnt_overflow_flag(&mut self, val: super::vals::TsicntOverflowFlag) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "End of Scan Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn eosf(&self) -> super::vals::Eosf {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Eosf::from_bits(val as u8)
    }
    #[doc = "End of Scan Flag"]
    #[inline(always)]
    pub const fn set_eosf(&mut self, val: super::vals::Eosf) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Overrun Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn overrunf(&self) -> super::vals::Overrunf {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Overrunf::from_bits(val as u8)
    }
    #[doc = "Overrun Flag"]
    #[inline(always)]
    pub const fn set_overrunf(&mut self, val: super::vals::Overrunf) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Out of Range Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn outrgf(&self) -> super::vals::Outrgf {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Outrgf::from_bits(val as u8)
    }
    #[doc = "Out of Range Flag"]
    #[inline(always)]
    pub const fn set_outrgf(&mut self, val: super::vals::Outrgf) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "DPD Out of Range Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn dpd_outrgf(&self) -> super::vals::DpdOutrgf {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::DpdOutrgf::from_bits(val as u8)
    }
    #[doc = "DPD Out of Range Flag"]
    #[inline(always)]
    pub const fn set_dpd_outrgf(&mut self, val: super::vals::DpdOutrgf) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Data {
    #[inline(always)]
    fn default() -> Data {
        Data(0)
    }
}
impl core::fmt::Debug for Data {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Data")
            .field("tsicnt", &self.tsicnt())
            .field("tsicnt_overflow_flag", &self.tsicnt_overflow_flag())
            .field("eosf", &self.eosf())
            .field("overrunf", &self.overrunf())
            .field("outrgf", &self.outrgf())
            .field("dpd_outrgf", &self.dpd_outrgf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Data {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Data {{ tsicnt: {=u16:?}, tsicnt_overflow_flag: {:?}, eosf: {:?}, overrunf: {:?}, outrgf: {:?}, dpd_outrgf: {:?} }}",
            self.tsicnt(),
            self.tsicnt_overflow_flag(),
            self.eosf(),
            self.overrunf(),
            self.outrgf(),
            self.dpd_outrgf()
        )
    }
}
#[doc = "TSI General Control and Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gencs(pub u32);
impl Gencs {
    #[doc = "In-progress DMA Transfer Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmaen_eos(&self) -> super::vals::DmaenEos {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::DmaenEos::from_bits(val as u8)
    }
    #[doc = "In-progress DMA Transfer Request Enable"]
    #[inline(always)]
    pub const fn set_dmaen_eos(&mut self, val: super::vals::DmaenEos) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Out of Range DMA Transfer Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmaen_outrg(&self) -> super::vals::DmaenOutrg {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::DmaenOutrg::from_bits(val as u8)
    }
    #[doc = "Out of Range DMA Transfer Request Enable"]
    #[inline(always)]
    pub const fn set_dmaen_outrg(&mut self, val: super::vals::DmaenOutrg) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Scan Trigger Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn stm(&self) -> super::vals::Stm {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Stm::from_bits(val as u8)
    }
    #[doc = "Scan Trigger Mode"]
    #[inline(always)]
    pub const fn set_stm(&mut self, val: super::vals::Stm) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Touch Sensing Input Module Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tsien(&self) -> super::vals::Tsien {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Tsien::from_bits(val as u8)
    }
    #[doc = "Touch Sensing Input Module Enable"]
    #[inline(always)]
    pub const fn set_tsien(&mut self, val: super::vals::Tsien) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "KHz clock selection"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel(&self) -> super::vals::ClkSel {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::ClkSel::from_bits(val as u8)
    }
    #[doc = "KHz clock selection"]
    #[inline(always)]
    pub const fn set_clk_sel(&mut self, val: super::vals::ClkSel) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Software Trigger Start"]
    #[must_use]
    #[inline(always)]
    pub const fn swts(&self) -> super::vals::Swts {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Swts::from_bits(val as u8)
    }
    #[doc = "Software Trigger Start"]
    #[inline(always)]
    pub const fn set_swts(&mut self, val: super::vals::Swts) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Enable of 3v domain logic in this IP"]
    #[must_use]
    #[inline(always)]
    pub const fn tsien_dpd(&self) -> super::vals::TsienDpd {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::TsienDpd::from_bits(val as u8)
    }
    #[doc = "Enable of 3v domain logic in this IP"]
    #[inline(always)]
    pub const fn set_tsien_dpd(&mut self, val: super::vals::TsienDpd) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "DVOLT"]
    #[must_use]
    #[inline(always)]
    pub const fn dvolt(&self) -> super::vals::Dvolt {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Dvolt::from_bits(val as u8)
    }
    #[doc = "DVOLT"]
    #[inline(always)]
    pub const fn set_dvolt(&mut self, val: super::vals::Dvolt) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "DEBOUNCE"]
    #[must_use]
    #[inline(always)]
    pub const fn debounce_counter_configured(&self) -> super::vals::DebounceCounterConfigured {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::DebounceCounterConfigured::from_bits(val as u8)
    }
    #[doc = "DEBOUNCE"]
    #[inline(always)]
    pub const fn set_debounce_counter_configured(
        &mut self,
        val: super::vals::DebounceCounterConfigured,
    ) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "End of Scan Interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn esor(&self) -> super::vals::Esor {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Esor::from_bits(val as u8)
    }
    #[doc = "End of Scan Interrupt enable."]
    #[inline(always)]
    pub const fn set_esor(&mut self, val: super::vals::Esor) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Out-of-range interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn outrg_en(&self) -> super::vals::OutrgEn {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::OutrgEn::from_bits(val as u8)
    }
    #[doc = "Out-of-range interrupt enable."]
    #[inline(always)]
    pub const fn set_outrg_en(&mut self, val: super::vals::OutrgEn) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for Gencs {
    #[inline(always)]
    fn default() -> Gencs {
        Gencs(0)
    }
}
impl core::fmt::Debug for Gencs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gencs")
            .field("dmaen_eos", &self.dmaen_eos())
            .field("dmaen_outrg", &self.dmaen_outrg())
            .field("stm", &self.stm())
            .field("tsien", &self.tsien())
            .field("clk_sel", &self.clk_sel())
            .field("swts", &self.swts())
            .field("tsien_dpd", &self.tsien_dpd())
            .field("dvolt", &self.dvolt())
            .field(
                "debounce_counter_configured",
                &self.debounce_counter_configured(),
            )
            .field("esor", &self.esor())
            .field("outrg_en", &self.outrg_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gencs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gencs {{ dmaen_eos: {:?}, dmaen_outrg: {:?}, stm: {:?}, tsien: {:?}, clk_sel: {:?}, swts: {:?}, tsien_dpd: {:?}, dvolt: {:?}, debounce_counter_configured: {:?}, esor: {:?}, outrg_en: {:?} }}",
            self.dmaen_eos(),
            self.dmaen_outrg(),
            self.stm(),
            self.tsien(),
            self.clk_sel(),
            self.swts(),
            self.tsien_dpd(),
            self.dvolt(),
            self.debounce_counter_configured(),
            self.esor(),
            self.outrg_en()
        )
    }
}
#[doc = "TSI Miscellaneous Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Misc(pub u32);
impl Misc {
    #[doc = "enable the 2500fF caps inside cap bank"]
    #[must_use]
    #[inline(always)]
    pub const fn cap_bank_2500ff(&self) -> super::vals::CapBank2500ff {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::CapBank2500ff::from_bits(val as u8)
    }
    #[doc = "enable the 2500fF caps inside cap bank"]
    #[inline(always)]
    pub const fn set_cap_bank_2500ff(&mut self, val: super::vals::CapBank2500ff) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "enable the 312fF caps inside cap bank"]
    #[must_use]
    #[inline(always)]
    pub const fn cap_bank_312ff(&self) -> super::vals::CapBank312ff {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::CapBank312ff::from_bits(val as u8)
    }
    #[doc = "enable the 312fF caps inside cap bank"]
    #[inline(always)]
    pub const fn set_cap_bank_312ff(&mut self, val: super::vals::CapBank312ff) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "DPD_MODE_ENABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn dpd_mode_enable(&self) -> super::vals::DpdModeEnable {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::DpdModeEnable::from_bits(val as u8)
    }
    #[doc = "DPD_MODE_ENABLE"]
    #[inline(always)]
    pub const fn set_dpd_mode_enable(&mut self, val: super::vals::DpdModeEnable) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "CAP_BANK_148FF"]
    #[must_use]
    #[inline(always)]
    pub const fn cap_bank_148ff(&self) -> super::vals::CapBank148ff {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::CapBank148ff::from_bits(val as u8)
    }
    #[doc = "CAP_BANK_148FF"]
    #[inline(always)]
    pub const fn set_cap_bank_148ff(&mut self, val: super::vals::CapBank148ff) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "Test finger function enable signal"]
    #[must_use]
    #[inline(always)]
    pub const fn cap_bank_en(&self) -> super::vals::CapBankEn {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::CapBankEn::from_bits(val as u8)
    }
    #[doc = "Test finger function enable signal"]
    #[inline(always)]
    pub const fn set_cap_bank_en(&mut self, val: super::vals::CapBankEn) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "TSI Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_divider(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "TSI Clock Divider"]
    #[inline(always)]
    pub const fn set_clk_divider(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "COUNTING_CLK_HIGH"]
    #[must_use]
    #[inline(always)]
    pub const fn counting_clk_high(&self) -> super::vals::CountingClkHigh {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::CountingClkHigh::from_bits(val as u8)
    }
    #[doc = "COUNTING_CLK_HIGH"]
    #[inline(always)]
    pub const fn set_counting_clk_high(&mut self, val: super::vals::CountingClkHigh) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Misc {
    #[inline(always)]
    fn default() -> Misc {
        Misc(0)
    }
}
impl core::fmt::Debug for Misc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Misc")
            .field("cap_bank_2500ff", &self.cap_bank_2500ff())
            .field("cap_bank_312ff", &self.cap_bank_312ff())
            .field("dpd_mode_enable", &self.dpd_mode_enable())
            .field("cap_bank_148ff", &self.cap_bank_148ff())
            .field("cap_bank_en", &self.cap_bank_en())
            .field("clk_divider", &self.clk_divider())
            .field("counting_clk_high", &self.counting_clk_high())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Misc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Misc {{ cap_bank_2500ff: {:?}, cap_bank_312ff: {:?}, dpd_mode_enable: {:?}, cap_bank_148ff: {:?}, cap_bank_en: {:?}, clk_divider: {=u8:?}, counting_clk_high: {:?} }}",
            self.cap_bank_2500ff(),
            self.cap_bank_312ff(),
            self.dpd_mode_enable(),
            self.cap_bank_148ff(),
            self.cap_bank_en(),
            self.clk_divider(),
            self.counting_clk_high()
        )
    }
}
#[doc = "TSI Mutual-cap Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mul(pub u32);
impl Mul {
    #[doc = "M_VPRE_CHOOSE"]
    #[must_use]
    #[inline(always)]
    pub const fn m_vpre_choose(&self) -> super::vals::MVpreChoose {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MVpreChoose::from_bits(val as u8)
    }
    #[doc = "M_VPRE_CHOOSE"]
    #[inline(always)]
    pub const fn set_m_vpre_choose(&mut self, val: super::vals::MVpreChoose) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "M_MODE"]
    #[must_use]
    #[inline(always)]
    pub const fn m_mode(&self) -> super::vals::MMode {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::MMode::from_bits(val as u8)
    }
    #[doc = "M_MODE"]
    #[inline(always)]
    pub const fn set_m_mode(&mut self, val: super::vals::MMode) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "M_TRIM_CAP"]
    #[must_use]
    #[inline(always)]
    pub const fn m_trim_cap(&self) -> super::vals::MTrimCap {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::MTrimCap::from_bits(val as u8)
    }
    #[doc = "M_TRIM_CAP"]
    #[inline(always)]
    pub const fn set_m_trim_cap(&mut self, val: super::vals::MTrimCap) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Not use, just reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn m_trim_used(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Not use, just reserved"]
    #[inline(always)]
    pub const fn set_m_trim_used(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Mul {
    #[inline(always)]
    fn default() -> Mul {
        Mul(0)
    }
}
impl core::fmt::Debug for Mul {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mul")
            .field("m_vpre_choose", &self.m_vpre_choose())
            .field("m_mode", &self.m_mode())
            .field("m_trim_cap", &self.m_trim_cap())
            .field("m_trim_used", &self.m_trim_used())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mul {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mul {{ m_vpre_choose: {:?}, m_mode: {:?}, m_trim_cap: {:?}, m_trim_used: {=u16:?} }}",
            self.m_vpre_choose(),
            self.m_mode(),
            self.m_trim_cap(),
            self.m_trim_used()
        )
    }
}
#[doc = "Mutual RX mode channels selection(CH31~CH0)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MutualRxSel310(pub u32);
impl MutualRxSel310 {
    #[doc = "config tsi channel as mutual RX mode among CH31~CH0"]
    #[must_use]
    #[inline(always)]
    pub const fn as_mutual_rx_31_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "config tsi channel as mutual RX mode among CH31~CH0"]
    #[inline(always)]
    pub const fn set_as_mutual_rx_31_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MutualRxSel310 {
    #[inline(always)]
    fn default() -> MutualRxSel310 {
        MutualRxSel310(0)
    }
}
impl core::fmt::Debug for MutualRxSel310 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MutualRxSel310")
            .field("as_mutual_rx_31_0", &self.as_mutual_rx_31_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MutualRxSel310 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MutualRxSel310 {{ as_mutual_rx_31_0: {=u32:?} }}",
            self.as_mutual_rx_31_0()
        )
    }
}
#[doc = "Mutual RX mode channels selection(CH63~CH32)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MutualRxSel6332(pub u32);
impl MutualRxSel6332 {
    #[doc = "config tsi channel as mutual RX mode among CH63~CH32"]
    #[must_use]
    #[inline(always)]
    pub const fn as_mutual_rx_63_32(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "config tsi channel as mutual RX mode among CH63~CH32"]
    #[inline(always)]
    pub const fn set_as_mutual_rx_63_32(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MutualRxSel6332 {
    #[inline(always)]
    fn default() -> MutualRxSel6332 {
        MutualRxSel6332(0)
    }
}
impl core::fmt::Debug for MutualRxSel6332 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MutualRxSel6332")
            .field("as_mutual_rx_63_32", &self.as_mutual_rx_63_32())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MutualRxSel6332 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MutualRxSel6332 {{ as_mutual_rx_63_32: {=u32:?} }}",
            self.as_mutual_rx_63_32()
        )
    }
}
#[doc = "Mutual RX mode channels selection(CH69~CH64)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MutualRxSel6964(pub u32);
impl MutualRxSel6964 {
    #[doc = "config tsi channel as mutual RX mode among CH69~CH64"]
    #[must_use]
    #[inline(always)]
    pub const fn as_mutual_rx_69_64(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "config tsi channel as mutual RX mode among CH69~CH64"]
    #[inline(always)]
    pub const fn set_as_mutual_rx_69_64(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for MutualRxSel6964 {
    #[inline(always)]
    fn default() -> MutualRxSel6964 {
        MutualRxSel6964(0)
    }
}
impl core::fmt::Debug for MutualRxSel6964 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MutualRxSel6964")
            .field("as_mutual_rx_69_64", &self.as_mutual_rx_69_64())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MutualRxSel6964 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MutualRxSel6964 {{ as_mutual_rx_69_64: {=u8:?} }}",
            self.as_mutual_rx_69_64()
        )
    }
}
#[doc = "Mutual TX mode channels selection(CH31~CH0)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MutualTxSel310(pub u32);
impl MutualTxSel310 {
    #[doc = "config tsi channel as mutual TX mode among CH31~CH0"]
    #[must_use]
    #[inline(always)]
    pub const fn as_mutual_tx_31_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "config tsi channel as mutual TX mode among CH31~CH0"]
    #[inline(always)]
    pub const fn set_as_mutual_tx_31_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MutualTxSel310 {
    #[inline(always)]
    fn default() -> MutualTxSel310 {
        MutualTxSel310(0)
    }
}
impl core::fmt::Debug for MutualTxSel310 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MutualTxSel310")
            .field("as_mutual_tx_31_0", &self.as_mutual_tx_31_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MutualTxSel310 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MutualTxSel310 {{ as_mutual_tx_31_0: {=u32:?} }}",
            self.as_mutual_tx_31_0()
        )
    }
}
#[doc = "Mutual TX mode channels selection(CH63~CH32)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MutualTxSel6332(pub u32);
impl MutualTxSel6332 {
    #[doc = "config tsi channel as mutual TX mode among CH63~CH32"]
    #[must_use]
    #[inline(always)]
    pub const fn as_mutual_tx_63_32(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "config tsi channel as mutual TX mode among CH63~CH32"]
    #[inline(always)]
    pub const fn set_as_mutual_tx_63_32(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MutualTxSel6332 {
    #[inline(always)]
    fn default() -> MutualTxSel6332 {
        MutualTxSel6332(0)
    }
}
impl core::fmt::Debug for MutualTxSel6332 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MutualTxSel6332")
            .field("as_mutual_tx_63_32", &self.as_mutual_tx_63_32())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MutualTxSel6332 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MutualTxSel6332 {{ as_mutual_tx_63_32: {=u32:?} }}",
            self.as_mutual_tx_63_32()
        )
    }
}
#[doc = "Mutual TX mode channels selection(CH69~CH64)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MutualTxSel6964(pub u32);
impl MutualTxSel6964 {
    #[doc = "config tsi channel as mutual TX mode among CH69~CH64"]
    #[must_use]
    #[inline(always)]
    pub const fn as_mutual_tx_69_64(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "config tsi channel as mutual TX mode among CH69~CH64"]
    #[inline(always)]
    pub const fn set_as_mutual_tx_69_64(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for MutualTxSel6964 {
    #[inline(always)]
    fn default() -> MutualTxSel6964 {
        MutualTxSel6964(0)
    }
}
impl core::fmt::Debug for MutualTxSel6964 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MutualTxSel6964")
            .field("as_mutual_tx_69_64", &self.as_mutual_tx_69_64())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MutualTxSel6964 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MutualTxSel6964 {{ as_mutual_tx_69_64: {=u8:?} }}",
            self.as_mutual_tx_69_64()
        )
    }
}
#[doc = "Self-cap mode channels selection(CH31~CH0)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SelfSel310(pub u32);
impl SelfSel310 {
    #[doc = "config tsi channel as self-cap mode among CH31~CH0"]
    #[must_use]
    #[inline(always)]
    pub const fn as_self_31_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "config tsi channel as self-cap mode among CH31~CH0"]
    #[inline(always)]
    pub const fn set_as_self_31_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SelfSel310 {
    #[inline(always)]
    fn default() -> SelfSel310 {
        SelfSel310(0)
    }
}
impl core::fmt::Debug for SelfSel310 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SelfSel310")
            .field("as_self_31_0", &self.as_self_31_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SelfSel310 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SelfSel310 {{ as_self_31_0: {=u32:?} }}",
            self.as_self_31_0()
        )
    }
}
#[doc = "Self-cap mode channels selection(CH63~CH32)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SelfSel6332(pub u32);
impl SelfSel6332 {
    #[doc = "config tsi channel as self-cap mode among CH63~CH32"]
    #[must_use]
    #[inline(always)]
    pub const fn as_self_63_32(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "config tsi channel as self-cap mode among CH63~CH32"]
    #[inline(always)]
    pub const fn set_as_self_63_32(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SelfSel6332 {
    #[inline(always)]
    fn default() -> SelfSel6332 {
        SelfSel6332(0)
    }
}
impl core::fmt::Debug for SelfSel6332 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SelfSel6332")
            .field("as_self_63_32", &self.as_self_63_32())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SelfSel6332 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SelfSel6332 {{ as_self_63_32: {=u32:?} }}",
            self.as_self_63_32()
        )
    }
}
#[doc = "Self-cap mode channels selection(CH69~CH64)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SelfSel6964(pub u32);
impl SelfSel6964 {
    #[doc = "config tsi channel as self-cap mode among CH69~CH64"]
    #[must_use]
    #[inline(always)]
    pub const fn as_self_69_64(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "config tsi channel as self-cap mode among CH69~CH64"]
    #[inline(always)]
    pub const fn set_as_self_69_64(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for SelfSel6964 {
    #[inline(always)]
    fn default() -> SelfSel6964 {
        SelfSel6964(0)
    }
}
impl core::fmt::Debug for SelfSel6964 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SelfSel6964")
            .field("as_self_69_64", &self.as_self_69_64())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SelfSel6964 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SelfSel6964 {{ as_self_69_64: {=u8:?} }}",
            self.as_self_69_64()
        )
    }
}
#[doc = "TSI Shield Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shield(pub u32);
impl Shield {
    #[doc = "M_SEN_RES"]
    #[must_use]
    #[inline(always)]
    pub const fn m_sen_res(&self) -> super::vals::MSenRes {
        let val = (self.0 >> 25usize) & 0x3f;
        super::vals::MSenRes::from_bits(val as u8)
    }
    #[doc = "M_SEN_RES"]
    #[inline(always)]
    pub const fn set_m_sen_res(&mut self, val: super::vals::MSenRes) {
        self.0 = (self.0 & !(0x3f << 25usize)) | (((val.to_bits() as u32) & 0x3f) << 25usize);
    }
}
impl Default for Shield {
    #[inline(always)]
    fn default() -> Shield {
        Shield(0)
    }
}
impl core::fmt::Debug for Shield {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shield")
            .field("m_sen_res", &self.m_sen_res())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shield {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Shield {{ m_sen_res: {:?} }}", self.m_sen_res())
    }
}
#[doc = "Shield mode channels selection(CH31~CH0)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ShieldSel310(pub u32);
impl ShieldSel310 {
    #[doc = "config tsi channel as shield mode among CH31~CH0"]
    #[must_use]
    #[inline(always)]
    pub const fn as_shield_31_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "config tsi channel as shield mode among CH31~CH0"]
    #[inline(always)]
    pub const fn set_as_shield_31_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ShieldSel310 {
    #[inline(always)]
    fn default() -> ShieldSel310 {
        ShieldSel310(0)
    }
}
impl core::fmt::Debug for ShieldSel310 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ShieldSel310")
            .field("as_shield_31_0", &self.as_shield_31_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ShieldSel310 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ShieldSel310 {{ as_shield_31_0: {=u32:?} }}",
            self.as_shield_31_0()
        )
    }
}
#[doc = "Shield mode channels selection(CH63~CH32)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ShieldSel6332(pub u32);
impl ShieldSel6332 {
    #[doc = "config tsi channel as shield mode among CH63~CH32"]
    #[must_use]
    #[inline(always)]
    pub const fn as_shield_63_32(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "config tsi channel as shield mode among CH63~CH32"]
    #[inline(always)]
    pub const fn set_as_shield_63_32(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ShieldSel6332 {
    #[inline(always)]
    fn default() -> ShieldSel6332 {
        ShieldSel6332(0)
    }
}
impl core::fmt::Debug for ShieldSel6332 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ShieldSel6332")
            .field("as_shield_63_32", &self.as_shield_63_32())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ShieldSel6332 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ShieldSel6332 {{ as_shield_63_32: {=u32:?} }}",
            self.as_shield_63_32()
        )
    }
}
#[doc = "Shield mode channels selection(CH69~CH64)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ShieldSel6964(pub u32);
impl ShieldSel6964 {
    #[doc = "config tsi channel as shield mode among CH69~CH64"]
    #[must_use]
    #[inline(always)]
    pub const fn as_shield_69_64(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "config tsi channel as shield mode among CH69~CH64"]
    #[inline(always)]
    pub const fn set_as_shield_69_64(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for ShieldSel6964 {
    #[inline(always)]
    fn default() -> ShieldSel6964 {
        ShieldSel6964(0)
    }
}
impl core::fmt::Debug for ShieldSel6964 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ShieldSel6964")
            .field("as_shield_69_64", &self.as_shield_69_64())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ShieldSel6964 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ShieldSel6964 {{ as_shield_69_64: {=u8:?} }}",
            self.as_shield_69_64()
        )
    }
}
#[doc = "TSI SINC filter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sinc(pub u32);
impl Sinc {
    #[doc = "SSC_CONTROL_OUT"]
    #[must_use]
    #[inline(always)]
    pub const fn ssc_control_out(&self) -> super::vals::SscControlOut {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SscControlOut::from_bits(val as u8)
    }
    #[doc = "SSC_CONTROL_OUT"]
    #[inline(always)]
    pub const fn set_ssc_control_out(&mut self, val: super::vals::SscControlOut) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "SINC_VALID"]
    #[must_use]
    #[inline(always)]
    pub const fn sinc_valid(&self) -> super::vals::SincValid {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SincValid::from_bits(val as u8)
    }
    #[doc = "SINC_VALID"]
    #[inline(always)]
    pub const fn set_sinc_valid(&mut self, val: super::vals::SincValid) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "SINC_OVERFLOW_FLAG"]
    #[must_use]
    #[inline(always)]
    pub const fn sinc_overflow_flag(&self) -> super::vals::SincOverflowFlag {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SincOverflowFlag::from_bits(val as u8)
    }
    #[doc = "SINC_OVERFLOW_FLAG"]
    #[inline(always)]
    pub const fn set_sinc_overflow_flag(&mut self, val: super::vals::SincOverflowFlag) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "SWITCH_ENABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn switch_enable(&self) -> super::vals::SwitchEnable {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SwitchEnable::from_bits(val as u8)
    }
    #[doc = "SWITCH_ENABLE"]
    #[inline(always)]
    pub const fn set_switch_enable(&mut self, val: super::vals::SwitchEnable) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "SINC_DECIMATION"]
    #[must_use]
    #[inline(always)]
    pub const fn sinc_decimation(&self) -> super::vals::SincDecimation {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::SincDecimation::from_bits(val as u8)
    }
    #[doc = "SINC_DECIMATION"]
    #[inline(always)]
    pub const fn set_sinc_decimation(&mut self, val: super::vals::SincDecimation) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "SINC_ORDER"]
    #[must_use]
    #[inline(always)]
    pub const fn sinc_order(&self) -> super::vals::SincOrder {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::SincOrder::from_bits(val as u8)
    }
    #[doc = "SINC_ORDER"]
    #[inline(always)]
    pub const fn set_sinc_order(&mut self, val: super::vals::SincOrder) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "SINC_CUTOFF"]
    #[must_use]
    #[inline(always)]
    pub const fn sinc_cutoff(&self) -> super::vals::SincCutoff {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::SincCutoff::from_bits(val as u8)
    }
    #[doc = "SINC_CUTOFF"]
    #[inline(always)]
    pub const fn set_sinc_cutoff(&mut self, val: super::vals::SincCutoff) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Sinc {
    #[inline(always)]
    fn default() -> Sinc {
        Sinc(0)
    }
}
impl core::fmt::Debug for Sinc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sinc")
            .field("ssc_control_out", &self.ssc_control_out())
            .field("sinc_valid", &self.sinc_valid())
            .field("sinc_overflow_flag", &self.sinc_overflow_flag())
            .field("switch_enable", &self.switch_enable())
            .field("sinc_decimation", &self.sinc_decimation())
            .field("sinc_order", &self.sinc_order())
            .field("sinc_cutoff", &self.sinc_cutoff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sinc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sinc {{ ssc_control_out: {:?}, sinc_valid: {:?}, sinc_overflow_flag: {:?}, switch_enable: {:?}, sinc_decimation: {:?}, sinc_order: {:?}, sinc_cutoff: {:?} }}",
            self.ssc_control_out(),
            self.sinc_valid(),
            self.sinc_overflow_flag(),
            self.switch_enable(),
            self.sinc_decimation(),
            self.sinc_order(),
            self.sinc_cutoff()
        )
    }
}
#[doc = "TSI SSC Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssc0(pub u32);
impl Ssc0 {
    #[doc = "SSC_PRESCALE_NUM"]
    #[must_use]
    #[inline(always)]
    pub const fn ssc_prescale_num(&self) -> super::vals::SscPrescaleNum {
        let val = (self.0 >> 0usize) & 0x0fff;
        super::vals::SscPrescaleNum::from_bits(val as u16)
    }
    #[doc = "SSC_PRESCALE_NUM"]
    #[inline(always)]
    pub const fn set_ssc_prescale_num(&mut self, val: super::vals::SscPrescaleNum) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val.to_bits() as u32) & 0x0fff) << 0usize);
    }
    #[doc = "BASE_NOCHARGE_NUM"]
    #[must_use]
    #[inline(always)]
    pub const fn base_nocharge_num(&self) -> super::vals::BaseNochargeNum {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::BaseNochargeNum::from_bits(val as u8)
    }
    #[doc = "BASE_NOCHARGE_NUM"]
    #[inline(always)]
    pub const fn set_base_nocharge_num(&mut self, val: super::vals::BaseNochargeNum) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "CHARGE_NUM"]
    #[must_use]
    #[inline(always)]
    pub const fn charge_num(&self) -> super::vals::ChargeNum {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::ChargeNum::from_bits(val as u8)
    }
    #[doc = "CHARGE_NUM"]
    #[inline(always)]
    pub const fn set_charge_num(&mut self, val: super::vals::ChargeNum) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "SSC_CONTROL_REVERSE"]
    #[must_use]
    #[inline(always)]
    pub const fn ssc_control_reverse(&self) -> super::vals::SscControlReverse {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::SscControlReverse::from_bits(val as u8)
    }
    #[doc = "SSC_CONTROL_REVERSE"]
    #[inline(always)]
    pub const fn set_ssc_control_reverse(&mut self, val: super::vals::SscControlReverse) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "SSC_MODE"]
    #[must_use]
    #[inline(always)]
    pub const fn ssc_mode(&self) -> super::vals::SscMode {
        let val = (self.0 >> 25usize) & 0x03;
        super::vals::SscMode::from_bits(val as u8)
    }
    #[doc = "SSC_MODE"]
    #[inline(always)]
    pub const fn set_ssc_mode(&mut self, val: super::vals::SscMode) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
    #[doc = "PRBS_OUTSEL"]
    #[must_use]
    #[inline(always)]
    pub const fn prbs_outsel(&self) -> super::vals::PrbsOutsel {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::PrbsOutsel::from_bits(val as u8)
    }
    #[doc = "PRBS_OUTSEL"]
    #[inline(always)]
    pub const fn set_prbs_outsel(&mut self, val: super::vals::PrbsOutsel) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Ssc0 {
    #[inline(always)]
    fn default() -> Ssc0 {
        Ssc0(0)
    }
}
impl core::fmt::Debug for Ssc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ssc0")
            .field("ssc_prescale_num", &self.ssc_prescale_num())
            .field("base_nocharge_num", &self.base_nocharge_num())
            .field("charge_num", &self.charge_num())
            .field("ssc_control_reverse", &self.ssc_control_reverse())
            .field("ssc_mode", &self.ssc_mode())
            .field("prbs_outsel", &self.prbs_outsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ssc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ssc0 {{ ssc_prescale_num: {:?}, base_nocharge_num: {:?}, charge_num: {:?}, ssc_control_reverse: {:?}, ssc_mode: {:?}, prbs_outsel: {:?} }}",
            self.ssc_prescale_num(),
            self.base_nocharge_num(),
            self.charge_num(),
            self.ssc_control_reverse(),
            self.ssc_mode(),
            self.prbs_outsel()
        )
    }
}
#[doc = "TSI SSC Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssc1(pub u32);
impl Ssc1 {
    #[doc = "PRBS_SEED_LO"]
    #[must_use]
    #[inline(always)]
    pub const fn prbs_seed_lo(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "PRBS_SEED_LO"]
    #[inline(always)]
    pub const fn set_prbs_seed_lo(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "PRBS_SEED_HI"]
    #[must_use]
    #[inline(always)]
    pub const fn prbs_seed_hi(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "PRBS_SEED_HI"]
    #[inline(always)]
    pub const fn set_prbs_seed_hi(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "PRBS_WEIGHT_LO"]
    #[must_use]
    #[inline(always)]
    pub const fn prbs_weight_lo(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "PRBS_WEIGHT_LO"]
    #[inline(always)]
    pub const fn set_prbs_weight_lo(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "PRBS_WEIGHT_HI"]
    #[must_use]
    #[inline(always)]
    pub const fn prbs_weight_hi(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "PRBS_WEIGHT_HI"]
    #[inline(always)]
    pub const fn set_prbs_weight_hi(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ssc1 {
    #[inline(always)]
    fn default() -> Ssc1 {
        Ssc1(0)
    }
}
impl core::fmt::Debug for Ssc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ssc1")
            .field("prbs_seed_lo", &self.prbs_seed_lo())
            .field("prbs_seed_hi", &self.prbs_seed_hi())
            .field("prbs_weight_lo", &self.prbs_weight_lo())
            .field("prbs_weight_hi", &self.prbs_weight_hi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ssc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ssc1 {{ prbs_seed_lo: {=u8:?}, prbs_seed_hi: {=u8:?}, prbs_weight_lo: {=u8:?}, prbs_weight_hi: {=u8:?} }}",
            self.prbs_seed_lo(),
            self.prbs_seed_hi(),
            self.prbs_weight_lo(),
            self.prbs_weight_hi()
        )
    }
}
#[doc = "TSI SSC Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssc2(pub u32);
impl Ssc2 {
    #[doc = "MOVE_REPEAT_NUM"]
    #[must_use]
    #[inline(always)]
    pub const fn move_repeat_num(&self) -> super::vals::MoveRepeatNum {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::MoveRepeatNum::from_bits(val as u8)
    }
    #[doc = "MOVE_REPEAT_NUM"]
    #[inline(always)]
    pub const fn set_move_repeat_num(&mut self, val: super::vals::MoveRepeatNum) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "MOVE_STEPS_NUM"]
    #[must_use]
    #[inline(always)]
    pub const fn move_steps_num(&self) -> super::vals::MoveStepsNum {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::MoveStepsNum::from_bits(val as u8)
    }
    #[doc = "MOVE_STEPS_NUM"]
    #[inline(always)]
    pub const fn set_move_steps_num(&mut self, val: super::vals::MoveStepsNum) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "MOVE_NOCHARGE_MAX"]
    #[must_use]
    #[inline(always)]
    pub const fn move_nocharge_max(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "MOVE_NOCHARGE_MAX"]
    #[inline(always)]
    pub const fn set_move_nocharge_max(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "MOVE_NOCHARGE_MIN"]
    #[must_use]
    #[inline(always)]
    pub const fn move_nocharge_min(&self) -> super::vals::MoveNochargeMin {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::MoveNochargeMin::from_bits(val as u8)
    }
    #[doc = "MOVE_NOCHARGE_MIN"]
    #[inline(always)]
    pub const fn set_move_nocharge_min(&mut self, val: super::vals::MoveNochargeMin) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Ssc2 {
    #[inline(always)]
    fn default() -> Ssc2 {
        Ssc2(0)
    }
}
impl core::fmt::Debug for Ssc2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ssc2")
            .field("move_repeat_num", &self.move_repeat_num())
            .field("move_steps_num", &self.move_steps_num())
            .field("move_nocharge_max", &self.move_nocharge_max())
            .field("move_nocharge_min", &self.move_nocharge_min())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ssc2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ssc2 {{ move_repeat_num: {:?}, move_steps_num: {:?}, move_nocharge_max: {=u8:?}, move_nocharge_min: {:?} }}",
            self.move_repeat_num(),
            self.move_steps_num(),
            self.move_nocharge_max(),
            self.move_nocharge_min()
        )
    }
}
#[doc = "TSI TEST Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Test(pub u32);
impl Test {
    #[doc = "TSI_TEST_EN"]
    #[must_use]
    #[inline(always)]
    pub const fn tsi_test_en(&self) -> super::vals::TsiTestEn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::TsiTestEn::from_bits(val as u8)
    }
    #[doc = "TSI_TEST_EN"]
    #[inline(always)]
    pub const fn set_tsi_test_en(&mut self, val: super::vals::TsiTestEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "DPD_TEST_EN"]
    #[must_use]
    #[inline(always)]
    pub const fn dpd_test_en(&self) -> super::vals::DpdTestEn {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::DpdTestEn::from_bits(val as u8)
    }
    #[doc = "DPD_TEST_EN"]
    #[inline(always)]
    pub const fn set_dpd_test_en(&mut self, val: super::vals::DpdTestEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "End of Scan Flag in DPD test mode"]
    #[must_use]
    #[inline(always)]
    pub const fn dpd_test_eosf(&self) -> super::vals::DpdTestEosf {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::DpdTestEosf::from_bits(val as u8)
    }
    #[doc = "End of Scan Flag in DPD test mode"]
    #[inline(always)]
    pub const fn set_dpd_test_eosf(&mut self, val: super::vals::DpdTestEosf) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "TSI_ANA_TEST_SETTING"]
    #[must_use]
    #[inline(always)]
    pub const fn tsi_ana_test_setting(&self) -> super::vals::TsiAnaTestSetting {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::TsiAnaTestSetting::from_bits(val as u8)
    }
    #[doc = "TSI_ANA_TEST_SETTING"]
    #[inline(always)]
    pub const fn set_tsi_ana_test_setting(&mut self, val: super::vals::TsiAnaTestSetting) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "CLKSOC_SEL"]
    #[must_use]
    #[inline(always)]
    pub const fn clksoc_sel(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "CLKSOC_SEL"]
    #[inline(always)]
    pub const fn set_clksoc_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "RUN_CTRL"]
    #[must_use]
    #[inline(always)]
    pub const fn run_ctrl(&self) -> super::vals::RunCtrl {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::RunCtrl::from_bits(val as u8)
    }
    #[doc = "RUN_CTRL"]
    #[inline(always)]
    pub const fn set_run_ctrl(&mut self, val: super::vals::RunCtrl) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
}
impl Default for Test {
    #[inline(always)]
    fn default() -> Test {
        Test(0)
    }
}
impl core::fmt::Debug for Test {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Test")
            .field("tsi_test_en", &self.tsi_test_en())
            .field("dpd_test_en", &self.dpd_test_en())
            .field("dpd_test_eosf", &self.dpd_test_eosf())
            .field("tsi_ana_test_setting", &self.tsi_ana_test_setting())
            .field("clksoc_sel", &self.clksoc_sel())
            .field("run_ctrl", &self.run_ctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Test {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Test {{ tsi_test_en: {:?}, dpd_test_en: {:?}, dpd_test_eosf: {:?}, tsi_ana_test_setting: {:?}, clksoc_sel: {=bool:?}, run_ctrl: {:?} }}",
            self.tsi_test_en(),
            self.dpd_test_en(),
            self.dpd_test_eosf(),
            self.tsi_ana_test_setting(),
            self.clksoc_sel(),
            self.run_ctrl()
        )
    }
}
#[doc = "TSI AUTO TRIG Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig(pub u32);
impl Trig {
    #[doc = "TRIG_PERIOD_COUNTER"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_period_counter(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "TRIG_PERIOD_COUNTER"]
    #[inline(always)]
    pub const fn set_trig_period_counter(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
    #[doc = "TRIG_CLK_DIVIDER"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_clk_divider(&self) -> super::vals::TrigClkDivider {
        let val = (self.0 >> 24usize) & 0x1f;
        super::vals::TrigClkDivider::from_bits(val as u8)
    }
    #[doc = "TRIG_CLK_DIVIDER"]
    #[inline(always)]
    pub const fn set_trig_clk_divider(&mut self, val: super::vals::TrigClkDivider) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val.to_bits() as u32) & 0x1f) << 24usize);
    }
    #[doc = "TRIG_EN"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_en(&self) -> super::vals::TrigEn {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::TrigEn::from_bits(val as u8)
    }
    #[doc = "TRIG_EN"]
    #[inline(always)]
    pub const fn set_trig_en(&mut self, val: super::vals::TrigEn) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "TRIG_CLK_SEL"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_clk_sel(&self) -> super::vals::TrigClkSel {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::TrigClkSel::from_bits(val as u8)
    }
    #[doc = "TRIG_CLK_SEL"]
    #[inline(always)]
    pub const fn set_trig_clk_sel(&mut self, val: super::vals::TrigClkSel) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Trig {
    #[inline(always)]
    fn default() -> Trig {
        Trig(0)
    }
}
impl core::fmt::Debug for Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig")
            .field("trig_period_counter", &self.trig_period_counter())
            .field("trig_clk_divider", &self.trig_clk_divider())
            .field("trig_en", &self.trig_en())
            .field("trig_clk_sel", &self.trig_clk_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig {{ trig_period_counter: {=u32:?}, trig_clk_divider: {:?}, trig_en: {:?}, trig_clk_sel: {:?} }}",
            self.trig_period_counter(),
            self.trig_clk_divider(),
            self.trig_en(),
            self.trig_clk_sel()
        )
    }
}
#[doc = "TSI Threshold Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tshd(pub u32);
impl Tshd {
    #[doc = "TSI Wakeup Channel Low-threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn thresl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "TSI Wakeup Channel Low-threshold"]
    #[inline(always)]
    pub const fn set_thresl(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "TSI Wakeup Channel High-threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn thresh(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "TSI Wakeup Channel High-threshold"]
    #[inline(always)]
    pub const fn set_thresh(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Tshd {
    #[inline(always)]
    fn default() -> Tshd {
        Tshd(0)
    }
}
impl core::fmt::Debug for Tshd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tshd")
            .field("thresl", &self.thresl())
            .field("thresh", &self.thresh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tshd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tshd {{ thresl: {=u16:?}, thresh: {=u16:?} }}",
            self.thresl(),
            self.thresh()
        )
    }
}
