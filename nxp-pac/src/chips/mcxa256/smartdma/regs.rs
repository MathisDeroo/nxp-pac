#[doc = "ARM to EZH Interrupt Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Arm2ezh(pub u32);
impl Arm2ezh {
    #[doc = "Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ie(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ie(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "General purpose register bits"]
    #[must_use]
    #[inline(always)]
    pub const fn gp(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "General purpose register bits"]
    #[inline(always)]
    pub const fn set_gp(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for Arm2ezh {
    #[inline(always)]
    fn default() -> Arm2ezh {
        Arm2ezh(0)
    }
}
impl core::fmt::Debug for Arm2ezh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Arm2ezh")
            .field("ie", &self.ie())
            .field("gp", &self.gp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Arm2ezh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Arm2ezh {{ ie: {=u8:?}, gp: {=u32:?} }}",
            self.ie(),
            self.gp()
        )
    }
}
#[doc = "Boot Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootadr(pub u32);
impl Bootadr {
    #[doc = "32-bit boot address, the boot address should be 4-byte aligned."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "32-bit boot address, the boot address should be 4-byte aligned."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for Bootadr {
    #[inline(always)]
    fn default() -> Bootadr {
        Bootadr(0)
    }
}
impl core::fmt::Debug for Bootadr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bootadr")
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bootadr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bootadr {{ addr: {=u32:?} }}", self.addr())
    }
}
#[doc = "Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Start Bit Ignition"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Start Bit Ignition"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "External Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn exf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "External Flag"]
    #[inline(always)]
    pub const fn set_exf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Error Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn errdis(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Error Disable"]
    #[inline(always)]
    pub const fn set_errdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Buffer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bufen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer Enable"]
    #[inline(always)]
    pub const fn set_bufen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Sync Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn syncen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Sync Enable"]
    #[inline(always)]
    pub const fn set_syncen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write Key"]
    #[must_use]
    #[inline(always)]
    pub const fn wkey(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Write Key"]
    #[inline(always)]
    pub const fn set_wkey(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("start", &self.start())
            .field("exf", &self.exf())
            .field("errdis", &self.errdis())
            .field("bufen", &self.bufen())
            .field("syncen", &self.syncen())
            .field("wkey", &self.wkey())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ start: {=bool:?}, exf: {=bool:?}, errdis: {=bool:?}, bufen: {=bool:?}, syncen: {=bool:?}, wkey: {=u16:?} }}",
            self.start(),
            self.exf(),
            self.errdis(),
            self.bufen(),
            self.syncen(),
            self.wkey()
        )
    }
}
#[doc = "EZH to ARM Trigger"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ezh2arm(pub u32);
impl Ezh2arm {
    #[doc = "General purpose register bits Writing to EZH2ARM triggers the ARM interrupt when ARM2EZH \\[1:0\\] == 2h"]
    #[must_use]
    #[inline(always)]
    pub const fn gp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "General purpose register bits Writing to EZH2ARM triggers the ARM interrupt when ARM2EZH \\[1:0\\] == 2h"]
    #[inline(always)]
    pub const fn set_gp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ezh2arm {
    #[inline(always)]
    fn default() -> Ezh2arm {
        Ezh2arm(0)
    }
}
impl core::fmt::Debug for Ezh2arm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ezh2arm").field("gp", &self.gp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ezh2arm {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ezh2arm {{ gp: {=u32:?} }}", self.gp())
    }
}
#[doc = "Program Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pc(pub u32);
impl Pc {
    #[doc = "Program Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn pc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Program Counter"]
    #[inline(always)]
    pub const fn set_pc(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pc {
    #[inline(always)]
    fn default() -> Pc {
        Pc(0)
    }
}
impl core::fmt::Debug for Pc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pc").field("pc", &self.pc()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pc {{ pc: {=u32:?} }}", self.pc())
    }
}
#[doc = "Pending Trap Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pendtrap(pub u32);
impl Pendtrap {
    #[doc = "Status Flag or Pending Trap Request"]
    #[must_use]
    #[inline(always)]
    pub const fn status(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Status Flag or Pending Trap Request"]
    #[inline(always)]
    pub const fn set_status(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pol(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Polarity"]
    #[inline(always)]
    pub const fn set_pol(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Enable Pending Trap"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Enable Pending Trap"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Pendtrap {
    #[inline(always)]
    fn default() -> Pendtrap {
        Pendtrap(0)
    }
}
impl core::fmt::Debug for Pendtrap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pendtrap")
            .field("status", &self.status())
            .field("pol", &self.pol())
            .field("en", &self.en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pendtrap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pendtrap {{ status: {=u8:?}, pol: {=u8:?}, en: {=u8:?} }}",
            self.status(),
            self.pol(),
            self.en()
        )
    }
}
#[doc = "Stack Pointer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sp(pub u32);
impl Sp {
    #[doc = "Stack Pointer"]
    #[must_use]
    #[inline(always)]
    pub const fn sp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Stack Pointer"]
    #[inline(always)]
    pub const fn set_sp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Sp {
    #[inline(always)]
    fn default() -> Sp {
        Sp(0)
    }
}
impl core::fmt::Debug for Sp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sp").field("sp", &self.sp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sp {{ sp: {=u32:?} }}", self.sp())
    }
}
