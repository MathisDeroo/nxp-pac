#[doc = "Active Power Mode Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ActiveCfg(pub u32);
impl ActiveCfg {
    #[doc = "LDO_CORE VDD Drive Strength"]
    #[must_use]
    #[inline(always)]
    pub const fn coreldo_vdd_ds(&self) -> super::vals::ActiveCfgCoreldoVddDs {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ActiveCfgCoreldoVddDs::from_bits(val as u8)
    }
    #[doc = "LDO_CORE VDD Drive Strength"]
    #[inline(always)]
    pub const fn set_coreldo_vdd_ds(&mut self, val: super::vals::ActiveCfgCoreldoVddDs) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "LDO_CORE VDD Regulator Voltage Level"]
    #[must_use]
    #[inline(always)]
    pub const fn coreldo_vdd_lvl(&self) -> super::vals::ActiveCfgCoreldoVddLvl {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::ActiveCfgCoreldoVddLvl::from_bits(val as u8)
    }
    #[doc = "LDO_CORE VDD Regulator Voltage Level"]
    #[inline(always)]
    pub const fn set_coreldo_vdd_lvl(&mut self, val: super::vals::ActiveCfgCoreldoVddLvl) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Glitch Detect Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn glitch_detect_disable(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Glitch Detect Disable"]
    #[inline(always)]
    pub const fn set_glitch_detect_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CMP Bandgap Buffer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lpbuff_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "CMP Bandgap Buffer Enable"]
    #[inline(always)]
    pub const fn set_lpbuff_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Bandgap Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn bgmode(&self) -> super::vals::ActiveCfgBgmode {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::ActiveCfgBgmode::from_bits(val as u8)
    }
    #[doc = "Bandgap Mode"]
    #[inline(always)]
    pub const fn set_bgmode(&mut self, val: super::vals::ActiveCfgBgmode) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "VDD Voltage Detect Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd_vd_disable(&self) -> super::vals::VddVdDisable {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::VddVdDisable::from_bits(val as u8)
    }
    #[doc = "VDD Voltage Detect Disable"]
    #[inline(always)]
    pub const fn set_vdd_vd_disable(&mut self, val: super::vals::VddVdDisable) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Core Low-Voltage Detection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn core_lvde(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Core Low-Voltage Detection Enable"]
    #[inline(always)]
    pub const fn set_core_lvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "System Low-Voltage Detection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_lvde(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "System Low-Voltage Detection Enable"]
    #[inline(always)]
    pub const fn set_sys_lvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Core High-Voltage Detection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn core_hvde(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Core High-Voltage Detection Enable"]
    #[inline(always)]
    pub const fn set_core_hvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "System High-Voltage Detection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_hvde(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "System High-Voltage Detection Enable"]
    #[inline(always)]
    pub const fn set_sys_hvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for ActiveCfg {
    #[inline(always)]
    fn default() -> ActiveCfg {
        ActiveCfg(0)
    }
}
impl core::fmt::Debug for ActiveCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ActiveCfg")
            .field("coreldo_vdd_ds", &self.coreldo_vdd_ds())
            .field("coreldo_vdd_lvl", &self.coreldo_vdd_lvl())
            .field("glitch_detect_disable", &self.glitch_detect_disable())
            .field("lpbuff_en", &self.lpbuff_en())
            .field("bgmode", &self.bgmode())
            .field("vdd_vd_disable", &self.vdd_vd_disable())
            .field("core_lvde", &self.core_lvde())
            .field("sys_lvde", &self.sys_lvde())
            .field("core_hvde", &self.core_hvde())
            .field("sys_hvde", &self.sys_hvde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ActiveCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ActiveCfg {{ coreldo_vdd_ds: {:?}, coreldo_vdd_lvl: {:?}, glitch_detect_disable: {=bool:?}, lpbuff_en: {=bool:?}, bgmode: {:?}, vdd_vd_disable: {:?}, core_lvde: {=bool:?}, sys_lvde: {=bool:?}, core_hvde: {=bool:?}, sys_hvde: {=bool:?} }}",
            self.coreldo_vdd_ds(),
            self.coreldo_vdd_lvl(),
            self.glitch_detect_disable(),
            self.lpbuff_en(),
            self.bgmode(),
            self.vdd_vd_disable(),
            self.core_lvde(),
            self.sys_lvde(),
            self.core_hvde(),
            self.sys_hvde()
        )
    }
}
#[doc = "Active Power Mode Configuration 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ActiveCfg1(pub u32);
impl ActiveCfg1 {
    #[doc = "Active Config Chip Control"]
    #[must_use]
    #[inline(always)]
    pub const fn soc_cntrl(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Active Config Chip Control"]
    #[inline(always)]
    pub const fn set_soc_cntrl(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ActiveCfg1 {
    #[inline(always)]
    fn default() -> ActiveCfg1 {
        ActiveCfg1(0)
    }
}
impl core::fmt::Debug for ActiveCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ActiveCfg1")
            .field("soc_cntrl", &self.soc_cntrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ActiveCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ActiveCfg1 {{ soc_cntrl: {=u32:?} }}", self.soc_cntrl())
    }
}
#[doc = "Active Voltage Trim Delay"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ActiveVdelay(pub u32);
impl ActiveVdelay {
    #[doc = "Active Voltage Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn active_vdelay(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Active Voltage Delay"]
    #[inline(always)]
    pub const fn set_active_vdelay(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for ActiveVdelay {
    #[inline(always)]
    fn default() -> ActiveVdelay {
        ActiveVdelay(0)
    }
}
impl core::fmt::Debug for ActiveVdelay {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ActiveVdelay")
            .field("active_vdelay", &self.active_vdelay())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ActiveVdelay {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ActiveVdelay {{ active_vdelay: {=u16:?} }}",
            self.active_vdelay()
        )
    }
}
#[doc = "SPC Regulator Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cntrl(pub u32);
impl Cntrl {
    #[doc = "LDO_CORE Regulator Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn coreldo_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LDO_CORE Regulator Enable"]
    #[inline(always)]
    pub const fn set_coreldo_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Cntrl {
    #[inline(always)]
    fn default() -> Cntrl {
        Cntrl(0)
    }
}
impl core::fmt::Debug for Cntrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cntrl")
            .field("coreldo_en", &self.coreldo_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cntrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cntrl {{ coreldo_en: {=bool:?} }}", self.coreldo_en())
    }
}
#[doc = "LDO_CORE Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoreldoCfg(pub u32);
impl CoreldoCfg {
    #[doc = "CORELDO_SPARE"]
    #[must_use]
    #[inline(always)]
    pub const fn coreldo_spare(&self) -> super::vals::CoreldoSpare {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::CoreldoSpare::from_bits(val as u8)
    }
    #[doc = "CORELDO_SPARE"]
    #[inline(always)]
    pub const fn set_coreldo_spare(&mut self, val: super::vals::CoreldoSpare) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "LDO_CORE Deep Power Down Pulldown Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dpdown_pulldown_disable(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "LDO_CORE Deep Power Down Pulldown Disable"]
    #[inline(always)]
    pub const fn set_dpdown_pulldown_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for CoreldoCfg {
    #[inline(always)]
    fn default() -> CoreldoCfg {
        CoreldoCfg(0)
    }
}
impl core::fmt::Debug for CoreldoCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CoreldoCfg")
            .field("coreldo_spare", &self.coreldo_spare())
            .field("dpdown_pulldown_disable", &self.dpdown_pulldown_disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CoreldoCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CoreldoCfg {{ coreldo_spare: {:?}, dpdown_pulldown_disable: {=bool:?} }}",
            self.coreldo_spare(),
            self.dpdown_pulldown_disable()
        )
    }
}
#[doc = "External Voltage Domain Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvdCfg(pub u32);
impl EvdCfg {
    #[doc = "External Voltage Domain Isolation"]
    #[must_use]
    #[inline(always)]
    pub const fn evdiso(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "External Voltage Domain Isolation"]
    #[inline(always)]
    pub const fn set_evdiso(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "External Voltage Domain Low-Power Isolation"]
    #[must_use]
    #[inline(always)]
    pub const fn evdlpiso(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "External Voltage Domain Low-Power Isolation"]
    #[inline(always)]
    pub const fn set_evdlpiso(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "External Voltage Domain Status"]
    #[must_use]
    #[inline(always)]
    pub const fn evdstat(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "External Voltage Domain Status"]
    #[inline(always)]
    pub const fn set_evdstat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
}
impl Default for EvdCfg {
    #[inline(always)]
    fn default() -> EvdCfg {
        EvdCfg(0)
    }
}
impl core::fmt::Debug for EvdCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EvdCfg")
            .field("evdiso", &self.evdiso())
            .field("evdlpiso", &self.evdlpiso())
            .field("evdstat", &self.evdstat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EvdCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EvdCfg {{ evdiso: {=u8:?}, evdlpiso: {=u8:?}, evdstat: {=u8:?} }}",
            self.evdiso(),
            self.evdlpiso(),
            self.evdstat()
        )
    }
}
#[doc = "Glitch Detect Status Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlitchDetectSc(pub u32);
impl GlitchDetectSc {
    #[doc = "Counter Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt_select(&self) -> super::vals::GlitchDetectScCntSelect {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::GlitchDetectScCntSelect::from_bits(val as u8)
    }
    #[doc = "Counter Select"]
    #[inline(always)]
    pub const fn set_cnt_select(&mut self, val: super::vals::GlitchDetectScCntSelect) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn timeout(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[doc = "Timeout"]
    #[inline(always)]
    pub const fn set_timeout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[doc = "Glitch Detect Reset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn re(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Glitch Detect Reset Enable"]
    #[inline(always)]
    pub const fn set_re(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Glitch Detect Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Glitch Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "GLITCH_DETECT_FLAG"]
    #[must_use]
    #[inline(always)]
    pub const fn glitch_detect_flag(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "GLITCH_DETECT_FLAG"]
    #[inline(always)]
    pub const fn set_glitch_detect_flag(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Glitch Detect Reset Enable Lock Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Glitch Detect Reset Enable Lock Bit"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "High Gain Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn high_gain_mode(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "High Gain Mode Enable"]
    #[inline(always)]
    pub const fn set_high_gain_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "HVDD AMP DISABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn hvdd_amp_disable(&self) -> super::vals::HvddAmpDisable {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::HvddAmpDisable::from_bits(val as u8)
    }
    #[doc = "HVDD AMP DISABLE"]
    #[inline(always)]
    pub const fn set_hvdd_amp_disable(&mut self, val: super::vals::HvddAmpDisable) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
}
impl Default for GlitchDetectSc {
    #[inline(always)]
    fn default() -> GlitchDetectSc {
        GlitchDetectSc(0)
    }
}
impl core::fmt::Debug for GlitchDetectSc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlitchDetectSc")
            .field("cnt_select", &self.cnt_select())
            .field("timeout", &self.timeout())
            .field("re", &self.re())
            .field("ie", &self.ie())
            .field("glitch_detect_flag", &self.glitch_detect_flag())
            .field("lock", &self.lock())
            .field("high_gain_mode", &self.high_gain_mode())
            .field("hvdd_amp_disable", &self.hvdd_amp_disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlitchDetectSc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlitchDetectSc {{ cnt_select: {:?}, timeout: {=u8:?}, re: {=bool:?}, ie: {=bool:?}, glitch_detect_flag: {=u8:?}, lock: {=bool:?}, high_gain_mode: {=bool:?}, hvdd_amp_disable: {:?} }}",
            self.cnt_select(),
            self.timeout(),
            self.re(),
            self.ie(),
            self.glitch_detect_flag(),
            self.lock(),
            self.high_gain_mode(),
            self.hvdd_amp_disable()
        )
    }
}
#[doc = "VDDA Glitch Detect Status Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlitchVddaDetectSc(pub u32);
impl GlitchVddaDetectSc {
    #[doc = "Counter Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt_select(&self) -> super::vals::GlitchVddaDetectScCntSelect {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::GlitchVddaDetectScCntSelect::from_bits(val as u8)
    }
    #[doc = "Counter Select"]
    #[inline(always)]
    pub const fn set_cnt_select(&mut self, val: super::vals::GlitchVddaDetectScCntSelect) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn timeout(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[doc = "Timeout"]
    #[inline(always)]
    pub const fn set_timeout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[doc = "Glitch Detect Reset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn re(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Glitch Detect Reset Enable"]
    #[inline(always)]
    pub const fn set_re(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Glitch Detect Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Glitch Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "GLITCH_DETECT_FLAG"]
    #[must_use]
    #[inline(always)]
    pub const fn glitch_detect_flag(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "GLITCH_DETECT_FLAG"]
    #[inline(always)]
    pub const fn set_glitch_detect_flag(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Glitch Detect Reset Enable Lock Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Glitch Detect Reset Enable Lock Bit"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GlitchVddaDetectSc {
    #[inline(always)]
    fn default() -> GlitchVddaDetectSc {
        GlitchVddaDetectSc(0)
    }
}
impl core::fmt::Debug for GlitchVddaDetectSc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlitchVddaDetectSc")
            .field("cnt_select", &self.cnt_select())
            .field("timeout", &self.timeout())
            .field("re", &self.re())
            .field("ie", &self.ie())
            .field("glitch_detect_flag", &self.glitch_detect_flag())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlitchVddaDetectSc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlitchVddaDetectSc {{ cnt_select: {:?}, timeout: {=u8:?}, re: {=bool:?}, ie: {=bool:?}, glitch_detect_flag: {=u8:?}, lock: {=bool:?} }}",
            self.cnt_select(),
            self.timeout(),
            self.re(),
            self.ie(),
            self.glitch_detect_flag(),
            self.lock()
        )
    }
}
#[doc = "Low-Power Mode Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpCfg(pub u32);
impl LpCfg {
    #[doc = "LDO_CORE VDD Drive Strength"]
    #[must_use]
    #[inline(always)]
    pub const fn coreldo_vdd_ds(&self) -> super::vals::LpCfgCoreldoVddDs {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LpCfgCoreldoVddDs::from_bits(val as u8)
    }
    #[doc = "LDO_CORE VDD Drive Strength"]
    #[inline(always)]
    pub const fn set_coreldo_vdd_ds(&mut self, val: super::vals::LpCfgCoreldoVddDs) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "LDO_CORE VDD Regulator Voltage Level"]
    #[must_use]
    #[inline(always)]
    pub const fn coreldo_vdd_lvl(&self) -> super::vals::LpCfgCoreldoVddLvl {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::LpCfgCoreldoVddLvl::from_bits(val as u8)
    }
    #[doc = "LDO_CORE VDD Regulator Voltage Level"]
    #[inline(always)]
    pub const fn set_coreldo_vdd_lvl(&mut self, val: super::vals::LpCfgCoreldoVddLvl) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Glitch Detect Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn glitch_detect_disable(&self) -> super::vals::LpCfgGlitchDetectDisable {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::LpCfgGlitchDetectDisable::from_bits(val as u8)
    }
    #[doc = "Glitch Detect Disable"]
    #[inline(always)]
    pub const fn set_glitch_detect_disable(&mut self, val: super::vals::LpCfgGlitchDetectDisable) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "CMP Bandgap Buffer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lpbuff_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "CMP Bandgap Buffer Enable"]
    #[inline(always)]
    pub const fn set_lpbuff_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Bandgap Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn bgmode(&self) -> super::vals::LpCfgBgmode {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::LpCfgBgmode::from_bits(val as u8)
    }
    #[doc = "Bandgap Mode"]
    #[inline(always)]
    pub const fn set_bgmode(&mut self, val: super::vals::LpCfgBgmode) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Low-Power IREF Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_irefen(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Low-Power IREF Enable"]
    #[inline(always)]
    pub const fn set_lp_irefen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Core Low Voltage Detect Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn core_lvde(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Core Low Voltage Detect Enable"]
    #[inline(always)]
    pub const fn set_core_lvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "System Low Voltage Detect Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_lvde(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "System Low Voltage Detect Enable"]
    #[inline(always)]
    pub const fn set_sys_lvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Core High Voltage Detect Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn core_hvde(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Core High Voltage Detect Enable"]
    #[inline(always)]
    pub const fn set_core_hvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "System High Voltage Detect Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_hvde(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "System High Voltage Detect Enable"]
    #[inline(always)]
    pub const fn set_sys_hvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for LpCfg {
    #[inline(always)]
    fn default() -> LpCfg {
        LpCfg(0)
    }
}
impl core::fmt::Debug for LpCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpCfg")
            .field("coreldo_vdd_ds", &self.coreldo_vdd_ds())
            .field("coreldo_vdd_lvl", &self.coreldo_vdd_lvl())
            .field("glitch_detect_disable", &self.glitch_detect_disable())
            .field("lpbuff_en", &self.lpbuff_en())
            .field("bgmode", &self.bgmode())
            .field("lp_irefen", &self.lp_irefen())
            .field("core_lvde", &self.core_lvde())
            .field("sys_lvde", &self.sys_lvde())
            .field("core_hvde", &self.core_hvde())
            .field("sys_hvde", &self.sys_hvde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LpCfg {{ coreldo_vdd_ds: {:?}, coreldo_vdd_lvl: {:?}, glitch_detect_disable: {:?}, lpbuff_en: {=bool:?}, bgmode: {:?}, lp_irefen: {=bool:?}, core_lvde: {=bool:?}, sys_lvde: {=bool:?}, core_hvde: {=bool:?}, sys_hvde: {=bool:?} }}",
            self.coreldo_vdd_ds(),
            self.coreldo_vdd_lvl(),
            self.glitch_detect_disable(),
            self.lpbuff_en(),
            self.bgmode(),
            self.lp_irefen(),
            self.core_lvde(),
            self.sys_lvde(),
            self.core_hvde(),
            self.sys_hvde()
        )
    }
}
#[doc = "Low Power Mode Configuration 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpCfg1(pub u32);
impl LpCfg1 {
    #[doc = "Low-Power Configuration Chip Control"]
    #[must_use]
    #[inline(always)]
    pub const fn soc_cntrl(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Low-Power Configuration Chip Control"]
    #[inline(always)]
    pub const fn set_soc_cntrl(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LpCfg1 {
    #[inline(always)]
    fn default() -> LpCfg1 {
        LpCfg1(0)
    }
}
impl core::fmt::Debug for LpCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpCfg1")
            .field("soc_cntrl", &self.soc_cntrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LpCfg1 {{ soc_cntrl: {=u32:?} }}", self.soc_cntrl())
    }
}
#[doc = "Low-Power Request Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpreqCfg(pub u32);
impl LpreqCfg {
    #[doc = "Low-Power Request Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lpreqoe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Low-Power Request Output Enable"]
    #[inline(always)]
    pub const fn set_lpreqoe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Low-Power Request Output Pin Polarity Control"]
    #[must_use]
    #[inline(always)]
    pub const fn lpreqpol(&self) -> super::vals::Lpreqpol {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Lpreqpol::from_bits(val as u8)
    }
    #[doc = "Low-Power Request Output Pin Polarity Control"]
    #[inline(always)]
    pub const fn set_lpreqpol(&mut self, val: super::vals::Lpreqpol) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Low-Power Request Output Override"]
    #[must_use]
    #[inline(always)]
    pub const fn lpreqov(&self) -> super::vals::Lpreqov {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Lpreqov::from_bits(val as u8)
    }
    #[doc = "Low-Power Request Output Override"]
    #[inline(always)]
    pub const fn set_lpreqov(&mut self, val: super::vals::Lpreqov) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for LpreqCfg {
    #[inline(always)]
    fn default() -> LpreqCfg {
        LpreqCfg(0)
    }
}
impl core::fmt::Debug for LpreqCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpreqCfg")
            .field("lpreqoe", &self.lpreqoe())
            .field("lpreqpol", &self.lpreqpol())
            .field("lpreqov", &self.lpreqov())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpreqCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LpreqCfg {{ lpreqoe: {=bool:?}, lpreqpol: {:?}, lpreqov: {:?} }}",
            self.lpreqoe(),
            self.lpreqpol(),
            self.lpreqov()
        )
    }
}
#[doc = "Low Power Wake-Up Delay"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpwkupDelay(pub u32);
impl LpwkupDelay {
    #[doc = "Low-Power Wake-Up Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn lpwkup_delay(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Low-Power Wake-Up Delay"]
    #[inline(always)]
    pub const fn set_lpwkup_delay(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for LpwkupDelay {
    #[inline(always)]
    fn default() -> LpwkupDelay {
        LpwkupDelay(0)
    }
}
impl core::fmt::Debug for LpwkupDelay {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpwkupDelay")
            .field("lpwkup_delay", &self.lpwkup_delay())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpwkupDelay {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LpwkupDelay {{ lpwkup_delay: {=u16:?} }}",
            self.lpwkup_delay()
        )
    }
}
#[doc = "SPC Power Domain Mode Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdStatus0(pub u32);
impl PdStatus0 {
    #[doc = "Power Request Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn pwr_req_status(&self) -> super::vals::PwrReqStatus {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PwrReqStatus::from_bits(val as u8)
    }
    #[doc = "Power Request Status Flag"]
    #[inline(always)]
    pub const fn set_pwr_req_status(&mut self, val: super::vals::PwrReqStatus) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Power Domain Low Power Request Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn pd_lp_req(&self) -> super::vals::PdLpReq {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::PdLpReq::from_bits(val as u8)
    }
    #[doc = "Power Domain Low Power Request Flag"]
    #[inline(always)]
    pub const fn set_pd_lp_req(&mut self, val: super::vals::PdLpReq) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Power Domain Low Power Mode Request"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_mode(&self) -> super::vals::LpMode {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::LpMode::from_bits(val as u8)
    }
    #[doc = "Power Domain Low Power Mode Request"]
    #[inline(always)]
    pub const fn set_lp_mode(&mut self, val: super::vals::LpMode) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
}
impl Default for PdStatus0 {
    #[inline(always)]
    fn default() -> PdStatus0 {
        PdStatus0(0)
    }
}
impl core::fmt::Debug for PdStatus0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PdStatus0")
            .field("pwr_req_status", &self.pwr_req_status())
            .field("pd_lp_req", &self.pd_lp_req())
            .field("lp_mode", &self.lp_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PdStatus0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PdStatus0 {{ pwr_req_status: {:?}, pd_lp_req: {:?}, lp_mode: {:?} }}",
            self.pwr_req_status(),
            self.pd_lp_req(),
            self.lp_mode()
        )
    }
}
#[doc = "Status Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sc(pub u32);
impl Sc {
    #[doc = "SPC Busy Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SPC Busy Status Flag"]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SPC Power Mode Configuration Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn spc_lp_req(&self) -> super::vals::SpcLpReq {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SpcLpReq::from_bits(val as u8)
    }
    #[doc = "SPC Power Mode Configuration Status Flag"]
    #[inline(always)]
    pub const fn set_spc_lp_req(&mut self, val: super::vals::SpcLpReq) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "SPC REG Busy Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn reg_busy(&self) -> super::vals::RegBusy {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::RegBusy::from_bits(val as u8)
    }
    #[doc = "SPC REG Busy Status Flag"]
    #[inline(always)]
    pub const fn set_reg_busy(&mut self, val: super::vals::RegBusy) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Power Domain Low-Power Mode Request"]
    #[must_use]
    #[inline(always)]
    pub const fn spc_lp_mode(&self) -> super::vals::SpcLpMode {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::SpcLpMode::from_bits(val as u8)
    }
    #[doc = "Power Domain Low-Power Mode Request"]
    #[inline(always)]
    pub const fn set_spc_lp_mode(&mut self, val: super::vals::SpcLpMode) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Isolation Clear Flags"]
    #[must_use]
    #[inline(always)]
    pub const fn iso_clr(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Isolation Clear Flags"]
    #[inline(always)]
    pub const fn set_iso_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Sc {
    #[inline(always)]
    fn default() -> Sc {
        Sc(0)
    }
}
impl core::fmt::Debug for Sc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sc")
            .field("busy", &self.busy())
            .field("spc_lp_req", &self.spc_lp_req())
            .field("reg_busy", &self.reg_busy())
            .field("spc_lp_mode", &self.spc_lp_mode())
            .field("iso_clr", &self.iso_clr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sc {{ busy: {=bool:?}, spc_lp_req: {:?}, reg_busy: {:?}, spc_lp_mode: {:?}, iso_clr: {=bool:?} }}",
            self.busy(),
            self.spc_lp_req(),
            self.reg_busy(),
            self.spc_lp_mode(),
            self.iso_clr()
        )
    }
}
#[doc = "SRAM Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramctl(pub u32);
impl Sramctl {
    #[doc = "Voltage Select Margin"]
    #[must_use]
    #[inline(always)]
    pub const fn vsm(&self) -> super::vals::Vsm {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Vsm::from_bits(val as u8)
    }
    #[doc = "Voltage Select Margin"]
    #[inline(always)]
    pub const fn set_vsm(&mut self, val: super::vals::Vsm) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "SRAM Voltage Update Request"]
    #[must_use]
    #[inline(always)]
    pub const fn req(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "SRAM Voltage Update Request"]
    #[inline(always)]
    pub const fn set_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "SRAM Voltage Update Request Acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn ack(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "SRAM Voltage Update Request Acknowledge"]
    #[inline(always)]
    pub const fn set_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Sramctl {
    #[inline(always)]
    fn default() -> Sramctl {
        Sramctl(0)
    }
}
impl core::fmt::Debug for Sramctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sramctl")
            .field("vsm", &self.vsm())
            .field("req", &self.req())
            .field("ack", &self.ack())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sramctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sramctl {{ vsm: {:?}, req: {=bool:?}, ack: {=bool:?} }}",
            self.vsm(),
            self.req(),
            self.ack()
        )
    }
}
#[doc = "Core Voltage Detect Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VdCoreCfg(pub u32);
impl VdCoreCfg {
    #[doc = "Core LVD Reset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdre(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Core LVD Reset Enable"]
    #[inline(always)]
    pub const fn set_lvdre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Core LVD Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Core LVD Interrupt Enable"]
    #[inline(always)]
    pub const fn set_lvdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Core VDD HVD Reset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hvdre(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Core VDD HVD Reset Enable"]
    #[inline(always)]
    pub const fn set_hvdre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Core VDD HVD Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hvdie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Core VDD HVD Interrupt Enable"]
    #[inline(always)]
    pub const fn set_hvdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Core Voltage Detect Reset Enable Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::VdCoreCfgLock {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::VdCoreCfgLock::from_bits(val as u8)
    }
    #[doc = "Core Voltage Detect Reset Enable Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: super::vals::VdCoreCfgLock) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for VdCoreCfg {
    #[inline(always)]
    fn default() -> VdCoreCfg {
        VdCoreCfg(0)
    }
}
impl core::fmt::Debug for VdCoreCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VdCoreCfg")
            .field("lvdre", &self.lvdre())
            .field("lvdie", &self.lvdie())
            .field("hvdre", &self.hvdre())
            .field("hvdie", &self.hvdie())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VdCoreCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VdCoreCfg {{ lvdre: {=bool:?}, lvdie: {=bool:?}, hvdre: {=bool:?}, hvdie: {=bool:?}, lock: {:?} }}",
            self.lvdre(),
            self.lvdie(),
            self.hvdre(),
            self.hvdie(),
            self.lock()
        )
    }
}
#[doc = "Voltage Detect Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VdStat(pub u32);
impl VdStat {
    #[doc = "Core Low-Voltage Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn corevdd_lvdf(&self) -> super::vals::CorevddLvdf {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CorevddLvdf::from_bits(val as u8)
    }
    #[doc = "Core Low-Voltage Detect Flag"]
    #[inline(always)]
    pub const fn set_corevdd_lvdf(&mut self, val: super::vals::CorevddLvdf) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "System Low-Voltage Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn sysvdd_lvdf(&self) -> super::vals::SysvddLvdf {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SysvddLvdf::from_bits(val as u8)
    }
    #[doc = "System Low-Voltage Detect Flag"]
    #[inline(always)]
    pub const fn set_sysvdd_lvdf(&mut self, val: super::vals::SysvddLvdf) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Core VDD HVD Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn corevdd_hvdf(&self) -> super::vals::CorevddHvdf {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::CorevddHvdf::from_bits(val as u8)
    }
    #[doc = "Core VDD HVD Flag"]
    #[inline(always)]
    pub const fn set_corevdd_hvdf(&mut self, val: super::vals::CorevddHvdf) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "System HVD Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn sysvdd_hvdf(&self) -> super::vals::SysvddHvdf {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::SysvddHvdf::from_bits(val as u8)
    }
    #[doc = "System HVD Flag"]
    #[inline(always)]
    pub const fn set_sysvdd_hvdf(&mut self, val: super::vals::SysvddHvdf) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Core Low-Voltage Monitor Status"]
    #[must_use]
    #[inline(always)]
    pub const fn corevdd_lvd_status(&self) -> super::vals::CorevddLvdStatus {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::CorevddLvdStatus::from_bits(val as u8)
    }
    #[doc = "Core Low-Voltage Monitor Status"]
    #[inline(always)]
    pub const fn set_corevdd_lvd_status(&mut self, val: super::vals::CorevddLvdStatus) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "System Low-Voltage Monitor Status"]
    #[must_use]
    #[inline(always)]
    pub const fn sysvdd_lvd_status(&self) -> super::vals::SysvddLvdStatus {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::SysvddLvdStatus::from_bits(val as u8)
    }
    #[doc = "System Low-Voltage Monitor Status"]
    #[inline(always)]
    pub const fn set_sysvdd_lvd_status(&mut self, val: super::vals::SysvddLvdStatus) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Core VDD HVD Monitor Status"]
    #[must_use]
    #[inline(always)]
    pub const fn corevdd_hvd_status(&self) -> super::vals::CorevddHvdStatus {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::CorevddHvdStatus::from_bits(val as u8)
    }
    #[doc = "Core VDD HVD Monitor Status"]
    #[inline(always)]
    pub const fn set_corevdd_hvd_status(&mut self, val: super::vals::CorevddHvdStatus) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "System HVD Monitor Status"]
    #[must_use]
    #[inline(always)]
    pub const fn sysvdd_hvd_status(&self) -> super::vals::SysvddHvdStatus {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::SysvddHvdStatus::from_bits(val as u8)
    }
    #[doc = "System HVD Monitor Status"]
    #[inline(always)]
    pub const fn set_sysvdd_hvd_status(&mut self, val: super::vals::SysvddHvdStatus) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
}
impl Default for VdStat {
    #[inline(always)]
    fn default() -> VdStat {
        VdStat(0)
    }
}
impl core::fmt::Debug for VdStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VdStat")
            .field("corevdd_lvdf", &self.corevdd_lvdf())
            .field("sysvdd_lvdf", &self.sysvdd_lvdf())
            .field("corevdd_hvdf", &self.corevdd_hvdf())
            .field("sysvdd_hvdf", &self.sysvdd_hvdf())
            .field("corevdd_lvd_status", &self.corevdd_lvd_status())
            .field("sysvdd_lvd_status", &self.sysvdd_lvd_status())
            .field("corevdd_hvd_status", &self.corevdd_hvd_status())
            .field("sysvdd_hvd_status", &self.sysvdd_hvd_status())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VdStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VdStat {{ corevdd_lvdf: {:?}, sysvdd_lvdf: {:?}, corevdd_hvdf: {:?}, sysvdd_hvdf: {:?}, corevdd_lvd_status: {:?}, sysvdd_lvd_status: {:?}, corevdd_hvd_status: {:?}, sysvdd_hvd_status: {:?} }}",
            self.corevdd_lvdf(),
            self.sysvdd_lvdf(),
            self.corevdd_hvdf(),
            self.sysvdd_hvdf(),
            self.corevdd_lvd_status(),
            self.sysvdd_lvd_status(),
            self.corevdd_hvd_status(),
            self.sysvdd_hvd_status()
        )
    }
}
#[doc = "System Voltage Detect Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VdSysCfg(pub u32);
impl VdSysCfg {
    #[doc = "System LVD Reset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdre(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "System LVD Reset Enable"]
    #[inline(always)]
    pub const fn set_lvdre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "System LVD Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "System LVD Interrupt Enable"]
    #[inline(always)]
    pub const fn set_lvdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "System HVD Reset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hvdre(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "System HVD Reset Enable"]
    #[inline(always)]
    pub const fn set_hvdre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "System HVD Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hvdie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "System HVD Interrupt Enable"]
    #[inline(always)]
    pub const fn set_hvdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "System Low-Voltage Level Select"]
    #[must_use]
    #[inline(always)]
    pub const fn lvsel(&self) -> super::vals::Lvsel {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Lvsel::from_bits(val as u8)
    }
    #[doc = "System Low-Voltage Level Select"]
    #[inline(always)]
    pub const fn set_lvsel(&mut self, val: super::vals::Lvsel) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "System Voltage Detect Reset Enable Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::VdSysCfgLock {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::VdSysCfgLock::from_bits(val as u8)
    }
    #[doc = "System Voltage Detect Reset Enable Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: super::vals::VdSysCfgLock) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for VdSysCfg {
    #[inline(always)]
    fn default() -> VdSysCfg {
        VdSysCfg(0)
    }
}
impl core::fmt::Debug for VdSysCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VdSysCfg")
            .field("lvdre", &self.lvdre())
            .field("lvdie", &self.lvdie())
            .field("hvdre", &self.hvdre())
            .field("hvdie", &self.hvdie())
            .field("lvsel", &self.lvsel())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VdSysCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VdSysCfg {{ lvdre: {=bool:?}, lvdie: {=bool:?}, hvdre: {=bool:?}, hvdie: {=bool:?}, lvsel: {:?}, lock: {:?} }}",
            self.lvdre(),
            self.lvdie(),
            self.hvdre(),
            self.hvdie(),
            self.lvsel(),
            self.lock()
        )
    }
}
#[doc = "Version ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Specification Number"]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> super::vals::Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Feature::from_bits(val as u16)
    }
    #[doc = "Feature Specification Number"]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: super::vals::Feature) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number"]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number"]
    #[inline(always)]
    pub const fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Verid {
    #[inline(always)]
    fn default() -> Verid {
        Verid(0)
    }
}
impl core::fmt::Debug for Verid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Verid")
            .field("feature", &self.feature())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Verid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Verid {{ feature: {:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.feature(),
            self.minor(),
            self.major()
        )
    }
}
