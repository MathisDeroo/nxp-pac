#[doc = "CFPA State Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CfpaLcState(pub u32);
impl CfpaLcState {
    #[doc = "CFPA state"]
    #[must_use]
    #[inline(always)]
    pub const fn cfpa_lc_state(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CFPA state"]
    #[inline(always)]
    pub const fn set_cfpa_lc_state(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CfpaLcState {
    #[inline(always)]
    fn default() -> CfpaLcState {
        CfpaLcState(0)
    }
}
impl core::fmt::Debug for CfpaLcState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CfpaLcState")
            .field("cfpa_lc_state", &self.cfpa_lc_state())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CfpaLcState {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CfpaLcState {{ cfpa_lc_state: {=u32:?} }}",
            self.cfpa_lc_state()
        )
    }
}
#[doc = "Secure CPU0 System Tick Calibration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu0stckcal(pub u32);
impl Cpu0stckcal {
    #[doc = "Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[must_use]
    #[inline(always)]
    pub const fn tenms(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[inline(always)]
    pub const fn set_tenms(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Whether the TENMS value is exact."]
    #[must_use]
    #[inline(always)]
    pub const fn skew(&self) -> super::vals::Skew {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Skew::from_bits(val as u8)
    }
    #[doc = "Whether the TENMS value is exact."]
    #[inline(always)]
    pub const fn set_skew(&mut self, val: super::vals::Skew) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Whether the device provides a reference clock to the processor."]
    #[must_use]
    #[inline(always)]
    pub const fn noref(&self) -> super::vals::Noref {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Noref::from_bits(val as u8)
    }
    #[doc = "Whether the device provides a reference clock to the processor."]
    #[inline(always)]
    pub const fn set_noref(&mut self, val: super::vals::Noref) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for Cpu0stckcal {
    #[inline(always)]
    fn default() -> Cpu0stckcal {
        Cpu0stckcal(0)
    }
}
impl core::fmt::Debug for Cpu0stckcal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpu0stckcal")
            .field("tenms", &self.tenms())
            .field("skew", &self.skew())
            .field("noref", &self.noref())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpu0stckcal {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpu0stckcal {{ tenms: {=u32:?}, skew: {:?}, noref: {:?} }}",
            self.tenms(),
            self.skew(),
            self.noref()
        )
    }
}
#[doc = "Debug Authentication BEACON"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugAuthBeacon(pub u32);
impl DebugAuthBeacon {
    #[doc = "Sets by the debug authentication code in ROM to pass the debug beacons (Credential Beacon and Authentication Beacon) to the application code."]
    #[must_use]
    #[inline(always)]
    pub const fn beacon(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Sets by the debug authentication code in ROM to pass the debug beacons (Credential Beacon and Authentication Beacon) to the application code."]
    #[inline(always)]
    pub const fn set_beacon(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DebugAuthBeacon {
    #[inline(always)]
    fn default() -> DebugAuthBeacon {
        DebugAuthBeacon(0)
    }
}
impl core::fmt::Debug for DebugAuthBeacon {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DebugAuthBeacon")
            .field("beacon", &self.beacon())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugAuthBeacon {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DebugAuthBeacon {{ beacon: {=u32:?} }}", self.beacon())
    }
}
#[doc = "Cortex Debug Features Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugFeatures(pub u32);
impl DebugFeatures {
    #[doc = "CPU0 invasive debug control"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_dbgen(&self) -> super::vals::DebugFeaturesCpu0Dbgen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::DebugFeaturesCpu0Dbgen::from_bits(val as u8)
    }
    #[doc = "CPU0 invasive debug control"]
    #[inline(always)]
    pub const fn set_cpu0_dbgen(&mut self, val: super::vals::DebugFeaturesCpu0Dbgen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CPU0 non-invasive debug control"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_niden(&self) -> super::vals::DebugFeaturesCpu0Niden {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::DebugFeaturesCpu0Niden::from_bits(val as u8)
    }
    #[doc = "CPU0 non-invasive debug control"]
    #[inline(always)]
    pub const fn set_cpu0_niden(&mut self, val: super::vals::DebugFeaturesCpu0Niden) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "CPU0 secure privileged invasive debug control"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_spiden(&self) -> super::vals::DebugFeaturesCpu0Spiden {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::DebugFeaturesCpu0Spiden::from_bits(val as u8)
    }
    #[doc = "CPU0 secure privileged invasive debug control"]
    #[inline(always)]
    pub const fn set_cpu0_spiden(&mut self, val: super::vals::DebugFeaturesCpu0Spiden) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "CPU0 secure privileged non-invasive debug control"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_spniden(&self) -> super::vals::DebugFeaturesCpu0Spniden {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::DebugFeaturesCpu0Spniden::from_bits(val as u8)
    }
    #[doc = "CPU0 secure privileged non-invasive debug control"]
    #[inline(always)]
    pub const fn set_cpu0_spniden(&mut self, val: super::vals::DebugFeaturesCpu0Spniden) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
}
impl Default for DebugFeatures {
    #[inline(always)]
    fn default() -> DebugFeatures {
        DebugFeatures(0)
    }
}
impl core::fmt::Debug for DebugFeatures {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DebugFeatures")
            .field("cpu0_dbgen", &self.cpu0_dbgen())
            .field("cpu0_niden", &self.cpu0_niden())
            .field("cpu0_spiden", &self.cpu0_spiden())
            .field("cpu0_spniden", &self.cpu0_spniden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugFeatures {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DebugFeatures {{ cpu0_dbgen: {:?}, cpu0_niden: {:?}, cpu0_spiden: {:?}, cpu0_spniden: {:?} }}",
            self.cpu0_dbgen(),
            self.cpu0_niden(),
            self.cpu0_spiden(),
            self.cpu0_spniden()
        )
    }
}
#[doc = "Cortex Debug Features Control (Duplicate)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugFeaturesDp(pub u32);
impl DebugFeaturesDp {
    #[doc = "CPU0 invasive debug control"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_dbgen(&self) -> super::vals::DebugFeaturesDpCpu0Dbgen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::DebugFeaturesDpCpu0Dbgen::from_bits(val as u8)
    }
    #[doc = "CPU0 invasive debug control"]
    #[inline(always)]
    pub const fn set_cpu0_dbgen(&mut self, val: super::vals::DebugFeaturesDpCpu0Dbgen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CPU0 non-invasive debug control"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_niden(&self) -> super::vals::DebugFeaturesDpCpu0Niden {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::DebugFeaturesDpCpu0Niden::from_bits(val as u8)
    }
    #[doc = "CPU0 non-invasive debug control"]
    #[inline(always)]
    pub const fn set_cpu0_niden(&mut self, val: super::vals::DebugFeaturesDpCpu0Niden) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "CPU0 secure privileged invasive debug control"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_spiden(&self) -> super::vals::DebugFeaturesDpCpu0Spiden {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::DebugFeaturesDpCpu0Spiden::from_bits(val as u8)
    }
    #[doc = "CPU0 secure privileged invasive debug control"]
    #[inline(always)]
    pub const fn set_cpu0_spiden(&mut self, val: super::vals::DebugFeaturesDpCpu0Spiden) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "CPU0 secure privileged non-invasive debug control"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_spniden(&self) -> super::vals::DebugFeaturesDpCpu0Spniden {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::DebugFeaturesDpCpu0Spniden::from_bits(val as u8)
    }
    #[doc = "CPU0 secure privileged non-invasive debug control"]
    #[inline(always)]
    pub const fn set_cpu0_spniden(&mut self, val: super::vals::DebugFeaturesDpCpu0Spniden) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
}
impl Default for DebugFeaturesDp {
    #[inline(always)]
    fn default() -> DebugFeaturesDp {
        DebugFeaturesDp(0)
    }
}
impl core::fmt::Debug for DebugFeaturesDp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DebugFeaturesDp")
            .field("cpu0_dbgen", &self.cpu0_dbgen())
            .field("cpu0_niden", &self.cpu0_niden())
            .field("cpu0_spiden", &self.cpu0_spiden())
            .field("cpu0_spniden", &self.cpu0_spniden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugFeaturesDp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DebugFeaturesDp {{ cpu0_dbgen: {:?}, cpu0_niden: {:?}, cpu0_spiden: {:?}, cpu0_spniden: {:?} }}",
            self.cpu0_dbgen(),
            self.cpu0_niden(),
            self.cpu0_spiden(),
            self.cpu0_spniden()
        )
    }
}
#[doc = "Control Write Access to Security"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugLockEn(pub u32);
impl DebugLockEn {
    #[doc = "Controls write access to the security registers"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_all(&self) -> super::vals::LockAll {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::LockAll::from_bits(val as u8)
    }
    #[doc = "Controls write access to the security registers"]
    #[inline(always)]
    pub const fn set_lock_all(&mut self, val: super::vals::LockAll) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for DebugLockEn {
    #[inline(always)]
    fn default() -> DebugLockEn {
        DebugLockEn(0)
    }
}
impl core::fmt::Debug for DebugLockEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DebugLockEn")
            .field("lock_all", &self.lock_all())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugLockEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DebugLockEn {{ lock_all: {:?} }}", self.lock_all())
    }
}
#[doc = "Life Cycle State Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsOtpLcState(pub u32);
impl ElsOtpLcState {
    #[doc = "OTP life cycle state"]
    #[must_use]
    #[inline(always)]
    pub const fn otp_lc_state(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "OTP life cycle state"]
    #[inline(always)]
    pub const fn set_otp_lc_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ElsOtpLcState {
    #[inline(always)]
    fn default() -> ElsOtpLcState {
        ElsOtpLcState(0)
    }
}
impl core::fmt::Debug for ElsOtpLcState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsOtpLcState")
            .field("otp_lc_state", &self.otp_lc_state())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsOtpLcState {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsOtpLcState {{ otp_lc_state: {=u8:?} }}",
            self.otp_lc_state()
        )
    }
}
#[doc = "Life Cycle State Register (Duplicate)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsOtpLcStateDp(pub u32);
impl ElsOtpLcStateDp {
    #[doc = "OTP life cycle state"]
    #[must_use]
    #[inline(always)]
    pub const fn otp_lc_state_dp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "OTP life cycle state"]
    #[inline(always)]
    pub const fn set_otp_lc_state_dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ElsOtpLcStateDp {
    #[inline(always)]
    fn default() -> ElsOtpLcStateDp {
        ElsOtpLcStateDp(0)
    }
}
impl core::fmt::Debug for ElsOtpLcStateDp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsOtpLcStateDp")
            .field("otp_lc_state_dp", &self.otp_lc_state_dp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsOtpLcStateDp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsOtpLcStateDp {{ otp_lc_state_dp: {=u8:?} }}",
            self.otp_lc_state_dp()
        )
    }
}
#[doc = "GDET0 Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gdet0Ctrl(pub u32);
impl Gdet0Ctrl {
    #[doc = "Controls the GDET0 clean event counter"]
    #[must_use]
    #[inline(always)]
    pub const fn gdet_evtcnt_clr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Controls the GDET0 clean event counter"]
    #[inline(always)]
    pub const fn set_gdet_evtcnt_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clears GDET0 error status"]
    #[must_use]
    #[inline(always)]
    pub const fn gdet_err_clr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Clears GDET0 error status"]
    #[inline(always)]
    pub const fn set_gdet_err_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "GDET0 isolation control"]
    #[must_use]
    #[inline(always)]
    pub const fn gdet_iso_sw(&self) -> super::vals::GdetIsoSw {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::GdetIsoSw::from_bits(val as u8)
    }
    #[doc = "GDET0 isolation control"]
    #[inline(always)]
    pub const fn set_gdet_iso_sw(&mut self, val: super::vals::GdetIsoSw) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Event count value"]
    #[must_use]
    #[inline(always)]
    pub const fn event_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Event count value"]
    #[inline(always)]
    pub const fn set_event_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Positive glitch detected"]
    #[must_use]
    #[inline(always)]
    pub const fn pos_sync(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Positive glitch detected"]
    #[inline(always)]
    pub const fn set_pos_sync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Negative glitch detected"]
    #[must_use]
    #[inline(always)]
    pub const fn neg_sync(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Negative glitch detected"]
    #[inline(always)]
    pub const fn set_neg_sync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Event counter cleared"]
    #[must_use]
    #[inline(always)]
    pub const fn event_clr_flag(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Event counter cleared"]
    #[inline(always)]
    pub const fn set_event_clr_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Gdet0Ctrl {
    #[inline(always)]
    fn default() -> Gdet0Ctrl {
        Gdet0Ctrl(0)
    }
}
impl core::fmt::Debug for Gdet0Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gdet0Ctrl")
            .field("gdet_evtcnt_clr", &self.gdet_evtcnt_clr())
            .field("gdet_err_clr", &self.gdet_err_clr())
            .field("gdet_iso_sw", &self.gdet_iso_sw())
            .field("event_cnt", &self.event_cnt())
            .field("pos_sync", &self.pos_sync())
            .field("neg_sync", &self.neg_sync())
            .field("event_clr_flag", &self.event_clr_flag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gdet0Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gdet0Ctrl {{ gdet_evtcnt_clr: {=bool:?}, gdet_err_clr: {=bool:?}, gdet_iso_sw: {:?}, event_cnt: {=u8:?}, pos_sync: {=bool:?}, neg_sync: {=bool:?}, event_clr_flag: {=bool:?} }}",
            self.gdet_evtcnt_clr(),
            self.gdet_err_clr(),
            self.gdet_iso_sw(),
            self.event_cnt(),
            self.pos_sync(),
            self.neg_sync(),
            self.event_clr_flag()
        )
    }
}
#[doc = "MSF Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msfcfg(pub u32);
impl Msfcfg {
    #[doc = "user IFR block0 sector 0 erase control"]
    #[must_use]
    #[inline(always)]
    pub const fn ifr_block0_erase_dis0(&self) -> super::vals::IfrBlock0EraseDis0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::IfrBlock0EraseDis0::from_bits(val as u8)
    }
    #[doc = "user IFR block0 sector 0 erase control"]
    #[inline(always)]
    pub const fn set_ifr_block0_erase_dis0(&mut self, val: super::vals::IfrBlock0EraseDis0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "user IFR block0 sector 1 erase control"]
    #[must_use]
    #[inline(always)]
    pub const fn ifr_block0_erase_dis1(&self) -> super::vals::IfrBlock0EraseDis1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::IfrBlock0EraseDis1::from_bits(val as u8)
    }
    #[doc = "user IFR block0 sector 1 erase control"]
    #[inline(always)]
    pub const fn set_ifr_block0_erase_dis1(&mut self, val: super::vals::IfrBlock0EraseDis1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "user IFR block0 sector 2 erase control"]
    #[must_use]
    #[inline(always)]
    pub const fn ifr_block0_erase_dis2(&self) -> super::vals::IfrBlock0EraseDis2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::IfrBlock0EraseDis2::from_bits(val as u8)
    }
    #[doc = "user IFR block0 sector 2 erase control"]
    #[inline(always)]
    pub const fn set_ifr_block0_erase_dis2(&mut self, val: super::vals::IfrBlock0EraseDis2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "user IFR block0 sector 3 erase control"]
    #[must_use]
    #[inline(always)]
    pub const fn ifr_block0_erase_dis3(&self) -> super::vals::IfrBlock0EraseDis3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::IfrBlock0EraseDis3::from_bits(val as u8)
    }
    #[doc = "user IFR block0 sector 3 erase control"]
    #[inline(always)]
    pub const fn set_ifr_block0_erase_dis3(&mut self, val: super::vals::IfrBlock0EraseDis3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "user IFR block1 sector 0 erase control"]
    #[must_use]
    #[inline(always)]
    pub const fn ifr_block1_erase_dis0(&self) -> super::vals::IfrBlock1EraseDis0 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::IfrBlock1EraseDis0::from_bits(val as u8)
    }
    #[doc = "user IFR block1 sector 0 erase control"]
    #[inline(always)]
    pub const fn set_ifr_block1_erase_dis0(&mut self, val: super::vals::IfrBlock1EraseDis0) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "user IFR block1 sector 1 erase control"]
    #[must_use]
    #[inline(always)]
    pub const fn ifr_block1_erase_dis1(&self) -> super::vals::IfrBlock1EraseDis1 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::IfrBlock1EraseDis1::from_bits(val as u8)
    }
    #[doc = "user IFR block1 sector 1 erase control"]
    #[inline(always)]
    pub const fn set_ifr_block1_erase_dis1(&mut self, val: super::vals::IfrBlock1EraseDis1) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "user IFR block1 sector 2 erase control"]
    #[must_use]
    #[inline(always)]
    pub const fn ifr_block1_erase_dis2(&self) -> super::vals::IfrBlock1EraseDis2 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::IfrBlock1EraseDis2::from_bits(val as u8)
    }
    #[doc = "user IFR block1 sector 2 erase control"]
    #[inline(always)]
    pub const fn set_ifr_block1_erase_dis2(&mut self, val: super::vals::IfrBlock1EraseDis2) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "user IFR block1 sector 3 erase control"]
    #[must_use]
    #[inline(always)]
    pub const fn ifr_block1_erase_dis3(&self) -> super::vals::IfrBlock1EraseDis3 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::IfrBlock1EraseDis3::from_bits(val as u8)
    }
    #[doc = "user IFR block1 sector 3 erase control"]
    #[inline(always)]
    pub const fn set_ifr_block1_erase_dis3(&mut self, val: super::vals::IfrBlock1EraseDis3) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Mass erase control"]
    #[must_use]
    #[inline(always)]
    pub const fn mass_erase_dis(&self) -> super::vals::MassEraseDis {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::MassEraseDis::from_bits(val as u8)
    }
    #[doc = "Mass erase control"]
    #[inline(always)]
    pub const fn set_mass_erase_dis(&mut self, val: super::vals::MassEraseDis) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "FMU CG override"]
    #[must_use]
    #[inline(always)]
    pub const fn cg_override(&self) -> super::vals::CgOverride {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::CgOverride::from_bits(val as u8)
    }
    #[doc = "FMU CG override"]
    #[inline(always)]
    pub const fn set_cg_override(&mut self, val: super::vals::CgOverride) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for Msfcfg {
    #[inline(always)]
    fn default() -> Msfcfg {
        Msfcfg(0)
    }
}
impl core::fmt::Debug for Msfcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Msfcfg")
            .field("ifr_block0_erase_dis0", &self.ifr_block0_erase_dis0())
            .field("ifr_block0_erase_dis1", &self.ifr_block0_erase_dis1())
            .field("ifr_block0_erase_dis2", &self.ifr_block0_erase_dis2())
            .field("ifr_block0_erase_dis3", &self.ifr_block0_erase_dis3())
            .field("ifr_block1_erase_dis0", &self.ifr_block1_erase_dis0())
            .field("ifr_block1_erase_dis1", &self.ifr_block1_erase_dis1())
            .field("ifr_block1_erase_dis2", &self.ifr_block1_erase_dis2())
            .field("ifr_block1_erase_dis3", &self.ifr_block1_erase_dis3())
            .field("mass_erase_dis", &self.mass_erase_dis())
            .field("cg_override", &self.cg_override())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Msfcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Msfcfg {{ ifr_block0_erase_dis0: {:?}, ifr_block0_erase_dis1: {:?}, ifr_block0_erase_dis2: {:?}, ifr_block0_erase_dis3: {:?}, ifr_block1_erase_dis0: {:?}, ifr_block1_erase_dis1: {:?}, ifr_block1_erase_dis2: {:?}, ifr_block1_erase_dis3: {:?}, mass_erase_dis: {:?}, cg_override: {:?} }}",
            self.ifr_block0_erase_dis0(),
            self.ifr_block0_erase_dis1(),
            self.ifr_block0_erase_dis2(),
            self.ifr_block0_erase_dis3(),
            self.ifr_block1_erase_dis0(),
            self.ifr_block1_erase_dis1(),
            self.ifr_block1_erase_dis2(),
            self.ifr_block1_erase_dis3(),
            self.mass_erase_dis(),
            self.cg_override()
        )
    }
}
#[doc = "RAM XEN Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamXen(pub u32);
impl RamXen {
    #[doc = "RAMX0-3 Execute permission control."]
    #[must_use]
    #[inline(always)]
    pub const fn ramx0_xen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RAMX0-3 Execute permission control."]
    #[inline(always)]
    pub const fn set_ramx0_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RAMXn (excepts RAMA0-3) Execute permission control."]
    #[must_use]
    #[inline(always)]
    pub const fn ramx1_xen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RAMXn (excepts RAMA0-3) Execute permission control."]
    #[inline(always)]
    pub const fn set_ramx1_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RAMA0 Execute permission control."]
    #[must_use]
    #[inline(always)]
    pub const fn rama0_xen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RAMA0 Execute permission control."]
    #[inline(always)]
    pub const fn set_rama0_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "RAMAx (excepts RAMA0) Execute permission control."]
    #[must_use]
    #[inline(always)]
    pub const fn rama1_xen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "RAMAx (excepts RAMA0) Execute permission control."]
    #[inline(always)]
    pub const fn set_rama1_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RAMBx Execute permission control."]
    #[must_use]
    #[inline(always)]
    pub const fn ramb_xen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "RAMBx Execute permission control."]
    #[inline(always)]
    pub const fn set_ramb_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "This 1-bit field provides a mechanism to limit writes to this register (and RAM_XEN_DP) to protect its contents. Once set, this bit remains asserted until a system reset."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This 1-bit field provides a mechanism to limit writes to this register (and RAM_XEN_DP) to protect its contents. Once set, this bit remains asserted until a system reset."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RamXen {
    #[inline(always)]
    fn default() -> RamXen {
        RamXen(0)
    }
}
impl core::fmt::Debug for RamXen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamXen")
            .field("ramx0_xen", &self.ramx0_xen())
            .field("ramx1_xen", &self.ramx1_xen())
            .field("rama0_xen", &self.rama0_xen())
            .field("rama1_xen", &self.rama1_xen())
            .field("ramb_xen", &self.ramb_xen())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamXen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RamXen {{ ramx0_xen: {=bool:?}, ramx1_xen: {=bool:?}, rama0_xen: {=bool:?}, rama1_xen: {=bool:?}, ramb_xen: {=bool:?}, lock: {=bool:?} }}",
            self.ramx0_xen(),
            self.ramx1_xen(),
            self.rama0_xen(),
            self.rama1_xen(),
            self.ramb_xen(),
            self.lock()
        )
    }
}
#[doc = "RAM XEN Control (Duplicate)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamXenDp(pub u32);
impl RamXenDp {
    #[doc = "Refer to RAM_XEN for more details."]
    #[must_use]
    #[inline(always)]
    pub const fn ramx0_xen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Refer to RAM_XEN for more details."]
    #[inline(always)]
    pub const fn set_ramx0_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Refer to RAM_XEN for more details."]
    #[must_use]
    #[inline(always)]
    pub const fn ramx1_xen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Refer to RAM_XEN for more details."]
    #[inline(always)]
    pub const fn set_ramx1_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Refer to RAM_XEN for more details."]
    #[must_use]
    #[inline(always)]
    pub const fn rama0_xen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Refer to RAM_XEN for more details."]
    #[inline(always)]
    pub const fn set_rama0_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Refer to RAM_XEN for more details."]
    #[must_use]
    #[inline(always)]
    pub const fn rama1_xen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Refer to RAM_XEN for more details."]
    #[inline(always)]
    pub const fn set_rama1_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Refer to RAM_XEN for more details."]
    #[must_use]
    #[inline(always)]
    pub const fn ramb_xen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Refer to RAM_XEN for more details."]
    #[inline(always)]
    pub const fn set_ramb_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for RamXenDp {
    #[inline(always)]
    fn default() -> RamXenDp {
        RamXenDp(0)
    }
}
impl core::fmt::Debug for RamXenDp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamXenDp")
            .field("ramx0_xen", &self.ramx0_xen())
            .field("ramx1_xen", &self.ramx1_xen())
            .field("rama0_xen", &self.rama0_xen())
            .field("rama1_xen", &self.rama1_xen())
            .field("ramb_xen", &self.ramb_xen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamXenDp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RamXenDp {{ ramx0_xen: {=bool:?}, ramx1_xen: {=bool:?}, rama0_xen: {=bool:?}, rama1_xen: {=bool:?}, ramb_xen: {=bool:?} }}",
            self.ramx0_xen(),
            self.ramx1_xen(),
            self.rama0_xen(),
            self.rama1_xen(),
            self.ramb_xen()
        )
    }
}
#[doc = "CPU0 Software Debug Access"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwdAccessCpu0(pub u32);
impl SwdAccessCpu0 {
    #[doc = "CPU0 SWD-AP: 0x12345678"]
    #[must_use]
    #[inline(always)]
    pub const fn sec_code(&self) -> super::vals::SecCode {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::SecCode::from_bits(val as u32)
    }
    #[doc = "CPU0 SWD-AP: 0x12345678"]
    #[inline(always)]
    pub const fn set_sec_code(&mut self, val: super::vals::SecCode) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SwdAccessCpu0 {
    #[inline(always)]
    fn default() -> SwdAccessCpu0 {
        SwdAccessCpu0(0)
    }
}
impl core::fmt::Debug for SwdAccessCpu0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwdAccessCpu0")
            .field("sec_code", &self.sec_code())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwdAccessCpu0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SwdAccessCpu0 {{ sec_code: {:?} }}", self.sec_code())
    }
}
