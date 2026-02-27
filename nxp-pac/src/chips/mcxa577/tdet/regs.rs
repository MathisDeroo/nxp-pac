#[doc = "Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn swr(&self) -> super::vals::Swr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Swr::from_bits(val as u8)
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_swr(&mut self, val: super::vals::Swr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Digital Tamper Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn den(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Digital Tamper Enable"]
    #[inline(always)]
    pub const fn set_den(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Tamper Force System Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn tfsr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Force System Reset"]
    #[inline(always)]
    pub const fn set_tfsr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Update Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn um(&self) -> super::vals::Um {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Um::from_bits(val as u8)
    }
    #[doc = "Update Mode"]
    #[inline(always)]
    pub const fn set_um(&mut self, val: super::vals::Um) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Disable Prescaler On Tamper"]
    #[must_use]
    #[inline(always)]
    pub const fn distam(&self) -> super::vals::Distam {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Distam::from_bits(val as u8)
    }
    #[doc = "Disable Prescaler On Tamper"]
    #[inline(always)]
    pub const fn set_distam(&mut self, val: super::vals::Distam) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Digital Tamper Prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn dpr(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x7fff;
        val as u16
    }
    #[doc = "Digital Tamper Prescaler"]
    #[inline(always)]
    pub const fn set_dpr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
impl core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr")
            .field("swr", &self.swr())
            .field("den", &self.den())
            .field("tfsr", &self.tfsr())
            .field("um", &self.um())
            .field("distam", &self.distam())
            .field("dpr", &self.dpr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr {{ swr: {:?}, den: {=bool:?}, tfsr: {=bool:?}, um: {:?}, distam: {:?}, dpr: {=u16:?} }}",
            self.swr(),
            self.den(),
            self.tfsr(),
            self.um(),
            self.distam(),
            self.dpr()
        )
    }
}
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc = "Digital Tamper Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dtie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Digital Tamper Interrupt Enable"]
    #[inline(always)]
    pub const fn set_dtie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Tamper Input n Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tiie0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tiie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Tamper Input n Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tiie1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tiie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Tamper Input n Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tiie2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tiie2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Tamper Input n Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tiie3(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tiie3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Tamper Input n Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tiie4(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tiie4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Tamper Input n Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tiie5(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tiie5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Tamper Input n Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tiie6(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tiie6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Tamper Input n Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tiie7(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tiie7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Tamper Input n Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tiie8(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tiie8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Tamper Input n Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tiie9(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tiie9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Tamper Input n Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tiie10(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tiie10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Tamper Pin n Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tpie0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tpie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Tamper Pin n Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tpie1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tpie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Tamper Pin n Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tpie2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tpie2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Tamper Pin n Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tpie3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tpie3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Tamper Pin n Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tpie4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tpie4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Tamper Pin n Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tpie5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tpie5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Tamper Pin n Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tpie6(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tpie6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Tamper Pin n Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tpie7(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tpie7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Ier {
    #[inline(always)]
    fn default() -> Ier {
        Ier(0)
    }
}
impl core::fmt::Debug for Ier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ier")
            .field("dtie", &self.dtie())
            .field("tiie0", &self.tiie0())
            .field("tiie1", &self.tiie1())
            .field("tiie2", &self.tiie2())
            .field("tiie3", &self.tiie3())
            .field("tiie4", &self.tiie4())
            .field("tiie5", &self.tiie5())
            .field("tiie6", &self.tiie6())
            .field("tiie7", &self.tiie7())
            .field("tiie8", &self.tiie8())
            .field("tiie9", &self.tiie9())
            .field("tiie10", &self.tiie10())
            .field("tpie0", &self.tpie0())
            .field("tpie1", &self.tpie1())
            .field("tpie2", &self.tpie2())
            .field("tpie3", &self.tpie3())
            .field("tpie4", &self.tpie4())
            .field("tpie5", &self.tpie5())
            .field("tpie6", &self.tpie6())
            .field("tpie7", &self.tpie7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ier {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ier {{ dtie: {=bool:?}, tiie0: {=bool:?}, tiie1: {=bool:?}, tiie2: {=bool:?}, tiie3: {=bool:?}, tiie4: {=bool:?}, tiie5: {=bool:?}, tiie6: {=bool:?}, tiie7: {=bool:?}, tiie8: {=bool:?}, tiie9: {=bool:?}, tiie10: {=bool:?}, tpie0: {=bool:?}, tpie1: {=bool:?}, tpie2: {=bool:?}, tpie3: {=bool:?}, tpie4: {=bool:?}, tpie5: {=bool:?}, tpie6: {=bool:?}, tpie7: {=bool:?} }}",
            self.dtie(),
            self.tiie0(),
            self.tiie1(),
            self.tiie2(),
            self.tiie3(),
            self.tiie4(),
            self.tiie5(),
            self.tiie6(),
            self.tiie7(),
            self.tiie8(),
            self.tiie9(),
            self.tiie10(),
            self.tpie0(),
            self.tpie1(),
            self.tpie2(),
            self.tpie3(),
            self.tpie4(),
            self.tpie5(),
            self.tpie6(),
            self.tpie7()
        )
    }
}
#[doc = "Lock"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lr(pub u32);
impl Lr {
    #[doc = "Control Register Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn crl(&self) -> super::vals::Crl {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Crl::from_bits(val as u8)
    }
    #[doc = "Control Register Lock"]
    #[inline(always)]
    pub const fn set_crl(&mut self, val: super::vals::Crl) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Status Register Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn srl(&self) -> super::vals::Srl {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Srl::from_bits(val as u8)
    }
    #[doc = "Status Register Lock"]
    #[inline(always)]
    pub const fn set_srl(&mut self, val: super::vals::Srl) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Lock Register Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lrl(&self) -> super::vals::Lrl {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Lrl::from_bits(val as u8)
    }
    #[doc = "Lock Register Lock"]
    #[inline(always)]
    pub const fn set_lrl(&mut self, val: super::vals::Lrl) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt Enable Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn iel(&self) -> super::vals::Iel {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Iel::from_bits(val as u8)
    }
    #[doc = "Interrupt Enable Lock"]
    #[inline(always)]
    pub const fn set_iel(&mut self, val: super::vals::Iel) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Tamper Seconds Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn tsl(&self) -> super::vals::Tsl {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Tsl::from_bits(val as u8)
    }
    #[doc = "Tamper Seconds Lock"]
    #[inline(always)]
    pub const fn set_tsl(&mut self, val: super::vals::Tsl) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Tamper Enable Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn tel(&self) -> super::vals::Tel {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Tel::from_bits(val as u8)
    }
    #[doc = "Tamper Enable Lock"]
    #[inline(always)]
    pub const fn set_tel(&mut self, val: super::vals::Tel) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Pin Polarity Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn ppl(&self) -> super::vals::Ppl {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Ppl::from_bits(val as u8)
    }
    #[doc = "Pin Polarity Lock"]
    #[inline(always)]
    pub const fn set_ppl(&mut self, val: super::vals::Ppl) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Glitch Filter Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn gfl0(&self) -> super::vals::Gfl0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Gfl0::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Lock"]
    #[inline(always)]
    pub const fn set_gfl0(&mut self, val: super::vals::Gfl0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Glitch Filter Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn gfl1(&self) -> super::vals::Gfl1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Gfl1::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Lock"]
    #[inline(always)]
    pub const fn set_gfl1(&mut self, val: super::vals::Gfl1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Glitch Filter Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn gfl2(&self) -> super::vals::Gfl2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Gfl2::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Lock"]
    #[inline(always)]
    pub const fn set_gfl2(&mut self, val: super::vals::Gfl2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Glitch Filter Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn gfl3(&self) -> super::vals::Gfl3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Gfl3::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Lock"]
    #[inline(always)]
    pub const fn set_gfl3(&mut self, val: super::vals::Gfl3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Glitch Filter Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn gfl4(&self) -> super::vals::Gfl4 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Gfl4::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Lock"]
    #[inline(always)]
    pub const fn set_gfl4(&mut self, val: super::vals::Gfl4) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Glitch Filter Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn gfl5(&self) -> super::vals::Gfl5 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Gfl5::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Lock"]
    #[inline(always)]
    pub const fn set_gfl5(&mut self, val: super::vals::Gfl5) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Glitch Filter Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn gfl6(&self) -> super::vals::Gfl6 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Gfl6::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Lock"]
    #[inline(always)]
    pub const fn set_gfl6(&mut self, val: super::vals::Gfl6) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Glitch Filter Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn gfl7(&self) -> super::vals::Gfl7 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Gfl7::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Lock"]
    #[inline(always)]
    pub const fn set_gfl7(&mut self, val: super::vals::Gfl7) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for Lr {
    #[inline(always)]
    fn default() -> Lr {
        Lr(0)
    }
}
impl core::fmt::Debug for Lr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lr")
            .field("crl", &self.crl())
            .field("srl", &self.srl())
            .field("lrl", &self.lrl())
            .field("iel", &self.iel())
            .field("tsl", &self.tsl())
            .field("tel", &self.tel())
            .field("ppl", &self.ppl())
            .field("gfl0", &self.gfl0())
            .field("gfl1", &self.gfl1())
            .field("gfl2", &self.gfl2())
            .field("gfl3", &self.gfl3())
            .field("gfl4", &self.gfl4())
            .field("gfl5", &self.gfl5())
            .field("gfl6", &self.gfl6())
            .field("gfl7", &self.gfl7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lr {{ crl: {:?}, srl: {:?}, lrl: {:?}, iel: {:?}, tsl: {:?}, tel: {:?}, ppl: {:?}, gfl0: {:?}, gfl1: {:?}, gfl2: {:?}, gfl3: {:?}, gfl4: {:?}, gfl5: {:?}, gfl6: {:?}, gfl7: {:?} }}",
            self.crl(),
            self.srl(),
            self.lrl(),
            self.iel(),
            self.tsl(),
            self.tel(),
            self.ppl(),
            self.gfl0(),
            self.gfl1(),
            self.gfl2(),
            self.gfl3(),
            self.gfl4(),
            self.gfl5(),
            self.gfl6(),
            self.gfl7()
        )
    }
}
#[doc = "Pin Glitch Filter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pgfr(pub u32);
impl Pgfr {
    #[doc = "Glitch Filter Width"]
    #[must_use]
    #[inline(always)]
    pub const fn gfw(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Glitch Filter Width"]
    #[inline(always)]
    pub const fn set_gfw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Glitch Filter Prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn gfp(&self) -> super::vals::Gfp {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Gfp::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Prescaler"]
    #[inline(always)]
    pub const fn set_gfp(&mut self, val: super::vals::Gfp) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Glitch Filter Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gfe(&self) -> super::vals::Gfe {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Gfe::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Enable"]
    #[inline(always)]
    pub const fn set_gfe(&mut self, val: super::vals::Gfe) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Tamper Pin Sample Width"]
    #[must_use]
    #[inline(always)]
    pub const fn tpsw(&self) -> super::vals::Tpsw {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Tpsw::from_bits(val as u8)
    }
    #[doc = "Tamper Pin Sample Width"]
    #[inline(always)]
    pub const fn set_tpsw(&mut self, val: super::vals::Tpsw) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Tamper Pin Sample Frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn tpsf(&self) -> super::vals::Tpsf {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Tpsf::from_bits(val as u8)
    }
    #[doc = "Tamper Pin Sample Frequency"]
    #[inline(always)]
    pub const fn set_tpsf(&mut self, val: super::vals::Tpsf) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Tamper Pull Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tpe(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pull Enable"]
    #[inline(always)]
    pub const fn set_tpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Tamper Pull Select"]
    #[must_use]
    #[inline(always)]
    pub const fn tps(&self) -> super::vals::Tps {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Tps::from_bits(val as u8)
    }
    #[doc = "Tamper Pull Select"]
    #[inline(always)]
    pub const fn set_tps(&mut self, val: super::vals::Tps) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for Pgfr {
    #[inline(always)]
    fn default() -> Pgfr {
        Pgfr(0)
    }
}
impl core::fmt::Debug for Pgfr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pgfr")
            .field("gfw", &self.gfw())
            .field("gfp", &self.gfp())
            .field("gfe", &self.gfe())
            .field("tpsw", &self.tpsw())
            .field("tpsf", &self.tpsf())
            .field("tpe", &self.tpe())
            .field("tps", &self.tps())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pgfr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pgfr {{ gfw: {=u8:?}, gfp: {:?}, gfe: {:?}, tpsw: {:?}, tpsf: {:?}, tpe: {=bool:?}, tps: {:?} }}",
            self.gfw(),
            self.gfp(),
            self.gfe(),
            self.tpsw(),
            self.tpsf(),
            self.tpe(),
            self.tps()
        )
    }
}
#[doc = "Pin Polarity"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppr(pub u32);
impl Ppr {
    #[doc = "Tamper Pin n Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn tpp0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Polarity"]
    #[inline(always)]
    pub const fn set_tpp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Tamper Pin n Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn tpp1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Polarity"]
    #[inline(always)]
    pub const fn set_tpp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Tamper Pin n Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn tpp2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Polarity"]
    #[inline(always)]
    pub const fn set_tpp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Tamper Pin n Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn tpp3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Polarity"]
    #[inline(always)]
    pub const fn set_tpp3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Tamper Pin n Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn tpp4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Polarity"]
    #[inline(always)]
    pub const fn set_tpp4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Tamper Pin n Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn tpp5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Polarity"]
    #[inline(always)]
    pub const fn set_tpp5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Tamper Pin n Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn tpp6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Polarity"]
    #[inline(always)]
    pub const fn set_tpp6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Tamper Pin n Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn tpp7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Polarity"]
    #[inline(always)]
    pub const fn set_tpp7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Ppr {
    #[inline(always)]
    fn default() -> Ppr {
        Ppr(0)
    }
}
impl core::fmt::Debug for Ppr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ppr")
            .field("tpp0", &self.tpp0())
            .field("tpp1", &self.tpp1())
            .field("tpp2", &self.tpp2())
            .field("tpp3", &self.tpp3())
            .field("tpp4", &self.tpp4())
            .field("tpp5", &self.tpp5())
            .field("tpp6", &self.tpp6())
            .field("tpp7", &self.tpp7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ppr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ppr {{ tpp0: {=bool:?}, tpp1: {=bool:?}, tpp2: {=bool:?}, tpp3: {=bool:?}, tpp4: {=bool:?}, tpp5: {=bool:?}, tpp6: {=bool:?}, tpp7: {=bool:?} }}",
            self.tpp0(),
            self.tpp1(),
            self.tpp2(),
            self.tpp3(),
            self.tpp4(),
            self.tpp5(),
            self.tpp6(),
            self.tpp7()
        )
    }
}
#[doc = "Read Access Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rac(pub u32);
impl Rac {
    #[doc = "Control Register Read"]
    #[must_use]
    #[inline(always)]
    pub const fn crr(&self) -> super::vals::Crr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Crr::from_bits(val as u8)
    }
    #[doc = "Control Register Read"]
    #[inline(always)]
    pub const fn set_crr(&mut self, val: super::vals::Crr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Status Register Read"]
    #[must_use]
    #[inline(always)]
    pub const fn srr(&self) -> super::vals::Srr {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Srr::from_bits(val as u8)
    }
    #[doc = "Status Register Read"]
    #[inline(always)]
    pub const fn set_srr(&mut self, val: super::vals::Srr) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Lock Register Read"]
    #[must_use]
    #[inline(always)]
    pub const fn lrr(&self) -> super::vals::Lrr {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Lrr::from_bits(val as u8)
    }
    #[doc = "Lock Register Read"]
    #[inline(always)]
    pub const fn set_lrr(&mut self, val: super::vals::Lrr) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt Enable Read"]
    #[must_use]
    #[inline(always)]
    pub const fn ier(&self) -> super::vals::Ier {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Ier::from_bits(val as u8)
    }
    #[doc = "Interrupt Enable Read"]
    #[inline(always)]
    pub const fn set_ier(&mut self, val: super::vals::Ier) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Tamper Seconds Register Read"]
    #[must_use]
    #[inline(always)]
    pub const fn tsrr(&self) -> super::vals::Tsrr {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Tsrr::from_bits(val as u8)
    }
    #[doc = "Tamper Seconds Register Read"]
    #[inline(always)]
    pub const fn set_tsrr(&mut self, val: super::vals::Tsrr) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Tamper Enable Read"]
    #[must_use]
    #[inline(always)]
    pub const fn ter(&self) -> super::vals::Ter {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Ter::from_bits(val as u8)
    }
    #[doc = "Tamper Enable Read"]
    #[inline(always)]
    pub const fn set_ter(&mut self, val: super::vals::Ter) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Pin Polarity Read"]
    #[must_use]
    #[inline(always)]
    pub const fn ppr(&self) -> super::vals::Ppr {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Ppr::from_bits(val as u8)
    }
    #[doc = "Pin Polarity Read"]
    #[inline(always)]
    pub const fn set_ppr(&mut self, val: super::vals::Ppr) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Glitch Filter Read"]
    #[must_use]
    #[inline(always)]
    pub const fn gfr0(&self) -> super::vals::Gfr0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Gfr0::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Read"]
    #[inline(always)]
    pub const fn set_gfr0(&mut self, val: super::vals::Gfr0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Glitch Filter Read"]
    #[must_use]
    #[inline(always)]
    pub const fn gfr1(&self) -> super::vals::Gfr1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Gfr1::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Read"]
    #[inline(always)]
    pub const fn set_gfr1(&mut self, val: super::vals::Gfr1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Glitch Filter Read"]
    #[must_use]
    #[inline(always)]
    pub const fn gfr2(&self) -> super::vals::Gfr2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Gfr2::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Read"]
    #[inline(always)]
    pub const fn set_gfr2(&mut self, val: super::vals::Gfr2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Glitch Filter Read"]
    #[must_use]
    #[inline(always)]
    pub const fn gfr3(&self) -> super::vals::Gfr3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Gfr3::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Read"]
    #[inline(always)]
    pub const fn set_gfr3(&mut self, val: super::vals::Gfr3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Glitch Filter Read"]
    #[must_use]
    #[inline(always)]
    pub const fn gfr4(&self) -> super::vals::Gfr4 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Gfr4::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Read"]
    #[inline(always)]
    pub const fn set_gfr4(&mut self, val: super::vals::Gfr4) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Glitch Filter Read"]
    #[must_use]
    #[inline(always)]
    pub const fn gfr5(&self) -> super::vals::Gfr5 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Gfr5::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Read"]
    #[inline(always)]
    pub const fn set_gfr5(&mut self, val: super::vals::Gfr5) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Glitch Filter Read"]
    #[must_use]
    #[inline(always)]
    pub const fn gfr6(&self) -> super::vals::Gfr6 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Gfr6::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Read"]
    #[inline(always)]
    pub const fn set_gfr6(&mut self, val: super::vals::Gfr6) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Glitch Filter Read"]
    #[must_use]
    #[inline(always)]
    pub const fn gfr7(&self) -> super::vals::Gfr7 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Gfr7::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Read"]
    #[inline(always)]
    pub const fn set_gfr7(&mut self, val: super::vals::Gfr7) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for Rac {
    #[inline(always)]
    fn default() -> Rac {
        Rac(0)
    }
}
impl core::fmt::Debug for Rac {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rac")
            .field("crr", &self.crr())
            .field("srr", &self.srr())
            .field("lrr", &self.lrr())
            .field("ier", &self.ier())
            .field("tsrr", &self.tsrr())
            .field("ter", &self.ter())
            .field("ppr", &self.ppr())
            .field("gfr0", &self.gfr0())
            .field("gfr1", &self.gfr1())
            .field("gfr2", &self.gfr2())
            .field("gfr3", &self.gfr3())
            .field("gfr4", &self.gfr4())
            .field("gfr5", &self.gfr5())
            .field("gfr6", &self.gfr6())
            .field("gfr7", &self.gfr7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rac {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rac {{ crr: {:?}, srr: {:?}, lrr: {:?}, ier: {:?}, tsrr: {:?}, ter: {:?}, ppr: {:?}, gfr0: {:?}, gfr1: {:?}, gfr2: {:?}, gfr3: {:?}, gfr4: {:?}, gfr5: {:?}, gfr6: {:?}, gfr7: {:?} }}",
            self.crr(),
            self.srr(),
            self.lrr(),
            self.ier(),
            self.tsrr(),
            self.ter(),
            self.ppr(),
            self.gfr0(),
            self.gfr1(),
            self.gfr2(),
            self.gfr3(),
            self.gfr4(),
            self.gfr5(),
            self.gfr6(),
            self.gfr7()
        )
    }
}
#[doc = "Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Digital Tamper Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn dtf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Digital Tamper Flag"]
    #[inline(always)]
    pub const fn set_dtf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Tamper Acknowledge Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn taf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Acknowledge Flag"]
    #[inline(always)]
    pub const fn set_taf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Tamper Input n Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tif0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Flag"]
    #[inline(always)]
    pub const fn set_tif0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Tamper Input n Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tif1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Flag"]
    #[inline(always)]
    pub const fn set_tif1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Tamper Input n Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tif2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Flag"]
    #[inline(always)]
    pub const fn set_tif2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Tamper Input n Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tif3(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Flag"]
    #[inline(always)]
    pub const fn set_tif3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Tamper Input n Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tif4(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Flag"]
    #[inline(always)]
    pub const fn set_tif4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Tamper Input n Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tif5(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Flag"]
    #[inline(always)]
    pub const fn set_tif5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Tamper Input n Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tif6(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Flag"]
    #[inline(always)]
    pub const fn set_tif6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Tamper Input n Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tif7(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Flag"]
    #[inline(always)]
    pub const fn set_tif7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Tamper Input n Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tif8(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Flag"]
    #[inline(always)]
    pub const fn set_tif8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Tamper Input n Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tif9(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Flag"]
    #[inline(always)]
    pub const fn set_tif9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Tamper Input n Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tif10(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input n Flag"]
    #[inline(always)]
    pub const fn set_tif10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Tamper Pin n Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tpf0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Flag"]
    #[inline(always)]
    pub const fn set_tpf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Tamper Pin n Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tpf1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Flag"]
    #[inline(always)]
    pub const fn set_tpf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Tamper Pin n Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tpf2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Flag"]
    #[inline(always)]
    pub const fn set_tpf2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Tamper Pin n Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tpf3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Flag"]
    #[inline(always)]
    pub const fn set_tpf3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Tamper Pin n Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tpf4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Flag"]
    #[inline(always)]
    pub const fn set_tpf4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Tamper Pin n Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tpf5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Flag"]
    #[inline(always)]
    pub const fn set_tpf5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Tamper Pin n Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tpf6(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Flag"]
    #[inline(always)]
    pub const fn set_tpf6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Tamper Pin n Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tpf7(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin n Flag"]
    #[inline(always)]
    pub const fn set_tpf7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
impl core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sr")
            .field("dtf", &self.dtf())
            .field("taf", &self.taf())
            .field("tif0", &self.tif0())
            .field("tif1", &self.tif1())
            .field("tif2", &self.tif2())
            .field("tif3", &self.tif3())
            .field("tif4", &self.tif4())
            .field("tif5", &self.tif5())
            .field("tif6", &self.tif6())
            .field("tif7", &self.tif7())
            .field("tif8", &self.tif8())
            .field("tif9", &self.tif9())
            .field("tif10", &self.tif10())
            .field("tpf0", &self.tpf0())
            .field("tpf1", &self.tpf1())
            .field("tpf2", &self.tpf2())
            .field("tpf3", &self.tpf3())
            .field("tpf4", &self.tpf4())
            .field("tpf5", &self.tpf5())
            .field("tpf6", &self.tpf6())
            .field("tpf7", &self.tpf7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sr {{ dtf: {=bool:?}, taf: {=bool:?}, tif0: {=bool:?}, tif1: {=bool:?}, tif2: {=bool:?}, tif3: {=bool:?}, tif4: {=bool:?}, tif5: {=bool:?}, tif6: {=bool:?}, tif7: {=bool:?}, tif8: {=bool:?}, tif9: {=bool:?}, tif10: {=bool:?}, tpf0: {=bool:?}, tpf1: {=bool:?}, tpf2: {=bool:?}, tpf3: {=bool:?}, tpf4: {=bool:?}, tpf5: {=bool:?}, tpf6: {=bool:?}, tpf7: {=bool:?} }}",
            self.dtf(),
            self.taf(),
            self.tif0(),
            self.tif1(),
            self.tif2(),
            self.tif3(),
            self.tif4(),
            self.tif5(),
            self.tif6(),
            self.tif7(),
            self.tif8(),
            self.tif9(),
            self.tif10(),
            self.tpf0(),
            self.tpf1(),
            self.tpf2(),
            self.tpf3(),
            self.tpf4(),
            self.tpf5(),
            self.tpf6(),
            self.tpf7()
        )
    }
}
#[doc = "Tamper Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ter(pub u32);
impl Ter {
    #[doc = "Tamper Input Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input Enable"]
    #[inline(always)]
    pub const fn set_tie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Tamper Input Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input Enable"]
    #[inline(always)]
    pub const fn set_tie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Tamper Input Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input Enable"]
    #[inline(always)]
    pub const fn set_tie2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Tamper Input Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie3(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input Enable"]
    #[inline(always)]
    pub const fn set_tie3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Tamper Input Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie4(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input Enable"]
    #[inline(always)]
    pub const fn set_tie4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Tamper Input Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie5(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input Enable"]
    #[inline(always)]
    pub const fn set_tie5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Tamper Input Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie6(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input Enable"]
    #[inline(always)]
    pub const fn set_tie6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Tamper Input Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie7(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input Enable"]
    #[inline(always)]
    pub const fn set_tie7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Tamper Input Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie8(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input Enable"]
    #[inline(always)]
    pub const fn set_tie8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Tamper Input Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie9(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input Enable"]
    #[inline(always)]
    pub const fn set_tie9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Tamper Input Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie10(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Input Enable"]
    #[inline(always)]
    pub const fn set_tie10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Tamper Pin Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tpe0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin Enable"]
    #[inline(always)]
    pub const fn set_tpe0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Tamper Pin Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tpe1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin Enable"]
    #[inline(always)]
    pub const fn set_tpe1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Tamper Pin Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tpe2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin Enable"]
    #[inline(always)]
    pub const fn set_tpe2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Tamper Pin Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tpe3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin Enable"]
    #[inline(always)]
    pub const fn set_tpe3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Tamper Pin Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tpe4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin Enable"]
    #[inline(always)]
    pub const fn set_tpe4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Tamper Pin Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tpe5(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin Enable"]
    #[inline(always)]
    pub const fn set_tpe5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Tamper Pin Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tpe6(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin Enable"]
    #[inline(always)]
    pub const fn set_tpe6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Tamper Pin Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tpe7(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Pin Enable"]
    #[inline(always)]
    pub const fn set_tpe7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Ter {
    #[inline(always)]
    fn default() -> Ter {
        Ter(0)
    }
}
impl core::fmt::Debug for Ter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ter")
            .field("tie0", &self.tie0())
            .field("tie1", &self.tie1())
            .field("tie2", &self.tie2())
            .field("tie3", &self.tie3())
            .field("tie4", &self.tie4())
            .field("tie5", &self.tie5())
            .field("tie6", &self.tie6())
            .field("tie7", &self.tie7())
            .field("tie8", &self.tie8())
            .field("tie9", &self.tie9())
            .field("tie10", &self.tie10())
            .field("tpe0", &self.tpe0())
            .field("tpe1", &self.tpe1())
            .field("tpe2", &self.tpe2())
            .field("tpe3", &self.tpe3())
            .field("tpe4", &self.tpe4())
            .field("tpe5", &self.tpe5())
            .field("tpe6", &self.tpe6())
            .field("tpe7", &self.tpe7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ter {{ tie0: {=bool:?}, tie1: {=bool:?}, tie2: {=bool:?}, tie3: {=bool:?}, tie4: {=bool:?}, tie5: {=bool:?}, tie6: {=bool:?}, tie7: {=bool:?}, tie8: {=bool:?}, tie9: {=bool:?}, tie10: {=bool:?}, tpe0: {=bool:?}, tpe1: {=bool:?}, tpe2: {=bool:?}, tpe3: {=bool:?}, tpe4: {=bool:?}, tpe5: {=bool:?}, tpe6: {=bool:?}, tpe7: {=bool:?} }}",
            self.tie0(),
            self.tie1(),
            self.tie2(),
            self.tie3(),
            self.tie4(),
            self.tie5(),
            self.tie6(),
            self.tie7(),
            self.tie8(),
            self.tie9(),
            self.tie10(),
            self.tpe0(),
            self.tpe1(),
            self.tpe2(),
            self.tpe3(),
            self.tpe4(),
            self.tpe5(),
            self.tpe6(),
            self.tpe7()
        )
    }
}
#[doc = "Tamper Seconds"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsr(pub u32);
impl Tsr {
    #[doc = "Tamper Time Seconds"]
    #[must_use]
    #[inline(always)]
    pub const fn tts(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Tamper Time Seconds"]
    #[inline(always)]
    pub const fn set_tts(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tsr {
    #[inline(always)]
    fn default() -> Tsr {
        Tsr(0)
    }
}
impl core::fmt::Debug for Tsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsr").field("tts", &self.tts()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tsr {{ tts: {=u32:?} }}", self.tts())
    }
}
#[doc = "Write Access Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wac(pub u32);
impl Wac {
    #[doc = "Control Register Write"]
    #[must_use]
    #[inline(always)]
    pub const fn crw(&self) -> super::vals::Crw {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Crw::from_bits(val as u8)
    }
    #[doc = "Control Register Write"]
    #[inline(always)]
    pub const fn set_crw(&mut self, val: super::vals::Crw) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Status Register Write"]
    #[must_use]
    #[inline(always)]
    pub const fn srw(&self) -> super::vals::Srw {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Srw::from_bits(val as u8)
    }
    #[doc = "Status Register Write"]
    #[inline(always)]
    pub const fn set_srw(&mut self, val: super::vals::Srw) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Lock Register Write"]
    #[must_use]
    #[inline(always)]
    pub const fn lrw(&self) -> super::vals::Lrw {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Lrw::from_bits(val as u8)
    }
    #[doc = "Lock Register Write"]
    #[inline(always)]
    pub const fn set_lrw(&mut self, val: super::vals::Lrw) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt Enable Write"]
    #[must_use]
    #[inline(always)]
    pub const fn iew(&self) -> super::vals::Iew {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Iew::from_bits(val as u8)
    }
    #[doc = "Interrupt Enable Write"]
    #[inline(always)]
    pub const fn set_iew(&mut self, val: super::vals::Iew) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Tamper Seconds Register Write"]
    #[must_use]
    #[inline(always)]
    pub const fn tsrw(&self) -> super::vals::Tsrw {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Tsrw::from_bits(val as u8)
    }
    #[doc = "Tamper Seconds Register Write"]
    #[inline(always)]
    pub const fn set_tsrw(&mut self, val: super::vals::Tsrw) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Tamper Enable Write"]
    #[must_use]
    #[inline(always)]
    pub const fn tew(&self) -> super::vals::Tew {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Tew::from_bits(val as u8)
    }
    #[doc = "Tamper Enable Write"]
    #[inline(always)]
    pub const fn set_tew(&mut self, val: super::vals::Tew) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Pin Polarity Write"]
    #[must_use]
    #[inline(always)]
    pub const fn ppw(&self) -> super::vals::Ppw {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Ppw::from_bits(val as u8)
    }
    #[doc = "Pin Polarity Write"]
    #[inline(always)]
    pub const fn set_ppw(&mut self, val: super::vals::Ppw) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Glitch Filter Write"]
    #[must_use]
    #[inline(always)]
    pub const fn gfw0(&self) -> super::vals::Gfw0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Gfw0::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Write"]
    #[inline(always)]
    pub const fn set_gfw0(&mut self, val: super::vals::Gfw0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Glitch Filter Write"]
    #[must_use]
    #[inline(always)]
    pub const fn gfw1(&self) -> super::vals::Gfw1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Gfw1::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Write"]
    #[inline(always)]
    pub const fn set_gfw1(&mut self, val: super::vals::Gfw1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Glitch Filter Write"]
    #[must_use]
    #[inline(always)]
    pub const fn gfw2(&self) -> super::vals::Gfw2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Gfw2::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Write"]
    #[inline(always)]
    pub const fn set_gfw2(&mut self, val: super::vals::Gfw2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Glitch Filter Write"]
    #[must_use]
    #[inline(always)]
    pub const fn gfw3(&self) -> super::vals::Gfw3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Gfw3::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Write"]
    #[inline(always)]
    pub const fn set_gfw3(&mut self, val: super::vals::Gfw3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Glitch Filter Write"]
    #[must_use]
    #[inline(always)]
    pub const fn gfw4(&self) -> super::vals::Gfw4 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Gfw4::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Write"]
    #[inline(always)]
    pub const fn set_gfw4(&mut self, val: super::vals::Gfw4) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Glitch Filter Write"]
    #[must_use]
    #[inline(always)]
    pub const fn gfw5(&self) -> super::vals::Gfw5 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Gfw5::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Write"]
    #[inline(always)]
    pub const fn set_gfw5(&mut self, val: super::vals::Gfw5) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Glitch Filter Write"]
    #[must_use]
    #[inline(always)]
    pub const fn gfw6(&self) -> super::vals::Gfw6 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Gfw6::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Write"]
    #[inline(always)]
    pub const fn set_gfw6(&mut self, val: super::vals::Gfw6) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Glitch Filter Write"]
    #[must_use]
    #[inline(always)]
    pub const fn gfw7(&self) -> super::vals::Gfw7 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Gfw7::from_bits(val as u8)
    }
    #[doc = "Glitch Filter Write"]
    #[inline(always)]
    pub const fn set_gfw7(&mut self, val: super::vals::Gfw7) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for Wac {
    #[inline(always)]
    fn default() -> Wac {
        Wac(0)
    }
}
impl core::fmt::Debug for Wac {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wac")
            .field("crw", &self.crw())
            .field("srw", &self.srw())
            .field("lrw", &self.lrw())
            .field("iew", &self.iew())
            .field("tsrw", &self.tsrw())
            .field("tew", &self.tew())
            .field("ppw", &self.ppw())
            .field("gfw0", &self.gfw0())
            .field("gfw1", &self.gfw1())
            .field("gfw2", &self.gfw2())
            .field("gfw3", &self.gfw3())
            .field("gfw4", &self.gfw4())
            .field("gfw5", &self.gfw5())
            .field("gfw6", &self.gfw6())
            .field("gfw7", &self.gfw7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wac {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wac {{ crw: {:?}, srw: {:?}, lrw: {:?}, iew: {:?}, tsrw: {:?}, tew: {:?}, ppw: {:?}, gfw0: {:?}, gfw1: {:?}, gfw2: {:?}, gfw3: {:?}, gfw4: {:?}, gfw5: {:?}, gfw6: {:?}, gfw7: {:?} }}",
            self.crw(),
            self.srw(),
            self.lrw(),
            self.iew(),
            self.tsrw(),
            self.tew(),
            self.ppw(),
            self.gfw0(),
            self.gfw1(),
            self.gfw2(),
            self.gfw3(),
            self.gfw4(),
            self.gfw5(),
            self.gfw6(),
            self.gfw7()
        )
    }
}
