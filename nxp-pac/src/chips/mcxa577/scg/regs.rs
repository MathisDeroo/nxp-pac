#[doc = "Clock Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc = "System Clock Source"]
    #[must_use]
    #[inline(always)]
    pub const fn scs(&self) -> super::vals::Scs {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Scs::from_bits(val as u8)
    }
    #[doc = "System Clock Source"]
    #[inline(always)]
    pub const fn set_scs(&mut self, val: super::vals::Scs) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
}
impl Default for Csr {
    #[inline(always)]
    fn default() -> Csr {
        Csr(0)
    }
}
impl core::fmt::Debug for Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Csr").field("scs", &self.scs()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Csr {{ scs: {:?} }}", self.scs())
    }
}
#[doc = "FIRC Auto-trimming Counter 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircatc1(pub u32);
impl Fircatc1 {
    #[doc = "Ideal Counter SOSC"]
    #[must_use]
    #[inline(always)]
    pub const fn idealc_sosc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Ideal Counter SOSC"]
    #[inline(always)]
    pub const fn set_idealc_sosc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Sample Cycle SOSC"]
    #[must_use]
    #[inline(always)]
    pub const fn samcyc_sosc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Sample Cycle SOSC"]
    #[inline(always)]
    pub const fn set_samcyc_sosc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Setup Cycle SOSC"]
    #[must_use]
    #[inline(always)]
    pub const fn setcyc_sosc(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Setup Cycle SOSC"]
    #[inline(always)]
    pub const fn set_setcyc_sosc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Fircatc1 {
    #[inline(always)]
    fn default() -> Fircatc1 {
        Fircatc1(0)
    }
}
impl core::fmt::Debug for Fircatc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircatc1")
            .field("idealc_sosc", &self.idealc_sosc())
            .field("samcyc_sosc", &self.samcyc_sosc())
            .field("setcyc_sosc", &self.setcyc_sosc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircatc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircatc1 {{ idealc_sosc: {=u16:?}, samcyc_sosc: {=u8:?}, setcyc_sosc: {=u8:?} }}",
            self.idealc_sosc(),
            self.samcyc_sosc(),
            self.setcyc_sosc()
        )
    }
}
#[doc = "FIRC Auto-trimming Counter 10"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircatc10(pub u32);
impl Fircatc10 {
    #[doc = "Fine Trim Maximum Counter SOF"]
    #[must_use]
    #[inline(always)]
    pub const fn finemaxc_sof(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Fine Trim Maximum Counter SOF"]
    #[inline(always)]
    pub const fn set_finemaxc_sof(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Fircatc10 {
    #[inline(always)]
    fn default() -> Fircatc10 {
        Fircatc10(0)
    }
}
impl core::fmt::Debug for Fircatc10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircatc10")
            .field("finemaxc_sof", &self.finemaxc_sof())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircatc10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircatc10 {{ finemaxc_sof: {=u32:?} }}",
            self.finemaxc_sof()
        )
    }
}
#[doc = "FIRC Auto-trimming Counter 11"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircatc11(pub u32);
impl Fircatc11 {
    #[doc = "Fine Trim Minimum Counter SOF"]
    #[must_use]
    #[inline(always)]
    pub const fn fineminc_sof(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Fine Trim Minimum Counter SOF"]
    #[inline(always)]
    pub const fn set_fineminc_sof(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Fircatc11 {
    #[inline(always)]
    fn default() -> Fircatc11 {
        Fircatc11(0)
    }
}
impl core::fmt::Debug for Fircatc11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircatc11")
            .field("fineminc_sof", &self.fineminc_sof())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircatc11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircatc11 {{ fineminc_sof: {=u32:?} }}",
            self.fineminc_sof()
        )
    }
}
#[doc = "FIRC Auto-trimming Counter 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircatc2(pub u32);
impl Fircatc2 {
    #[doc = "Coarse Trim Minimum Counter SOSC"]
    #[must_use]
    #[inline(always)]
    pub const fn coarminc_sosc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Coarse Trim Minimum Counter SOSC"]
    #[inline(always)]
    pub const fn set_coarminc_sosc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Coarse Trim Maximum Counter SOSC"]
    #[must_use]
    #[inline(always)]
    pub const fn coarmaxc_sosc(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Coarse Trim Maximum Counter SOSC"]
    #[inline(always)]
    pub const fn set_coarmaxc_sosc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Fircatc2 {
    #[inline(always)]
    fn default() -> Fircatc2 {
        Fircatc2(0)
    }
}
impl core::fmt::Debug for Fircatc2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircatc2")
            .field("coarminc_sosc", &self.coarminc_sosc())
            .field("coarmaxc_sosc", &self.coarmaxc_sosc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircatc2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircatc2 {{ coarminc_sosc: {=u16:?}, coarmaxc_sosc: {=u16:?} }}",
            self.coarminc_sosc(),
            self.coarmaxc_sosc()
        )
    }
}
#[doc = "FIRC Auto-trimming Counter 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircatc3(pub u32);
impl Fircatc3 {
    #[doc = "Fine Trim Minimum Counter SOSC"]
    #[must_use]
    #[inline(always)]
    pub const fn fineminc_sosc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Fine Trim Minimum Counter SOSC"]
    #[inline(always)]
    pub const fn set_fineminc_sosc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Fine Trim Maximum Counter SOSC"]
    #[must_use]
    #[inline(always)]
    pub const fn finemaxc_sosc(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Fine Trim Maximum Counter SOSC"]
    #[inline(always)]
    pub const fn set_finemaxc_sosc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Fircatc3 {
    #[inline(always)]
    fn default() -> Fircatc3 {
        Fircatc3(0)
    }
}
impl core::fmt::Debug for Fircatc3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircatc3")
            .field("fineminc_sosc", &self.fineminc_sosc())
            .field("finemaxc_sosc", &self.finemaxc_sosc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircatc3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircatc3 {{ fineminc_sosc: {=u16:?}, finemaxc_sosc: {=u16:?} }}",
            self.fineminc_sosc(),
            self.finemaxc_sosc()
        )
    }
}
#[doc = "FIRC Auto-trimming Counter 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircatc4(pub u32);
impl Fircatc4 {
    #[doc = "Ideal Counter ROSC"]
    #[must_use]
    #[inline(always)]
    pub const fn idealc_rosc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Ideal Counter ROSC"]
    #[inline(always)]
    pub const fn set_idealc_rosc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Setup Cycle ROSC"]
    #[must_use]
    #[inline(always)]
    pub const fn setcyc_rosc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Setup Cycle ROSC"]
    #[inline(always)]
    pub const fn set_setcyc_rosc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Sample Cycle ROSC"]
    #[must_use]
    #[inline(always)]
    pub const fn samcyc_rosc(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Sample Cycle ROSC"]
    #[inline(always)]
    pub const fn set_samcyc_rosc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Fircatc4 {
    #[inline(always)]
    fn default() -> Fircatc4 {
        Fircatc4(0)
    }
}
impl core::fmt::Debug for Fircatc4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircatc4")
            .field("idealc_rosc", &self.idealc_rosc())
            .field("setcyc_rosc", &self.setcyc_rosc())
            .field("samcyc_rosc", &self.samcyc_rosc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircatc4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircatc4 {{ idealc_rosc: {=u16:?}, setcyc_rosc: {=u8:?}, samcyc_rosc: {=u8:?} }}",
            self.idealc_rosc(),
            self.setcyc_rosc(),
            self.samcyc_rosc()
        )
    }
}
#[doc = "FIRC Auto-trimming Counter 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircatc5(pub u32);
impl Fircatc5 {
    #[doc = "Coarse Trim Minimum Counter ROSC"]
    #[must_use]
    #[inline(always)]
    pub const fn coarminc_rosc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Coarse Trim Minimum Counter ROSC"]
    #[inline(always)]
    pub const fn set_coarminc_rosc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Coarse Trim Maximum Counter ROSC"]
    #[must_use]
    #[inline(always)]
    pub const fn coarmaxc_rosc(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Coarse Trim Maximum Counter ROSC"]
    #[inline(always)]
    pub const fn set_coarmaxc_rosc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Fircatc5 {
    #[inline(always)]
    fn default() -> Fircatc5 {
        Fircatc5(0)
    }
}
impl core::fmt::Debug for Fircatc5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircatc5")
            .field("coarminc_rosc", &self.coarminc_rosc())
            .field("coarmaxc_rosc", &self.coarmaxc_rosc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircatc5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircatc5 {{ coarminc_rosc: {=u16:?}, coarmaxc_rosc: {=u16:?} }}",
            self.coarminc_rosc(),
            self.coarmaxc_rosc()
        )
    }
}
#[doc = "FIRC Auto-trimming Counter 6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircatc6(pub u32);
impl Fircatc6 {
    #[doc = "Fine Trim Minimum Counter ROSC"]
    #[must_use]
    #[inline(always)]
    pub const fn fineminc_rosc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Fine Trim Minimum Counter ROSC"]
    #[inline(always)]
    pub const fn set_fineminc_rosc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Fine Trim Maximum Counter ROSC"]
    #[must_use]
    #[inline(always)]
    pub const fn finemaxc_rosc(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Fine Trim Maximum Counter ROSC"]
    #[inline(always)]
    pub const fn set_finemaxc_rosc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Fircatc6 {
    #[inline(always)]
    fn default() -> Fircatc6 {
        Fircatc6(0)
    }
}
impl core::fmt::Debug for Fircatc6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircatc6")
            .field("fineminc_rosc", &self.fineminc_rosc())
            .field("finemaxc_rosc", &self.finemaxc_rosc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircatc6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircatc6 {{ fineminc_rosc: {=u16:?}, finemaxc_rosc: {=u16:?} }}",
            self.fineminc_rosc(),
            self.finemaxc_rosc()
        )
    }
}
#[doc = "FIRC Auto-trimming Counter 7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircatc7(pub u32);
impl Fircatc7 {
    #[doc = "Ideal Counter SOF"]
    #[must_use]
    #[inline(always)]
    pub const fn idealc_sof(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Ideal Counter SOF"]
    #[inline(always)]
    pub const fn set_idealc_sof(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
    #[doc = "Setup Cycle SOF"]
    #[must_use]
    #[inline(always)]
    pub const fn setcyc_sof(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Setup Cycle SOF"]
    #[inline(always)]
    pub const fn set_setcyc_sof(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Sample Cycle SOF"]
    #[must_use]
    #[inline(always)]
    pub const fn samcyc_sof(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Sample Cycle SOF"]
    #[inline(always)]
    pub const fn set_samcyc_sof(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Fircatc7 {
    #[inline(always)]
    fn default() -> Fircatc7 {
        Fircatc7(0)
    }
}
impl core::fmt::Debug for Fircatc7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircatc7")
            .field("idealc_sof", &self.idealc_sof())
            .field("setcyc_sof", &self.setcyc_sof())
            .field("samcyc_sof", &self.samcyc_sof())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircatc7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircatc7 {{ idealc_sof: {=u32:?}, setcyc_sof: {=u8:?}, samcyc_sof: {=u8:?} }}",
            self.idealc_sof(),
            self.setcyc_sof(),
            self.samcyc_sof()
        )
    }
}
#[doc = "FIRC Auto-trimming Counter 8"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircatc8(pub u32);
impl Fircatc8 {
    #[doc = "Coarse Trim Maximum Counter SOF"]
    #[must_use]
    #[inline(always)]
    pub const fn coarmaxc_sof(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Coarse Trim Maximum Counter SOF"]
    #[inline(always)]
    pub const fn set_coarmaxc_sof(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Fircatc8 {
    #[inline(always)]
    fn default() -> Fircatc8 {
        Fircatc8(0)
    }
}
impl core::fmt::Debug for Fircatc8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircatc8")
            .field("coarmaxc_sof", &self.coarmaxc_sof())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircatc8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircatc8 {{ coarmaxc_sof: {=u32:?} }}",
            self.coarmaxc_sof()
        )
    }
}
#[doc = "FIRC Auto-trimming Counter 9"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircatc9(pub u32);
impl Fircatc9 {
    #[doc = "Coarse Trim Minimum Counter SOF"]
    #[must_use]
    #[inline(always)]
    pub const fn coarminc_sof(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Coarse Trim Minimum Counter SOF"]
    #[inline(always)]
    pub const fn set_coarminc_sof(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Fircatc9 {
    #[inline(always)]
    fn default() -> Fircatc9 {
        Fircatc9(0)
    }
}
impl core::fmt::Debug for Fircatc9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircatc9")
            .field("coarminc_sof", &self.coarminc_sof())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircatc9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircatc9 {{ coarminc_sof: {=u32:?} }}",
            self.coarminc_sof()
        )
    }
}
#[doc = "FIRC Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Firccfg(pub u32);
impl Firccfg {
    #[doc = "Frequency select"]
    #[must_use]
    #[inline(always)]
    pub const fn freq_sel(&self) -> super::vals::FreqSel {
        let val = (self.0 >> 1usize) & 0x07;
        super::vals::FreqSel::from_bits(val as u8)
    }
    #[doc = "Frequency select"]
    #[inline(always)]
    pub const fn set_freq_sel(&mut self, val: super::vals::FreqSel) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
    }
}
impl Default for Firccfg {
    #[inline(always)]
    fn default() -> Firccfg {
        Firccfg(0)
    }
}
impl core::fmt::Debug for Firccfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Firccfg")
            .field("freq_sel", &self.freq_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Firccfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Firccfg {{ freq_sel: {:?} }}", self.freq_sel())
    }
}
#[doc = "FIRC Control Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Firccsr(pub u32);
impl Firccsr {
    #[doc = "FIRC Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fircen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIRC Enable"]
    #[inline(always)]
    pub const fn set_fircen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIRC Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fircsten(&self) -> super::vals::Fircsten {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Fircsten::from_bits(val as u8)
    }
    #[doc = "FIRC Stop Enable"]
    #[inline(always)]
    pub const fn set_fircsten(&mut self, val: super::vals::Fircsten) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "FIRC 48 MHz Clock to peripherals Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn firc_sclk_periph_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "FIRC 48 MHz Clock to peripherals Enable"]
    #[inline(always)]
    pub const fn set_firc_sclk_periph_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "FRO_HF Clock to peripherals Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn firc_fclk_periph_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "FRO_HF Clock to peripherals Enable"]
    #[inline(always)]
    pub const fn set_firc_fclk_periph_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "FRO_HF Trim Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn firctren(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "FRO_HF Trim Enable"]
    #[inline(always)]
    pub const fn set_firctren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "FIRC Trim Update"]
    #[must_use]
    #[inline(always)]
    pub const fn firctrup(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "FIRC Trim Update"]
    #[inline(always)]
    pub const fn set_firctrup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "FIRC TRIM LOCK"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_lock(&self) -> super::vals::FirccsrTrimLock {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::FirccsrTrimLock::from_bits(val as u8)
    }
    #[doc = "FIRC TRIM LOCK"]
    #[inline(always)]
    pub const fn set_trim_lock(&mut self, val: super::vals::FirccsrTrimLock) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Coarse Auto Trim Bypass"]
    #[must_use]
    #[inline(always)]
    pub const fn coarse_trim_bypass(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Coarse Auto Trim Bypass"]
    #[inline(always)]
    pub const fn set_coarse_trim_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::FirccsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::FirccsrLk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::FirccsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "FIRC Valid status"]
    #[must_use]
    #[inline(always)]
    pub const fn fircvld(&self) -> super::vals::Fircvld {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Fircvld::from_bits(val as u8)
    }
    #[doc = "FIRC Valid status"]
    #[inline(always)]
    pub const fn set_fircvld(&mut self, val: super::vals::Fircvld) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "FIRC Selected"]
    #[must_use]
    #[inline(always)]
    pub const fn fircsel(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "FIRC Selected"]
    #[inline(always)]
    pub const fn set_fircsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "FIRC Clock Error"]
    #[must_use]
    #[inline(always)]
    pub const fn fircerr(&self) -> super::vals::Fircerr {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Fircerr::from_bits(val as u8)
    }
    #[doc = "FIRC Clock Error"]
    #[inline(always)]
    pub const fn set_fircerr(&mut self, val: super::vals::Fircerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "FIRC Clock Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fircerr_ie(&self) -> super::vals::FircerrIe {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::FircerrIe::from_bits(val as u8)
    }
    #[doc = "FIRC Clock Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fircerr_ie(&mut self, val: super::vals::FircerrIe) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "FIRC Accurate Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fircacc_ie(&self) -> super::vals::FircaccIe {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::FircaccIe::from_bits(val as u8)
    }
    #[doc = "FIRC Accurate Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fircacc_ie(&mut self, val: super::vals::FircaccIe) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "FIRC Frequency Accurate"]
    #[must_use]
    #[inline(always)]
    pub const fn fircacc(&self) -> super::vals::Fircacc {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Fircacc::from_bits(val as u8)
    }
    #[doc = "FIRC Frequency Accurate"]
    #[inline(always)]
    pub const fn set_fircacc(&mut self, val: super::vals::Fircacc) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Firccsr {
    #[inline(always)]
    fn default() -> Firccsr {
        Firccsr(0)
    }
}
impl core::fmt::Debug for Firccsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Firccsr")
            .field("fircen", &self.fircen())
            .field("fircsten", &self.fircsten())
            .field("firc_sclk_periph_en", &self.firc_sclk_periph_en())
            .field("firc_fclk_periph_en", &self.firc_fclk_periph_en())
            .field("firctren", &self.firctren())
            .field("firctrup", &self.firctrup())
            .field("trim_lock", &self.trim_lock())
            .field("coarse_trim_bypass", &self.coarse_trim_bypass())
            .field("lk", &self.lk())
            .field("fircvld", &self.fircvld())
            .field("fircsel", &self.fircsel())
            .field("fircerr", &self.fircerr())
            .field("fircerr_ie", &self.fircerr_ie())
            .field("fircacc_ie", &self.fircacc_ie())
            .field("fircacc", &self.fircacc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Firccsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Firccsr {{ fircen: {=bool:?}, fircsten: {:?}, firc_sclk_periph_en: {=bool:?}, firc_fclk_periph_en: {=bool:?}, firctren: {=bool:?}, firctrup: {=bool:?}, trim_lock: {:?}, coarse_trim_bypass: {=bool:?}, lk: {:?}, fircvld: {:?}, fircsel: {=bool:?}, fircerr: {:?}, fircerr_ie: {:?}, fircacc_ie: {:?}, fircacc: {:?} }}",
            self.fircen(),
            self.fircsten(),
            self.firc_sclk_periph_en(),
            self.firc_fclk_periph_en(),
            self.firctren(),
            self.firctrup(),
            self.trim_lock(),
            self.coarse_trim_bypass(),
            self.lk(),
            self.fircvld(),
            self.fircsel(),
            self.fircerr(),
            self.fircerr_ie(),
            self.fircacc_ie(),
            self.fircacc()
        )
    }
}
#[doc = "FIRC Auto-trimming Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircstat(pub u32);
impl Fircstat {
    #[doc = "Trim Fine"]
    #[must_use]
    #[inline(always)]
    pub const fn trimfine(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Trim Fine"]
    #[inline(always)]
    pub const fn set_trimfine(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Trim Coarse"]
    #[must_use]
    #[inline(always)]
    pub const fn trimcoar(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Trim Coarse"]
    #[inline(always)]
    pub const fn set_trimcoar(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for Fircstat {
    #[inline(always)]
    fn default() -> Fircstat {
        Fircstat(0)
    }
}
impl core::fmt::Debug for Fircstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircstat")
            .field("trimfine", &self.trimfine())
            .field("trimcoar", &self.trimcoar())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircstat {{ trimfine: {=u8:?}, trimcoar: {=u8:?} }}",
            self.trimfine(),
            self.trimcoar()
        )
    }
}
#[doc = "FIRC Trim Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Firctcfg(pub u32);
impl Firctcfg {
    #[doc = "Trim Source"]
    #[must_use]
    #[inline(always)]
    pub const fn trimsrc(&self) -> super::vals::FirctcfgTrimsrc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::FirctcfgTrimsrc::from_bits(val as u8)
    }
    #[doc = "Trim Source"]
    #[inline(always)]
    pub const fn set_trimsrc(&mut self, val: super::vals::FirctcfgTrimsrc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "FIRC Trim Pre-divider"]
    #[must_use]
    #[inline(always)]
    pub const fn trimdiv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "FIRC Trim Pre-divider"]
    #[inline(always)]
    pub const fn set_trimdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for Firctcfg {
    #[inline(always)]
    fn default() -> Firctcfg {
        Firctcfg(0)
    }
}
impl core::fmt::Debug for Firctcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Firctcfg")
            .field("trimsrc", &self.trimsrc())
            .field("trimdiv", &self.trimdiv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Firctcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Firctcfg {{ trimsrc: {:?}, trimdiv: {=u8:?} }}",
            self.trimsrc(),
            self.trimdiv()
        )
    }
}
#[doc = "FIRC Trim"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Firctrim(pub u32);
impl Firctrim {
    #[doc = "Trim Fine"]
    #[must_use]
    #[inline(always)]
    pub const fn trimfine(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Trim Fine"]
    #[inline(always)]
    pub const fn set_trimfine(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Trim Coarse"]
    #[must_use]
    #[inline(always)]
    pub const fn trimcoar(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Trim Coarse"]
    #[inline(always)]
    pub const fn set_trimcoar(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Trim Temperature"]
    #[must_use]
    #[inline(always)]
    pub const fn trimtemp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Trim Temperature"]
    #[inline(always)]
    pub const fn set_trimtemp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Trim Start"]
    #[must_use]
    #[inline(always)]
    pub const fn trimstart(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Trim Start"]
    #[inline(always)]
    pub const fn set_trimstart(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Firctrim {
    #[inline(always)]
    fn default() -> Firctrim {
        Firctrim(0)
    }
}
impl core::fmt::Debug for Firctrim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Firctrim")
            .field("trimfine", &self.trimfine())
            .field("trimcoar", &self.trimcoar())
            .field("trimtemp", &self.trimtemp())
            .field("trimstart", &self.trimstart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Firctrim {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Firctrim {{ trimfine: {=u8:?}, trimcoar: {=u8:?}, trimtemp: {=u8:?}, trimstart: {=u8:?} }}",
            self.trimfine(),
            self.trimcoar(),
            self.trimtemp(),
            self.trimstart()
        )
    }
}
#[doc = "LDO Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldocsr(pub u32);
impl Ldocsr {
    #[doc = "LDO Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ldoen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LDO Enable"]
    #[inline(always)]
    pub const fn set_ldoen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LDO output voltage select"]
    #[must_use]
    #[inline(always)]
    pub const fn vout_sel(&self) -> super::vals::VoutSel {
        let val = (self.0 >> 1usize) & 0x07;
        super::vals::VoutSel::from_bits(val as u8)
    }
    #[doc = "LDO output voltage select"]
    #[inline(always)]
    pub const fn set_vout_sel(&mut self, val: super::vals::VoutSel) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
    }
    #[doc = "LDO Bypass"]
    #[must_use]
    #[inline(always)]
    pub const fn ldobypass(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "LDO Bypass"]
    #[inline(always)]
    pub const fn set_ldobypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "LDO VOUT OK Inform."]
    #[must_use]
    #[inline(always)]
    pub const fn vout_ok(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LDO VOUT OK Inform."]
    #[inline(always)]
    pub const fn set_vout_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ldocsr {
    #[inline(always)]
    fn default() -> Ldocsr {
        Ldocsr(0)
    }
}
impl core::fmt::Debug for Ldocsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ldocsr")
            .field("ldoen", &self.ldoen())
            .field("vout_sel", &self.vout_sel())
            .field("ldobypass", &self.ldobypass())
            .field("vout_ok", &self.vout_ok())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ldocsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ldocsr {{ ldoen: {=bool:?}, vout_sel: {:?}, ldobypass: {=bool:?}, vout_ok: {=bool:?} }}",
            self.ldoen(),
            self.vout_sel(),
            self.ldobypass(),
            self.vout_ok()
        )
    }
}
#[doc = "Parameter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "SOSC Clock Present"]
    #[must_use]
    #[inline(always)]
    pub const fn soscclkpres(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Clock Present"]
    #[inline(always)]
    pub const fn set_soscclkpres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SIRC Clock Present"]
    #[must_use]
    #[inline(always)]
    pub const fn sircclkpres(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SIRC Clock Present"]
    #[inline(always)]
    pub const fn set_sircclkpres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "FIRC Clock Present"]
    #[must_use]
    #[inline(always)]
    pub const fn fircclkpres(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "FIRC Clock Present"]
    #[inline(always)]
    pub const fn set_fircclkpres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "ROSC Clock Present"]
    #[must_use]
    #[inline(always)]
    pub const fn roscclkpres(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "ROSC Clock Present"]
    #[inline(always)]
    pub const fn set_roscclkpres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "SPLL Clock Present"]
    #[must_use]
    #[inline(always)]
    pub const fn spllclkpres(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "SPLL Clock Present"]
    #[inline(always)]
    pub const fn set_spllclkpres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Param {
    #[inline(always)]
    fn default() -> Param {
        Param(0)
    }
}
impl core::fmt::Debug for Param {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Param")
            .field("soscclkpres", &self.soscclkpres())
            .field("sircclkpres", &self.sircclkpres())
            .field("fircclkpres", &self.fircclkpres())
            .field("roscclkpres", &self.roscclkpres())
            .field("spllclkpres", &self.spllclkpres())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Param {{ soscclkpres: {=bool:?}, sircclkpres: {=bool:?}, fircclkpres: {=bool:?}, roscclkpres: {=bool:?}, spllclkpres: {=bool:?} }}",
            self.soscclkpres(),
            self.sircclkpres(),
            self.fircclkpres(),
            self.roscclkpres(),
            self.spllclkpres()
        )
    }
}
#[doc = "Run Clock Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rccr(pub u32);
impl Rccr {
    #[doc = "System Clock Source"]
    #[must_use]
    #[inline(always)]
    pub const fn scs(&self) -> super::vals::Scs {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Scs::from_bits(val as u8)
    }
    #[doc = "System Clock Source"]
    #[inline(always)]
    pub const fn set_scs(&mut self, val: super::vals::Scs) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
}
impl Default for Rccr {
    #[inline(always)]
    fn default() -> Rccr {
        Rccr(0)
    }
}
impl core::fmt::Debug for Rccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rccr").field("scs", &self.scs()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rccr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rccr {{ scs: {:?} }}", self.scs())
    }
}
#[doc = "ROSC Control Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rosccsr(pub u32);
impl Rosccsr {
    #[doc = "ROSC Clock Monitor"]
    #[must_use]
    #[inline(always)]
    pub const fn rosccm(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ROSC Clock Monitor"]
    #[inline(always)]
    pub const fn set_rosccm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ROSC Clock Monitor Reset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rosccmre(&self) -> super::vals::Rosccmre {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Rosccmre::from_bits(val as u8)
    }
    #[doc = "ROSC Clock Monitor Reset Enable"]
    #[inline(always)]
    pub const fn set_rosccmre(&mut self, val: super::vals::Rosccmre) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::RosccsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::RosccsrLk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::RosccsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "ROSC Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn roscvld(&self) -> super::vals::Roscvld {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Roscvld::from_bits(val as u8)
    }
    #[doc = "ROSC Valid"]
    #[inline(always)]
    pub const fn set_roscvld(&mut self, val: super::vals::Roscvld) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "ROSC Selected"]
    #[must_use]
    #[inline(always)]
    pub const fn roscsel(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "ROSC Selected"]
    #[inline(always)]
    pub const fn set_roscsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "ROSC Clock Error"]
    #[must_use]
    #[inline(always)]
    pub const fn roscerr(&self) -> super::vals::Roscerr {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Roscerr::from_bits(val as u8)
    }
    #[doc = "ROSC Clock Error"]
    #[inline(always)]
    pub const fn set_roscerr(&mut self, val: super::vals::Roscerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for Rosccsr {
    #[inline(always)]
    fn default() -> Rosccsr {
        Rosccsr(0)
    }
}
impl core::fmt::Debug for Rosccsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rosccsr")
            .field("rosccm", &self.rosccm())
            .field("rosccmre", &self.rosccmre())
            .field("lk", &self.lk())
            .field("roscvld", &self.roscvld())
            .field("roscsel", &self.roscsel())
            .field("roscerr", &self.roscerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rosccsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rosccsr {{ rosccm: {=bool:?}, rosccmre: {:?}, lk: {:?}, roscvld: {:?}, roscsel: {=bool:?}, roscerr: {:?} }}",
            self.rosccm(),
            self.rosccmre(),
            self.lk(),
            self.roscvld(),
            self.roscsel(),
            self.roscerr()
        )
    }
}
#[doc = "SIRC Control Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sirccsr(pub u32);
impl Sirccsr {
    #[doc = "SIRC Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sircsten(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SIRC Stop Enable"]
    #[inline(always)]
    pub const fn set_sircsten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SIRC Clock to Peripherals Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sirc_clk_periph_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "SIRC Clock to Peripherals Enable"]
    #[inline(always)]
    pub const fn set_sirc_clk_periph_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "SIRC 12 MHz Trim Enable (SIRCCFG\\[RANGE\\]=1)"]
    #[must_use]
    #[inline(always)]
    pub const fn sirctren(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SIRC 12 MHz Trim Enable (SIRCCFG\\[RANGE\\]=1)"]
    #[inline(always)]
    pub const fn set_sirctren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SIRC Trim Update"]
    #[must_use]
    #[inline(always)]
    pub const fn sirctrup(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SIRC Trim Update"]
    #[inline(always)]
    pub const fn set_sirctrup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SIRC TRIM LOCK"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_lock(&self) -> super::vals::SirccsrTrimLock {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::SirccsrTrimLock::from_bits(val as u8)
    }
    #[doc = "SIRC TRIM LOCK"]
    #[inline(always)]
    pub const fn set_trim_lock(&mut self, val: super::vals::SirccsrTrimLock) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Coarse Auto Trim Bypass"]
    #[must_use]
    #[inline(always)]
    pub const fn coarse_trim_bypass(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Coarse Auto Trim Bypass"]
    #[inline(always)]
    pub const fn set_coarse_trim_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::SirccsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::SirccsrLk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::SirccsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "SIRC Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn sircvld(&self) -> super::vals::Sircvld {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Sircvld::from_bits(val as u8)
    }
    #[doc = "SIRC Valid"]
    #[inline(always)]
    pub const fn set_sircvld(&mut self, val: super::vals::Sircvld) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "SIRC Selected"]
    #[must_use]
    #[inline(always)]
    pub const fn sircsel(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SIRC Selected"]
    #[inline(always)]
    pub const fn set_sircsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "SIRC Clock Error"]
    #[must_use]
    #[inline(always)]
    pub const fn sircerr(&self) -> super::vals::Sircerr {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Sircerr::from_bits(val as u8)
    }
    #[doc = "SIRC Clock Error"]
    #[inline(always)]
    pub const fn set_sircerr(&mut self, val: super::vals::Sircerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "SIRC Clock Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sircerr_ie(&self) -> super::vals::SircerrIe {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::SircerrIe::from_bits(val as u8)
    }
    #[doc = "SIRC Clock Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_sircerr_ie(&mut self, val: super::vals::SircerrIe) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Sirccsr {
    #[inline(always)]
    fn default() -> Sirccsr {
        Sirccsr(0)
    }
}
impl core::fmt::Debug for Sirccsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sirccsr")
            .field("sircsten", &self.sircsten())
            .field("sirc_clk_periph_en", &self.sirc_clk_periph_en())
            .field("sirctren", &self.sirctren())
            .field("sirctrup", &self.sirctrup())
            .field("trim_lock", &self.trim_lock())
            .field("coarse_trim_bypass", &self.coarse_trim_bypass())
            .field("lk", &self.lk())
            .field("sircvld", &self.sircvld())
            .field("sircsel", &self.sircsel())
            .field("sircerr", &self.sircerr())
            .field("sircerr_ie", &self.sircerr_ie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sirccsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sirccsr {{ sircsten: {=bool:?}, sirc_clk_periph_en: {=bool:?}, sirctren: {=bool:?}, sirctrup: {=bool:?}, trim_lock: {:?}, coarse_trim_bypass: {=bool:?}, lk: {:?}, sircvld: {:?}, sircsel: {=bool:?}, sircerr: {:?}, sircerr_ie: {:?} }}",
            self.sircsten(),
            self.sirc_clk_periph_en(),
            self.sirctren(),
            self.sirctrup(),
            self.trim_lock(),
            self.coarse_trim_bypass(),
            self.lk(),
            self.sircvld(),
            self.sircsel(),
            self.sircerr(),
            self.sircerr_ie()
        )
    }
}
#[doc = "SIRC Auto-trimming Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sircstat(pub u32);
impl Sircstat {
    #[doc = "CCO Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn ccotrim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "CCO Trim"]
    #[inline(always)]
    pub const fn set_ccotrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "CL Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn cltrim(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "CL Trim"]
    #[inline(always)]
    pub const fn set_cltrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for Sircstat {
    #[inline(always)]
    fn default() -> Sircstat {
        Sircstat(0)
    }
}
impl core::fmt::Debug for Sircstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sircstat")
            .field("ccotrim", &self.ccotrim())
            .field("cltrim", &self.cltrim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sircstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sircstat {{ ccotrim: {=u8:?}, cltrim: {=u8:?} }}",
            self.ccotrim(),
            self.cltrim()
        )
    }
}
#[doc = "SIRC Trim Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sirctcfg(pub u32);
impl Sirctcfg {
    #[doc = "Trim Source"]
    #[must_use]
    #[inline(always)]
    pub const fn trimsrc(&self) -> super::vals::SirctcfgTrimsrc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SirctcfgTrimsrc::from_bits(val as u8)
    }
    #[doc = "Trim Source"]
    #[inline(always)]
    pub const fn set_trimsrc(&mut self, val: super::vals::SirctcfgTrimsrc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "SIRC Trim Pre-divider"]
    #[must_use]
    #[inline(always)]
    pub const fn trimdiv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "SIRC Trim Pre-divider"]
    #[inline(always)]
    pub const fn set_trimdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for Sirctcfg {
    #[inline(always)]
    fn default() -> Sirctcfg {
        Sirctcfg(0)
    }
}
impl core::fmt::Debug for Sirctcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sirctcfg")
            .field("trimsrc", &self.trimsrc())
            .field("trimdiv", &self.trimdiv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sirctcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sirctcfg {{ trimsrc: {:?}, trimdiv: {=u8:?} }}",
            self.trimsrc(),
            self.trimdiv()
        )
    }
}
#[doc = "SIRC Trim"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sirctrim(pub u32);
impl Sirctrim {
    #[doc = "CCO Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn ccotrim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "CCO Trim"]
    #[inline(always)]
    pub const fn set_ccotrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "CL Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn cltrim(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "CL Trim"]
    #[inline(always)]
    pub const fn set_cltrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Trim Temp"]
    #[must_use]
    #[inline(always)]
    pub const fn tctrim(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Trim Temp"]
    #[inline(always)]
    pub const fn set_tctrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Calibrates the replica voltage in FSU for CCO to get well frequency at initial period"]
    #[must_use]
    #[inline(always)]
    pub const fn fvchtrim(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Calibrates the replica voltage in FSU for CCO to get well frequency at initial period"]
    #[inline(always)]
    pub const fn set_fvchtrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for Sirctrim {
    #[inline(always)]
    fn default() -> Sirctrim {
        Sirctrim(0)
    }
}
impl core::fmt::Debug for Sirctrim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sirctrim")
            .field("ccotrim", &self.ccotrim())
            .field("cltrim", &self.cltrim())
            .field("tctrim", &self.tctrim())
            .field("fvchtrim", &self.fvchtrim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sirctrim {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sirctrim {{ ccotrim: {=u8:?}, cltrim: {=u8:?}, tctrim: {=u8:?}, fvchtrim: {=u8:?} }}",
            self.ccotrim(),
            self.cltrim(),
            self.tctrim(),
            self.fvchtrim()
        )
    }
}
#[doc = "SOSC Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sosccfg(pub u32);
impl Sosccfg {
    #[doc = "External Reference Select"]
    #[must_use]
    #[inline(always)]
    pub const fn erefs(&self) -> super::vals::Erefs {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Erefs::from_bits(val as u8)
    }
    #[doc = "External Reference Select"]
    #[inline(always)]
    pub const fn set_erefs(&mut self, val: super::vals::Erefs) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "SOSC Range Select"]
    #[must_use]
    #[inline(always)]
    pub const fn range(&self) -> super::vals::Range {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Range::from_bits(val as u8)
    }
    #[doc = "SOSC Range Select"]
    #[inline(always)]
    pub const fn set_range(&mut self, val: super::vals::Range) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for Sosccfg {
    #[inline(always)]
    fn default() -> Sosccfg {
        Sosccfg(0)
    }
}
impl core::fmt::Debug for Sosccfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sosccfg")
            .field("erefs", &self.erefs())
            .field("range", &self.range())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sosccfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sosccfg {{ erefs: {:?}, range: {:?} }}",
            self.erefs(),
            self.range()
        )
    }
}
#[doc = "SOSC Control Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sosccsr(pub u32);
impl Sosccsr {
    #[doc = "SOSC Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn soscen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Enable"]
    #[inline(always)]
    pub const fn set_soscen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SOSC Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn soscsten(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Stop Enable"]
    #[inline(always)]
    pub const fn set_soscsten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SOSC Clock Monitor Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sosccm(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Clock Monitor Enable"]
    #[inline(always)]
    pub const fn set_sosccm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SOSC Clock Monitor Reset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sosccmre(&self) -> super::vals::Sosccmre {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Sosccmre::from_bits(val as u8)
    }
    #[doc = "SOSC Clock Monitor Reset Enable"]
    #[inline(always)]
    pub const fn set_sosccmre(&mut self, val: super::vals::Sosccmre) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::SosccsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::SosccsrLk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::SosccsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "SOSC Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn soscvld(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Valid"]
    #[inline(always)]
    pub const fn set_soscvld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "SOSC Selected"]
    #[must_use]
    #[inline(always)]
    pub const fn soscsel(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Selected"]
    #[inline(always)]
    pub const fn set_soscsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "SOSC Clock Error"]
    #[must_use]
    #[inline(always)]
    pub const fn soscerr(&self) -> super::vals::Soscerr {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Soscerr::from_bits(val as u8)
    }
    #[doc = "SOSC Clock Error"]
    #[inline(always)]
    pub const fn set_soscerr(&mut self, val: super::vals::Soscerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "SOSC Valid Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn soscvld_ie(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Valid Interrupt Enable"]
    #[inline(always)]
    pub const fn set_soscvld_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "SOSC Clock Safety Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sosc_safe_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Clock Safety Enable"]
    #[inline(always)]
    pub const fn set_sosc_safe_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Sosccsr {
    #[inline(always)]
    fn default() -> Sosccsr {
        Sosccsr(0)
    }
}
impl core::fmt::Debug for Sosccsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sosccsr")
            .field("soscen", &self.soscen())
            .field("soscsten", &self.soscsten())
            .field("sosccm", &self.sosccm())
            .field("sosccmre", &self.sosccmre())
            .field("lk", &self.lk())
            .field("soscvld", &self.soscvld())
            .field("soscsel", &self.soscsel())
            .field("soscerr", &self.soscerr())
            .field("soscvld_ie", &self.soscvld_ie())
            .field("sosc_safe_en", &self.sosc_safe_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sosccsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sosccsr {{ soscen: {=bool:?}, soscsten: {=bool:?}, sosccm: {=bool:?}, sosccmre: {:?}, lk: {:?}, soscvld: {=bool:?}, soscsel: {=bool:?}, soscerr: {:?}, soscvld_ie: {=bool:?}, sosc_safe_en: {=bool:?} }}",
            self.soscen(),
            self.soscsten(),
            self.sosccm(),
            self.sosccmre(),
            self.lk(),
            self.soscvld(),
            self.soscsel(),
            self.soscerr(),
            self.soscvld_ie(),
            self.sosc_safe_en()
        )
    }
}
#[doc = "SPLL Control Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllcsr(pub u32);
impl Spllcsr {
    #[doc = "SPLL Power Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn spllpwren(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SPLL Power Enable"]
    #[inline(always)]
    pub const fn set_spllpwren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SPLL Clock Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn spllclken(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SPLL Clock Enable"]
    #[inline(always)]
    pub const fn set_spllclken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SPLL Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn spllsten(&self) -> super::vals::Spllsten {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Spllsten::from_bits(val as u8)
    }
    #[doc = "SPLL Stop Enable"]
    #[inline(always)]
    pub const fn set_spllsten(&mut self, val: super::vals::Spllsten) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Free running mode clock stable"]
    #[must_use]
    #[inline(always)]
    pub const fn frm_clockstable(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Free running mode clock stable"]
    #[inline(always)]
    pub const fn set_frm_clockstable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "SPLL Divide-by-2 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn spll_div2_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "SPLL Divide-by-2 Enable"]
    #[inline(always)]
    pub const fn set_spll_div2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "SPLL Clock Monitor"]
    #[must_use]
    #[inline(always)]
    pub const fn spllcm(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SPLL Clock Monitor"]
    #[inline(always)]
    pub const fn set_spllcm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SPLL Clock Monitor Reset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn spllcmre(&self) -> super::vals::Spllcmre {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Spllcmre::from_bits(val as u8)
    }
    #[doc = "SPLL Clock Monitor Reset Enable"]
    #[inline(always)]
    pub const fn set_spllcmre(&mut self, val: super::vals::Spllcmre) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::SpllcsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::SpllcsrLk::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::SpllcsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "SPLL LOCK"]
    #[must_use]
    #[inline(always)]
    pub const fn spll_lock(&self) -> super::vals::SpllLock {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::SpllLock::from_bits(val as u8)
    }
    #[doc = "SPLL LOCK"]
    #[inline(always)]
    pub const fn set_spll_lock(&mut self, val: super::vals::SpllLock) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "SPLL Selected"]
    #[must_use]
    #[inline(always)]
    pub const fn spllsel(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SPLL Selected"]
    #[inline(always)]
    pub const fn set_spllsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "SPLL Clock Error"]
    #[must_use]
    #[inline(always)]
    pub const fn spllerr(&self) -> super::vals::Spllerr {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Spllerr::from_bits(val as u8)
    }
    #[doc = "SPLL Clock Error"]
    #[inline(always)]
    pub const fn set_spllerr(&mut self, val: super::vals::Spllerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "SPLL LOCK Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn spll_lock_ie(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "SPLL LOCK Interrupt Enable"]
    #[inline(always)]
    pub const fn set_spll_lock_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Spllcsr {
    #[inline(always)]
    fn default() -> Spllcsr {
        Spllcsr(0)
    }
}
impl core::fmt::Debug for Spllcsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllcsr")
            .field("spllpwren", &self.spllpwren())
            .field("spllclken", &self.spllclken())
            .field("spllsten", &self.spllsten())
            .field("frm_clockstable", &self.frm_clockstable())
            .field("spll_div2_en", &self.spll_div2_en())
            .field("spllcm", &self.spllcm())
            .field("spllcmre", &self.spllcmre())
            .field("lk", &self.lk())
            .field("spll_lock", &self.spll_lock())
            .field("spllsel", &self.spllsel())
            .field("spllerr", &self.spllerr())
            .field("spll_lock_ie", &self.spll_lock_ie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllcsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllcsr {{ spllpwren: {=bool:?}, spllclken: {=bool:?}, spllsten: {:?}, frm_clockstable: {=bool:?}, spll_div2_en: {=bool:?}, spllcm: {=bool:?}, spllcmre: {:?}, lk: {:?}, spll_lock: {:?}, spllsel: {=bool:?}, spllerr: {:?}, spll_lock_ie: {=bool:?} }}",
            self.spllpwren(),
            self.spllclken(),
            self.spllsten(),
            self.frm_clockstable(),
            self.spll_div2_en(),
            self.spllcm(),
            self.spllcmre(),
            self.lk(),
            self.spll_lock(),
            self.spllsel(),
            self.spllerr(),
            self.spll_lock_ie()
        )
    }
}
#[doc = "SPLL Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllctrl(pub u32);
impl Spllctrl {
    #[doc = "Bandwidth select R (resistor) value."]
    #[must_use]
    #[inline(always)]
    pub const fn selr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Bandwidth select R (resistor) value."]
    #[inline(always)]
    pub const fn set_selr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Bandwidth select I (interation) value."]
    #[must_use]
    #[inline(always)]
    pub const fn seli(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x3f;
        val as u8
    }
    #[doc = "Bandwidth select I (interation) value."]
    #[inline(always)]
    pub const fn set_seli(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 4usize)) | (((val as u32) & 0x3f) << 4usize);
    }
    #[doc = "Bandwidth select P (proportional) value."]
    #[must_use]
    #[inline(always)]
    pub const fn selp(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "Bandwidth select P (proportional) value."]
    #[inline(always)]
    pub const fn set_selp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[doc = "Bypass of the divide-by-2 divider"]
    #[must_use]
    #[inline(always)]
    pub const fn bypasspostdiv2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass of the divide-by-2 divider"]
    #[inline(always)]
    pub const fn set_bypasspostdiv2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Up Limiter."]
    #[must_use]
    #[inline(always)]
    pub const fn limupoff(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Up Limiter."]
    #[inline(always)]
    pub const fn set_limupoff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Control of the bandwidth of the PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn banddirect(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Control of the bandwidth of the PLL."]
    #[inline(always)]
    pub const fn set_banddirect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Bypass of the pre-divider."]
    #[must_use]
    #[inline(always)]
    pub const fn bypassprediv(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass of the pre-divider."]
    #[inline(always)]
    pub const fn set_bypassprediv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Bypass of the post-divider."]
    #[must_use]
    #[inline(always)]
    pub const fn bypasspostdiv(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass of the post-divider."]
    #[inline(always)]
    pub const fn set_bypasspostdiv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Free Running Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn frm(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Free Running Mode Enable"]
    #[inline(always)]
    pub const fn set_frm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Clock Source"]
    #[must_use]
    #[inline(always)]
    pub const fn source(&self) -> super::vals::Source {
        let val = (self.0 >> 25usize) & 0x03;
        super::vals::Source::from_bits(val as u8)
    }
    #[doc = "Clock Source"]
    #[inline(always)]
    pub const fn set_source(&mut self, val: super::vals::Source) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
}
impl Default for Spllctrl {
    #[inline(always)]
    fn default() -> Spllctrl {
        Spllctrl(0)
    }
}
impl core::fmt::Debug for Spllctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllctrl")
            .field("selr", &self.selr())
            .field("seli", &self.seli())
            .field("selp", &self.selp())
            .field("bypasspostdiv2", &self.bypasspostdiv2())
            .field("limupoff", &self.limupoff())
            .field("banddirect", &self.banddirect())
            .field("bypassprediv", &self.bypassprediv())
            .field("bypasspostdiv", &self.bypasspostdiv())
            .field("frm", &self.frm())
            .field("source", &self.source())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllctrl {{ selr: {=u8:?}, seli: {=u8:?}, selp: {=u8:?}, bypasspostdiv2: {=bool:?}, limupoff: {=bool:?}, banddirect: {=bool:?}, bypassprediv: {=bool:?}, bypasspostdiv: {=bool:?}, frm: {=bool:?}, source: {:?} }}",
            self.selr(),
            self.seli(),
            self.selp(),
            self.bypasspostdiv2(),
            self.limupoff(),
            self.banddirect(),
            self.bypassprediv(),
            self.bypasspostdiv(),
            self.frm(),
            self.source()
        )
    }
}
#[doc = "SPLL LOCK Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SplllockCnfg(pub u32);
impl SplllockCnfg {
    #[doc = "Configures the number of reference clocks to count before SPLL is considered locked."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_time(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "Configures the number of reference clocks to count before SPLL is considered locked."]
    #[inline(always)]
    pub const fn set_lock_time(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
    }
}
impl Default for SplllockCnfg {
    #[inline(always)]
    fn default() -> SplllockCnfg {
        SplllockCnfg(0)
    }
}
impl core::fmt::Debug for SplllockCnfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SplllockCnfg")
            .field("lock_time", &self.lock_time())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SplllockCnfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SplllockCnfg {{ lock_time: {=u32:?} }}",
            self.lock_time()
        )
    }
}
#[doc = "SPLL M Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllmdiv(pub u32);
impl Spllmdiv {
    #[doc = "Feedback divider ratio (M-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn mdiv(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Feedback divider ratio (M-divider)."]
    #[inline(always)]
    pub const fn set_mdiv(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Feedback ratio change request."]
    #[must_use]
    #[inline(always)]
    pub const fn mreq(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Feedback ratio change request."]
    #[inline(always)]
    pub const fn set_mreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Spllmdiv {
    #[inline(always)]
    fn default() -> Spllmdiv {
        Spllmdiv(0)
    }
}
impl core::fmt::Debug for Spllmdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllmdiv")
            .field("mdiv", &self.mdiv())
            .field("mreq", &self.mreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllmdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllmdiv {{ mdiv: {=u16:?}, mreq: {=bool:?} }}",
            self.mdiv(),
            self.mreq()
        )
    }
}
#[doc = "SPLL N Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllndiv(pub u32);
impl Spllndiv {
    #[doc = "Pre-divider divider ratio (N-divider)."]
    #[must_use]
    #[inline(always)]
    pub const fn ndiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Pre-divider divider ratio (N-divider)."]
    #[inline(always)]
    pub const fn set_ndiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Pre-divider ratio change request."]
    #[must_use]
    #[inline(always)]
    pub const fn nreq(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Pre-divider ratio change request."]
    #[inline(always)]
    pub const fn set_nreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Spllndiv {
    #[inline(always)]
    fn default() -> Spllndiv {
        Spllndiv(0)
    }
}
impl core::fmt::Debug for Spllndiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllndiv")
            .field("ndiv", &self.ndiv())
            .field("nreq", &self.nreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllndiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllndiv {{ ndiv: {=u8:?}, nreq: {=bool:?} }}",
            self.ndiv(),
            self.nreq()
        )
    }
}
#[doc = "SPLL P Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllpdiv(pub u32);
impl Spllpdiv {
    #[doc = "Post-divider divider ratio (P-divider)"]
    #[must_use]
    #[inline(always)]
    pub const fn pdiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Post-divider divider ratio (P-divider)"]
    #[inline(always)]
    pub const fn set_pdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Post-divider ratio change request"]
    #[must_use]
    #[inline(always)]
    pub const fn preq(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Post-divider ratio change request"]
    #[inline(always)]
    pub const fn set_preq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Spllpdiv {
    #[inline(always)]
    fn default() -> Spllpdiv {
        Spllpdiv(0)
    }
}
impl core::fmt::Debug for Spllpdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllpdiv")
            .field("pdiv", &self.pdiv())
            .field("preq", &self.preq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllpdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllpdiv {{ pdiv: {=u8:?}, preq: {=bool:?} }}",
            self.pdiv(),
            self.preq()
        )
    }
}
#[doc = "SPLL Spread Spectrum Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllsscg0(pub u32);
impl Spllsscg0 {
    #[doc = "SS_MDIV\\[31:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn ss_mdiv_lsb(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SS_MDIV\\[31:0\\]"]
    #[inline(always)]
    pub const fn set_ss_mdiv_lsb(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Spllsscg0 {
    #[inline(always)]
    fn default() -> Spllsscg0 {
        Spllsscg0(0)
    }
}
impl core::fmt::Debug for Spllsscg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllsscg0")
            .field("ss_mdiv_lsb", &self.ss_mdiv_lsb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllsscg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllsscg0 {{ ss_mdiv_lsb: {=u32:?} }}",
            self.ss_mdiv_lsb()
        )
    }
}
#[doc = "SPLL Spread Spectrum Control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllsscg1(pub u32);
impl Spllsscg1 {
    #[doc = "SS_MDIV\\[32\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn ss_mdiv_msb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SS_MDIV\\[32\\]"]
    #[inline(always)]
    pub const fn set_ss_mdiv_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SS_MDIV\\[32:0\\] change request."]
    #[must_use]
    #[inline(always)]
    pub const fn ss_mdiv_req(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SS_MDIV\\[32:0\\] change request."]
    #[inline(always)]
    pub const fn set_ss_mdiv_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Modulation Frequency Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mf(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[doc = "Modulation Frequency Control"]
    #[inline(always)]
    pub const fn set_mf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
    }
    #[doc = "Modulation Depth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mr(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "Modulation Depth Control"]
    #[inline(always)]
    pub const fn set_mr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "Modulation Waveform Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mc(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Modulation Waveform Control"]
    #[inline(always)]
    pub const fn set_mc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Dither Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dither(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Dither Enable"]
    #[inline(always)]
    pub const fn set_dither(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SS_MDIV select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel_ss_mdiv(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SS_MDIV select."]
    #[inline(always)]
    pub const fn set_sel_ss_mdiv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "SSCG Power Down"]
    #[must_use]
    #[inline(always)]
    pub const fn ss_pd(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "SSCG Power Down"]
    #[inline(always)]
    pub const fn set_ss_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Spllsscg1 {
    #[inline(always)]
    fn default() -> Spllsscg1 {
        Spllsscg1(0)
    }
}
impl core::fmt::Debug for Spllsscg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllsscg1")
            .field("ss_mdiv_msb", &self.ss_mdiv_msb())
            .field("ss_mdiv_req", &self.ss_mdiv_req())
            .field("mf", &self.mf())
            .field("mr", &self.mr())
            .field("mc", &self.mc())
            .field("dither", &self.dither())
            .field("sel_ss_mdiv", &self.sel_ss_mdiv())
            .field("ss_pd", &self.ss_pd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllsscg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllsscg1 {{ ss_mdiv_msb: {=bool:?}, ss_mdiv_req: {=bool:?}, mf: {=u8:?}, mr: {=u8:?}, mc: {=u8:?}, dither: {=bool:?}, sel_ss_mdiv: {=bool:?}, ss_pd: {=bool:?} }}",
            self.ss_mdiv_msb(),
            self.ss_mdiv_req(),
            self.mf(),
            self.mr(),
            self.mc(),
            self.dither(),
            self.sel_ss_mdiv(),
            self.ss_pd()
        )
    }
}
#[doc = "SPLL SSCG Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllsscgstat(pub u32);
impl Spllsscgstat {
    #[doc = "SS_MDIV change acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn ss_mdiv_ack(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SS_MDIV change acknowledge"]
    #[inline(always)]
    pub const fn set_ss_mdiv_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Spllsscgstat {
    #[inline(always)]
    fn default() -> Spllsscgstat {
        Spllsscgstat(0)
    }
}
impl core::fmt::Debug for Spllsscgstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllsscgstat")
            .field("ss_mdiv_ack", &self.ss_mdiv_ack())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllsscgstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllsscgstat {{ ss_mdiv_ack: {=bool:?} }}",
            self.ss_mdiv_ack()
        )
    }
}
#[doc = "SPLL Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spllstat(pub u32);
impl Spllstat {
    #[doc = "Pre-divider (N) ratio change acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn ndivack(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pre-divider (N) ratio change acknowledge"]
    #[inline(always)]
    pub const fn set_ndivack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Feedback (M) divider ratio change acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn mdivack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Feedback (M) divider ratio change acknowledge"]
    #[inline(always)]
    pub const fn set_mdivack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Post-divider (P) ratio change acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn pdivack(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Post-divider (P) ratio change acknowledge"]
    #[inline(always)]
    pub const fn set_pdivack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Free running detector (active high)"]
    #[must_use]
    #[inline(always)]
    pub const fn frmdet(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Free running detector (active high)"]
    #[inline(always)]
    pub const fn set_frmdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Spllstat {
    #[inline(always)]
    fn default() -> Spllstat {
        Spllstat(0)
    }
}
impl core::fmt::Debug for Spllstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spllstat")
            .field("ndivack", &self.ndivack())
            .field("mdivack", &self.mdivack())
            .field("pdivack", &self.pdivack())
            .field("frmdet", &self.frmdet())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spllstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Spllstat {{ ndivack: {=bool:?}, mdivack: {=bool:?}, pdivack: {=bool:?}, frmdet: {=bool:?} }}",
            self.ndivack(),
            self.mdivack(),
            self.pdivack(),
            self.frmdet()
        )
    }
}
#[doc = "Trim Lock"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrimLock(pub u32);
impl TrimLock {
    #[doc = "TRIM_UNLOCK"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_unlock(&self) -> super::vals::TrimUnlock {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::TrimUnlock::from_bits(val as u8)
    }
    #[doc = "TRIM_UNLOCK"]
    #[inline(always)]
    pub const fn set_trim_unlock(&mut self, val: super::vals::TrimUnlock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "IFR_DISABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn ifr_disable(&self) -> super::vals::IfrDisable {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::IfrDisable::from_bits(val as u8)
    }
    #[doc = "IFR_DISABLE"]
    #[inline(always)]
    pub const fn set_ifr_disable(&mut self, val: super::vals::IfrDisable) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "TRIM_LOCK_KEY"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_lock_key(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIM_LOCK_KEY"]
    #[inline(always)]
    pub const fn set_trim_lock_key(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for TrimLock {
    #[inline(always)]
    fn default() -> TrimLock {
        TrimLock(0)
    }
}
impl core::fmt::Debug for TrimLock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrimLock")
            .field("trim_unlock", &self.trim_unlock())
            .field("ifr_disable", &self.ifr_disable())
            .field("trim_lock_key", &self.trim_lock_key())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrimLock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TrimLock {{ trim_unlock: {:?}, ifr_disable: {:?}, trim_lock_key: {=u16:?} }}",
            self.trim_unlock(),
            self.ifr_disable(),
            self.trim_lock_key()
        )
    }
}
#[doc = "Version ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "SCG Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn version(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SCG Version Number"]
    #[inline(always)]
    pub const fn set_version(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
            .field("version", &self.version())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Verid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Verid {{ version: {=u32:?} }}", self.version())
    }
}
