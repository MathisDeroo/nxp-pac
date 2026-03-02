#[doc = "Entropy Read Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ent(pub u32);
impl Ent {
    #[doc = "Entropy Value"]
    #[must_use]
    #[inline(always)]
    pub const fn ent(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Entropy Value"]
    #[inline(always)]
    pub const fn set_ent(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ent {
    #[inline(always)]
    fn default() -> Ent {
        Ent(0)
    }
}
impl core::fmt::Debug for Ent {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ent").field("ent", &self.ent()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ent {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ent {{ ent: {=u32:?} }}", self.ent())
    }
}
#[doc = "Frequency Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frqcnt(pub u32);
impl Frqcnt {
    #[doc = "Frequency Count"]
    #[must_use]
    #[inline(always)]
    pub const fn frq_ct(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Frequency Count"]
    #[inline(always)]
    pub const fn set_frq_ct(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
}
impl Default for Frqcnt {
    #[inline(always)]
    fn default() -> Frqcnt {
        Frqcnt(0)
    }
}
impl core::fmt::Debug for Frqcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frqcnt")
            .field("frq_ct", &self.frq_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frqcnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Frqcnt {{ frq_ct: {=u32:?} }}", self.frq_ct())
    }
}
#[doc = "Frequency Count Maximum Limit Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frqmax(pub u32);
impl Frqmax {
    #[doc = "Frequency Counter Maximum Limit"]
    #[must_use]
    #[inline(always)]
    pub const fn frq_max(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Frequency Counter Maximum Limit"]
    #[inline(always)]
    pub const fn set_frq_max(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
}
impl Default for Frqmax {
    #[inline(always)]
    fn default() -> Frqmax {
        Frqmax(0)
    }
}
impl core::fmt::Debug for Frqmax {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frqmax")
            .field("frq_max", &self.frq_max())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frqmax {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Frqmax {{ frq_max: {=u32:?} }}", self.frq_max())
    }
}
#[doc = "Frequency Count Minimum Limit Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frqmin(pub u32);
impl Frqmin {
    #[doc = "Frequency Count Minimum Limit"]
    #[must_use]
    #[inline(always)]
    pub const fn frq_min(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Frequency Count Minimum Limit"]
    #[inline(always)]
    pub const fn set_frq_min(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
}
impl Default for Frqmin {
    #[inline(always)]
    fn default() -> Frqmin {
        Frqmin(0)
    }
}
impl core::fmt::Debug for Frqmin {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frqmin")
            .field("frq_min", &self.frq_min())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frqmin {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Frqmin {{ frq_min: {=u32:?} }}", self.frq_min())
    }
}
#[doc = "Interrupt Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntCtrl(pub u32);
impl IntCtrl {
    #[doc = "Clear the HW_ERR interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn hw_err(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear the HW_ERR interrupt."]
    #[inline(always)]
    pub const fn set_hw_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clear the ENT_VAL interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ent_val(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Clear the ENT_VAL interrupt."]
    #[inline(always)]
    pub const fn set_ent_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Clear the FRQ_CT_FAIL interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn frq_ct_fail(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Clear the FRQ_CT_FAIL interrupt."]
    #[inline(always)]
    pub const fn set_frq_ct_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clear the INTG_FLT interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn intg_flt(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Clear the INTG_FLT interrupt."]
    #[inline(always)]
    pub const fn set_intg_flt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for IntCtrl {
    #[inline(always)]
    fn default() -> IntCtrl {
        IntCtrl(0)
    }
}
impl core::fmt::Debug for IntCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntCtrl")
            .field("hw_err", &self.hw_err())
            .field("ent_val", &self.ent_val())
            .field("frq_ct_fail", &self.frq_ct_fail())
            .field("intg_flt", &self.intg_flt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntCtrl {{ hw_err: {=bool:?}, ent_val: {=bool:?}, frq_ct_fail: {=bool:?}, intg_flt: {=bool:?} }}",
            self.hw_err(),
            self.ent_val(),
            self.frq_ct_fail(),
            self.intg_flt()
        )
    }
}
#[doc = "Mask Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntMask(pub u32);
impl IntMask {
    #[doc = "Mask the HW_ERR interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn hw_err(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mask the HW_ERR interrupt."]
    #[inline(always)]
    pub const fn set_hw_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mask the ENT_VAL interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ent_val(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Mask the ENT_VAL interrupt."]
    #[inline(always)]
    pub const fn set_ent_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Mask the FRQ_CT_FAIL interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn frq_ct_fail(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Mask the FRQ_CT_FAIL interrupt."]
    #[inline(always)]
    pub const fn set_frq_ct_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Mask the INTG_FLT interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn intg_flt(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Mask the INTG_FLT interrupt."]
    #[inline(always)]
    pub const fn set_intg_flt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for IntMask {
    #[inline(always)]
    fn default() -> IntMask {
        IntMask(0)
    }
}
impl core::fmt::Debug for IntMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntMask")
            .field("hw_err", &self.hw_err())
            .field("ent_val", &self.ent_val())
            .field("frq_ct_fail", &self.frq_ct_fail())
            .field("intg_flt", &self.intg_flt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntMask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntMask {{ hw_err: {=bool:?}, ent_val: {=bool:?}, frq_ct_fail: {=bool:?}, intg_flt: {=bool:?} }}",
            self.hw_err(),
            self.ent_val(),
            self.frq_ct_fail(),
            self.intg_flt()
        )
    }
}
#[doc = "Interrupt Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntStatus(pub u32);
impl IntStatus {
    #[doc = "Read: TRNG Error. Any error in the TRNG will trigger this interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn hw_err(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: TRNG Error. Any error in the TRNG will trigger this interrupt."]
    #[inline(always)]
    pub const fn set_hw_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Entropy Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn ent_val(&self) -> super::vals::IntStatusEntVal {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::IntStatusEntVal::from_bits(val as u8)
    }
    #[doc = "Entropy Valid"]
    #[inline(always)]
    pub const fn set_ent_val(&mut self, val: super::vals::IntStatusEntVal) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Frequency Count Fail"]
    #[must_use]
    #[inline(always)]
    pub const fn frq_ct_fail(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Frequency Count Fail"]
    #[inline(always)]
    pub const fn set_frq_ct_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Integrity Fault. An internal fault has occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn intg_flt(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Integrity Fault. An internal fault has occurred."]
    #[inline(always)]
    pub const fn set_intg_flt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for IntStatus {
    #[inline(always)]
    fn default() -> IntStatus {
        IntStatus(0)
    }
}
impl core::fmt::Debug for IntStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntStatus")
            .field("hw_err", &self.hw_err())
            .field("ent_val", &self.ent_val())
            .field("frq_ct_fail", &self.frq_ct_fail())
            .field("intg_flt", &self.intg_flt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntStatus {{ hw_err: {=bool:?}, ent_val: {:?}, frq_ct_fail: {=bool:?}, intg_flt: {=bool:?} }}",
            self.hw_err(),
            self.ent_val(),
            self.frq_ct_fail(),
            self.intg_flt()
        )
    }
}
#[doc = "Miscellaneous Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mctl(pub u32);
impl Mctl {
    #[doc = "Oscillator1 Divide"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_div(&self) -> super::vals::OscDiv {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::OscDiv::from_bits(val as u8)
    }
    #[doc = "Oscillator1 Divide"]
    #[inline(always)]
    pub const fn set_osc_div(&mut self, val: super::vals::OscDiv) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Clock Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_out_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Output Enable"]
    #[inline(always)]
    pub const fn set_clk_out_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "TRNG Access Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn trng_acc(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG Access Mode"]
    #[inline(always)]
    pub const fn set_trng_acc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Reset Defaults"]
    #[must_use]
    #[inline(always)]
    pub const fn rst_def(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Reset Defaults"]
    #[inline(always)]
    pub const fn set_rst_def(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Frequency Count Fail"]
    #[must_use]
    #[inline(always)]
    pub const fn fct_fail(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Frequency Count Fail"]
    #[inline(always)]
    pub const fn set_fct_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Frequency Count Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn fct_val(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Frequency Count Valid"]
    #[inline(always)]
    pub const fn set_fct_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Entropy Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn ent_val(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Entropy Valid"]
    #[inline(always)]
    pub const fn set_ent_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Error Status"]
    #[must_use]
    #[inline(always)]
    pub const fn err(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Error Status"]
    #[inline(always)]
    pub const fn set_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "TRNG is ok to stop"]
    #[must_use]
    #[inline(always)]
    pub const fn tstop_ok(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG is ok to stop"]
    #[inline(always)]
    pub const fn set_tstop_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Oscillator 2 Failure"]
    #[must_use]
    #[inline(always)]
    pub const fn osc2_fail(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Oscillator 2 Failure"]
    #[inline(always)]
    pub const fn set_osc2_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Program Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn prgm(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Program Mode"]
    #[inline(always)]
    pub const fn set_prgm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Mctl {
    #[inline(always)]
    fn default() -> Mctl {
        Mctl(0)
    }
}
impl core::fmt::Debug for Mctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mctl")
            .field("osc_div", &self.osc_div())
            .field("clk_out_en", &self.clk_out_en())
            .field("trng_acc", &self.trng_acc())
            .field("rst_def", &self.rst_def())
            .field("fct_fail", &self.fct_fail())
            .field("fct_val", &self.fct_val())
            .field("ent_val", &self.ent_val())
            .field("err", &self.err())
            .field("tstop_ok", &self.tstop_ok())
            .field("osc2_fail", &self.osc2_fail())
            .field("prgm", &self.prgm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mctl {{ osc_div: {:?}, clk_out_en: {=bool:?}, trng_acc: {=bool:?}, rst_def: {=bool:?}, fct_fail: {=bool:?}, fct_val: {=bool:?}, ent_val: {=bool:?}, err: {=bool:?}, tstop_ok: {=bool:?}, osc2_fail: {=bool:?}, prgm: {=bool:?} }}",
            self.osc_div(),
            self.clk_out_en(),
            self.trng_acc(),
            self.rst_def(),
            self.fct_fail(),
            self.fct_val(),
            self.ent_val(),
            self.err(),
            self.tstop_ok(),
            self.osc2_fail(),
            self.prgm()
        )
    }
}
#[doc = "TRNG Oscillator 2 Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osc2Ctl(pub u32);
impl Osc2Ctl {
    #[doc = "TRNG entropy generation control."]
    #[must_use]
    #[inline(always)]
    pub const fn trng_ent_ctl(&self) -> super::vals::TrngEntCtl {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::TrngEntCtl::from_bits(val as u8)
    }
    #[doc = "TRNG entropy generation control."]
    #[inline(always)]
    pub const fn set_trng_ent_ctl(&mut self, val: super::vals::TrngEntCtl) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Oscillator 2 Divide."]
    #[must_use]
    #[inline(always)]
    pub const fn osc2_div(&self) -> super::vals::Osc2Div {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Osc2Div::from_bits(val as u8)
    }
    #[doc = "Oscillator 2 Divide."]
    #[inline(always)]
    pub const fn set_osc2_div(&mut self, val: super::vals::Osc2Div) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Oscillator 2 Clock Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn osc2_out_en(&self) -> super::vals::Osc2OutEn {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Osc2OutEn::from_bits(val as u8)
    }
    #[doc = "Oscillator 2 Clock Output Enable"]
    #[inline(always)]
    pub const fn set_osc2_out_en(&mut self, val: super::vals::Osc2OutEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "TRNG Oscillator 2 Frequency Count Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn osc2_fct_val(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG Oscillator 2 Frequency Count Valid"]
    #[inline(always)]
    pub const fn set_osc2_fct_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Oscillator fail safe limit."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_failsafe_lmt(&self) -> super::vals::OscFailsafeLmt {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::OscFailsafeLmt::from_bits(val as u8)
    }
    #[doc = "Oscillator fail safe limit."]
    #[inline(always)]
    pub const fn set_osc_failsafe_lmt(&mut self, val: super::vals::OscFailsafeLmt) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Oscillator fail safe test."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_failsafe_test(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Oscillator fail safe test."]
    #[inline(always)]
    pub const fn set_osc_failsafe_test(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Osc2Ctl {
    #[inline(always)]
    fn default() -> Osc2Ctl {
        Osc2Ctl(0)
    }
}
impl core::fmt::Debug for Osc2Ctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Osc2Ctl")
            .field("trng_ent_ctl", &self.trng_ent_ctl())
            .field("osc2_div", &self.osc2_div())
            .field("osc2_out_en", &self.osc2_out_en())
            .field("osc2_fct_val", &self.osc2_fct_val())
            .field("osc_failsafe_lmt", &self.osc_failsafe_lmt())
            .field("osc_failsafe_test", &self.osc_failsafe_test())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Osc2Ctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Osc2Ctl {{ trng_ent_ctl: {:?}, osc2_div: {:?}, osc2_out_en: {:?}, osc2_fct_val: {=bool:?}, osc_failsafe_lmt: {:?}, osc_failsafe_test: {=bool:?} }}",
            self.trng_ent_ctl(),
            self.osc2_div(),
            self.osc2_out_en(),
            self.osc2_fct_val(),
            self.osc_failsafe_lmt(),
            self.osc_failsafe_test()
        )
    }
}
#[doc = "Oscillator-2 Frequency Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osc2Frqcnt(pub u32);
impl Osc2Frqcnt {
    #[doc = "Frequency Count"]
    #[must_use]
    #[inline(always)]
    pub const fn osc2_frq_ct(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Frequency Count"]
    #[inline(always)]
    pub const fn set_osc2_frq_ct(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
}
impl Default for Osc2Frqcnt {
    #[inline(always)]
    fn default() -> Osc2Frqcnt {
        Osc2Frqcnt(0)
    }
}
impl core::fmt::Debug for Osc2Frqcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Osc2Frqcnt")
            .field("osc2_frq_ct", &self.osc2_frq_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Osc2Frqcnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Osc2Frqcnt {{ osc2_frq_ct: {=u32:?} }}",
            self.osc2_frq_ct()
        )
    }
}
#[doc = "Statistical Check Monobit Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scmc(pub u32);
impl Scmc {
    #[doc = "Monobit Count"]
    #[must_use]
    #[inline(always)]
    pub const fn mono_ct(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Monobit Count"]
    #[inline(always)]
    pub const fn set_mono_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Scmc {
    #[inline(always)]
    fn default() -> Scmc {
        Scmc(0)
    }
}
impl core::fmt::Debug for Scmc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scmc")
            .field("mono_ct", &self.mono_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scmc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Scmc {{ mono_ct: {=u16:?} }}", self.mono_ct())
    }
}
#[doc = "Statistical Check Miscellaneous Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scmisc(pub u32);
impl Scmisc {
    #[doc = "Long run max limit"]
    #[must_use]
    #[inline(always)]
    pub const fn lrun_max(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Long run max limit"]
    #[inline(always)]
    pub const fn set_lrun_max(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Retry count"]
    #[must_use]
    #[inline(always)]
    pub const fn rty_ct(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Retry count"]
    #[inline(always)]
    pub const fn set_rty_ct(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Scmisc {
    #[inline(always)]
    fn default() -> Scmisc {
        Scmisc(0)
    }
}
impl core::fmt::Debug for Scmisc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scmisc")
            .field("lrun_max", &self.lrun_max())
            .field("rty_ct", &self.rty_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scmisc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scmisc {{ lrun_max: {=u8:?}, rty_ct: {=u8:?} }}",
            self.lrun_max(),
            self.rty_ct()
        )
    }
}
#[doc = "Statistical Check Monobit Limit Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scml(pub u32);
impl Scml {
    #[doc = "Monobit Maximum Limit"]
    #[must_use]
    #[inline(always)]
    pub const fn mono_max(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Monobit Maximum Limit"]
    #[inline(always)]
    pub const fn set_mono_max(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Monobit Range"]
    #[must_use]
    #[inline(always)]
    pub const fn mono_rng(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Monobit Range"]
    #[inline(always)]
    pub const fn set_mono_rng(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Scml {
    #[inline(always)]
    fn default() -> Scml {
        Scml(0)
    }
}
impl core::fmt::Debug for Scml {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scml")
            .field("mono_max", &self.mono_max())
            .field("mono_rng", &self.mono_rng())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scml {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scml {{ mono_max: {=u16:?}, mono_rng: {=u16:?} }}",
            self.mono_max(),
            self.mono_rng()
        )
    }
}
#[doc = "Statistical Check Run Length 1 Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr1c(pub u32);
impl Scr1c {
    #[doc = "Runs of Zero, Length 1 Count"]
    #[must_use]
    #[inline(always)]
    pub const fn r1_0_ct(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Runs of Zero, Length 1 Count"]
    #[inline(always)]
    pub const fn set_r1_0_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[doc = "Runs of One, Length 1 Count"]
    #[must_use]
    #[inline(always)]
    pub const fn r1_1_ct(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x7fff;
        val as u16
    }
    #[doc = "Runs of One, Length 1 Count"]
    #[inline(always)]
    pub const fn set_r1_1_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
    }
}
impl Default for Scr1c {
    #[inline(always)]
    fn default() -> Scr1c {
        Scr1c(0)
    }
}
impl core::fmt::Debug for Scr1c {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr1c")
            .field("r1_0_ct", &self.r1_0_ct())
            .field("r1_1_ct", &self.r1_1_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr1c {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr1c {{ r1_0_ct: {=u16:?}, r1_1_ct: {=u16:?} }}",
            self.r1_0_ct(),
            self.r1_1_ct()
        )
    }
}
#[doc = "Statistical Check Run Length 1 Limit Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr1l(pub u32);
impl Scr1l {
    #[doc = "Run Length 1 Maximum Limit"]
    #[must_use]
    #[inline(always)]
    pub const fn run1_max(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Run Length 1 Maximum Limit"]
    #[inline(always)]
    pub const fn set_run1_max(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[doc = "Run Length 1 Range"]
    #[must_use]
    #[inline(always)]
    pub const fn run1_rng(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x7fff;
        val as u16
    }
    #[doc = "Run Length 1 Range"]
    #[inline(always)]
    pub const fn set_run1_rng(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
    }
}
impl Default for Scr1l {
    #[inline(always)]
    fn default() -> Scr1l {
        Scr1l(0)
    }
}
impl core::fmt::Debug for Scr1l {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr1l")
            .field("run1_max", &self.run1_max())
            .field("run1_rng", &self.run1_rng())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr1l {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr1l {{ run1_max: {=u16:?}, run1_rng: {=u16:?} }}",
            self.run1_max(),
            self.run1_rng()
        )
    }
}
#[doc = "Statistical Check Run Length 2 Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr2c(pub u32);
impl Scr2c {
    #[doc = "Runs of Zero, Length 2 Count"]
    #[must_use]
    #[inline(always)]
    pub const fn r2_0_ct(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Runs of Zero, Length 2 Count"]
    #[inline(always)]
    pub const fn set_r2_0_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "Runs of One, Length 2 Count"]
    #[must_use]
    #[inline(always)]
    pub const fn r2_1_ct(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x3fff;
        val as u16
    }
    #[doc = "Runs of One, Length 2 Count"]
    #[inline(always)]
    pub const fn set_r2_1_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
    }
}
impl Default for Scr2c {
    #[inline(always)]
    fn default() -> Scr2c {
        Scr2c(0)
    }
}
impl core::fmt::Debug for Scr2c {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr2c")
            .field("r2_0_ct", &self.r2_0_ct())
            .field("r2_1_ct", &self.r2_1_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr2c {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr2c {{ r2_0_ct: {=u16:?}, r2_1_ct: {=u16:?} }}",
            self.r2_0_ct(),
            self.r2_1_ct()
        )
    }
}
#[doc = "Statistical Check Run Length 2 Limit Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr2l(pub u32);
impl Scr2l {
    #[doc = "Run Length 2 Maximum Limit"]
    #[must_use]
    #[inline(always)]
    pub const fn run2_max(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Run Length 2 Maximum Limit"]
    #[inline(always)]
    pub const fn set_run2_max(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "Run Length 2 Range"]
    #[must_use]
    #[inline(always)]
    pub const fn run2_rng(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x3fff;
        val as u16
    }
    #[doc = "Run Length 2 Range"]
    #[inline(always)]
    pub const fn set_run2_rng(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
    }
}
impl Default for Scr2l {
    #[inline(always)]
    fn default() -> Scr2l {
        Scr2l(0)
    }
}
impl core::fmt::Debug for Scr2l {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr2l")
            .field("run2_max", &self.run2_max())
            .field("run2_rng", &self.run2_rng())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr2l {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr2l {{ run2_max: {=u16:?}, run2_rng: {=u16:?} }}",
            self.run2_max(),
            self.run2_rng()
        )
    }
}
#[doc = "Statistical Check Run Length 3 Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr3c(pub u32);
impl Scr3c {
    #[doc = "Runs of Zeroes, Length 3 Count"]
    #[must_use]
    #[inline(always)]
    pub const fn r3_0_ct(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "Runs of Zeroes, Length 3 Count"]
    #[inline(always)]
    pub const fn set_r3_0_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
    #[doc = "Runs of Ones, Length 3 Count"]
    #[must_use]
    #[inline(always)]
    pub const fn r3_1_ct(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x1fff;
        val as u16
    }
    #[doc = "Runs of Ones, Length 3 Count"]
    #[inline(always)]
    pub const fn set_r3_1_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
    }
}
impl Default for Scr3c {
    #[inline(always)]
    fn default() -> Scr3c {
        Scr3c(0)
    }
}
impl core::fmt::Debug for Scr3c {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr3c")
            .field("r3_0_ct", &self.r3_0_ct())
            .field("r3_1_ct", &self.r3_1_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr3c {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr3c {{ r3_0_ct: {=u16:?}, r3_1_ct: {=u16:?} }}",
            self.r3_0_ct(),
            self.r3_1_ct()
        )
    }
}
#[doc = "Statistical Check Run Length 3 Limit Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr3l(pub u32);
impl Scr3l {
    #[doc = "Run Length 3 Maximum Limit"]
    #[must_use]
    #[inline(always)]
    pub const fn run3_max(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "Run Length 3 Maximum Limit"]
    #[inline(always)]
    pub const fn set_run3_max(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
    #[doc = "Run Length 3 Range"]
    #[must_use]
    #[inline(always)]
    pub const fn run3_rng(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x1fff;
        val as u16
    }
    #[doc = "Run Length 3 Range"]
    #[inline(always)]
    pub const fn set_run3_rng(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
    }
}
impl Default for Scr3l {
    #[inline(always)]
    fn default() -> Scr3l {
        Scr3l(0)
    }
}
impl core::fmt::Debug for Scr3l {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr3l")
            .field("run3_max", &self.run3_max())
            .field("run3_rng", &self.run3_rng())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr3l {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr3l {{ run3_max: {=u16:?}, run3_rng: {=u16:?} }}",
            self.run3_max(),
            self.run3_rng()
        )
    }
}
#[doc = "Seed Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdctl(pub u32);
impl Sdctl {
    #[doc = "Sample Size"]
    #[must_use]
    #[inline(always)]
    pub const fn samp_size(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Sample Size"]
    #[inline(always)]
    pub const fn set_samp_size(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Entropy Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn ent_dly(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Entropy Delay"]
    #[inline(always)]
    pub const fn set_ent_dly(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Sdctl {
    #[inline(always)]
    fn default() -> Sdctl {
        Sdctl(0)
    }
}
impl core::fmt::Debug for Sdctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sdctl")
            .field("samp_size", &self.samp_size())
            .field("ent_dly", &self.ent_dly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sdctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sdctl {{ samp_size: {=u16:?}, ent_dly: {=u16:?} }}",
            self.samp_size(),
            self.ent_dly()
        )
    }
}
#[doc = "Security Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCfg(pub u32);
impl SecCfg {
    #[doc = "If set, below mentioned TRNG configuration registers cannot be programmed: Oscillator 2 Control Register (OSC2_CTL): TRNG Entropy Generation Control \\[1:0\\] Oscillator 2 Divider \\[3:2\\] Oscillator Fail Safe Limit \\[13:12\\] Oscillator Fail Safe Test \\[14\\] TRNG Seed Control Register (SDCTL) TRNG Frequency Count Minimum Limit Register (FRQMIN) TRNG Frequency Count Maximum Limit Register (FRQMAX) TRNG Statistical Check Monobit Limit Register (SCML) TRNG Statistical Check Run Length 1 Limit Register (SCR1L) TRNG Statistical Check Run Length 2 Limit Register (SCR2L) TRNG Statistical Check Run Length 3 Limit Register (SCR3L) TRNG Miscellaneous Control Register (MCTL): Sample Mode \\[1:0\\] Oscillator Divider \\[3:2\\] Reset Defaults \\[6\\] Force System Clock \\[7\\] Long Runs Continuation Mode \\[14\\] After this bit has been written to a 1, it cannot be changed"]
    #[must_use]
    #[inline(always)]
    pub const fn no_prgm(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If set, below mentioned TRNG configuration registers cannot be programmed: Oscillator 2 Control Register (OSC2_CTL): TRNG Entropy Generation Control \\[1:0\\] Oscillator 2 Divider \\[3:2\\] Oscillator Fail Safe Limit \\[13:12\\] Oscillator Fail Safe Test \\[14\\] TRNG Seed Control Register (SDCTL) TRNG Frequency Count Minimum Limit Register (FRQMIN) TRNG Frequency Count Maximum Limit Register (FRQMAX) TRNG Statistical Check Monobit Limit Register (SCML) TRNG Statistical Check Run Length 1 Limit Register (SCR1L) TRNG Statistical Check Run Length 2 Limit Register (SCR2L) TRNG Statistical Check Run Length 3 Limit Register (SCR3L) TRNG Miscellaneous Control Register (MCTL): Sample Mode \\[1:0\\] Oscillator Divider \\[3:2\\] Reset Defaults \\[6\\] Force System Clock \\[7\\] Long Runs Continuation Mode \\[14\\] After this bit has been written to a 1, it cannot be changed"]
    #[inline(always)]
    pub const fn set_no_prgm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for SecCfg {
    #[inline(always)]
    fn default() -> SecCfg {
        SecCfg(0)
    }
}
impl core::fmt::Debug for SecCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCfg")
            .field("no_prgm", &self.no_prgm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SecCfg {{ no_prgm: {=bool:?} }}", self.no_prgm())
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Test Fail, 1-Bit Run, Sampling 0s."]
    #[must_use]
    #[inline(always)]
    pub const fn tf1br0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, 1-Bit Run, Sampling 0s."]
    #[inline(always)]
    pub const fn set_tf1br0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Test Fail, 1-Bit Run, Sampling 1s."]
    #[must_use]
    #[inline(always)]
    pub const fn tf1br1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, 1-Bit Run, Sampling 1s."]
    #[inline(always)]
    pub const fn set_tf1br1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Test Fail, 2-Bit Run, Sampling 0s."]
    #[must_use]
    #[inline(always)]
    pub const fn tf2br0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, 2-Bit Run, Sampling 0s."]
    #[inline(always)]
    pub const fn set_tf2br0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Test Fail, 2-Bit Run, Sampling 1s."]
    #[must_use]
    #[inline(always)]
    pub const fn tf2br1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, 2-Bit Run, Sampling 1s."]
    #[inline(always)]
    pub const fn set_tf2br1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Test Fail, 3-Bit Run, Sampling 0s."]
    #[must_use]
    #[inline(always)]
    pub const fn tf3br0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, 3-Bit Run, Sampling 0s."]
    #[inline(always)]
    pub const fn set_tf3br0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Test Fail"]
    #[must_use]
    #[inline(always)]
    pub const fn tf3br1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail"]
    #[inline(always)]
    pub const fn set_tf3br1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Test Fail, Long Run."]
    #[must_use]
    #[inline(always)]
    pub const fn tflr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, Long Run."]
    #[inline(always)]
    pub const fn set_tflr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Test Fail, Mono Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn tfmb(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, Mono Bit."]
    #[inline(always)]
    pub const fn set_tfmb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "RETRY COUNT"]
    #[must_use]
    #[inline(always)]
    pub const fn retry_ct(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "RETRY COUNT"]
    #[inline(always)]
    pub const fn set_retry_ct(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status")
            .field("tf1br0", &self.tf1br0())
            .field("tf1br1", &self.tf1br1())
            .field("tf2br0", &self.tf2br0())
            .field("tf2br1", &self.tf2br1())
            .field("tf3br0", &self.tf3br0())
            .field("tf3br1", &self.tf3br1())
            .field("tflr", &self.tflr())
            .field("tfmb", &self.tfmb())
            .field("retry_ct", &self.retry_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{ tf1br0: {=bool:?}, tf1br1: {=bool:?}, tf2br0: {=bool:?}, tf2br1: {=bool:?}, tf3br0: {=bool:?}, tf3br1: {=bool:?}, tflr: {=bool:?}, tfmb: {=bool:?}, retry_ct: {=u8:?} }}",
            self.tf1br0(),
            self.tf1br1(),
            self.tf2br0(),
            self.tf2br1(),
            self.tf3br0(),
            self.tf3br1(),
            self.tflr(),
            self.tfmb(),
            self.retry_ct()
        )
    }
}
#[doc = "Version ID Register (MS)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vid1(pub u32);
impl Vid1 {
    #[doc = "Shows the IP's Minor revision of the TRNG."]
    #[must_use]
    #[inline(always)]
    pub const fn min_rev(&self) -> super::vals::MinRev {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::MinRev::from_bits(val as u8)
    }
    #[doc = "Shows the IP's Minor revision of the TRNG."]
    #[inline(always)]
    pub const fn set_min_rev(&mut self, val: super::vals::MinRev) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "Shows the IP's Major revision of the TRNG"]
    #[must_use]
    #[inline(always)]
    pub const fn maj_rev(&self) -> super::vals::MajRev {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::MajRev::from_bits(val as u8)
    }
    #[doc = "Shows the IP's Major revision of the TRNG"]
    #[inline(always)]
    pub const fn set_maj_rev(&mut self, val: super::vals::MajRev) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
    }
    #[doc = "Shows the IP ID."]
    #[must_use]
    #[inline(always)]
    pub const fn ip_id(&self) -> super::vals::IpId {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::IpId::from_bits(val as u16)
    }
    #[doc = "Shows the IP ID."]
    #[inline(always)]
    pub const fn set_ip_id(&mut self, val: super::vals::IpId) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for Vid1 {
    #[inline(always)]
    fn default() -> Vid1 {
        Vid1(0)
    }
}
impl core::fmt::Debug for Vid1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vid1")
            .field("min_rev", &self.min_rev())
            .field("maj_rev", &self.maj_rev())
            .field("ip_id", &self.ip_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vid1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Vid1 {{ min_rev: {:?}, maj_rev: {:?}, ip_id: {:?} }}",
            self.min_rev(),
            self.maj_rev(),
            self.ip_id()
        )
    }
}
#[doc = "Version ID Register (LS)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vid2(pub u32);
impl Vid2 {
    #[doc = "Shows the IP's Configuaration options for the TRNG."]
    #[must_use]
    #[inline(always)]
    pub const fn config_opt(&self) -> super::vals::ConfigOpt {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::ConfigOpt::from_bits(val as u8)
    }
    #[doc = "Shows the IP's Configuaration options for the TRNG."]
    #[inline(always)]
    pub const fn set_config_opt(&mut self, val: super::vals::ConfigOpt) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "Shows the IP's ECO revision of the TRNG."]
    #[must_use]
    #[inline(always)]
    pub const fn eco_rev(&self) -> super::vals::EcoRev {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::EcoRev::from_bits(val as u8)
    }
    #[doc = "Shows the IP's ECO revision of the TRNG."]
    #[inline(always)]
    pub const fn set_eco_rev(&mut self, val: super::vals::EcoRev) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
    }
    #[doc = "Shows the integration options for the TRNG."]
    #[must_use]
    #[inline(always)]
    pub const fn intg_opt(&self) -> super::vals::IntgOpt {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::IntgOpt::from_bits(val as u8)
    }
    #[doc = "Shows the integration options for the TRNG."]
    #[inline(always)]
    pub const fn set_intg_opt(&mut self, val: super::vals::IntgOpt) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Shows the ERA of the TRNG."]
    #[must_use]
    #[inline(always)]
    pub const fn era(&self) -> super::vals::Era {
        let val = (self.0 >> 24usize) & 0xff;
        super::vals::Era::from_bits(val as u8)
    }
    #[doc = "Shows the ERA of the TRNG."]
    #[inline(always)]
    pub const fn set_era(&mut self, val: super::vals::Era) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
    }
}
impl Default for Vid2 {
    #[inline(always)]
    fn default() -> Vid2 {
        Vid2(0)
    }
}
impl core::fmt::Debug for Vid2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vid2")
            .field("config_opt", &self.config_opt())
            .field("eco_rev", &self.eco_rev())
            .field("intg_opt", &self.intg_opt())
            .field("era", &self.era())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vid2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Vid2 {{ config_opt: {:?}, eco_rev: {:?}, intg_opt: {:?}, era: {:?} }}",
            self.config_opt(),
            self.eco_rev(),
            self.intg_opt(),
            self.era()
        )
    }
}
