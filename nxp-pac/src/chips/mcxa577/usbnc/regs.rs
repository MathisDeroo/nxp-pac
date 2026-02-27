#[doc = "USB Clock Recovery Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRecoverCtrl(pub u8);
impl ClkRecoverCtrl {
    #[doc = "Selects the source for the initial FIRC192M trim fine value used after a reset."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_init_val_sel(&self) -> super::vals::TrimInitValSel {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::TrimInitValSel::from_bits(val as u8)
    }
    #[doc = "Selects the source for the initial FIRC192M trim fine value used after a reset."]
    #[inline(always)]
    pub const fn set_trim_init_val_sel(&mut self, val: super::vals::TrimInitValSel) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Restart from IFR Trim Value"]
    #[must_use]
    #[inline(always)]
    pub const fn restart_ifrtrim_en(&self) -> super::vals::RestartIfrtrimEn {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::RestartIfrtrimEn::from_bits(val as u8)
    }
    #[doc = "Restart from IFR Trim Value"]
    #[inline(always)]
    pub const fn set_restart_ifrtrim_en(&mut self, val: super::vals::RestartIfrtrimEn) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "Reset or Resume to Rough Phase Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn reset_resume_rough_en(&self) -> super::vals::ResetResumeRoughEn {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::ResetResumeRoughEn::from_bits(val as u8)
    }
    #[doc = "Reset or Resume to Rough Phase Enable"]
    #[inline(always)]
    pub const fn set_reset_resume_rough_en(&mut self, val: super::vals::ResetResumeRoughEn) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Crystal-Less USB Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn clock_recover_en(&self) -> super::vals::ClockRecoverEn {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::ClockRecoverEn::from_bits(val as u8)
    }
    #[doc = "Crystal-Less USB Enable"]
    #[inline(always)]
    pub const fn set_clock_recover_en(&mut self, val: super::vals::ClockRecoverEn) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for ClkRecoverCtrl {
    #[inline(always)]
    fn default() -> ClkRecoverCtrl {
        ClkRecoverCtrl(0)
    }
}
impl core::fmt::Debug for ClkRecoverCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkRecoverCtrl")
            .field("trim_init_val_sel", &self.trim_init_val_sel())
            .field("restart_ifrtrim_en", &self.restart_ifrtrim_en())
            .field("reset_resume_rough_en", &self.reset_resume_rough_en())
            .field("clock_recover_en", &self.clock_recover_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkRecoverCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ClkRecoverCtrl {{ trim_init_val_sel: {:?}, restart_ifrtrim_en: {:?}, reset_resume_rough_en: {:?}, clock_recover_en: {:?} }}",
            self.trim_init_val_sel(),
            self.restart_ifrtrim_en(),
            self.reset_resume_rough_en(),
            self.clock_recover_en()
        )
    }
}
#[doc = "Clock Recovery Combined Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRecoverIntEn(pub u8);
impl ClkRecoverIntEn {
    #[doc = "Overflow error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ovf_error_en(&self) -> super::vals::OvfErrorEn {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::OvfErrorEn::from_bits(val as u8)
    }
    #[doc = "Overflow error interrupt enable"]
    #[inline(always)]
    pub const fn set_ovf_error_en(&mut self, val: super::vals::OvfErrorEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
}
impl Default for ClkRecoverIntEn {
    #[inline(always)]
    fn default() -> ClkRecoverIntEn {
        ClkRecoverIntEn(0)
    }
}
impl core::fmt::Debug for ClkRecoverIntEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkRecoverIntEn")
            .field("ovf_error_en", &self.ovf_error_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkRecoverIntEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ClkRecoverIntEn {{ ovf_error_en: {:?} }}",
            self.ovf_error_en()
        )
    }
}
#[doc = "Clock Recovery Separated Interrupt Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRecoverIntStatus(pub u8);
impl ClkRecoverIntStatus {
    #[doc = "Overflow Error Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ovf_error(&self) -> super::vals::OvfError {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::OvfError::from_bits(val as u8)
    }
    #[doc = "Overflow Error Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_ovf_error(&mut self, val: super::vals::OvfError) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
}
impl Default for ClkRecoverIntStatus {
    #[inline(always)]
    fn default() -> ClkRecoverIntStatus {
        ClkRecoverIntStatus(0)
    }
}
impl core::fmt::Debug for ClkRecoverIntStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkRecoverIntStatus")
            .field("ovf_error", &self.ovf_error())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkRecoverIntStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ClkRecoverIntStatus {{ ovf_error: {:?} }}",
            self.ovf_error()
        )
    }
}
#[doc = "FIRC Oscillator Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRecoverIrcEn(pub u8);
impl ClkRecoverIrcEn {
    #[doc = "Fast IRC enable"]
    #[must_use]
    #[inline(always)]
    pub const fn irc_en(&self) -> super::vals::IrcEn {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::IrcEn::from_bits(val as u8)
    }
    #[doc = "Fast IRC enable"]
    #[inline(always)]
    pub const fn set_irc_en(&mut self, val: super::vals::IrcEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
}
impl Default for ClkRecoverIrcEn {
    #[inline(always)]
    fn default() -> ClkRecoverIrcEn {
        ClkRecoverIrcEn(0)
    }
}
impl core::fmt::Debug for ClkRecoverIrcEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkRecoverIrcEn")
            .field("irc_en", &self.irc_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkRecoverIrcEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ClkRecoverIrcEn {{ irc_en: {:?} }}", self.irc_en())
    }
}
#[doc = "USB Control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1(pub u32);
impl Ctrl1 {
    #[doc = "Overcurrent Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn over_cur_dis(&self) -> super::vals::OverCurDis {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::OverCurDis::from_bits(val as u8)
    }
    #[doc = "Overcurrent Disable"]
    #[inline(always)]
    pub const fn set_over_cur_dis(&mut self, val: super::vals::OverCurDis) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Overcurrent Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn over_cur_pol(&self) -> super::vals::OverCurPol {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::OverCurPol::from_bits(val as u8)
    }
    #[doc = "Overcurrent Polarity"]
    #[inline(always)]
    pub const fn set_over_cur_pol(&mut self, val: super::vals::OverCurPol) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Power Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pwr_pol(&self) -> super::vals::PwrPol {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PwrPol::from_bits(val as u8)
    }
    #[doc = "Power Polarity"]
    #[inline(always)]
    pub const fn set_pwr_pol(&mut self, val: super::vals::PwrPol) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Wake-Up Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wie(&self) -> super::vals::Wie {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Wie::from_bits(val as u8)
    }
    #[doc = "Wake-Up Interrupt Enable"]
    #[inline(always)]
    pub const fn set_wie(&mut self, val: super::vals::Wie) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Software Wake-Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wkup_sw_en(&self) -> super::vals::WkupSwEn {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::WkupSwEn::from_bits(val as u8)
    }
    #[doc = "Software Wake-Up Enable"]
    #[inline(always)]
    pub const fn set_wkup_sw_en(&mut self, val: super::vals::WkupSwEn) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Software Wake-Up"]
    #[must_use]
    #[inline(always)]
    pub const fn wkup_sw(&self) -> super::vals::WkupSw {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::WkupSw::from_bits(val as u8)
    }
    #[doc = "Software Wake-Up"]
    #[inline(always)]
    pub const fn set_wkup_sw(&mut self, val: super::vals::WkupSw) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Wake-Up After ID Change Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wkup_id_en(&self) -> super::vals::WkupIdEn {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::WkupIdEn::from_bits(val as u8)
    }
    #[doc = "Wake-Up After ID Change Enable"]
    #[inline(always)]
    pub const fn set_wkup_id_en(&mut self, val: super::vals::WkupIdEn) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-Up After VBUS Change Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wkup_vbus_en(&self) -> super::vals::WkupVbusEn {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::WkupVbusEn::from_bits(val as u8)
    }
    #[doc = "Wake-Up After VBUS Change Enable"]
    #[inline(always)]
    pub const fn set_wkup_vbus_en(&mut self, val: super::vals::WkupVbusEn) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Remote Wake-Up Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn remote_wakeup_en(&self) -> super::vals::RemoteWakeupEn {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::RemoteWakeupEn::from_bits(val as u8)
    }
    #[doc = "Remote Wake-Up Enable"]
    #[inline(always)]
    pub const fn set_remote_wakeup_en(&mut self, val: super::vals::RemoteWakeupEn) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Wake-Up After DP or DM Change Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wkup_dpdm_en(&self) -> super::vals::WkupDpdmEn {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::WkupDpdmEn::from_bits(val as u8)
    }
    #[doc = "Wake-Up After DP or DM Change Enable"]
    #[inline(always)]
    pub const fn set_wkup_dpdm_en(&mut self, val: super::vals::WkupDpdmEn) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Wake-Up Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn wir(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-Up Interrupt Request"]
    #[inline(always)]
    pub const fn set_wir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl1 {
    #[inline(always)]
    fn default() -> Ctrl1 {
        Ctrl1(0)
    }
}
impl core::fmt::Debug for Ctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl1")
            .field("over_cur_dis", &self.over_cur_dis())
            .field("over_cur_pol", &self.over_cur_pol())
            .field("pwr_pol", &self.pwr_pol())
            .field("wie", &self.wie())
            .field("wkup_sw_en", &self.wkup_sw_en())
            .field("wkup_sw", &self.wkup_sw())
            .field("wkup_id_en", &self.wkup_id_en())
            .field("wkup_vbus_en", &self.wkup_vbus_en())
            .field("remote_wakeup_en", &self.remote_wakeup_en())
            .field("wkup_dpdm_en", &self.wkup_dpdm_en())
            .field("wir", &self.wir())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl1 {{ over_cur_dis: {:?}, over_cur_pol: {:?}, pwr_pol: {:?}, wie: {:?}, wkup_sw_en: {:?}, wkup_sw: {:?}, wkup_id_en: {:?}, wkup_vbus_en: {:?}, remote_wakeup_en: {:?}, wkup_dpdm_en: {:?}, wir: {=bool:?} }}",
            self.over_cur_dis(),
            self.over_cur_pol(),
            self.pwr_pol(),
            self.wie(),
            self.wkup_sw_en(),
            self.wkup_sw(),
            self.wkup_id_en(),
            self.wkup_vbus_en(),
            self.remote_wakeup_en(),
            self.wkup_dpdm_en(),
            self.wir()
        )
    }
}
#[doc = "USB Control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl2(pub u32);
impl Ctrl2 {
    #[doc = "VBUS Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_source_sel(&self) -> super::vals::VbusSourceSel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::VbusSourceSel::from_bits(val as u8)
    }
    #[doc = "VBUS Source Select"]
    #[inline(always)]
    pub const fn set_vbus_source_sel(&mut self, val: super::vals::VbusSourceSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "UTMI Clock Valid Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_clk_vld(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Clock Valid Flag"]
    #[inline(always)]
    pub const fn set_utmi_clk_vld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl2 {
    #[inline(always)]
    fn default() -> Ctrl2 {
        Ctrl2(0)
    }
}
impl core::fmt::Debug for Ctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl2")
            .field("vbus_source_sel", &self.vbus_source_sel())
            .field("utmi_clk_vld", &self.utmi_clk_vld())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl2 {{ vbus_source_sel: {:?}, utmi_clk_vld: {=bool:?} }}",
            self.vbus_source_sel(),
            self.utmi_clk_vld()
        )
    }
}
#[doc = "HSIC DLL Configure Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HsicDllCfg4(pub u32);
impl HsicDllCfg4 {
    #[doc = "LPM EXT token ENDP check enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_en_endp_chk(&self) -> super::vals::LpmEnEndpChk {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::LpmEnEndpChk::from_bits(val as u8)
    }
    #[doc = "LPM EXT token ENDP check enable"]
    #[inline(always)]
    pub const fn set_lpm_en_endp_chk(&mut self, val: super::vals::LpmEnEndpChk) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "FS Isochronous back to back transfer enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fs_iso_b2b_fixen(&self) -> super::vals::FsIsoB2bFixen {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::FsIsoB2bFixen::from_bits(val as u8)
    }
    #[doc = "FS Isochronous back to back transfer enable"]
    #[inline(always)]
    pub const fn set_fs_iso_b2b_fixen(&mut self, val: super::vals::FsIsoB2bFixen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for HsicDllCfg4 {
    #[inline(always)]
    fn default() -> HsicDllCfg4 {
        HsicDllCfg4(0)
    }
}
impl core::fmt::Debug for HsicDllCfg4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HsicDllCfg4")
            .field("lpm_en_endp_chk", &self.lpm_en_endp_chk())
            .field("fs_iso_b2b_fixen", &self.fs_iso_b2b_fixen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HsicDllCfg4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HsicDllCfg4 {{ lpm_en_endp_chk: {:?}, fs_iso_b2b_fixen: {:?} }}",
            self.lpm_en_endp_chk(),
            self.fs_iso_b2b_fixen()
        )
    }
}
#[doc = "USB LPM Control and Status 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpmCsr0(pub u32);
impl LpmCsr0 {
    #[doc = "Link Power Management Feature Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_en(&self) -> super::vals::LpmEn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LpmEn::from_bits(val as u8)
    }
    #[doc = "Link Power Management Feature Enable"]
    #[inline(always)]
    pub const fn set_lpm_en(&mut self, val: super::vals::LpmEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Link Power Management ECN Errata Feature Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_errata_en(&self) -> super::vals::LpmErrataEn {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::LpmErrataEn::from_bits(val as u8)
    }
    #[doc = "Link Power Management ECN Errata Feature Enable"]
    #[inline(always)]
    pub const fn set_lpm_errata_en(&mut self, val: super::vals::LpmErrataEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Auto Low-Power Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_auto_phcd(&self) -> super::vals::LpmAutoPhcd {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::LpmAutoPhcd::from_bits(val as u8)
    }
    #[doc = "Auto Low-Power Mode"]
    #[inline(always)]
    pub const fn set_lpm_auto_phcd(&mut self, val: super::vals::LpmAutoPhcd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "LPM Resume OK"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_resumeok(&self) -> super::vals::LpmResumeok {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::LpmResumeok::from_bits(val as u8)
    }
    #[doc = "LPM Resume OK"]
    #[inline(always)]
    pub const fn set_lpm_resumeok(&mut self, val: super::vals::LpmResumeok) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "LPM Active"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_l1_active(&self) -> super::vals::LpmL1Active {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::LpmL1Active::from_bits(val as u8)
    }
    #[doc = "LPM Active"]
    #[inline(always)]
    pub const fn set_lpm_l1_active(&mut self, val: super::vals::LpmL1Active) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for LpmCsr0 {
    #[inline(always)]
    fn default() -> LpmCsr0 {
        LpmCsr0(0)
    }
}
impl core::fmt::Debug for LpmCsr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpmCsr0")
            .field("lpm_en", &self.lpm_en())
            .field("lpm_errata_en", &self.lpm_errata_en())
            .field("lpm_auto_phcd", &self.lpm_auto_phcd())
            .field("lpm_resumeok", &self.lpm_resumeok())
            .field("lpm_l1_active", &self.lpm_l1_active())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpmCsr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LpmCsr0 {{ lpm_en: {:?}, lpm_errata_en: {:?}, lpm_auto_phcd: {:?}, lpm_resumeok: {:?}, lpm_l1_active: {:?} }}",
            self.lpm_en(),
            self.lpm_errata_en(),
            self.lpm_auto_phcd(),
            self.lpm_resumeok(),
            self.lpm_l1_active()
        )
    }
}
#[doc = "USB LPM Control and Status 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpmCsr1(pub u32);
impl LpmCsr1 {
    #[doc = "Device Required Host Initiated Resume Duration"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_dev_beslthres(&self) -> super::vals::LpmDevBeslthres {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::LpmDevBeslthres::from_bits(val as u8)
    }
    #[doc = "Device Required Host Initiated Resume Duration"]
    #[inline(always)]
    pub const fn set_lpm_dev_beslthres(&mut self, val: super::vals::LpmDevBeslthres) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "LPM Device Response"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_dev_res(&self) -> super::vals::LpmDevRes {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::LpmDevRes::from_bits(val as u8)
    }
    #[doc = "LPM Device Response"]
    #[inline(always)]
    pub const fn set_lpm_dev_res(&mut self, val: super::vals::LpmDevRes) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "LPM Device Data Pending"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_dev_dp(&self) -> super::vals::LpmDevDp {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::LpmDevDp::from_bits(val as u8)
    }
    #[doc = "LPM Device Data Pending"]
    #[inline(always)]
    pub const fn set_lpm_dev_dp(&mut self, val: super::vals::LpmDevDp) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "LPM Device Response Status"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_dev_rspsts(&self) -> super::vals::LpmDevRspsts {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::LpmDevRspsts::from_bits(val as u8)
    }
    #[doc = "LPM Device Response Status"]
    #[inline(always)]
    pub const fn set_lpm_dev_rspsts(&mut self, val: super::vals::LpmDevRspsts) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "LPM Device Received bRemoteWake"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_dev_rwkenrcvd(&self) -> super::vals::LpmDevRwkenrcvd {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::LpmDevRwkenrcvd::from_bits(val as u8)
    }
    #[doc = "LPM Device Received bRemoteWake"]
    #[inline(always)]
    pub const fn set_lpm_dev_rwkenrcvd(&mut self, val: super::vals::LpmDevRwkenrcvd) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "LPM Device Received bLinkState"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_dev_lnkstrcvd(&self) -> super::vals::LpmDevLnkstrcvd {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::LpmDevLnkstrcvd::from_bits(val as u8)
    }
    #[doc = "LPM Device Received bLinkState"]
    #[inline(always)]
    pub const fn set_lpm_dev_lnkstrcvd(&mut self, val: super::vals::LpmDevLnkstrcvd) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "LPM Device Received BESL"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_dev_beslrcvd(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "LPM Device Received BESL"]
    #[inline(always)]
    pub const fn set_lpm_dev_beslrcvd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for LpmCsr1 {
    #[inline(always)]
    fn default() -> LpmCsr1 {
        LpmCsr1(0)
    }
}
impl core::fmt::Debug for LpmCsr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpmCsr1")
            .field("lpm_dev_beslthres", &self.lpm_dev_beslthres())
            .field("lpm_dev_res", &self.lpm_dev_res())
            .field("lpm_dev_dp", &self.lpm_dev_dp())
            .field("lpm_dev_rspsts", &self.lpm_dev_rspsts())
            .field("lpm_dev_rwkenrcvd", &self.lpm_dev_rwkenrcvd())
            .field("lpm_dev_lnkstrcvd", &self.lpm_dev_lnkstrcvd())
            .field("lpm_dev_beslrcvd", &self.lpm_dev_beslrcvd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpmCsr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LpmCsr1 {{ lpm_dev_beslthres: {:?}, lpm_dev_res: {:?}, lpm_dev_dp: {:?}, lpm_dev_rspsts: {:?}, lpm_dev_rwkenrcvd: {:?}, lpm_dev_lnkstrcvd: {:?}, lpm_dev_beslrcvd: {=u8:?} }}",
            self.lpm_dev_beslthres(),
            self.lpm_dev_res(),
            self.lpm_dev_dp(),
            self.lpm_dev_rspsts(),
            self.lpm_dev_rwkenrcvd(),
            self.lpm_dev_lnkstrcvd(),
            self.lpm_dev_beslrcvd()
        )
    }
}
#[doc = "USB LPM Control and Status 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpmCsr2(pub u32);
impl LpmCsr2 {
    #[doc = "LPM Host Send Extension Token"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_hst_send(&self) -> super::vals::LpmHstSend {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LpmHstSend::from_bits(val as u8)
    }
    #[doc = "LPM Host Send Extension Token"]
    #[inline(always)]
    pub const fn set_lpm_hst_send(&mut self, val: super::vals::LpmHstSend) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "LPM Host Extension Token's Device Address"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_hst_devadd(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "LPM Host Extension Token's Device Address"]
    #[inline(always)]
    pub const fn set_lpm_hst_devadd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "LPM Host Extension Token's BESL or HIRD"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_hst_besl(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "LPM Host Extension Token's BESL or HIRD"]
    #[inline(always)]
    pub const fn set_lpm_hst_besl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "LPM Host Extension Token's bRemoteWake"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_hst_rwken(&self) -> super::vals::LpmHstRwken {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::LpmHstRwken::from_bits(val as u8)
    }
    #[doc = "LPM Host Extension Token's bRemoteWake"]
    #[inline(always)]
    pub const fn set_lpm_hst_rwken(&mut self, val: super::vals::LpmHstRwken) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "LPM Host Response Status from the Device"]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_hst_stsrcvd(&self) -> super::vals::LpmHstStsrcvd {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::LpmHstStsrcvd::from_bits(val as u8)
    }
    #[doc = "LPM Host Response Status from the Device"]
    #[inline(always)]
    pub const fn set_lpm_hst_stsrcvd(&mut self, val: super::vals::LpmHstStsrcvd) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
}
impl Default for LpmCsr2 {
    #[inline(always)]
    fn default() -> LpmCsr2 {
        LpmCsr2(0)
    }
}
impl core::fmt::Debug for LpmCsr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpmCsr2")
            .field("lpm_hst_send", &self.lpm_hst_send())
            .field("lpm_hst_devadd", &self.lpm_hst_devadd())
            .field("lpm_hst_besl", &self.lpm_hst_besl())
            .field("lpm_hst_rwken", &self.lpm_hst_rwken())
            .field("lpm_hst_stsrcvd", &self.lpm_hst_stsrcvd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpmCsr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LpmCsr2 {{ lpm_hst_send: {:?}, lpm_hst_devadd: {=u8:?}, lpm_hst_besl: {=u8:?}, lpm_hst_rwken: {:?}, lpm_hst_stsrcvd: {:?} }}",
            self.lpm_hst_send(),
            self.lpm_hst_devadd(),
            self.lpm_hst_besl(),
            self.lpm_hst_rwken(),
            self.lpm_hst_stsrcvd()
        )
    }
}
