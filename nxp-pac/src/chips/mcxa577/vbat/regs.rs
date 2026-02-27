#[doc = "FRO16K Clock Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Froclke(pub u32);
impl Froclke {
    #[doc = "Clock Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn clke(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Clock Enable"]
    #[inline(always)]
    pub const fn set_clke(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Froclke {
    #[inline(always)]
    fn default() -> Froclke {
        Froclke(0)
    }
}
impl core::fmt::Debug for Froclke {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Froclke")
            .field("clke", &self.clke())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Froclke {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Froclke {{ clke: {=u8:?} }}", self.clke())
    }
}
#[doc = "FRO16K Control A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Froctla(pub u32);
impl Froctla {
    #[doc = "FRO16K Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fro_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FRO16K Enable"]
    #[inline(always)]
    pub const fn set_fro_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Froctla {
    #[inline(always)]
    fn default() -> Froctla {
        Froctla(0)
    }
}
impl core::fmt::Debug for Froctla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Froctla")
            .field("fro_en", &self.fro_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Froctla {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Froctla {{ fro_en: {=bool:?} }}", self.fro_en())
    }
}
#[doc = "FRO16K Lock A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frolcka(pub u32);
impl Frolcka {
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Frolcka {
    #[inline(always)]
    fn default() -> Frolcka {
        Frolcka(0)
    }
}
impl core::fmt::Debug for Frolcka {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frolcka")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frolcka {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Frolcka {{ lock: {=bool:?} }}", self.lock())
    }
}
#[doc = "Interrupt Enable A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irqena(pub u32);
impl Irqena {
    #[doc = "POR Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn por_det(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "POR Detect"]
    #[inline(always)]
    pub const fn set_por_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Wakeup Pin Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_flag(&self) -> super::vals::IrqenaWakeupFlag {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::IrqenaWakeupFlag::from_bits(val as u8)
    }
    #[doc = "Wakeup Pin Flag"]
    #[inline(always)]
    pub const fn set_wakeup_flag(&mut self, val: super::vals::IrqenaWakeupFlag) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Bandgap Timer 0"]
    #[must_use]
    #[inline(always)]
    pub const fn timer0_flag(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Timer 0"]
    #[inline(always)]
    pub const fn set_timer0_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Bandgap Timer 2"]
    #[must_use]
    #[inline(always)]
    pub const fn timer1_flag(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Timer 2"]
    #[inline(always)]
    pub const fn set_timer1_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "LDO Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn ldo_rdy(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "LDO Ready"]
    #[inline(always)]
    pub const fn set_ldo_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "OSC32k Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_rdy(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OSC32k Ready"]
    #[inline(always)]
    pub const fn set_osc_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Clock Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn clock_det(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Detect"]
    #[inline(always)]
    pub const fn set_clock_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt 0 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn irq0_det(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt 0 Detect"]
    #[inline(always)]
    pub const fn set_irq0_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Interrupt 1 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn irq1_det(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt 1 Detect"]
    #[inline(always)]
    pub const fn set_irq1_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Interrupt 2 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn irq2_det(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt 2 Detect"]
    #[inline(always)]
    pub const fn set_irq2_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Interrupt 3 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn irq3_det(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt 3 Detect"]
    #[inline(always)]
    pub const fn set_irq3_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Irqena {
    #[inline(always)]
    fn default() -> Irqena {
        Irqena(0)
    }
}
impl core::fmt::Debug for Irqena {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Irqena")
            .field("por_det", &self.por_det())
            .field("wakeup_flag", &self.wakeup_flag())
            .field("timer0_flag", &self.timer0_flag())
            .field("timer1_flag", &self.timer1_flag())
            .field("ldo_rdy", &self.ldo_rdy())
            .field("osc_rdy", &self.osc_rdy())
            .field("clock_det", &self.clock_det())
            .field("irq0_det", &self.irq0_det())
            .field("irq1_det", &self.irq1_det())
            .field("irq2_det", &self.irq2_det())
            .field("irq3_det", &self.irq3_det())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Irqena {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Irqena {{ por_det: {=bool:?}, wakeup_flag: {:?}, timer0_flag: {=bool:?}, timer1_flag: {=bool:?}, ldo_rdy: {=bool:?}, osc_rdy: {=bool:?}, clock_det: {=bool:?}, irq0_det: {=bool:?}, irq1_det: {=bool:?}, irq2_det: {=bool:?}, irq3_det: {=bool:?} }}",
            self.por_det(),
            self.wakeup_flag(),
            self.timer0_flag(),
            self.timer1_flag(),
            self.ldo_rdy(),
            self.osc_rdy(),
            self.clock_det(),
            self.irq0_det(),
            self.irq1_det(),
            self.irq2_det(),
            self.irq3_det()
        )
    }
}
#[doc = "LDO_RAM Control A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldoctla(pub u32);
impl Ldoctla {
    #[doc = "Bandgap Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bg_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Enable"]
    #[inline(always)]
    pub const fn set_bg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LDO Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ldo_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LDO Enable"]
    #[inline(always)]
    pub const fn set_ldo_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Refresh Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn refresh_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Refresh Enable"]
    #[inline(always)]
    pub const fn set_refresh_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Ldoctla {
    #[inline(always)]
    fn default() -> Ldoctla {
        Ldoctla(0)
    }
}
impl core::fmt::Debug for Ldoctla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ldoctla")
            .field("bg_en", &self.bg_en())
            .field("ldo_en", &self.ldo_en())
            .field("refresh_en", &self.refresh_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ldoctla {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ldoctla {{ bg_en: {=bool:?}, ldo_en: {=bool:?}, refresh_en: {=bool:?} }}",
            self.bg_en(),
            self.ldo_en(),
            self.refresh_en()
        )
    }
}
#[doc = "LDO_RAM Lock A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldolcka(pub u32);
impl Ldolcka {
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Ldolcka {
    #[inline(always)]
    fn default() -> Ldolcka {
        Ldolcka(0)
    }
}
impl core::fmt::Debug for Ldolcka {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ldolcka")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ldolcka {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ldolcka {{ lock: {=bool:?} }}", self.lock())
    }
}
#[doc = "RAM Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldoramc(pub u32);
impl Ldoramc {
    #[doc = "Isolate SRAM"]
    #[must_use]
    #[inline(always)]
    pub const fn iso(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Isolate SRAM"]
    #[inline(always)]
    pub const fn set_iso(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Switch SRAM"]
    #[must_use]
    #[inline(always)]
    pub const fn swi(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Switch SRAM"]
    #[inline(always)]
    pub const fn set_swi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Retention"]
    #[inline(always)]
    pub const fn set_ret0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Retention"]
    #[inline(always)]
    pub const fn set_ret1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Retention"]
    #[inline(always)]
    pub const fn set_ret2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Retention"]
    #[inline(always)]
    pub const fn set_ret3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Ldoramc {
    #[inline(always)]
    fn default() -> Ldoramc {
        Ldoramc(0)
    }
}
impl core::fmt::Debug for Ldoramc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ldoramc")
            .field("iso", &self.iso())
            .field("swi", &self.swi())
            .field("ret0", &self.ret0())
            .field("ret1", &self.ret1())
            .field("ret2", &self.ret2())
            .field("ret3", &self.ret3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ldoramc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ldoramc {{ iso: {=bool:?}, swi: {=bool:?}, ret0: {=bool:?}, ret1: {=bool:?}, ret2: {=bool:?}, ret3: {=bool:?} }}",
            self.iso(),
            self.swi(),
            self.ret0(),
            self.ret1(),
            self.ret2(),
            self.ret3()
        )
    }
}
#[doc = "Bandgap Timer 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldotimer0(pub u32);
impl Ldotimer0 {
    #[doc = "Timeout Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn timcfg(&self) -> super::vals::Timcfg {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Timcfg::from_bits(val as u8)
    }
    #[doc = "Timeout Configuration"]
    #[inline(always)]
    pub const fn set_timcfg(&mut self, val: super::vals::Timcfg) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Bandgap Timeout Period Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn timen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Timeout Period Enable"]
    #[inline(always)]
    pub const fn set_timen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ldotimer0 {
    #[inline(always)]
    fn default() -> Ldotimer0 {
        Ldotimer0(0)
    }
}
impl core::fmt::Debug for Ldotimer0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ldotimer0")
            .field("timcfg", &self.timcfg())
            .field("timen", &self.timen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ldotimer0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ldotimer0 {{ timcfg: {:?}, timen: {=bool:?} }}",
            self.timcfg(),
            self.timen()
        )
    }
}
#[doc = "Bandgap Timer 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldotimer1(pub u32);
impl Ldotimer1 {
    #[doc = "Timeout Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn timcfg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Timeout Configuration"]
    #[inline(always)]
    pub const fn set_timcfg(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Bandgap Timeout Period Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn timen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Timeout Period Enable"]
    #[inline(always)]
    pub const fn set_timen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ldotimer1 {
    #[inline(always)]
    fn default() -> Ldotimer1 {
        Ldotimer1(0)
    }
}
impl core::fmt::Debug for Ldotimer1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ldotimer1")
            .field("timcfg", &self.timcfg())
            .field("timen", &self.timen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ldotimer1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ldotimer1 {{ timcfg: {=u32:?}, timen: {=bool:?} }}",
            self.timcfg(),
            self.timen()
        )
    }
}
#[doc = "Lock A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Locka(pub u32);
impl Locka {
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Locka {
    #[inline(always)]
    fn default() -> Locka {
        Locka(0)
    }
}
impl core::fmt::Debug for Locka {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Locka").field("lock", &self.lock()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Locka {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Locka {{ lock: {=bool:?} }}", self.lock())
    }
}
#[doc = "CLKMON Configuration A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Moncfga(pub u32);
impl Moncfga {
    #[doc = "Frequency Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn freq_trim(&self) -> super::vals::FreqTrim {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::FreqTrim::from_bits(val as u8)
    }
    #[doc = "Frequency Trim"]
    #[inline(always)]
    pub const fn set_freq_trim(&mut self, val: super::vals::FreqTrim) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Divide Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn divide_trim(&self) -> super::vals::DivideTrim {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::DivideTrim::from_bits(val as u8)
    }
    #[doc = "Divide Trim"]
    #[inline(always)]
    pub const fn set_divide_trim(&mut self, val: super::vals::DivideTrim) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for Moncfga {
    #[inline(always)]
    fn default() -> Moncfga {
        Moncfga(0)
    }
}
impl core::fmt::Debug for Moncfga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Moncfga")
            .field("freq_trim", &self.freq_trim())
            .field("divide_trim", &self.divide_trim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Moncfga {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Moncfga {{ freq_trim: {:?}, divide_trim: {:?} }}",
            self.freq_trim(),
            self.divide_trim()
        )
    }
}
#[doc = "CLKMON Control A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Monctla(pub u32);
impl Monctla {
    #[doc = "CLKMON Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mon_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CLKMON Enable"]
    #[inline(always)]
    pub const fn set_mon_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Monctla {
    #[inline(always)]
    fn default() -> Monctla {
        Monctla(0)
    }
}
impl core::fmt::Debug for Monctla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Monctla")
            .field("mon_en", &self.mon_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Monctla {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Monctla {{ mon_en: {=bool:?} }}", self.mon_en())
    }
}
#[doc = "CLKMON Lock A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Monlcka(pub u32);
impl Monlcka {
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Monlcka {
    #[inline(always)]
    fn default() -> Monlcka {
        Monlcka(0)
    }
}
impl core::fmt::Debug for Monlcka {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Monlcka")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Monlcka {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Monlcka {{ lock: {=bool:?} }}", self.lock())
    }
}
#[doc = "CLKMON Test A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Montsta(pub u32);
impl Montsta {
    #[doc = "Test Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn tstmode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Test Mode"]
    #[inline(always)]
    pub const fn set_tstmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Montsta {
    #[inline(always)]
    fn default() -> Montsta {
        Montsta(0)
    }
}
impl core::fmt::Debug for Montsta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Montsta")
            .field("tstmode", &self.tstmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Montsta {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Montsta {{ tstmode: {=u8:?} }}", self.tstmode())
    }
}
#[doc = "Oscillator Configuration A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osccfga(pub u32);
impl Osccfga {
    #[doc = "Comparator Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp_trim(&self) -> super::vals::CmpTrim {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::CmpTrim::from_bits(val as u8)
    }
    #[doc = "Comparator Trim"]
    #[inline(always)]
    pub const fn set_cmp_trim(&mut self, val: super::vals::CmpTrim) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CAP2_TRIM"]
    #[must_use]
    #[inline(always)]
    pub const fn cap2_trim(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CAP2_TRIM"]
    #[inline(always)]
    pub const fn set_cap2_trim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Delay Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn dly_trim(&self) -> super::vals::DlyTrim {
        let val = (self.0 >> 3usize) & 0x0f;
        super::vals::DlyTrim::from_bits(val as u8)
    }
    #[doc = "Delay Trim"]
    #[inline(always)]
    pub const fn set_dly_trim(&mut self, val: super::vals::DlyTrim) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val.to_bits() as u32) & 0x0f) << 3usize);
    }
    #[doc = "Capacitor Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn cap_trim(&self) -> super::vals::CapTrim {
        let val = (self.0 >> 7usize) & 0x03;
        super::vals::CapTrim::from_bits(val as u8)
    }
    #[doc = "Capacitor Trim"]
    #[inline(always)]
    pub const fn set_cap_trim(&mut self, val: super::vals::CapTrim) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val.to_bits() as u32) & 0x03) << 7usize);
    }
    #[doc = "Initialization Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn init_trim(&self) -> super::vals::InitTrim {
        let val = (self.0 >> 9usize) & 0x07;
        super::vals::InitTrim::from_bits(val as u8)
    }
    #[doc = "Initialization Trim"]
    #[inline(always)]
    pub const fn set_init_trim(&mut self, val: super::vals::InitTrim) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val.to_bits() as u32) & 0x07) << 9usize);
    }
}
impl Default for Osccfga {
    #[inline(always)]
    fn default() -> Osccfga {
        Osccfga(0)
    }
}
impl core::fmt::Debug for Osccfga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Osccfga")
            .field("cmp_trim", &self.cmp_trim())
            .field("cap2_trim", &self.cap2_trim())
            .field("dly_trim", &self.dly_trim())
            .field("cap_trim", &self.cap_trim())
            .field("init_trim", &self.init_trim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Osccfga {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Osccfga {{ cmp_trim: {:?}, cap2_trim: {=bool:?}, dly_trim: {:?}, cap_trim: {:?}, init_trim: {:?} }}",
            self.cmp_trim(),
            self.cap2_trim(),
            self.dly_trim(),
            self.cap_trim(),
            self.init_trim()
        )
    }
}
#[doc = "Oscillator Clock Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oscclke(pub u32);
impl Oscclke {
    #[doc = "Clock Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn clke(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Clock Enable"]
    #[inline(always)]
    pub const fn set_clke(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Oscclke {
    #[inline(always)]
    fn default() -> Oscclke {
        Oscclke(0)
    }
}
impl core::fmt::Debug for Oscclke {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Oscclke")
            .field("clke", &self.clke())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Oscclke {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Oscclke {{ clke: {=u8:?} }}", self.clke())
    }
}
#[doc = "Oscillator Control A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oscctla(pub u32);
impl Oscctla {
    #[doc = "Crystal Oscillator Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Crystal Oscillator Enable"]
    #[inline(always)]
    pub const fn set_osc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Crystal Oscillator Bypass Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_byp_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Crystal Oscillator Bypass Enable"]
    #[inline(always)]
    pub const fn set_osc_byp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Amplifier Gain Coarse Adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn coarse_amp_gain(&self) -> super::vals::CoarseAmpGain {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::CoarseAmpGain::from_bits(val as u8)
    }
    #[doc = "Amplifier Gain Coarse Adjustment"]
    #[inline(always)]
    pub const fn set_coarse_amp_gain(&mut self, val: super::vals::CoarseAmpGain) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Crystal Load Capacitance Selection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cap_sel_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Crystal Load Capacitance Selection Enable"]
    #[inline(always)]
    pub const fn set_cap_sel_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Crystal Load Capacitance Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn extal_cap_sel(&self) -> super::vals::ExtalCapSel {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::ExtalCapSel::from_bits(val as u8)
    }
    #[doc = "Crystal Load Capacitance Selection"]
    #[inline(always)]
    pub const fn set_extal_cap_sel(&mut self, val: super::vals::ExtalCapSel) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Crystal Load Capacitance Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn xtal_cap_sel(&self) -> super::vals::XtalCapSel {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::XtalCapSel::from_bits(val as u8)
    }
    #[doc = "Crystal Load Capacitance Selection"]
    #[inline(always)]
    pub const fn set_xtal_cap_sel(&mut self, val: super::vals::XtalCapSel) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mode_en(&self) -> super::vals::ModeEn {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::ModeEn::from_bits(val as u8)
    }
    #[doc = "Mode Enable"]
    #[inline(always)]
    pub const fn set_mode_en(&mut self, val: super::vals::ModeEn) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Supply Detector Trim"]
    #[must_use]
    #[inline(always)]
    pub const fn supply_det(&self) -> super::vals::SupplyDet {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::SupplyDet::from_bits(val as u8)
    }
    #[doc = "Supply Detector Trim"]
    #[inline(always)]
    pub const fn set_supply_det(&mut self, val: super::vals::SupplyDet) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
}
impl Default for Oscctla {
    #[inline(always)]
    fn default() -> Oscctla {
        Oscctla(0)
    }
}
impl core::fmt::Debug for Oscctla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Oscctla")
            .field("osc_en", &self.osc_en())
            .field("osc_byp_en", &self.osc_byp_en())
            .field("coarse_amp_gain", &self.coarse_amp_gain())
            .field("cap_sel_en", &self.cap_sel_en())
            .field("extal_cap_sel", &self.extal_cap_sel())
            .field("xtal_cap_sel", &self.xtal_cap_sel())
            .field("mode_en", &self.mode_en())
            .field("supply_det", &self.supply_det())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Oscctla {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Oscctla {{ osc_en: {=bool:?}, osc_byp_en: {=bool:?}, coarse_amp_gain: {:?}, cap_sel_en: {=bool:?}, extal_cap_sel: {:?}, xtal_cap_sel: {:?}, mode_en: {:?}, supply_det: {:?} }}",
            self.osc_en(),
            self.osc_byp_en(),
            self.coarse_amp_gain(),
            self.cap_sel_en(),
            self.extal_cap_sel(),
            self.xtal_cap_sel(),
            self.mode_en(),
            self.supply_det()
        )
    }
}
#[doc = "Oscillator Lock A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osclcka(pub u32);
impl Osclcka {
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Osclcka {
    #[inline(always)]
    fn default() -> Osclcka {
        Osclcka(0)
    }
}
impl core::fmt::Debug for Osclcka {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Osclcka")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Osclcka {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Osclcka {{ lock: {=bool:?} }}", self.lock())
    }
}
#[doc = "Status A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Statusa(pub u32);
impl Statusa {
    #[doc = "POR Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn por_det(&self) -> super::vals::StatusaPorDet {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::StatusaPorDet::from_bits(val as u8)
    }
    #[doc = "POR Detect Flag"]
    #[inline(always)]
    pub const fn set_por_det(&mut self, val: super::vals::StatusaPorDet) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Wakeup Pin Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_flag(&self) -> super::vals::StatusaWakeupFlag {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::StatusaWakeupFlag::from_bits(val as u8)
    }
    #[doc = "Wakeup Pin Flag"]
    #[inline(always)]
    pub const fn set_wakeup_flag(&mut self, val: super::vals::StatusaWakeupFlag) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Bandgap Timer 0 Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn timer0_flag(&self) -> super::vals::StatusaTimer0Flag {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::StatusaTimer0Flag::from_bits(val as u8)
    }
    #[doc = "Bandgap Timer 0 Flag"]
    #[inline(always)]
    pub const fn set_timer0_flag(&mut self, val: super::vals::StatusaTimer0Flag) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Bandgap Timer 1 Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn timer1_flag(&self) -> super::vals::StatusaTimer1Flag {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::StatusaTimer1Flag::from_bits(val as u8)
    }
    #[doc = "Bandgap Timer 1 Flag"]
    #[inline(always)]
    pub const fn set_timer1_flag(&mut self, val: super::vals::StatusaTimer1Flag) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "LDO Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn ldo_rdy(&self) -> super::vals::StatusaLdoRdy {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::StatusaLdoRdy::from_bits(val as u8)
    }
    #[doc = "LDO Ready"]
    #[inline(always)]
    pub const fn set_ldo_rdy(&mut self, val: super::vals::StatusaLdoRdy) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "OSC32k Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_rdy(&self) -> super::vals::StatusaOscRdy {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::StatusaOscRdy::from_bits(val as u8)
    }
    #[doc = "OSC32k Ready"]
    #[inline(always)]
    pub const fn set_osc_rdy(&mut self, val: super::vals::StatusaOscRdy) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Clock Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn clock_det(&self) -> super::vals::StatusaClockDet {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::StatusaClockDet::from_bits(val as u8)
    }
    #[doc = "Clock Detect"]
    #[inline(always)]
    pub const fn set_clock_det(&mut self, val: super::vals::StatusaClockDet) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input 0 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn sec0_det(&self) -> super::vals::Sec0Det {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Sec0Det::from_bits(val as u8)
    }
    #[doc = "Input 0 Detect"]
    #[inline(always)]
    pub const fn set_sec0_det(&mut self, val: super::vals::Sec0Det) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt 0 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn irq0_det(&self) -> super::vals::StatusaIrq0Det {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::StatusaIrq0Det::from_bits(val as u8)
    }
    #[doc = "Interrupt 0 Detect"]
    #[inline(always)]
    pub const fn set_irq0_det(&mut self, val: super::vals::StatusaIrq0Det) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Interrupt 1 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn irq1_det(&self) -> super::vals::StatusaIrq1Det {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::StatusaIrq1Det::from_bits(val as u8)
    }
    #[doc = "Interrupt 1 Detect"]
    #[inline(always)]
    pub const fn set_irq1_det(&mut self, val: super::vals::StatusaIrq1Det) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Interrupt 2 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn irq2_det(&self) -> super::vals::StatusaIrq2Det {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::StatusaIrq2Det::from_bits(val as u8)
    }
    #[doc = "Interrupt 2 Detect"]
    #[inline(always)]
    pub const fn set_irq2_det(&mut self, val: super::vals::StatusaIrq2Det) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Interrupt 3 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn irq3_det(&self) -> super::vals::StatusaIrq3Det {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::StatusaIrq3Det::from_bits(val as u8)
    }
    #[doc = "Interrupt 3 Detect"]
    #[inline(always)]
    pub const fn set_irq3_det(&mut self, val: super::vals::StatusaIrq3Det) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
}
impl Default for Statusa {
    #[inline(always)]
    fn default() -> Statusa {
        Statusa(0)
    }
}
impl core::fmt::Debug for Statusa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Statusa")
            .field("por_det", &self.por_det())
            .field("wakeup_flag", &self.wakeup_flag())
            .field("timer0_flag", &self.timer0_flag())
            .field("timer1_flag", &self.timer1_flag())
            .field("ldo_rdy", &self.ldo_rdy())
            .field("osc_rdy", &self.osc_rdy())
            .field("clock_det", &self.clock_det())
            .field("sec0_det", &self.sec0_det())
            .field("irq0_det", &self.irq0_det())
            .field("irq1_det", &self.irq1_det())
            .field("irq2_det", &self.irq2_det())
            .field("irq3_det", &self.irq3_det())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Statusa {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Statusa {{ por_det: {:?}, wakeup_flag: {:?}, timer0_flag: {:?}, timer1_flag: {:?}, ldo_rdy: {:?}, osc_rdy: {:?}, clock_det: {:?}, sec0_det: {:?}, irq0_det: {:?}, irq1_det: {:?}, irq2_det: {:?}, irq3_det: {:?} }}",
            self.por_det(),
            self.wakeup_flag(),
            self.timer0_flag(),
            self.timer1_flag(),
            self.ldo_rdy(),
            self.osc_rdy(),
            self.clock_det(),
            self.sec0_det(),
            self.irq0_det(),
            self.irq1_det(),
            self.irq2_det(),
            self.irq3_det()
        )
    }
}
#[doc = "Switch Control A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swictla(pub u32);
impl Swictla {
    #[doc = "Switch Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn swi_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Switch Enable"]
    #[inline(always)]
    pub const fn set_swi_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Low Power Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Low Power Enable"]
    #[inline(always)]
    pub const fn set_lp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Swictla {
    #[inline(always)]
    fn default() -> Swictla {
        Swictla(0)
    }
}
impl core::fmt::Debug for Swictla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swictla")
            .field("swi_en", &self.swi_en())
            .field("lp_en", &self.lp_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swictla {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Swictla {{ swi_en: {=bool:?}, lp_en: {=bool:?} }}",
            self.swi_en(),
            self.lp_en()
        )
    }
}
#[doc = "Switch Lock A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swilcka(pub u32);
impl Swilcka {
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Swilcka {
    #[inline(always)]
    fn default() -> Swilcka {
        Swilcka(0)
    }
}
impl core::fmt::Debug for Swilcka {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swilcka")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swilcka {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Swilcka {{ lock: {=bool:?} }}", self.lock())
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
    pub const fn feature(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Feature Specification Number"]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
            "Verid {{ feature: {=u16:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.feature(),
            self.minor(),
            self.major()
        )
    }
}
#[doc = "Wake-up Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakecfg(pub u32);
impl Wakecfg {
    #[doc = "Output"]
    #[must_use]
    #[inline(always)]
    pub const fn out(&self) -> super::vals::Out {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Out::from_bits(val as u8)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub const fn set_out(&mut self, val: super::vals::Out) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Wakecfg {
    #[inline(always)]
    fn default() -> Wakecfg {
        Wakecfg(0)
    }
}
impl core::fmt::Debug for Wakecfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wakecfg").field("out", &self.out()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wakecfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wakecfg {{ out: {:?} }}", self.out())
    }
}
#[doc = "Wake-up Enable A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakena(pub u32);
impl Wakena {
    #[doc = "POR Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn por_det(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "POR Detect"]
    #[inline(always)]
    pub const fn set_por_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Wake-up Pin Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_flag(&self) -> super::vals::WakenaWakeupFlag {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::WakenaWakeupFlag::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Flag"]
    #[inline(always)]
    pub const fn set_wakeup_flag(&mut self, val: super::vals::WakenaWakeupFlag) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Bandgap Timer 0"]
    #[must_use]
    #[inline(always)]
    pub const fn timer0_flag(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Timer 0"]
    #[inline(always)]
    pub const fn set_timer0_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Bandgap Timer 2"]
    #[must_use]
    #[inline(always)]
    pub const fn timer1_flag(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Bandgap Timer 2"]
    #[inline(always)]
    pub const fn set_timer1_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "LDO Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn ldo_rdy(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "LDO Ready"]
    #[inline(always)]
    pub const fn set_ldo_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "OSC32K Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_rdy(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OSC32K Ready"]
    #[inline(always)]
    pub const fn set_osc_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Clock Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn clock_det(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Detect"]
    #[inline(always)]
    pub const fn set_clock_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt 0 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn irq0_det(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt 0 Detect"]
    #[inline(always)]
    pub const fn set_irq0_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Interrupt 1 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn irq1_det(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt 1 Detect"]
    #[inline(always)]
    pub const fn set_irq1_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Interrupt 2 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn irq2_det(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt 2 Detect"]
    #[inline(always)]
    pub const fn set_irq2_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Interrupt 3 Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn irq3_det(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt 3 Detect"]
    #[inline(always)]
    pub const fn set_irq3_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Wakena {
    #[inline(always)]
    fn default() -> Wakena {
        Wakena(0)
    }
}
impl core::fmt::Debug for Wakena {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wakena")
            .field("por_det", &self.por_det())
            .field("wakeup_flag", &self.wakeup_flag())
            .field("timer0_flag", &self.timer0_flag())
            .field("timer1_flag", &self.timer1_flag())
            .field("ldo_rdy", &self.ldo_rdy())
            .field("osc_rdy", &self.osc_rdy())
            .field("clock_det", &self.clock_det())
            .field("irq0_det", &self.irq0_det())
            .field("irq1_det", &self.irq1_det())
            .field("irq2_det", &self.irq2_det())
            .field("irq3_det", &self.irq3_det())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wakena {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wakena {{ por_det: {=bool:?}, wakeup_flag: {:?}, timer0_flag: {=bool:?}, timer1_flag: {=bool:?}, ldo_rdy: {=bool:?}, osc_rdy: {=bool:?}, clock_det: {=bool:?}, irq0_det: {=bool:?}, irq1_det: {=bool:?}, irq2_det: {=bool:?}, irq3_det: {=bool:?} }}",
            self.por_det(),
            self.wakeup_flag(),
            self.timer0_flag(),
            self.timer1_flag(),
            self.ldo_rdy(),
            self.osc_rdy(),
            self.clock_det(),
            self.irq0_det(),
            self.irq1_det(),
            self.irq2_det(),
            self.irq3_det()
        )
    }
}
#[doc = "Wakeup 0 Register A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakeupa(pub u32);
impl Wakeupa {
    #[doc = "Register"]
    #[must_use]
    #[inline(always)]
    pub const fn reg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Register"]
    #[inline(always)]
    pub const fn set_reg(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Wakeupa {
    #[inline(always)]
    fn default() -> Wakeupa {
        Wakeupa(0)
    }
}
impl core::fmt::Debug for Wakeupa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wakeupa").field("reg", &self.reg()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wakeupa {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wakeupa {{ reg: {=u32:?} }}", self.reg())
    }
}
#[doc = "Wakeup Lock A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Waklcka(pub u32);
impl Waklcka {
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Waklcka {
    #[inline(always)]
    fn default() -> Waklcka {
        Waklcka(0)
    }
}
impl core::fmt::Debug for Waklcka {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Waklcka")
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Waklcka {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Waklcka {{ lock: {=bool:?} }}", self.lock())
    }
}
