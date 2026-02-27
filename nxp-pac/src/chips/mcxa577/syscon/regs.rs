#[doc = "System Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbclkdiv(pub u32);
impl Ahbclkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::AhbclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::AhbclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::AhbclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbclkdiv {
    #[inline(always)]
    fn default() -> Ahbclkdiv {
        Ahbclkdiv(0)
    }
}
impl core::fmt::Debug for Ahbclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbclkdiv")
            .field("div", &self.div())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbclkdiv {{ div: {=u8:?}, unstab: {:?} }}",
            self.div(),
            self.unstab()
        )
    }
}
#[doc = "AHB Matrix Priority Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbmatprio(pub u32);
impl Ahbmatprio {
    #[doc = "CPU0 C-AHB bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_cbus(&self) -> super::vals::Cpu0Cbus {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cpu0Cbus::from_bits(val as u8)
    }
    #[doc = "CPU0 C-AHB bus master priority level"]
    #[inline(always)]
    pub const fn set_cpu0_cbus(&mut self, val: super::vals::Cpu0Cbus) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CPU0 S-AHB bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_sbus(&self) -> super::vals::AhbmatprioCpu0Sbus {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::AhbmatprioCpu0Sbus::from_bits(val as u8)
    }
    #[doc = "CPU0 S-AHB bus master priority level"]
    #[inline(always)]
    pub const fn set_cpu0_sbus(&mut self, val: super::vals::AhbmatprioCpu0Sbus) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "SmartDMA-I bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_cbus_smart_dma_i(&self) -> super::vals::Cpu1CbusSmartDmaI {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Cpu1CbusSmartDmaI::from_bits(val as u8)
    }
    #[doc = "SmartDMA-I bus master priority level"]
    #[inline(always)]
    pub const fn set_cpu1_cbus_smart_dma_i(&mut self, val: super::vals::Cpu1CbusSmartDmaI) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "SmartDMA-D bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_sbus_smart_dma_d(&self) -> super::vals::Cpu1SbusSmartDmaD {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Cpu1SbusSmartDmaD::from_bits(val as u8)
    }
    #[doc = "SmartDMA-D bus master priority level"]
    #[inline(always)]
    pub const fn set_cpu1_sbus_smart_dma_d(&mut self, val: super::vals::Cpu1SbusSmartDmaD) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "DMA0 controller bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> super::vals::AhbmatprioDma0 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::AhbmatprioDma0::from_bits(val as u8)
    }
    #[doc = "DMA0 controller bus master priority level"]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: super::vals::AhbmatprioDma0) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "DMA1 controller bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1(&self) -> super::vals::AhbmatprioDma1 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::AhbmatprioDma1::from_bits(val as u8)
    }
    #[doc = "DMA1 controller bus master priority level"]
    #[inline(always)]
    pub const fn set_dma1(&mut self, val: super::vals::AhbmatprioDma1) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "PKC and ELS bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn pkc_els(&self) -> super::vals::AhbmatprioPkcEls {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::AhbmatprioPkcEls::from_bits(val as u8)
    }
    #[doc = "PKC and ELS bus master priority level"]
    #[inline(always)]
    pub const fn set_pkc_els(&mut self, val: super::vals::AhbmatprioPkcEls) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "ESPI bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn coolflux_y_espi(&self) -> super::vals::AhbmatprioCoolfluxYEspi {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::AhbmatprioCoolfluxYEspi::from_bits(val as u8)
    }
    #[doc = "ESPI bus master priority level"]
    #[inline(always)]
    pub const fn set_coolflux_y_espi(&mut self, val: super::vals::AhbmatprioCoolfluxYEspi) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "ENET bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn usb_fs_enet(&self) -> super::vals::AhbmatprioUsbFsEnet {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::AhbmatprioUsbFsEnet::from_bits(val as u8)
    }
    #[doc = "ENET bus master priority level"]
    #[inline(always)]
    pub const fn set_usb_fs_enet(&mut self, val: super::vals::AhbmatprioUsbFsEnet) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "USB-HS bus master priority level"]
    #[must_use]
    #[inline(always)]
    pub const fn usb_hs(&self) -> super::vals::AhbmatprioUsbHs {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::AhbmatprioUsbHs::from_bits(val as u8)
    }
    #[doc = "USB-HS bus master priority level"]
    #[inline(always)]
    pub const fn set_usb_hs(&mut self, val: super::vals::AhbmatprioUsbHs) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
}
impl Default for Ahbmatprio {
    #[inline(always)]
    fn default() -> Ahbmatprio {
        Ahbmatprio(0)
    }
}
impl core::fmt::Debug for Ahbmatprio {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbmatprio")
            .field("cpu0_cbus", &self.cpu0_cbus())
            .field("cpu0_sbus", &self.cpu0_sbus())
            .field("cpu1_cbus_smart_dma_i", &self.cpu1_cbus_smart_dma_i())
            .field("cpu1_sbus_smart_dma_d", &self.cpu1_sbus_smart_dma_d())
            .field("dma0", &self.dma0())
            .field("dma1", &self.dma1())
            .field("pkc_els", &self.pkc_els())
            .field("coolflux_y_espi", &self.coolflux_y_espi())
            .field("usb_fs_enet", &self.usb_fs_enet())
            .field("usb_hs", &self.usb_hs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbmatprio {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbmatprio {{ cpu0_cbus: {:?}, cpu0_sbus: {:?}, cpu1_cbus_smart_dma_i: {:?}, cpu1_sbus_smart_dma_d: {:?}, dma0: {:?}, dma1: {:?}, pkc_els: {:?}, coolflux_y_espi: {:?}, usb_fs_enet: {:?}, usb_hs: {:?} }}",
            self.cpu0_cbus(),
            self.cpu0_sbus(),
            self.cpu1_cbus_smart_dma_i(),
            self.cpu1_sbus_smart_dma_d(),
            self.dma0(),
            self.dma1(),
            self.pkc_els(),
            self.coolflux_y_espi(),
            self.usb_fs_enet(),
            self.usb_hs()
        )
    }
}
#[doc = "Gray to Binary Converter Binary Code \\[31:0\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BinaryCodeLsb(pub u32);
impl BinaryCodeLsb {
    #[doc = "Binary code \\[31:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn code_bin_31_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Binary code \\[31:0\\]"]
    #[inline(always)]
    pub const fn set_code_bin_31_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for BinaryCodeLsb {
    #[inline(always)]
    fn default() -> BinaryCodeLsb {
        BinaryCodeLsb(0)
    }
}
impl core::fmt::Debug for BinaryCodeLsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BinaryCodeLsb")
            .field("code_bin_31_0", &self.code_bin_31_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BinaryCodeLsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BinaryCodeLsb {{ code_bin_31_0: {=u32:?} }}",
            self.code_bin_31_0()
        )
    }
}
#[doc = "Gray to Binary Converter Binary Code \\[41:32\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BinaryCodeMsb(pub u32);
impl BinaryCodeMsb {
    #[doc = "Binary code \\[41:32\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn code_bin_41_32(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Binary code \\[41:32\\]"]
    #[inline(always)]
    pub const fn set_code_bin_41_32(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for BinaryCodeMsb {
    #[inline(always)]
    fn default() -> BinaryCodeMsb {
        BinaryCodeMsb(0)
    }
}
impl core::fmt::Debug for BinaryCodeMsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BinaryCodeMsb")
            .field("code_bin_41_32", &self.code_bin_41_32())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BinaryCodeMsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BinaryCodeMsb {{ code_bin_41_32: {=u16:?} }}",
            self.code_bin_41_32()
        )
    }
}
#[doc = "BUS_CLK Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busclkdiv(pub u32);
impl Busclkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::BusclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::BusclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::BusclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::BusclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::BusclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::BusclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::BusclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::BusclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::BusclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Busclkdiv {
    #[inline(always)]
    fn default() -> Busclkdiv {
        Busclkdiv(0)
    }
}
impl core::fmt::Debug for Busclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Busclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Busclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Busclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "Clock Configuration Unlock"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkunlock(pub u32);
impl Clkunlock {
    #[doc = "Controls clock configuration registers access (for example, SLOWCLKDIV, BUSCLKDIV, AHBCLKDIV, FROHFDIV, FROLFDIV, PLLxCLKDIV, MRCC_xxx_CLKDIV, MRCC_xxx_CLKSEL, MRCC_GLB_xxx)"]
    #[must_use]
    #[inline(always)]
    pub const fn unlock(&self) -> super::vals::Unlock {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Unlock::from_bits(val as u8)
    }
    #[doc = "Controls clock configuration registers access (for example, SLOWCLKDIV, BUSCLKDIV, AHBCLKDIV, FROHFDIV, FROLFDIV, PLLxCLKDIV, MRCC_xxx_CLKDIV, MRCC_xxx_CLKSEL, MRCC_GLB_xxx)"]
    #[inline(always)]
    pub const fn set_unlock(&mut self, val: super::vals::Unlock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Clkunlock {
    #[inline(always)]
    fn default() -> Clkunlock {
        Clkunlock(0)
    }
}
impl core::fmt::Debug for Clkunlock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkunlock")
            .field("unlock", &self.unlock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkunlock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Clkunlock {{ unlock: {:?} }}", self.unlock())
    }
}
#[doc = "Non-Secure CPU0 System Tick Calibration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu0nstckcal(pub u32);
impl Cpu0nstckcal {
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
    #[doc = "Indicates whether the TENMS value is exact."]
    #[must_use]
    #[inline(always)]
    pub const fn skew(&self) -> super::vals::Skew {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Skew::from_bits(val as u8)
    }
    #[doc = "Indicates whether the TENMS value is exact."]
    #[inline(always)]
    pub const fn set_skew(&mut self, val: super::vals::Skew) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Indicates whether the device provides a reference clock to the processor."]
    #[must_use]
    #[inline(always)]
    pub const fn noref(&self) -> super::vals::Noref {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Noref::from_bits(val as u8)
    }
    #[doc = "Indicates whether the device provides a reference clock to the processor."]
    #[inline(always)]
    pub const fn set_noref(&mut self, val: super::vals::Noref) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for Cpu0nstckcal {
    #[inline(always)]
    fn default() -> Cpu0nstckcal {
        Cpu0nstckcal(0)
    }
}
impl core::fmt::Debug for Cpu0nstckcal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpu0nstckcal")
            .field("tenms", &self.tenms())
            .field("skew", &self.skew())
            .field("noref", &self.noref())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpu0nstckcal {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpu0nstckcal {{ tenms: {=u32:?}, skew: {:?}, noref: {:?} }}",
            self.tenms(),
            self.skew(),
            self.noref()
        )
    }
}
#[doc = "CPU Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpustat(pub u32);
impl Cpustat {
    #[doc = "CPU0 sleeping state"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0sleeping(&self) -> super::vals::Cpu0sleeping {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Cpu0sleeping::from_bits(val as u8)
    }
    #[doc = "CPU0 sleeping state"]
    #[inline(always)]
    pub const fn set_cpu0sleeping(&mut self, val: super::vals::Cpu0sleeping) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "CPU0 lockup state"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0lockup(&self) -> super::vals::Cpu0lockup {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Cpu0lockup::from_bits(val as u8)
    }
    #[doc = "CPU0 lockup state"]
    #[inline(always)]
    pub const fn set_cpu0lockup(&mut self, val: super::vals::Cpu0lockup) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for Cpustat {
    #[inline(always)]
    fn default() -> Cpustat {
        Cpustat(0)
    }
}
impl core::fmt::Debug for Cpustat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpustat")
            .field("cpu0sleeping", &self.cpu0sleeping())
            .field("cpu0lockup", &self.cpu0lockup())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpustat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpustat {{ cpu0sleeping: {:?}, cpu0lockup: {:?} }}",
            self.cpu0sleeping(),
            self.cpu0lockup()
        )
    }
}
#[doc = "CTIMER Global Start Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimerglobalstarten(pub u32);
impl Ctimerglobalstarten {
    #[doc = "Enables the CTIMER0 function clock"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer0_clk_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CTIMER0 function clock"]
    #[inline(always)]
    pub const fn set_ctimer0_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the CTIMER1 function clock"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer1_clk_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CTIMER1 function clock"]
    #[inline(always)]
    pub const fn set_ctimer1_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the CTIMER2 function clock"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer2_clk_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CTIMER2 function clock"]
    #[inline(always)]
    pub const fn set_ctimer2_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables the CTIMER3 function clock"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer3_clk_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CTIMER3 function clock"]
    #[inline(always)]
    pub const fn set_ctimer3_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the CTIMER4 function clock"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer4_clk_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CTIMER4 function clock"]
    #[inline(always)]
    pub const fn set_ctimer4_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Ctimerglobalstarten {
    #[inline(always)]
    fn default() -> Ctimerglobalstarten {
        Ctimerglobalstarten(0)
    }
}
impl core::fmt::Debug for Ctimerglobalstarten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimerglobalstarten")
            .field("ctimer0_clk_en", &self.ctimer0_clk_en())
            .field("ctimer1_clk_en", &self.ctimer1_clk_en())
            .field("ctimer2_clk_en", &self.ctimer2_clk_en())
            .field("ctimer3_clk_en", &self.ctimer3_clk_en())
            .field("ctimer4_clk_en", &self.ctimer4_clk_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimerglobalstarten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctimerglobalstarten {{ ctimer0_clk_en: {=bool:?}, ctimer1_clk_en: {=bool:?}, ctimer2_clk_en: {=bool:?}, ctimer3_clk_en: {=bool:?}, ctimer4_clk_en: {=bool:?} }}",
            self.ctimer0_clk_en(),
            self.ctimer1_clk_en(),
            self.ctimer2_clk_en(),
            self.ctimer3_clk_en(),
            self.ctimer4_clk_en()
        )
    }
}
#[doc = "Device ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DeviceId0(pub u32);
impl DeviceId0 {
    #[doc = "Indicates the device's ram size"]
    #[must_use]
    #[inline(always)]
    pub const fn ram_size(&self) -> super::vals::RamSize {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::RamSize::from_bits(val as u8)
    }
    #[doc = "Indicates the device's ram size"]
    #[inline(always)]
    pub const fn set_ram_size(&mut self, val: super::vals::RamSize) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Indicates the device's flash size"]
    #[must_use]
    #[inline(always)]
    pub const fn flash_size(&self) -> super::vals::FlashSize {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::FlashSize::from_bits(val as u8)
    }
    #[doc = "Indicates the device's flash size"]
    #[inline(always)]
    pub const fn set_flash_size(&mut self, val: super::vals::FlashSize) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Indicates the device's ROM revision"]
    #[must_use]
    #[inline(always)]
    pub const fn rom_rev_minor(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates the device's ROM revision"]
    #[inline(always)]
    pub const fn set_rom_rev_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn security(&self) -> super::vals::Security {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Security::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_security(&mut self, val: super::vals::Security) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for DeviceId0 {
    #[inline(always)]
    fn default() -> DeviceId0 {
        DeviceId0(0)
    }
}
impl core::fmt::Debug for DeviceId0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DeviceId0")
            .field("ram_size", &self.ram_size())
            .field("flash_size", &self.flash_size())
            .field("rom_rev_minor", &self.rom_rev_minor())
            .field("security", &self.security())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DeviceId0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DeviceId0 {{ ram_size: {:?}, flash_size: {:?}, rom_rev_minor: {=u8:?}, security: {:?} }}",
            self.ram_size(),
            self.flash_size(),
            self.rom_rev_minor(),
            self.security()
        )
    }
}
#[doc = "Device Type"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DeviceType(pub u32);
impl DeviceType {
    #[doc = "Indicates the device part number"]
    #[must_use]
    #[inline(always)]
    pub const fn device_type_num(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Indicates the device part number"]
    #[inline(always)]
    pub const fn set_device_type_num(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Indicates the device type"]
    #[must_use]
    #[inline(always)]
    pub const fn device_type_sec(&self) -> super::vals::DeviceTypeSec {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::DeviceTypeSec::from_bits(val as u8)
    }
    #[doc = "Indicates the device type"]
    #[inline(always)]
    pub const fn set_device_type_sec(&mut self, val: super::vals::DeviceTypeSec) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Indicates the device's package type"]
    #[must_use]
    #[inline(always)]
    pub const fn device_type_pkg(&self) -> super::vals::DeviceTypePkg {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::DeviceTypePkg::from_bits(val as u8)
    }
    #[doc = "Indicates the device's package type"]
    #[inline(always)]
    pub const fn set_device_type_pkg(&mut self, val: super::vals::DeviceTypePkg) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "Indicates the device's pin number"]
    #[must_use]
    #[inline(always)]
    pub const fn device_type_pin(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the device's pin number"]
    #[inline(always)]
    pub const fn set_device_type_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for DeviceType {
    #[inline(always)]
    fn default() -> DeviceType {
        DeviceType(0)
    }
}
impl core::fmt::Debug for DeviceType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DeviceType")
            .field("device_type_num", &self.device_type_num())
            .field("device_type_sec", &self.device_type_sec())
            .field("device_type_pkg", &self.device_type_pkg())
            .field("device_type_pin", &self.device_type_pin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DeviceType {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DeviceType {{ device_type_num: {=u16:?}, device_type_sec: {:?}, device_type_pkg: {:?}, device_type_pin: {=u8:?} }}",
            self.device_type_num(),
            self.device_type_sec(),
            self.device_type_pkg(),
            self.device_type_pin()
        )
    }
}
#[doc = "Ethernet Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EnetCtrl(pub u32);
impl EnetCtrl {
    #[doc = "Selects external PHY interface"]
    #[must_use]
    #[inline(always)]
    pub const fn phy_intf(&self) -> super::vals::PhyIntf {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::PhyIntf::from_bits(val as u8)
    }
    #[doc = "Selects external PHY interface"]
    #[inline(always)]
    pub const fn set_phy_intf(&mut self, val: super::vals::PhyIntf) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Selects external PHY or on-chip 10BASE-T1S"]
    #[must_use]
    #[inline(always)]
    pub const fn phy_sel(&self) -> super::vals::PhySel {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::PhySel::from_bits(val as u8)
    }
    #[doc = "Selects external PHY or on-chip 10BASE-T1S"]
    #[inline(always)]
    pub const fn set_phy_sel(&mut self, val: super::vals::PhySel) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for EnetCtrl {
    #[inline(always)]
    fn default() -> EnetCtrl {
        EnetCtrl(0)
    }
}
impl core::fmt::Debug for EnetCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EnetCtrl")
            .field("phy_intf", &self.phy_intf())
            .field("phy_sel", &self.phy_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EnetCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EnetCtrl {{ phy_intf: {:?}, phy_sel: {:?} }}",
            self.phy_intf(),
            self.phy_sel()
        )
    }
}
#[doc = "Sideband Flow Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EnetSbdFlowCtrl(pub u32);
impl EnetSbdFlowCtrl {
    #[doc = "Sideband Flow Control for channel0"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_ch0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Sideband Flow Control for channel0"]
    #[inline(always)]
    pub const fn set_sel_ch0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for EnetSbdFlowCtrl {
    #[inline(always)]
    fn default() -> EnetSbdFlowCtrl {
        EnetSbdFlowCtrl(0)
    }
}
impl core::fmt::Debug for EnetSbdFlowCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EnetSbdFlowCtrl")
            .field("sel_ch0", &self.sel_ch0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EnetSbdFlowCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EnetSbdFlowCtrl {{ sel_ch0: {=bool:?} }}",
            self.sel_ch0()
        )
    }
}
#[doc = "FRO_HF_DIV Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frohfdiv(pub u32);
impl Frohfdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::FrohfdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::FrohfdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::FrohfdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::FrohfdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::FrohfdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::FrohfdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::FrohfdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::FrohfdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::FrohfdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Frohfdiv {
    #[inline(always)]
    fn default() -> Frohfdiv {
        Frohfdiv(0)
    }
}
impl core::fmt::Debug for Frohfdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frohfdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frohfdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Frohfdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "FRO_LF_DIV Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frolfdiv(pub u32);
impl Frolfdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::FrolfdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::FrolfdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::FrolfdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::FrolfdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::FrolfdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::FrolfdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::FrolfdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::FrolfdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::FrolfdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Frolfdiv {
    #[inline(always)]
    fn default() -> Frolfdiv {
        Frolfdiv(0)
    }
}
impl core::fmt::Debug for Frolfdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frolfdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frolfdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Frolfdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "Gray to Binary Converter Gray Code \\[31:0\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrayCodeLsb(pub u32);
impl GrayCodeLsb {
    #[doc = "Gray code \\[31:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn code_gray_31_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Gray code \\[31:0\\]"]
    #[inline(always)]
    pub const fn set_code_gray_31_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GrayCodeLsb {
    #[inline(always)]
    fn default() -> GrayCodeLsb {
        GrayCodeLsb(0)
    }
}
impl core::fmt::Debug for GrayCodeLsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GrayCodeLsb")
            .field("code_gray_31_0", &self.code_gray_31_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GrayCodeLsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GrayCodeLsb {{ code_gray_31_0: {=u32:?} }}",
            self.code_gray_31_0()
        )
    }
}
#[doc = "Gray to Binary Converter Gray Code \\[41:32\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrayCodeMsb(pub u32);
impl GrayCodeMsb {
    #[doc = "Gray code \\[41:32\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn code_gray_41_32(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Gray code \\[41:32\\]"]
    #[inline(always)]
    pub const fn set_code_gray_41_32(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for GrayCodeMsb {
    #[inline(always)]
    fn default() -> GrayCodeMsb {
        GrayCodeMsb(0)
    }
}
impl core::fmt::Debug for GrayCodeMsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GrayCodeMsb")
            .field("code_gray_41_32", &self.code_gray_41_32())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GrayCodeMsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GrayCodeMsb {{ code_gray_41_32: {=u16:?} }}",
            self.code_gray_41_32()
        )
    }
}
#[doc = "I3C Misc Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3cMiscCtrl(pub u32);
impl I3cMiscCtrl {
    #[doc = "Disables/enables the I3C0 filter function on SCL pin."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0_scl_filt(&self) -> super::vals::I3c0SclFilt {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::I3c0SclFilt::from_bits(val as u8)
    }
    #[doc = "Disables/enables the I3C0 filter function on SCL pin."]
    #[inline(always)]
    pub const fn set_i3c0_scl_filt(&mut self, val: super::vals::I3c0SclFilt) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Disables/enables the I3C0 filter function on SDA pin."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0_sda_filt(&self) -> super::vals::I3c0SdaFilt {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::I3c0SdaFilt::from_bits(val as u8)
    }
    #[doc = "Disables/enables the I3C0 filter function on SDA pin."]
    #[inline(always)]
    pub const fn set_i3c0_sda_filt(&mut self, val: super::vals::I3c0SdaFilt) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Disables/enables the I3C1 filter I3C1 function on SCL pin."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c1_scl_filt(&self) -> super::vals::I3c1SclFilt {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::I3c1SclFilt::from_bits(val as u8)
    }
    #[doc = "Disables/enables the I3C1 filter I3C1 function on SCL pin."]
    #[inline(always)]
    pub const fn set_i3c1_scl_filt(&mut self, val: super::vals::I3c1SclFilt) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Disables/enables the I3C1 filter function on SDA pin."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c1_sda_filt(&self) -> super::vals::I3c1SdaFilt {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::I3c1SdaFilt::from_bits(val as u8)
    }
    #[doc = "Disables/enables the I3C1 filter function on SDA pin."]
    #[inline(always)]
    pub const fn set_i3c1_sda_filt(&mut self, val: super::vals::I3c1SdaFilt) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Disables/enables the I3C2 filter function on SCL pin."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c2_scl_filt(&self) -> super::vals::I3c2SclFilt {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::I3c2SclFilt::from_bits(val as u8)
    }
    #[doc = "Disables/enables the I3C2 filter function on SCL pin."]
    #[inline(always)]
    pub const fn set_i3c2_scl_filt(&mut self, val: super::vals::I3c2SclFilt) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Disables/enables the I3C2 filter function on SDA pin."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c2_sda_filt(&self) -> super::vals::I3c2SdaFilt {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::I3c2SdaFilt::from_bits(val as u8)
    }
    #[doc = "Disables/enables the I3C2 filter function on SDA pin."]
    #[inline(always)]
    pub const fn set_i3c2_sda_filt(&mut self, val: super::vals::I3c2SdaFilt) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "Disables/enables the I3C3 filter function on SCL pin."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c3_scl_filt(&self) -> super::vals::I3c3SclFilt {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::I3c3SclFilt::from_bits(val as u8)
    }
    #[doc = "Disables/enables the I3C3 filter function on SCL pin."]
    #[inline(always)]
    pub const fn set_i3c3_scl_filt(&mut self, val: super::vals::I3c3SclFilt) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "Disables/enables the I3C3 filter function on SDA pin."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c3_sda_filt(&self) -> super::vals::I3c3SdaFilt {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::I3c3SdaFilt::from_bits(val as u8)
    }
    #[doc = "Disables/enables the I3C3 filter function on SDA pin."]
    #[inline(always)]
    pub const fn set_i3c3_sda_filt(&mut self, val: super::vals::I3c3SdaFilt) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for I3cMiscCtrl {
    #[inline(always)]
    fn default() -> I3cMiscCtrl {
        I3cMiscCtrl(0)
    }
}
impl core::fmt::Debug for I3cMiscCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3cMiscCtrl")
            .field("i3c0_scl_filt", &self.i3c0_scl_filt())
            .field("i3c0_sda_filt", &self.i3c0_sda_filt())
            .field("i3c1_scl_filt", &self.i3c1_scl_filt())
            .field("i3c1_sda_filt", &self.i3c1_sda_filt())
            .field("i3c2_scl_filt", &self.i3c2_scl_filt())
            .field("i3c2_sda_filt", &self.i3c2_sda_filt())
            .field("i3c3_scl_filt", &self.i3c3_scl_filt())
            .field("i3c3_sda_filt", &self.i3c3_sda_filt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3cMiscCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3cMiscCtrl {{ i3c0_scl_filt: {:?}, i3c0_sda_filt: {:?}, i3c1_scl_filt: {:?}, i3c1_sda_filt: {:?}, i3c2_scl_filt: {:?}, i3c2_sda_filt: {:?}, i3c3_scl_filt: {:?}, i3c3_sda_filt: {:?} }}",
            self.i3c0_scl_filt(),
            self.i3c0_sda_filt(),
            self.i3c1_scl_filt(),
            self.i3c1_sda_filt(),
            self.i3c2_scl_filt(),
            self.i3c2_sda_filt(),
            self.i3c3_scl_filt(),
            self.i3c3_sda_filt()
        )
    }
}
#[doc = "JTAG Chip ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct JtagId(pub u32);
impl JtagId {
    #[doc = "Indicates the device ID"]
    #[must_use]
    #[inline(always)]
    pub const fn jtag_id(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Indicates the device ID"]
    #[inline(always)]
    pub const fn set_jtag_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for JtagId {
    #[inline(always)]
    fn default() -> JtagId {
        JtagId(0)
    }
}
impl core::fmt::Debug for JtagId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JtagId")
            .field("jtag_id", &self.jtag_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for JtagId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "JtagId {{ jtag_id: {=u32:?} }}", self.jtag_id())
    }
}
#[doc = "LPCAC Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpcacCtrl(pub u32);
impl LpcacCtrl {
    #[doc = "Disables/enables the cache function."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_lpcac(&self) -> super::vals::DisLpcac {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::DisLpcac::from_bits(val as u8)
    }
    #[doc = "Disables/enables the cache function."]
    #[inline(always)]
    pub const fn set_dis_lpcac(&mut self, val: super::vals::DisLpcac) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Clears the cache function."]
    #[must_use]
    #[inline(always)]
    pub const fn clr_lpcac(&self) -> super::vals::ClrLpcac {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ClrLpcac::from_bits(val as u8)
    }
    #[doc = "Clears the cache function."]
    #[inline(always)]
    pub const fn set_clr_lpcac(&mut self, val: super::vals::ClrLpcac) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Forces no allocation."]
    #[must_use]
    #[inline(always)]
    pub const fn frc_no_alloc(&self) -> super::vals::FrcNoAlloc {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::FrcNoAlloc::from_bits(val as u8)
    }
    #[doc = "Forces no allocation."]
    #[inline(always)]
    pub const fn set_frc_no_alloc(&mut self, val: super::vals::FrcNoAlloc) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Disable LPCAC Write Through Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_lpcac_wtbf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Disable LPCAC Write Through Buffer."]
    #[inline(always)]
    pub const fn set_dis_lpcac_wtbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Limit LPCAC Write Through Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn lim_lpcac_wtbf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Limit LPCAC Write Through Buffer."]
    #[inline(always)]
    pub const fn set_lim_lpcac_wtbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "LPCAC XOM(eXecute-Only-Memory) attribute control"]
    #[must_use]
    #[inline(always)]
    pub const fn lpcac_xom(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "LPCAC XOM(eXecute-Only-Memory) attribute control"]
    #[inline(always)]
    pub const fn set_lpcac_xom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Request LPCAC memories."]
    #[must_use]
    #[inline(always)]
    pub const fn lpcac_mem_req(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Request LPCAC memories."]
    #[inline(always)]
    pub const fn set_lpcac_mem_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for LpcacCtrl {
    #[inline(always)]
    fn default() -> LpcacCtrl {
        LpcacCtrl(0)
    }
}
impl core::fmt::Debug for LpcacCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpcacCtrl")
            .field("dis_lpcac", &self.dis_lpcac())
            .field("clr_lpcac", &self.clr_lpcac())
            .field("frc_no_alloc", &self.frc_no_alloc())
            .field("dis_lpcac_wtbf", &self.dis_lpcac_wtbf())
            .field("lim_lpcac_wtbf", &self.lim_lpcac_wtbf())
            .field("lpcac_xom", &self.lpcac_xom())
            .field("lpcac_mem_req", &self.lpcac_mem_req())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpcacCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LpcacCtrl {{ dis_lpcac: {:?}, clr_lpcac: {:?}, frc_no_alloc: {:?}, dis_lpcac_wtbf: {=bool:?}, lim_lpcac_wtbf: {=bool:?}, lpcac_xom: {=bool:?}, lpcac_mem_req: {=bool:?} }}",
            self.dis_lpcac(),
            self.clr_lpcac(),
            self.frc_no_alloc(),
            self.dis_lpcac_wtbf(),
            self.lim_lpcac_wtbf(),
            self.lpcac_xom(),
            self.lpcac_mem_req()
        )
    }
}
#[doc = "NMI Source Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nmisrc(pub u32);
impl Nmisrc {
    #[doc = "The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for CPU0, if enabled by NMIENCPU0."]
    #[must_use]
    #[inline(always)]
    pub const fn irqcpu0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for CPU0, if enabled by NMIENCPU0."]
    #[inline(always)]
    pub const fn set_irqcpu0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Enables the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
    #[must_use]
    #[inline(always)]
    pub const fn nmiencpu0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
    #[inline(always)]
    pub const fn set_nmiencpu0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Nmisrc {
    #[inline(always)]
    fn default() -> Nmisrc {
        Nmisrc(0)
    }
}
impl core::fmt::Debug for Nmisrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nmisrc")
            .field("irqcpu0", &self.irqcpu0())
            .field("nmiencpu0", &self.nmiencpu0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nmisrc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Nmisrc {{ irqcpu0: {=u8:?}, nmiencpu0: {=bool:?} }}",
            self.irqcpu0(),
            self.nmiencpu0()
        )
    }
}
#[doc = "NVM Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NvmCtrl(pub u32);
impl NvmCtrl {
    #[doc = "Flash speculation control"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_flash_spec(&self) -> super::vals::DisFlashSpec {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::DisFlashSpec::from_bits(val as u8)
    }
    #[doc = "Flash speculation control"]
    #[inline(always)]
    pub const fn set_dis_flash_spec(&mut self, val: super::vals::DisFlashSpec) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Flash data speculation control"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_data_spec(&self) -> super::vals::DisDataSpec {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::DisDataSpec::from_bits(val as u8)
    }
    #[doc = "Flash data speculation control"]
    #[inline(always)]
    pub const fn set_dis_data_spec(&mut self, val: super::vals::DisDataSpec) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "FLASH stall on busy control"]
    #[must_use]
    #[inline(always)]
    pub const fn flash_stall_en(&self) -> super::vals::FlashStallEn {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::FlashStallEn::from_bits(val as u8)
    }
    #[doc = "FLASH stall on busy control"]
    #[inline(always)]
    pub const fn set_flash_stall_en(&mut self, val: super::vals::FlashStallEn) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Bus error on data multi-bit ECC error control Set this field to 0 if you want to enable flash speculative"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_mbecc_err_inst(&self) -> super::vals::DisMbeccErrInst {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::DisMbeccErrInst::from_bits(val as u8)
    }
    #[doc = "Bus error on data multi-bit ECC error control Set this field to 0 if you want to enable flash speculative"]
    #[inline(always)]
    pub const fn set_dis_mbecc_err_inst(&mut self, val: super::vals::DisMbeccErrInst) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Bus error on data multi-bit ECC error control Set this field to 0 if you want to enable flash speculative"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_mbecc_err_data(&self) -> super::vals::DisMbeccErrData {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::DisMbeccErrData::from_bits(val as u8)
    }
    #[doc = "Bus error on data multi-bit ECC error control Set this field to 0 if you want to enable flash speculative"]
    #[inline(always)]
    pub const fn set_dis_mbecc_err_data(&mut self, val: super::vals::DisMbeccErrData) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
}
impl Default for NvmCtrl {
    #[inline(always)]
    fn default() -> NvmCtrl {
        NvmCtrl(0)
    }
}
impl core::fmt::Debug for NvmCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NvmCtrl")
            .field("dis_flash_spec", &self.dis_flash_spec())
            .field("dis_data_spec", &self.dis_data_spec())
            .field("flash_stall_en", &self.flash_stall_en())
            .field("dis_mbecc_err_inst", &self.dis_mbecc_err_inst())
            .field("dis_mbecc_err_data", &self.dis_mbecc_err_data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NvmCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NvmCtrl {{ dis_flash_spec: {:?}, dis_data_spec: {:?}, flash_stall_en: {:?}, dis_mbecc_err_inst: {:?}, dis_mbecc_err_data: {:?} }}",
            self.dis_flash_spec(),
            self.dis_data_spec(),
            self.flash_stall_en(),
            self.dis_mbecc_err_inst(),
            self.dis_mbecc_err_data()
        )
    }
}
#[doc = "PLL1_CLK_DIV Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll1clkdiv(pub u32);
impl Pll1clkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Pll1clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Pll1clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Pll1clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Pll1clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Pll1clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Pll1clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::Pll1clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Pll1clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::Pll1clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pll1clkdiv {
    #[inline(always)]
    fn default() -> Pll1clkdiv {
        Pll1clkdiv(0)
    }
}
impl core::fmt::Debug for Pll1clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pll1clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pll1clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pll1clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "Protect Level Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Protlvl(pub u32);
impl Protlvl {
    #[doc = "Control privileged access of EIM, ERM, Flexcan, MBC, SCG, DMA, ROMCP and Flexspi."]
    #[must_use]
    #[inline(always)]
    pub const fn priv_(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control privileged access of EIM, ERM, Flexcan, MBC, SCG, DMA, ROMCP and Flexspi."]
    #[inline(always)]
    pub const fn set_priv_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control write access to Nonsecure MPU memory regions."]
    #[must_use]
    #[inline(always)]
    pub const fn locknsmpu(&self) -> super::vals::Locknsmpu {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Locknsmpu::from_bits(val as u8)
    }
    #[doc = "Control write access to Nonsecure MPU memory regions."]
    #[inline(always)]
    pub const fn set_locknsmpu(&mut self, val: super::vals::Locknsmpu) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "This 1-bit field provides a mechanism to limit writes to this register to protect its contents. Once set, this bit remains asserted until a system reset."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::ProtlvlLock {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::ProtlvlLock::from_bits(val as u8)
    }
    #[doc = "This 1-bit field provides a mechanism to limit writes to this register to protect its contents. Once set, this bit remains asserted until a system reset."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: super::vals::ProtlvlLock) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Protlvl {
    #[inline(always)]
    fn default() -> Protlvl {
        Protlvl(0)
    }
}
impl core::fmt::Debug for Protlvl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Protlvl")
            .field("priv_", &self.priv_())
            .field("locknsmpu", &self.locknsmpu())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Protlvl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Protlvl {{ priv_: {=bool:?}, locknsmpu: {:?}, lock: {:?} }}",
            self.priv_(),
            self.locknsmpu(),
            self.lock()
        )
    }
}
#[doc = "Controls Shared RAM Integration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamCaspCtrl(pub u32);
impl RamCaspCtrl {
    #[doc = "Controls RAM access for RAMA1 and RAMA2"]
    #[must_use]
    #[inline(always)]
    pub const fn interleave(&self) -> super::vals::Interleave {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Interleave::from_bits(val as u8)
    }
    #[doc = "Controls RAM access for RAMA1 and RAMA2"]
    #[inline(always)]
    pub const fn set_interleave(&mut self, val: super::vals::Interleave) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Request EZH memories."]
    #[must_use]
    #[inline(always)]
    pub const fn casp_req(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Request EZH memories."]
    #[inline(always)]
    pub const fn set_casp_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for RamCaspCtrl {
    #[inline(always)]
    fn default() -> RamCaspCtrl {
        RamCaspCtrl(0)
    }
}
impl core::fmt::Debug for RamCaspCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamCaspCtrl")
            .field("interleave", &self.interleave())
            .field("casp_req", &self.casp_req())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamCaspCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RamCaspCtrl {{ interleave: {:?}, casp_req: {=bool:?} }}",
            self.interleave(),
            self.casp_req()
        )
    }
}
#[doc = "RAM Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamCtrl(pub u32);
impl RamCtrl {
    #[doc = "RAMA ECC enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rama_ecc_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RAMA ECC enable"]
    #[inline(always)]
    pub const fn set_rama_ecc_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RAMA bank clock gating control, only avaiable when RAMA_ECC_ENABLE = 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rama_cg_override(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "RAMA bank clock gating control, only avaiable when RAMA_ECC_ENABLE = 0."]
    #[inline(always)]
    pub const fn set_rama_cg_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "RAMX bank clock gating control"]
    #[must_use]
    #[inline(always)]
    pub const fn ramx_cg_override(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "RAMX bank clock gating control"]
    #[inline(always)]
    pub const fn set_ramx_cg_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "RAMB bank clock gating control"]
    #[must_use]
    #[inline(always)]
    pub const fn ramb_cg_override(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "RAMB bank clock gating control"]
    #[inline(always)]
    pub const fn set_ramb_cg_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for RamCtrl {
    #[inline(always)]
    fn default() -> RamCtrl {
        RamCtrl(0)
    }
}
impl core::fmt::Debug for RamCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamCtrl")
            .field("rama_ecc_enable", &self.rama_ecc_enable())
            .field("rama_cg_override", &self.rama_cg_override())
            .field("ramx_cg_override", &self.ramx_cg_override())
            .field("ramb_cg_override", &self.ramb_cg_override())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RamCtrl {{ rama_ecc_enable: {=bool:?}, rama_cg_override: {=bool:?}, ramx_cg_override: {=bool:?}, ramb_cg_override: {=bool:?} }}",
            self.rama_ecc_enable(),
            self.rama_cg_override(),
            self.ramx_cg_override(),
            self.ramb_cg_override()
        )
    }
}
#[doc = "AHB Matrix Remap Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Remap(pub u32);
impl Remap {
    #[doc = "RAMX0 address remap for CPU System bus"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_sbus(&self) -> super::vals::RemapCpu0Sbus {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::RemapCpu0Sbus::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for CPU System bus"]
    #[inline(always)]
    pub const fn set_cpu0_sbus(&mut self, val: super::vals::RemapCpu0Sbus) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "RAMX0 address remap for SmartDMA I-BUS"]
    #[must_use]
    #[inline(always)]
    pub const fn smart_dma_i(&self) -> super::vals::SmartDmaI {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SmartDmaI::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for SmartDMA I-BUS"]
    #[inline(always)]
    pub const fn set_smart_dma_i(&mut self, val: super::vals::SmartDmaI) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "RAMX0 address remap for SmartDMA D-BUS"]
    #[must_use]
    #[inline(always)]
    pub const fn smart_dma_d(&self) -> super::vals::SmartDmaD {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::SmartDmaD::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for SmartDMA D-BUS"]
    #[inline(always)]
    pub const fn set_smart_dma_d(&mut self, val: super::vals::SmartDmaD) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "RAMX0 address remap for DMA0"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> super::vals::RemapDma0 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::RemapDma0::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for DMA0"]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: super::vals::RemapDma0) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "RAMX0 address remap for DMA1"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1(&self) -> super::vals::RemapDma1 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::RemapDma1::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for DMA1"]
    #[inline(always)]
    pub const fn set_dma1(&mut self, val: super::vals::RemapDma1) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "RAMX0 address remap for PKC_ELS"]
    #[must_use]
    #[inline(always)]
    pub const fn pkc_els(&self) -> super::vals::RemapPkcEls {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::RemapPkcEls::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for PKC_ELS"]
    #[inline(always)]
    pub const fn set_pkc_els(&mut self, val: super::vals::RemapPkcEls) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "RAMX0 address remap for ESPI"]
    #[must_use]
    #[inline(always)]
    pub const fn coolflux_y_espi(&self) -> super::vals::RemapCoolfluxYEspi {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::RemapCoolfluxYEspi::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for ESPI"]
    #[inline(always)]
    pub const fn set_coolflux_y_espi(&mut self, val: super::vals::RemapCoolfluxYEspi) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "RAMX0 address remap for ENET"]
    #[must_use]
    #[inline(always)]
    pub const fn usb_fs_enet(&self) -> super::vals::RemapUsbFsEnet {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::RemapUsbFsEnet::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for ENET"]
    #[inline(always)]
    pub const fn set_usb_fs_enet(&mut self, val: super::vals::RemapUsbFsEnet) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "RAMX0 address remap for USB-HS"]
    #[must_use]
    #[inline(always)]
    pub const fn usb_hs(&self) -> super::vals::RemapUsbHs {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::RemapUsbHs::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for USB-HS"]
    #[inline(always)]
    pub const fn set_usb_hs(&mut self, val: super::vals::RemapUsbHs) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "This 1-bit field provides a mechanism to limit writes to this register to protect its contents. Once set, this bit remains asserted until a system reset."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::RemapLock {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::RemapLock::from_bits(val as u8)
    }
    #[doc = "This 1-bit field provides a mechanism to limit writes to this register to protect its contents. Once set, this bit remains asserted until a system reset."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: super::vals::RemapLock) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Remap {
    #[inline(always)]
    fn default() -> Remap {
        Remap(0)
    }
}
impl core::fmt::Debug for Remap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Remap")
            .field("cpu0_sbus", &self.cpu0_sbus())
            .field("smart_dma_i", &self.smart_dma_i())
            .field("smart_dma_d", &self.smart_dma_d())
            .field("dma0", &self.dma0())
            .field("dma1", &self.dma1())
            .field("pkc_els", &self.pkc_els())
            .field("coolflux_y_espi", &self.coolflux_y_espi())
            .field("usb_fs_enet", &self.usb_fs_enet())
            .field("usb_hs", &self.usb_hs())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Remap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Remap {{ cpu0_sbus: {:?}, smart_dma_i: {:?}, smart_dma_d: {:?}, dma0: {:?}, dma1: {:?}, pkc_els: {:?}, coolflux_y_espi: {:?}, usb_fs_enet: {:?}, usb_hs: {:?}, lock: {:?} }}",
            self.cpu0_sbus(),
            self.smart_dma_i(),
            self.smart_dma_d(),
            self.dma0(),
            self.dma1(),
            self.pkc_els(),
            self.coolflux_y_espi(),
            self.usb_fs_enet(),
            self.usb_hs(),
            self.lock()
        )
    }
}
#[doc = "SLOW_CLK Clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slowclkdiv(pub u32);
impl Slowclkdiv {
    #[doc = "Clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::SlowclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::SlowclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::SlowclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::SlowclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::SlowclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::SlowclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> super::vals::SlowclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SlowclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag"]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: super::vals::SlowclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Slowclkdiv {
    #[inline(always)]
    fn default() -> Slowclkdiv {
        Slowclkdiv(0)
    }
}
impl core::fmt::Debug for Slowclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Slowclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Slowclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Slowclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "SmartDMA Interrupt Hijack"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmartDmaint(pub u32);
impl SmartDmaint {
    #[doc = "SmartDMA hijack NVIC IRQ2"]
    #[must_use]
    #[inline(always)]
    pub const fn int0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ2"]
    #[inline(always)]
    pub const fn set_int0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ23"]
    #[must_use]
    #[inline(always)]
    pub const fn int1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ23"]
    #[inline(always)]
    pub const fn set_int1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ26"]
    #[must_use]
    #[inline(always)]
    pub const fn int2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ26"]
    #[inline(always)]
    pub const fn set_int2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ27"]
    #[must_use]
    #[inline(always)]
    pub const fn int3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ27"]
    #[inline(always)]
    pub const fn set_int3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ28"]
    #[must_use]
    #[inline(always)]
    pub const fn int4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ28"]
    #[inline(always)]
    pub const fn set_int4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ29"]
    #[must_use]
    #[inline(always)]
    pub const fn int5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ29"]
    #[inline(always)]
    pub const fn set_int5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ31"]
    #[must_use]
    #[inline(always)]
    pub const fn int6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ31"]
    #[inline(always)]
    pub const fn set_int6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ32"]
    #[must_use]
    #[inline(always)]
    pub const fn int7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ32"]
    #[inline(always)]
    pub const fn set_int7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ33"]
    #[must_use]
    #[inline(always)]
    pub const fn int8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ33"]
    #[inline(always)]
    pub const fn set_int8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ34"]
    #[must_use]
    #[inline(always)]
    pub const fn int9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ34"]
    #[inline(always)]
    pub const fn set_int9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ39"]
    #[must_use]
    #[inline(always)]
    pub const fn int11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ39"]
    #[inline(always)]
    pub const fn set_int11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ40"]
    #[must_use]
    #[inline(always)]
    pub const fn int12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ40"]
    #[inline(always)]
    pub const fn set_int12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ41"]
    #[must_use]
    #[inline(always)]
    pub const fn int13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ41"]
    #[inline(always)]
    pub const fn set_int13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ59"]
    #[must_use]
    #[inline(always)]
    pub const fn int14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ59"]
    #[inline(always)]
    pub const fn set_int14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ62"]
    #[must_use]
    #[inline(always)]
    pub const fn int15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ62"]
    #[inline(always)]
    pub const fn set_int15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ64"]
    #[must_use]
    #[inline(always)]
    pub const fn int16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ64"]
    #[inline(always)]
    pub const fn set_int16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ71"]
    #[must_use]
    #[inline(always)]
    pub const fn int17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ71"]
    #[inline(always)]
    pub const fn set_int17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ72"]
    #[must_use]
    #[inline(always)]
    pub const fn int18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ72"]
    #[inline(always)]
    pub const fn set_int18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ73"]
    #[must_use]
    #[inline(always)]
    pub const fn int19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ73"]
    #[inline(always)]
    pub const fn set_int19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ74"]
    #[must_use]
    #[inline(always)]
    pub const fn int20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ74"]
    #[inline(always)]
    pub const fn set_int20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ75"]
    #[must_use]
    #[inline(always)]
    pub const fn int21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ75"]
    #[inline(always)]
    pub const fn set_int21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ103"]
    #[must_use]
    #[inline(always)]
    pub const fn int22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ103"]
    #[inline(always)]
    pub const fn set_int22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ104"]
    #[must_use]
    #[inline(always)]
    pub const fn int23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ104"]
    #[inline(always)]
    pub const fn set_int23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for SmartDmaint {
    #[inline(always)]
    fn default() -> SmartDmaint {
        SmartDmaint(0)
    }
}
impl core::fmt::Debug for SmartDmaint {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmartDmaint")
            .field("int0", &self.int0())
            .field("int1", &self.int1())
            .field("int2", &self.int2())
            .field("int3", &self.int3())
            .field("int4", &self.int4())
            .field("int5", &self.int5())
            .field("int6", &self.int6())
            .field("int7", &self.int7())
            .field("int8", &self.int8())
            .field("int9", &self.int9())
            .field("int11", &self.int11())
            .field("int12", &self.int12())
            .field("int13", &self.int13())
            .field("int14", &self.int14())
            .field("int15", &self.int15())
            .field("int16", &self.int16())
            .field("int17", &self.int17())
            .field("int18", &self.int18())
            .field("int19", &self.int19())
            .field("int20", &self.int20())
            .field("int21", &self.int21())
            .field("int22", &self.int22())
            .field("int23", &self.int23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmartDmaint {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmartDmaint {{ int0: {=bool:?}, int1: {=bool:?}, int2: {=bool:?}, int3: {=bool:?}, int4: {=bool:?}, int5: {=bool:?}, int6: {=bool:?}, int7: {=bool:?}, int8: {=bool:?}, int9: {=bool:?}, int11: {=bool:?}, int12: {=bool:?}, int13: {=bool:?}, int14: {=bool:?}, int15: {=bool:?}, int16: {=bool:?}, int17: {=bool:?}, int18: {=bool:?}, int19: {=bool:?}, int20: {=bool:?}, int21: {=bool:?}, int22: {=bool:?}, int23: {=bool:?} }}",
            self.int0(),
            self.int1(),
            self.int2(),
            self.int3(),
            self.int4(),
            self.int5(),
            self.int6(),
            self.int7(),
            self.int8(),
            self.int9(),
            self.int11(),
            self.int12(),
            self.int13(),
            self.int14(),
            self.int15(),
            self.int16(),
            self.int17(),
            self.int18(),
            self.int19(),
            self.int20(),
            self.int21(),
            self.int22(),
            self.int23()
        )
    }
}
