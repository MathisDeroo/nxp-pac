#[doc = "DMA Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmactrl(pub u32);
impl Dmactrl {
    #[doc = "DMA 0 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0en(&self) -> super::vals::Dma0en {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Dma0en::from_bits(val as u8)
    }
    #[doc = "DMA 0 Enable"]
    #[inline(always)]
    pub const fn set_dma0en(&mut self, val: super::vals::Dma0en) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "DMA 1 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1en(&self) -> super::vals::Dma1en {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Dma1en::from_bits(val as u8)
    }
    #[doc = "DMA 1 Enable"]
    #[inline(always)]
    pub const fn set_dma1en(&mut self, val: super::vals::Dma1en) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "DMA 0 Port"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0port(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "DMA 0 Port"]
    #[inline(always)]
    pub const fn set_dma0port(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Selects port operating DMA"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1port(&self) -> super::vals::Dma1port {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Dma1port::from_bits(val as u8)
    }
    #[doc = "Selects port operating DMA"]
    #[inline(always)]
    pub const fn set_dma1port(&mut self, val: super::vals::Dma1port) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Reload Count 0"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt0(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Reload Count 0"]
    #[inline(always)]
    pub const fn set_cnt0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "Reload Count 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt1(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "Reload Count 1"]
    #[inline(always)]
    pub const fn set_cnt1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
}
impl Default for Dmactrl {
    #[inline(always)]
    fn default() -> Dmactrl {
        Dmactrl(0)
    }
}
impl core::fmt::Debug for Dmactrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmactrl")
            .field("dma0en", &self.dma0en())
            .field("dma1en", &self.dma1en())
            .field("dma0port", &self.dma0port())
            .field("dma1port", &self.dma1port())
            .field("cnt0", &self.cnt0())
            .field("cnt1", &self.cnt1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmactrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmactrl {{ dma0en: {:?}, dma1en: {:?}, dma0port: {=u8:?}, dma1port: {:?}, cnt0: {=u8:?}, cnt1: {=u8:?} }}",
            self.dma0en(),
            self.dma1en(),
            self.dma0port(),
            self.dma1port(),
            self.cnt0(),
            self.cnt1()
        )
    }
}
#[doc = "eSPI Capabilities"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Espicap(pub u32);
impl Espicap {
    #[doc = "SPI mode allowed (host still has to select):"]
    #[must_use]
    #[inline(always)]
    pub const fn spicap(&self) -> super::vals::Spicap {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Spicap::from_bits(val as u8)
    }
    #[doc = "SPI mode allowed (host still has to select):"]
    #[inline(always)]
    pub const fn set_spicap(&mut self, val: super::vals::Spicap) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Maximum SPI Clock Speed"]
    #[must_use]
    #[inline(always)]
    pub const fn maxspd(&self) -> super::vals::Maxspd {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Maxspd::from_bits(val as u8)
    }
    #[doc = "Maximum SPI Clock Speed"]
    #[inline(always)]
    pub const fn set_maxspd(&mut self, val: super::vals::Maxspd) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Alert Pin"]
    #[must_use]
    #[inline(always)]
    pub const fn alpin(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Alert Pin"]
    #[inline(always)]
    pub const fn set_alpin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "OOB Allow"]
    #[must_use]
    #[inline(always)]
    pub const fn oobok(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OOB Allow"]
    #[inline(always)]
    pub const fn set_oobok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Maximum Flash Payload Size"]
    #[must_use]
    #[inline(always)]
    pub const fn flashmx(&self) -> super::vals::Flashmx {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Flashmx::from_bits(val as u8)
    }
    #[doc = "Maximum Flash Payload Size"]
    #[inline(always)]
    pub const fn set_flashmx(&mut self, val: super::vals::Flashmx) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Slave-Attached Flash"]
    #[must_use]
    #[inline(always)]
    pub const fn saf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Slave-Attached Flash"]
    #[inline(always)]
    pub const fn set_saf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SAF Erase Sector"]
    #[must_use]
    #[inline(always)]
    pub const fn safera(&self) -> super::vals::Safera {
        let val = (self.0 >> 13usize) & 0x0f;
        super::vals::Safera::from_bits(val as u8)
    }
    #[doc = "SAF Erase Sector"]
    #[inline(always)]
    pub const fn set_safera(&mut self, val: super::vals::Safera) {
        self.0 = (self.0 & !(0x0f << 13usize)) | (((val.to_bits() as u32) & 0x0f) << 13usize);
    }
    #[doc = "Master-Attached Flash"]
    #[must_use]
    #[inline(always)]
    pub const fn maf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Master-Attached Flash"]
    #[inline(always)]
    pub const fn set_maf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Target Maximum Read Request Size Supported"]
    #[must_use]
    #[inline(always)]
    pub const fn trgt_req_size_supp(&self) -> super::vals::TrgtReqSizeSupp {
        let val = (self.0 >> 18usize) & 0x07;
        super::vals::TrgtReqSizeSupp::from_bits(val as u8)
    }
    #[doc = "Target Maximum Read Request Size Supported"]
    #[inline(always)]
    pub const fn set_trgt_req_size_supp(&mut self, val: super::vals::TrgtReqSizeSupp) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val.to_bits() as u32) & 0x07) << 18usize);
    }
    #[doc = "Peripheral Channel Maximum Payload Size Supported"]
    #[must_use]
    #[inline(always)]
    pub const fn memmx(&self) -> super::vals::Memmx {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::Memmx::from_bits(val as u8)
    }
    #[doc = "Peripheral Channel Maximum Payload Size Supported"]
    #[inline(always)]
    pub const fn set_memmx(&mut self, val: super::vals::Memmx) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "OOB Message Channel Maximum Payload Size Supported"]
    #[must_use]
    #[inline(always)]
    pub const fn oobmx(&self) -> super::vals::Oobmx {
        let val = (self.0 >> 23usize) & 0x03;
        super::vals::Oobmx::from_bits(val as u8)
    }
    #[doc = "OOB Message Channel Maximum Payload Size Supported"]
    #[inline(always)]
    pub const fn set_oobmx(&mut self, val: super::vals::Oobmx) {
        self.0 = (self.0 & !(0x03 << 23usize)) | (((val.to_bits() as u32) & 0x03) << 23usize);
    }
}
impl Default for Espicap {
    #[inline(always)]
    fn default() -> Espicap {
        Espicap(0)
    }
}
impl core::fmt::Debug for Espicap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Espicap")
            .field("spicap", &self.spicap())
            .field("maxspd", &self.maxspd())
            .field("alpin", &self.alpin())
            .field("oobok", &self.oobok())
            .field("flashmx", &self.flashmx())
            .field("saf", &self.saf())
            .field("safera", &self.safera())
            .field("maf", &self.maf())
            .field("trgt_req_size_supp", &self.trgt_req_size_supp())
            .field("memmx", &self.memmx())
            .field("oobmx", &self.oobmx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Espicap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Espicap {{ spicap: {:?}, maxspd: {:?}, alpin: {=bool:?}, oobok: {=bool:?}, flashmx: {:?}, saf: {=bool:?}, safera: {:?}, maf: {=bool:?}, trgt_req_size_supp: {:?}, memmx: {:?}, oobmx: {:?} }}",
            self.spicap(),
            self.maxspd(),
            self.alpin(),
            self.oobok(),
            self.flashmx(),
            self.saf(),
            self.safera(),
            self.maf(),
            self.trgt_req_size_supp(),
            self.memmx(),
            self.oobmx()
        )
    }
}
#[doc = "eSPI Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Espicfg(pub u32);
impl Espicfg {
    #[doc = "Slave-Attached Flash Supported"]
    #[must_use]
    #[inline(always)]
    pub const fn saf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Slave-Attached Flash Supported"]
    #[inline(always)]
    pub const fn set_saf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Flash Size"]
    #[must_use]
    #[inline(always)]
    pub const fn flashsz(&self) -> super::vals::Flashsz {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Flashsz::from_bits(val as u8)
    }
    #[doc = "Flash Size"]
    #[inline(always)]
    pub const fn set_flashsz(&mut self, val: super::vals::Flashsz) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "SPI Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn spimod(&self) -> super::vals::Spimod {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::Spimod::from_bits(val as u8)
    }
    #[doc = "SPI Mode"]
    #[inline(always)]
    pub const fn set_spimod(&mut self, val: super::vals::Spimod) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Alert Is Pin"]
    #[must_use]
    #[inline(always)]
    pub const fn alert(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Alert Is Pin"]
    #[inline(always)]
    pub const fn set_alert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Alert Is Open Drain as Pin"]
    #[must_use]
    #[inline(always)]
    pub const fn alertod(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Alert Is Open Drain as Pin"]
    #[inline(always)]
    pub const fn set_alertod(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SPI Speed"]
    #[must_use]
    #[inline(always)]
    pub const fn spispd(&self) -> super::vals::Spispd {
        let val = (self.0 >> 7usize) & 0x07;
        super::vals::Spispd::from_bits(val as u8)
    }
    #[doc = "SPI Speed"]
    #[inline(always)]
    pub const fn set_spispd(&mut self, val: super::vals::Spispd) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val.to_bits() as u32) & 0x07) << 7usize);
    }
    #[doc = "CRC Checking Enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn crc(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CRC Checking Enabled"]
    #[inline(always)]
    pub const fn set_crc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Bus Master OK"]
    #[must_use]
    #[inline(always)]
    pub const fn busmok(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Master OK"]
    #[inline(always)]
    pub const fn set_busmok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Channel 0 (Memory) Enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn memena(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 (Memory) Enabled"]
    #[inline(always)]
    pub const fn set_memena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Channel 1 (VWire) Enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn vwok(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 (VWire) Enabled"]
    #[inline(always)]
    pub const fn set_vwok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Channel 2 (OOB) Enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn oobok(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 2 (OOB) Enabled"]
    #[inline(always)]
    pub const fn set_oobok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Flash Erase Size and Whether Enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn flshera(&self) -> super::vals::Flshera {
        let val = (self.0 >> 15usize) & 0x07;
        super::vals::Flshera::from_bits(val as u8)
    }
    #[doc = "Flash Erase Size and Whether Enabled"]
    #[inline(always)]
    pub const fn set_flshera(&mut self, val: super::vals::Flshera) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val.to_bits() as u32) & 0x07) << 15usize);
    }
    #[doc = "Channel 3 (Flash) Enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn flshok(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 3 (Flash) Enabled"]
    #[inline(always)]
    pub const fn set_flshok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Peripheral Channel Maximum Payload Size Selected"]
    #[must_use]
    #[inline(always)]
    pub const fn memsz(&self) -> super::vals::Memsz {
        let val = (self.0 >> 19usize) & 0x03;
        super::vals::Memsz::from_bits(val as u8)
    }
    #[doc = "Peripheral Channel Maximum Payload Size Selected"]
    #[inline(always)]
    pub const fn set_memsz(&mut self, val: super::vals::Memsz) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val.to_bits() as u32) & 0x03) << 19usize);
    }
    #[doc = "OOB Message Channel Maximum Payload Size Selected"]
    #[must_use]
    #[inline(always)]
    pub const fn oobsz(&self) -> super::vals::Oobsz {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::Oobsz::from_bits(val as u8)
    }
    #[doc = "OOB Message Channel Maximum Payload Size Selected"]
    #[inline(always)]
    pub const fn set_oobsz(&mut self, val: super::vals::Oobsz) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
}
impl Default for Espicfg {
    #[inline(always)]
    fn default() -> Espicfg {
        Espicfg(0)
    }
}
impl core::fmt::Debug for Espicfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Espicfg")
            .field("saf", &self.saf())
            .field("flashsz", &self.flashsz())
            .field("spimod", &self.spimod())
            .field("alert", &self.alert())
            .field("alertod", &self.alertod())
            .field("spispd", &self.spispd())
            .field("crc", &self.crc())
            .field("busmok", &self.busmok())
            .field("memena", &self.memena())
            .field("vwok", &self.vwok())
            .field("oobok", &self.oobok())
            .field("flshera", &self.flshera())
            .field("flshok", &self.flshok())
            .field("memsz", &self.memsz())
            .field("oobsz", &self.oobsz())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Espicfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Espicfg {{ saf: {=bool:?}, flashsz: {:?}, spimod: {:?}, alert: {=bool:?}, alertod: {=bool:?}, spispd: {:?}, crc: {=bool:?}, busmok: {=bool:?}, memena: {=bool:?}, vwok: {=bool:?}, oobok: {=bool:?}, flshera: {:?}, flshok: {=bool:?}, memsz: {:?}, oobsz: {:?} }}",
            self.saf(),
            self.flashsz(),
            self.spimod(),
            self.alert(),
            self.alertod(),
            self.spispd(),
            self.crc(),
            self.busmok(),
            self.memena(),
            self.vwok(),
            self.oobok(),
            self.flshera(),
            self.flshok(),
            self.memsz(),
            self.oobsz()
        )
    }
}
#[doc = "eSPI Miscellaneous"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Espimisc(pub u32);
impl Espimisc {
    #[doc = "GPIO Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_oe(&self) -> super::vals::GpioOe {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::GpioOe::from_bits(val as u8)
    }
    #[doc = "GPIO Output Enable"]
    #[inline(always)]
    pub const fn set_gpio_oe(&mut self, val: super::vals::GpioOe) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "GPIO Open Drain"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_od(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO Open Drain"]
    #[inline(always)]
    pub const fn set_gpio_od(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Not used in eSPI if ESPICFG\\[ALERT\\] is 1"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_out(&self) -> super::vals::GpioOut {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::GpioOut::from_bits(val as u8)
    }
    #[doc = "Not used in eSPI if ESPICFG\\[ALERT\\] is 1"]
    #[inline(always)]
    pub const fn set_gpio_out(&mut self, val: super::vals::GpioOut) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "GPIO Input"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_in(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO Input"]
    #[inline(always)]
    pub const fn set_gpio_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RSTN Pin Is GPIO"]
    #[must_use]
    #[inline(always)]
    pub const fn risgp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "RSTN Pin Is GPIO"]
    #[inline(always)]
    pub const fn set_risgp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Power Save"]
    #[must_use]
    #[inline(always)]
    pub const fn pwrsav(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Power Save"]
    #[inline(always)]
    pub const fn set_pwrsav(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Espimisc {
    #[inline(always)]
    fn default() -> Espimisc {
        Espimisc(0)
    }
}
impl core::fmt::Debug for Espimisc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Espimisc")
            .field("gpio_oe", &self.gpio_oe())
            .field("gpio_od", &self.gpio_od())
            .field("gpio_out", &self.gpio_out())
            .field("gpio_in", &self.gpio_in())
            .field("risgp", &self.risgp())
            .field("pwrsav", &self.pwrsav())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Espimisc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Espimisc {{ gpio_oe: {:?}, gpio_od: {=bool:?}, gpio_out: {:?}, gpio_in: {=bool:?}, risgp: {=bool:?}, pwrsav: {=bool:?} }}",
            self.gpio_oe(),
            self.gpio_od(),
            self.gpio_out(),
            self.gpio_in(),
            self.risgp(),
            self.pwrsav()
        )
    }
}
#[doc = "Interrupt Clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenclr(pub u32);
impl Intenclr {
    #[doc = "Port Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn portint(&self) -> super::vals::IntenclrPortint {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::IntenclrPortint::from_bits(val as u8)
    }
    #[doc = "Port Interrupt"]
    #[inline(always)]
    pub const fn set_portint(&mut self, val: super::vals::IntenclrPortint) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Port80 Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn p80int(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port80 Interrupt"]
    #[inline(always)]
    pub const fn set_p80int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bus Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn busrst(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Reset"]
    #[inline(always)]
    pub const fn set_busrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "IRQ Update"]
    #[must_use]
    #[inline(always)]
    pub const fn irqupd(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ Update"]
    #[inline(always)]
    pub const fn set_irqupd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Wire Change"]
    #[must_use]
    #[inline(always)]
    pub const fn wirechg(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Wire Change"]
    #[inline(always)]
    pub const fn set_wirechg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Host Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn hstall(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Host Stall"]
    #[inline(always)]
    pub const fn set_hstall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CRC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn crcerr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CRC Error"]
    #[inline(always)]
    pub const fn set_crcerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GPIO"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO"]
    #[inline(always)]
    pub const fn set_gpio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "CS_INT"]
    #[must_use]
    #[inline(always)]
    pub const fn cs_int(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "CS_INT"]
    #[inline(always)]
    pub const fn set_cs_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Coprocessor Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn cportint(&self) -> super::vals::IntenclrCportint {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::IntenclrCportint::from_bits(val as u8)
    }
    #[doc = "Coprocessor Interrupt"]
    #[inline(always)]
    pub const fn set_cportint(&mut self, val: super::vals::IntenclrCportint) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Coprocesssor Port80 Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn cp80int(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Coprocesssor Port80 Interrupt"]
    #[inline(always)]
    pub const fn set_cp80int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Coprocessor Burst"]
    #[must_use]
    #[inline(always)]
    pub const fn cbusrst(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Coprocessor Burst"]
    #[inline(always)]
    pub const fn set_cbusrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Coprocessor IRQ Update"]
    #[must_use]
    #[inline(always)]
    pub const fn cirqupd(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Coprocessor IRQ Update"]
    #[inline(always)]
    pub const fn set_cirqupd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Coprocessor Wire Change"]
    #[must_use]
    #[inline(always)]
    pub const fn cwirechg(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Coprocessor Wire Change"]
    #[inline(always)]
    pub const fn set_cwirechg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Coprocessor Host Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn chstall(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Coprocessor Host Stall"]
    #[inline(always)]
    pub const fn set_chstall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Coprocessor CRC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn ccrcerr(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Coprocessor CRC Error"]
    #[inline(always)]
    pub const fn set_ccrcerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Coprocessor GPIO"]
    #[must_use]
    #[inline(always)]
    pub const fn cgpio(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Coprocessor GPIO"]
    #[inline(always)]
    pub const fn set_cgpio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Intenclr {
    #[inline(always)]
    fn default() -> Intenclr {
        Intenclr(0)
    }
}
impl core::fmt::Debug for Intenclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intenclr")
            .field("portint", &self.portint())
            .field("p80int", &self.p80int())
            .field("busrst", &self.busrst())
            .field("irqupd", &self.irqupd())
            .field("wirechg", &self.wirechg())
            .field("hstall", &self.hstall())
            .field("crcerr", &self.crcerr())
            .field("gpio", &self.gpio())
            .field("cs_int", &self.cs_int())
            .field("cportint", &self.cportint())
            .field("cp80int", &self.cp80int())
            .field("cbusrst", &self.cbusrst())
            .field("cirqupd", &self.cirqupd())
            .field("cwirechg", &self.cwirechg())
            .field("chstall", &self.chstall())
            .field("ccrcerr", &self.ccrcerr())
            .field("cgpio", &self.cgpio())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intenclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intenclr {{ portint: {:?}, p80int: {=bool:?}, busrst: {=bool:?}, irqupd: {=bool:?}, wirechg: {=bool:?}, hstall: {=bool:?}, crcerr: {=bool:?}, gpio: {=bool:?}, cs_int: {=bool:?}, cportint: {:?}, cp80int: {=bool:?}, cbusrst: {=bool:?}, cirqupd: {=bool:?}, cwirechg: {=bool:?}, chstall: {=bool:?}, ccrcerr: {=bool:?}, cgpio: {=bool:?} }}",
            self.portint(),
            self.p80int(),
            self.busrst(),
            self.irqupd(),
            self.wirechg(),
            self.hstall(),
            self.crcerr(),
            self.gpio(),
            self.cs_int(),
            self.cportint(),
            self.cp80int(),
            self.cbusrst(),
            self.cirqupd(),
            self.cwirechg(),
            self.chstall(),
            self.ccrcerr(),
            self.cgpio()
        )
    }
}
#[doc = "Interrupt Enable Set"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenset(pub u32);
impl Intenset {
    #[doc = "Port interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn portint(&self) -> super::vals::IntensetPortint {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::IntensetPortint::from_bits(val as u8)
    }
    #[doc = "Port interrupt"]
    #[inline(always)]
    pub const fn set_portint(&mut self, val: super::vals::IntensetPortint) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Port80 Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn p80int(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port80 Interrupt"]
    #[inline(always)]
    pub const fn set_p80int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bus Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn busrst(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Reset"]
    #[inline(always)]
    pub const fn set_busrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "IRQ Update"]
    #[must_use]
    #[inline(always)]
    pub const fn irqupd(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ Update"]
    #[inline(always)]
    pub const fn set_irqupd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Wire Change"]
    #[must_use]
    #[inline(always)]
    pub const fn wirechg(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Wire Change"]
    #[inline(always)]
    pub const fn set_wirechg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Host Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn hstall(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Host Stall"]
    #[inline(always)]
    pub const fn set_hstall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CRC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn crcerr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CRC Error"]
    #[inline(always)]
    pub const fn set_crcerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GPIO"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO"]
    #[inline(always)]
    pub const fn set_gpio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "CS_INT"]
    #[must_use]
    #[inline(always)]
    pub const fn cs_int(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "CS_INT"]
    #[inline(always)]
    pub const fn set_cs_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Coprocessor Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn cportint(&self) -> super::vals::IntensetCportint {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::IntensetCportint::from_bits(val as u8)
    }
    #[doc = "Coprocessor Interrupt"]
    #[inline(always)]
    pub const fn set_cportint(&mut self, val: super::vals::IntensetCportint) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Coprocessor Port80 Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn cp80int(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Coprocessor Port80 Interrupt"]
    #[inline(always)]
    pub const fn set_cp80int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Coprocessor Burst"]
    #[must_use]
    #[inline(always)]
    pub const fn cbusrst(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Coprocessor Burst"]
    #[inline(always)]
    pub const fn set_cbusrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Coprocessor Interrupt Update"]
    #[must_use]
    #[inline(always)]
    pub const fn cirqupd(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Coprocessor Interrupt Update"]
    #[inline(always)]
    pub const fn set_cirqupd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Coprocessor Wire Change"]
    #[must_use]
    #[inline(always)]
    pub const fn cwirechg(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Coprocessor Wire Change"]
    #[inline(always)]
    pub const fn set_cwirechg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Coprocessor Host Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn chstall(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Coprocessor Host Stall"]
    #[inline(always)]
    pub const fn set_chstall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Coprocessor Error"]
    #[must_use]
    #[inline(always)]
    pub const fn ccrcerr(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Coprocessor Error"]
    #[inline(always)]
    pub const fn set_ccrcerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Coprocessor GPIO"]
    #[must_use]
    #[inline(always)]
    pub const fn cgpio(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Coprocessor GPIO"]
    #[inline(always)]
    pub const fn set_cgpio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Intenset {
    #[inline(always)]
    fn default() -> Intenset {
        Intenset(0)
    }
}
impl core::fmt::Debug for Intenset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intenset")
            .field("portint", &self.portint())
            .field("p80int", &self.p80int())
            .field("busrst", &self.busrst())
            .field("irqupd", &self.irqupd())
            .field("wirechg", &self.wirechg())
            .field("hstall", &self.hstall())
            .field("crcerr", &self.crcerr())
            .field("gpio", &self.gpio())
            .field("cs_int", &self.cs_int())
            .field("cportint", &self.cportint())
            .field("cp80int", &self.cp80int())
            .field("cbusrst", &self.cbusrst())
            .field("cirqupd", &self.cirqupd())
            .field("cwirechg", &self.cwirechg())
            .field("chstall", &self.chstall())
            .field("ccrcerr", &self.ccrcerr())
            .field("cgpio", &self.cgpio())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intenset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intenset {{ portint: {:?}, p80int: {=bool:?}, busrst: {=bool:?}, irqupd: {=bool:?}, wirechg: {=bool:?}, hstall: {=bool:?}, crcerr: {=bool:?}, gpio: {=bool:?}, cs_int: {=bool:?}, cportint: {:?}, cp80int: {=bool:?}, cbusrst: {=bool:?}, cirqupd: {=bool:?}, cwirechg: {=bool:?}, chstall: {=bool:?}, ccrcerr: {=bool:?}, cgpio: {=bool:?} }}",
            self.portint(),
            self.p80int(),
            self.busrst(),
            self.irqupd(),
            self.wirechg(),
            self.hstall(),
            self.crcerr(),
            self.gpio(),
            self.cs_int(),
            self.cportint(),
            self.cp80int(),
            self.cbusrst(),
            self.cirqupd(),
            self.cwirechg(),
            self.chstall(),
            self.ccrcerr(),
            self.cgpio()
        )
    }
}
#[doc = "Masked Interrupt Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc = "Port Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn portint(&self) -> super::vals::IntstatPortint {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::IntstatPortint::from_bits(val as u8)
    }
    #[doc = "Port Interrupt"]
    #[inline(always)]
    pub const fn set_portint(&mut self, val: super::vals::IntstatPortint) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Port80 Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn p80int(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port80 Interrupt"]
    #[inline(always)]
    pub const fn set_p80int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bus Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn busrst(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Reset"]
    #[inline(always)]
    pub const fn set_busrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "IRQ Update"]
    #[must_use]
    #[inline(always)]
    pub const fn irqupd(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ Update"]
    #[inline(always)]
    pub const fn set_irqupd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Wire Change"]
    #[must_use]
    #[inline(always)]
    pub const fn wirechg(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Wire Change"]
    #[inline(always)]
    pub const fn set_wirechg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Host Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn hstall(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Host Stall"]
    #[inline(always)]
    pub const fn set_hstall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CRC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn crcerr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CRC Error"]
    #[inline(always)]
    pub const fn set_crcerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GPIO"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO"]
    #[inline(always)]
    pub const fn set_gpio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Coprocessor Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn cportint(&self) -> super::vals::IntstatCportint {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::IntstatCportint::from_bits(val as u8)
    }
    #[doc = "Coprocessor Interrupt"]
    #[inline(always)]
    pub const fn set_cportint(&mut self, val: super::vals::IntstatCportint) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Coprocessor Port80 Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn cp80int(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Coprocessor Port80 Interrupt"]
    #[inline(always)]
    pub const fn set_cp80int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Coprocessor Burst"]
    #[must_use]
    #[inline(always)]
    pub const fn cbusrst(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Coprocessor Burst"]
    #[inline(always)]
    pub const fn set_cbusrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Coprocessor IRQ Update"]
    #[must_use]
    #[inline(always)]
    pub const fn cirqupd(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Coprocessor IRQ Update"]
    #[inline(always)]
    pub const fn set_cirqupd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Coprocessor Wire Change"]
    #[must_use]
    #[inline(always)]
    pub const fn cwirechg(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Coprocessor Wire Change"]
    #[inline(always)]
    pub const fn set_cwirechg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Coprocessor Host Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn chstall(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Coprocessor Host Stall"]
    #[inline(always)]
    pub const fn set_chstall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Coprocessor CRC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn ccrcerr(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Coprocessor CRC Error"]
    #[inline(always)]
    pub const fn set_ccrcerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Coprocessor GPIO"]
    #[must_use]
    #[inline(always)]
    pub const fn cgpio(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Coprocessor GPIO"]
    #[inline(always)]
    pub const fn set_cgpio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Intstat {
    #[inline(always)]
    fn default() -> Intstat {
        Intstat(0)
    }
}
impl core::fmt::Debug for Intstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intstat")
            .field("portint", &self.portint())
            .field("p80int", &self.p80int())
            .field("busrst", &self.busrst())
            .field("irqupd", &self.irqupd())
            .field("wirechg", &self.wirechg())
            .field("hstall", &self.hstall())
            .field("crcerr", &self.crcerr())
            .field("gpio", &self.gpio())
            .field("cportint", &self.cportint())
            .field("cp80int", &self.cp80int())
            .field("cbusrst", &self.cbusrst())
            .field("cirqupd", &self.cirqupd())
            .field("cwirechg", &self.cwirechg())
            .field("chstall", &self.chstall())
            .field("ccrcerr", &self.ccrcerr())
            .field("cgpio", &self.cgpio())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intstat {{ portint: {:?}, p80int: {=bool:?}, busrst: {=bool:?}, irqupd: {=bool:?}, wirechg: {=bool:?}, hstall: {=bool:?}, crcerr: {=bool:?}, gpio: {=bool:?}, cportint: {:?}, cp80int: {=bool:?}, cbusrst: {=bool:?}, cirqupd: {=bool:?}, cwirechg: {=bool:?}, chstall: {=bool:?}, ccrcerr: {=bool:?}, cgpio: {=bool:?} }}",
            self.portint(),
            self.p80int(),
            self.busrst(),
            self.irqupd(),
            self.wirechg(),
            self.hstall(),
            self.crcerr(),
            self.gpio(),
            self.cportint(),
            self.cp80int(),
            self.cbusrst(),
            self.cirqupd(),
            self.cwirechg(),
            self.chstall(),
            self.ccrcerr(),
            self.cgpio()
        )
    }
}
#[doc = "IRQ Push"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irqpush(pub u32);
impl Irqpush {
    #[doc = "Interrupt Request Queue"]
    #[must_use]
    #[inline(always)]
    pub const fn irq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Interrupt Request Queue"]
    #[inline(always)]
    pub const fn set_irq(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "IRQ Update Done"]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ Update Done"]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Irqpush {
    #[inline(always)]
    fn default() -> Irqpush {
        Irqpush(0)
    }
}
impl core::fmt::Debug for Irqpush {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Irqpush")
            .field("irq", &self.irq())
            .field("done", &self.done())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Irqpush {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Irqpush {{ irq: {=u8:?}, done: {=bool:?} }}",
            self.irq(),
            self.done()
        )
    }
}
#[doc = "Mapped Base"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mapbase(pub u32);
impl Mapbase {
    #[doc = "Base 0"]
    #[must_use]
    #[inline(always)]
    pub const fn base0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Base 0"]
    #[inline(always)]
    pub const fn set_base0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Base 1"]
    #[must_use]
    #[inline(always)]
    pub const fn base1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Base 1"]
    #[inline(always)]
    pub const fn set_base1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Mapbase {
    #[inline(always)]
    fn default() -> Mapbase {
        Mapbase(0)
    }
}
impl core::fmt::Debug for Mapbase {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mapbase")
            .field("base0", &self.base0())
            .field("base1", &self.base1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mapbase {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mapbase {{ base0: {=u16:?}, base1: {=u16:?} }}",
            self.base0(),
            self.base1()
        )
    }
}
#[doc = "Master Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mctrl(pub u32);
impl Mctrl {
    #[doc = "Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Enable::from_bits(val as u8)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: super::vals::Enable) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "RTC-Integrated-BMC"]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_int_bmc(&self) -> super::vals::RtcIntBmc {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::RtcIntBmc::from_bits(val as u8)
    }
    #[doc = "RTC-Integrated-BMC"]
    #[inline(always)]
    pub const fn set_rtc_int_bmc(&mut self, val: super::vals::RtcIntBmc) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Port Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pena(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Port Enable"]
    #[inline(always)]
    pub const fn set_pena(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Port 80 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn p80ena(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Port 80 Enable"]
    #[inline(always)]
    pub const fn set_p80ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Status Block Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sblkena(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Status Block Enable"]
    #[inline(always)]
    pub const fn set_sblkena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Clock Division Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_div_disable(&self) -> super::vals::ClkDivDisable {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::ClkDivDisable::from_bits(val as u8)
    }
    #[doc = "Clock Division Disable"]
    #[inline(always)]
    pub const fn set_clk_div_disable(&mut self, val: super::vals::ClkDivDisable) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Early Sample"]
    #[must_use]
    #[inline(always)]
    pub const fn early_sample(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Early Sample"]
    #[inline(always)]
    pub const fn set_early_sample(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Mctrl {
    #[inline(always)]
    fn default() -> Mctrl {
        Mctrl(0)
    }
}
impl core::fmt::Debug for Mctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mctrl")
            .field("enable", &self.enable())
            .field("rtc_int_bmc", &self.rtc_int_bmc())
            .field("pena", &self.pena())
            .field("p80ena", &self.p80ena())
            .field("sblkena", &self.sblkena())
            .field("clk_div_disable", &self.clk_div_disable())
            .field("early_sample", &self.early_sample())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mctrl {{ enable: {:?}, rtc_int_bmc: {:?}, pena: {=u8:?}, p80ena: {=bool:?}, sblkena: {=bool:?}, clk_div_disable: {:?}, early_sample: {=bool:?} }}",
            self.enable(),
            self.rtc_int_bmc(),
            self.pena(),
            self.p80ena(),
            self.sblkena(),
            self.clk_div_disable(),
            self.early_sample()
        )
    }
}
#[doc = "Master Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstat(pub u32);
impl Mstat {
    #[doc = "Port Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn portint(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Port Interrupt"]
    #[inline(always)]
    pub const fn set_portint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Port80 Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn p80int(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port80 Interrupt Request"]
    #[inline(always)]
    pub const fn set_p80int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bus Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn busrst(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Reset"]
    #[inline(always)]
    pub const fn set_busrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt Request Update Completion"]
    #[must_use]
    #[inline(always)]
    pub const fn irqupd(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request Update Completion"]
    #[inline(always)]
    pub const fn set_irqupd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Wire Change"]
    #[must_use]
    #[inline(always)]
    pub const fn wirechg(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Wire Change"]
    #[inline(always)]
    pub const fn set_wirechg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Host Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn hstall(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Host Stall"]
    #[inline(always)]
    pub const fn set_hstall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Cyclic Redundancy Check (CRC) Error"]
    #[must_use]
    #[inline(always)]
    pub const fn crcerr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Cyclic Redundancy Check (CRC) Error"]
    #[inline(always)]
    pub const fn set_crcerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "General Purpose Input/Output (GPIO)"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "General Purpose Input/Output (GPIO)"]
    #[inline(always)]
    pub const fn set_gpio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Bus Busy"]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Busy"]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Bus Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn inrst(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Reset"]
    #[inline(always)]
    pub const fn set_inrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Completion Pending"]
    #[must_use]
    #[inline(always)]
    pub const fn comppend(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Completion Pending"]
    #[inline(always)]
    pub const fn set_comppend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Master Pending"]
    #[must_use]
    #[inline(always)]
    pub const fn mastpend(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Master Pending"]
    #[inline(always)]
    pub const fn set_mastpend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Alert Pending"]
    #[must_use]
    #[inline(always)]
    pub const fn alertpend(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Alert Pending"]
    #[inline(always)]
    pub const fn set_alertpend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Mstat {
    #[inline(always)]
    fn default() -> Mstat {
        Mstat(0)
    }
}
impl core::fmt::Debug for Mstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mstat")
            .field("portint", &self.portint())
            .field("p80int", &self.p80int())
            .field("busrst", &self.busrst())
            .field("irqupd", &self.irqupd())
            .field("wirechg", &self.wirechg())
            .field("hstall", &self.hstall())
            .field("crcerr", &self.crcerr())
            .field("gpio", &self.gpio())
            .field("busy", &self.busy())
            .field("inrst", &self.inrst())
            .field("comppend", &self.comppend())
            .field("mastpend", &self.mastpend())
            .field("alertpend", &self.alertpend())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mstat {{ portint: {=u8:?}, p80int: {=bool:?}, busrst: {=bool:?}, irqupd: {=bool:?}, wirechg: {=bool:?}, hstall: {=bool:?}, crcerr: {=bool:?}, gpio: {=bool:?}, busy: {=bool:?}, inrst: {=bool:?}, comppend: {=bool:?}, mastpend: {=bool:?}, alertpend: {=bool:?} }}",
            self.portint(),
            self.p80int(),
            self.busrst(),
            self.irqupd(),
            self.wirechg(),
            self.hstall(),
            self.crcerr(),
            self.gpio(),
            self.busy(),
            self.inrst(),
            self.comppend(),
            self.mastpend(),
            self.alertpend()
        )
    }
}
#[doc = "Port Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0addr(pub u32);
impl P0addr {
    #[doc = "Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn off(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Offset"]
    #[inline(always)]
    pub const fn set_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "The meaning is dependent on type of port:"]
    #[must_use]
    #[inline(always)]
    pub const fn base_asz(&self) -> super::vals::P0addrBaseAsz {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::P0addrBaseAsz::from_bits(val as u8)
    }
    #[doc = "The meaning is dependent on type of port:"]
    #[inline(always)]
    pub const fn set_base_asz(&mut self, val: super::vals::P0addrBaseAsz) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Index Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn idxoff(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Index Offset"]
    #[inline(always)]
    pub const fn set_idxoff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "For index-and-data register only:"]
    #[must_use]
    #[inline(always)]
    pub const fn idx1st(&self) -> super::vals::P0addrIdx1st {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::P0addrIdx1st::from_bits(val as u8)
    }
    #[doc = "For index-and-data register only:"]
    #[inline(always)]
    pub const fn set_idx1st(&mut self, val: super::vals::P0addrIdx1st) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
}
impl Default for P0addr {
    #[inline(always)]
    fn default() -> P0addr {
        P0addr(0)
    }
}
impl core::fmt::Debug for P0addr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0addr")
            .field("off", &self.off())
            .field("base_asz", &self.base_asz())
            .field("idxoff", &self.idxoff())
            .field("idx1st", &self.idx1st())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P0addr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P0addr {{ off: {=u16:?}, base_asz: {:?}, idxoff: {=u8:?}, idx1st: {:?} }}",
            self.off(),
            self.base_asz(),
            self.idxoff(),
            self.idx1st()
        )
    }
}
#[doc = "Port Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0cfg(pub u32);
impl P0cfg {
    #[doc = "Interaction Type between Port and Host"]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> super::vals::P0cfgType {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::P0cfgType::from_bits(val as u8)
    }
    #[doc = "Interaction Type between Port and Host"]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: super::vals::P0cfgType) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn direction(&self) -> super::vals::P0cfgDirection {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::P0cfgDirection::from_bits(val as u8)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_direction(&mut self, val: super::vals::P0cfgDirection) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Mailbox: map interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn mbintall(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Mailbox: map interrupt"]
    #[inline(always)]
    pub const fn set_mbintall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Stall on any read"]
    #[must_use]
    #[inline(always)]
    pub const fn stallrd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Stall on any read"]
    #[inline(always)]
    pub const fn set_stallrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Stall on write"]
    #[must_use]
    #[inline(always)]
    pub const fn stallwr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Stall on write"]
    #[inline(always)]
    pub const fn set_stallwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Error Origin"]
    #[must_use]
    #[inline(always)]
    pub const fn errorign(&self) -> super::vals::P0cfgErrorign {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::P0cfgErrorign::from_bits(val as u8)
    }
    #[doc = "Error Origin"]
    #[inline(always)]
    pub const fn set_errorign(&mut self, val: super::vals::P0cfgErrorign) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for P0cfg {
    #[inline(always)]
    fn default() -> P0cfg {
        P0cfg(0)
    }
}
impl core::fmt::Debug for P0cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0cfg")
            .field("type_", &self.type_())
            .field("direction", &self.direction())
            .field("mbintall", &self.mbintall())
            .field("stallrd", &self.stallrd())
            .field("stallwr", &self.stallwr())
            .field("errorign", &self.errorign())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P0cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P0cfg {{ type_: {:?}, direction: {:?}, mbintall: {=bool:?}, stallrd: {=bool:?}, stallwr: {=bool:?}, errorign: {:?} }}",
            self.type_(),
            self.direction(),
            self.mbintall(),
            self.stallrd(),
            self.stallwr(),
            self.errorign()
        )
    }
}
#[doc = "Port Data Input"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0datain(pub u32);
impl P0datain {
    #[doc = "Data Length"]
    #[must_use]
    #[inline(always)]
    pub const fn data_len(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Length"]
    #[inline(always)]
    pub const fn set_data_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Direction of last access"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> super::vals::P0datainDir {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::P0datainDir::from_bits(val as u8)
    }
    #[doc = "Direction of last access"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: super::vals::P0datainDir) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Index of Last Access"]
    #[must_use]
    #[inline(always)]
    pub const fn idx(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Index of Last Access"]
    #[inline(always)]
    pub const fn set_idx(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for P0datain {
    #[inline(always)]
    fn default() -> P0datain {
        P0datain(0)
    }
}
impl core::fmt::Debug for P0datain {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0datain")
            .field("data_len", &self.data_len())
            .field("dir", &self.dir())
            .field("idx", &self.idx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P0datain {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P0datain {{ data_len: {=u8:?}, dir: {:?}, idx: {=u16:?} }}",
            self.data_len(),
            self.dir(),
            self.idx()
        )
    }
}
#[doc = "Port Data Out"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0dataout(pub u32);
impl P0dataout {
    #[doc = "Data to Send to Host"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data to Send to Host"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for P0dataout {
    #[inline(always)]
    fn default() -> P0dataout {
        P0dataout(0)
    }
}
impl core::fmt::Debug for P0dataout {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0dataout")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P0dataout {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "P0dataout {{ data: {=u8:?} }}", self.data())
    }
}
#[doc = "Set Interrupt Rules and User Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0irulestat(pub u32);
impl P0irulestat {
    #[doc = "User-Defined Status Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn ustat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "User-Defined Status Bits"]
    #[inline(always)]
    pub const fn set_ustat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Interrupt If Error Detected"]
    #[must_use]
    #[inline(always)]
    pub const fn interr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt If Error Detected"]
    #[inline(always)]
    pub const fn set_interr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt If Read or First Read or Bus Master Started"]
    #[must_use]
    #[inline(always)]
    pub const fn intrd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt If Read or First Read or Bus Master Started"]
    #[inline(always)]
    pub const fn set_intrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt If Write or First Write or Bus Master Finished"]
    #[must_use]
    #[inline(always)]
    pub const fn intwr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt If Write or First Write or Bus Master Finished"]
    #[inline(always)]
    pub const fn set_intwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Select Interrupts for PnSTAT\\[INTSPC0\\] through PnSTAT\\[INSTSPC3\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Select Interrupts for PnSTAT\\[INTSPC0\\] through PnSTAT\\[INSTSPC3\\]"]
    #[inline(always)]
    pub const fn set_intspc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Status Set and Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn sstcl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Status Set and Clear"]
    #[inline(always)]
    pub const fn set_sstcl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Reset PnSTAT\\[RDSTAT\\] and PnSTAT\\[WRSTAT\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn srst(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Reset PnSTAT\\[RDSTAT\\] and PnSTAT\\[WRSTAT\\]"]
    #[inline(always)]
    pub const fn set_srst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Flash Completion Type"]
    #[must_use]
    #[inline(always)]
    pub const fn flash_completion_type(&self) -> super::vals::P0irulestatFlashCompletionType {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::P0irulestatFlashCompletionType::from_bits(val as u8)
    }
    #[doc = "Flash Completion Type"]
    #[inline(always)]
    pub const fn set_flash_completion_type(
        &mut self,
        val: super::vals::P0irulestatFlashCompletionType,
    ) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "CPU Tag"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu_tag(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x0f;
        val as u8
    }
    #[doc = "CPU Tag"]
    #[inline(always)]
    pub const fn set_cpu_tag(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val as u32) & 0x0f) << 23usize);
    }
}
impl Default for P0irulestat {
    #[inline(always)]
    fn default() -> P0irulestat {
        P0irulestat(0)
    }
}
impl core::fmt::Debug for P0irulestat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0irulestat")
            .field("ustat", &self.ustat())
            .field("interr", &self.interr())
            .field("intrd", &self.intrd())
            .field("intwr", &self.intwr())
            .field("intspc", &self.intspc())
            .field("sstcl", &self.sstcl())
            .field("srst", &self.srst())
            .field("flash_completion_type", &self.flash_completion_type())
            .field("cpu_tag", &self.cpu_tag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P0irulestat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P0irulestat {{ ustat: {=u8:?}, interr: {=bool:?}, intrd: {=bool:?}, intwr: {=bool:?}, intspc: {=u8:?}, sstcl: {=u8:?}, srst: {=bool:?}, flash_completion_type: {:?}, cpu_tag: {=u8:?} }}",
            self.ustat(),
            self.interr(),
            self.intrd(),
            self.intwr(),
            self.intspc(),
            self.sstcl(),
            self.srst(),
            self.flash_completion_type(),
            self.cpu_tag()
        )
    }
}
#[doc = "Port OOB, Mastering, and Flash Length"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0omflen(pub u32);
impl P0omflen {
    #[doc = "Length in Bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Length in Bytes"]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Transfer Request"]
    #[must_use]
    #[inline(always)]
    pub const fn trans(&self) -> super::vals::P0omflenTrans {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::P0omflenTrans::from_bits(val as u8)
    }
    #[doc = "Transfer Request"]
    #[inline(always)]
    pub const fn set_trans(&mut self, val: super::vals::P0omflenTrans) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for P0omflen {
    #[inline(always)]
    fn default() -> P0omflen {
        P0omflen(0)
    }
}
impl core::fmt::Debug for P0omflen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0omflen")
            .field("len", &self.len())
            .field("trans", &self.trans())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P0omflen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P0omflen {{ len: {=u8:?}, trans: {:?} }}",
            self.len(),
            self.trans()
        )
    }
}
#[doc = "Port RAM Use"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0ramuse(pub u32);
impl P0ramuse {
    #[doc = "Offset into RAM"]
    #[must_use]
    #[inline(always)]
    pub const fn off(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Offset into RAM"]
    #[inline(always)]
    pub const fn set_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Length"]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Length"]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for P0ramuse {
    #[inline(always)]
    fn default() -> P0ramuse {
        P0ramuse(0)
    }
}
impl core::fmt::Debug for P0ramuse {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0ramuse")
            .field("off", &self.off())
            .field("len", &self.len())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P0ramuse {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P0ramuse {{ off: {=u16:?}, len: {=u8:?} }}",
            self.off(),
            self.len()
        )
    }
}
#[doc = "Port Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0stat(pub u32);
impl P0stat {
    #[doc = "Host Read Data Status"]
    #[must_use]
    #[inline(always)]
    pub const fn rdstat(&self) -> super::vals::P0statRdstat {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::P0statRdstat::from_bits(val as u8)
    }
    #[doc = "Host Read Data Status"]
    #[inline(always)]
    pub const fn set_rdstat(&mut self, val: super::vals::P0statRdstat) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Status of Host Writes"]
    #[must_use]
    #[inline(always)]
    pub const fn wrstat(&self) -> super::vals::P0statWrstat {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::P0statWrstat::from_bits(val as u8)
    }
    #[doc = "Status of Host Writes"]
    #[inline(always)]
    pub const fn set_wrstat(&mut self, val: super::vals::P0statWrstat) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Interrupt Caused by Error"]
    #[must_use]
    #[inline(always)]
    pub const fn interr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Caused by Error"]
    #[inline(always)]
    pub const fn set_interr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt Caused by Read"]
    #[must_use]
    #[inline(always)]
    pub const fn intrd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Caused by Read"]
    #[inline(always)]
    pub const fn set_intrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt Caused by Write"]
    #[must_use]
    #[inline(always)]
    pub const fn intwr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Caused by Write"]
    #[inline(always)]
    pub const fn set_intwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SPC0 Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SPC0 Interrupt"]
    #[inline(always)]
    pub const fn set_intspc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "SPC1 Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SPC1 Interrupt"]
    #[inline(always)]
    pub const fn set_intspc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SPC2 Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc2(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SPC2 Interrupt"]
    #[inline(always)]
    pub const fn set_intspc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SPC3 Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn instspc3(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SPC3 Interrupt"]
    #[inline(always)]
    pub const fn set_instspc3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Error 0"]
    #[must_use]
    #[inline(always)]
    pub const fn err0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Error 0"]
    #[inline(always)]
    pub const fn set_err0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Error 1"]
    #[must_use]
    #[inline(always)]
    pub const fn err1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Error 1"]
    #[inline(always)]
    pub const fn set_err1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Error 2"]
    #[must_use]
    #[inline(always)]
    pub const fn err2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Error 2"]
    #[inline(always)]
    pub const fn set_err2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Error 3"]
    #[must_use]
    #[inline(always)]
    pub const fn err3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Error 3"]
    #[inline(always)]
    pub const fn set_err3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "RPMC 1 or 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rpmc_1_or_2(&self) -> super::vals::P0statRpmc1Or2 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::P0statRpmc1Or2::from_bits(val as u8)
    }
    #[doc = "RPMC 1 or 2"]
    #[inline(always)]
    pub const fn set_rpmc_1_or_2(&mut self, val: super::vals::P0statRpmc1Or2) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "RPMC Flash Device"]
    #[must_use]
    #[inline(always)]
    pub const fn rpmc_flash_device(&self) -> super::vals::P0statRpmcFlashDevice {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::P0statRpmcFlashDevice::from_bits(val as u8)
    }
    #[doc = "RPMC Flash Device"]
    #[inline(always)]
    pub const fn set_rpmc_flash_device(&mut self, val: super::vals::P0statRpmcFlashDevice) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
}
impl Default for P0stat {
    #[inline(always)]
    fn default() -> P0stat {
        P0stat(0)
    }
}
impl core::fmt::Debug for P0stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0stat")
            .field("rdstat", &self.rdstat())
            .field("wrstat", &self.wrstat())
            .field("interr", &self.interr())
            .field("intrd", &self.intrd())
            .field("intwr", &self.intwr())
            .field("intspc0", &self.intspc0())
            .field("intspc1", &self.intspc1())
            .field("intspc2", &self.intspc2())
            .field("instspc3", &self.instspc3())
            .field("err0", &self.err0())
            .field("err1", &self.err1())
            .field("err2", &self.err2())
            .field("err3", &self.err3())
            .field("rpmc_1_or_2", &self.rpmc_1_or_2())
            .field("rpmc_flash_device", &self.rpmc_flash_device())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P0stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P0stat {{ rdstat: {:?}, wrstat: {:?}, interr: {=bool:?}, intrd: {=bool:?}, intwr: {=bool:?}, intspc0: {=bool:?}, intspc1: {=bool:?}, intspc2: {=bool:?}, instspc3: {=bool:?}, err0: {=bool:?}, err1: {=bool:?}, err2: {=bool:?}, err3: {=bool:?}, rpmc_1_or_2: {:?}, rpmc_flash_device: {:?} }}",
            self.rdstat(),
            self.wrstat(),
            self.interr(),
            self.intrd(),
            self.intwr(),
            self.intspc0(),
            self.intspc1(),
            self.intspc2(),
            self.instspc3(),
            self.err0(),
            self.err1(),
            self.err2(),
            self.err3(),
            self.rpmc_1_or_2(),
            self.rpmc_flash_device()
        )
    }
}
#[doc = "Port Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1addr(pub u32);
impl P1addr {
    #[doc = "Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn off(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Offset"]
    #[inline(always)]
    pub const fn set_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "The meaning is dependent on type of port:"]
    #[must_use]
    #[inline(always)]
    pub const fn base_asz(&self) -> super::vals::P1addrBaseAsz {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::P1addrBaseAsz::from_bits(val as u8)
    }
    #[doc = "The meaning is dependent on type of port:"]
    #[inline(always)]
    pub const fn set_base_asz(&mut self, val: super::vals::P1addrBaseAsz) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Index Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn idxoff(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Index Offset"]
    #[inline(always)]
    pub const fn set_idxoff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "For index-and-data register only:"]
    #[must_use]
    #[inline(always)]
    pub const fn idx1st(&self) -> super::vals::P1addrIdx1st {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::P1addrIdx1st::from_bits(val as u8)
    }
    #[doc = "For index-and-data register only:"]
    #[inline(always)]
    pub const fn set_idx1st(&mut self, val: super::vals::P1addrIdx1st) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
}
impl Default for P1addr {
    #[inline(always)]
    fn default() -> P1addr {
        P1addr(0)
    }
}
impl core::fmt::Debug for P1addr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1addr")
            .field("off", &self.off())
            .field("base_asz", &self.base_asz())
            .field("idxoff", &self.idxoff())
            .field("idx1st", &self.idx1st())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P1addr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P1addr {{ off: {=u16:?}, base_asz: {:?}, idxoff: {=u8:?}, idx1st: {:?} }}",
            self.off(),
            self.base_asz(),
            self.idxoff(),
            self.idx1st()
        )
    }
}
#[doc = "Port Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1cfg(pub u32);
impl P1cfg {
    #[doc = "Interaction Type between Port and Host"]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> super::vals::P1cfgType {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::P1cfgType::from_bits(val as u8)
    }
    #[doc = "Interaction Type between Port and Host"]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: super::vals::P1cfgType) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn direction(&self) -> super::vals::P1cfgDirection {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::P1cfgDirection::from_bits(val as u8)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_direction(&mut self, val: super::vals::P1cfgDirection) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Mailbox: map interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn mbintall(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Mailbox: map interrupt"]
    #[inline(always)]
    pub const fn set_mbintall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Stall on any read"]
    #[must_use]
    #[inline(always)]
    pub const fn stallrd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Stall on any read"]
    #[inline(always)]
    pub const fn set_stallrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Stall on write"]
    #[must_use]
    #[inline(always)]
    pub const fn stallwr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Stall on write"]
    #[inline(always)]
    pub const fn set_stallwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Error Origin"]
    #[must_use]
    #[inline(always)]
    pub const fn errorign(&self) -> super::vals::P1cfgErrorign {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::P1cfgErrorign::from_bits(val as u8)
    }
    #[doc = "Error Origin"]
    #[inline(always)]
    pub const fn set_errorign(&mut self, val: super::vals::P1cfgErrorign) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for P1cfg {
    #[inline(always)]
    fn default() -> P1cfg {
        P1cfg(0)
    }
}
impl core::fmt::Debug for P1cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1cfg")
            .field("type_", &self.type_())
            .field("direction", &self.direction())
            .field("mbintall", &self.mbintall())
            .field("stallrd", &self.stallrd())
            .field("stallwr", &self.stallwr())
            .field("errorign", &self.errorign())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P1cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P1cfg {{ type_: {:?}, direction: {:?}, mbintall: {=bool:?}, stallrd: {=bool:?}, stallwr: {=bool:?}, errorign: {:?} }}",
            self.type_(),
            self.direction(),
            self.mbintall(),
            self.stallrd(),
            self.stallwr(),
            self.errorign()
        )
    }
}
#[doc = "Port Data Input"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1datain(pub u32);
impl P1datain {
    #[doc = "Data Length"]
    #[must_use]
    #[inline(always)]
    pub const fn data_len(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Length"]
    #[inline(always)]
    pub const fn set_data_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Direction of last access"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> super::vals::P1datainDir {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::P1datainDir::from_bits(val as u8)
    }
    #[doc = "Direction of last access"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: super::vals::P1datainDir) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Index of Last Access"]
    #[must_use]
    #[inline(always)]
    pub const fn idx(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Index of Last Access"]
    #[inline(always)]
    pub const fn set_idx(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for P1datain {
    #[inline(always)]
    fn default() -> P1datain {
        P1datain(0)
    }
}
impl core::fmt::Debug for P1datain {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1datain")
            .field("data_len", &self.data_len())
            .field("dir", &self.dir())
            .field("idx", &self.idx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P1datain {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P1datain {{ data_len: {=u8:?}, dir: {:?}, idx: {=u16:?} }}",
            self.data_len(),
            self.dir(),
            self.idx()
        )
    }
}
#[doc = "Port Data Out"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1dataout(pub u32);
impl P1dataout {
    #[doc = "Data to Send to Host"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data to Send to Host"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for P1dataout {
    #[inline(always)]
    fn default() -> P1dataout {
        P1dataout(0)
    }
}
impl core::fmt::Debug for P1dataout {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1dataout")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P1dataout {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "P1dataout {{ data: {=u8:?} }}", self.data())
    }
}
#[doc = "Set Interrupt Rules and User Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1irulestat(pub u32);
impl P1irulestat {
    #[doc = "User-Defined Status Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn ustat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "User-Defined Status Bits"]
    #[inline(always)]
    pub const fn set_ustat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Interrupt If Error Detected"]
    #[must_use]
    #[inline(always)]
    pub const fn interr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt If Error Detected"]
    #[inline(always)]
    pub const fn set_interr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt If Read or First Read or Bus Master Started"]
    #[must_use]
    #[inline(always)]
    pub const fn intrd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt If Read or First Read or Bus Master Started"]
    #[inline(always)]
    pub const fn set_intrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt If Write or First Write or Bus Master Finished"]
    #[must_use]
    #[inline(always)]
    pub const fn intwr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt If Write or First Write or Bus Master Finished"]
    #[inline(always)]
    pub const fn set_intwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Select Interrupts for PnSTAT\\[INTSPC0\\] through PnSTAT\\[INSTSPC3\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Select Interrupts for PnSTAT\\[INTSPC0\\] through PnSTAT\\[INSTSPC3\\]"]
    #[inline(always)]
    pub const fn set_intspc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Status Set and Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn sstcl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Status Set and Clear"]
    #[inline(always)]
    pub const fn set_sstcl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Reset PnSTAT\\[RDSTAT\\] and PnSTAT\\[WRSTAT\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn srst(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Reset PnSTAT\\[RDSTAT\\] and PnSTAT\\[WRSTAT\\]"]
    #[inline(always)]
    pub const fn set_srst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Flash Completion Type"]
    #[must_use]
    #[inline(always)]
    pub const fn flash_completion_type(&self) -> super::vals::P1irulestatFlashCompletionType {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::P1irulestatFlashCompletionType::from_bits(val as u8)
    }
    #[doc = "Flash Completion Type"]
    #[inline(always)]
    pub const fn set_flash_completion_type(
        &mut self,
        val: super::vals::P1irulestatFlashCompletionType,
    ) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "CPU Tag"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu_tag(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x0f;
        val as u8
    }
    #[doc = "CPU Tag"]
    #[inline(always)]
    pub const fn set_cpu_tag(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val as u32) & 0x0f) << 23usize);
    }
}
impl Default for P1irulestat {
    #[inline(always)]
    fn default() -> P1irulestat {
        P1irulestat(0)
    }
}
impl core::fmt::Debug for P1irulestat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1irulestat")
            .field("ustat", &self.ustat())
            .field("interr", &self.interr())
            .field("intrd", &self.intrd())
            .field("intwr", &self.intwr())
            .field("intspc", &self.intspc())
            .field("sstcl", &self.sstcl())
            .field("srst", &self.srst())
            .field("flash_completion_type", &self.flash_completion_type())
            .field("cpu_tag", &self.cpu_tag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P1irulestat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P1irulestat {{ ustat: {=u8:?}, interr: {=bool:?}, intrd: {=bool:?}, intwr: {=bool:?}, intspc: {=u8:?}, sstcl: {=u8:?}, srst: {=bool:?}, flash_completion_type: {:?}, cpu_tag: {=u8:?} }}",
            self.ustat(),
            self.interr(),
            self.intrd(),
            self.intwr(),
            self.intspc(),
            self.sstcl(),
            self.srst(),
            self.flash_completion_type(),
            self.cpu_tag()
        )
    }
}
#[doc = "Port OOB, Mastering, and Flash Length"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1omflen(pub u32);
impl P1omflen {
    #[doc = "Length in Bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Length in Bytes"]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Transfer Request"]
    #[must_use]
    #[inline(always)]
    pub const fn trans(&self) -> super::vals::P1omflenTrans {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::P1omflenTrans::from_bits(val as u8)
    }
    #[doc = "Transfer Request"]
    #[inline(always)]
    pub const fn set_trans(&mut self, val: super::vals::P1omflenTrans) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for P1omflen {
    #[inline(always)]
    fn default() -> P1omflen {
        P1omflen(0)
    }
}
impl core::fmt::Debug for P1omflen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1omflen")
            .field("len", &self.len())
            .field("trans", &self.trans())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P1omflen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P1omflen {{ len: {=u8:?}, trans: {:?} }}",
            self.len(),
            self.trans()
        )
    }
}
#[doc = "Port RAM Use"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1ramuse(pub u32);
impl P1ramuse {
    #[doc = "Offset into RAM"]
    #[must_use]
    #[inline(always)]
    pub const fn off(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Offset into RAM"]
    #[inline(always)]
    pub const fn set_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Length"]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Length"]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for P1ramuse {
    #[inline(always)]
    fn default() -> P1ramuse {
        P1ramuse(0)
    }
}
impl core::fmt::Debug for P1ramuse {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1ramuse")
            .field("off", &self.off())
            .field("len", &self.len())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P1ramuse {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P1ramuse {{ off: {=u16:?}, len: {=u8:?} }}",
            self.off(),
            self.len()
        )
    }
}
#[doc = "Port Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1stat(pub u32);
impl P1stat {
    #[doc = "Host Read Data Status"]
    #[must_use]
    #[inline(always)]
    pub const fn rdstat(&self) -> super::vals::P1statRdstat {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::P1statRdstat::from_bits(val as u8)
    }
    #[doc = "Host Read Data Status"]
    #[inline(always)]
    pub const fn set_rdstat(&mut self, val: super::vals::P1statRdstat) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Status of Host Writes"]
    #[must_use]
    #[inline(always)]
    pub const fn wrstat(&self) -> super::vals::P1statWrstat {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::P1statWrstat::from_bits(val as u8)
    }
    #[doc = "Status of Host Writes"]
    #[inline(always)]
    pub const fn set_wrstat(&mut self, val: super::vals::P1statWrstat) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Interrupt Caused by Error"]
    #[must_use]
    #[inline(always)]
    pub const fn interr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Caused by Error"]
    #[inline(always)]
    pub const fn set_interr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt Caused by Read"]
    #[must_use]
    #[inline(always)]
    pub const fn intrd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Caused by Read"]
    #[inline(always)]
    pub const fn set_intrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt Caused by Write"]
    #[must_use]
    #[inline(always)]
    pub const fn intwr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Caused by Write"]
    #[inline(always)]
    pub const fn set_intwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SPC0 Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SPC0 Interrupt"]
    #[inline(always)]
    pub const fn set_intspc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "SPC1 Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SPC1 Interrupt"]
    #[inline(always)]
    pub const fn set_intspc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SPC2 Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc2(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SPC2 Interrupt"]
    #[inline(always)]
    pub const fn set_intspc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SPC3 Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn instspc3(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SPC3 Interrupt"]
    #[inline(always)]
    pub const fn set_instspc3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Error 0"]
    #[must_use]
    #[inline(always)]
    pub const fn err0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Error 0"]
    #[inline(always)]
    pub const fn set_err0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Error 1"]
    #[must_use]
    #[inline(always)]
    pub const fn err1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Error 1"]
    #[inline(always)]
    pub const fn set_err1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Error 2"]
    #[must_use]
    #[inline(always)]
    pub const fn err2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Error 2"]
    #[inline(always)]
    pub const fn set_err2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Error 3"]
    #[must_use]
    #[inline(always)]
    pub const fn err3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Error 3"]
    #[inline(always)]
    pub const fn set_err3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "RPMC 1 or 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rpmc_1_or_2(&self) -> super::vals::P1statRpmc1Or2 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::P1statRpmc1Or2::from_bits(val as u8)
    }
    #[doc = "RPMC 1 or 2"]
    #[inline(always)]
    pub const fn set_rpmc_1_or_2(&mut self, val: super::vals::P1statRpmc1Or2) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "RPMC Flash Device"]
    #[must_use]
    #[inline(always)]
    pub const fn rpmc_flash_device(&self) -> super::vals::P1statRpmcFlashDevice {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::P1statRpmcFlashDevice::from_bits(val as u8)
    }
    #[doc = "RPMC Flash Device"]
    #[inline(always)]
    pub const fn set_rpmc_flash_device(&mut self, val: super::vals::P1statRpmcFlashDevice) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
}
impl Default for P1stat {
    #[inline(always)]
    fn default() -> P1stat {
        P1stat(0)
    }
}
impl core::fmt::Debug for P1stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1stat")
            .field("rdstat", &self.rdstat())
            .field("wrstat", &self.wrstat())
            .field("interr", &self.interr())
            .field("intrd", &self.intrd())
            .field("intwr", &self.intwr())
            .field("intspc0", &self.intspc0())
            .field("intspc1", &self.intspc1())
            .field("intspc2", &self.intspc2())
            .field("instspc3", &self.instspc3())
            .field("err0", &self.err0())
            .field("err1", &self.err1())
            .field("err2", &self.err2())
            .field("err3", &self.err3())
            .field("rpmc_1_or_2", &self.rpmc_1_or_2())
            .field("rpmc_flash_device", &self.rpmc_flash_device())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P1stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P1stat {{ rdstat: {:?}, wrstat: {:?}, interr: {=bool:?}, intrd: {=bool:?}, intwr: {=bool:?}, intspc0: {=bool:?}, intspc1: {=bool:?}, intspc2: {=bool:?}, instspc3: {=bool:?}, err0: {=bool:?}, err1: {=bool:?}, err2: {=bool:?}, err3: {=bool:?}, rpmc_1_or_2: {:?}, rpmc_flash_device: {:?} }}",
            self.rdstat(),
            self.wrstat(),
            self.interr(),
            self.intrd(),
            self.intwr(),
            self.intspc0(),
            self.intspc1(),
            self.intspc2(),
            self.instspc3(),
            self.err0(),
            self.err1(),
            self.err2(),
            self.err3(),
            self.rpmc_1_or_2(),
            self.rpmc_flash_device()
        )
    }
}
#[doc = "Port Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2addr(pub u32);
impl P2addr {
    #[doc = "Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn off(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Offset"]
    #[inline(always)]
    pub const fn set_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "The meaning is dependent on type of port:"]
    #[must_use]
    #[inline(always)]
    pub const fn base_asz(&self) -> super::vals::P2addrBaseAsz {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::P2addrBaseAsz::from_bits(val as u8)
    }
    #[doc = "The meaning is dependent on type of port:"]
    #[inline(always)]
    pub const fn set_base_asz(&mut self, val: super::vals::P2addrBaseAsz) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Index Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn idxoff(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Index Offset"]
    #[inline(always)]
    pub const fn set_idxoff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "For index-and-data register only:"]
    #[must_use]
    #[inline(always)]
    pub const fn idx1st(&self) -> super::vals::P2addrIdx1st {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::P2addrIdx1st::from_bits(val as u8)
    }
    #[doc = "For index-and-data register only:"]
    #[inline(always)]
    pub const fn set_idx1st(&mut self, val: super::vals::P2addrIdx1st) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
}
impl Default for P2addr {
    #[inline(always)]
    fn default() -> P2addr {
        P2addr(0)
    }
}
impl core::fmt::Debug for P2addr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2addr")
            .field("off", &self.off())
            .field("base_asz", &self.base_asz())
            .field("idxoff", &self.idxoff())
            .field("idx1st", &self.idx1st())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P2addr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P2addr {{ off: {=u16:?}, base_asz: {:?}, idxoff: {=u8:?}, idx1st: {:?} }}",
            self.off(),
            self.base_asz(),
            self.idxoff(),
            self.idx1st()
        )
    }
}
#[doc = "Port Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2cfg(pub u32);
impl P2cfg {
    #[doc = "Interaction Type between Port and Host"]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> super::vals::P2cfgType {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::P2cfgType::from_bits(val as u8)
    }
    #[doc = "Interaction Type between Port and Host"]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: super::vals::P2cfgType) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn direction(&self) -> super::vals::P2cfgDirection {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::P2cfgDirection::from_bits(val as u8)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_direction(&mut self, val: super::vals::P2cfgDirection) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Mailbox: map interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn mbintall(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Mailbox: map interrupt"]
    #[inline(always)]
    pub const fn set_mbintall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Stall on any read"]
    #[must_use]
    #[inline(always)]
    pub const fn stallrd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Stall on any read"]
    #[inline(always)]
    pub const fn set_stallrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Stall on write"]
    #[must_use]
    #[inline(always)]
    pub const fn stallwr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Stall on write"]
    #[inline(always)]
    pub const fn set_stallwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Error Origin"]
    #[must_use]
    #[inline(always)]
    pub const fn errorign(&self) -> super::vals::P2cfgErrorign {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::P2cfgErrorign::from_bits(val as u8)
    }
    #[doc = "Error Origin"]
    #[inline(always)]
    pub const fn set_errorign(&mut self, val: super::vals::P2cfgErrorign) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for P2cfg {
    #[inline(always)]
    fn default() -> P2cfg {
        P2cfg(0)
    }
}
impl core::fmt::Debug for P2cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2cfg")
            .field("type_", &self.type_())
            .field("direction", &self.direction())
            .field("mbintall", &self.mbintall())
            .field("stallrd", &self.stallrd())
            .field("stallwr", &self.stallwr())
            .field("errorign", &self.errorign())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P2cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P2cfg {{ type_: {:?}, direction: {:?}, mbintall: {=bool:?}, stallrd: {=bool:?}, stallwr: {=bool:?}, errorign: {:?} }}",
            self.type_(),
            self.direction(),
            self.mbintall(),
            self.stallrd(),
            self.stallwr(),
            self.errorign()
        )
    }
}
#[doc = "Port Data Input"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2datain(pub u32);
impl P2datain {
    #[doc = "Data Length"]
    #[must_use]
    #[inline(always)]
    pub const fn data_len(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Length"]
    #[inline(always)]
    pub const fn set_data_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Direction of last access"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> super::vals::P2datainDir {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::P2datainDir::from_bits(val as u8)
    }
    #[doc = "Direction of last access"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: super::vals::P2datainDir) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Index of Last Access"]
    #[must_use]
    #[inline(always)]
    pub const fn idx(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Index of Last Access"]
    #[inline(always)]
    pub const fn set_idx(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for P2datain {
    #[inline(always)]
    fn default() -> P2datain {
        P2datain(0)
    }
}
impl core::fmt::Debug for P2datain {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2datain")
            .field("data_len", &self.data_len())
            .field("dir", &self.dir())
            .field("idx", &self.idx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P2datain {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P2datain {{ data_len: {=u8:?}, dir: {:?}, idx: {=u16:?} }}",
            self.data_len(),
            self.dir(),
            self.idx()
        )
    }
}
#[doc = "Port Data Out"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2dataout(pub u32);
impl P2dataout {
    #[doc = "Data to Send to Host"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data to Send to Host"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for P2dataout {
    #[inline(always)]
    fn default() -> P2dataout {
        P2dataout(0)
    }
}
impl core::fmt::Debug for P2dataout {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2dataout")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P2dataout {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "P2dataout {{ data: {=u8:?} }}", self.data())
    }
}
#[doc = "Set Interrupt Rules and User Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2irulestat(pub u32);
impl P2irulestat {
    #[doc = "User-Defined Status Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn ustat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "User-Defined Status Bits"]
    #[inline(always)]
    pub const fn set_ustat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Interrupt If Error Detected"]
    #[must_use]
    #[inline(always)]
    pub const fn interr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt If Error Detected"]
    #[inline(always)]
    pub const fn set_interr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt If Read or First Read or Bus Master Started"]
    #[must_use]
    #[inline(always)]
    pub const fn intrd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt If Read or First Read or Bus Master Started"]
    #[inline(always)]
    pub const fn set_intrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt If Write or First Write or Bus Master Finished"]
    #[must_use]
    #[inline(always)]
    pub const fn intwr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt If Write or First Write or Bus Master Finished"]
    #[inline(always)]
    pub const fn set_intwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Select Interrupts for PnSTAT\\[INTSPC0\\] through PnSTAT\\[INSTSPC3\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Select Interrupts for PnSTAT\\[INTSPC0\\] through PnSTAT\\[INSTSPC3\\]"]
    #[inline(always)]
    pub const fn set_intspc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Status Set and Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn sstcl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Status Set and Clear"]
    #[inline(always)]
    pub const fn set_sstcl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Reset PnSTAT\\[RDSTAT\\] and PnSTAT\\[WRSTAT\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn srst(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Reset PnSTAT\\[RDSTAT\\] and PnSTAT\\[WRSTAT\\]"]
    #[inline(always)]
    pub const fn set_srst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Flash Completion Type"]
    #[must_use]
    #[inline(always)]
    pub const fn flash_completion_type(&self) -> super::vals::P2irulestatFlashCompletionType {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::P2irulestatFlashCompletionType::from_bits(val as u8)
    }
    #[doc = "Flash Completion Type"]
    #[inline(always)]
    pub const fn set_flash_completion_type(
        &mut self,
        val: super::vals::P2irulestatFlashCompletionType,
    ) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "CPU Tag"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu_tag(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x0f;
        val as u8
    }
    #[doc = "CPU Tag"]
    #[inline(always)]
    pub const fn set_cpu_tag(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val as u32) & 0x0f) << 23usize);
    }
}
impl Default for P2irulestat {
    #[inline(always)]
    fn default() -> P2irulestat {
        P2irulestat(0)
    }
}
impl core::fmt::Debug for P2irulestat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2irulestat")
            .field("ustat", &self.ustat())
            .field("interr", &self.interr())
            .field("intrd", &self.intrd())
            .field("intwr", &self.intwr())
            .field("intspc", &self.intspc())
            .field("sstcl", &self.sstcl())
            .field("srst", &self.srst())
            .field("flash_completion_type", &self.flash_completion_type())
            .field("cpu_tag", &self.cpu_tag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P2irulestat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P2irulestat {{ ustat: {=u8:?}, interr: {=bool:?}, intrd: {=bool:?}, intwr: {=bool:?}, intspc: {=u8:?}, sstcl: {=u8:?}, srst: {=bool:?}, flash_completion_type: {:?}, cpu_tag: {=u8:?} }}",
            self.ustat(),
            self.interr(),
            self.intrd(),
            self.intwr(),
            self.intspc(),
            self.sstcl(),
            self.srst(),
            self.flash_completion_type(),
            self.cpu_tag()
        )
    }
}
#[doc = "Port OOB, Mastering, and Flash Length"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2omflen(pub u32);
impl P2omflen {
    #[doc = "Length in Bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Length in Bytes"]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Transfer Request"]
    #[must_use]
    #[inline(always)]
    pub const fn trans(&self) -> super::vals::P2omflenTrans {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::P2omflenTrans::from_bits(val as u8)
    }
    #[doc = "Transfer Request"]
    #[inline(always)]
    pub const fn set_trans(&mut self, val: super::vals::P2omflenTrans) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for P2omflen {
    #[inline(always)]
    fn default() -> P2omflen {
        P2omflen(0)
    }
}
impl core::fmt::Debug for P2omflen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2omflen")
            .field("len", &self.len())
            .field("trans", &self.trans())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P2omflen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P2omflen {{ len: {=u8:?}, trans: {:?} }}",
            self.len(),
            self.trans()
        )
    }
}
#[doc = "Port RAM Use"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2ramuse(pub u32);
impl P2ramuse {
    #[doc = "Offset into RAM"]
    #[must_use]
    #[inline(always)]
    pub const fn off(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Offset into RAM"]
    #[inline(always)]
    pub const fn set_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Length"]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Length"]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for P2ramuse {
    #[inline(always)]
    fn default() -> P2ramuse {
        P2ramuse(0)
    }
}
impl core::fmt::Debug for P2ramuse {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2ramuse")
            .field("off", &self.off())
            .field("len", &self.len())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P2ramuse {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P2ramuse {{ off: {=u16:?}, len: {=u8:?} }}",
            self.off(),
            self.len()
        )
    }
}
#[doc = "Port Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2stat(pub u32);
impl P2stat {
    #[doc = "Host Read Data Status"]
    #[must_use]
    #[inline(always)]
    pub const fn rdstat(&self) -> super::vals::P2statRdstat {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::P2statRdstat::from_bits(val as u8)
    }
    #[doc = "Host Read Data Status"]
    #[inline(always)]
    pub const fn set_rdstat(&mut self, val: super::vals::P2statRdstat) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Status of Host Writes"]
    #[must_use]
    #[inline(always)]
    pub const fn wrstat(&self) -> super::vals::P2statWrstat {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::P2statWrstat::from_bits(val as u8)
    }
    #[doc = "Status of Host Writes"]
    #[inline(always)]
    pub const fn set_wrstat(&mut self, val: super::vals::P2statWrstat) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Interrupt Caused by Error"]
    #[must_use]
    #[inline(always)]
    pub const fn interr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Caused by Error"]
    #[inline(always)]
    pub const fn set_interr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt Caused by Read"]
    #[must_use]
    #[inline(always)]
    pub const fn intrd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Caused by Read"]
    #[inline(always)]
    pub const fn set_intrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt Caused by Write"]
    #[must_use]
    #[inline(always)]
    pub const fn intwr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Caused by Write"]
    #[inline(always)]
    pub const fn set_intwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SPC0 Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SPC0 Interrupt"]
    #[inline(always)]
    pub const fn set_intspc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "SPC1 Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SPC1 Interrupt"]
    #[inline(always)]
    pub const fn set_intspc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SPC2 Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc2(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SPC2 Interrupt"]
    #[inline(always)]
    pub const fn set_intspc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SPC3 Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn instspc3(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SPC3 Interrupt"]
    #[inline(always)]
    pub const fn set_instspc3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Error 0"]
    #[must_use]
    #[inline(always)]
    pub const fn err0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Error 0"]
    #[inline(always)]
    pub const fn set_err0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Error 1"]
    #[must_use]
    #[inline(always)]
    pub const fn err1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Error 1"]
    #[inline(always)]
    pub const fn set_err1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Error 2"]
    #[must_use]
    #[inline(always)]
    pub const fn err2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Error 2"]
    #[inline(always)]
    pub const fn set_err2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Error 3"]
    #[must_use]
    #[inline(always)]
    pub const fn err3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Error 3"]
    #[inline(always)]
    pub const fn set_err3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "RPMC 1 or 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rpmc_1_or_2(&self) -> super::vals::P2statRpmc1Or2 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::P2statRpmc1Or2::from_bits(val as u8)
    }
    #[doc = "RPMC 1 or 2"]
    #[inline(always)]
    pub const fn set_rpmc_1_or_2(&mut self, val: super::vals::P2statRpmc1Or2) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "RPMC Flash Device"]
    #[must_use]
    #[inline(always)]
    pub const fn rpmc_flash_device(&self) -> super::vals::P2statRpmcFlashDevice {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::P2statRpmcFlashDevice::from_bits(val as u8)
    }
    #[doc = "RPMC Flash Device"]
    #[inline(always)]
    pub const fn set_rpmc_flash_device(&mut self, val: super::vals::P2statRpmcFlashDevice) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
}
impl Default for P2stat {
    #[inline(always)]
    fn default() -> P2stat {
        P2stat(0)
    }
}
impl core::fmt::Debug for P2stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2stat")
            .field("rdstat", &self.rdstat())
            .field("wrstat", &self.wrstat())
            .field("interr", &self.interr())
            .field("intrd", &self.intrd())
            .field("intwr", &self.intwr())
            .field("intspc0", &self.intspc0())
            .field("intspc1", &self.intspc1())
            .field("intspc2", &self.intspc2())
            .field("instspc3", &self.instspc3())
            .field("err0", &self.err0())
            .field("err1", &self.err1())
            .field("err2", &self.err2())
            .field("err3", &self.err3())
            .field("rpmc_1_or_2", &self.rpmc_1_or_2())
            .field("rpmc_flash_device", &self.rpmc_flash_device())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P2stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P2stat {{ rdstat: {:?}, wrstat: {:?}, interr: {=bool:?}, intrd: {=bool:?}, intwr: {=bool:?}, intspc0: {=bool:?}, intspc1: {=bool:?}, intspc2: {=bool:?}, instspc3: {=bool:?}, err0: {=bool:?}, err1: {=bool:?}, err2: {=bool:?}, err3: {=bool:?}, rpmc_1_or_2: {:?}, rpmc_flash_device: {:?} }}",
            self.rdstat(),
            self.wrstat(),
            self.interr(),
            self.intrd(),
            self.intwr(),
            self.intspc0(),
            self.intspc1(),
            self.intspc2(),
            self.instspc3(),
            self.err0(),
            self.err1(),
            self.err2(),
            self.err3(),
            self.rpmc_1_or_2(),
            self.rpmc_flash_device()
        )
    }
}
#[doc = "Port Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3addr(pub u32);
impl P3addr {
    #[doc = "Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn off(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Offset"]
    #[inline(always)]
    pub const fn set_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "The meaning is dependent on type of port:"]
    #[must_use]
    #[inline(always)]
    pub const fn base_asz(&self) -> super::vals::P3addrBaseAsz {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::P3addrBaseAsz::from_bits(val as u8)
    }
    #[doc = "The meaning is dependent on type of port:"]
    #[inline(always)]
    pub const fn set_base_asz(&mut self, val: super::vals::P3addrBaseAsz) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Index Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn idxoff(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Index Offset"]
    #[inline(always)]
    pub const fn set_idxoff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "For index-and-data register only:"]
    #[must_use]
    #[inline(always)]
    pub const fn idx1st(&self) -> super::vals::P3addrIdx1st {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::P3addrIdx1st::from_bits(val as u8)
    }
    #[doc = "For index-and-data register only:"]
    #[inline(always)]
    pub const fn set_idx1st(&mut self, val: super::vals::P3addrIdx1st) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
}
impl Default for P3addr {
    #[inline(always)]
    fn default() -> P3addr {
        P3addr(0)
    }
}
impl core::fmt::Debug for P3addr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P3addr")
            .field("off", &self.off())
            .field("base_asz", &self.base_asz())
            .field("idxoff", &self.idxoff())
            .field("idx1st", &self.idx1st())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P3addr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P3addr {{ off: {=u16:?}, base_asz: {:?}, idxoff: {=u8:?}, idx1st: {:?} }}",
            self.off(),
            self.base_asz(),
            self.idxoff(),
            self.idx1st()
        )
    }
}
#[doc = "Port Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3cfg(pub u32);
impl P3cfg {
    #[doc = "Interaction Type between Port and Host"]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> super::vals::P3cfgType {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::P3cfgType::from_bits(val as u8)
    }
    #[doc = "Interaction Type between Port and Host"]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: super::vals::P3cfgType) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn direction(&self) -> super::vals::P3cfgDirection {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::P3cfgDirection::from_bits(val as u8)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_direction(&mut self, val: super::vals::P3cfgDirection) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Mailbox: map interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn mbintall(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Mailbox: map interrupt"]
    #[inline(always)]
    pub const fn set_mbintall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Stall on any read"]
    #[must_use]
    #[inline(always)]
    pub const fn stallrd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Stall on any read"]
    #[inline(always)]
    pub const fn set_stallrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Stall on write"]
    #[must_use]
    #[inline(always)]
    pub const fn stallwr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Stall on write"]
    #[inline(always)]
    pub const fn set_stallwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Error Origin"]
    #[must_use]
    #[inline(always)]
    pub const fn errorign(&self) -> super::vals::P3cfgErrorign {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::P3cfgErrorign::from_bits(val as u8)
    }
    #[doc = "Error Origin"]
    #[inline(always)]
    pub const fn set_errorign(&mut self, val: super::vals::P3cfgErrorign) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for P3cfg {
    #[inline(always)]
    fn default() -> P3cfg {
        P3cfg(0)
    }
}
impl core::fmt::Debug for P3cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P3cfg")
            .field("type_", &self.type_())
            .field("direction", &self.direction())
            .field("mbintall", &self.mbintall())
            .field("stallrd", &self.stallrd())
            .field("stallwr", &self.stallwr())
            .field("errorign", &self.errorign())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P3cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P3cfg {{ type_: {:?}, direction: {:?}, mbintall: {=bool:?}, stallrd: {=bool:?}, stallwr: {=bool:?}, errorign: {:?} }}",
            self.type_(),
            self.direction(),
            self.mbintall(),
            self.stallrd(),
            self.stallwr(),
            self.errorign()
        )
    }
}
#[doc = "Port Data Input"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3datain(pub u32);
impl P3datain {
    #[doc = "Data Length"]
    #[must_use]
    #[inline(always)]
    pub const fn data_len(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Length"]
    #[inline(always)]
    pub const fn set_data_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Direction of last access"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> super::vals::P3datainDir {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::P3datainDir::from_bits(val as u8)
    }
    #[doc = "Direction of last access"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: super::vals::P3datainDir) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Index of Last Access"]
    #[must_use]
    #[inline(always)]
    pub const fn idx(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Index of Last Access"]
    #[inline(always)]
    pub const fn set_idx(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for P3datain {
    #[inline(always)]
    fn default() -> P3datain {
        P3datain(0)
    }
}
impl core::fmt::Debug for P3datain {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P3datain")
            .field("data_len", &self.data_len())
            .field("dir", &self.dir())
            .field("idx", &self.idx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P3datain {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P3datain {{ data_len: {=u8:?}, dir: {:?}, idx: {=u16:?} }}",
            self.data_len(),
            self.dir(),
            self.idx()
        )
    }
}
#[doc = "Port Data Out"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3dataout(pub u32);
impl P3dataout {
    #[doc = "Data to Send to Host"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data to Send to Host"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for P3dataout {
    #[inline(always)]
    fn default() -> P3dataout {
        P3dataout(0)
    }
}
impl core::fmt::Debug for P3dataout {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P3dataout")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P3dataout {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "P3dataout {{ data: {=u8:?} }}", self.data())
    }
}
#[doc = "Set Interrupt Rules and User Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3irulestat(pub u32);
impl P3irulestat {
    #[doc = "User-Defined Status Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn ustat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "User-Defined Status Bits"]
    #[inline(always)]
    pub const fn set_ustat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Interrupt If Error Detected"]
    #[must_use]
    #[inline(always)]
    pub const fn interr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt If Error Detected"]
    #[inline(always)]
    pub const fn set_interr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt If Read or First Read or Bus Master Started"]
    #[must_use]
    #[inline(always)]
    pub const fn intrd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt If Read or First Read or Bus Master Started"]
    #[inline(always)]
    pub const fn set_intrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt If Write or First Write or Bus Master Finished"]
    #[must_use]
    #[inline(always)]
    pub const fn intwr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt If Write or First Write or Bus Master Finished"]
    #[inline(always)]
    pub const fn set_intwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Select Interrupts for PnSTAT\\[INTSPC0\\] through PnSTAT\\[INSTSPC3\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Select Interrupts for PnSTAT\\[INTSPC0\\] through PnSTAT\\[INSTSPC3\\]"]
    #[inline(always)]
    pub const fn set_intspc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Status Set and Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn sstcl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Status Set and Clear"]
    #[inline(always)]
    pub const fn set_sstcl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Reset PnSTAT\\[RDSTAT\\] and PnSTAT\\[WRSTAT\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn srst(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Reset PnSTAT\\[RDSTAT\\] and PnSTAT\\[WRSTAT\\]"]
    #[inline(always)]
    pub const fn set_srst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Flash Completion Type"]
    #[must_use]
    #[inline(always)]
    pub const fn flash_completion_type(&self) -> super::vals::P3irulestatFlashCompletionType {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::P3irulestatFlashCompletionType::from_bits(val as u8)
    }
    #[doc = "Flash Completion Type"]
    #[inline(always)]
    pub const fn set_flash_completion_type(
        &mut self,
        val: super::vals::P3irulestatFlashCompletionType,
    ) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "CPU Tag"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu_tag(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x0f;
        val as u8
    }
    #[doc = "CPU Tag"]
    #[inline(always)]
    pub const fn set_cpu_tag(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val as u32) & 0x0f) << 23usize);
    }
}
impl Default for P3irulestat {
    #[inline(always)]
    fn default() -> P3irulestat {
        P3irulestat(0)
    }
}
impl core::fmt::Debug for P3irulestat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P3irulestat")
            .field("ustat", &self.ustat())
            .field("interr", &self.interr())
            .field("intrd", &self.intrd())
            .field("intwr", &self.intwr())
            .field("intspc", &self.intspc())
            .field("sstcl", &self.sstcl())
            .field("srst", &self.srst())
            .field("flash_completion_type", &self.flash_completion_type())
            .field("cpu_tag", &self.cpu_tag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P3irulestat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P3irulestat {{ ustat: {=u8:?}, interr: {=bool:?}, intrd: {=bool:?}, intwr: {=bool:?}, intspc: {=u8:?}, sstcl: {=u8:?}, srst: {=bool:?}, flash_completion_type: {:?}, cpu_tag: {=u8:?} }}",
            self.ustat(),
            self.interr(),
            self.intrd(),
            self.intwr(),
            self.intspc(),
            self.sstcl(),
            self.srst(),
            self.flash_completion_type(),
            self.cpu_tag()
        )
    }
}
#[doc = "Port OOB, Mastering, and Flash Length"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3omflen(pub u32);
impl P3omflen {
    #[doc = "Length in Bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Length in Bytes"]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Transfer Request"]
    #[must_use]
    #[inline(always)]
    pub const fn trans(&self) -> super::vals::P3omflenTrans {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::P3omflenTrans::from_bits(val as u8)
    }
    #[doc = "Transfer Request"]
    #[inline(always)]
    pub const fn set_trans(&mut self, val: super::vals::P3omflenTrans) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for P3omflen {
    #[inline(always)]
    fn default() -> P3omflen {
        P3omflen(0)
    }
}
impl core::fmt::Debug for P3omflen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P3omflen")
            .field("len", &self.len())
            .field("trans", &self.trans())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P3omflen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P3omflen {{ len: {=u8:?}, trans: {:?} }}",
            self.len(),
            self.trans()
        )
    }
}
#[doc = "Port RAM Use"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3ramuse(pub u32);
impl P3ramuse {
    #[doc = "Offset into RAM"]
    #[must_use]
    #[inline(always)]
    pub const fn off(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Offset into RAM"]
    #[inline(always)]
    pub const fn set_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Length"]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Length"]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for P3ramuse {
    #[inline(always)]
    fn default() -> P3ramuse {
        P3ramuse(0)
    }
}
impl core::fmt::Debug for P3ramuse {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P3ramuse")
            .field("off", &self.off())
            .field("len", &self.len())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P3ramuse {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P3ramuse {{ off: {=u16:?}, len: {=u8:?} }}",
            self.off(),
            self.len()
        )
    }
}
#[doc = "Port Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3stat(pub u32);
impl P3stat {
    #[doc = "Host Read Data Status"]
    #[must_use]
    #[inline(always)]
    pub const fn rdstat(&self) -> super::vals::P3statRdstat {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::P3statRdstat::from_bits(val as u8)
    }
    #[doc = "Host Read Data Status"]
    #[inline(always)]
    pub const fn set_rdstat(&mut self, val: super::vals::P3statRdstat) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Status of Host Writes"]
    #[must_use]
    #[inline(always)]
    pub const fn wrstat(&self) -> super::vals::P3statWrstat {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::P3statWrstat::from_bits(val as u8)
    }
    #[doc = "Status of Host Writes"]
    #[inline(always)]
    pub const fn set_wrstat(&mut self, val: super::vals::P3statWrstat) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Interrupt Caused by Error"]
    #[must_use]
    #[inline(always)]
    pub const fn interr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Caused by Error"]
    #[inline(always)]
    pub const fn set_interr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt Caused by Read"]
    #[must_use]
    #[inline(always)]
    pub const fn intrd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Caused by Read"]
    #[inline(always)]
    pub const fn set_intrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt Caused by Write"]
    #[must_use]
    #[inline(always)]
    pub const fn intwr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Caused by Write"]
    #[inline(always)]
    pub const fn set_intwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SPC0 Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SPC0 Interrupt"]
    #[inline(always)]
    pub const fn set_intspc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "SPC1 Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SPC1 Interrupt"]
    #[inline(always)]
    pub const fn set_intspc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SPC2 Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc2(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SPC2 Interrupt"]
    #[inline(always)]
    pub const fn set_intspc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SPC3 Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn instspc3(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SPC3 Interrupt"]
    #[inline(always)]
    pub const fn set_instspc3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Error 0"]
    #[must_use]
    #[inline(always)]
    pub const fn err0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Error 0"]
    #[inline(always)]
    pub const fn set_err0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Error 1"]
    #[must_use]
    #[inline(always)]
    pub const fn err1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Error 1"]
    #[inline(always)]
    pub const fn set_err1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Error 2"]
    #[must_use]
    #[inline(always)]
    pub const fn err2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Error 2"]
    #[inline(always)]
    pub const fn set_err2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Error 3"]
    #[must_use]
    #[inline(always)]
    pub const fn err3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Error 3"]
    #[inline(always)]
    pub const fn set_err3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "RPMC 1 or 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rpmc_1_or_2(&self) -> super::vals::P3statRpmc1Or2 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::P3statRpmc1Or2::from_bits(val as u8)
    }
    #[doc = "RPMC 1 or 2"]
    #[inline(always)]
    pub const fn set_rpmc_1_or_2(&mut self, val: super::vals::P3statRpmc1Or2) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "RPMC Flash Device"]
    #[must_use]
    #[inline(always)]
    pub const fn rpmc_flash_device(&self) -> super::vals::P3statRpmcFlashDevice {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::P3statRpmcFlashDevice::from_bits(val as u8)
    }
    #[doc = "RPMC Flash Device"]
    #[inline(always)]
    pub const fn set_rpmc_flash_device(&mut self, val: super::vals::P3statRpmcFlashDevice) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
}
impl Default for P3stat {
    #[inline(always)]
    fn default() -> P3stat {
        P3stat(0)
    }
}
impl core::fmt::Debug for P3stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P3stat")
            .field("rdstat", &self.rdstat())
            .field("wrstat", &self.wrstat())
            .field("interr", &self.interr())
            .field("intrd", &self.intrd())
            .field("intwr", &self.intwr())
            .field("intspc0", &self.intspc0())
            .field("intspc1", &self.intspc1())
            .field("intspc2", &self.intspc2())
            .field("instspc3", &self.instspc3())
            .field("err0", &self.err0())
            .field("err1", &self.err1())
            .field("err2", &self.err2())
            .field("err3", &self.err3())
            .field("rpmc_1_or_2", &self.rpmc_1_or_2())
            .field("rpmc_flash_device", &self.rpmc_flash_device())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P3stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P3stat {{ rdstat: {:?}, wrstat: {:?}, interr: {=bool:?}, intrd: {=bool:?}, intwr: {=bool:?}, intspc0: {=bool:?}, intspc1: {=bool:?}, intspc2: {=bool:?}, instspc3: {=bool:?}, err0: {=bool:?}, err1: {=bool:?}, err2: {=bool:?}, err3: {=bool:?}, rpmc_1_or_2: {:?}, rpmc_flash_device: {:?} }}",
            self.rdstat(),
            self.wrstat(),
            self.interr(),
            self.intrd(),
            self.intwr(),
            self.intspc0(),
            self.intspc1(),
            self.intspc2(),
            self.instspc3(),
            self.err0(),
            self.err1(),
            self.err2(),
            self.err3(),
            self.rpmc_1_or_2(),
            self.rpmc_flash_device()
        )
    }
}
#[doc = "Port Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4addr(pub u32);
impl P4addr {
    #[doc = "Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn off(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Offset"]
    #[inline(always)]
    pub const fn set_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "The meaning is dependent on type of port:"]
    #[must_use]
    #[inline(always)]
    pub const fn base_asz(&self) -> super::vals::P4addrBaseAsz {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::P4addrBaseAsz::from_bits(val as u8)
    }
    #[doc = "The meaning is dependent on type of port:"]
    #[inline(always)]
    pub const fn set_base_asz(&mut self, val: super::vals::P4addrBaseAsz) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Index Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn idxoff(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Index Offset"]
    #[inline(always)]
    pub const fn set_idxoff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "For index-and-data register only:"]
    #[must_use]
    #[inline(always)]
    pub const fn idx1st(&self) -> super::vals::P4addrIdx1st {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::P4addrIdx1st::from_bits(val as u8)
    }
    #[doc = "For index-and-data register only:"]
    #[inline(always)]
    pub const fn set_idx1st(&mut self, val: super::vals::P4addrIdx1st) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
}
impl Default for P4addr {
    #[inline(always)]
    fn default() -> P4addr {
        P4addr(0)
    }
}
impl core::fmt::Debug for P4addr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P4addr")
            .field("off", &self.off())
            .field("base_asz", &self.base_asz())
            .field("idxoff", &self.idxoff())
            .field("idx1st", &self.idx1st())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P4addr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P4addr {{ off: {=u16:?}, base_asz: {:?}, idxoff: {=u8:?}, idx1st: {:?} }}",
            self.off(),
            self.base_asz(),
            self.idxoff(),
            self.idx1st()
        )
    }
}
#[doc = "Port Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4cfg(pub u32);
impl P4cfg {
    #[doc = "Interaction Type between Port and Host"]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> super::vals::P4cfgType {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::P4cfgType::from_bits(val as u8)
    }
    #[doc = "Interaction Type between Port and Host"]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: super::vals::P4cfgType) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Port Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn direction(&self) -> super::vals::P4cfgDirection {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::P4cfgDirection::from_bits(val as u8)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub const fn set_direction(&mut self, val: super::vals::P4cfgDirection) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Mailbox: map interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn mbintall(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Mailbox: map interrupt"]
    #[inline(always)]
    pub const fn set_mbintall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Stall on any read"]
    #[must_use]
    #[inline(always)]
    pub const fn stallrd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Stall on any read"]
    #[inline(always)]
    pub const fn set_stallrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Stall on write"]
    #[must_use]
    #[inline(always)]
    pub const fn stallwr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Stall on write"]
    #[inline(always)]
    pub const fn set_stallwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Error Origin"]
    #[must_use]
    #[inline(always)]
    pub const fn errorign(&self) -> super::vals::P4cfgErrorign {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::P4cfgErrorign::from_bits(val as u8)
    }
    #[doc = "Error Origin"]
    #[inline(always)]
    pub const fn set_errorign(&mut self, val: super::vals::P4cfgErrorign) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for P4cfg {
    #[inline(always)]
    fn default() -> P4cfg {
        P4cfg(0)
    }
}
impl core::fmt::Debug for P4cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P4cfg")
            .field("type_", &self.type_())
            .field("direction", &self.direction())
            .field("mbintall", &self.mbintall())
            .field("stallrd", &self.stallrd())
            .field("stallwr", &self.stallwr())
            .field("errorign", &self.errorign())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P4cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P4cfg {{ type_: {:?}, direction: {:?}, mbintall: {=bool:?}, stallrd: {=bool:?}, stallwr: {=bool:?}, errorign: {:?} }}",
            self.type_(),
            self.direction(),
            self.mbintall(),
            self.stallrd(),
            self.stallwr(),
            self.errorign()
        )
    }
}
#[doc = "Port Data Input"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4datain(pub u32);
impl P4datain {
    #[doc = "Data Length"]
    #[must_use]
    #[inline(always)]
    pub const fn data_len(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Length"]
    #[inline(always)]
    pub const fn set_data_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Direction of last access"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> super::vals::P4datainDir {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::P4datainDir::from_bits(val as u8)
    }
    #[doc = "Direction of last access"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: super::vals::P4datainDir) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Index of Last Access"]
    #[must_use]
    #[inline(always)]
    pub const fn idx(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Index of Last Access"]
    #[inline(always)]
    pub const fn set_idx(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for P4datain {
    #[inline(always)]
    fn default() -> P4datain {
        P4datain(0)
    }
}
impl core::fmt::Debug for P4datain {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P4datain")
            .field("data_len", &self.data_len())
            .field("dir", &self.dir())
            .field("idx", &self.idx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P4datain {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P4datain {{ data_len: {=u8:?}, dir: {:?}, idx: {=u16:?} }}",
            self.data_len(),
            self.dir(),
            self.idx()
        )
    }
}
#[doc = "Port Data Out"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4dataout(pub u32);
impl P4dataout {
    #[doc = "Data to Send to Host"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data to Send to Host"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for P4dataout {
    #[inline(always)]
    fn default() -> P4dataout {
        P4dataout(0)
    }
}
impl core::fmt::Debug for P4dataout {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P4dataout")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P4dataout {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "P4dataout {{ data: {=u8:?} }}", self.data())
    }
}
#[doc = "Set Interrupt Rules and User Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4irulestat(pub u32);
impl P4irulestat {
    #[doc = "User-Defined Status Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn ustat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "User-Defined Status Bits"]
    #[inline(always)]
    pub const fn set_ustat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Interrupt If Error Detected"]
    #[must_use]
    #[inline(always)]
    pub const fn interr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt If Error Detected"]
    #[inline(always)]
    pub const fn set_interr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt If Read or First Read or Bus Master Started"]
    #[must_use]
    #[inline(always)]
    pub const fn intrd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt If Read or First Read or Bus Master Started"]
    #[inline(always)]
    pub const fn set_intrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt If Write or First Write or Bus Master Finished"]
    #[must_use]
    #[inline(always)]
    pub const fn intwr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt If Write or First Write or Bus Master Finished"]
    #[inline(always)]
    pub const fn set_intwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Select Interrupts for PnSTAT\\[INTSPC0\\] through PnSTAT\\[INSTSPC3\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Select Interrupts for PnSTAT\\[INTSPC0\\] through PnSTAT\\[INSTSPC3\\]"]
    #[inline(always)]
    pub const fn set_intspc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Status Set and Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn sstcl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Status Set and Clear"]
    #[inline(always)]
    pub const fn set_sstcl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Reset PnSTAT\\[RDSTAT\\] and PnSTAT\\[WRSTAT\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn srst(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Reset PnSTAT\\[RDSTAT\\] and PnSTAT\\[WRSTAT\\]"]
    #[inline(always)]
    pub const fn set_srst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Flash Completion Type"]
    #[must_use]
    #[inline(always)]
    pub const fn flash_completion_type(&self) -> super::vals::P4irulestatFlashCompletionType {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::P4irulestatFlashCompletionType::from_bits(val as u8)
    }
    #[doc = "Flash Completion Type"]
    #[inline(always)]
    pub const fn set_flash_completion_type(
        &mut self,
        val: super::vals::P4irulestatFlashCompletionType,
    ) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "CPU Tag"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu_tag(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x0f;
        val as u8
    }
    #[doc = "CPU Tag"]
    #[inline(always)]
    pub const fn set_cpu_tag(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val as u32) & 0x0f) << 23usize);
    }
}
impl Default for P4irulestat {
    #[inline(always)]
    fn default() -> P4irulestat {
        P4irulestat(0)
    }
}
impl core::fmt::Debug for P4irulestat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P4irulestat")
            .field("ustat", &self.ustat())
            .field("interr", &self.interr())
            .field("intrd", &self.intrd())
            .field("intwr", &self.intwr())
            .field("intspc", &self.intspc())
            .field("sstcl", &self.sstcl())
            .field("srst", &self.srst())
            .field("flash_completion_type", &self.flash_completion_type())
            .field("cpu_tag", &self.cpu_tag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P4irulestat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P4irulestat {{ ustat: {=u8:?}, interr: {=bool:?}, intrd: {=bool:?}, intwr: {=bool:?}, intspc: {=u8:?}, sstcl: {=u8:?}, srst: {=bool:?}, flash_completion_type: {:?}, cpu_tag: {=u8:?} }}",
            self.ustat(),
            self.interr(),
            self.intrd(),
            self.intwr(),
            self.intspc(),
            self.sstcl(),
            self.srst(),
            self.flash_completion_type(),
            self.cpu_tag()
        )
    }
}
#[doc = "Port OOB, Mastering, and Flash Length"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4omflen(pub u32);
impl P4omflen {
    #[doc = "Length in Bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Length in Bytes"]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Transfer Request"]
    #[must_use]
    #[inline(always)]
    pub const fn trans(&self) -> super::vals::P4omflenTrans {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::P4omflenTrans::from_bits(val as u8)
    }
    #[doc = "Transfer Request"]
    #[inline(always)]
    pub const fn set_trans(&mut self, val: super::vals::P4omflenTrans) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for P4omflen {
    #[inline(always)]
    fn default() -> P4omflen {
        P4omflen(0)
    }
}
impl core::fmt::Debug for P4omflen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P4omflen")
            .field("len", &self.len())
            .field("trans", &self.trans())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P4omflen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P4omflen {{ len: {=u8:?}, trans: {:?} }}",
            self.len(),
            self.trans()
        )
    }
}
#[doc = "Port RAM Use"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4ramuse(pub u32);
impl P4ramuse {
    #[doc = "Offset into RAM"]
    #[must_use]
    #[inline(always)]
    pub const fn off(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Offset into RAM"]
    #[inline(always)]
    pub const fn set_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Length"]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Length"]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for P4ramuse {
    #[inline(always)]
    fn default() -> P4ramuse {
        P4ramuse(0)
    }
}
impl core::fmt::Debug for P4ramuse {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P4ramuse")
            .field("off", &self.off())
            .field("len", &self.len())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P4ramuse {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P4ramuse {{ off: {=u16:?}, len: {=u8:?} }}",
            self.off(),
            self.len()
        )
    }
}
#[doc = "Port Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4stat(pub u32);
impl P4stat {
    #[doc = "Host Read Data Status"]
    #[must_use]
    #[inline(always)]
    pub const fn rdstat(&self) -> super::vals::P4statRdstat {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::P4statRdstat::from_bits(val as u8)
    }
    #[doc = "Host Read Data Status"]
    #[inline(always)]
    pub const fn set_rdstat(&mut self, val: super::vals::P4statRdstat) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Status of Host Writes"]
    #[must_use]
    #[inline(always)]
    pub const fn wrstat(&self) -> super::vals::P4statWrstat {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::P4statWrstat::from_bits(val as u8)
    }
    #[doc = "Status of Host Writes"]
    #[inline(always)]
    pub const fn set_wrstat(&mut self, val: super::vals::P4statWrstat) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Interrupt Caused by Error"]
    #[must_use]
    #[inline(always)]
    pub const fn interr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Caused by Error"]
    #[inline(always)]
    pub const fn set_interr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt Caused by Read"]
    #[must_use]
    #[inline(always)]
    pub const fn intrd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Caused by Read"]
    #[inline(always)]
    pub const fn set_intrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt Caused by Write"]
    #[must_use]
    #[inline(always)]
    pub const fn intwr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Caused by Write"]
    #[inline(always)]
    pub const fn set_intwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SPC0 Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SPC0 Interrupt"]
    #[inline(always)]
    pub const fn set_intspc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "SPC1 Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SPC1 Interrupt"]
    #[inline(always)]
    pub const fn set_intspc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SPC2 Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn intspc2(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SPC2 Interrupt"]
    #[inline(always)]
    pub const fn set_intspc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SPC3 Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn instspc3(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SPC3 Interrupt"]
    #[inline(always)]
    pub const fn set_instspc3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Error 0"]
    #[must_use]
    #[inline(always)]
    pub const fn err0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Error 0"]
    #[inline(always)]
    pub const fn set_err0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Error 1"]
    #[must_use]
    #[inline(always)]
    pub const fn err1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Error 1"]
    #[inline(always)]
    pub const fn set_err1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Error 2"]
    #[must_use]
    #[inline(always)]
    pub const fn err2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Error 2"]
    #[inline(always)]
    pub const fn set_err2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Error 3"]
    #[must_use]
    #[inline(always)]
    pub const fn err3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Error 3"]
    #[inline(always)]
    pub const fn set_err3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "RPMC 1 or 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rpmc_1_or_2(&self) -> super::vals::P4statRpmc1Or2 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::P4statRpmc1Or2::from_bits(val as u8)
    }
    #[doc = "RPMC 1 or 2"]
    #[inline(always)]
    pub const fn set_rpmc_1_or_2(&mut self, val: super::vals::P4statRpmc1Or2) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "RPMC Flash Device"]
    #[must_use]
    #[inline(always)]
    pub const fn rpmc_flash_device(&self) -> super::vals::P4statRpmcFlashDevice {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::P4statRpmcFlashDevice::from_bits(val as u8)
    }
    #[doc = "RPMC Flash Device"]
    #[inline(always)]
    pub const fn set_rpmc_flash_device(&mut self, val: super::vals::P4statRpmcFlashDevice) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
}
impl Default for P4stat {
    #[inline(always)]
    fn default() -> P4stat {
        P4stat(0)
    }
}
impl core::fmt::Debug for P4stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P4stat")
            .field("rdstat", &self.rdstat())
            .field("wrstat", &self.wrstat())
            .field("interr", &self.interr())
            .field("intrd", &self.intrd())
            .field("intwr", &self.intwr())
            .field("intspc0", &self.intspc0())
            .field("intspc1", &self.intspc1())
            .field("intspc2", &self.intspc2())
            .field("instspc3", &self.instspc3())
            .field("err0", &self.err0())
            .field("err1", &self.err1())
            .field("err2", &self.err2())
            .field("err3", &self.err3())
            .field("rpmc_1_or_2", &self.rpmc_1_or_2())
            .field("rpmc_flash_device", &self.rpmc_flash_device())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P4stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P4stat {{ rdstat: {:?}, wrstat: {:?}, interr: {=bool:?}, intrd: {=bool:?}, intwr: {=bool:?}, intspc0: {=bool:?}, intspc1: {=bool:?}, intspc2: {=bool:?}, instspc3: {=bool:?}, err0: {=bool:?}, err1: {=bool:?}, err2: {=bool:?}, err3: {=bool:?}, rpmc_1_or_2: {:?}, rpmc_flash_device: {:?} }}",
            self.rdstat(),
            self.wrstat(),
            self.interr(),
            self.intrd(),
            self.intwr(),
            self.intspc0(),
            self.intspc1(),
            self.intspc2(),
            self.instspc3(),
            self.err0(),
            self.err1(),
            self.err2(),
            self.err3(),
            self.rpmc_1_or_2(),
            self.rpmc_flash_device()
        )
    }
}
#[doc = "Port 80 Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P80stat(pub u32);
impl P80stat {
    #[doc = "Current Port80 Value"]
    #[must_use]
    #[inline(always)]
    pub const fn curr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Current Port80 Value"]
    #[inline(always)]
    pub const fn set_curr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Previous Port80 Value"]
    #[must_use]
    #[inline(always)]
    pub const fn prev(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Previous Port80 Value"]
    #[inline(always)]
    pub const fn set_prev(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Counter"]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Counter Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn rst(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Counter Reset"]
    #[inline(always)]
    pub const fn set_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for P80stat {
    #[inline(always)]
    fn default() -> P80stat {
        P80stat(0)
    }
}
impl core::fmt::Debug for P80stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P80stat")
            .field("curr", &self.curr())
            .field("prev", &self.prev())
            .field("cnt", &self.cnt())
            .field("rst", &self.rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P80stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P80stat {{ curr: {=u8:?}, prev: {=u8:?}, cnt: {=u8:?}, rst: {=bool:?} }}",
            self.curr(),
            self.prev(),
            self.cnt(),
            self.rst()
        )
    }
}
#[doc = "RAM Base"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rambase(pub u32);
impl Rambase {
    #[doc = "Always 0"]
    #[must_use]
    #[inline(always)]
    pub const fn zero(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Always 0"]
    #[inline(always)]
    pub const fn set_zero(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "RAM Location"]
    #[must_use]
    #[inline(always)]
    pub const fn ram(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "RAM Location"]
    #[inline(always)]
    pub const fn set_ram(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Rambase {
    #[inline(always)]
    fn default() -> Rambase {
        Rambase(0)
    }
}
impl core::fmt::Debug for Rambase {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rambase")
            .field("zero", &self.zero())
            .field("ram", &self.ram())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rambase {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rambase {{ zero: {=u16:?}, ram: {=u32:?} }}",
            self.zero(),
            self.ram()
        )
    }
}
#[doc = "RPMC Support 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RpmcSupport1(pub u32);
impl RpmcSupport1 {
    #[doc = "Target RPMC Supported"]
    #[must_use]
    #[inline(always)]
    pub const fn target_rpmc_supported(&self) -> super::vals::TargetRpmcSupported {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::TargetRpmcSupported::from_bits(val as u8)
    }
    #[doc = "Target RPMC Supported"]
    #[inline(always)]
    pub const fn set_target_rpmc_supported(&mut self, val: super::vals::TargetRpmcSupported) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "Number of Target Attached Flash RPMC flash devices"]
    #[must_use]
    #[inline(always)]
    pub const fn num_of_target(&self) -> super::vals::NumOfTarget {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::NumOfTarget::from_bits(val as u8)
    }
    #[doc = "Number of Target Attached Flash RPMC flash devices"]
    #[inline(always)]
    pub const fn set_num_of_target(&mut self, val: super::vals::NumOfTarget) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "RPMC OP1 Opcode on the 1st RPMC Flash device"]
    #[must_use]
    #[inline(always)]
    pub const fn rpmc_op1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "RPMC OP1 Opcode on the 1st RPMC Flash device"]
    #[inline(always)]
    pub const fn set_rpmc_op1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "RPMC Counter on the 1st RPMC Flash device"]
    #[must_use]
    #[inline(always)]
    pub const fn rpmc_counter(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "RPMC Counter on the 1st RPMC Flash device"]
    #[inline(always)]
    pub const fn set_rpmc_counter(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "RPMC OP1 Opcode on the 2nd RPMC Flash device"]
    #[must_use]
    #[inline(always)]
    pub const fn rpmc_op1_2(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0xff;
        val as u8
    }
    #[doc = "RPMC OP1 Opcode on the 2nd RPMC Flash device"]
    #[inline(always)]
    pub const fn set_rpmc_op1_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
    }
    #[doc = "RPMC Counter on the 2nd RPMC Flash device"]
    #[must_use]
    #[inline(always)]
    pub const fn rpmc_counter_2(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "RPMC Counter on the 2nd RPMC Flash device"]
    #[inline(always)]
    pub const fn set_rpmc_counter_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for RpmcSupport1 {
    #[inline(always)]
    fn default() -> RpmcSupport1 {
        RpmcSupport1(0)
    }
}
impl core::fmt::Debug for RpmcSupport1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RpmcSupport1")
            .field("target_rpmc_supported", &self.target_rpmc_supported())
            .field("num_of_target", &self.num_of_target())
            .field("rpmc_op1", &self.rpmc_op1())
            .field("rpmc_counter", &self.rpmc_counter())
            .field("rpmc_op1_2", &self.rpmc_op1_2())
            .field("rpmc_counter_2", &self.rpmc_counter_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RpmcSupport1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RpmcSupport1 {{ target_rpmc_supported: {:?}, num_of_target: {:?}, rpmc_op1: {=u8:?}, rpmc_counter: {=u8:?}, rpmc_op1_2: {=u8:?}, rpmc_counter_2: {=u8:?} }}",
            self.target_rpmc_supported(),
            self.num_of_target(),
            self.rpmc_op1(),
            self.rpmc_counter(),
            self.rpmc_op1_2(),
            self.rpmc_counter_2()
        )
    }
}
#[doc = "RPMC Support 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RpmcSupport2(pub u32);
impl RpmcSupport2 {
    #[doc = "RPMC Counter on the 3rd RPMC Flash device"]
    #[must_use]
    #[inline(always)]
    pub const fn rpmc_counter_3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "RPMC Counter on the 3rd RPMC Flash device"]
    #[inline(always)]
    pub const fn set_rpmc_counter_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "RPMC OP1 Opcode on the 3rd RPMC Flash device"]
    #[must_use]
    #[inline(always)]
    pub const fn rpmc_op1_3(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0xff;
        val as u8
    }
    #[doc = "RPMC OP1 Opcode on the 3rd RPMC Flash device"]
    #[inline(always)]
    pub const fn set_rpmc_op1_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
    }
    #[doc = "RPMC Counter on the 4th RPMC Flash device"]
    #[must_use]
    #[inline(always)]
    pub const fn rpmc_counter_4(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "RPMC Counter on the 4th RPMC Flash device"]
    #[inline(always)]
    pub const fn set_rpmc_counter_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "RPMC OP1 Opcode on the 4th RPMC Flash device"]
    #[must_use]
    #[inline(always)]
    pub const fn rpmc_op1_4(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "RPMC OP1 Opcode on the 4th RPMC Flash device"]
    #[inline(always)]
    pub const fn set_rpmc_op1_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for RpmcSupport2 {
    #[inline(always)]
    fn default() -> RpmcSupport2 {
        RpmcSupport2(0)
    }
}
impl core::fmt::Debug for RpmcSupport2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RpmcSupport2")
            .field("rpmc_counter_3", &self.rpmc_counter_3())
            .field("rpmc_op1_3", &self.rpmc_op1_3())
            .field("rpmc_counter_4", &self.rpmc_counter_4())
            .field("rpmc_op1_4", &self.rpmc_op1_4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RpmcSupport2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RpmcSupport2 {{ rpmc_counter_3: {=u8:?}, rpmc_op1_3: {=u8:?}, rpmc_counter_4: {=u8:?}, rpmc_op1_4: {=u8:?} }}",
            self.rpmc_counter_3(),
            self.rpmc_op1_3(),
            self.rpmc_counter_4(),
            self.rpmc_op1_4()
        )
    }
}
#[doc = "Status Block Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stataddr(pub u32);
impl Stataddr {
    #[doc = "Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn off(&self) -> u16 {
        let val = (self.0 >> 3usize) & 0x1fff;
        val as u16
    }
    #[doc = "Offset"]
    #[inline(always)]
    pub const fn set_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 3usize)) | (((val as u32) & 0x1fff) << 3usize);
    }
    #[doc = "Offset Base"]
    #[must_use]
    #[inline(always)]
    pub const fn base(&self) -> super::vals::Base {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Base::from_bits(val as u8)
    }
    #[doc = "Offset Base"]
    #[inline(always)]
    pub const fn set_base(&mut self, val: super::vals::Base) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
}
impl Default for Stataddr {
    #[inline(always)]
    fn default() -> Stataddr {
        Stataddr(0)
    }
}
impl core::fmt::Debug for Stataddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stataddr")
            .field("off", &self.off())
            .field("base", &self.base())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stataddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stataddr {{ off: {=u16:?}, base: {:?} }}",
            self.off(),
            self.base()
        )
    }
}
#[doc = "WIREIN_GPIO"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WireinGpio(pub u32);
impl WireinGpio {
    #[doc = "Level"]
    #[must_use]
    #[inline(always)]
    pub const fn level(&self) -> super::vals::WireinGpioLevel {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::WireinGpioLevel::from_bits(val as u8)
    }
    #[doc = "Level"]
    #[inline(always)]
    pub const fn set_level(&mut self, val: super::vals::WireinGpioLevel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> super::vals::WireinGpioValid {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::WireinGpioValid::from_bits(val as u8)
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: super::vals::WireinGpioValid) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Index"]
    #[must_use]
    #[inline(always)]
    pub const fn index(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Index"]
    #[inline(always)]
    pub const fn set_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for WireinGpio {
    #[inline(always)]
    fn default() -> WireinGpio {
        WireinGpio(0)
    }
}
impl core::fmt::Debug for WireinGpio {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WireinGpio")
            .field("level", &self.level())
            .field("valid", &self.valid())
            .field("index", &self.index())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WireinGpio {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WireinGpio {{ level: {:?}, valid: {:?}, index: {=u8:?} }}",
            self.level(),
            self.valid(),
            self.index()
        )
    }
}
#[doc = "WIREOUT_GPIO"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WireoutGpio(pub u32);
impl WireoutGpio {
    #[doc = "Level"]
    #[must_use]
    #[inline(always)]
    pub const fn level(&self) -> super::vals::WireoutGpioLevel {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::WireoutGpioLevel::from_bits(val as u8)
    }
    #[doc = "Level"]
    #[inline(always)]
    pub const fn set_level(&mut self, val: super::vals::WireoutGpioLevel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> super::vals::WireoutGpioValid {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::WireoutGpioValid::from_bits(val as u8)
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: super::vals::WireoutGpioValid) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Index"]
    #[must_use]
    #[inline(always)]
    pub const fn index(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Index"]
    #[inline(always)]
    pub const fn set_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for WireoutGpio {
    #[inline(always)]
    fn default() -> WireoutGpio {
        WireoutGpio(0)
    }
}
impl core::fmt::Debug for WireoutGpio {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WireoutGpio")
            .field("level", &self.level())
            .field("valid", &self.valid())
            .field("index", &self.index())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WireoutGpio {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WireoutGpio {{ level: {:?}, valid: {:?}, index: {=u8:?} }}",
            self.level(),
            self.valid(),
            self.index()
        )
    }
}
#[doc = "Virtual Wire Host-to-MCU"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wirero(pub u32);
impl Wirero {
    #[doc = "Sleep State 3"]
    #[must_use]
    #[inline(always)]
    pub const fn slp_s3n(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Sleep State 3"]
    #[inline(always)]
    pub const fn set_slp_s3n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Sleep State 4"]
    #[must_use]
    #[inline(always)]
    pub const fn slp_s4n(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Sleep State 4"]
    #[inline(always)]
    pub const fn set_slp_s4n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Sleep State 5"]
    #[must_use]
    #[inline(always)]
    pub const fn slp_s5n(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Sleep State 5"]
    #[inline(always)]
    pub const fn set_slp_s5n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Suspend Status"]
    #[must_use]
    #[inline(always)]
    pub const fn sus_stat(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Suspend Status"]
    #[inline(always)]
    pub const fn set_sus_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Reset Request"]
    #[must_use]
    #[inline(always)]
    pub const fn pltrstn(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Reset Request"]
    #[inline(always)]
    pub const fn set_pltrstn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Out-Of-Band Reset Warning"]
    #[must_use]
    #[inline(always)]
    pub const fn oob_rst_warn(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Out-Of-Band Reset Warning"]
    #[inline(always)]
    pub const fn set_oob_rst_warn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Host Reset Warning"]
    #[must_use]
    #[inline(always)]
    pub const fn host_rst_warn(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Host Reset Warning"]
    #[inline(always)]
    pub const fn set_host_rst_warn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Suspend Warning"]
    #[must_use]
    #[inline(always)]
    pub const fn sus_warn(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Suspend Warning"]
    #[inline(always)]
    pub const fn set_sus_warn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Suspend Power Well Acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn sus_pwrdn_ackn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Suspend Power Well Acknowledge"]
    #[inline(always)]
    pub const fn set_sus_pwrdn_ackn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Sleep AN"]
    #[must_use]
    #[inline(always)]
    pub const fn slp_an(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Sleep AN"]
    #[inline(always)]
    pub const fn set_slp_an(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Wired LAN Sleep"]
    #[must_use]
    #[inline(always)]
    pub const fn slp_lan(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Wired LAN Sleep"]
    #[inline(always)]
    pub const fn set_slp_lan(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Wireless LAN Sleep"]
    #[must_use]
    #[inline(always)]
    pub const fn slp_wlan(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Wireless LAN Sleep"]
    #[inline(always)]
    pub const fn set_slp_wlan(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "PCIe to EC"]
    #[must_use]
    #[inline(always)]
    pub const fn p2e(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0xff;
        val as u8
    }
    #[doc = "PCIe to EC"]
    #[inline(always)]
    pub const fn set_p2e(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 12usize)) | (((val as u32) & 0xff) << 12usize);
    }
    #[doc = "Host Entering Deep Power Down C10 State"]
    #[must_use]
    #[inline(always)]
    pub const fn host_c10n(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Host Entering Deep Power Down C10 State"]
    #[inline(always)]
    pub const fn set_host_c10n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Wirero {
    #[inline(always)]
    fn default() -> Wirero {
        Wirero(0)
    }
}
impl core::fmt::Debug for Wirero {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wirero")
            .field("slp_s3n", &self.slp_s3n())
            .field("slp_s4n", &self.slp_s4n())
            .field("slp_s5n", &self.slp_s5n())
            .field("sus_stat", &self.sus_stat())
            .field("pltrstn", &self.pltrstn())
            .field("oob_rst_warn", &self.oob_rst_warn())
            .field("host_rst_warn", &self.host_rst_warn())
            .field("sus_warn", &self.sus_warn())
            .field("sus_pwrdn_ackn", &self.sus_pwrdn_ackn())
            .field("slp_an", &self.slp_an())
            .field("slp_lan", &self.slp_lan())
            .field("slp_wlan", &self.slp_wlan())
            .field("p2e", &self.p2e())
            .field("host_c10n", &self.host_c10n())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wirero {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wirero {{ slp_s3n: {=bool:?}, slp_s4n: {=bool:?}, slp_s5n: {=bool:?}, sus_stat: {=bool:?}, pltrstn: {=bool:?}, oob_rst_warn: {=bool:?}, host_rst_warn: {=bool:?}, sus_warn: {=bool:?}, sus_pwrdn_ackn: {=bool:?}, slp_an: {=bool:?}, slp_lan: {=bool:?}, slp_wlan: {=bool:?}, p2e: {=u8:?}, host_c10n: {=bool:?} }}",
            self.slp_s3n(),
            self.slp_s4n(),
            self.slp_s5n(),
            self.sus_stat(),
            self.pltrstn(),
            self.oob_rst_warn(),
            self.host_rst_warn(),
            self.sus_warn(),
            self.sus_pwrdn_ackn(),
            self.slp_an(),
            self.slp_lan(),
            self.slp_wlan(),
            self.p2e(),
            self.host_c10n()
        )
    }
}
#[doc = "Virtual Wire MCU-to-host"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wirewo(pub u32);
impl Wirewo {
    #[doc = "Out-Of-Band Reset Acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn oob_rst_ack(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Out-Of-Band Reset Acknowledge"]
    #[inline(always)]
    pub const fn set_oob_rst_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IRQ1"]
    #[must_use]
    #[inline(always)]
    pub const fn waken_scin(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ1"]
    #[inline(always)]
    pub const fn set_waken_scin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SMIN"]
    #[must_use]
    #[inline(always)]
    pub const fn pmen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SMIN"]
    #[inline(always)]
    pub const fn set_pmen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IRQ2"]
    #[must_use]
    #[inline(always)]
    pub const fn scin(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ2"]
    #[inline(always)]
    pub const fn set_scin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IRQ3"]
    #[must_use]
    #[inline(always)]
    pub const fn smin(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ3"]
    #[inline(always)]
    pub const fn set_smin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IRQ4"]
    #[must_use]
    #[inline(always)]
    pub const fn rcinn(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ4"]
    #[inline(always)]
    pub const fn set_rcinn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "IRQ5"]
    #[must_use]
    #[inline(always)]
    pub const fn host_rst_ack(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ5"]
    #[inline(always)]
    pub const fn set_host_rst_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "IRQ6"]
    #[must_use]
    #[inline(always)]
    pub const fn susackn(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "IRQ6"]
    #[inline(always)]
    pub const fn set_susackn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "IRQ7-IRQ15"]
    #[must_use]
    #[inline(always)]
    pub const fn e2p(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "IRQ7-IRQ15"]
    #[inline(always)]
    pub const fn set_e2p(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Boot Load Done"]
    #[must_use]
    #[inline(always)]
    pub const fn boot_done(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Boot Load Done"]
    #[inline(always)]
    pub const fn set_boot_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Boot Load Error"]
    #[must_use]
    #[inline(always)]
    pub const fn boot_errn(&self) -> super::vals::BootErrn {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::BootErrn::from_bits(val as u8)
    }
    #[doc = "Boot Load Error"]
    #[inline(always)]
    pub const fn set_boot_errn(&mut self, val: super::vals::BootErrn) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Force"]
    #[must_use]
    #[inline(always)]
    pub const fn dsw_pwrok_rst(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Force"]
    #[inline(always)]
    pub const fn set_dsw_pwrok_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Write Done"]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Write Done"]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Wirewo {
    #[inline(always)]
    fn default() -> Wirewo {
        Wirewo(0)
    }
}
impl core::fmt::Debug for Wirewo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wirewo")
            .field("oob_rst_ack", &self.oob_rst_ack())
            .field("waken_scin", &self.waken_scin())
            .field("pmen", &self.pmen())
            .field("scin", &self.scin())
            .field("smin", &self.smin())
            .field("rcinn", &self.rcinn())
            .field("host_rst_ack", &self.host_rst_ack())
            .field("susackn", &self.susackn())
            .field("e2p", &self.e2p())
            .field("boot_done", &self.boot_done())
            .field("boot_errn", &self.boot_errn())
            .field("dsw_pwrok_rst", &self.dsw_pwrok_rst())
            .field("done", &self.done())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wirewo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wirewo {{ oob_rst_ack: {=bool:?}, waken_scin: {=bool:?}, pmen: {=bool:?}, scin: {=bool:?}, smin: {=bool:?}, rcinn: {=bool:?}, host_rst_ack: {=bool:?}, susackn: {=bool:?}, e2p: {=u8:?}, boot_done: {=bool:?}, boot_errn: {:?}, dsw_pwrok_rst: {=bool:?}, done: {=bool:?} }}",
            self.oob_rst_ack(),
            self.waken_scin(),
            self.pmen(),
            self.scin(),
            self.smin(),
            self.rcinn(),
            self.host_rst_ack(),
            self.susackn(),
            self.e2p(),
            self.boot_done(),
            self.boot_errn(),
            self.dsw_pwrok_rst(),
            self.done()
        )
    }
}
