#[doc = "AHB Peripheral 0 Memory Rule 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheral0MemRule1(pub u32);
impl AhbPeripheral0MemRule1 {
    #[doc = "GPIO0"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0(&self) -> super::vals::Gpio0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Gpio0::from_bits(val as u8)
    }
    #[doc = "GPIO0"]
    #[inline(always)]
    pub const fn set_gpio0(&mut self, val: super::vals::Gpio0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "GPIO0 ALIAS"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio0_alias(&self) -> super::vals::Gpio0Alias {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Gpio0Alias::from_bits(val as u8)
    }
    #[doc = "GPIO0 ALIAS"]
    #[inline(always)]
    pub const fn set_gpio0_alias(&mut self, val: super::vals::Gpio0Alias) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for AhbPeripheral0MemRule1 {
    #[inline(always)]
    fn default() -> AhbPeripheral0MemRule1 {
        AhbPeripheral0MemRule1(0)
    }
}
impl core::fmt::Debug for AhbPeripheral0MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheral0MemRule1")
            .field("gpio0", &self.gpio0())
            .field("gpio0_alias", &self.gpio0_alias())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheral0MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheral0MemRule1 {{ gpio0: {:?}, gpio0_alias: {:?} }}",
            self.gpio0(),
            self.gpio0_alias()
        )
    }
}
#[doc = "AHB Peripheral 0 Memory Rule 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheral0MemRule2(pub u32);
impl AhbPeripheral0MemRule2 {
    #[doc = "GPIO1"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1(&self) -> super::vals::Gpio1 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Gpio1::from_bits(val as u8)
    }
    #[doc = "GPIO1"]
    #[inline(always)]
    pub const fn set_gpio1(&mut self, val: super::vals::Gpio1) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "GPIO1 ALIAS"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio1_alias(&self) -> super::vals::Gpio1Alias {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Gpio1Alias::from_bits(val as u8)
    }
    #[doc = "GPIO1 ALIAS"]
    #[inline(always)]
    pub const fn set_gpio1_alias(&mut self, val: super::vals::Gpio1Alias) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for AhbPeripheral0MemRule2 {
    #[inline(always)]
    fn default() -> AhbPeripheral0MemRule2 {
        AhbPeripheral0MemRule2(0)
    }
}
impl core::fmt::Debug for AhbPeripheral0MemRule2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheral0MemRule2")
            .field("gpio1", &self.gpio1())
            .field("gpio1_alias", &self.gpio1_alias())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheral0MemRule2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheral0MemRule2 {{ gpio1: {:?}, gpio1_alias: {:?} }}",
            self.gpio1(),
            self.gpio1_alias()
        )
    }
}
#[doc = "AHB Peripheral 0 Memory Rule 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheral0MemRule3(pub u32);
impl AhbPeripheral0MemRule3 {
    #[doc = "GPIO2"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> super::vals::Gpio2 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Gpio2::from_bits(val as u8)
    }
    #[doc = "GPIO2"]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: super::vals::Gpio2) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "GPIO2 ALIAS"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2_alias(&self) -> super::vals::Gpio2Alias {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Gpio2Alias::from_bits(val as u8)
    }
    #[doc = "GPIO2 ALIAS"]
    #[inline(always)]
    pub const fn set_gpio2_alias(&mut self, val: super::vals::Gpio2Alias) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for AhbPeripheral0MemRule3 {
    #[inline(always)]
    fn default() -> AhbPeripheral0MemRule3 {
        AhbPeripheral0MemRule3(0)
    }
}
impl core::fmt::Debug for AhbPeripheral0MemRule3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheral0MemRule3")
            .field("gpio2", &self.gpio2())
            .field("gpio2_alias", &self.gpio2_alias())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheral0MemRule3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheral0MemRule3 {{ gpio2: {:?}, gpio2_alias: {:?} }}",
            self.gpio2(),
            self.gpio2_alias()
        )
    }
}
#[doc = "AHB Peripheral 0 Memory Rule 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheral0MemRule4(pub u32);
impl AhbPeripheral0MemRule4 {
    #[doc = "GPIO3"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3(&self) -> super::vals::Gpio3 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Gpio3::from_bits(val as u8)
    }
    #[doc = "GPIO3"]
    #[inline(always)]
    pub const fn set_gpio3(&mut self, val: super::vals::Gpio3) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "GPIO3 ALIAS"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio3_alias(&self) -> super::vals::Gpio3Alias {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Gpio3Alias::from_bits(val as u8)
    }
    #[doc = "GPIO3 ALIAS"]
    #[inline(always)]
    pub const fn set_gpio3_alias(&mut self, val: super::vals::Gpio3Alias) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for AhbPeripheral0MemRule4 {
    #[inline(always)]
    fn default() -> AhbPeripheral0MemRule4 {
        AhbPeripheral0MemRule4(0)
    }
}
impl core::fmt::Debug for AhbPeripheral0MemRule4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheral0MemRule4")
            .field("gpio3", &self.gpio3())
            .field("gpio3_alias", &self.gpio3_alias())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheral0MemRule4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheral0MemRule4 {{ gpio3: {:?}, gpio3_alias: {:?} }}",
            self.gpio3(),
            self.gpio3_alias()
        )
    }
}
#[doc = "AHB Peripheral 0 Memory Rule 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbPeripheral0MemRule5(pub u32);
impl AhbPeripheral0MemRule5 {
    #[doc = "GPIO4"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4(&self) -> super::vals::Gpio4 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Gpio4::from_bits(val as u8)
    }
    #[doc = "GPIO4"]
    #[inline(always)]
    pub const fn set_gpio4(&mut self, val: super::vals::Gpio4) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "GPIO4 ALIAS"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio4_alias(&self) -> super::vals::Gpio4Alias {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Gpio4Alias::from_bits(val as u8)
    }
    #[doc = "GPIO4 ALIAS"]
    #[inline(always)]
    pub const fn set_gpio4_alias(&mut self, val: super::vals::Gpio4Alias) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for AhbPeripheral0MemRule5 {
    #[inline(always)]
    fn default() -> AhbPeripheral0MemRule5 {
        AhbPeripheral0MemRule5(0)
    }
}
impl core::fmt::Debug for AhbPeripheral0MemRule5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbPeripheral0MemRule5")
            .field("gpio4", &self.gpio4())
            .field("gpio4_alias", &self.gpio4_alias())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbPeripheral0MemRule5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbPeripheral0MemRule5 {{ gpio4: {:?}, gpio4_alias: {:?} }}",
            self.gpio4(),
            self.gpio4_alias()
        )
    }
}
#[doc = "AHB Secure Control Peripheral Rule"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbSecureCtrlMemRule0(pub u32);
impl AhbSecureCtrlMemRule0 {
    #[doc = "AHBSC"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::AhbSecureCtrlMemRule0Rule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::AhbSecureCtrlMemRule0Rule0::from_bits(val as u8)
    }
    #[doc = "AHBSC"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::AhbSecureCtrlMemRule0Rule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "AHBSC ALIAS0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::AhbSecureCtrlMemRule0Rule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::AhbSecureCtrlMemRule0Rule1::from_bits(val as u8)
    }
    #[doc = "AHBSC ALIAS0"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::AhbSecureCtrlMemRule0Rule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "AHBSC ALIAS1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::AhbSecureCtrlMemRule0Rule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::AhbSecureCtrlMemRule0Rule2::from_bits(val as u8)
    }
    #[doc = "AHBSC ALIAS1"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::AhbSecureCtrlMemRule0Rule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "AHBSC ALIAS2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::AhbSecureCtrlMemRule0Rule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::AhbSecureCtrlMemRule0Rule3::from_bits(val as u8)
    }
    #[doc = "AHBSC ALIAS2"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::AhbSecureCtrlMemRule0Rule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for AhbSecureCtrlMemRule0 {
    #[inline(always)]
    fn default() -> AhbSecureCtrlMemRule0 {
        AhbSecureCtrlMemRule0(0)
    }
}
impl core::fmt::Debug for AhbSecureCtrlMemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbSecureCtrlMemRule0")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbSecureCtrlMemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbSecureCtrlMemRule0 {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3()
        )
    }
}
#[doc = "AHB Slave Port 5 Rule Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbSlavePortP5SlaveRule0(pub u32);
impl AhbSlavePortP5SlaveRule0 {
    #[doc = "CDOG0"]
    #[must_use]
    #[inline(always)]
    pub const fn cdog0(&self) -> super::vals::Cdog0 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Cdog0::from_bits(val as u8)
    }
    #[doc = "CDOG0"]
    #[inline(always)]
    pub const fn set_cdog0(&mut self, val: super::vals::Cdog0) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "CDOG1"]
    #[must_use]
    #[inline(always)]
    pub const fn cdog1(&self) -> super::vals::Cdog1 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Cdog1::from_bits(val as u8)
    }
    #[doc = "CDOG1"]
    #[inline(always)]
    pub const fn set_cdog1(&mut self, val: super::vals::Cdog1) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "DEBUG_MAILBOX"]
    #[must_use]
    #[inline(always)]
    pub const fn debug_mailbox(&self) -> super::vals::DebugMailbox {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::DebugMailbox::from_bits(val as u8)
    }
    #[doc = "DEBUG_MAILBOX"]
    #[inline(always)]
    pub const fn set_debug_mailbox(&mut self, val: super::vals::DebugMailbox) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn mau0(&self) -> super::vals::Mau0 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Mau0::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_mau0(&mut self, val: super::vals::Mau0) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for AhbSlavePortP5SlaveRule0 {
    #[inline(always)]
    fn default() -> AhbSlavePortP5SlaveRule0 {
        AhbSlavePortP5SlaveRule0(0)
    }
}
impl core::fmt::Debug for AhbSlavePortP5SlaveRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbSlavePortP5SlaveRule0")
            .field("cdog0", &self.cdog0())
            .field("cdog1", &self.cdog1())
            .field("debug_mailbox", &self.debug_mailbox())
            .field("mau0", &self.mau0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbSlavePortP5SlaveRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbSlavePortP5SlaveRule0 {{ cdog0: {:?}, cdog1: {:?}, debug_mailbox: {:?}, mau0: {:?} }}",
            self.cdog0(),
            self.cdog1(),
            self.debug_mailbox(),
            self.mau0()
        )
    }
}
#[doc = "AIPS Bridge Group 0 Memory Rule 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup0MemRule0(pub u32);
impl AipsBridgeGroup0MemRule0 {
    #[doc = "EWM0"]
    #[must_use]
    #[inline(always)]
    pub const fn ewm0(&self) -> super::vals::Ewm0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ewm0::from_bits(val as u8)
    }
    #[doc = "EWM0"]
    #[inline(always)]
    pub const fn set_ewm0(&mut self, val: super::vals::Ewm0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "ROMCP"]
    #[must_use]
    #[inline(always)]
    pub const fn romcp(&self) -> super::vals::Romcp {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Romcp::from_bits(val as u8)
    }
    #[doc = "ROMCP"]
    #[inline(always)]
    pub const fn set_romcp(&mut self, val: super::vals::Romcp) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "PKC0"]
    #[must_use]
    #[inline(always)]
    pub const fn pkc0(&self) -> super::vals::Pkc0 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Pkc0::from_bits(val as u8)
    }
    #[doc = "PKC0"]
    #[inline(always)]
    pub const fn set_pkc0(&mut self, val: super::vals::Pkc0) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "DMA_1_MP"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_1_mp(&self) -> super::vals::Dma1Mp {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Dma1Mp::from_bits(val as u8)
    }
    #[doc = "DMA_1_MP"]
    #[inline(always)]
    pub const fn set_dma_1_mp(&mut self, val: super::vals::Dma1Mp) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "DMA_1_CH0"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_1_ch0(&self) -> super::vals::Dma1Ch0 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Dma1Ch0::from_bits(val as u8)
    }
    #[doc = "DMA_1_CH0"]
    #[inline(always)]
    pub const fn set_dma_1_ch0(&mut self, val: super::vals::Dma1Ch0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "DMA_1_CH1"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_1_ch1(&self) -> super::vals::Dma1Ch1 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Dma1Ch1::from_bits(val as u8)
    }
    #[doc = "DMA_1_CH1"]
    #[inline(always)]
    pub const fn set_dma_1_ch1(&mut self, val: super::vals::Dma1Ch1) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "DMA_1_CH2"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_1_ch2(&self) -> super::vals::Dma1Ch2 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Dma1Ch2::from_bits(val as u8)
    }
    #[doc = "DMA_1_CH2"]
    #[inline(always)]
    pub const fn set_dma_1_ch2(&mut self, val: super::vals::Dma1Ch2) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "DMA_1_CH3"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_1_ch3(&self) -> super::vals::Dma1Ch3 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Dma1Ch3::from_bits(val as u8)
    }
    #[doc = "DMA_1_CH3"]
    #[inline(always)]
    pub const fn set_dma_1_ch3(&mut self, val: super::vals::Dma1Ch3) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup0MemRule0 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup0MemRule0 {
        AipsBridgeGroup0MemRule0(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup0MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup0MemRule0")
            .field("ewm0", &self.ewm0())
            .field("romcp", &self.romcp())
            .field("pkc0", &self.pkc0())
            .field("dma_1_mp", &self.dma_1_mp())
            .field("dma_1_ch0", &self.dma_1_ch0())
            .field("dma_1_ch1", &self.dma_1_ch1())
            .field("dma_1_ch2", &self.dma_1_ch2())
            .field("dma_1_ch3", &self.dma_1_ch3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup0MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup0MemRule0 {{ ewm0: {:?}, romcp: {:?}, pkc0: {:?}, dma_1_mp: {:?}, dma_1_ch0: {:?}, dma_1_ch1: {:?}, dma_1_ch2: {:?}, dma_1_ch3: {:?} }}",
            self.ewm0(),
            self.romcp(),
            self.pkc0(),
            self.dma_1_mp(),
            self.dma_1_ch0(),
            self.dma_1_ch1(),
            self.dma_1_ch2(),
            self.dma_1_ch3()
        )
    }
}
#[doc = "AIPS Bridge Group 0 Memory Rule 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup0MemRule1(pub u32);
impl AipsBridgeGroup0MemRule1 {
    #[doc = "ENET0_0"]
    #[must_use]
    #[inline(always)]
    pub const fn enet0_0(&self) -> super::vals::Enet00 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Enet00::from_bits(val as u8)
    }
    #[doc = "ENET0_0"]
    #[inline(always)]
    pub const fn set_enet0_0(&mut self, val: super::vals::Enet00) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "ENET0_1"]
    #[must_use]
    #[inline(always)]
    pub const fn enet0_1(&self) -> super::vals::Enet01 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Enet01::from_bits(val as u8)
    }
    #[doc = "ENET0_1"]
    #[inline(always)]
    pub const fn set_enet0_1(&mut self, val: super::vals::Enet01) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "eSPI"]
    #[must_use]
    #[inline(always)]
    pub const fn e_spi(&self) -> super::vals::ESpi {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::ESpi::from_bits(val as u8)
    }
    #[doc = "eSPI"]
    #[inline(always)]
    pub const fn set_e_spi(&mut self, val: super::vals::ESpi) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup0MemRule1 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup0MemRule1 {
        AipsBridgeGroup0MemRule1(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup0MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup0MemRule1")
            .field("enet0_0", &self.enet0_0())
            .field("enet0_1", &self.enet0_1())
            .field("e_spi", &self.e_spi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup0MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup0MemRule1 {{ enet0_0: {:?}, enet0_1: {:?}, e_spi: {:?} }}",
            self.enet0_0(),
            self.enet0_1(),
            self.e_spi()
        )
    }
}
#[doc = "AIPS Bridge Group 1 Memory Rule 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup1MemRule0(pub u32);
impl AipsBridgeGroup1MemRule0 {
    #[doc = "FLEXSPI0"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi0(&self) -> super::vals::Flexspi0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flexspi0::from_bits(val as u8)
    }
    #[doc = "FLEXSPI0"]
    #[inline(always)]
    pub const fn set_flexspi0(&mut self, val: super::vals::Flexspi0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "LPSPI2"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi2(&self) -> super::vals::Lpspi2 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Lpspi2::from_bits(val as u8)
    }
    #[doc = "LPSPI2"]
    #[inline(always)]
    pub const fn set_lpspi2(&mut self, val: super::vals::Lpspi2) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "LPSPI3"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi3(&self) -> super::vals::Lpspi3 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Lpspi3::from_bits(val as u8)
    }
    #[doc = "LPSPI3"]
    #[inline(always)]
    pub const fn set_lpspi3(&mut self, val: super::vals::Lpspi3) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "LPSPI4"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi4(&self) -> super::vals::Lpspi4 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Lpspi4::from_bits(val as u8)
    }
    #[doc = "LPSPI4"]
    #[inline(always)]
    pub const fn set_lpspi4(&mut self, val: super::vals::Lpspi4) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "LPSPI5"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi5(&self) -> super::vals::Lpspi5 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Lpspi5::from_bits(val as u8)
    }
    #[doc = "LPSPI5"]
    #[inline(always)]
    pub const fn set_lpspi5(&mut self, val: super::vals::Lpspi5) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
}
impl Default for AipsBridgeGroup1MemRule0 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup1MemRule0 {
        AipsBridgeGroup1MemRule0(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup1MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup1MemRule0")
            .field("flexspi0", &self.flexspi0())
            .field("lpspi2", &self.lpspi2())
            .field("lpspi3", &self.lpspi3())
            .field("lpspi4", &self.lpspi4())
            .field("lpspi5", &self.lpspi5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup1MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup1MemRule0 {{ flexspi0: {:?}, lpspi2: {:?}, lpspi3: {:?}, lpspi4: {:?}, lpspi5: {:?} }}",
            self.flexspi0(),
            self.lpspi2(),
            self.lpspi3(),
            self.lpspi4(),
            self.lpspi5()
        )
    }
}
#[doc = "AIPS Bridge Group 1 Memory Rule 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup1MemRule1(pub u32);
impl AipsBridgeGroup1MemRule1 {
    #[doc = "SPI_FILETER0"]
    #[must_use]
    #[inline(always)]
    pub const fn spi_fileter0(&self) -> super::vals::SpiFileter0 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SpiFileter0::from_bits(val as u8)
    }
    #[doc = "SPI_FILETER0"]
    #[inline(always)]
    pub const fn set_spi_fileter0(&mut self, val: super::vals::SpiFileter0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "10BASE_T1S0"]
    #[must_use]
    #[inline(always)]
    pub const fn t1s0(&self) -> super::vals::T1s0 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::T1s0::from_bits(val as u8)
    }
    #[doc = "10BASE_T1S0"]
    #[inline(always)]
    pub const fn set_t1s0(&mut self, val: super::vals::T1s0) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "USB1"]
    #[must_use]
    #[inline(always)]
    pub const fn usb1(&self) -> super::vals::AipsBridgeGroup1MemRule1Usb1 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::AipsBridgeGroup1MemRule1Usb1::from_bits(val as u8)
    }
    #[doc = "USB1"]
    #[inline(always)]
    pub const fn set_usb1(&mut self, val: super::vals::AipsBridgeGroup1MemRule1Usb1) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "USB1_PHY"]
    #[must_use]
    #[inline(always)]
    pub const fn usb1_phy(&self) -> super::vals::Usb1Phy {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Usb1Phy::from_bits(val as u8)
    }
    #[doc = "USB1_PHY"]
    #[inline(always)]
    pub const fn set_usb1_phy(&mut self, val: super::vals::Usb1Phy) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup1MemRule1 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup1MemRule1 {
        AipsBridgeGroup1MemRule1(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup1MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup1MemRule1")
            .field("spi_fileter0", &self.spi_fileter0())
            .field("t1s0", &self.t1s0())
            .field("usb1", &self.usb1())
            .field("usb1_phy", &self.usb1_phy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup1MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup1MemRule1 {{ spi_fileter0: {:?}, t1s0: {:?}, usb1: {:?}, usb1_phy: {:?} }}",
            self.spi_fileter0(),
            self.t1s0(),
            self.usb1(),
            self.usb1_phy()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule0(pub u32);
impl AipsBridgeGroup2MemRule0 {
    #[doc = "DMA_0_MP"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_mp(&self) -> super::vals::Dma0Mp {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Dma0Mp::from_bits(val as u8)
    }
    #[doc = "DMA_0_MP"]
    #[inline(always)]
    pub const fn set_dma_0_mp(&mut self, val: super::vals::Dma0Mp) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "DMA_0_CH0"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch0(&self) -> super::vals::Dma0Ch0 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Dma0Ch0::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH0"]
    #[inline(always)]
    pub const fn set_dma_0_ch0(&mut self, val: super::vals::Dma0Ch0) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "DMA_0_CH1"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch1(&self) -> super::vals::Dma0Ch1 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Dma0Ch1::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH1"]
    #[inline(always)]
    pub const fn set_dma_0_ch1(&mut self, val: super::vals::Dma0Ch1) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "DMA_0_CH2"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch2(&self) -> super::vals::Dma0Ch2 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Dma0Ch2::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH2"]
    #[inline(always)]
    pub const fn set_dma_0_ch2(&mut self, val: super::vals::Dma0Ch2) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "DMA_0_CH3"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch3(&self) -> super::vals::Dma0Ch3 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Dma0Ch3::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH3"]
    #[inline(always)]
    pub const fn set_dma_0_ch3(&mut self, val: super::vals::Dma0Ch3) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "DMA_0_CH4"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch4(&self) -> super::vals::Dma0Ch4 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Dma0Ch4::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH4"]
    #[inline(always)]
    pub const fn set_dma_0_ch4(&mut self, val: super::vals::Dma0Ch4) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "DMA_0_CH5"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch5(&self) -> super::vals::Dma0Ch5 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Dma0Ch5::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH5"]
    #[inline(always)]
    pub const fn set_dma_0_ch5(&mut self, val: super::vals::Dma0Ch5) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "DMA_0_CH6"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch6(&self) -> super::vals::Dma0Ch6 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Dma0Ch6::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH6"]
    #[inline(always)]
    pub const fn set_dma_0_ch6(&mut self, val: super::vals::Dma0Ch6) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup2MemRule0 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule0 {
        AipsBridgeGroup2MemRule0(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule0")
            .field("dma_0_mp", &self.dma_0_mp())
            .field("dma_0_ch0", &self.dma_0_ch0())
            .field("dma_0_ch1", &self.dma_0_ch1())
            .field("dma_0_ch2", &self.dma_0_ch2())
            .field("dma_0_ch3", &self.dma_0_ch3())
            .field("dma_0_ch4", &self.dma_0_ch4())
            .field("dma_0_ch5", &self.dma_0_ch5())
            .field("dma_0_ch6", &self.dma_0_ch6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule0 {{ dma_0_mp: {:?}, dma_0_ch0: {:?}, dma_0_ch1: {:?}, dma_0_ch2: {:?}, dma_0_ch3: {:?}, dma_0_ch4: {:?}, dma_0_ch5: {:?}, dma_0_ch6: {:?} }}",
            self.dma_0_mp(),
            self.dma_0_ch0(),
            self.dma_0_ch1(),
            self.dma_0_ch2(),
            self.dma_0_ch3(),
            self.dma_0_ch4(),
            self.dma_0_ch5(),
            self.dma_0_ch6()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule1(pub u32);
impl AipsBridgeGroup2MemRule1 {
    #[doc = "DMA_0_CH7"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch7(&self) -> super::vals::Dma0Ch7 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Dma0Ch7::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH7"]
    #[inline(always)]
    pub const fn set_dma_0_ch7(&mut self, val: super::vals::Dma0Ch7) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "DMA_0_CH8"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch8(&self) -> super::vals::Dma0Ch8 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Dma0Ch8::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH8"]
    #[inline(always)]
    pub const fn set_dma_0_ch8(&mut self, val: super::vals::Dma0Ch8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "DMA_0_CH9"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch9(&self) -> super::vals::Dma0Ch9 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Dma0Ch9::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH9"]
    #[inline(always)]
    pub const fn set_dma_0_ch9(&mut self, val: super::vals::Dma0Ch9) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "DMA_0_CH10"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch10(&self) -> super::vals::Dma0Ch10 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Dma0Ch10::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH10"]
    #[inline(always)]
    pub const fn set_dma_0_ch10(&mut self, val: super::vals::Dma0Ch10) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "DMA_0_CH11"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_0_ch11(&self) -> super::vals::Dma0Ch11 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Dma0Ch11::from_bits(val as u8)
    }
    #[doc = "DMA_0_CH11"]
    #[inline(always)]
    pub const fn set_dma_0_ch11(&mut self, val: super::vals::Dma0Ch11) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
}
impl Default for AipsBridgeGroup2MemRule1 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule1 {
        AipsBridgeGroup2MemRule1(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule1")
            .field("dma_0_ch7", &self.dma_0_ch7())
            .field("dma_0_ch8", &self.dma_0_ch8())
            .field("dma_0_ch9", &self.dma_0_ch9())
            .field("dma_0_ch10", &self.dma_0_ch10())
            .field("dma_0_ch11", &self.dma_0_ch11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule1 {{ dma_0_ch7: {:?}, dma_0_ch8: {:?}, dma_0_ch9: {:?}, dma_0_ch10: {:?}, dma_0_ch11: {:?} }}",
            self.dma_0_ch7(),
            self.dma_0_ch8(),
            self.dma_0_ch9(),
            self.dma_0_ch10(),
            self.dma_0_ch11()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 10"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule10(pub u32);
impl AipsBridgeGroup2MemRule10 {
    #[doc = "CAN1 Region 0"]
    #[must_use]
    #[inline(always)]
    pub const fn can1_region0(&self) -> super::vals::Can1Region0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Can1Region0::from_bits(val as u8)
    }
    #[doc = "CAN1 Region 0"]
    #[inline(always)]
    pub const fn set_can1_region0(&mut self, val: super::vals::Can1Region0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CAN1 Region 1"]
    #[must_use]
    #[inline(always)]
    pub const fn can1_region1(&self) -> super::vals::Can1Region1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Can1Region1::from_bits(val as u8)
    }
    #[doc = "CAN1 Region 1"]
    #[inline(always)]
    pub const fn set_can1_region1(&mut self, val: super::vals::Can1Region1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "CAN1 Region 2"]
    #[must_use]
    #[inline(always)]
    pub const fn can1_region2(&self) -> super::vals::Can1Region2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Can1Region2::from_bits(val as u8)
    }
    #[doc = "CAN1 Region 2"]
    #[inline(always)]
    pub const fn set_can1_region2(&mut self, val: super::vals::Can1Region2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CAN1 Region 3"]
    #[must_use]
    #[inline(always)]
    pub const fn can1_region3(&self) -> super::vals::Can1Region3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Can1Region3::from_bits(val as u8)
    }
    #[doc = "CAN1 Region 3"]
    #[inline(always)]
    pub const fn set_can1_region3(&mut self, val: super::vals::Can1Region3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "LPI2C2"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2(&self) -> super::vals::Lpi2c2 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Lpi2c2::from_bits(val as u8)
    }
    #[doc = "LPI2C2"]
    #[inline(always)]
    pub const fn set_lpi2c2(&mut self, val: super::vals::Lpi2c2) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "LPI2C3"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c3(&self) -> super::vals::Lpi2c3 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Lpi2c3::from_bits(val as u8)
    }
    #[doc = "LPI2C3"]
    #[inline(always)]
    pub const fn set_lpi2c3(&mut self, val: super::vals::Lpi2c3) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "LPI2C4"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c4(&self) -> super::vals::Lpi2c4 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Lpi2c4::from_bits(val as u8)
    }
    #[doc = "LPI2C4"]
    #[inline(always)]
    pub const fn set_lpi2c4(&mut self, val: super::vals::Lpi2c4) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for AipsBridgeGroup2MemRule10 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule10 {
        AipsBridgeGroup2MemRule10(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule10")
            .field("can1_region0", &self.can1_region0())
            .field("can1_region1", &self.can1_region1())
            .field("can1_region2", &self.can1_region2())
            .field("can1_region3", &self.can1_region3())
            .field("lpi2c2", &self.lpi2c2())
            .field("lpi2c3", &self.lpi2c3())
            .field("lpi2c4", &self.lpi2c4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule10 {{ can1_region0: {:?}, can1_region1: {:?}, can1_region2: {:?}, can1_region3: {:?}, lpi2c2: {:?}, lpi2c3: {:?}, lpi2c4: {:?} }}",
            self.can1_region0(),
            self.can1_region1(),
            self.can1_region2(),
            self.can1_region3(),
            self.lpi2c2(),
            self.lpi2c3(),
            self.lpi2c4()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 11"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule11(pub u32);
impl AipsBridgeGroup2MemRule11 {
    #[doc = "LPUART5"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart5(&self) -> super::vals::Lpuart5 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Lpuart5::from_bits(val as u8)
    }
    #[doc = "LPUART5"]
    #[inline(always)]
    pub const fn set_lpuart5(&mut self, val: super::vals::Lpuart5) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "I3C3"]
    #[must_use]
    #[inline(always)]
    pub const fn i3c3(&self) -> super::vals::I3c3 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::I3c3::from_bits(val as u8)
    }
    #[doc = "I3C3"]
    #[inline(always)]
    pub const fn set_i3c3(&mut self, val: super::vals::I3c3) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "GPIO5"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio5(&self) -> super::vals::Gpio5 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Gpio5::from_bits(val as u8)
    }
    #[doc = "GPIO5"]
    #[inline(always)]
    pub const fn set_gpio5(&mut self, val: super::vals::Gpio5) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup2MemRule11 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule11 {
        AipsBridgeGroup2MemRule11(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule11")
            .field("lpuart5", &self.lpuart5())
            .field("i3c3", &self.i3c3())
            .field("gpio5", &self.gpio5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule11 {{ lpuart5: {:?}, i3c3: {:?}, gpio5: {:?} }}",
            self.lpuart5(),
            self.i3c3(),
            self.gpio5()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 12"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule12(pub u32);
impl AipsBridgeGroup2MemRule12 {
    #[doc = "GPIO5_ALIAS"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio5_alias(&self) -> super::vals::Gpio5Alias {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Gpio5Alias::from_bits(val as u8)
    }
    #[doc = "GPIO5_ALIAS"]
    #[inline(always)]
    pub const fn set_gpio5_alias(&mut self, val: super::vals::Gpio5Alias) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "PORT5"]
    #[must_use]
    #[inline(always)]
    pub const fn port5(&self) -> super::vals::Port5 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Port5::from_bits(val as u8)
    }
    #[doc = "PORT5"]
    #[inline(always)]
    pub const fn set_port5(&mut self, val: super::vals::Port5) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "DGDET0"]
    #[must_use]
    #[inline(always)]
    pub const fn dgdet0(&self) -> super::vals::Dgdet0 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Dgdet0::from_bits(val as u8)
    }
    #[doc = "DGDET0"]
    #[inline(always)]
    pub const fn set_dgdet0(&mut self, val: super::vals::Dgdet0) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "ITRC0"]
    #[must_use]
    #[inline(always)]
    pub const fn itrc0(&self) -> super::vals::Itrc0 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Itrc0::from_bits(val as u8)
    }
    #[doc = "ITRC0"]
    #[inline(always)]
    pub const fn set_itrc0(&mut self, val: super::vals::Itrc0) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup2MemRule12 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule12 {
        AipsBridgeGroup2MemRule12(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule12")
            .field("gpio5_alias", &self.gpio5_alias())
            .field("port5", &self.port5())
            .field("dgdet0", &self.dgdet0())
            .field("itrc0", &self.itrc0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule12 {{ gpio5_alias: {:?}, port5: {:?}, dgdet0: {:?}, itrc0: {:?} }}",
            self.gpio5_alias(),
            self.port5(),
            self.dgdet0(),
            self.itrc0()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 13"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule13(pub u32);
impl AipsBridgeGroup2MemRule13 {
    #[doc = "GLIKEY0"]
    #[must_use]
    #[inline(always)]
    pub const fn glikey0(&self) -> super::vals::Glikey0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Glikey0::from_bits(val as u8)
    }
    #[doc = "GLIKEY0"]
    #[inline(always)]
    pub const fn set_glikey0(&mut self, val: super::vals::Glikey0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "TDET0"]
    #[must_use]
    #[inline(always)]
    pub const fn tdet0(&self) -> super::vals::Tdet0 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Tdet0::from_bits(val as u8)
    }
    #[doc = "TDET0"]
    #[inline(always)]
    pub const fn set_tdet0(&mut self, val: super::vals::Tdet0) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "SECCON"]
    #[must_use]
    #[inline(always)]
    pub const fn seccon(&self) -> super::vals::Seccon {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Seccon::from_bits(val as u8)
    }
    #[doc = "SECCON"]
    #[inline(always)]
    pub const fn set_seccon(&mut self, val: super::vals::Seccon) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "SGI0"]
    #[must_use]
    #[inline(always)]
    pub const fn sgi0(&self) -> super::vals::Sgi0 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Sgi0::from_bits(val as u8)
    }
    #[doc = "SGI0"]
    #[inline(always)]
    pub const fn set_sgi0(&mut self, val: super::vals::Sgi0) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "TRNG0"]
    #[must_use]
    #[inline(always)]
    pub const fn trng0(&self) -> super::vals::Trng0 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Trng0::from_bits(val as u8)
    }
    #[doc = "TRNG0"]
    #[inline(always)]
    pub const fn set_trng0(&mut self, val: super::vals::Trng0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "UDF0"]
    #[must_use]
    #[inline(always)]
    pub const fn udf0(&self) -> super::vals::Udf0 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Udf0::from_bits(val as u8)
    }
    #[doc = "UDF0"]
    #[inline(always)]
    pub const fn set_udf0(&mut self, val: super::vals::Udf0) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "RTC0"]
    #[must_use]
    #[inline(always)]
    pub const fn rtc0(&self) -> super::vals::Rtc0 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Rtc0::from_bits(val as u8)
    }
    #[doc = "RTC0"]
    #[inline(always)]
    pub const fn set_rtc0(&mut self, val: super::vals::Rtc0) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for AipsBridgeGroup2MemRule13 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule13 {
        AipsBridgeGroup2MemRule13(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule13")
            .field("glikey0", &self.glikey0())
            .field("tdet0", &self.tdet0())
            .field("seccon", &self.seccon())
            .field("sgi0", &self.sgi0())
            .field("trng0", &self.trng0())
            .field("udf0", &self.udf0())
            .field("rtc0", &self.rtc0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule13 {{ glikey0: {:?}, tdet0: {:?}, seccon: {:?}, sgi0: {:?}, trng0: {:?}, udf0: {:?}, rtc0: {:?} }}",
            self.glikey0(),
            self.tdet0(),
            self.seccon(),
            self.sgi0(),
            self.trng0(),
            self.udf0(),
            self.rtc0()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule2(pub u32);
impl AipsBridgeGroup2MemRule2 {
    #[doc = "SYSCON"]
    #[must_use]
    #[inline(always)]
    pub const fn syscon(&self) -> super::vals::Syscon {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Syscon::from_bits(val as u8)
    }
    #[doc = "SYSCON"]
    #[inline(always)]
    pub const fn set_syscon(&mut self, val: super::vals::Syscon) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "WUU"]
    #[must_use]
    #[inline(always)]
    pub const fn wuu(&self) -> super::vals::Wuu {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Wuu::from_bits(val as u8)
    }
    #[doc = "WUU"]
    #[inline(always)]
    pub const fn set_wuu(&mut self, val: super::vals::Wuu) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "VBAT"]
    #[must_use]
    #[inline(always)]
    pub const fn vbat(&self) -> super::vals::Vbat {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Vbat::from_bits(val as u8)
    }
    #[doc = "VBAT"]
    #[inline(always)]
    pub const fn set_vbat(&mut self, val: super::vals::Vbat) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "FMC"]
    #[must_use]
    #[inline(always)]
    pub const fn fmc(&self) -> super::vals::Fmc {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Fmc::from_bits(val as u8)
    }
    #[doc = "FMC"]
    #[inline(always)]
    pub const fn set_fmc(&mut self, val: super::vals::Fmc) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "FMU"]
    #[must_use]
    #[inline(always)]
    pub const fn fmu(&self) -> super::vals::Fmu {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Fmu::from_bits(val as u8)
    }
    #[doc = "FMU"]
    #[inline(always)]
    pub const fn set_fmu(&mut self, val: super::vals::Fmu) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
}
impl Default for AipsBridgeGroup2MemRule2 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule2 {
        AipsBridgeGroup2MemRule2(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule2")
            .field("syscon", &self.syscon())
            .field("wuu", &self.wuu())
            .field("vbat", &self.vbat())
            .field("fmc", &self.fmc())
            .field("fmu", &self.fmu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule2 {{ syscon: {:?}, wuu: {:?}, vbat: {:?}, fmc: {:?}, fmu: {:?} }}",
            self.syscon(),
            self.wuu(),
            self.vbat(),
            self.fmc(),
            self.fmu()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule3(pub u32);
impl AipsBridgeGroup2MemRule3 {
    #[doc = "FLEXIO"]
    #[must_use]
    #[inline(always)]
    pub const fn flexio(&self) -> super::vals::Flexio {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Flexio::from_bits(val as u8)
    }
    #[doc = "FLEXIO"]
    #[inline(always)]
    pub const fn set_flexio(&mut self, val: super::vals::Flexio) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "LPI2C0"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c0(&self) -> super::vals::Lpi2c0 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Lpi2c0::from_bits(val as u8)
    }
    #[doc = "LPI2C0"]
    #[inline(always)]
    pub const fn set_lpi2c0(&mut self, val: super::vals::Lpi2c0) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "LPI2C1"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c1(&self) -> super::vals::Lpi2c1 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Lpi2c1::from_bits(val as u8)
    }
    #[doc = "LPI2C1"]
    #[inline(always)]
    pub const fn set_lpi2c1(&mut self, val: super::vals::Lpi2c1) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "LPSPI0"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi0(&self) -> super::vals::Lpspi0 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Lpspi0::from_bits(val as u8)
    }
    #[doc = "LPSPI0"]
    #[inline(always)]
    pub const fn set_lpspi0(&mut self, val: super::vals::Lpspi0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "LPSPI1"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1(&self) -> super::vals::Lpspi1 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Lpspi1::from_bits(val as u8)
    }
    #[doc = "LPSPI1"]
    #[inline(always)]
    pub const fn set_lpspi1(&mut self, val: super::vals::Lpspi1) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "I3C2"]
    #[must_use]
    #[inline(always)]
    pub const fn i3c2(&self) -> super::vals::I3c2 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::I3c2::from_bits(val as u8)
    }
    #[doc = "I3C2"]
    #[inline(always)]
    pub const fn set_i3c2(&mut self, val: super::vals::I3c2) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "LPUART0"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart0(&self) -> super::vals::Lpuart0 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Lpuart0::from_bits(val as u8)
    }
    #[doc = "LPUART0"]
    #[inline(always)]
    pub const fn set_lpuart0(&mut self, val: super::vals::Lpuart0) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup2MemRule3 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule3 {
        AipsBridgeGroup2MemRule3(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule3")
            .field("flexio", &self.flexio())
            .field("lpi2c0", &self.lpi2c0())
            .field("lpi2c1", &self.lpi2c1())
            .field("lpspi0", &self.lpspi0())
            .field("lpspi1", &self.lpspi1())
            .field("i3c2", &self.i3c2())
            .field("lpuart0", &self.lpuart0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule3 {{ flexio: {:?}, lpi2c0: {:?}, lpi2c1: {:?}, lpspi0: {:?}, lpspi1: {:?}, i3c2: {:?}, lpuart0: {:?} }}",
            self.flexio(),
            self.lpi2c0(),
            self.lpi2c1(),
            self.lpspi0(),
            self.lpspi1(),
            self.i3c2(),
            self.lpuart0()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule4(pub u32);
impl AipsBridgeGroup2MemRule4 {
    #[doc = "LPUART1"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1(&self) -> super::vals::Lpuart1 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpuart1::from_bits(val as u8)
    }
    #[doc = "LPUART1"]
    #[inline(always)]
    pub const fn set_lpuart1(&mut self, val: super::vals::Lpuart1) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "LPUART2"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2(&self) -> super::vals::Lpuart2 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Lpuart2::from_bits(val as u8)
    }
    #[doc = "LPUART2"]
    #[inline(always)]
    pub const fn set_lpuart2(&mut self, val: super::vals::Lpuart2) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "LPUART3"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3(&self) -> super::vals::Lpuart3 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Lpuart3::from_bits(val as u8)
    }
    #[doc = "LPUART3"]
    #[inline(always)]
    pub const fn set_lpuart3(&mut self, val: super::vals::Lpuart3) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "LPUART4"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4(&self) -> super::vals::Lpuart4 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Lpuart4::from_bits(val as u8)
    }
    #[doc = "LPUART4"]
    #[inline(always)]
    pub const fn set_lpuart4(&mut self, val: super::vals::Lpuart4) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for AipsBridgeGroup2MemRule4 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule4 {
        AipsBridgeGroup2MemRule4(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule4")
            .field("lpuart1", &self.lpuart1())
            .field("lpuart2", &self.lpuart2())
            .field("lpuart3", &self.lpuart3())
            .field("lpuart4", &self.lpuart4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule4 {{ lpuart1: {:?}, lpuart2: {:?}, lpuart3: {:?}, lpuart4: {:?} }}",
            self.lpuart1(),
            self.lpuart2(),
            self.lpuart3(),
            self.lpuart4()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule5(pub u32);
impl AipsBridgeGroup2MemRule5 {
    #[doc = "LPTMR"]
    #[must_use]
    #[inline(always)]
    pub const fn lptmr(&self) -> super::vals::Lptmr {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Lptmr::from_bits(val as u8)
    }
    #[doc = "LPTMR"]
    #[inline(always)]
    pub const fn set_lptmr(&mut self, val: super::vals::Lptmr) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "OSTIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer(&self) -> super::vals::Ostimer {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ostimer::from_bits(val as u8)
    }
    #[doc = "OSTIMER"]
    #[inline(always)]
    pub const fn set_ostimer(&mut self, val: super::vals::Ostimer) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "WAKE_TIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn wake_timer(&self) -> super::vals::WakeTimer {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::WakeTimer::from_bits(val as u8)
    }
    #[doc = "WAKE_TIMER"]
    #[inline(always)]
    pub const fn set_wake_timer(&mut self, val: super::vals::WakeTimer) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "ADC0"]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> super::vals::Adc0 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Adc0::from_bits(val as u8)
    }
    #[doc = "ADC0"]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: super::vals::Adc0) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup2MemRule5 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule5 {
        AipsBridgeGroup2MemRule5(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule5")
            .field("lptmr", &self.lptmr())
            .field("ostimer", &self.ostimer())
            .field("wake_timer", &self.wake_timer())
            .field("adc0", &self.adc0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule5 {{ lptmr: {:?}, ostimer: {:?}, wake_timer: {:?}, adc0: {:?} }}",
            self.lptmr(),
            self.ostimer(),
            self.wake_timer(),
            self.adc0()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule6(pub u32);
impl AipsBridgeGroup2MemRule6 {
    #[doc = "ADC1"]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> super::vals::Adc1 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Adc1::from_bits(val as u8)
    }
    #[doc = "ADC1"]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: super::vals::Adc1) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CMP0"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp0(&self) -> super::vals::Cmp0 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Cmp0::from_bits(val as u8)
    }
    #[doc = "CMP0"]
    #[inline(always)]
    pub const fn set_cmp0(&mut self, val: super::vals::Cmp0) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "DAC0"]
    #[must_use]
    #[inline(always)]
    pub const fn dac0(&self) -> super::vals::Dac0 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Dac0::from_bits(val as u8)
    }
    #[doc = "DAC0"]
    #[inline(always)]
    pub const fn set_dac0(&mut self, val: super::vals::Dac0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "DAC1"]
    #[must_use]
    #[inline(always)]
    pub const fn dac1(&self) -> super::vals::Dac1 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Dac1::from_bits(val as u8)
    }
    #[doc = "DAC1"]
    #[inline(always)]
    pub const fn set_dac1(&mut self, val: super::vals::Dac1) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
}
impl Default for AipsBridgeGroup2MemRule6 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule6 {
        AipsBridgeGroup2MemRule6(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule6")
            .field("adc1", &self.adc1())
            .field("cmp0", &self.cmp0())
            .field("dac0", &self.dac0())
            .field("dac1", &self.dac1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule6 {{ adc1: {:?}, cmp0: {:?}, dac0: {:?}, dac1: {:?} }}",
            self.adc1(),
            self.cmp0(),
            self.dac0(),
            self.dac1()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule7(pub u32);
impl AipsBridgeGroup2MemRule7 {
    #[doc = "VREF0"]
    #[must_use]
    #[inline(always)]
    pub const fn vref0(&self) -> super::vals::Vref0 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Vref0::from_bits(val as u8)
    }
    #[doc = "VREF0"]
    #[inline(always)]
    pub const fn set_vref0(&mut self, val: super::vals::Vref0) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "PORT0"]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> super::vals::Port0 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Port0::from_bits(val as u8)
    }
    #[doc = "PORT0"]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: super::vals::Port0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "PORT1"]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> super::vals::Port1 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Port1::from_bits(val as u8)
    }
    #[doc = "PORT1"]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: super::vals::Port1) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "PORT2"]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> super::vals::Port2 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Port2::from_bits(val as u8)
    }
    #[doc = "PORT2"]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: super::vals::Port2) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "PORT3"]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> super::vals::Port3 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Port3::from_bits(val as u8)
    }
    #[doc = "PORT3"]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: super::vals::Port3) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup2MemRule7 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule7 {
        AipsBridgeGroup2MemRule7(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule7")
            .field("vref0", &self.vref0())
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule7 {{ vref0: {:?}, port0: {:?}, port1: {:?}, port2: {:?}, port3: {:?} }}",
            self.vref0(),
            self.port0(),
            self.port1(),
            self.port2(),
            self.port3()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 8"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule8(pub u32);
impl AipsBridgeGroup2MemRule8 {
    #[doc = "PORT4"]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> super::vals::Port4 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Port4::from_bits(val as u8)
    }
    #[doc = "PORT4"]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: super::vals::Port4) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "TSI0"]
    #[must_use]
    #[inline(always)]
    pub const fn tsi0(&self) -> super::vals::Tsi0 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Tsi0::from_bits(val as u8)
    }
    #[doc = "TSI0"]
    #[inline(always)]
    pub const fn set_tsi0(&mut self, val: super::vals::Tsi0) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "AOI0"]
    #[must_use]
    #[inline(always)]
    pub const fn aoi0(&self) -> super::vals::Aoi0 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Aoi0::from_bits(val as u8)
    }
    #[doc = "AOI0"]
    #[inline(always)]
    pub const fn set_aoi0(&mut self, val: super::vals::Aoi0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "CRC0"]
    #[must_use]
    #[inline(always)]
    pub const fn crc0(&self) -> super::vals::Crc0 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Crc0::from_bits(val as u8)
    }
    #[doc = "CRC0"]
    #[inline(always)]
    pub const fn set_crc0(&mut self, val: super::vals::Crc0) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "CMC"]
    #[must_use]
    #[inline(always)]
    pub const fn cmc(&self) -> super::vals::Cmc {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Cmc::from_bits(val as u8)
    }
    #[doc = "CMC"]
    #[inline(always)]
    pub const fn set_cmc(&mut self, val: super::vals::Cmc) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "EIM"]
    #[must_use]
    #[inline(always)]
    pub const fn eim(&self) -> super::vals::Eim {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Eim::from_bits(val as u8)
    }
    #[doc = "EIM"]
    #[inline(always)]
    pub const fn set_eim(&mut self, val: super::vals::Eim) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup2MemRule8 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule8 {
        AipsBridgeGroup2MemRule8(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule8")
            .field("port4", &self.port4())
            .field("tsi0", &self.tsi0())
            .field("aoi0", &self.aoi0())
            .field("crc0", &self.crc0())
            .field("cmc", &self.cmc())
            .field("eim", &self.eim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule8 {{ port4: {:?}, tsi0: {:?}, aoi0: {:?}, crc0: {:?}, cmc: {:?}, eim: {:?} }}",
            self.port4(),
            self.tsi0(),
            self.aoi0(),
            self.crc0(),
            self.cmc(),
            self.eim()
        )
    }
}
#[doc = "AIPS Bridge Group 2 Rule 9"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AipsBridgeGroup2MemRule9(pub u32);
impl AipsBridgeGroup2MemRule9 {
    #[doc = "ERM"]
    #[must_use]
    #[inline(always)]
    pub const fn erm(&self) -> super::vals::Erm {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Erm::from_bits(val as u8)
    }
    #[doc = "ERM"]
    #[inline(always)]
    pub const fn set_erm(&mut self, val: super::vals::Erm) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "MBC"]
    #[must_use]
    #[inline(always)]
    pub const fn mbc(&self) -> super::vals::Mbc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Mbc::from_bits(val as u8)
    }
    #[doc = "MBC"]
    #[inline(always)]
    pub const fn set_mbc(&mut self, val: super::vals::Mbc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "SCG"]
    #[must_use]
    #[inline(always)]
    pub const fn scg(&self) -> super::vals::Scg {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Scg::from_bits(val as u8)
    }
    #[doc = "SCG"]
    #[inline(always)]
    pub const fn set_scg(&mut self, val: super::vals::Scg) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "SPC"]
    #[must_use]
    #[inline(always)]
    pub const fn spc(&self) -> super::vals::Spc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Spc::from_bits(val as u8)
    }
    #[doc = "SPC"]
    #[inline(always)]
    pub const fn set_spc(&mut self, val: super::vals::Spc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "CAN0 Region 0"]
    #[must_use]
    #[inline(always)]
    pub const fn can0_region0(&self) -> super::vals::Can0Region0 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Can0Region0::from_bits(val as u8)
    }
    #[doc = "CAN0 Region 0"]
    #[inline(always)]
    pub const fn set_can0_region0(&mut self, val: super::vals::Can0Region0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "CAN0 Region 1"]
    #[must_use]
    #[inline(always)]
    pub const fn can0_region1(&self) -> super::vals::Can0Region1 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Can0Region1::from_bits(val as u8)
    }
    #[doc = "CAN0 Region 1"]
    #[inline(always)]
    pub const fn set_can0_region1(&mut self, val: super::vals::Can0Region1) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "CAN0 Region 2"]
    #[must_use]
    #[inline(always)]
    pub const fn can0_region2(&self) -> super::vals::Can0Region2 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Can0Region2::from_bits(val as u8)
    }
    #[doc = "CAN0 Region 2"]
    #[inline(always)]
    pub const fn set_can0_region2(&mut self, val: super::vals::Can0Region2) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "CAN0 Region 3"]
    #[must_use]
    #[inline(always)]
    pub const fn can0_region3(&self) -> super::vals::Can0Region3 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Can0Region3::from_bits(val as u8)
    }
    #[doc = "CAN0 Region 3"]
    #[inline(always)]
    pub const fn set_can0_region3(&mut self, val: super::vals::Can0Region3) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for AipsBridgeGroup2MemRule9 {
    #[inline(always)]
    fn default() -> AipsBridgeGroup2MemRule9 {
        AipsBridgeGroup2MemRule9(0)
    }
}
impl core::fmt::Debug for AipsBridgeGroup2MemRule9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AipsBridgeGroup2MemRule9")
            .field("erm", &self.erm())
            .field("mbc", &self.mbc())
            .field("scg", &self.scg())
            .field("spc", &self.spc())
            .field("can0_region0", &self.can0_region0())
            .field("can0_region1", &self.can0_region1())
            .field("can0_region2", &self.can0_region2())
            .field("can0_region3", &self.can0_region3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AipsBridgeGroup2MemRule9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AipsBridgeGroup2MemRule9 {{ erm: {:?}, mbc: {:?}, scg: {:?}, spc: {:?}, can0_region0: {:?}, can0_region1: {:?}, can0_region2: {:?}, can0_region3: {:?} }}",
            self.erm(),
            self.mbc(),
            self.scg(),
            self.spc(),
            self.can0_region0(),
            self.can0_region1(),
            self.can0_region2(),
            self.can0_region3()
        )
    }
}
#[doc = "APB Bridge Group 0 Memory Rule Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbPeripheralGroup0MemRule0(pub u32);
impl ApbPeripheralGroup0MemRule0 {
    #[doc = "INPUTMUX"]
    #[must_use]
    #[inline(always)]
    pub const fn inputmux(&self) -> super::vals::Inputmux {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Inputmux::from_bits(val as u8)
    }
    #[doc = "INPUTMUX"]
    #[inline(always)]
    pub const fn set_inputmux(&mut self, val: super::vals::Inputmux) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "I3C0"]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0(&self) -> super::vals::I3c0 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::I3c0::from_bits(val as u8)
    }
    #[doc = "I3C0"]
    #[inline(always)]
    pub const fn set_i3c0(&mut self, val: super::vals::I3c0) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "I3C1"]
    #[must_use]
    #[inline(always)]
    pub const fn i3c1(&self) -> super::vals::I3c1 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::I3c1::from_bits(val as u8)
    }
    #[doc = "I3C1"]
    #[inline(always)]
    pub const fn set_i3c1(&mut self, val: super::vals::I3c1) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "CTIMER0"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer0(&self) -> super::vals::Ctimer0 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ctimer0::from_bits(val as u8)
    }
    #[doc = "CTIMER0"]
    #[inline(always)]
    pub const fn set_ctimer0(&mut self, val: super::vals::Ctimer0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "CTIMER1"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer1(&self) -> super::vals::Ctimer1 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Ctimer1::from_bits(val as u8)
    }
    #[doc = "CTIMER1"]
    #[inline(always)]
    pub const fn set_ctimer1(&mut self, val: super::vals::Ctimer1) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "CTIMER2"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer2(&self) -> super::vals::Ctimer2 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Ctimer2::from_bits(val as u8)
    }
    #[doc = "CTIMER2"]
    #[inline(always)]
    pub const fn set_ctimer2(&mut self, val: super::vals::Ctimer2) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "CTIMER3"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer3(&self) -> super::vals::Ctimer3 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Ctimer3::from_bits(val as u8)
    }
    #[doc = "CTIMER3"]
    #[inline(always)]
    pub const fn set_ctimer3(&mut self, val: super::vals::Ctimer3) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for ApbPeripheralGroup0MemRule0 {
    #[inline(always)]
    fn default() -> ApbPeripheralGroup0MemRule0 {
        ApbPeripheralGroup0MemRule0(0)
    }
}
impl core::fmt::Debug for ApbPeripheralGroup0MemRule0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ApbPeripheralGroup0MemRule0")
            .field("inputmux", &self.inputmux())
            .field("i3c0", &self.i3c0())
            .field("i3c1", &self.i3c1())
            .field("ctimer0", &self.ctimer0())
            .field("ctimer1", &self.ctimer1())
            .field("ctimer2", &self.ctimer2())
            .field("ctimer3", &self.ctimer3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbPeripheralGroup0MemRule0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApbPeripheralGroup0MemRule0 {{ inputmux: {:?}, i3c0: {:?}, i3c1: {:?}, ctimer0: {:?}, ctimer1: {:?}, ctimer2: {:?}, ctimer3: {:?} }}",
            self.inputmux(),
            self.i3c0(),
            self.i3c1(),
            self.ctimer0(),
            self.ctimer1(),
            self.ctimer2(),
            self.ctimer3()
        )
    }
}
#[doc = "APB Bridge Group 0 Memory Rule Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbPeripheralGroup0MemRule1(pub u32);
impl ApbPeripheralGroup0MemRule1 {
    #[doc = "CTIMER4"]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer4(&self) -> super::vals::Ctimer4 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ctimer4::from_bits(val as u8)
    }
    #[doc = "CTIMER4"]
    #[inline(always)]
    pub const fn set_ctimer4(&mut self, val: super::vals::Ctimer4) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "FREQME"]
    #[must_use]
    #[inline(always)]
    pub const fn freqme(&self) -> super::vals::Freqme {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Freqme::from_bits(val as u8)
    }
    #[doc = "FREQME"]
    #[inline(always)]
    pub const fn set_freqme(&mut self, val: super::vals::Freqme) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "UTCIK0"]
    #[must_use]
    #[inline(always)]
    pub const fn utick(&self) -> super::vals::Utick {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Utick::from_bits(val as u8)
    }
    #[doc = "UTCIK0"]
    #[inline(always)]
    pub const fn set_utick(&mut self, val: super::vals::Utick) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "WWDT0"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0(&self) -> super::vals::Wwdt0 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Wwdt0::from_bits(val as u8)
    }
    #[doc = "WWDT0"]
    #[inline(always)]
    pub const fn set_wwdt0(&mut self, val: super::vals::Wwdt0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "WWDT1"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt1(&self) -> super::vals::Wwdt1 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Wwdt1::from_bits(val as u8)
    }
    #[doc = "WWDT1"]
    #[inline(always)]
    pub const fn set_wwdt1(&mut self, val: super::vals::Wwdt1) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "SmartDMA"]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma(&self) -> super::vals::ApbPeripheralGroup0MemRule1Smartdma {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::ApbPeripheralGroup0MemRule1Smartdma::from_bits(val as u8)
    }
    #[doc = "SmartDMA"]
    #[inline(always)]
    pub const fn set_smartdma(&mut self, val: super::vals::ApbPeripheralGroup0MemRule1Smartdma) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for ApbPeripheralGroup0MemRule1 {
    #[inline(always)]
    fn default() -> ApbPeripheralGroup0MemRule1 {
        ApbPeripheralGroup0MemRule1(0)
    }
}
impl core::fmt::Debug for ApbPeripheralGroup0MemRule1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ApbPeripheralGroup0MemRule1")
            .field("ctimer4", &self.ctimer4())
            .field("freqme", &self.freqme())
            .field("utick", &self.utick())
            .field("wwdt0", &self.wwdt0())
            .field("wwdt1", &self.wwdt1())
            .field("smartdma", &self.smartdma())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbPeripheralGroup0MemRule1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ApbPeripheralGroup0MemRule1 {{ ctimer4: {:?}, freqme: {:?}, utick: {:?}, wwdt0: {:?}, wwdt1: {:?}, smartdma: {:?} }}",
            self.ctimer4(),
            self.freqme(),
            self.utick(),
            self.wwdt0(),
            self.wwdt1(),
            self.smartdma()
        )
    }
}
#[doc = "Miscellaneous CPU0 Control Signals"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu0LockReg(pub u32);
impl Cpu0LockReg {
    #[doc = "LOCK_NS_VTOR"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ns_vtor(&self) -> super::vals::LockNsVtor {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::LockNsVtor::from_bits(val as u8)
    }
    #[doc = "LOCK_NS_VTOR"]
    #[inline(always)]
    pub const fn set_lock_ns_vtor(&mut self, val: super::vals::LockNsVtor) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "LOCK_NS_MPU"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ns_mpu(&self) -> super::vals::LockNsMpu {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::LockNsMpu::from_bits(val as u8)
    }
    #[doc = "LOCK_NS_MPU"]
    #[inline(always)]
    pub const fn set_lock_ns_mpu(&mut self, val: super::vals::LockNsMpu) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "LOCK_S_VTAIRCR"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_s_vtaircr(&self) -> super::vals::LockSVtaircr {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::LockSVtaircr::from_bits(val as u8)
    }
    #[doc = "LOCK_S_VTAIRCR"]
    #[inline(always)]
    pub const fn set_lock_s_vtaircr(&mut self, val: super::vals::LockSVtaircr) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "LOCK_S_MPU"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_s_mpu(&self) -> super::vals::LockSMpu {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::LockSMpu::from_bits(val as u8)
    }
    #[doc = "LOCK_S_MPU"]
    #[inline(always)]
    pub const fn set_lock_s_mpu(&mut self, val: super::vals::LockSMpu) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "LOCK_SAU"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_sau(&self) -> super::vals::LockSau {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::LockSau::from_bits(val as u8)
    }
    #[doc = "LOCK_SAU"]
    #[inline(always)]
    pub const fn set_lock_sau(&mut self, val: super::vals::LockSau) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
}
impl Default for Cpu0LockReg {
    #[inline(always)]
    fn default() -> Cpu0LockReg {
        Cpu0LockReg(0)
    }
}
impl core::fmt::Debug for Cpu0LockReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpu0LockReg")
            .field("lock_ns_vtor", &self.lock_ns_vtor())
            .field("lock_ns_mpu", &self.lock_ns_mpu())
            .field("lock_s_vtaircr", &self.lock_s_vtaircr())
            .field("lock_s_mpu", &self.lock_s_mpu())
            .field("lock_sau", &self.lock_sau())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpu0LockReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpu0LockReg {{ lock_ns_vtor: {:?}, lock_ns_mpu: {:?}, lock_s_vtaircr: {:?}, lock_s_mpu: {:?}, lock_sau: {:?} }}",
            self.lock_ns_vtor(),
            self.lock_ns_mpu(),
            self.lock_s_vtaircr(),
            self.lock_s_mpu(),
            self.lock_sau()
        )
    }
}
#[doc = "Flash Memory Rule"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash00MemRule(pub u32);
impl Flash00MemRule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Flash00MemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flash00MemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Flash00MemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Flash00MemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Flash00MemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Flash00MemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Flash00MemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Flash00MemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Flash00MemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Flash00MemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Flash00MemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Flash00MemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Flash00MemRuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Flash00MemRuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Flash00MemRuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Flash00MemRuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Flash00MemRuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Flash00MemRuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Flash00MemRuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Flash00MemRuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Flash00MemRuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Flash00MemRuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Flash00MemRuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Flash00MemRuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Flash00MemRule {
    #[inline(always)]
    fn default() -> Flash00MemRule {
        Flash00MemRule(0)
    }
}
impl core::fmt::Debug for Flash00MemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flash00MemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flash00MemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flash00MemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "Flash Memory Rule"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash01MemRule(pub u32);
impl Flash01MemRule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Flash01MemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flash01MemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Flash01MemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Flash01MemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Flash01MemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Flash01MemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Flash01MemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Flash01MemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Flash01MemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Flash01MemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Flash01MemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Flash01MemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Flash01MemRuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Flash01MemRuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Flash01MemRuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Flash01MemRuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Flash01MemRuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Flash01MemRuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Flash01MemRuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Flash01MemRuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Flash01MemRuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Flash01MemRuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Flash01MemRuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Flash01MemRuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Flash01MemRule {
    #[inline(always)]
    fn default() -> Flash01MemRule {
        Flash01MemRule(0)
    }
}
impl core::fmt::Debug for Flash01MemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flash01MemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flash01MemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flash01MemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "Flash IFR0 Rule register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash02MemRule(pub u32);
impl Flash02MemRule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Flash02MemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flash02MemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Flash02MemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Flash02MemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Flash02MemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Flash02MemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Flash02MemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Flash02MemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Flash02MemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Flash02MemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Flash02MemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Flash02MemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for Flash02MemRule {
    #[inline(always)]
    fn default() -> Flash02MemRule {
        Flash02MemRule(0)
    }
}
impl core::fmt::Debug for Flash02MemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flash02MemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flash02MemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flash02MemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3()
        )
    }
}
#[doc = "Flash Memory Rule"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash03MemRule(pub u32);
impl Flash03MemRule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Flash03MemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flash03MemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Flash03MemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Flash03MemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Flash03MemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Flash03MemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Flash03MemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Flash03MemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Flash03MemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Flash03MemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Flash03MemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Flash03MemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for Flash03MemRule {
    #[inline(always)]
    fn default() -> Flash03MemRule {
        Flash03MemRule(0)
    }
}
impl core::fmt::Debug for Flash03MemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flash03MemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flash03MemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flash03MemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3()
        )
    }
}
#[doc = "FLEXSPI0 Region 0 Memory Rule"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi0Region0MemRule(pub u32);
impl Flexspi0Region0MemRule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Flexspi0Region0MemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flexspi0Region0MemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Flexspi0Region0MemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Flexspi0Region0MemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Flexspi0Region0MemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Flexspi0Region0MemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Flexspi0Region0MemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Flexspi0Region0MemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Flexspi0Region0MemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Flexspi0Region0MemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Flexspi0Region0MemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Flexspi0Region0MemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::Flexspi0Region0MemRuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Flexspi0Region0MemRuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::Flexspi0Region0MemRuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::Flexspi0Region0MemRuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Flexspi0Region0MemRuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::Flexspi0Region0MemRuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::Flexspi0Region0MemRuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Flexspi0Region0MemRuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::Flexspi0Region0MemRuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::Flexspi0Region0MemRuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Flexspi0Region0MemRuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::Flexspi0Region0MemRuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Flexspi0Region0MemRule {
    #[inline(always)]
    fn default() -> Flexspi0Region0MemRule {
        Flexspi0Region0MemRule(0)
    }
}
impl core::fmt::Debug for Flexspi0Region0MemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi0Region0MemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi0Region0MemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexspi0Region0MemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "FLEXSPI0 Region index Memory Rule"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi0RegionMemRule(pub u32);
impl Flexspi0RegionMemRule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::Flexspi0RegionMemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Flexspi0RegionMemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::Flexspi0RegionMemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::Flexspi0RegionMemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Flexspi0RegionMemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::Flexspi0RegionMemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::Flexspi0RegionMemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Flexspi0RegionMemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::Flexspi0RegionMemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::Flexspi0RegionMemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Flexspi0RegionMemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::Flexspi0RegionMemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for Flexspi0RegionMemRule {
    #[inline(always)]
    fn default() -> Flexspi0RegionMemRule {
        Flexspi0RegionMemRule(0)
    }
}
impl core::fmt::Debug for Flexspi0RegionMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspi0RegionMemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspi0RegionMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexspi0RegionMemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3()
        )
    }
}
#[doc = "Master Secure Level"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MasterSecAntiPolReg(pub u32);
impl MasterSecAntiPolReg {
    #[doc = "SMARTDMA Data"]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma(&self) -> super::vals::MasterSecAntiPolRegSmartdma {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::MasterSecAntiPolRegSmartdma::from_bits(val as u8)
    }
    #[doc = "SMARTDMA Data"]
    #[inline(always)]
    pub const fn set_smartdma(&mut self, val: super::vals::MasterSecAntiPolRegSmartdma) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "eDMA0"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> super::vals::MasterSecAntiPolRegDma0 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::MasterSecAntiPolRegDma0::from_bits(val as u8)
    }
    #[doc = "eDMA0"]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: super::vals::MasterSecAntiPolRegDma0) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "eDMA1"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1(&self) -> super::vals::MasterSecAntiPolRegDma1 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::MasterSecAntiPolRegDma1::from_bits(val as u8)
    }
    #[doc = "eDMA1"]
    #[inline(always)]
    pub const fn set_dma1(&mut self, val: super::vals::MasterSecAntiPolRegDma1) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "PKC"]
    #[must_use]
    #[inline(always)]
    pub const fn pkc(&self) -> super::vals::MasterSecAntiPolRegPkc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::MasterSecAntiPolRegPkc::from_bits(val as u8)
    }
    #[doc = "PKC"]
    #[inline(always)]
    pub const fn set_pkc(&mut self, val: super::vals::MasterSecAntiPolRegPkc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "ENET0"]
    #[must_use]
    #[inline(always)]
    pub const fn enet0(&self) -> super::vals::MasterSecAntiPolRegEnet0 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::MasterSecAntiPolRegEnet0::from_bits(val as u8)
    }
    #[doc = "ENET0"]
    #[inline(always)]
    pub const fn set_enet0(&mut self, val: super::vals::MasterSecAntiPolRegEnet0) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "USB1"]
    #[must_use]
    #[inline(always)]
    pub const fn usb1(&self) -> super::vals::MasterSecAntiPolRegUsb1 {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::MasterSecAntiPolRegUsb1::from_bits(val as u8)
    }
    #[doc = "USB1"]
    #[inline(always)]
    pub const fn set_usb1(&mut self, val: super::vals::MasterSecAntiPolRegUsb1) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
}
impl Default for MasterSecAntiPolReg {
    #[inline(always)]
    fn default() -> MasterSecAntiPolReg {
        MasterSecAntiPolReg(0)
    }
}
impl core::fmt::Debug for MasterSecAntiPolReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MasterSecAntiPolReg")
            .field("smartdma", &self.smartdma())
            .field("dma0", &self.dma0())
            .field("dma1", &self.dma1())
            .field("pkc", &self.pkc())
            .field("enet0", &self.enet0())
            .field("usb1", &self.usb1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MasterSecAntiPolReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MasterSecAntiPolReg {{ smartdma: {:?}, dma0: {:?}, dma1: {:?}, pkc: {:?}, enet0: {:?}, usb1: {:?} }}",
            self.smartdma(),
            self.dma0(),
            self.dma1(),
            self.pkc(),
            self.enet0(),
            self.usb1()
        )
    }
}
#[doc = "Master Secure Level"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MasterSecLevel(pub u32);
impl MasterSecLevel {
    #[doc = "SMARTDMA Data"]
    #[must_use]
    #[inline(always)]
    pub const fn smartdma(&self) -> super::vals::MasterSecLevelSmartdma {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::MasterSecLevelSmartdma::from_bits(val as u8)
    }
    #[doc = "SMARTDMA Data"]
    #[inline(always)]
    pub const fn set_smartdma(&mut self, val: super::vals::MasterSecLevelSmartdma) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "eDMA0"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> super::vals::MasterSecLevelDma0 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::MasterSecLevelDma0::from_bits(val as u8)
    }
    #[doc = "eDMA0"]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: super::vals::MasterSecLevelDma0) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "eDMA1"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1(&self) -> super::vals::MasterSecLevelDma1 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::MasterSecLevelDma1::from_bits(val as u8)
    }
    #[doc = "eDMA1"]
    #[inline(always)]
    pub const fn set_dma1(&mut self, val: super::vals::MasterSecLevelDma1) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "PKC"]
    #[must_use]
    #[inline(always)]
    pub const fn pkc(&self) -> super::vals::MasterSecLevelPkc {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::MasterSecLevelPkc::from_bits(val as u8)
    }
    #[doc = "PKC"]
    #[inline(always)]
    pub const fn set_pkc(&mut self, val: super::vals::MasterSecLevelPkc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "ENET0"]
    #[must_use]
    #[inline(always)]
    pub const fn enet0(&self) -> super::vals::MasterSecLevelEnet0 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::MasterSecLevelEnet0::from_bits(val as u8)
    }
    #[doc = "ENET0"]
    #[inline(always)]
    pub const fn set_enet0(&mut self, val: super::vals::MasterSecLevelEnet0) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "USB1"]
    #[must_use]
    #[inline(always)]
    pub const fn usb1(&self) -> super::vals::MasterSecLevelUsb1 {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::MasterSecLevelUsb1::from_bits(val as u8)
    }
    #[doc = "USB1"]
    #[inline(always)]
    pub const fn set_usb1(&mut self, val: super::vals::MasterSecLevelUsb1) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
}
impl Default for MasterSecLevel {
    #[inline(always)]
    fn default() -> MasterSecLevel {
        MasterSecLevel(0)
    }
}
impl core::fmt::Debug for MasterSecLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MasterSecLevel")
            .field("smartdma", &self.smartdma())
            .field("dma0", &self.dma0())
            .field("dma1", &self.dma1())
            .field("pkc", &self.pkc())
            .field("enet0", &self.enet0())
            .field("usb1", &self.usb1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MasterSecLevel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MasterSecLevel {{ smartdma: {:?}, dma0: {:?}, dma1: {:?}, pkc: {:?}, enet0: {:?}, usb1: {:?} }}",
            self.smartdma(),
            self.dma0(),
            self.dma1(),
            self.pkc(),
            self.enet0(),
            self.usb1()
        )
    }
}
#[doc = "Secure Control Duplicate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiscCtrlDpReg(pub u32);
impl MiscCtrlDpReg {
    #[doc = "Write Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn write_lock(&self) -> super::vals::MiscCtrlDpRegWriteLock {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MiscCtrlDpRegWriteLock::from_bits(val as u8)
    }
    #[doc = "Write Lock"]
    #[inline(always)]
    pub const fn set_write_lock(&mut self, val: super::vals::MiscCtrlDpRegWriteLock) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Enable Secure Checking"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_secure_checking(&self) -> super::vals::MiscCtrlDpRegEnableSecureChecking {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::MiscCtrlDpRegEnableSecureChecking::from_bits(val as u8)
    }
    #[doc = "Enable Secure Checking"]
    #[inline(always)]
    pub const fn set_enable_secure_checking(
        &mut self,
        val: super::vals::MiscCtrlDpRegEnableSecureChecking,
    ) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable Secure Privilege Checking"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_s_priv_check(&self) -> super::vals::MiscCtrlDpRegEnableSPrivCheck {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::MiscCtrlDpRegEnableSPrivCheck::from_bits(val as u8)
    }
    #[doc = "Enable Secure Privilege Checking"]
    #[inline(always)]
    pub const fn set_enable_s_priv_check(
        &mut self,
        val: super::vals::MiscCtrlDpRegEnableSPrivCheck,
    ) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Enable Non-Secure Privilege Checking"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ns_priv_check(&self) -> super::vals::MiscCtrlDpRegEnableNsPrivCheck {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::MiscCtrlDpRegEnableNsPrivCheck::from_bits(val as u8)
    }
    #[doc = "Enable Non-Secure Privilege Checking"]
    #[inline(always)]
    pub const fn set_enable_ns_priv_check(
        &mut self,
        val: super::vals::MiscCtrlDpRegEnableNsPrivCheck,
    ) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Disable Violation Abort"]
    #[must_use]
    #[inline(always)]
    pub const fn disable_violation_abort(&self) -> super::vals::MiscCtrlDpRegDisableViolationAbort {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::MiscCtrlDpRegDisableViolationAbort::from_bits(val as u8)
    }
    #[doc = "Disable Violation Abort"]
    #[inline(always)]
    pub const fn set_disable_violation_abort(
        &mut self,
        val: super::vals::MiscCtrlDpRegDisableViolationAbort,
    ) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Disable Strict Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn disable_strict_mode(&self) -> super::vals::MiscCtrlDpRegDisableStrictMode {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::MiscCtrlDpRegDisableStrictMode::from_bits(val as u8)
    }
    #[doc = "Disable Strict Mode"]
    #[inline(always)]
    pub const fn set_disable_strict_mode(
        &mut self,
        val: super::vals::MiscCtrlDpRegDisableStrictMode,
    ) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "IDAU All Non-Secure"]
    #[must_use]
    #[inline(always)]
    pub const fn idau_all_ns(&self) -> super::vals::MiscCtrlDpRegIdauAllNs {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::MiscCtrlDpRegIdauAllNs::from_bits(val as u8)
    }
    #[doc = "IDAU All Non-Secure"]
    #[inline(always)]
    pub const fn set_idau_all_ns(&mut self, val: super::vals::MiscCtrlDpRegIdauAllNs) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
}
impl Default for MiscCtrlDpReg {
    #[inline(always)]
    fn default() -> MiscCtrlDpReg {
        MiscCtrlDpReg(0)
    }
}
impl core::fmt::Debug for MiscCtrlDpReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MiscCtrlDpReg")
            .field("write_lock", &self.write_lock())
            .field("enable_secure_checking", &self.enable_secure_checking())
            .field("enable_s_priv_check", &self.enable_s_priv_check())
            .field("enable_ns_priv_check", &self.enable_ns_priv_check())
            .field("disable_violation_abort", &self.disable_violation_abort())
            .field("disable_strict_mode", &self.disable_strict_mode())
            .field("idau_all_ns", &self.idau_all_ns())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MiscCtrlDpReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MiscCtrlDpReg {{ write_lock: {:?}, enable_secure_checking: {:?}, enable_s_priv_check: {:?}, enable_ns_priv_check: {:?}, disable_violation_abort: {:?}, disable_strict_mode: {:?}, idau_all_ns: {:?} }}",
            self.write_lock(),
            self.enable_secure_checking(),
            self.enable_s_priv_check(),
            self.enable_ns_priv_check(),
            self.disable_violation_abort(),
            self.disable_strict_mode(),
            self.idau_all_ns()
        )
    }
}
#[doc = "Secure Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiscCtrlReg(pub u32);
impl MiscCtrlReg {
    #[doc = "Write Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn write_lock(&self) -> super::vals::MiscCtrlRegWriteLock {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MiscCtrlRegWriteLock::from_bits(val as u8)
    }
    #[doc = "Write Lock"]
    #[inline(always)]
    pub const fn set_write_lock(&mut self, val: super::vals::MiscCtrlRegWriteLock) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Enable Secure Checking"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_secure_checking(&self) -> super::vals::MiscCtrlRegEnableSecureChecking {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::MiscCtrlRegEnableSecureChecking::from_bits(val as u8)
    }
    #[doc = "Enable Secure Checking"]
    #[inline(always)]
    pub const fn set_enable_secure_checking(
        &mut self,
        val: super::vals::MiscCtrlRegEnableSecureChecking,
    ) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable Secure Privilege Checking"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_s_priv_check(&self) -> super::vals::MiscCtrlRegEnableSPrivCheck {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::MiscCtrlRegEnableSPrivCheck::from_bits(val as u8)
    }
    #[doc = "Enable Secure Privilege Checking"]
    #[inline(always)]
    pub const fn set_enable_s_priv_check(&mut self, val: super::vals::MiscCtrlRegEnableSPrivCheck) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Enable Non-Secure Privilege Checking"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ns_priv_check(&self) -> super::vals::MiscCtrlRegEnableNsPrivCheck {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::MiscCtrlRegEnableNsPrivCheck::from_bits(val as u8)
    }
    #[doc = "Enable Non-Secure Privilege Checking"]
    #[inline(always)]
    pub const fn set_enable_ns_priv_check(
        &mut self,
        val: super::vals::MiscCtrlRegEnableNsPrivCheck,
    ) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Disable Violation Abort"]
    #[must_use]
    #[inline(always)]
    pub const fn disable_violation_abort(&self) -> super::vals::MiscCtrlRegDisableViolationAbort {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::MiscCtrlRegDisableViolationAbort::from_bits(val as u8)
    }
    #[doc = "Disable Violation Abort"]
    #[inline(always)]
    pub const fn set_disable_violation_abort(
        &mut self,
        val: super::vals::MiscCtrlRegDisableViolationAbort,
    ) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Disable Strict Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn disable_strict_mode(&self) -> super::vals::MiscCtrlRegDisableStrictMode {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::MiscCtrlRegDisableStrictMode::from_bits(val as u8)
    }
    #[doc = "Disable Strict Mode"]
    #[inline(always)]
    pub const fn set_disable_strict_mode(
        &mut self,
        val: super::vals::MiscCtrlRegDisableStrictMode,
    ) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "IDAU All Non-Secure"]
    #[must_use]
    #[inline(always)]
    pub const fn idau_all_ns(&self) -> super::vals::MiscCtrlRegIdauAllNs {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::MiscCtrlRegIdauAllNs::from_bits(val as u8)
    }
    #[doc = "IDAU All Non-Secure"]
    #[inline(always)]
    pub const fn set_idau_all_ns(&mut self, val: super::vals::MiscCtrlRegIdauAllNs) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
}
impl Default for MiscCtrlReg {
    #[inline(always)]
    fn default() -> MiscCtrlReg {
        MiscCtrlReg(0)
    }
}
impl core::fmt::Debug for MiscCtrlReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MiscCtrlReg")
            .field("write_lock", &self.write_lock())
            .field("enable_secure_checking", &self.enable_secure_checking())
            .field("enable_s_priv_check", &self.enable_s_priv_check())
            .field("enable_ns_priv_check", &self.enable_ns_priv_check())
            .field("disable_violation_abort", &self.disable_violation_abort())
            .field("disable_strict_mode", &self.disable_strict_mode())
            .field("idau_all_ns", &self.idau_all_ns())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MiscCtrlReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MiscCtrlReg {{ write_lock: {:?}, enable_secure_checking: {:?}, enable_s_priv_check: {:?}, enable_ns_priv_check: {:?}, disable_violation_abort: {:?}, disable_strict_mode: {:?}, idau_all_ns: {:?} }}",
            self.write_lock(),
            self.enable_secure_checking(),
            self.enable_s_priv_check(),
            self.enable_ns_priv_check(),
            self.disable_violation_abort(),
            self.disable_strict_mode(),
            self.idau_all_ns()
        )
    }
}
#[doc = "RAMA Memory Rule 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamaMemRule(pub u32);
impl RamaMemRule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::RamaMemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::RamaMemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::RamaMemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::RamaMemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::RamaMemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::RamaMemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::RamaMemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::RamaMemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::RamaMemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::RamaMemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::RamaMemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::RamaMemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::RamaMemRuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::RamaMemRuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::RamaMemRuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::RamaMemRuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::RamaMemRuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::RamaMemRuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::RamaMemRuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::RamaMemRuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::RamaMemRuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::RamaMemRuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::RamaMemRuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::RamaMemRuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for RamaMemRule {
    #[inline(always)]
    fn default() -> RamaMemRule {
        RamaMemRule(0)
    }
}
impl core::fmt::Debug for RamaMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamaMemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamaMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RamaMemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "RAMB Memory Rule 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RambMemRule(pub u32);
impl RambMemRule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::RambMemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::RambMemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::RambMemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::RambMemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::RambMemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::RambMemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::RambMemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::RambMemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::RambMemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::RambMemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::RambMemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::RambMemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::RambMemRuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::RambMemRuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::RambMemRuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::RambMemRuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::RambMemRuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::RambMemRuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::RambMemRuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::RambMemRuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::RambMemRuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::RambMemRuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::RambMemRuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::RambMemRuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for RambMemRule {
    #[inline(always)]
    fn default() -> RambMemRule {
        RambMemRule(0)
    }
}
impl core::fmt::Debug for RambMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RambMemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RambMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RambMemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "RAMX Memory Rule"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamxMemRule(pub u32);
impl RamxMemRule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::RamxMemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::RamxMemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::RamxMemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::RamxMemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::RamxMemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::RamxMemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::RamxMemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::RamxMemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::RamxMemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::RamxMemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::RamxMemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::RamxMemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::RamxMemRuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::RamxMemRuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::RamxMemRuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::RamxMemRuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::RamxMemRuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::RamxMemRuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::RamxMemRuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::RamxMemRuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::RamxMemRuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::RamxMemRuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::RamxMemRuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::RamxMemRuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for RamxMemRule {
    #[inline(always)]
    fn default() -> RamxMemRule {
        RamxMemRule(0)
    }
}
impl core::fmt::Debug for RamxMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamxMemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamxMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RamxMemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "ROM Memory Rule"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RomMemRule(pub u32);
impl RomMemRule {
    #[doc = "Rule 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rule0(&self) -> super::vals::RomMemRuleRule0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::RomMemRuleRule0::from_bits(val as u8)
    }
    #[doc = "Rule 0"]
    #[inline(always)]
    pub const fn set_rule0(&mut self, val: super::vals::RomMemRuleRule0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Rule 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rule1(&self) -> super::vals::RomMemRuleRule1 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::RomMemRuleRule1::from_bits(val as u8)
    }
    #[doc = "Rule 1"]
    #[inline(always)]
    pub const fn set_rule1(&mut self, val: super::vals::RomMemRuleRule1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Rule 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rule2(&self) -> super::vals::RomMemRuleRule2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::RomMemRuleRule2::from_bits(val as u8)
    }
    #[doc = "Rule 2"]
    #[inline(always)]
    pub const fn set_rule2(&mut self, val: super::vals::RomMemRuleRule2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Rule 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rule3(&self) -> super::vals::RomMemRuleRule3 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::RomMemRuleRule3::from_bits(val as u8)
    }
    #[doc = "Rule 3"]
    #[inline(always)]
    pub const fn set_rule3(&mut self, val: super::vals::RomMemRuleRule3) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Rule 4"]
    #[must_use]
    #[inline(always)]
    pub const fn rule4(&self) -> super::vals::RomMemRuleRule4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::RomMemRuleRule4::from_bits(val as u8)
    }
    #[doc = "Rule 4"]
    #[inline(always)]
    pub const fn set_rule4(&mut self, val: super::vals::RomMemRuleRule4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Rule 5"]
    #[must_use]
    #[inline(always)]
    pub const fn rule5(&self) -> super::vals::RomMemRuleRule5 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::RomMemRuleRule5::from_bits(val as u8)
    }
    #[doc = "Rule 5"]
    #[inline(always)]
    pub const fn set_rule5(&mut self, val: super::vals::RomMemRuleRule5) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Rule 6"]
    #[must_use]
    #[inline(always)]
    pub const fn rule6(&self) -> super::vals::RomMemRuleRule6 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::RomMemRuleRule6::from_bits(val as u8)
    }
    #[doc = "Rule 6"]
    #[inline(always)]
    pub const fn set_rule6(&mut self, val: super::vals::RomMemRuleRule6) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Rule 7"]
    #[must_use]
    #[inline(always)]
    pub const fn rule7(&self) -> super::vals::RomMemRuleRule7 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::RomMemRuleRule7::from_bits(val as u8)
    }
    #[doc = "Rule 7"]
    #[inline(always)]
    pub const fn set_rule7(&mut self, val: super::vals::RomMemRuleRule7) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for RomMemRule {
    #[inline(always)]
    fn default() -> RomMemRule {
        RomMemRule(0)
    }
}
impl core::fmt::Debug for RomMemRule {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RomMemRule")
            .field("rule0", &self.rule0())
            .field("rule1", &self.rule1())
            .field("rule2", &self.rule2())
            .field("rule3", &self.rule3())
            .field("rule4", &self.rule4())
            .field("rule5", &self.rule5())
            .field("rule6", &self.rule6())
            .field("rule7", &self.rule7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RomMemRule {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RomMemRule {{ rule0: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?}, rule6: {:?}, rule7: {:?} }}",
            self.rule0(),
            self.rule1(),
            self.rule2(),
            self.rule3(),
            self.rule4(),
            self.rule5(),
            self.rule6(),
            self.rule7()
        )
    }
}
#[doc = "Secure general purpose registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpReg0(pub u32);
impl SecGpReg0 {
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpReg0 {
    #[inline(always)]
    fn default() -> SecGpReg0 {
        SecGpReg0(0)
    }
}
impl core::fmt::Debug for SecGpReg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpReg0")
            .field("dma0_ipd_req_0", &self.dma0_ipd_req_0())
            .field("dma0_ipd_req_1", &self.dma0_ipd_req_1())
            .field("dma0_ipd_req_2", &self.dma0_ipd_req_2())
            .field("dma0_ipd_req_3", &self.dma0_ipd_req_3())
            .field("dma0_ipd_req_4", &self.dma0_ipd_req_4())
            .field("dma0_ipd_req_5", &self.dma0_ipd_req_5())
            .field("dma0_ipd_req_6", &self.dma0_ipd_req_6())
            .field("dma0_ipd_req_7", &self.dma0_ipd_req_7())
            .field("dma0_ipd_req_8", &self.dma0_ipd_req_8())
            .field("dma0_ipd_req_9", &self.dma0_ipd_req_9())
            .field("dma0_ipd_req_10", &self.dma0_ipd_req_10())
            .field("dma0_ipd_req_11", &self.dma0_ipd_req_11())
            .field("dma0_ipd_req_12", &self.dma0_ipd_req_12())
            .field("dma0_ipd_req_13", &self.dma0_ipd_req_13())
            .field("dma0_ipd_req_14", &self.dma0_ipd_req_14())
            .field("dma0_ipd_req_15", &self.dma0_ipd_req_15())
            .field("dma0_ipd_req_16", &self.dma0_ipd_req_16())
            .field("dma0_ipd_req_17", &self.dma0_ipd_req_17())
            .field("dma0_ipd_req_18", &self.dma0_ipd_req_18())
            .field("dma0_ipd_req_19", &self.dma0_ipd_req_19())
            .field("dma0_ipd_req_20", &self.dma0_ipd_req_20())
            .field("dma0_ipd_req_21", &self.dma0_ipd_req_21())
            .field("dma0_ipd_req_22", &self.dma0_ipd_req_22())
            .field("dma0_ipd_req_23", &self.dma0_ipd_req_23())
            .field("dma0_ipd_req_24", &self.dma0_ipd_req_24())
            .field("dma0_ipd_req_25", &self.dma0_ipd_req_25())
            .field("dma0_ipd_req_26", &self.dma0_ipd_req_26())
            .field("dma0_ipd_req_27", &self.dma0_ipd_req_27())
            .field("dma0_ipd_req_28", &self.dma0_ipd_req_28())
            .field("dma0_ipd_req_29", &self.dma0_ipd_req_29())
            .field("dma0_ipd_req_30", &self.dma0_ipd_req_30())
            .field("dma0_ipd_req_31", &self.dma0_ipd_req_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpReg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpReg0 {{ dma0_ipd_req_0: {=bool:?}, dma0_ipd_req_1: {=bool:?}, dma0_ipd_req_2: {=bool:?}, dma0_ipd_req_3: {=bool:?}, dma0_ipd_req_4: {=bool:?}, dma0_ipd_req_5: {=bool:?}, dma0_ipd_req_6: {=bool:?}, dma0_ipd_req_7: {=bool:?}, dma0_ipd_req_8: {=bool:?}, dma0_ipd_req_9: {=bool:?}, dma0_ipd_req_10: {=bool:?}, dma0_ipd_req_11: {=bool:?}, dma0_ipd_req_12: {=bool:?}, dma0_ipd_req_13: {=bool:?}, dma0_ipd_req_14: {=bool:?}, dma0_ipd_req_15: {=bool:?}, dma0_ipd_req_16: {=bool:?}, dma0_ipd_req_17: {=bool:?}, dma0_ipd_req_18: {=bool:?}, dma0_ipd_req_19: {=bool:?}, dma0_ipd_req_20: {=bool:?}, dma0_ipd_req_21: {=bool:?}, dma0_ipd_req_22: {=bool:?}, dma0_ipd_req_23: {=bool:?}, dma0_ipd_req_24: {=bool:?}, dma0_ipd_req_25: {=bool:?}, dma0_ipd_req_26: {=bool:?}, dma0_ipd_req_27: {=bool:?}, dma0_ipd_req_28: {=bool:?}, dma0_ipd_req_29: {=bool:?}, dma0_ipd_req_30: {=bool:?}, dma0_ipd_req_31: {=bool:?} }}",
            self.dma0_ipd_req_0(),
            self.dma0_ipd_req_1(),
            self.dma0_ipd_req_2(),
            self.dma0_ipd_req_3(),
            self.dma0_ipd_req_4(),
            self.dma0_ipd_req_5(),
            self.dma0_ipd_req_6(),
            self.dma0_ipd_req_7(),
            self.dma0_ipd_req_8(),
            self.dma0_ipd_req_9(),
            self.dma0_ipd_req_10(),
            self.dma0_ipd_req_11(),
            self.dma0_ipd_req_12(),
            self.dma0_ipd_req_13(),
            self.dma0_ipd_req_14(),
            self.dma0_ipd_req_15(),
            self.dma0_ipd_req_16(),
            self.dma0_ipd_req_17(),
            self.dma0_ipd_req_18(),
            self.dma0_ipd_req_19(),
            self.dma0_ipd_req_20(),
            self.dma0_ipd_req_21(),
            self.dma0_ipd_req_22(),
            self.dma0_ipd_req_23(),
            self.dma0_ipd_req_24(),
            self.dma0_ipd_req_25(),
            self.dma0_ipd_req_26(),
            self.dma0_ipd_req_27(),
            self.dma0_ipd_req_28(),
            self.dma0_ipd_req_29(),
            self.dma0_ipd_req_30(),
            self.dma0_ipd_req_31()
        )
    }
}
#[doc = "Secure general purpose registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpReg1(pub u32);
impl SecGpReg1 {
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpReg1 {
    #[inline(always)]
    fn default() -> SecGpReg1 {
        SecGpReg1(0)
    }
}
impl core::fmt::Debug for SecGpReg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpReg1")
            .field("dma0_ipd_req_0", &self.dma0_ipd_req_0())
            .field("dma0_ipd_req_1", &self.dma0_ipd_req_1())
            .field("dma0_ipd_req_2", &self.dma0_ipd_req_2())
            .field("dma0_ipd_req_3", &self.dma0_ipd_req_3())
            .field("dma0_ipd_req_4", &self.dma0_ipd_req_4())
            .field("dma0_ipd_req_5", &self.dma0_ipd_req_5())
            .field("dma0_ipd_req_6", &self.dma0_ipd_req_6())
            .field("dma0_ipd_req_7", &self.dma0_ipd_req_7())
            .field("dma0_ipd_req_8", &self.dma0_ipd_req_8())
            .field("dma0_ipd_req_9", &self.dma0_ipd_req_9())
            .field("dma0_ipd_req_10", &self.dma0_ipd_req_10())
            .field("dma0_ipd_req_11", &self.dma0_ipd_req_11())
            .field("dma0_ipd_req_12", &self.dma0_ipd_req_12())
            .field("dma0_ipd_req_13", &self.dma0_ipd_req_13())
            .field("dma0_ipd_req_14", &self.dma0_ipd_req_14())
            .field("dma0_ipd_req_15", &self.dma0_ipd_req_15())
            .field("dma0_ipd_req_16", &self.dma0_ipd_req_16())
            .field("dma0_ipd_req_17", &self.dma0_ipd_req_17())
            .field("dma0_ipd_req_18", &self.dma0_ipd_req_18())
            .field("dma0_ipd_req_19", &self.dma0_ipd_req_19())
            .field("dma0_ipd_req_20", &self.dma0_ipd_req_20())
            .field("dma0_ipd_req_21", &self.dma0_ipd_req_21())
            .field("dma0_ipd_req_22", &self.dma0_ipd_req_22())
            .field("dma0_ipd_req_23", &self.dma0_ipd_req_23())
            .field("dma0_ipd_req_24", &self.dma0_ipd_req_24())
            .field("dma0_ipd_req_25", &self.dma0_ipd_req_25())
            .field("dma0_ipd_req_26", &self.dma0_ipd_req_26())
            .field("dma0_ipd_req_27", &self.dma0_ipd_req_27())
            .field("dma0_ipd_req_28", &self.dma0_ipd_req_28())
            .field("dma0_ipd_req_29", &self.dma0_ipd_req_29())
            .field("dma0_ipd_req_30", &self.dma0_ipd_req_30())
            .field("dma0_ipd_req_31", &self.dma0_ipd_req_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpReg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpReg1 {{ dma0_ipd_req_0: {=bool:?}, dma0_ipd_req_1: {=bool:?}, dma0_ipd_req_2: {=bool:?}, dma0_ipd_req_3: {=bool:?}, dma0_ipd_req_4: {=bool:?}, dma0_ipd_req_5: {=bool:?}, dma0_ipd_req_6: {=bool:?}, dma0_ipd_req_7: {=bool:?}, dma0_ipd_req_8: {=bool:?}, dma0_ipd_req_9: {=bool:?}, dma0_ipd_req_10: {=bool:?}, dma0_ipd_req_11: {=bool:?}, dma0_ipd_req_12: {=bool:?}, dma0_ipd_req_13: {=bool:?}, dma0_ipd_req_14: {=bool:?}, dma0_ipd_req_15: {=bool:?}, dma0_ipd_req_16: {=bool:?}, dma0_ipd_req_17: {=bool:?}, dma0_ipd_req_18: {=bool:?}, dma0_ipd_req_19: {=bool:?}, dma0_ipd_req_20: {=bool:?}, dma0_ipd_req_21: {=bool:?}, dma0_ipd_req_22: {=bool:?}, dma0_ipd_req_23: {=bool:?}, dma0_ipd_req_24: {=bool:?}, dma0_ipd_req_25: {=bool:?}, dma0_ipd_req_26: {=bool:?}, dma0_ipd_req_27: {=bool:?}, dma0_ipd_req_28: {=bool:?}, dma0_ipd_req_29: {=bool:?}, dma0_ipd_req_30: {=bool:?}, dma0_ipd_req_31: {=bool:?} }}",
            self.dma0_ipd_req_0(),
            self.dma0_ipd_req_1(),
            self.dma0_ipd_req_2(),
            self.dma0_ipd_req_3(),
            self.dma0_ipd_req_4(),
            self.dma0_ipd_req_5(),
            self.dma0_ipd_req_6(),
            self.dma0_ipd_req_7(),
            self.dma0_ipd_req_8(),
            self.dma0_ipd_req_9(),
            self.dma0_ipd_req_10(),
            self.dma0_ipd_req_11(),
            self.dma0_ipd_req_12(),
            self.dma0_ipd_req_13(),
            self.dma0_ipd_req_14(),
            self.dma0_ipd_req_15(),
            self.dma0_ipd_req_16(),
            self.dma0_ipd_req_17(),
            self.dma0_ipd_req_18(),
            self.dma0_ipd_req_19(),
            self.dma0_ipd_req_20(),
            self.dma0_ipd_req_21(),
            self.dma0_ipd_req_22(),
            self.dma0_ipd_req_23(),
            self.dma0_ipd_req_24(),
            self.dma0_ipd_req_25(),
            self.dma0_ipd_req_26(),
            self.dma0_ipd_req_27(),
            self.dma0_ipd_req_28(),
            self.dma0_ipd_req_29(),
            self.dma0_ipd_req_30(),
            self.dma0_ipd_req_31()
        )
    }
}
#[doc = "Secure general purpose registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpReg2(pub u32);
impl SecGpReg2 {
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpReg2 {
    #[inline(always)]
    fn default() -> SecGpReg2 {
        SecGpReg2(0)
    }
}
impl core::fmt::Debug for SecGpReg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpReg2")
            .field("dma0_ipd_req_0", &self.dma0_ipd_req_0())
            .field("dma0_ipd_req_1", &self.dma0_ipd_req_1())
            .field("dma0_ipd_req_2", &self.dma0_ipd_req_2())
            .field("dma0_ipd_req_3", &self.dma0_ipd_req_3())
            .field("dma0_ipd_req_4", &self.dma0_ipd_req_4())
            .field("dma0_ipd_req_5", &self.dma0_ipd_req_5())
            .field("dma0_ipd_req_6", &self.dma0_ipd_req_6())
            .field("dma0_ipd_req_7", &self.dma0_ipd_req_7())
            .field("dma0_ipd_req_8", &self.dma0_ipd_req_8())
            .field("dma0_ipd_req_9", &self.dma0_ipd_req_9())
            .field("dma0_ipd_req_10", &self.dma0_ipd_req_10())
            .field("dma0_ipd_req_11", &self.dma0_ipd_req_11())
            .field("dma0_ipd_req_12", &self.dma0_ipd_req_12())
            .field("dma0_ipd_req_13", &self.dma0_ipd_req_13())
            .field("dma0_ipd_req_14", &self.dma0_ipd_req_14())
            .field("dma0_ipd_req_15", &self.dma0_ipd_req_15())
            .field("dma0_ipd_req_16", &self.dma0_ipd_req_16())
            .field("dma0_ipd_req_17", &self.dma0_ipd_req_17())
            .field("dma0_ipd_req_18", &self.dma0_ipd_req_18())
            .field("dma0_ipd_req_19", &self.dma0_ipd_req_19())
            .field("dma0_ipd_req_20", &self.dma0_ipd_req_20())
            .field("dma0_ipd_req_21", &self.dma0_ipd_req_21())
            .field("dma0_ipd_req_22", &self.dma0_ipd_req_22())
            .field("dma0_ipd_req_23", &self.dma0_ipd_req_23())
            .field("dma0_ipd_req_24", &self.dma0_ipd_req_24())
            .field("dma0_ipd_req_25", &self.dma0_ipd_req_25())
            .field("dma0_ipd_req_26", &self.dma0_ipd_req_26())
            .field("dma0_ipd_req_27", &self.dma0_ipd_req_27())
            .field("dma0_ipd_req_28", &self.dma0_ipd_req_28())
            .field("dma0_ipd_req_29", &self.dma0_ipd_req_29())
            .field("dma0_ipd_req_30", &self.dma0_ipd_req_30())
            .field("dma0_ipd_req_31", &self.dma0_ipd_req_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpReg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpReg2 {{ dma0_ipd_req_0: {=bool:?}, dma0_ipd_req_1: {=bool:?}, dma0_ipd_req_2: {=bool:?}, dma0_ipd_req_3: {=bool:?}, dma0_ipd_req_4: {=bool:?}, dma0_ipd_req_5: {=bool:?}, dma0_ipd_req_6: {=bool:?}, dma0_ipd_req_7: {=bool:?}, dma0_ipd_req_8: {=bool:?}, dma0_ipd_req_9: {=bool:?}, dma0_ipd_req_10: {=bool:?}, dma0_ipd_req_11: {=bool:?}, dma0_ipd_req_12: {=bool:?}, dma0_ipd_req_13: {=bool:?}, dma0_ipd_req_14: {=bool:?}, dma0_ipd_req_15: {=bool:?}, dma0_ipd_req_16: {=bool:?}, dma0_ipd_req_17: {=bool:?}, dma0_ipd_req_18: {=bool:?}, dma0_ipd_req_19: {=bool:?}, dma0_ipd_req_20: {=bool:?}, dma0_ipd_req_21: {=bool:?}, dma0_ipd_req_22: {=bool:?}, dma0_ipd_req_23: {=bool:?}, dma0_ipd_req_24: {=bool:?}, dma0_ipd_req_25: {=bool:?}, dma0_ipd_req_26: {=bool:?}, dma0_ipd_req_27: {=bool:?}, dma0_ipd_req_28: {=bool:?}, dma0_ipd_req_29: {=bool:?}, dma0_ipd_req_30: {=bool:?}, dma0_ipd_req_31: {=bool:?} }}",
            self.dma0_ipd_req_0(),
            self.dma0_ipd_req_1(),
            self.dma0_ipd_req_2(),
            self.dma0_ipd_req_3(),
            self.dma0_ipd_req_4(),
            self.dma0_ipd_req_5(),
            self.dma0_ipd_req_6(),
            self.dma0_ipd_req_7(),
            self.dma0_ipd_req_8(),
            self.dma0_ipd_req_9(),
            self.dma0_ipd_req_10(),
            self.dma0_ipd_req_11(),
            self.dma0_ipd_req_12(),
            self.dma0_ipd_req_13(),
            self.dma0_ipd_req_14(),
            self.dma0_ipd_req_15(),
            self.dma0_ipd_req_16(),
            self.dma0_ipd_req_17(),
            self.dma0_ipd_req_18(),
            self.dma0_ipd_req_19(),
            self.dma0_ipd_req_20(),
            self.dma0_ipd_req_21(),
            self.dma0_ipd_req_22(),
            self.dma0_ipd_req_23(),
            self.dma0_ipd_req_24(),
            self.dma0_ipd_req_25(),
            self.dma0_ipd_req_26(),
            self.dma0_ipd_req_27(),
            self.dma0_ipd_req_28(),
            self.dma0_ipd_req_29(),
            self.dma0_ipd_req_30(),
            self.dma0_ipd_req_31()
        )
    }
}
#[doc = "Secure general purpose registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpReg3(pub u32);
impl SecGpReg3 {
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpReg3 {
    #[inline(always)]
    fn default() -> SecGpReg3 {
        SecGpReg3(0)
    }
}
impl core::fmt::Debug for SecGpReg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpReg3")
            .field("dma0_ipd_req_0", &self.dma0_ipd_req_0())
            .field("dma0_ipd_req_1", &self.dma0_ipd_req_1())
            .field("dma0_ipd_req_2", &self.dma0_ipd_req_2())
            .field("dma0_ipd_req_3", &self.dma0_ipd_req_3())
            .field("dma0_ipd_req_4", &self.dma0_ipd_req_4())
            .field("dma0_ipd_req_5", &self.dma0_ipd_req_5())
            .field("dma0_ipd_req_6", &self.dma0_ipd_req_6())
            .field("dma0_ipd_req_7", &self.dma0_ipd_req_7())
            .field("dma0_ipd_req_8", &self.dma0_ipd_req_8())
            .field("dma0_ipd_req_9", &self.dma0_ipd_req_9())
            .field("dma0_ipd_req_10", &self.dma0_ipd_req_10())
            .field("dma0_ipd_req_11", &self.dma0_ipd_req_11())
            .field("dma0_ipd_req_12", &self.dma0_ipd_req_12())
            .field("dma0_ipd_req_13", &self.dma0_ipd_req_13())
            .field("dma0_ipd_req_14", &self.dma0_ipd_req_14())
            .field("dma0_ipd_req_15", &self.dma0_ipd_req_15())
            .field("dma0_ipd_req_16", &self.dma0_ipd_req_16())
            .field("dma0_ipd_req_17", &self.dma0_ipd_req_17())
            .field("dma0_ipd_req_18", &self.dma0_ipd_req_18())
            .field("dma0_ipd_req_19", &self.dma0_ipd_req_19())
            .field("dma0_ipd_req_20", &self.dma0_ipd_req_20())
            .field("dma0_ipd_req_21", &self.dma0_ipd_req_21())
            .field("dma0_ipd_req_22", &self.dma0_ipd_req_22())
            .field("dma0_ipd_req_23", &self.dma0_ipd_req_23())
            .field("dma0_ipd_req_24", &self.dma0_ipd_req_24())
            .field("dma0_ipd_req_25", &self.dma0_ipd_req_25())
            .field("dma0_ipd_req_26", &self.dma0_ipd_req_26())
            .field("dma0_ipd_req_27", &self.dma0_ipd_req_27())
            .field("dma0_ipd_req_28", &self.dma0_ipd_req_28())
            .field("dma0_ipd_req_29", &self.dma0_ipd_req_29())
            .field("dma0_ipd_req_30", &self.dma0_ipd_req_30())
            .field("dma0_ipd_req_31", &self.dma0_ipd_req_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpReg3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpReg3 {{ dma0_ipd_req_0: {=bool:?}, dma0_ipd_req_1: {=bool:?}, dma0_ipd_req_2: {=bool:?}, dma0_ipd_req_3: {=bool:?}, dma0_ipd_req_4: {=bool:?}, dma0_ipd_req_5: {=bool:?}, dma0_ipd_req_6: {=bool:?}, dma0_ipd_req_7: {=bool:?}, dma0_ipd_req_8: {=bool:?}, dma0_ipd_req_9: {=bool:?}, dma0_ipd_req_10: {=bool:?}, dma0_ipd_req_11: {=bool:?}, dma0_ipd_req_12: {=bool:?}, dma0_ipd_req_13: {=bool:?}, dma0_ipd_req_14: {=bool:?}, dma0_ipd_req_15: {=bool:?}, dma0_ipd_req_16: {=bool:?}, dma0_ipd_req_17: {=bool:?}, dma0_ipd_req_18: {=bool:?}, dma0_ipd_req_19: {=bool:?}, dma0_ipd_req_20: {=bool:?}, dma0_ipd_req_21: {=bool:?}, dma0_ipd_req_22: {=bool:?}, dma0_ipd_req_23: {=bool:?}, dma0_ipd_req_24: {=bool:?}, dma0_ipd_req_25: {=bool:?}, dma0_ipd_req_26: {=bool:?}, dma0_ipd_req_27: {=bool:?}, dma0_ipd_req_28: {=bool:?}, dma0_ipd_req_29: {=bool:?}, dma0_ipd_req_30: {=bool:?}, dma0_ipd_req_31: {=bool:?} }}",
            self.dma0_ipd_req_0(),
            self.dma0_ipd_req_1(),
            self.dma0_ipd_req_2(),
            self.dma0_ipd_req_3(),
            self.dma0_ipd_req_4(),
            self.dma0_ipd_req_5(),
            self.dma0_ipd_req_6(),
            self.dma0_ipd_req_7(),
            self.dma0_ipd_req_8(),
            self.dma0_ipd_req_9(),
            self.dma0_ipd_req_10(),
            self.dma0_ipd_req_11(),
            self.dma0_ipd_req_12(),
            self.dma0_ipd_req_13(),
            self.dma0_ipd_req_14(),
            self.dma0_ipd_req_15(),
            self.dma0_ipd_req_16(),
            self.dma0_ipd_req_17(),
            self.dma0_ipd_req_18(),
            self.dma0_ipd_req_19(),
            self.dma0_ipd_req_20(),
            self.dma0_ipd_req_21(),
            self.dma0_ipd_req_22(),
            self.dma0_ipd_req_23(),
            self.dma0_ipd_req_24(),
            self.dma0_ipd_req_25(),
            self.dma0_ipd_req_26(),
            self.dma0_ipd_req_27(),
            self.dma0_ipd_req_28(),
            self.dma0_ipd_req_29(),
            self.dma0_ipd_req_30(),
            self.dma0_ipd_req_31()
        )
    }
}
#[doc = "Secure general purpose registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpReg4(pub u32);
impl SecGpReg4 {
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA0 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_ipd_req_31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA0 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma0_ipd_req_31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpReg4 {
    #[inline(always)]
    fn default() -> SecGpReg4 {
        SecGpReg4(0)
    }
}
impl core::fmt::Debug for SecGpReg4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpReg4")
            .field("dma0_ipd_req_0", &self.dma0_ipd_req_0())
            .field("dma0_ipd_req_1", &self.dma0_ipd_req_1())
            .field("dma0_ipd_req_2", &self.dma0_ipd_req_2())
            .field("dma0_ipd_req_3", &self.dma0_ipd_req_3())
            .field("dma0_ipd_req_4", &self.dma0_ipd_req_4())
            .field("dma0_ipd_req_5", &self.dma0_ipd_req_5())
            .field("dma0_ipd_req_6", &self.dma0_ipd_req_6())
            .field("dma0_ipd_req_7", &self.dma0_ipd_req_7())
            .field("dma0_ipd_req_8", &self.dma0_ipd_req_8())
            .field("dma0_ipd_req_9", &self.dma0_ipd_req_9())
            .field("dma0_ipd_req_10", &self.dma0_ipd_req_10())
            .field("dma0_ipd_req_11", &self.dma0_ipd_req_11())
            .field("dma0_ipd_req_12", &self.dma0_ipd_req_12())
            .field("dma0_ipd_req_13", &self.dma0_ipd_req_13())
            .field("dma0_ipd_req_14", &self.dma0_ipd_req_14())
            .field("dma0_ipd_req_15", &self.dma0_ipd_req_15())
            .field("dma0_ipd_req_16", &self.dma0_ipd_req_16())
            .field("dma0_ipd_req_17", &self.dma0_ipd_req_17())
            .field("dma0_ipd_req_18", &self.dma0_ipd_req_18())
            .field("dma0_ipd_req_19", &self.dma0_ipd_req_19())
            .field("dma0_ipd_req_20", &self.dma0_ipd_req_20())
            .field("dma0_ipd_req_21", &self.dma0_ipd_req_21())
            .field("dma0_ipd_req_22", &self.dma0_ipd_req_22())
            .field("dma0_ipd_req_23", &self.dma0_ipd_req_23())
            .field("dma0_ipd_req_24", &self.dma0_ipd_req_24())
            .field("dma0_ipd_req_25", &self.dma0_ipd_req_25())
            .field("dma0_ipd_req_26", &self.dma0_ipd_req_26())
            .field("dma0_ipd_req_27", &self.dma0_ipd_req_27())
            .field("dma0_ipd_req_28", &self.dma0_ipd_req_28())
            .field("dma0_ipd_req_29", &self.dma0_ipd_req_29())
            .field("dma0_ipd_req_30", &self.dma0_ipd_req_30())
            .field("dma0_ipd_req_31", &self.dma0_ipd_req_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpReg4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpReg4 {{ dma0_ipd_req_0: {=bool:?}, dma0_ipd_req_1: {=bool:?}, dma0_ipd_req_2: {=bool:?}, dma0_ipd_req_3: {=bool:?}, dma0_ipd_req_4: {=bool:?}, dma0_ipd_req_5: {=bool:?}, dma0_ipd_req_6: {=bool:?}, dma0_ipd_req_7: {=bool:?}, dma0_ipd_req_8: {=bool:?}, dma0_ipd_req_9: {=bool:?}, dma0_ipd_req_10: {=bool:?}, dma0_ipd_req_11: {=bool:?}, dma0_ipd_req_12: {=bool:?}, dma0_ipd_req_13: {=bool:?}, dma0_ipd_req_14: {=bool:?}, dma0_ipd_req_15: {=bool:?}, dma0_ipd_req_16: {=bool:?}, dma0_ipd_req_17: {=bool:?}, dma0_ipd_req_18: {=bool:?}, dma0_ipd_req_19: {=bool:?}, dma0_ipd_req_20: {=bool:?}, dma0_ipd_req_21: {=bool:?}, dma0_ipd_req_22: {=bool:?}, dma0_ipd_req_23: {=bool:?}, dma0_ipd_req_24: {=bool:?}, dma0_ipd_req_25: {=bool:?}, dma0_ipd_req_26: {=bool:?}, dma0_ipd_req_27: {=bool:?}, dma0_ipd_req_28: {=bool:?}, dma0_ipd_req_29: {=bool:?}, dma0_ipd_req_30: {=bool:?}, dma0_ipd_req_31: {=bool:?} }}",
            self.dma0_ipd_req_0(),
            self.dma0_ipd_req_1(),
            self.dma0_ipd_req_2(),
            self.dma0_ipd_req_3(),
            self.dma0_ipd_req_4(),
            self.dma0_ipd_req_5(),
            self.dma0_ipd_req_6(),
            self.dma0_ipd_req_7(),
            self.dma0_ipd_req_8(),
            self.dma0_ipd_req_9(),
            self.dma0_ipd_req_10(),
            self.dma0_ipd_req_11(),
            self.dma0_ipd_req_12(),
            self.dma0_ipd_req_13(),
            self.dma0_ipd_req_14(),
            self.dma0_ipd_req_15(),
            self.dma0_ipd_req_16(),
            self.dma0_ipd_req_17(),
            self.dma0_ipd_req_18(),
            self.dma0_ipd_req_19(),
            self.dma0_ipd_req_20(),
            self.dma0_ipd_req_21(),
            self.dma0_ipd_req_22(),
            self.dma0_ipd_req_23(),
            self.dma0_ipd_req_24(),
            self.dma0_ipd_req_25(),
            self.dma0_ipd_req_26(),
            self.dma0_ipd_req_27(),
            self.dma0_ipd_req_28(),
            self.dma0_ipd_req_29(),
            self.dma0_ipd_req_30(),
            self.dma0_ipd_req_31()
        )
    }
}
#[doc = "Secure general purpose registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpReg5(pub u32);
impl SecGpReg5 {
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpReg5 {
    #[inline(always)]
    fn default() -> SecGpReg5 {
        SecGpReg5(0)
    }
}
impl core::fmt::Debug for SecGpReg5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpReg5")
            .field("dma1_ipd_req_0", &self.dma1_ipd_req_0())
            .field("dma1_ipd_req_1", &self.dma1_ipd_req_1())
            .field("dma1_ipd_req_2", &self.dma1_ipd_req_2())
            .field("dma1_ipd_req_3", &self.dma1_ipd_req_3())
            .field("dma1_ipd_req_4", &self.dma1_ipd_req_4())
            .field("dma1_ipd_req_5", &self.dma1_ipd_req_5())
            .field("dma1_ipd_req_6", &self.dma1_ipd_req_6())
            .field("dma1_ipd_req_7", &self.dma1_ipd_req_7())
            .field("dma1_ipd_req_8", &self.dma1_ipd_req_8())
            .field("dma1_ipd_req_9", &self.dma1_ipd_req_9())
            .field("dma1_ipd_req_10", &self.dma1_ipd_req_10())
            .field("dma1_ipd_req_11", &self.dma1_ipd_req_11())
            .field("dma1_ipd_req_12", &self.dma1_ipd_req_12())
            .field("dma1_ipd_req_13", &self.dma1_ipd_req_13())
            .field("dma1_ipd_req_14", &self.dma1_ipd_req_14())
            .field("dma1_ipd_req_15", &self.dma1_ipd_req_15())
            .field("dma1_ipd_req_16", &self.dma1_ipd_req_16())
            .field("dma1_ipd_req_17", &self.dma1_ipd_req_17())
            .field("dma1_ipd_req_18", &self.dma1_ipd_req_18())
            .field("dma1_ipd_req_19", &self.dma1_ipd_req_19())
            .field("dma1_ipd_req_20", &self.dma1_ipd_req_20())
            .field("dma1_ipd_req_21", &self.dma1_ipd_req_21())
            .field("dma1_ipd_req_22", &self.dma1_ipd_req_22())
            .field("dma1_ipd_req_23", &self.dma1_ipd_req_23())
            .field("dma1_ipd_req_24", &self.dma1_ipd_req_24())
            .field("dma1_ipd_req_25", &self.dma1_ipd_req_25())
            .field("dma1_ipd_req_26", &self.dma1_ipd_req_26())
            .field("dma1_ipd_req_27", &self.dma1_ipd_req_27())
            .field("dma1_ipd_req_28", &self.dma1_ipd_req_28())
            .field("dma1_ipd_req_29", &self.dma1_ipd_req_29())
            .field("dma1_ipd_req_30", &self.dma1_ipd_req_30())
            .field("dma1_ipd_req_31", &self.dma1_ipd_req_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpReg5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpReg5 {{ dma1_ipd_req_0: {=bool:?}, dma1_ipd_req_1: {=bool:?}, dma1_ipd_req_2: {=bool:?}, dma1_ipd_req_3: {=bool:?}, dma1_ipd_req_4: {=bool:?}, dma1_ipd_req_5: {=bool:?}, dma1_ipd_req_6: {=bool:?}, dma1_ipd_req_7: {=bool:?}, dma1_ipd_req_8: {=bool:?}, dma1_ipd_req_9: {=bool:?}, dma1_ipd_req_10: {=bool:?}, dma1_ipd_req_11: {=bool:?}, dma1_ipd_req_12: {=bool:?}, dma1_ipd_req_13: {=bool:?}, dma1_ipd_req_14: {=bool:?}, dma1_ipd_req_15: {=bool:?}, dma1_ipd_req_16: {=bool:?}, dma1_ipd_req_17: {=bool:?}, dma1_ipd_req_18: {=bool:?}, dma1_ipd_req_19: {=bool:?}, dma1_ipd_req_20: {=bool:?}, dma1_ipd_req_21: {=bool:?}, dma1_ipd_req_22: {=bool:?}, dma1_ipd_req_23: {=bool:?}, dma1_ipd_req_24: {=bool:?}, dma1_ipd_req_25: {=bool:?}, dma1_ipd_req_26: {=bool:?}, dma1_ipd_req_27: {=bool:?}, dma1_ipd_req_28: {=bool:?}, dma1_ipd_req_29: {=bool:?}, dma1_ipd_req_30: {=bool:?}, dma1_ipd_req_31: {=bool:?} }}",
            self.dma1_ipd_req_0(),
            self.dma1_ipd_req_1(),
            self.dma1_ipd_req_2(),
            self.dma1_ipd_req_3(),
            self.dma1_ipd_req_4(),
            self.dma1_ipd_req_5(),
            self.dma1_ipd_req_6(),
            self.dma1_ipd_req_7(),
            self.dma1_ipd_req_8(),
            self.dma1_ipd_req_9(),
            self.dma1_ipd_req_10(),
            self.dma1_ipd_req_11(),
            self.dma1_ipd_req_12(),
            self.dma1_ipd_req_13(),
            self.dma1_ipd_req_14(),
            self.dma1_ipd_req_15(),
            self.dma1_ipd_req_16(),
            self.dma1_ipd_req_17(),
            self.dma1_ipd_req_18(),
            self.dma1_ipd_req_19(),
            self.dma1_ipd_req_20(),
            self.dma1_ipd_req_21(),
            self.dma1_ipd_req_22(),
            self.dma1_ipd_req_23(),
            self.dma1_ipd_req_24(),
            self.dma1_ipd_req_25(),
            self.dma1_ipd_req_26(),
            self.dma1_ipd_req_27(),
            self.dma1_ipd_req_28(),
            self.dma1_ipd_req_29(),
            self.dma1_ipd_req_30(),
            self.dma1_ipd_req_31()
        )
    }
}
#[doc = "Secure general purpose registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpReg6(pub u32);
impl SecGpReg6 {
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpReg6 {
    #[inline(always)]
    fn default() -> SecGpReg6 {
        SecGpReg6(0)
    }
}
impl core::fmt::Debug for SecGpReg6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpReg6")
            .field("dma1_ipd_req_0", &self.dma1_ipd_req_0())
            .field("dma1_ipd_req_1", &self.dma1_ipd_req_1())
            .field("dma1_ipd_req_2", &self.dma1_ipd_req_2())
            .field("dma1_ipd_req_3", &self.dma1_ipd_req_3())
            .field("dma1_ipd_req_4", &self.dma1_ipd_req_4())
            .field("dma1_ipd_req_5", &self.dma1_ipd_req_5())
            .field("dma1_ipd_req_6", &self.dma1_ipd_req_6())
            .field("dma1_ipd_req_7", &self.dma1_ipd_req_7())
            .field("dma1_ipd_req_8", &self.dma1_ipd_req_8())
            .field("dma1_ipd_req_9", &self.dma1_ipd_req_9())
            .field("dma1_ipd_req_10", &self.dma1_ipd_req_10())
            .field("dma1_ipd_req_11", &self.dma1_ipd_req_11())
            .field("dma1_ipd_req_12", &self.dma1_ipd_req_12())
            .field("dma1_ipd_req_13", &self.dma1_ipd_req_13())
            .field("dma1_ipd_req_14", &self.dma1_ipd_req_14())
            .field("dma1_ipd_req_15", &self.dma1_ipd_req_15())
            .field("dma1_ipd_req_16", &self.dma1_ipd_req_16())
            .field("dma1_ipd_req_17", &self.dma1_ipd_req_17())
            .field("dma1_ipd_req_18", &self.dma1_ipd_req_18())
            .field("dma1_ipd_req_19", &self.dma1_ipd_req_19())
            .field("dma1_ipd_req_20", &self.dma1_ipd_req_20())
            .field("dma1_ipd_req_21", &self.dma1_ipd_req_21())
            .field("dma1_ipd_req_22", &self.dma1_ipd_req_22())
            .field("dma1_ipd_req_23", &self.dma1_ipd_req_23())
            .field("dma1_ipd_req_24", &self.dma1_ipd_req_24())
            .field("dma1_ipd_req_25", &self.dma1_ipd_req_25())
            .field("dma1_ipd_req_26", &self.dma1_ipd_req_26())
            .field("dma1_ipd_req_27", &self.dma1_ipd_req_27())
            .field("dma1_ipd_req_28", &self.dma1_ipd_req_28())
            .field("dma1_ipd_req_29", &self.dma1_ipd_req_29())
            .field("dma1_ipd_req_30", &self.dma1_ipd_req_30())
            .field("dma1_ipd_req_31", &self.dma1_ipd_req_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpReg6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpReg6 {{ dma1_ipd_req_0: {=bool:?}, dma1_ipd_req_1: {=bool:?}, dma1_ipd_req_2: {=bool:?}, dma1_ipd_req_3: {=bool:?}, dma1_ipd_req_4: {=bool:?}, dma1_ipd_req_5: {=bool:?}, dma1_ipd_req_6: {=bool:?}, dma1_ipd_req_7: {=bool:?}, dma1_ipd_req_8: {=bool:?}, dma1_ipd_req_9: {=bool:?}, dma1_ipd_req_10: {=bool:?}, dma1_ipd_req_11: {=bool:?}, dma1_ipd_req_12: {=bool:?}, dma1_ipd_req_13: {=bool:?}, dma1_ipd_req_14: {=bool:?}, dma1_ipd_req_15: {=bool:?}, dma1_ipd_req_16: {=bool:?}, dma1_ipd_req_17: {=bool:?}, dma1_ipd_req_18: {=bool:?}, dma1_ipd_req_19: {=bool:?}, dma1_ipd_req_20: {=bool:?}, dma1_ipd_req_21: {=bool:?}, dma1_ipd_req_22: {=bool:?}, dma1_ipd_req_23: {=bool:?}, dma1_ipd_req_24: {=bool:?}, dma1_ipd_req_25: {=bool:?}, dma1_ipd_req_26: {=bool:?}, dma1_ipd_req_27: {=bool:?}, dma1_ipd_req_28: {=bool:?}, dma1_ipd_req_29: {=bool:?}, dma1_ipd_req_30: {=bool:?}, dma1_ipd_req_31: {=bool:?} }}",
            self.dma1_ipd_req_0(),
            self.dma1_ipd_req_1(),
            self.dma1_ipd_req_2(),
            self.dma1_ipd_req_3(),
            self.dma1_ipd_req_4(),
            self.dma1_ipd_req_5(),
            self.dma1_ipd_req_6(),
            self.dma1_ipd_req_7(),
            self.dma1_ipd_req_8(),
            self.dma1_ipd_req_9(),
            self.dma1_ipd_req_10(),
            self.dma1_ipd_req_11(),
            self.dma1_ipd_req_12(),
            self.dma1_ipd_req_13(),
            self.dma1_ipd_req_14(),
            self.dma1_ipd_req_15(),
            self.dma1_ipd_req_16(),
            self.dma1_ipd_req_17(),
            self.dma1_ipd_req_18(),
            self.dma1_ipd_req_19(),
            self.dma1_ipd_req_20(),
            self.dma1_ipd_req_21(),
            self.dma1_ipd_req_22(),
            self.dma1_ipd_req_23(),
            self.dma1_ipd_req_24(),
            self.dma1_ipd_req_25(),
            self.dma1_ipd_req_26(),
            self.dma1_ipd_req_27(),
            self.dma1_ipd_req_28(),
            self.dma1_ipd_req_29(),
            self.dma1_ipd_req_30(),
            self.dma1_ipd_req_31()
        )
    }
}
#[doc = "Secure general purpose registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpReg7(pub u32);
impl SecGpReg7 {
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpReg7 {
    #[inline(always)]
    fn default() -> SecGpReg7 {
        SecGpReg7(0)
    }
}
impl core::fmt::Debug for SecGpReg7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpReg7")
            .field("dma1_ipd_req_0", &self.dma1_ipd_req_0())
            .field("dma1_ipd_req_1", &self.dma1_ipd_req_1())
            .field("dma1_ipd_req_2", &self.dma1_ipd_req_2())
            .field("dma1_ipd_req_3", &self.dma1_ipd_req_3())
            .field("dma1_ipd_req_4", &self.dma1_ipd_req_4())
            .field("dma1_ipd_req_5", &self.dma1_ipd_req_5())
            .field("dma1_ipd_req_6", &self.dma1_ipd_req_6())
            .field("dma1_ipd_req_7", &self.dma1_ipd_req_7())
            .field("dma1_ipd_req_8", &self.dma1_ipd_req_8())
            .field("dma1_ipd_req_9", &self.dma1_ipd_req_9())
            .field("dma1_ipd_req_10", &self.dma1_ipd_req_10())
            .field("dma1_ipd_req_11", &self.dma1_ipd_req_11())
            .field("dma1_ipd_req_12", &self.dma1_ipd_req_12())
            .field("dma1_ipd_req_13", &self.dma1_ipd_req_13())
            .field("dma1_ipd_req_14", &self.dma1_ipd_req_14())
            .field("dma1_ipd_req_15", &self.dma1_ipd_req_15())
            .field("dma1_ipd_req_16", &self.dma1_ipd_req_16())
            .field("dma1_ipd_req_17", &self.dma1_ipd_req_17())
            .field("dma1_ipd_req_18", &self.dma1_ipd_req_18())
            .field("dma1_ipd_req_19", &self.dma1_ipd_req_19())
            .field("dma1_ipd_req_20", &self.dma1_ipd_req_20())
            .field("dma1_ipd_req_21", &self.dma1_ipd_req_21())
            .field("dma1_ipd_req_22", &self.dma1_ipd_req_22())
            .field("dma1_ipd_req_23", &self.dma1_ipd_req_23())
            .field("dma1_ipd_req_24", &self.dma1_ipd_req_24())
            .field("dma1_ipd_req_25", &self.dma1_ipd_req_25())
            .field("dma1_ipd_req_26", &self.dma1_ipd_req_26())
            .field("dma1_ipd_req_27", &self.dma1_ipd_req_27())
            .field("dma1_ipd_req_28", &self.dma1_ipd_req_28())
            .field("dma1_ipd_req_29", &self.dma1_ipd_req_29())
            .field("dma1_ipd_req_30", &self.dma1_ipd_req_30())
            .field("dma1_ipd_req_31", &self.dma1_ipd_req_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpReg7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpReg7 {{ dma1_ipd_req_0: {=bool:?}, dma1_ipd_req_1: {=bool:?}, dma1_ipd_req_2: {=bool:?}, dma1_ipd_req_3: {=bool:?}, dma1_ipd_req_4: {=bool:?}, dma1_ipd_req_5: {=bool:?}, dma1_ipd_req_6: {=bool:?}, dma1_ipd_req_7: {=bool:?}, dma1_ipd_req_8: {=bool:?}, dma1_ipd_req_9: {=bool:?}, dma1_ipd_req_10: {=bool:?}, dma1_ipd_req_11: {=bool:?}, dma1_ipd_req_12: {=bool:?}, dma1_ipd_req_13: {=bool:?}, dma1_ipd_req_14: {=bool:?}, dma1_ipd_req_15: {=bool:?}, dma1_ipd_req_16: {=bool:?}, dma1_ipd_req_17: {=bool:?}, dma1_ipd_req_18: {=bool:?}, dma1_ipd_req_19: {=bool:?}, dma1_ipd_req_20: {=bool:?}, dma1_ipd_req_21: {=bool:?}, dma1_ipd_req_22: {=bool:?}, dma1_ipd_req_23: {=bool:?}, dma1_ipd_req_24: {=bool:?}, dma1_ipd_req_25: {=bool:?}, dma1_ipd_req_26: {=bool:?}, dma1_ipd_req_27: {=bool:?}, dma1_ipd_req_28: {=bool:?}, dma1_ipd_req_29: {=bool:?}, dma1_ipd_req_30: {=bool:?}, dma1_ipd_req_31: {=bool:?} }}",
            self.dma1_ipd_req_0(),
            self.dma1_ipd_req_1(),
            self.dma1_ipd_req_2(),
            self.dma1_ipd_req_3(),
            self.dma1_ipd_req_4(),
            self.dma1_ipd_req_5(),
            self.dma1_ipd_req_6(),
            self.dma1_ipd_req_7(),
            self.dma1_ipd_req_8(),
            self.dma1_ipd_req_9(),
            self.dma1_ipd_req_10(),
            self.dma1_ipd_req_11(),
            self.dma1_ipd_req_12(),
            self.dma1_ipd_req_13(),
            self.dma1_ipd_req_14(),
            self.dma1_ipd_req_15(),
            self.dma1_ipd_req_16(),
            self.dma1_ipd_req_17(),
            self.dma1_ipd_req_18(),
            self.dma1_ipd_req_19(),
            self.dma1_ipd_req_20(),
            self.dma1_ipd_req_21(),
            self.dma1_ipd_req_22(),
            self.dma1_ipd_req_23(),
            self.dma1_ipd_req_24(),
            self.dma1_ipd_req_25(),
            self.dma1_ipd_req_26(),
            self.dma1_ipd_req_27(),
            self.dma1_ipd_req_28(),
            self.dma1_ipd_req_29(),
            self.dma1_ipd_req_30(),
            self.dma1_ipd_req_31()
        )
    }
}
#[doc = "Secure general purpose registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpReg8(pub u32);
impl SecGpReg8 {
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpReg8 {
    #[inline(always)]
    fn default() -> SecGpReg8 {
        SecGpReg8(0)
    }
}
impl core::fmt::Debug for SecGpReg8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpReg8")
            .field("dma1_ipd_req_0", &self.dma1_ipd_req_0())
            .field("dma1_ipd_req_1", &self.dma1_ipd_req_1())
            .field("dma1_ipd_req_2", &self.dma1_ipd_req_2())
            .field("dma1_ipd_req_3", &self.dma1_ipd_req_3())
            .field("dma1_ipd_req_4", &self.dma1_ipd_req_4())
            .field("dma1_ipd_req_5", &self.dma1_ipd_req_5())
            .field("dma1_ipd_req_6", &self.dma1_ipd_req_6())
            .field("dma1_ipd_req_7", &self.dma1_ipd_req_7())
            .field("dma1_ipd_req_8", &self.dma1_ipd_req_8())
            .field("dma1_ipd_req_9", &self.dma1_ipd_req_9())
            .field("dma1_ipd_req_10", &self.dma1_ipd_req_10())
            .field("dma1_ipd_req_11", &self.dma1_ipd_req_11())
            .field("dma1_ipd_req_12", &self.dma1_ipd_req_12())
            .field("dma1_ipd_req_13", &self.dma1_ipd_req_13())
            .field("dma1_ipd_req_14", &self.dma1_ipd_req_14())
            .field("dma1_ipd_req_15", &self.dma1_ipd_req_15())
            .field("dma1_ipd_req_16", &self.dma1_ipd_req_16())
            .field("dma1_ipd_req_17", &self.dma1_ipd_req_17())
            .field("dma1_ipd_req_18", &self.dma1_ipd_req_18())
            .field("dma1_ipd_req_19", &self.dma1_ipd_req_19())
            .field("dma1_ipd_req_20", &self.dma1_ipd_req_20())
            .field("dma1_ipd_req_21", &self.dma1_ipd_req_21())
            .field("dma1_ipd_req_22", &self.dma1_ipd_req_22())
            .field("dma1_ipd_req_23", &self.dma1_ipd_req_23())
            .field("dma1_ipd_req_24", &self.dma1_ipd_req_24())
            .field("dma1_ipd_req_25", &self.dma1_ipd_req_25())
            .field("dma1_ipd_req_26", &self.dma1_ipd_req_26())
            .field("dma1_ipd_req_27", &self.dma1_ipd_req_27())
            .field("dma1_ipd_req_28", &self.dma1_ipd_req_28())
            .field("dma1_ipd_req_29", &self.dma1_ipd_req_29())
            .field("dma1_ipd_req_30", &self.dma1_ipd_req_30())
            .field("dma1_ipd_req_31", &self.dma1_ipd_req_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpReg8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpReg8 {{ dma1_ipd_req_0: {=bool:?}, dma1_ipd_req_1: {=bool:?}, dma1_ipd_req_2: {=bool:?}, dma1_ipd_req_3: {=bool:?}, dma1_ipd_req_4: {=bool:?}, dma1_ipd_req_5: {=bool:?}, dma1_ipd_req_6: {=bool:?}, dma1_ipd_req_7: {=bool:?}, dma1_ipd_req_8: {=bool:?}, dma1_ipd_req_9: {=bool:?}, dma1_ipd_req_10: {=bool:?}, dma1_ipd_req_11: {=bool:?}, dma1_ipd_req_12: {=bool:?}, dma1_ipd_req_13: {=bool:?}, dma1_ipd_req_14: {=bool:?}, dma1_ipd_req_15: {=bool:?}, dma1_ipd_req_16: {=bool:?}, dma1_ipd_req_17: {=bool:?}, dma1_ipd_req_18: {=bool:?}, dma1_ipd_req_19: {=bool:?}, dma1_ipd_req_20: {=bool:?}, dma1_ipd_req_21: {=bool:?}, dma1_ipd_req_22: {=bool:?}, dma1_ipd_req_23: {=bool:?}, dma1_ipd_req_24: {=bool:?}, dma1_ipd_req_25: {=bool:?}, dma1_ipd_req_26: {=bool:?}, dma1_ipd_req_27: {=bool:?}, dma1_ipd_req_28: {=bool:?}, dma1_ipd_req_29: {=bool:?}, dma1_ipd_req_30: {=bool:?}, dma1_ipd_req_31: {=bool:?} }}",
            self.dma1_ipd_req_0(),
            self.dma1_ipd_req_1(),
            self.dma1_ipd_req_2(),
            self.dma1_ipd_req_3(),
            self.dma1_ipd_req_4(),
            self.dma1_ipd_req_5(),
            self.dma1_ipd_req_6(),
            self.dma1_ipd_req_7(),
            self.dma1_ipd_req_8(),
            self.dma1_ipd_req_9(),
            self.dma1_ipd_req_10(),
            self.dma1_ipd_req_11(),
            self.dma1_ipd_req_12(),
            self.dma1_ipd_req_13(),
            self.dma1_ipd_req_14(),
            self.dma1_ipd_req_15(),
            self.dma1_ipd_req_16(),
            self.dma1_ipd_req_17(),
            self.dma1_ipd_req_18(),
            self.dma1_ipd_req_19(),
            self.dma1_ipd_req_20(),
            self.dma1_ipd_req_21(),
            self.dma1_ipd_req_22(),
            self.dma1_ipd_req_23(),
            self.dma1_ipd_req_24(),
            self.dma1_ipd_req_25(),
            self.dma1_ipd_req_26(),
            self.dma1_ipd_req_27(),
            self.dma1_ipd_req_28(),
            self.dma1_ipd_req_29(),
            self.dma1_ipd_req_30(),
            self.dma1_ipd_req_31()
        )
    }
}
#[doc = "Secure general purpose registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecGpReg9(pub u32);
impl SecGpReg9 {
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA1 IPD_REQ"]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_ipd_req_31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA1 IPD_REQ"]
    #[inline(always)]
    pub const fn set_dma1_ipd_req_31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SecGpReg9 {
    #[inline(always)]
    fn default() -> SecGpReg9 {
        SecGpReg9(0)
    }
}
impl core::fmt::Debug for SecGpReg9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecGpReg9")
            .field("dma1_ipd_req_0", &self.dma1_ipd_req_0())
            .field("dma1_ipd_req_1", &self.dma1_ipd_req_1())
            .field("dma1_ipd_req_2", &self.dma1_ipd_req_2())
            .field("dma1_ipd_req_3", &self.dma1_ipd_req_3())
            .field("dma1_ipd_req_4", &self.dma1_ipd_req_4())
            .field("dma1_ipd_req_5", &self.dma1_ipd_req_5())
            .field("dma1_ipd_req_6", &self.dma1_ipd_req_6())
            .field("dma1_ipd_req_7", &self.dma1_ipd_req_7())
            .field("dma1_ipd_req_8", &self.dma1_ipd_req_8())
            .field("dma1_ipd_req_9", &self.dma1_ipd_req_9())
            .field("dma1_ipd_req_10", &self.dma1_ipd_req_10())
            .field("dma1_ipd_req_11", &self.dma1_ipd_req_11())
            .field("dma1_ipd_req_12", &self.dma1_ipd_req_12())
            .field("dma1_ipd_req_13", &self.dma1_ipd_req_13())
            .field("dma1_ipd_req_14", &self.dma1_ipd_req_14())
            .field("dma1_ipd_req_15", &self.dma1_ipd_req_15())
            .field("dma1_ipd_req_16", &self.dma1_ipd_req_16())
            .field("dma1_ipd_req_17", &self.dma1_ipd_req_17())
            .field("dma1_ipd_req_18", &self.dma1_ipd_req_18())
            .field("dma1_ipd_req_19", &self.dma1_ipd_req_19())
            .field("dma1_ipd_req_20", &self.dma1_ipd_req_20())
            .field("dma1_ipd_req_21", &self.dma1_ipd_req_21())
            .field("dma1_ipd_req_22", &self.dma1_ipd_req_22())
            .field("dma1_ipd_req_23", &self.dma1_ipd_req_23())
            .field("dma1_ipd_req_24", &self.dma1_ipd_req_24())
            .field("dma1_ipd_req_25", &self.dma1_ipd_req_25())
            .field("dma1_ipd_req_26", &self.dma1_ipd_req_26())
            .field("dma1_ipd_req_27", &self.dma1_ipd_req_27())
            .field("dma1_ipd_req_28", &self.dma1_ipd_req_28())
            .field("dma1_ipd_req_29", &self.dma1_ipd_req_29())
            .field("dma1_ipd_req_30", &self.dma1_ipd_req_30())
            .field("dma1_ipd_req_31", &self.dma1_ipd_req_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecGpReg9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecGpReg9 {{ dma1_ipd_req_0: {=bool:?}, dma1_ipd_req_1: {=bool:?}, dma1_ipd_req_2: {=bool:?}, dma1_ipd_req_3: {=bool:?}, dma1_ipd_req_4: {=bool:?}, dma1_ipd_req_5: {=bool:?}, dma1_ipd_req_6: {=bool:?}, dma1_ipd_req_7: {=bool:?}, dma1_ipd_req_8: {=bool:?}, dma1_ipd_req_9: {=bool:?}, dma1_ipd_req_10: {=bool:?}, dma1_ipd_req_11: {=bool:?}, dma1_ipd_req_12: {=bool:?}, dma1_ipd_req_13: {=bool:?}, dma1_ipd_req_14: {=bool:?}, dma1_ipd_req_15: {=bool:?}, dma1_ipd_req_16: {=bool:?}, dma1_ipd_req_17: {=bool:?}, dma1_ipd_req_18: {=bool:?}, dma1_ipd_req_19: {=bool:?}, dma1_ipd_req_20: {=bool:?}, dma1_ipd_req_21: {=bool:?}, dma1_ipd_req_22: {=bool:?}, dma1_ipd_req_23: {=bool:?}, dma1_ipd_req_24: {=bool:?}, dma1_ipd_req_25: {=bool:?}, dma1_ipd_req_26: {=bool:?}, dma1_ipd_req_27: {=bool:?}, dma1_ipd_req_28: {=bool:?}, dma1_ipd_req_29: {=bool:?}, dma1_ipd_req_30: {=bool:?}, dma1_ipd_req_31: {=bool:?} }}",
            self.dma1_ipd_req_0(),
            self.dma1_ipd_req_1(),
            self.dma1_ipd_req_2(),
            self.dma1_ipd_req_3(),
            self.dma1_ipd_req_4(),
            self.dma1_ipd_req_5(),
            self.dma1_ipd_req_6(),
            self.dma1_ipd_req_7(),
            self.dma1_ipd_req_8(),
            self.dma1_ipd_req_9(),
            self.dma1_ipd_req_10(),
            self.dma1_ipd_req_11(),
            self.dma1_ipd_req_12(),
            self.dma1_ipd_req_13(),
            self.dma1_ipd_req_14(),
            self.dma1_ipd_req_15(),
            self.dma1_ipd_req_16(),
            self.dma1_ipd_req_17(),
            self.dma1_ipd_req_18(),
            self.dma1_ipd_req_19(),
            self.dma1_ipd_req_20(),
            self.dma1_ipd_req_21(),
            self.dma1_ipd_req_22(),
            self.dma1_ipd_req_23(),
            self.dma1_ipd_req_24(),
            self.dma1_ipd_req_25(),
            self.dma1_ipd_req_26(),
            self.dma1_ipd_req_27(),
            self.dma1_ipd_req_28(),
            self.dma1_ipd_req_29(),
            self.dma1_ipd_req_30(),
            self.dma1_ipd_req_31()
        )
    }
}
#[doc = "Security Violation Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecVioAddr(pub u32);
impl SecVioAddr {
    #[doc = "Security violation address for AHB layer a reset value 0"]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Security violation address for AHB layer a reset value 0"]
    #[inline(always)]
    pub const fn set_sec_vio_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SecVioAddr {
    #[inline(always)]
    fn default() -> SecVioAddr {
        SecVioAddr(0)
    }
}
impl core::fmt::Debug for SecVioAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecVioAddr")
            .field("sec_vio_addr", &self.sec_vio_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecVioAddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecVioAddr {{ sec_vio_addr: {=u32:?} }}",
            self.sec_vio_addr()
        )
    }
}
#[doc = "Security Violation Info Validity for Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecVioInfoValid(pub u32);
impl SecVioInfoValid {
    #[doc = "Violation information valid flag for AHB port 0"]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 0"]
    #[inline(always)]
    pub const fn set_vio_info_valid0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Violation information valid flag for AHB port 1"]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 1"]
    #[inline(always)]
    pub const fn set_vio_info_valid1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Violation information valid flag for AHB port 2"]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 2"]
    #[inline(always)]
    pub const fn set_vio_info_valid2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Violation information valid flag for AHB port 3"]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 3"]
    #[inline(always)]
    pub const fn set_vio_info_valid3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Violation information valid flag for AHB port 4"]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 4"]
    #[inline(always)]
    pub const fn set_vio_info_valid4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Violation information valid flag for AHB port 5"]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 5"]
    #[inline(always)]
    pub const fn set_vio_info_valid5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Violation information valid flag for AHB port 6"]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 6"]
    #[inline(always)]
    pub const fn set_vio_info_valid6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Violation information valid flag for AHB port 7"]
    #[must_use]
    #[inline(always)]
    pub const fn vio_info_valid7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Violation information valid flag for AHB port 7"]
    #[inline(always)]
    pub const fn set_vio_info_valid7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for SecVioInfoValid {
    #[inline(always)]
    fn default() -> SecVioInfoValid {
        SecVioInfoValid(0)
    }
}
impl core::fmt::Debug for SecVioInfoValid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecVioInfoValid")
            .field("vio_info_valid0", &self.vio_info_valid0())
            .field("vio_info_valid1", &self.vio_info_valid1())
            .field("vio_info_valid2", &self.vio_info_valid2())
            .field("vio_info_valid3", &self.vio_info_valid3())
            .field("vio_info_valid4", &self.vio_info_valid4())
            .field("vio_info_valid5", &self.vio_info_valid5())
            .field("vio_info_valid6", &self.vio_info_valid6())
            .field("vio_info_valid7", &self.vio_info_valid7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecVioInfoValid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecVioInfoValid {{ vio_info_valid0: {=bool:?}, vio_info_valid1: {=bool:?}, vio_info_valid2: {=bool:?}, vio_info_valid3: {=bool:?}, vio_info_valid4: {=bool:?}, vio_info_valid5: {=bool:?}, vio_info_valid6: {=bool:?}, vio_info_valid7: {=bool:?} }}",
            self.vio_info_valid0(),
            self.vio_info_valid1(),
            self.vio_info_valid2(),
            self.vio_info_valid3(),
            self.vio_info_valid4(),
            self.vio_info_valid5(),
            self.vio_info_valid6(),
            self.vio_info_valid7()
        )
    }
}
#[doc = "Security Violation Miscellaneous Information at Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecVioMiscInfo(pub u32);
impl SecVioMiscInfo {
    #[doc = "Security violation access read/write indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_info_write(&self) -> super::vals::SecVioInfoWrite {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SecVioInfoWrite::from_bits(val as u8)
    }
    #[doc = "Security violation access read/write indicator"]
    #[inline(always)]
    pub const fn set_sec_vio_info_write(&mut self, val: super::vals::SecVioInfoWrite) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Security Violation Info Data Access"]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_info_data_access(&self) -> super::vals::SecVioInfoDataAccess {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SecVioInfoDataAccess::from_bits(val as u8)
    }
    #[doc = "Security Violation Info Data Access"]
    #[inline(always)]
    pub const fn set_sec_vio_info_data_access(&mut self, val: super::vals::SecVioInfoDataAccess) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Security Violation Info Master Security Level"]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_info_master_sec_level(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Security Violation Info Master Security Level"]
    #[inline(always)]
    pub const fn set_sec_vio_info_master_sec_level(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Security violation master number"]
    #[must_use]
    #[inline(always)]
    pub const fn sec_vio_info_master(&self) -> super::vals::SecVioInfoMaster {
        let val = (self.0 >> 8usize) & 0x1f;
        super::vals::SecVioInfoMaster::from_bits(val as u8)
    }
    #[doc = "Security violation master number"]
    #[inline(always)]
    pub const fn set_sec_vio_info_master(&mut self, val: super::vals::SecVioInfoMaster) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
    }
}
impl Default for SecVioMiscInfo {
    #[inline(always)]
    fn default() -> SecVioMiscInfo {
        SecVioMiscInfo(0)
    }
}
impl core::fmt::Debug for SecVioMiscInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecVioMiscInfo")
            .field("sec_vio_info_write", &self.sec_vio_info_write())
            .field("sec_vio_info_data_access", &self.sec_vio_info_data_access())
            .field(
                "sec_vio_info_master_sec_level",
                &self.sec_vio_info_master_sec_level(),
            )
            .field("sec_vio_info_master", &self.sec_vio_info_master())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecVioMiscInfo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecVioMiscInfo {{ sec_vio_info_write: {:?}, sec_vio_info_data_access: {:?}, sec_vio_info_master_sec_level: {=u8:?}, sec_vio_info_master: {:?} }}",
            self.sec_vio_info_write(),
            self.sec_vio_info_data_access(),
            self.sec_vio_info_master_sec_level(),
            self.sec_vio_info_master()
        )
    }
}
