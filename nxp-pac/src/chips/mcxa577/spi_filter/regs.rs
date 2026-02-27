#[doc = "Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Flash memory organization"]
    #[must_use]
    #[inline(always)]
    pub const fn p1_single_flash(&self) -> super::vals::P1SingleFlash {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::P1SingleFlash::from_bits(val as u8)
    }
    #[doc = "Flash memory organization"]
    #[inline(always)]
    pub const fn set_p1_single_flash(&mut self, val: super::vals::P1SingleFlash) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Single Flash mode Chip Select"]
    #[must_use]
    #[inline(always)]
    pub const fn p1_single_flash_chip_select(&self) -> super::vals::P1SingleFlashChipSelect {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::P1SingleFlashChipSelect::from_bits(val as u8)
    }
    #[doc = "Single Flash mode Chip Select"]
    #[inline(always)]
    pub const fn set_p1_single_flash_chip_select(
        &mut self,
        val: super::vals::P1SingleFlashChipSelect,
    ) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Single Flash Allow Write"]
    #[must_use]
    #[inline(always)]
    pub const fn p1_single_flash_allow_write(&self) -> super::vals::P1SingleFlashAllowWrite {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::P1SingleFlashAllowWrite::from_bits(val as u8)
    }
    #[doc = "Single Flash Allow Write"]
    #[inline(always)]
    pub const fn set_p1_single_flash_allow_write(
        &mut self,
        val: super::vals::P1SingleFlashAllowWrite,
    ) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Flash memory organization"]
    #[must_use]
    #[inline(always)]
    pub const fn p0_single_flash(&self) -> super::vals::P0SingleFlash {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::P0SingleFlash::from_bits(val as u8)
    }
    #[doc = "Flash memory organization"]
    #[inline(always)]
    pub const fn set_p0_single_flash(&mut self, val: super::vals::P0SingleFlash) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Single Flash mode Chip Select"]
    #[must_use]
    #[inline(always)]
    pub const fn p0_single_flash_chip_select(&self) -> super::vals::P0SingleFlashChipSelect {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::P0SingleFlashChipSelect::from_bits(val as u8)
    }
    #[doc = "Single Flash mode Chip Select"]
    #[inline(always)]
    pub const fn set_p0_single_flash_chip_select(
        &mut self,
        val: super::vals::P0SingleFlashChipSelect,
    ) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Single Flash Allow Write"]
    #[must_use]
    #[inline(always)]
    pub const fn p0_single_flash_allow_write(&self) -> super::vals::P0SingleFlashAllowWrite {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::P0SingleFlashAllowWrite::from_bits(val as u8)
    }
    #[doc = "Single Flash Allow Write"]
    #[inline(always)]
    pub const fn set_p0_single_flash_allow_write(
        &mut self,
        val: super::vals::P0SingleFlashAllowWrite,
    ) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Active Chip Select for Flash memory"]
    #[must_use]
    #[inline(always)]
    pub const fn p1_act_sel(&self) -> super::vals::P1ActSel {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::P1ActSel::from_bits(val as u8)
    }
    #[doc = "Active Chip Select for Flash memory"]
    #[inline(always)]
    pub const fn set_p1_act_sel(&mut self, val: super::vals::P1ActSel) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Write enable switch for address byte mode"]
    #[must_use]
    #[inline(always)]
    pub const fn p1_we_byte_md(&self) -> super::vals::P1WeByteMd {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::P1WeByteMd::from_bits(val as u8)
    }
    #[doc = "Write enable switch for address byte mode"]
    #[inline(always)]
    pub const fn set_p1_we_byte_md(&mut self, val: super::vals::P1WeByteMd) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Address Byte change mode"]
    #[must_use]
    #[inline(always)]
    pub const fn p1_perm_byte_md(&self) -> super::vals::P1PermByteMd {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::P1PermByteMd::from_bits(val as u8)
    }
    #[doc = "Address Byte change mode"]
    #[inline(always)]
    pub const fn set_p1_perm_byte_md(&mut self, val: super::vals::P1PermByteMd) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Address Byte Select"]
    #[must_use]
    #[inline(always)]
    pub const fn p1_byte_sel(&self) -> super::vals::P1ByteSel {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::P1ByteSel::from_bits(val as u8)
    }
    #[doc = "Address Byte Select"]
    #[inline(always)]
    pub const fn set_p1_byte_sel(&mut self, val: super::vals::P1ByteSel) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Address Byte Select Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn p1_byte_sel_md(&self) -> super::vals::P1ByteSelMd {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::P1ByteSelMd::from_bits(val as u8)
    }
    #[doc = "Address Byte Select Mode"]
    #[inline(always)]
    pub const fn set_p1_byte_sel_md(&mut self, val: super::vals::P1ByteSelMd) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Clear dirty state for P1"]
    #[must_use]
    #[inline(always)]
    pub const fn p1_dirty_clr(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Clear dirty state for P1"]
    #[inline(always)]
    pub const fn set_p1_dirty_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Filter Enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn p1_flt_en(&self) -> super::vals::P1FltEn {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::P1FltEn::from_bits(val as u8)
    }
    #[doc = "Filter Enable bit"]
    #[inline(always)]
    pub const fn set_p1_flt_en(&mut self, val: super::vals::P1FltEn) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Address mode setting after reset"]
    #[must_use]
    #[inline(always)]
    pub const fn p1_byte_md_rst(&self) -> super::vals::P1ByteMdRst {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::P1ByteMdRst::from_bits(val as u8)
    }
    #[doc = "Address mode setting after reset"]
    #[inline(always)]
    pub const fn set_p1_byte_md_rst(&mut self, val: super::vals::P1ByteMdRst) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Active Chip Select for Flash memory"]
    #[must_use]
    #[inline(always)]
    pub const fn p0_act_sel(&self) -> super::vals::P0ActSel {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::P0ActSel::from_bits(val as u8)
    }
    #[doc = "Active Chip Select for Flash memory"]
    #[inline(always)]
    pub const fn set_p0_act_sel(&mut self, val: super::vals::P0ActSel) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Write enable switch for address byte mode"]
    #[must_use]
    #[inline(always)]
    pub const fn p0_we_byte_md(&self) -> super::vals::P0WeByteMd {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::P0WeByteMd::from_bits(val as u8)
    }
    #[doc = "Write enable switch for address byte mode"]
    #[inline(always)]
    pub const fn set_p0_we_byte_md(&mut self, val: super::vals::P0WeByteMd) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Address Byte change mode"]
    #[must_use]
    #[inline(always)]
    pub const fn p0_perm_byte_md(&self) -> super::vals::P0PermByteMd {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::P0PermByteMd::from_bits(val as u8)
    }
    #[doc = "Address Byte change mode"]
    #[inline(always)]
    pub const fn set_p0_perm_byte_md(&mut self, val: super::vals::P0PermByteMd) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Address Byte Select Mode for P0"]
    #[must_use]
    #[inline(always)]
    pub const fn p0_byte_sel(&self) -> super::vals::P0ByteSel {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::P0ByteSel::from_bits(val as u8)
    }
    #[doc = "Address Byte Select Mode for P0"]
    #[inline(always)]
    pub const fn set_p0_byte_sel(&mut self, val: super::vals::P0ByteSel) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Address Byte Select Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn p0_byte_sel_md(&self) -> super::vals::P0ByteSelMd {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::P0ByteSelMd::from_bits(val as u8)
    }
    #[doc = "Address Byte Select Mode"]
    #[inline(always)]
    pub const fn set_p0_byte_sel_md(&mut self, val: super::vals::P0ByteSelMd) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Clear dirty state for P0"]
    #[must_use]
    #[inline(always)]
    pub const fn p0_dirty_clr(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Clear dirty state for P0"]
    #[inline(always)]
    pub const fn set_p0_dirty_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Filter Enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn p0_flt_en(&self) -> super::vals::P0FltEn {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::P0FltEn::from_bits(val as u8)
    }
    #[doc = "Filter Enable bit"]
    #[inline(always)]
    pub const fn set_p0_flt_en(&mut self, val: super::vals::P0FltEn) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Address mode setting after reset"]
    #[must_use]
    #[inline(always)]
    pub const fn p0_byte_md_rst(&self) -> super::vals::P0ByteMdRst {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::P0ByteMdRst::from_bits(val as u8)
    }
    #[doc = "Address mode setting after reset"]
    #[inline(always)]
    pub const fn set_p0_byte_md_rst(&mut self, val: super::vals::P0ByteMdRst) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("p1_single_flash", &self.p1_single_flash())
            .field(
                "p1_single_flash_chip_select",
                &self.p1_single_flash_chip_select(),
            )
            .field(
                "p1_single_flash_allow_write",
                &self.p1_single_flash_allow_write(),
            )
            .field("p0_single_flash", &self.p0_single_flash())
            .field(
                "p0_single_flash_chip_select",
                &self.p0_single_flash_chip_select(),
            )
            .field(
                "p0_single_flash_allow_write",
                &self.p0_single_flash_allow_write(),
            )
            .field("p1_act_sel", &self.p1_act_sel())
            .field("p1_we_byte_md", &self.p1_we_byte_md())
            .field("p1_perm_byte_md", &self.p1_perm_byte_md())
            .field("p1_byte_sel", &self.p1_byte_sel())
            .field("p1_byte_sel_md", &self.p1_byte_sel_md())
            .field("p1_dirty_clr", &self.p1_dirty_clr())
            .field("p1_flt_en", &self.p1_flt_en())
            .field("p1_byte_md_rst", &self.p1_byte_md_rst())
            .field("p0_act_sel", &self.p0_act_sel())
            .field("p0_we_byte_md", &self.p0_we_byte_md())
            .field("p0_perm_byte_md", &self.p0_perm_byte_md())
            .field("p0_byte_sel", &self.p0_byte_sel())
            .field("p0_byte_sel_md", &self.p0_byte_sel_md())
            .field("p0_dirty_clr", &self.p0_dirty_clr())
            .field("p0_flt_en", &self.p0_flt_en())
            .field("p0_byte_md_rst", &self.p0_byte_md_rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr {{ p1_single_flash: {:?}, p1_single_flash_chip_select: {:?}, p1_single_flash_allow_write: {:?}, p0_single_flash: {:?}, p0_single_flash_chip_select: {:?}, p0_single_flash_allow_write: {:?}, p1_act_sel: {:?}, p1_we_byte_md: {:?}, p1_perm_byte_md: {:?}, p1_byte_sel: {:?}, p1_byte_sel_md: {:?}, p1_dirty_clr: {=bool:?}, p1_flt_en: {:?}, p1_byte_md_rst: {:?}, p0_act_sel: {:?}, p0_we_byte_md: {:?}, p0_perm_byte_md: {:?}, p0_byte_sel: {:?}, p0_byte_sel_md: {:?}, p0_dirty_clr: {=bool:?}, p0_flt_en: {:?}, p0_byte_md_rst: {:?} }}",
            self.p1_single_flash(),
            self.p1_single_flash_chip_select(),
            self.p1_single_flash_allow_write(),
            self.p0_single_flash(),
            self.p0_single_flash_chip_select(),
            self.p0_single_flash_allow_write(),
            self.p1_act_sel(),
            self.p1_we_byte_md(),
            self.p1_perm_byte_md(),
            self.p1_byte_sel(),
            self.p1_byte_sel_md(),
            self.p1_dirty_clr(),
            self.p1_flt_en(),
            self.p1_byte_md_rst(),
            self.p0_act_sel(),
            self.p0_we_byte_md(),
            self.p0_perm_byte_md(),
            self.p0_byte_sel(),
            self.p0_byte_sel_md(),
            self.p0_dirty_clr(),
            self.p0_flt_en(),
            self.p0_byte_md_rst()
        )
    }
}
#[doc = "Interrupt Mask Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imr(pub u32);
impl Imr {
    #[doc = "Mask bit for P0_F8_INT"]
    #[must_use]
    #[inline(always)]
    pub const fn p0_f8_mask(&self) -> super::vals::P0F8Mask {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::P0F8Mask::from_bits(val as u8)
    }
    #[doc = "Mask bit for P0_F8_INT"]
    #[inline(always)]
    pub const fn set_p0_f8_mask(&mut self, val: super::vals::P0F8Mask) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Mask bit for P1_F8_INT"]
    #[must_use]
    #[inline(always)]
    pub const fn p1_f8_mask(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Mask bit for P1_F8_INT"]
    #[inline(always)]
    pub const fn set_p1_f8_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Mask bit for P0_DIRTY_INT"]
    #[must_use]
    #[inline(always)]
    pub const fn p0_dirty_mask(&self) -> super::vals::P0DirtyMask {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::P0DirtyMask::from_bits(val as u8)
    }
    #[doc = "Mask bit for P0_DIRTY_INT"]
    #[inline(always)]
    pub const fn set_p0_dirty_mask(&mut self, val: super::vals::P0DirtyMask) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Mask bit for P1_DIRTY_INT"]
    #[must_use]
    #[inline(always)]
    pub const fn p1_dirty_mask(&self) -> super::vals::P1DirtyMask {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::P1DirtyMask::from_bits(val as u8)
    }
    #[doc = "Mask bit for P1_DIRTY_INT"]
    #[inline(always)]
    pub const fn set_p1_dirty_mask(&mut self, val: super::vals::P1DirtyMask) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Mask bit for P0_BLK_INT"]
    #[must_use]
    #[inline(always)]
    pub const fn p0_blk_mask(&self) -> super::vals::P0BlkMask {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::P0BlkMask::from_bits(val as u8)
    }
    #[doc = "Mask bit for P0_BLK_INT"]
    #[inline(always)]
    pub const fn set_p0_blk_mask(&mut self, val: super::vals::P0BlkMask) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Mask bit for P1_BLK_INT"]
    #[must_use]
    #[inline(always)]
    pub const fn p1_blk_mask(&self) -> super::vals::P1BlkMask {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::P1BlkMask::from_bits(val as u8)
    }
    #[doc = "Mask bit for P1_BLK_INT"]
    #[inline(always)]
    pub const fn set_p1_blk_mask(&mut self, val: super::vals::P1BlkMask) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Mask bit for P0_BYTEMODE_INT"]
    #[must_use]
    #[inline(always)]
    pub const fn p0_bytemode_mask(&self) -> super::vals::P0BytemodeMask {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::P0BytemodeMask::from_bits(val as u8)
    }
    #[doc = "Mask bit for P0_BYTEMODE_INT"]
    #[inline(always)]
    pub const fn set_p0_bytemode_mask(&mut self, val: super::vals::P0BytemodeMask) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Mask bit for P1_BYTEMODE_INT"]
    #[must_use]
    #[inline(always)]
    pub const fn p1_bytemode_mask(&self) -> super::vals::P1BytemodeMask {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::P1BytemodeMask::from_bits(val as u8)
    }
    #[doc = "Mask bit for P1_BYTEMODE_INT"]
    #[inline(always)]
    pub const fn set_p1_bytemode_mask(&mut self, val: super::vals::P1BytemodeMask) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Imr {
    #[inline(always)]
    fn default() -> Imr {
        Imr(0)
    }
}
impl core::fmt::Debug for Imr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Imr")
            .field("p0_f8_mask", &self.p0_f8_mask())
            .field("p1_f8_mask", &self.p1_f8_mask())
            .field("p0_dirty_mask", &self.p0_dirty_mask())
            .field("p1_dirty_mask", &self.p1_dirty_mask())
            .field("p0_blk_mask", &self.p0_blk_mask())
            .field("p1_blk_mask", &self.p1_blk_mask())
            .field("p0_bytemode_mask", &self.p0_bytemode_mask())
            .field("p1_bytemode_mask", &self.p1_bytemode_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Imr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Imr {{ p0_f8_mask: {:?}, p1_f8_mask: {=bool:?}, p0_dirty_mask: {:?}, p1_dirty_mask: {:?}, p0_blk_mask: {:?}, p1_blk_mask: {:?}, p0_bytemode_mask: {:?}, p1_bytemode_mask: {:?} }}",
            self.p0_f8_mask(),
            self.p1_f8_mask(),
            self.p0_dirty_mask(),
            self.p1_dirty_mask(),
            self.p0_blk_mask(),
            self.p1_blk_mask(),
            self.p0_bytemode_mask(),
            self.p1_bytemode_mask()
        )
    }
}
#[doc = "Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc = "Indicates PORT0 F8 Op Code Command Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn p0_f8_int(&self) -> super::vals::P0F8Int {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::P0F8Int::from_bits(val as u8)
    }
    #[doc = "Indicates PORT0 F8 Op Code Command Interrupt"]
    #[inline(always)]
    pub const fn set_p0_f8_int(&mut self, val: super::vals::P0F8Int) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Indicates PORT1 F8 Op Code Command Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn p1_f8_int(&self) -> super::vals::P1F8Int {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::P1F8Int::from_bits(val as u8)
    }
    #[doc = "Indicates PORT1 F8 Op Code Command Interrupt"]
    #[inline(always)]
    pub const fn set_p1_f8_int(&mut self, val: super::vals::P1F8Int) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Indicates dirty state of the inactive Flash for P0 filter"]
    #[must_use]
    #[inline(always)]
    pub const fn p0_dirty_int(&self) -> super::vals::P0DirtyInt {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::P0DirtyInt::from_bits(val as u8)
    }
    #[doc = "Indicates dirty state of the inactive Flash for P0 filter"]
    #[inline(always)]
    pub const fn set_p0_dirty_int(&mut self, val: super::vals::P0DirtyInt) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Indicates dirty state of the inactive Flash for P1 filter"]
    #[must_use]
    #[inline(always)]
    pub const fn p1_dirty_int(&self) -> super::vals::P1DirtyInt {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::P1DirtyInt::from_bits(val as u8)
    }
    #[doc = "Indicates dirty state of the inactive Flash for P1 filter"]
    #[inline(always)]
    pub const fn set_p1_dirty_int(&mut self, val: super::vals::P1DirtyInt) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Indicates PORT0 Blocked Op Code Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn p0_blk_int(&self) -> super::vals::P0BlkInt {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::P0BlkInt::from_bits(val as u8)
    }
    #[doc = "Indicates PORT0 Blocked Op Code Interrupt"]
    #[inline(always)]
    pub const fn set_p0_blk_int(&mut self, val: super::vals::P0BlkInt) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Indicates PORT1 Blocked Op Code Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn p1_blk_int(&self) -> super::vals::P1BlkInt {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::P1BlkInt::from_bits(val as u8)
    }
    #[doc = "Indicates PORT1 Blocked Op Code Interrupt"]
    #[inline(always)]
    pub const fn set_p1_blk_int(&mut self, val: super::vals::P1BlkInt) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Indicates PORT0 Bytemode interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn p0_bytemode_int(&self) -> super::vals::P0BytemodeInt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::P0BytemodeInt::from_bits(val as u8)
    }
    #[doc = "Indicates PORT0 Bytemode interrupt"]
    #[inline(always)]
    pub const fn set_p0_bytemode_int(&mut self, val: super::vals::P0BytemodeInt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Indicates PORT1 Bytemode interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn p1_bytemode_int(&self) -> super::vals::P1BytemodeInt {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::P1BytemodeInt::from_bits(val as u8)
    }
    #[doc = "Indicates PORT1 Bytemode interrupt"]
    #[inline(always)]
    pub const fn set_p1_bytemode_int(&mut self, val: super::vals::P1BytemodeInt) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Isr {
    #[inline(always)]
    fn default() -> Isr {
        Isr(0)
    }
}
impl core::fmt::Debug for Isr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isr")
            .field("p0_f8_int", &self.p0_f8_int())
            .field("p1_f8_int", &self.p1_f8_int())
            .field("p0_dirty_int", &self.p0_dirty_int())
            .field("p1_dirty_int", &self.p1_dirty_int())
            .field("p0_blk_int", &self.p0_blk_int())
            .field("p1_blk_int", &self.p1_blk_int())
            .field("p0_bytemode_int", &self.p0_bytemode_int())
            .field("p1_bytemode_int", &self.p1_bytemode_int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Isr {{ p0_f8_int: {:?}, p1_f8_int: {:?}, p0_dirty_int: {:?}, p1_dirty_int: {:?}, p0_blk_int: {:?}, p1_blk_int: {:?}, p0_bytemode_int: {:?}, p1_bytemode_int: {:?} }}",
            self.p0_f8_int(),
            self.p1_f8_int(),
            self.p0_dirty_int(),
            self.p1_dirty_int(),
            self.p0_blk_int(),
            self.p1_blk_int(),
            self.p0_bytemode_int(),
            self.p1_bytemode_int()
        )
    }
}
#[doc = "P0 Blocked Op Code"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0boc(pub u32);
impl P0boc {
    #[doc = "PORT0 Blocked Op Code"]
    #[must_use]
    #[inline(always)]
    pub const fn p0boc(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "PORT0 Blocked Op Code"]
    #[inline(always)]
    pub const fn set_p0boc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for P0boc {
    #[inline(always)]
    fn default() -> P0boc {
        P0boc(0)
    }
}
impl core::fmt::Debug for P0boc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0boc")
            .field("p0boc", &self.p0boc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P0boc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "P0boc {{ p0boc: {=u8:?} }}", self.p0boc())
    }
}
#[doc = "PORT0 Filter Address Region 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0far1(pub u32);
impl P0far1 {
    #[doc = "Address Lower LSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_lower_lsbs(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Address Lower LSBs"]
    #[inline(always)]
    pub const fn set_address_lower_lsbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Address Lower MSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_lower_msbs(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Address Lower MSBs"]
    #[inline(always)]
    pub const fn set_address_lower_msbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Address Upper LSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_upper_lsbs(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Address Upper LSBs"]
    #[inline(always)]
    pub const fn set_address_upper_lsbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Address Upper MSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_upper_msbs(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Address Upper MSBs"]
    #[inline(always)]
    pub const fn set_address_upper_msbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for P0far1 {
    #[inline(always)]
    fn default() -> P0far1 {
        P0far1(0)
    }
}
impl core::fmt::Debug for P0far1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0far1")
            .field("address_lower_lsbs", &self.address_lower_lsbs())
            .field("address_lower_msbs", &self.address_lower_msbs())
            .field("address_upper_lsbs", &self.address_upper_lsbs())
            .field("address_upper_msbs", &self.address_upper_msbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P0far1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P0far1 {{ address_lower_lsbs: {=u8:?}, address_lower_msbs: {=u8:?}, address_upper_lsbs: {=u8:?}, address_upper_msbs: {=u8:?} }}",
            self.address_lower_lsbs(),
            self.address_lower_msbs(),
            self.address_upper_lsbs(),
            self.address_upper_msbs()
        )
    }
}
#[doc = "PORT0 Filter Address Region 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0far2(pub u32);
impl P0far2 {
    #[doc = "Address Lower LSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_lower_lsbs(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Address Lower LSBs"]
    #[inline(always)]
    pub const fn set_address_lower_lsbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Address Lower MSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_lower_msbs(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Address Lower MSBs"]
    #[inline(always)]
    pub const fn set_address_lower_msbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Address Upper LSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_upper_lsbs(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Address Upper LSBs"]
    #[inline(always)]
    pub const fn set_address_upper_lsbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Address Upper MSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_upper_msbs(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Address Upper MSBs"]
    #[inline(always)]
    pub const fn set_address_upper_msbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for P0far2 {
    #[inline(always)]
    fn default() -> P0far2 {
        P0far2(0)
    }
}
impl core::fmt::Debug for P0far2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0far2")
            .field("address_lower_lsbs", &self.address_lower_lsbs())
            .field("address_lower_msbs", &self.address_lower_msbs())
            .field("address_upper_lsbs", &self.address_upper_lsbs())
            .field("address_upper_msbs", &self.address_upper_msbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P0far2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P0far2 {{ address_lower_lsbs: {=u8:?}, address_lower_msbs: {=u8:?}, address_upper_lsbs: {=u8:?}, address_upper_msbs: {=u8:?} }}",
            self.address_lower_lsbs(),
            self.address_lower_msbs(),
            self.address_upper_lsbs(),
            self.address_upper_msbs()
        )
    }
}
#[doc = "PORT0 Filter Address Region 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0far3(pub u32);
impl P0far3 {
    #[doc = "Address Lower LSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_lower_lsbs(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Address Lower LSBs"]
    #[inline(always)]
    pub const fn set_address_lower_lsbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Address Lower MSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_lower_msbs(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Address Lower MSBs"]
    #[inline(always)]
    pub const fn set_address_lower_msbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Address Upper LSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_upper_lsbs(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Address Upper LSBs"]
    #[inline(always)]
    pub const fn set_address_upper_lsbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Address Upper MSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_upper_msbs(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Address Upper MSBs"]
    #[inline(always)]
    pub const fn set_address_upper_msbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for P0far3 {
    #[inline(always)]
    fn default() -> P0far3 {
        P0far3(0)
    }
}
impl core::fmt::Debug for P0far3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0far3")
            .field("address_lower_lsbs", &self.address_lower_lsbs())
            .field("address_lower_msbs", &self.address_lower_msbs())
            .field("address_upper_lsbs", &self.address_upper_lsbs())
            .field("address_upper_msbs", &self.address_upper_msbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P0far3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P0far3 {{ address_lower_lsbs: {=u8:?}, address_lower_msbs: {=u8:?}, address_upper_lsbs: {=u8:?}, address_upper_msbs: {=u8:?} }}",
            self.address_lower_lsbs(),
            self.address_lower_msbs(),
            self.address_upper_lsbs(),
            self.address_upper_msbs()
        )
    }
}
#[doc = "PORT0 Filter Address Region 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0far4(pub u32);
impl P0far4 {
    #[doc = "Address Lower LSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_lower_lsbs(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Address Lower LSBs"]
    #[inline(always)]
    pub const fn set_address_lower_lsbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Address Lower MSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_lower_msbs(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Address Lower MSBs"]
    #[inline(always)]
    pub const fn set_address_lower_msbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Address Upper LSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_upper_lsbs(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Address Upper LSBs"]
    #[inline(always)]
    pub const fn set_address_upper_lsbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Address Upper MSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_upper_msbs(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Address Upper MSBs"]
    #[inline(always)]
    pub const fn set_address_upper_msbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for P0far4 {
    #[inline(always)]
    fn default() -> P0far4 {
        P0far4(0)
    }
}
impl core::fmt::Debug for P0far4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0far4")
            .field("address_lower_lsbs", &self.address_lower_lsbs())
            .field("address_lower_msbs", &self.address_lower_msbs())
            .field("address_upper_lsbs", &self.address_upper_lsbs())
            .field("address_upper_msbs", &self.address_upper_msbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P0far4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P0far4 {{ address_lower_lsbs: {=u8:?}, address_lower_msbs: {=u8:?}, address_upper_lsbs: {=u8:?}, address_upper_msbs: {=u8:?} }}",
            self.address_lower_lsbs(),
            self.address_lower_msbs(),
            self.address_upper_lsbs(),
            self.address_upper_msbs()
        )
    }
}
#[doc = "PORT0 Filter Address Region 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0far5(pub u32);
impl P0far5 {
    #[doc = "Address Lower LSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_lower_lsbs(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Address Lower LSBs"]
    #[inline(always)]
    pub const fn set_address_lower_lsbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Address Lower MSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_lower_msbs(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Address Lower MSBs"]
    #[inline(always)]
    pub const fn set_address_lower_msbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Address Upper LSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_upper_lsbs(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Address Upper LSBs"]
    #[inline(always)]
    pub const fn set_address_upper_lsbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Address Upper MSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_upper_msbs(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Address Upper MSBs"]
    #[inline(always)]
    pub const fn set_address_upper_msbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for P0far5 {
    #[inline(always)]
    fn default() -> P0far5 {
        P0far5(0)
    }
}
impl core::fmt::Debug for P0far5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0far5")
            .field("address_lower_lsbs", &self.address_lower_lsbs())
            .field("address_lower_msbs", &self.address_lower_msbs())
            .field("address_upper_lsbs", &self.address_upper_lsbs())
            .field("address_upper_msbs", &self.address_upper_msbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P0far5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P0far5 {{ address_lower_lsbs: {=u8:?}, address_lower_msbs: {=u8:?}, address_upper_lsbs: {=u8:?}, address_upper_msbs: {=u8:?} }}",
            self.address_lower_lsbs(),
            self.address_lower_msbs(),
            self.address_upper_lsbs(),
            self.address_upper_msbs()
        )
    }
}
#[doc = "PORT0 Filter Address Region 6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0far6(pub u32);
impl P0far6 {
    #[doc = "Address Lower LSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_lower_lsbs(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Address Lower LSBs"]
    #[inline(always)]
    pub const fn set_address_lower_lsbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Address Lower MSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_lower_msbs(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Address Lower MSBs"]
    #[inline(always)]
    pub const fn set_address_lower_msbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Address Upper LSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_upper_lsbs(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Address Upper LSBs"]
    #[inline(always)]
    pub const fn set_address_upper_lsbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Address Upper MSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_upper_msbs(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Address Upper MSBs"]
    #[inline(always)]
    pub const fn set_address_upper_msbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for P0far6 {
    #[inline(always)]
    fn default() -> P0far6 {
        P0far6(0)
    }
}
impl core::fmt::Debug for P0far6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0far6")
            .field("address_lower_lsbs", &self.address_lower_lsbs())
            .field("address_lower_msbs", &self.address_lower_msbs())
            .field("address_upper_lsbs", &self.address_upper_lsbs())
            .field("address_upper_msbs", &self.address_upper_msbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P0far6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P0far6 {{ address_lower_lsbs: {=u8:?}, address_lower_msbs: {=u8:?}, address_upper_lsbs: {=u8:?}, address_upper_msbs: {=u8:?} }}",
            self.address_lower_lsbs(),
            self.address_lower_msbs(),
            self.address_upper_lsbs(),
            self.address_upper_msbs()
        )
    }
}
#[doc = "PORT0 Max Address Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0mam(pub u32);
impl P0mam {
    #[doc = "PORT0 Max Address Mask LSB"]
    #[must_use]
    #[inline(always)]
    pub const fn p0maml(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "PORT0 Max Address Mask LSB"]
    #[inline(always)]
    pub const fn set_p0maml(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "PORT0 Max Address Mask MSB"]
    #[must_use]
    #[inline(always)]
    pub const fn p0mamm(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "PORT0 Max Address Mask MSB"]
    #[inline(always)]
    pub const fn set_p0mamm(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for P0mam {
    #[inline(always)]
    fn default() -> P0mam {
        P0mam(0)
    }
}
impl core::fmt::Debug for P0mam {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0mam")
            .field("p0maml", &self.p0maml())
            .field("p0mamm", &self.p0mamm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P0mam {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P0mam {{ p0maml: {=u8:?}, p0mamm: {=u8:?} }}",
            self.p0maml(),
            self.p0mamm()
        )
    }
}
#[doc = "P1 Blocked Op Code"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1boc(pub u32);
impl P1boc {
    #[doc = "PORT1 Blocked Op Code"]
    #[must_use]
    #[inline(always)]
    pub const fn p1boc(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "PORT1 Blocked Op Code"]
    #[inline(always)]
    pub const fn set_p1boc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for P1boc {
    #[inline(always)]
    fn default() -> P1boc {
        P1boc(0)
    }
}
impl core::fmt::Debug for P1boc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1boc")
            .field("p1boc", &self.p1boc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P1boc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "P1boc {{ p1boc: {=u8:?} }}", self.p1boc())
    }
}
#[doc = "PORT1 Filter Address Region 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1far1(pub u32);
impl P1far1 {
    #[doc = "Address Lower LSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_lower_lsbs(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Address Lower LSBs"]
    #[inline(always)]
    pub const fn set_address_lower_lsbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Address Lower MSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_lower_msbs(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Address Lower MSBs"]
    #[inline(always)]
    pub const fn set_address_lower_msbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Address Upper LSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_upper_lsbs(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Address Upper LSBs"]
    #[inline(always)]
    pub const fn set_address_upper_lsbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Address Upper MSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_upper_msbs(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Address Upper MSBs"]
    #[inline(always)]
    pub const fn set_address_upper_msbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for P1far1 {
    #[inline(always)]
    fn default() -> P1far1 {
        P1far1(0)
    }
}
impl core::fmt::Debug for P1far1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1far1")
            .field("address_lower_lsbs", &self.address_lower_lsbs())
            .field("address_lower_msbs", &self.address_lower_msbs())
            .field("address_upper_lsbs", &self.address_upper_lsbs())
            .field("address_upper_msbs", &self.address_upper_msbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P1far1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P1far1 {{ address_lower_lsbs: {=u8:?}, address_lower_msbs: {=u8:?}, address_upper_lsbs: {=u8:?}, address_upper_msbs: {=u8:?} }}",
            self.address_lower_lsbs(),
            self.address_lower_msbs(),
            self.address_upper_lsbs(),
            self.address_upper_msbs()
        )
    }
}
#[doc = "PORT1 Filter Address Region 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1far2(pub u32);
impl P1far2 {
    #[doc = "Address Lower LSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_lower_lsbs(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Address Lower LSBs"]
    #[inline(always)]
    pub const fn set_address_lower_lsbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Address Lower MSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_lower_msbs(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Address Lower MSBs"]
    #[inline(always)]
    pub const fn set_address_lower_msbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Address Upper LSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_upper_lsbs(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Address Upper LSBs"]
    #[inline(always)]
    pub const fn set_address_upper_lsbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Address Upper MSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_upper_msbs(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Address Upper MSBs"]
    #[inline(always)]
    pub const fn set_address_upper_msbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for P1far2 {
    #[inline(always)]
    fn default() -> P1far2 {
        P1far2(0)
    }
}
impl core::fmt::Debug for P1far2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1far2")
            .field("address_lower_lsbs", &self.address_lower_lsbs())
            .field("address_lower_msbs", &self.address_lower_msbs())
            .field("address_upper_lsbs", &self.address_upper_lsbs())
            .field("address_upper_msbs", &self.address_upper_msbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P1far2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P1far2 {{ address_lower_lsbs: {=u8:?}, address_lower_msbs: {=u8:?}, address_upper_lsbs: {=u8:?}, address_upper_msbs: {=u8:?} }}",
            self.address_lower_lsbs(),
            self.address_lower_msbs(),
            self.address_upper_lsbs(),
            self.address_upper_msbs()
        )
    }
}
#[doc = "PORT1 Filter Address Region 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1far3(pub u32);
impl P1far3 {
    #[doc = "Address Lower LSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_lower_lsbs(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Address Lower LSBs"]
    #[inline(always)]
    pub const fn set_address_lower_lsbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Address Lower MSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_lower_msbs(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Address Lower MSBs"]
    #[inline(always)]
    pub const fn set_address_lower_msbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Address Upper LSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_upper_lsbs(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Address Upper LSBs"]
    #[inline(always)]
    pub const fn set_address_upper_lsbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Address Upper MSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_upper_msbs(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Address Upper MSBs"]
    #[inline(always)]
    pub const fn set_address_upper_msbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for P1far3 {
    #[inline(always)]
    fn default() -> P1far3 {
        P1far3(0)
    }
}
impl core::fmt::Debug for P1far3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1far3")
            .field("address_lower_lsbs", &self.address_lower_lsbs())
            .field("address_lower_msbs", &self.address_lower_msbs())
            .field("address_upper_lsbs", &self.address_upper_lsbs())
            .field("address_upper_msbs", &self.address_upper_msbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P1far3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P1far3 {{ address_lower_lsbs: {=u8:?}, address_lower_msbs: {=u8:?}, address_upper_lsbs: {=u8:?}, address_upper_msbs: {=u8:?} }}",
            self.address_lower_lsbs(),
            self.address_lower_msbs(),
            self.address_upper_lsbs(),
            self.address_upper_msbs()
        )
    }
}
#[doc = "PORT1 Filter Address Region 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1far4(pub u32);
impl P1far4 {
    #[doc = "Address Lower LSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_lower_lsbs(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Address Lower LSBs"]
    #[inline(always)]
    pub const fn set_address_lower_lsbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Address Lower MSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_lower_msbs(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Address Lower MSBs"]
    #[inline(always)]
    pub const fn set_address_lower_msbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Address Upper LSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_upper_lsbs(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Address Upper LSBs"]
    #[inline(always)]
    pub const fn set_address_upper_lsbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Address Upper MSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_upper_msbs(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Address Upper MSBs"]
    #[inline(always)]
    pub const fn set_address_upper_msbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for P1far4 {
    #[inline(always)]
    fn default() -> P1far4 {
        P1far4(0)
    }
}
impl core::fmt::Debug for P1far4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1far4")
            .field("address_lower_lsbs", &self.address_lower_lsbs())
            .field("address_lower_msbs", &self.address_lower_msbs())
            .field("address_upper_lsbs", &self.address_upper_lsbs())
            .field("address_upper_msbs", &self.address_upper_msbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P1far4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P1far4 {{ address_lower_lsbs: {=u8:?}, address_lower_msbs: {=u8:?}, address_upper_lsbs: {=u8:?}, address_upper_msbs: {=u8:?} }}",
            self.address_lower_lsbs(),
            self.address_lower_msbs(),
            self.address_upper_lsbs(),
            self.address_upper_msbs()
        )
    }
}
#[doc = "PORT1 Filter Address Region 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1far5(pub u32);
impl P1far5 {
    #[doc = "Address Lower LSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_lower_lsbs(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Address Lower LSBs"]
    #[inline(always)]
    pub const fn set_address_lower_lsbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Address Lower MSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_lower_msbs(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Address Lower MSBs"]
    #[inline(always)]
    pub const fn set_address_lower_msbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Address Upper LSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_upper_lsbs(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Address Upper LSBs"]
    #[inline(always)]
    pub const fn set_address_upper_lsbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Address Upper MSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_upper_msbs(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Address Upper MSBs"]
    #[inline(always)]
    pub const fn set_address_upper_msbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for P1far5 {
    #[inline(always)]
    fn default() -> P1far5 {
        P1far5(0)
    }
}
impl core::fmt::Debug for P1far5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1far5")
            .field("address_lower_lsbs", &self.address_lower_lsbs())
            .field("address_lower_msbs", &self.address_lower_msbs())
            .field("address_upper_lsbs", &self.address_upper_lsbs())
            .field("address_upper_msbs", &self.address_upper_msbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P1far5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P1far5 {{ address_lower_lsbs: {=u8:?}, address_lower_msbs: {=u8:?}, address_upper_lsbs: {=u8:?}, address_upper_msbs: {=u8:?} }}",
            self.address_lower_lsbs(),
            self.address_lower_msbs(),
            self.address_upper_lsbs(),
            self.address_upper_msbs()
        )
    }
}
#[doc = "PORT1 Filter Address Region 6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1far6(pub u32);
impl P1far6 {
    #[doc = "Address Lower LSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_lower_lsbs(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Address Lower LSBs"]
    #[inline(always)]
    pub const fn set_address_lower_lsbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Address Lower MSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_lower_msbs(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Address Lower MSBs"]
    #[inline(always)]
    pub const fn set_address_lower_msbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Address Upper LSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_upper_lsbs(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Address Upper LSBs"]
    #[inline(always)]
    pub const fn set_address_upper_lsbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Address Upper MSBs"]
    #[must_use]
    #[inline(always)]
    pub const fn address_upper_msbs(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Address Upper MSBs"]
    #[inline(always)]
    pub const fn set_address_upper_msbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for P1far6 {
    #[inline(always)]
    fn default() -> P1far6 {
        P1far6(0)
    }
}
impl core::fmt::Debug for P1far6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1far6")
            .field("address_lower_lsbs", &self.address_lower_lsbs())
            .field("address_lower_msbs", &self.address_lower_msbs())
            .field("address_upper_lsbs", &self.address_upper_lsbs())
            .field("address_upper_msbs", &self.address_upper_msbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P1far6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P1far6 {{ address_lower_lsbs: {=u8:?}, address_lower_msbs: {=u8:?}, address_upper_lsbs: {=u8:?}, address_upper_msbs: {=u8:?} }}",
            self.address_lower_lsbs(),
            self.address_lower_msbs(),
            self.address_upper_lsbs(),
            self.address_upper_msbs()
        )
    }
}
#[doc = "PORT1 Max Address Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1mam(pub u32);
impl P1mam {
    #[doc = "PORT1 Max Address Mask LSB"]
    #[must_use]
    #[inline(always)]
    pub const fn p1maml(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "PORT1 Max Address Mask LSB"]
    #[inline(always)]
    pub const fn set_p1maml(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "PORT1 Max Address Mask MSB"]
    #[must_use]
    #[inline(always)]
    pub const fn p1mamm(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "PORT1 Max Address Mask MSB"]
    #[inline(always)]
    pub const fn set_p1mamm(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for P1mam {
    #[inline(always)]
    fn default() -> P1mam {
        P1mam(0)
    }
}
impl core::fmt::Debug for P1mam {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1mam")
            .field("p1maml", &self.p1maml())
            .field("p1mamm", &self.p1mamm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for P1mam {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "P1mam {{ p1maml: {=u8:?}, p1mamm: {=u8:?} }}",
            self.p1maml(),
            self.p1mamm()
        )
    }
}
#[doc = "Programmable OP Code0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Popcode0(pub u32);
impl Popcode0 {
    #[doc = "Programmable Filter state 0"]
    #[must_use]
    #[inline(always)]
    pub const fn filter_state0(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Programmable Filter state 0"]
    #[inline(always)]
    pub const fn set_filter_state0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "This bit determines whether the Programmable Op Code filter is executed after receipt of the 7th or 8th bit of the Op Code"]
    #[must_use]
    #[inline(always)]
    pub const fn filter_bit0(&self) -> super::vals::FilterBit0 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::FilterBit0::from_bits(val as u8)
    }
    #[doc = "This bit determines whether the Programmable Op Code filter is executed after receipt of the 7th or 8th bit of the Op Code"]
    #[inline(always)]
    pub const fn set_filter_bit0(&mut self, val: super::vals::FilterBit0) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Programmable OP Code 0"]
    #[must_use]
    #[inline(always)]
    pub const fn prg_opcode0(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Programmable OP Code 0"]
    #[inline(always)]
    pub const fn set_prg_opcode0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Popcode0 {
    #[inline(always)]
    fn default() -> Popcode0 {
        Popcode0(0)
    }
}
impl core::fmt::Debug for Popcode0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Popcode0")
            .field("filter_state0", &self.filter_state0())
            .field("filter_bit0", &self.filter_bit0())
            .field("prg_opcode0", &self.prg_opcode0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Popcode0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Popcode0 {{ filter_state0: {=u8:?}, filter_bit0: {:?}, prg_opcode0: {=u8:?} }}",
            self.filter_state0(),
            self.filter_bit0(),
            self.prg_opcode0()
        )
    }
}
#[doc = "Programmable OP Code1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Popcode1(pub u32);
impl Popcode1 {
    #[doc = "Programmable Filter state 1"]
    #[must_use]
    #[inline(always)]
    pub const fn filter_state1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Programmable Filter state 1"]
    #[inline(always)]
    pub const fn set_filter_state1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "This bit determines whether the Programmable Op Code filter is executed after receipt of the 7th or 8th bit of the Op Code"]
    #[must_use]
    #[inline(always)]
    pub const fn filter_bit1(&self) -> super::vals::FilterBit1 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::FilterBit1::from_bits(val as u8)
    }
    #[doc = "This bit determines whether the Programmable Op Code filter is executed after receipt of the 7th or 8th bit of the Op Code"]
    #[inline(always)]
    pub const fn set_filter_bit1(&mut self, val: super::vals::FilterBit1) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Programmable OP Code 1"]
    #[must_use]
    #[inline(always)]
    pub const fn prg_opcode1(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Programmable OP Code 1"]
    #[inline(always)]
    pub const fn set_prg_opcode1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Popcode1 {
    #[inline(always)]
    fn default() -> Popcode1 {
        Popcode1(0)
    }
}
impl core::fmt::Debug for Popcode1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Popcode1")
            .field("filter_state1", &self.filter_state1())
            .field("filter_bit1", &self.filter_bit1())
            .field("prg_opcode1", &self.prg_opcode1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Popcode1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Popcode1 {{ filter_state1: {=u8:?}, filter_bit1: {:?}, prg_opcode1: {=u8:?} }}",
            self.filter_state1(),
            self.filter_bit1(),
            self.prg_opcode1()
        )
    }
}
#[doc = "Programmable OP Code2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Popcode2(pub u32);
impl Popcode2 {
    #[doc = "Programmable Filter state 2"]
    #[must_use]
    #[inline(always)]
    pub const fn filter_state2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Programmable Filter state 2"]
    #[inline(always)]
    pub const fn set_filter_state2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "This bit determines whether the Programmable Op Code filter is executed after receipt of the 7th or 8th bit of the Op Code"]
    #[must_use]
    #[inline(always)]
    pub const fn filter_bit2(&self) -> super::vals::FilterBit2 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::FilterBit2::from_bits(val as u8)
    }
    #[doc = "This bit determines whether the Programmable Op Code filter is executed after receipt of the 7th or 8th bit of the Op Code"]
    #[inline(always)]
    pub const fn set_filter_bit2(&mut self, val: super::vals::FilterBit2) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Programmable OP Code 0"]
    #[must_use]
    #[inline(always)]
    pub const fn prg_opcode2(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Programmable OP Code 0"]
    #[inline(always)]
    pub const fn set_prg_opcode2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Popcode2 {
    #[inline(always)]
    fn default() -> Popcode2 {
        Popcode2(0)
    }
}
impl core::fmt::Debug for Popcode2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Popcode2")
            .field("filter_state2", &self.filter_state2())
            .field("filter_bit2", &self.filter_bit2())
            .field("prg_opcode2", &self.prg_opcode2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Popcode2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Popcode2 {{ filter_state2: {=u8:?}, filter_bit2: {:?}, prg_opcode2: {=u8:?} }}",
            self.filter_state2(),
            self.filter_bit2(),
            self.prg_opcode2()
        )
    }
}
#[doc = "Programmable OP Code3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Popcode3(pub u32);
impl Popcode3 {
    #[doc = "Programmable Filter state 3"]
    #[must_use]
    #[inline(always)]
    pub const fn filter_state3(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Programmable Filter state 3"]
    #[inline(always)]
    pub const fn set_filter_state3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "This bit determines whether the Programmable Op Code filter is executed after receipt of the 7th or 8th bit of the Op Code"]
    #[must_use]
    #[inline(always)]
    pub const fn filter_bit3(&self) -> super::vals::FilterBit3 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::FilterBit3::from_bits(val as u8)
    }
    #[doc = "This bit determines whether the Programmable Op Code filter is executed after receipt of the 7th or 8th bit of the Op Code"]
    #[inline(always)]
    pub const fn set_filter_bit3(&mut self, val: super::vals::FilterBit3) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Programmable OP Code 3"]
    #[must_use]
    #[inline(always)]
    pub const fn prg_opcode3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Programmable OP Code 3"]
    #[inline(always)]
    pub const fn set_prg_opcode3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Popcode3 {
    #[inline(always)]
    fn default() -> Popcode3 {
        Popcode3(0)
    }
}
impl core::fmt::Debug for Popcode3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Popcode3")
            .field("filter_state3", &self.filter_state3())
            .field("filter_bit3", &self.filter_bit3())
            .field("prg_opcode3", &self.prg_opcode3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Popcode3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Popcode3 {{ filter_state3: {=u8:?}, filter_bit3: {:?}, prg_opcode3: {=u8:?} }}",
            self.filter_state3(),
            self.filter_bit3(),
            self.prg_opcode3()
        )
    }
}
#[doc = "Programmable OP Code4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Popcode4(pub u32);
impl Popcode4 {
    #[doc = "Programmable Filter state 4"]
    #[must_use]
    #[inline(always)]
    pub const fn filter_state4(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Programmable Filter state 4"]
    #[inline(always)]
    pub const fn set_filter_state4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "This bit determines whether the Programmable Op Code filter is executed after receipt of the 7th or 8th bit of the Op Code"]
    #[must_use]
    #[inline(always)]
    pub const fn filter_bit4(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "This bit determines whether the Programmable Op Code filter is executed after receipt of the 7th or 8th bit of the Op Code"]
    #[inline(always)]
    pub const fn set_filter_bit4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Programmable OP Code 4"]
    #[must_use]
    #[inline(always)]
    pub const fn prg_opcode4(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Programmable OP Code 4"]
    #[inline(always)]
    pub const fn set_prg_opcode4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Popcode4 {
    #[inline(always)]
    fn default() -> Popcode4 {
        Popcode4(0)
    }
}
impl core::fmt::Debug for Popcode4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Popcode4")
            .field("filter_state4", &self.filter_state4())
            .field("filter_bit4", &self.filter_bit4())
            .field("prg_opcode4", &self.prg_opcode4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Popcode4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Popcode4 {{ filter_state4: {=u8:?}, filter_bit4: {=bool:?}, prg_opcode4: {=u8:?} }}",
            self.filter_state4(),
            self.filter_bit4(),
            self.prg_opcode4()
        )
    }
}
#[doc = "Port Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Indicates the Dirty State of the P1 inactive Flash"]
    #[must_use]
    #[inline(always)]
    pub const fn p1_dirty(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the Dirty State of the P1 inactive Flash"]
    #[inline(always)]
    pub const fn set_p1_dirty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Indicates the P1 Address Byte Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn p1_byte_mode(&self) -> super::vals::P1ByteMode {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::P1ByteMode::from_bits(val as u8)
    }
    #[doc = "Indicates the P1 Address Byte Mode"]
    #[inline(always)]
    pub const fn set_p1_byte_mode(&mut self, val: super::vals::P1ByteMode) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Write enable command status"]
    #[must_use]
    #[inline(always)]
    pub const fn p1_write_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Write enable command status"]
    #[inline(always)]
    pub const fn set_p1_write_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Indicates the Dirty State of the P0 inactive Flash"]
    #[must_use]
    #[inline(always)]
    pub const fn p0_dirty(&self) -> super::vals::P0Dirty {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::P0Dirty::from_bits(val as u8)
    }
    #[doc = "Indicates the Dirty State of the P0 inactive Flash"]
    #[inline(always)]
    pub const fn set_p0_dirty(&mut self, val: super::vals::P0Dirty) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Indicates the P0 Address Byte Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn p0_byte_mode(&self) -> super::vals::P0ByteMode {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::P0ByteMode::from_bits(val as u8)
    }
    #[doc = "Indicates the P0 Address Byte Mode"]
    #[inline(always)]
    pub const fn set_p0_byte_mode(&mut self, val: super::vals::P0ByteMode) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Write enable command status"]
    #[must_use]
    #[inline(always)]
    pub const fn p0_write_en(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Write enable command status"]
    #[inline(always)]
    pub const fn set_p0_write_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
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
            .field("p1_dirty", &self.p1_dirty())
            .field("p1_byte_mode", &self.p1_byte_mode())
            .field("p1_write_en", &self.p1_write_en())
            .field("p0_dirty", &self.p0_dirty())
            .field("p0_byte_mode", &self.p0_byte_mode())
            .field("p0_write_en", &self.p0_write_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sr {{ p1_dirty: {=bool:?}, p1_byte_mode: {:?}, p1_write_en: {=bool:?}, p0_dirty: {:?}, p0_byte_mode: {:?}, p0_write_en: {=bool:?} }}",
            self.p1_dirty(),
            self.p1_byte_mode(),
            self.p1_write_en(),
            self.p0_dirty(),
            self.p0_byte_mode(),
            self.p0_write_en()
        )
    }
}
#[doc = "Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tr(pub u32);
impl Tr {
    #[doc = "Selects the active CS when PO_BYP_EN is set"]
    #[must_use]
    #[inline(always)]
    pub const fn p0_byp_sel(&self) -> super::vals::P0BypSel {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::P0BypSel::from_bits(val as u8)
    }
    #[doc = "Selects the active CS when PO_BYP_EN is set"]
    #[inline(always)]
    pub const fn set_p0_byp_sel(&mut self, val: super::vals::P0BypSel) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Enables bypassing of the filter logic"]
    #[must_use]
    #[inline(always)]
    pub const fn p0_byp_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Enables bypassing of the filter logic"]
    #[inline(always)]
    pub const fn set_p0_byp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Selects the active CS when P1_BYP_EN is set"]
    #[must_use]
    #[inline(always)]
    pub const fn p1_byp_sel(&self) -> super::vals::P1BypSel {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::P1BypSel::from_bits(val as u8)
    }
    #[doc = "Selects the active CS when P1_BYP_EN is set"]
    #[inline(always)]
    pub const fn set_p1_byp_sel(&mut self, val: super::vals::P1BypSel) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables bypassing of the filter logic"]
    #[must_use]
    #[inline(always)]
    pub const fn p1_byp_en(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enables bypassing of the filter logic"]
    #[inline(always)]
    pub const fn set_p1_byp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables bypassing of the synchronization logic of CS on negedge of SCLK"]
    #[must_use]
    #[inline(always)]
    pub const fn p0_ff_byp_en(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enables bypassing of the synchronization logic of CS on negedge of SCLK"]
    #[inline(always)]
    pub const fn set_p0_ff_byp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enables bypassing of the synchronization logic of CS on negedge of SCLK"]
    #[must_use]
    #[inline(always)]
    pub const fn p1_ff_byp_en(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables bypassing of the synchronization logic of CS on negedge of SCLK"]
    #[inline(always)]
    pub const fn set_p1_ff_byp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Sets the Flash memory manufacturer"]
    #[must_use]
    #[inline(always)]
    pub const fn p0_mfg_id(&self) -> super::vals::P0MfgId {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::P0MfgId::from_bits(val as u8)
    }
    #[doc = "Sets the Flash memory manufacturer"]
    #[inline(always)]
    pub const fn set_p0_mfg_id(&mut self, val: super::vals::P0MfgId) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "Sets the Flash memory manufacturer"]
    #[must_use]
    #[inline(always)]
    pub const fn p1_mfg_id(&self) -> super::vals::P1MfgId {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::P1MfgId::from_bits(val as u8)
    }
    #[doc = "Sets the Flash memory manufacturer"]
    #[inline(always)]
    pub const fn set_p1_mfg_id(&mut self, val: super::vals::P1MfgId) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Tr {
    #[inline(always)]
    fn default() -> Tr {
        Tr(0)
    }
}
impl core::fmt::Debug for Tr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tr")
            .field("p0_byp_sel", &self.p0_byp_sel())
            .field("p0_byp_en", &self.p0_byp_en())
            .field("p1_byp_sel", &self.p1_byp_sel())
            .field("p1_byp_en", &self.p1_byp_en())
            .field("p0_ff_byp_en", &self.p0_ff_byp_en())
            .field("p1_ff_byp_en", &self.p1_ff_byp_en())
            .field("p0_mfg_id", &self.p0_mfg_id())
            .field("p1_mfg_id", &self.p1_mfg_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tr {{ p0_byp_sel: {:?}, p0_byp_en: {=bool:?}, p1_byp_sel: {:?}, p1_byp_en: {=bool:?}, p0_ff_byp_en: {=bool:?}, p1_ff_byp_en: {=bool:?}, p0_mfg_id: {:?}, p1_mfg_id: {:?} }}",
            self.p0_byp_sel(),
            self.p0_byp_en(),
            self.p1_byp_sel(),
            self.p1_byp_en(),
            self.p0_ff_byp_en(),
            self.p1_ff_byp_en(),
            self.p0_mfg_id(),
            self.p1_mfg_id()
        )
    }
}
