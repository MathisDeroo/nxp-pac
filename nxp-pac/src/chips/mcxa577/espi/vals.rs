#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Base {
    #[doc = "Offset from 0 in host memory or I/O space"]
    OFFSET = 0x0,
    #[doc = "Base1 offset in host memory"]
    BASE1 = 0x01,
    #[doc = "Base2 offset in host memory"]
    BASE2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Base {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Base {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Base {
    #[inline(always)]
    fn from(val: u8) -> Base {
        Base::from_bits(val)
    }
}
impl From<Base> for u8 {
    #[inline(always)]
    fn from(val: Base) -> u8 {
        Base::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BootErrn {
    #[doc = "Boot load error"]
    ERROR = 0x0,
    #[doc = "Boot load successful"]
    SUCCESS = 0x01,
}
impl BootErrn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BootErrn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BootErrn {
    #[inline(always)]
    fn from(val: u8) -> BootErrn {
        BootErrn::from_bits(val)
    }
}
impl From<BootErrn> for u8 {
    #[inline(always)]
    fn from(val: BootErrn) -> u8 {
        BootErrn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkDivDisable {
    #[doc = "Clock division is enabled."]
    ENABLE = 0x0,
    #[doc = "Clock division is disabled."]
    DISABLE = 0x01,
}
impl ClkDivDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkDivDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkDivDisable {
    #[inline(always)]
    fn from(val: u8) -> ClkDivDisable {
        ClkDivDisable::from_bits(val)
    }
}
impl From<ClkDivDisable> for u8 {
    #[inline(always)]
    fn from(val: ClkDivDisable) -> u8 {
        ClkDivDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0en {
    #[doc = "Disabled. The DMA channel is not used."]
    DISABLED = 0x0,
    #[doc = "Triggers on Host Read empty (whether endpoint and a byte or mailbox and many bytes). Allows reload of memory."]
    TRIGGERD_ON_HOST_READ = 0x01,
    #[doc = "Triggers on Host Write complete/ready (whether endpoint and a byte, or mailbox and many bytes)."]
    TRIGGERS_ON_HOST_WRITE = 0x02,
    _RESERVED_3 = 0x03,
}
impl Dma0en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0en {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0en {
    #[inline(always)]
    fn from(val: u8) -> Dma0en {
        Dma0en::from_bits(val)
    }
}
impl From<Dma0en> for u8 {
    #[inline(always)]
    fn from(val: Dma0en) -> u8 {
        Dma0en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma1en {
    #[doc = "Disabled. The DMA channel is not used."]
    DISABLED = 0x0,
    #[doc = "Triggers on Host Read empty (whether endpoint and a byte or mailbox and many bytes). Allows reload of memory."]
    TRIGGERD_ON_HOST_READ = 0x01,
    #[doc = "Triggers on Host Write complete/ready (whether endpoint and a byte or mailbox and many bytes)."]
    TRIGGERS_ON_HOST_WRITE = 0x02,
    _RESERVED_3 = 0x03,
}
impl Dma1en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma1en {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma1en {
    #[inline(always)]
    fn from(val: u8) -> Dma1en {
        Dma1en::from_bits(val)
    }
}
impl From<Dma1en> for u8 {
    #[inline(always)]
    fn from(val: Dma1en) -> u8 {
        Dma1en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma1port {
    #[doc = "Port 0"]
    PORT0 = 0x0,
    #[doc = "Port 1"]
    PORT1 = 0x01,
    #[doc = "Port 2"]
    PORT2 = 0x02,
    #[doc = "Port 3"]
    PORT3 = 0x03,
    #[doc = "Port 4"]
    PORT4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Port80. Used to offload the Port80 bytes (only host writes apply to Port80)."]
    PORT80 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Dma1port {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma1port {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma1port {
    #[inline(always)]
    fn from(val: u8) -> Dma1port {
        Dma1port::from_bits(val)
    }
}
impl From<Dma1port> for u8 {
    #[inline(always)]
    fn from(val: Dma1port) -> u8 {
        Dma1port::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enable {
    #[doc = "Disabled. Block is not operational."]
    DISABLED = 0x0,
    #[doc = "eSPI (Enhanced Serial Peripheral Interface)"]
    ESPI = 0x01,
    #[doc = "LPC (Low Pin Count)"]
    LPC = 0x02,
    _RESERVED_3 = 0x03,
}
impl Enable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enable {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enable {
    #[inline(always)]
    fn from(val: u8) -> Enable {
        Enable::from_bits(val)
    }
}
impl From<Enable> for u8 {
    #[inline(always)]
    fn from(val: Enable) -> u8 {
        Enable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flashmx {
    #[doc = "64 bytes"]
    BYTE64 = 0x0,
    #[doc = "128 bytes"]
    BYTE128 = 0x01,
    #[doc = "256 bytes"]
    BYTE256 = 0x02,
    #[doc = "512 bytes"]
    BYTE512 = 0x03,
}
impl Flashmx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flashmx {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flashmx {
    #[inline(always)]
    fn from(val: u8) -> Flashmx {
        Flashmx::from_bits(val)
    }
}
impl From<Flashmx> for u8 {
    #[inline(always)]
    fn from(val: Flashmx) -> u8 {
        Flashmx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flashsz {
    #[doc = "64 bytes"]
    BYTE64 = 0x0,
    #[doc = "128 bytes"]
    BYTE128 = 0x01,
    #[doc = "256 bytes"]
    BYTE256 = 0x02,
    #[doc = "512 bytes"]
    BYTE512 = 0x03,
}
impl Flashsz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flashsz {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flashsz {
    #[inline(always)]
    fn from(val: u8) -> Flashsz {
        Flashsz::from_bits(val)
    }
}
impl From<Flashsz> for u8 {
    #[inline(always)]
    fn from(val: Flashsz) -> u8 {
        Flashsz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flshera {
    #[doc = "Flash not enabled"]
    DISABLED = 0x0,
    #[doc = "Erase is 4 kB"]
    ERASE_4K = 0x01,
    #[doc = "Erase is 64 kB"]
    ERASE_64K = 0x02,
    #[doc = "Erase allows 4 kB and 64 kB"]
    ERASE_4K_64K = 0x03,
    #[doc = "Erase is 128 kB"]
    ERASE_128K = 0x04,
    #[doc = "Erase is 256 kB"]
    ERASE_256K = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Flshera {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flshera {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flshera {
    #[inline(always)]
    fn from(val: u8) -> Flshera {
        Flshera::from_bits(val)
    }
}
impl From<Flshera> for u8 {
    #[inline(always)]
    fn from(val: Flshera) -> u8 {
        Flshera::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioOe {
    #[doc = "Input (High-Z)"]
    OE0 = 0x0,
    #[doc = "Alert or reset pin is set as an output GPIO."]
    OE1 = 0x01,
}
impl GpioOe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioOe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioOe {
    #[inline(always)]
    fn from(val: u8) -> GpioOe {
        GpioOe::from_bits(val)
    }
}
impl From<GpioOe> for u8 {
    #[inline(always)]
    fn from(val: GpioOe) -> u8 {
        GpioOe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioOut {
    #[doc = "Low"]
    OE0 = 0x0,
    #[doc = "High, high-Z if ESPIMISC\\[GPIO_OD\\] = 1."]
    OE1 = 0x01,
}
impl GpioOut {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioOut {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioOut {
    #[inline(always)]
    fn from(val: u8) -> GpioOut {
        GpioOut::from_bits(val)
    }
}
impl From<GpioOut> for u8 {
    #[inline(always)]
    fn from(val: GpioOut) -> u8 {
        GpioOut::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntenclrCportint {
    _RESERVED_0 = 0x0,
    #[doc = "Writes 0 to corresponding coprocessor port interrupt enable"]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl IntenclrCportint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntenclrCportint {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntenclrCportint {
    #[inline(always)]
    fn from(val: u8) -> IntenclrCportint {
        IntenclrCportint::from_bits(val)
    }
}
impl From<IntenclrCportint> for u8 {
    #[inline(always)]
    fn from(val: IntenclrCportint) -> u8 {
        IntenclrCportint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntenclrPortint {
    _RESERVED_0 = 0x0,
    #[doc = "Writes 0 to corresponding port interrupt enable"]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl IntenclrPortint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntenclrPortint {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntenclrPortint {
    #[inline(always)]
    fn from(val: u8) -> IntenclrPortint {
        IntenclrPortint::from_bits(val)
    }
}
impl From<IntenclrPortint> for u8 {
    #[inline(always)]
    fn from(val: IntenclrPortint) -> u8 {
        IntenclrPortint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntensetCportint {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "no description available"]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl IntensetCportint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntensetCportint {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntensetCportint {
    #[inline(always)]
    fn from(val: u8) -> IntensetCportint {
        IntensetCportint::from_bits(val)
    }
}
impl From<IntensetCportint> for u8 {
    #[inline(always)]
    fn from(val: IntensetCportint) -> u8 {
        IntensetCportint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntensetPortint {
    _RESERVED_0 = 0x0,
    #[doc = "Corresponding port interrupts main processor, if it matches IRule"]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl IntensetPortint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntensetPortint {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntensetPortint {
    #[inline(always)]
    fn from(val: u8) -> IntensetPortint {
        IntensetPortint::from_bits(val)
    }
}
impl From<IntensetPortint> for u8 {
    #[inline(always)]
    fn from(val: IntensetPortint) -> u8 {
        IntensetPortint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntstatCportint {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Interrupt is enabled and pending for the coprocessor"]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl IntstatCportint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntstatCportint {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntstatCportint {
    #[inline(always)]
    fn from(val: u8) -> IntstatCportint {
        IntstatCportint::from_bits(val)
    }
}
impl From<IntstatCportint> for u8 {
    #[inline(always)]
    fn from(val: IntstatCportint) -> u8 {
        IntstatCportint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntstatPortint {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Corresponding port interrupts main processor if it matches IRule"]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl IntstatPortint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntstatPortint {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntstatPortint {
    #[inline(always)]
    fn from(val: u8) -> IntstatPortint {
        IntstatPortint::from_bits(val)
    }
}
impl From<IntstatPortint> for u8 {
    #[inline(always)]
    fn from(val: IntstatPortint) -> u8 {
        IntstatPortint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Maxspd {
    #[doc = "<=20 MHz"]
    SMALL_THAN_20M = 0x0,
    #[doc = "<=25 MHz (24 Mhz)"]
    SMALL_THAN_25M = 0x01,
    #[doc = "<=33 MHz (30 MHz)"]
    SMALL_THAN_33M = 0x02,
    #[doc = "<=50 MHz (48 MHz)"]
    SMALL_THAN_50M = 0x03,
    #[doc = "<=66 MHz (60 MHz)"]
    SMALL_THAN_66M = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Maxspd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Maxspd {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Maxspd {
    #[inline(always)]
    fn from(val: u8) -> Maxspd {
        Maxspd::from_bits(val)
    }
}
impl From<Maxspd> for u8 {
    #[inline(always)]
    fn from(val: Maxspd) -> u8 {
        Maxspd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Memmx {
    _RESERVED_0 = 0x0,
    #[doc = "64 bytes address aligned max payload size"]
    MIN_4KB = 0x01,
    #[doc = "128 bytes address aligned max payload size"]
    MIN_8KB = 0x02,
    #[doc = "256 bytes address aligned max payload size"]
    MIN_16KB = 0x03,
}
impl Memmx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Memmx {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Memmx {
    #[inline(always)]
    fn from(val: u8) -> Memmx {
        Memmx::from_bits(val)
    }
}
impl From<Memmx> for u8 {
    #[inline(always)]
    fn from(val: Memmx) -> u8 {
        Memmx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Memsz {
    _RESERVED_0 = 0x0,
    #[doc = "64-byte payload for memory"]
    MEMSZ0 = 0x01,
    #[doc = "128-byte payload for memory and OOB access"]
    MEMSZ1 = 0x02,
    #[doc = "256-byte payload for memory and OOB access"]
    MEMSZ11 = 0x03,
}
impl Memsz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Memsz {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Memsz {
    #[inline(always)]
    fn from(val: u8) -> Memsz {
        Memsz::from_bits(val)
    }
}
impl From<Memsz> for u8 {
    #[inline(always)]
    fn from(val: Memsz) -> u8 {
        Memsz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NumOfTarget {
    #[doc = "1 RPMC flash device is supported"]
    MIXXN_2KB = 0x0,
    #[doc = "2 RPMC flash device is supported"]
    MIYYN_4KB = 0x01,
    #[doc = "3 RPMC flash device is supported"]
    MXXXIN_8KB = 0x02,
    #[doc = "4 RPMC flash device is supported"]
    MINTTT_16KB = 0x03,
}
impl NumOfTarget {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NumOfTarget {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NumOfTarget {
    #[inline(always)]
    fn from(val: u8) -> NumOfTarget {
        NumOfTarget::from_bits(val)
    }
}
impl From<NumOfTarget> for u8 {
    #[inline(always)]
    fn from(val: NumOfTarget) -> u8 {
        NumOfTarget::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Oobmx {
    _RESERVED_0 = 0x0,
    #[doc = "64 bytes address aligned max payload size"]
    MIN_4KB = 0x01,
    #[doc = "128 bytes address aligned max payload size"]
    MIN_8KB = 0x02,
    #[doc = "256 bytes address aligned max payload size"]
    MIN_16KB = 0x03,
}
impl Oobmx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oobmx {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oobmx {
    #[inline(always)]
    fn from(val: u8) -> Oobmx {
        Oobmx::from_bits(val)
    }
}
impl From<Oobmx> for u8 {
    #[inline(always)]
    fn from(val: Oobmx) -> u8 {
        Oobmx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Oobsz {
    _RESERVED_0 = 0x0,
    #[doc = "64-byte payload for memory"]
    MEMSZ0 = 0x01,
    #[doc = "128-byte payload for memory and OOB access"]
    MEMSZ1 = 0x02,
    #[doc = "256-byte payload for memory and OOB access"]
    MEMSZ11 = 0x03,
}
impl Oobsz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oobsz {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oobsz {
    #[inline(always)]
    fn from(val: u8) -> Oobsz {
        Oobsz::from_bits(val)
    }
}
impl From<Oobsz> for u8 {
    #[inline(always)]
    fn from(val: Oobsz) -> u8 {
        Oobsz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0addrBaseAsz {
    #[doc = "Offset from 0 in host memory or I/O space."]
    OFFSET_FROM_0 = 0x0,
    #[doc = "Uses BASE0 offset in host memory"]
    USE_BASE0 = 0x01,
    #[doc = "Uses BASE1 offset in host memory"]
    USE_BASE1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl P0addrBaseAsz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0addrBaseAsz {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0addrBaseAsz {
    #[inline(always)]
    fn from(val: u8) -> P0addrBaseAsz {
        P0addrBaseAsz::from_bits(val)
    }
}
impl From<P0addrBaseAsz> for u8 {
    #[inline(always)]
    fn from(val: P0addrBaseAsz) -> u8 {
        P0addrBaseAsz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0addrIdx1st {
    #[doc = "Index is higher address than data (for example, data at OFF, index at OFF+IDXOFF)"]
    DATAOFF = 0x0,
    #[doc = "Index is lower address than data (for example, index at OFF, data at OFF+IDXOFF)"]
    IDXOFF = 0x01,
}
impl P0addrIdx1st {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0addrIdx1st {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0addrIdx1st {
    #[inline(always)]
    fn from(val: u8) -> P0addrIdx1st {
        P0addrIdx1st::from_bits(val)
    }
}
impl From<P0addrIdx1st> for u8 {
    #[inline(always)]
    fn from(val: P0addrIdx1st) -> u8 {
        P0addrIdx1st::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0cfgDirection {
    #[doc = "Endpoint or index-and-data: bidirectional (normal). Mailbox Single: unenforced. Mailbox Split or Mailbox Shared: bidirectional."]
    DIR0 = 0x0,
    #[doc = "Endpoint or index-and-data: ignore read. Mailbox Single: write only. Mailbox Split or Mailbox Shared: ignore read."]
    DIR1 = 0x01,
    #[doc = "Endpoint or index-and-data: ignore write. Mailbox Single: read only. Mailbox Split or Mailbox Shared: ignore write."]
    DIR2 = 0x02,
    #[doc = "Endpoint or index-and-data: Ignore both. Mailbox Single: ignore both. Mailbox Split or Mailbox Shared: ignore both."]
    DIR3 = 0x03,
}
impl P0cfgDirection {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0cfgDirection {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0cfgDirection {
    #[inline(always)]
    fn from(val: u8) -> P0cfgDirection {
        P0cfgDirection::from_bits(val)
    }
}
impl From<P0cfgDirection> for u8 {
    #[inline(always)]
    fn from(val: P0cfgDirection) -> u8 {
        P0cfgDirection::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0cfgErrorign {
    #[doc = "The host receives an error when trying to perform a read or write that is blocked by PnCFG\\[DIRECTION\\]."]
    ERRORIGIN0 = 0x0,
    #[doc = "Ignored silently. Reads return FFh for each byte if ignored (host property). In either case, PnSTAT\\[INTERR\\] bit becomes 1, causing an interrupt if masked for it."]
    ERRORIGIN1 = 0x01,
}
impl P0cfgErrorign {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0cfgErrorign {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0cfgErrorign {
    #[inline(always)]
    fn from(val: u8) -> P0cfgErrorign {
        P0cfgErrorign::from_bits(val)
    }
}
impl From<P0cfgErrorign> for u8 {
    #[inline(always)]
    fn from(val: P0cfgErrorign) -> u8 {
        P0cfgErrorign::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0cfgType {
    #[doc = "Unconfigured (reset condition)"]
    UNCONFIGURED = 0x0,
    #[doc = "ACPI style Endpoint"]
    ACPI_END = 0x01,
    #[doc = "ACPI style index-and-data. Index and data byte locations. Index gives offset into implied space. Uses registers for data and index."]
    ACPI_INDEX = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Bus Master Memory Single"]
    BUS_M_MEM_S = 0x04,
    #[doc = "Bus Master Flash Single"]
    BUS_M_FLASH_S = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Mailbox Shared"]
    MAILBOX_SHARED = 0x08,
    #[doc = "Mailbox Single"]
    MAILBOX_SINGLE = 0x09,
    #[doc = "Mailbox Split"]
    MAILBOX_SPLIT = 0x0a,
    #[doc = "Mailbox OOB Split"]
    MAILBOX_OOB_SPLIT = 0x0b,
    #[doc = "Mailbox OEM"]
    MAILBOX_OEM = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl P0cfgType {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0cfgType {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0cfgType {
    #[inline(always)]
    fn from(val: u8) -> P0cfgType {
        P0cfgType::from_bits(val)
    }
}
impl From<P0cfgType> for u8 {
    #[inline(always)]
    fn from(val: P0cfgType) -> u8 {
        P0cfgType::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0datainDir {
    #[doc = "Read by host"]
    ENABLE = 0x0,
    #[doc = "Write by host"]
    DISABLE = 0x01,
}
impl P0datainDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0datainDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0datainDir {
    #[inline(always)]
    fn from(val: u8) -> P0datainDir {
        P0datainDir::from_bits(val)
    }
}
impl From<P0datainDir> for u8 {
    #[inline(always)]
    fn from(val: P0datainDir) -> u8 {
        P0datainDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0irulestatFlashCompletionType {
    #[doc = "Indicates the middle completion of a split completion sequence"]
    MIDDLE = 0x0,
    #[doc = "Indicates the first completion of a split completion sequence"]
    FIRST = 0x01,
    #[doc = "Indicates the last completion of a split completion sequence"]
    LAST = 0x02,
    #[doc = "Indicates the only completion for a split transaction."]
    ONLY = 0x03,
}
impl P0irulestatFlashCompletionType {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0irulestatFlashCompletionType {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0irulestatFlashCompletionType {
    #[inline(always)]
    fn from(val: u8) -> P0irulestatFlashCompletionType {
        P0irulestatFlashCompletionType::from_bits(val)
    }
}
impl From<P0irulestatFlashCompletionType> for u8 {
    #[inline(always)]
    fn from(val: P0irulestatFlashCompletionType) -> u8 {
        P0irulestatFlashCompletionType::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0omflenTrans {
    #[doc = "OOB: to host; Master: to host 32 (host reads with 32-bit address); SAF: Completion fail"]
    OOB = 0x0,
    #[doc = "Master: to host 64 (host reads w/64-bit address); MAF: read flash (location in RAM); SAF: completion with data"]
    READ = 0x01,
    #[doc = "Master: from host 32 (host writes w/32-bit address); MAF: write flash (location in RAM); SAF: completion with no data"]
    WRITE = 0x02,
    #[doc = "Master: from host 64 (host writes w/64-bit address); MAF: erase flash (sector in RAM)"]
    ERASE = 0x03,
}
impl P0omflenTrans {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0omflenTrans {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0omflenTrans {
    #[inline(always)]
    fn from(val: u8) -> P0omflenTrans {
        P0omflenTrans::from_bits(val)
    }
}
impl From<P0omflenTrans> for u8 {
    #[inline(always)]
    fn from(val: P0omflenTrans) -> u8 {
        P0omflenTrans::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0statRdstat {
    #[doc = "Endpoint or index-and-data: Empty. Mailbox: Empty (host read to end). Master/Flash: No requests."]
    RDSTAT0 = 0x0,
    #[doc = "Endpoint or index-and-data: Data waiting from MCU. Mailbox: Started (by MCU). Master/Flash: Started. From-host: goes to Pending next. To-host: goes to Complete next. SAF: host made request: Started."]
    RDSTAT1 = 0x01,
    #[doc = "Mailbox: Complete (by MCU). Master/Flash: Complete. SAF: Complete."]
    RDSTAT2 = 0x02,
    #[doc = "Mailbox: Partially read (by host). Master/Flash: From-host only, Pending (request made); goes to Complete next. SAF: MCU has setup completion."]
    RDSTAT3 = 0x03,
}
impl P0statRdstat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0statRdstat {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0statRdstat {
    #[inline(always)]
    fn from(val: u8) -> P0statRdstat {
        P0statRdstat::from_bits(val)
    }
}
impl From<P0statRdstat> for u8 {
    #[inline(always)]
    fn from(val: P0statRdstat) -> u8 {
        P0statRdstat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0statRpmc1Or2 {
    #[doc = "RPMC operation 1 has been chosen"]
    OPERATION_0 = 0x0,
    #[doc = "RPMC operation 2 has been chosen"]
    OPERATION_1 = 0x01,
}
impl P0statRpmc1Or2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0statRpmc1Or2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0statRpmc1Or2 {
    #[inline(always)]
    fn from(val: u8) -> P0statRpmc1Or2 {
        P0statRpmc1Or2::from_bits(val)
    }
}
impl From<P0statRpmc1Or2> for u8 {
    #[inline(always)]
    fn from(val: P0statRpmc1Or2) -> u8 {
        P0statRpmc1Or2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0statRpmcFlashDevice {
    #[doc = "First device"]
    FIRST = 0x0,
    #[doc = "Second device"]
    SECOND = 0x01,
    #[doc = "Third device"]
    THIRD = 0x02,
    _RESERVED_3 = 0x03,
}
impl P0statRpmcFlashDevice {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0statRpmcFlashDevice {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0statRpmcFlashDevice {
    #[inline(always)]
    fn from(val: u8) -> P0statRpmcFlashDevice {
        P0statRpmcFlashDevice::from_bits(val)
    }
}
impl From<P0statRpmcFlashDevice> for u8 {
    #[inline(always)]
    fn from(val: P0statRpmcFlashDevice) -> u8 {
        P0statRpmcFlashDevice::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P0statWrstat {
    #[doc = "Endpoint: Empty. Index-and-data: Empty. Mailbox: Empty. Master/flash: Not last request."]
    WRSTAT0 = 0x0,
    #[doc = "Endpoint: data waiting from host. Index-and-data: data waiting from host (index may have been written before). Mailbox: Started (by host). Master/flash: last request from-host/read-flash (writes to MCU). SAF: Request from master was or is Flash Read."]
    WRSTAT1 = 0x01,
    #[doc = "Endpoint: Empty, but last was CMD. Index-and-data: wrote index. Mailbox: complete/last (host to end). Master/flash: last request to-host/write-flash (read from MCU). SAF: Request from Master was or is Flash Write."]
    WRSTAT2 = 0x02,
    #[doc = "Endpoint: CMD waiting from host. Index-and-data: Wrote Data then Index. Mailbox: Partially read (by MCU). Flash: Last Requested Erase. SAF: Request from Master was or is Flash Erase."]
    WRSTAT3 = 0x03,
}
impl P0statWrstat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P0statWrstat {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P0statWrstat {
    #[inline(always)]
    fn from(val: u8) -> P0statWrstat {
        P0statWrstat::from_bits(val)
    }
}
impl From<P0statWrstat> for u8 {
    #[inline(always)]
    fn from(val: P0statWrstat) -> u8 {
        P0statWrstat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1addrBaseAsz {
    #[doc = "Offset from 0 in host memory or I/O space."]
    OFFSET_FROM_0 = 0x0,
    #[doc = "Uses BASE0 offset in host memory"]
    USE_BASE0 = 0x01,
    #[doc = "Uses BASE1 offset in host memory"]
    USE_BASE1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl P1addrBaseAsz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1addrBaseAsz {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1addrBaseAsz {
    #[inline(always)]
    fn from(val: u8) -> P1addrBaseAsz {
        P1addrBaseAsz::from_bits(val)
    }
}
impl From<P1addrBaseAsz> for u8 {
    #[inline(always)]
    fn from(val: P1addrBaseAsz) -> u8 {
        P1addrBaseAsz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1addrIdx1st {
    #[doc = "Index is higher address than data (for example, data at OFF, index at OFF+IDXOFF)"]
    DATAOFF = 0x0,
    #[doc = "Index is lower address than data (for example, index at OFF, data at OFF+IDXOFF)"]
    IDXOFF = 0x01,
}
impl P1addrIdx1st {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1addrIdx1st {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1addrIdx1st {
    #[inline(always)]
    fn from(val: u8) -> P1addrIdx1st {
        P1addrIdx1st::from_bits(val)
    }
}
impl From<P1addrIdx1st> for u8 {
    #[inline(always)]
    fn from(val: P1addrIdx1st) -> u8 {
        P1addrIdx1st::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1cfgDirection {
    #[doc = "Endpoint or index-and-data: bidirectional (normal). Mailbox Single: unenforced. Mailbox Split or Mailbox Shared: bidirectional."]
    DIR0 = 0x0,
    #[doc = "Endpoint or index-and-data: ignore read. Mailbox Single: write only. Mailbox Split or Mailbox Shared: ignore read."]
    DIR1 = 0x01,
    #[doc = "Endpoint or index-and-data: ignore write. Mailbox Single: read only. Mailbox Split or Mailbox Shared: ignore write."]
    DIR2 = 0x02,
    #[doc = "Endpoint or index-and-data: Ignore both. Mailbox Single: ignore both. Mailbox Split or Mailbox Shared: ignore both."]
    DIR3 = 0x03,
}
impl P1cfgDirection {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1cfgDirection {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1cfgDirection {
    #[inline(always)]
    fn from(val: u8) -> P1cfgDirection {
        P1cfgDirection::from_bits(val)
    }
}
impl From<P1cfgDirection> for u8 {
    #[inline(always)]
    fn from(val: P1cfgDirection) -> u8 {
        P1cfgDirection::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1cfgErrorign {
    #[doc = "The host receives an error when trying to perform a read or write that is blocked by PnCFG\\[DIRECTION\\]."]
    ERRORIGIN0 = 0x0,
    #[doc = "Ignored silently. Reads return FFh for each byte if ignored (host property). In either case, PnSTAT\\[INTERR\\] bit becomes 1, causing an interrupt if masked for it."]
    ERRORIGIN1 = 0x01,
}
impl P1cfgErrorign {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1cfgErrorign {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1cfgErrorign {
    #[inline(always)]
    fn from(val: u8) -> P1cfgErrorign {
        P1cfgErrorign::from_bits(val)
    }
}
impl From<P1cfgErrorign> for u8 {
    #[inline(always)]
    fn from(val: P1cfgErrorign) -> u8 {
        P1cfgErrorign::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1cfgType {
    #[doc = "Unconfigured (reset condition)"]
    UNCONFIGURED = 0x0,
    #[doc = "ACPI style Endpoint"]
    ACPI_END = 0x01,
    #[doc = "ACPI style index-and-data. Index and data byte locations. Index gives offset into implied space. Uses registers for data and index."]
    ACPI_INDEX = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Bus Master Memory Single"]
    BUS_M_MEM_S = 0x04,
    #[doc = "Bus Master Flash Single"]
    BUS_M_FLASH_S = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Mailbox Shared"]
    MAILBOX_SHARED = 0x08,
    #[doc = "Mailbox Single"]
    MAILBOX_SINGLE = 0x09,
    #[doc = "Mailbox Split"]
    MAILBOX_SPLIT = 0x0a,
    #[doc = "Mailbox OOB Split"]
    MAILBOX_OOB_SPLIT = 0x0b,
    #[doc = "Mailbox OEM"]
    MAILBOX_OEM = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl P1cfgType {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1cfgType {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1cfgType {
    #[inline(always)]
    fn from(val: u8) -> P1cfgType {
        P1cfgType::from_bits(val)
    }
}
impl From<P1cfgType> for u8 {
    #[inline(always)]
    fn from(val: P1cfgType) -> u8 {
        P1cfgType::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1datainDir {
    #[doc = "Read by host"]
    ENABLE = 0x0,
    #[doc = "Write by host"]
    DISABLE = 0x01,
}
impl P1datainDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1datainDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1datainDir {
    #[inline(always)]
    fn from(val: u8) -> P1datainDir {
        P1datainDir::from_bits(val)
    }
}
impl From<P1datainDir> for u8 {
    #[inline(always)]
    fn from(val: P1datainDir) -> u8 {
        P1datainDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1irulestatFlashCompletionType {
    #[doc = "Indicates the middle completion of a split completion sequence"]
    MIDDLE = 0x0,
    #[doc = "Indicates the first completion of a split completion sequence"]
    FIRST = 0x01,
    #[doc = "Indicates the last completion of a split completion sequence"]
    LAST = 0x02,
    #[doc = "Indicates the only completion for a split transaction."]
    ONLY = 0x03,
}
impl P1irulestatFlashCompletionType {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1irulestatFlashCompletionType {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1irulestatFlashCompletionType {
    #[inline(always)]
    fn from(val: u8) -> P1irulestatFlashCompletionType {
        P1irulestatFlashCompletionType::from_bits(val)
    }
}
impl From<P1irulestatFlashCompletionType> for u8 {
    #[inline(always)]
    fn from(val: P1irulestatFlashCompletionType) -> u8 {
        P1irulestatFlashCompletionType::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1omflenTrans {
    #[doc = "OOB: to host; Master: to host 32 (host reads with 32-bit address); SAF: Completion fail"]
    OOB = 0x0,
    #[doc = "Master: to host 64 (host reads w/64-bit address); MAF: read flash (location in RAM); SAF: completion with data"]
    READ = 0x01,
    #[doc = "Master: from host 32 (host writes w/32-bit address); MAF: write flash (location in RAM); SAF: completion with no data"]
    WRITE = 0x02,
    #[doc = "Master: from host 64 (host writes w/64-bit address); MAF: erase flash (sector in RAM)"]
    ERASE = 0x03,
}
impl P1omflenTrans {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1omflenTrans {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1omflenTrans {
    #[inline(always)]
    fn from(val: u8) -> P1omflenTrans {
        P1omflenTrans::from_bits(val)
    }
}
impl From<P1omflenTrans> for u8 {
    #[inline(always)]
    fn from(val: P1omflenTrans) -> u8 {
        P1omflenTrans::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1statRdstat {
    #[doc = "Endpoint or index-and-data: Empty. Mailbox: Empty (host read to end). Master/Flash: No requests."]
    RDSTAT0 = 0x0,
    #[doc = "Endpoint or index-and-data: Data waiting from MCU. Mailbox: Started (by MCU). Master/Flash: Started. From-host: goes to Pending next. To-host: goes to Complete next. SAF: host made request: Started."]
    RDSTAT1 = 0x01,
    #[doc = "Mailbox: Complete (by MCU). Master/Flash: Complete. SAF: Complete."]
    RDSTAT2 = 0x02,
    #[doc = "Mailbox: Partially read (by host). Master/Flash: From-host only, Pending (request made); goes to Complete next. SAF: MCU has setup completion."]
    RDSTAT3 = 0x03,
}
impl P1statRdstat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1statRdstat {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1statRdstat {
    #[inline(always)]
    fn from(val: u8) -> P1statRdstat {
        P1statRdstat::from_bits(val)
    }
}
impl From<P1statRdstat> for u8 {
    #[inline(always)]
    fn from(val: P1statRdstat) -> u8 {
        P1statRdstat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1statRpmc1Or2 {
    #[doc = "RPMC operation 1 has been chosen"]
    OPERATION_0 = 0x0,
    #[doc = "RPMC operation 2 has been chosen"]
    OPERATION_1 = 0x01,
}
impl P1statRpmc1Or2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1statRpmc1Or2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1statRpmc1Or2 {
    #[inline(always)]
    fn from(val: u8) -> P1statRpmc1Or2 {
        P1statRpmc1Or2::from_bits(val)
    }
}
impl From<P1statRpmc1Or2> for u8 {
    #[inline(always)]
    fn from(val: P1statRpmc1Or2) -> u8 {
        P1statRpmc1Or2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1statRpmcFlashDevice {
    #[doc = "First device"]
    FIRST = 0x0,
    #[doc = "Second device"]
    SECOND = 0x01,
    #[doc = "Third device"]
    THIRD = 0x02,
    _RESERVED_3 = 0x03,
}
impl P1statRpmcFlashDevice {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1statRpmcFlashDevice {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1statRpmcFlashDevice {
    #[inline(always)]
    fn from(val: u8) -> P1statRpmcFlashDevice {
        P1statRpmcFlashDevice::from_bits(val)
    }
}
impl From<P1statRpmcFlashDevice> for u8 {
    #[inline(always)]
    fn from(val: P1statRpmcFlashDevice) -> u8 {
        P1statRpmcFlashDevice::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P1statWrstat {
    #[doc = "Endpoint: Empty. Index-and-data: Empty. Mailbox: Empty. Master/flash: Not last request."]
    WRSTAT0 = 0x0,
    #[doc = "Endpoint: data waiting from host. Index-and-data: data waiting from host (index may have been written before). Mailbox: Started (by host). Master/flash: last request from-host/read-flash (writes to MCU). SAF: Request from master was or is Flash Read."]
    WRSTAT1 = 0x01,
    #[doc = "Endpoint: Empty, but last was CMD. Index-and-data: wrote index. Mailbox: complete/last (host to end). Master/flash: last request to-host/write-flash (read from MCU). SAF: Request from Master was or is Flash Write."]
    WRSTAT2 = 0x02,
    #[doc = "Endpoint: CMD waiting from host. Index-and-data: Wrote Data then Index. Mailbox: Partially read (by MCU). Flash: Last Requested Erase. SAF: Request from Master was or is Flash Erase."]
    WRSTAT3 = 0x03,
}
impl P1statWrstat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P1statWrstat {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P1statWrstat {
    #[inline(always)]
    fn from(val: u8) -> P1statWrstat {
        P1statWrstat::from_bits(val)
    }
}
impl From<P1statWrstat> for u8 {
    #[inline(always)]
    fn from(val: P1statWrstat) -> u8 {
        P1statWrstat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P2addrBaseAsz {
    #[doc = "Offset from 0 in host memory or I/O space."]
    OFFSET_FROM_0 = 0x0,
    #[doc = "Uses BASE0 offset in host memory"]
    USE_BASE0 = 0x01,
    #[doc = "Uses BASE1 offset in host memory"]
    USE_BASE1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl P2addrBaseAsz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P2addrBaseAsz {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P2addrBaseAsz {
    #[inline(always)]
    fn from(val: u8) -> P2addrBaseAsz {
        P2addrBaseAsz::from_bits(val)
    }
}
impl From<P2addrBaseAsz> for u8 {
    #[inline(always)]
    fn from(val: P2addrBaseAsz) -> u8 {
        P2addrBaseAsz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P2addrIdx1st {
    #[doc = "Index is higher address than data (for example, data at OFF, index at OFF+IDXOFF)"]
    DATAOFF = 0x0,
    #[doc = "Index is lower address than data (for example, index at OFF, data at OFF+IDXOFF)"]
    IDXOFF = 0x01,
}
impl P2addrIdx1st {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P2addrIdx1st {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P2addrIdx1st {
    #[inline(always)]
    fn from(val: u8) -> P2addrIdx1st {
        P2addrIdx1st::from_bits(val)
    }
}
impl From<P2addrIdx1st> for u8 {
    #[inline(always)]
    fn from(val: P2addrIdx1st) -> u8 {
        P2addrIdx1st::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P2cfgDirection {
    #[doc = "Endpoint or index-and-data: bidirectional (normal). Mailbox Single: unenforced. Mailbox Split or Mailbox Shared: bidirectional."]
    DIR0 = 0x0,
    #[doc = "Endpoint or index-and-data: ignore read. Mailbox Single: write only. Mailbox Split or Mailbox Shared: ignore read."]
    DIR1 = 0x01,
    #[doc = "Endpoint or index-and-data: ignore write. Mailbox Single: read only. Mailbox Split or Mailbox Shared: ignore write."]
    DIR2 = 0x02,
    #[doc = "Endpoint or index-and-data: Ignore both. Mailbox Single: ignore both. Mailbox Split or Mailbox Shared: ignore both."]
    DIR3 = 0x03,
}
impl P2cfgDirection {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P2cfgDirection {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P2cfgDirection {
    #[inline(always)]
    fn from(val: u8) -> P2cfgDirection {
        P2cfgDirection::from_bits(val)
    }
}
impl From<P2cfgDirection> for u8 {
    #[inline(always)]
    fn from(val: P2cfgDirection) -> u8 {
        P2cfgDirection::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P2cfgErrorign {
    #[doc = "The host receives an error when trying to perform a read or write that is blocked by PnCFG\\[DIRECTION\\]."]
    ERRORIGIN0 = 0x0,
    #[doc = "Ignored silently. Reads return FFh for each byte if ignored (host property). In either case, PnSTAT\\[INTERR\\] bit becomes 1, causing an interrupt if masked for it."]
    ERRORIGIN1 = 0x01,
}
impl P2cfgErrorign {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P2cfgErrorign {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P2cfgErrorign {
    #[inline(always)]
    fn from(val: u8) -> P2cfgErrorign {
        P2cfgErrorign::from_bits(val)
    }
}
impl From<P2cfgErrorign> for u8 {
    #[inline(always)]
    fn from(val: P2cfgErrorign) -> u8 {
        P2cfgErrorign::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P2cfgType {
    #[doc = "Unconfigured (reset condition)"]
    UNCONFIGURED = 0x0,
    #[doc = "ACPI style Endpoint"]
    ACPI_END = 0x01,
    #[doc = "ACPI style index-and-data. Index and data byte locations. Index gives offset into implied space. Uses registers for data and index."]
    ACPI_INDEX = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Bus Master Memory Single"]
    BUS_M_MEM_S = 0x04,
    #[doc = "Bus Master Flash Single"]
    BUS_M_FLASH_S = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Mailbox Shared"]
    MAILBOX_SHARED = 0x08,
    #[doc = "Mailbox Single"]
    MAILBOX_SINGLE = 0x09,
    #[doc = "Mailbox Split"]
    MAILBOX_SPLIT = 0x0a,
    #[doc = "Mailbox OOB Split"]
    MAILBOX_OOB_SPLIT = 0x0b,
    #[doc = "Mailbox OEM"]
    MAILBOX_OEM = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl P2cfgType {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P2cfgType {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P2cfgType {
    #[inline(always)]
    fn from(val: u8) -> P2cfgType {
        P2cfgType::from_bits(val)
    }
}
impl From<P2cfgType> for u8 {
    #[inline(always)]
    fn from(val: P2cfgType) -> u8 {
        P2cfgType::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P2datainDir {
    #[doc = "Read by host"]
    ENABLE = 0x0,
    #[doc = "Write by host"]
    DISABLE = 0x01,
}
impl P2datainDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P2datainDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P2datainDir {
    #[inline(always)]
    fn from(val: u8) -> P2datainDir {
        P2datainDir::from_bits(val)
    }
}
impl From<P2datainDir> for u8 {
    #[inline(always)]
    fn from(val: P2datainDir) -> u8 {
        P2datainDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P2irulestatFlashCompletionType {
    #[doc = "Indicates the middle completion of a split completion sequence"]
    MIDDLE = 0x0,
    #[doc = "Indicates the first completion of a split completion sequence"]
    FIRST = 0x01,
    #[doc = "Indicates the last completion of a split completion sequence"]
    LAST = 0x02,
    #[doc = "Indicates the only completion for a split transaction."]
    ONLY = 0x03,
}
impl P2irulestatFlashCompletionType {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P2irulestatFlashCompletionType {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P2irulestatFlashCompletionType {
    #[inline(always)]
    fn from(val: u8) -> P2irulestatFlashCompletionType {
        P2irulestatFlashCompletionType::from_bits(val)
    }
}
impl From<P2irulestatFlashCompletionType> for u8 {
    #[inline(always)]
    fn from(val: P2irulestatFlashCompletionType) -> u8 {
        P2irulestatFlashCompletionType::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P2omflenTrans {
    #[doc = "OOB: to host; Master: to host 32 (host reads with 32-bit address); SAF: Completion fail"]
    OOB = 0x0,
    #[doc = "Master: to host 64 (host reads w/64-bit address); MAF: read flash (location in RAM); SAF: completion with data"]
    READ = 0x01,
    #[doc = "Master: from host 32 (host writes w/32-bit address); MAF: write flash (location in RAM); SAF: completion with no data"]
    WRITE = 0x02,
    #[doc = "Master: from host 64 (host writes w/64-bit address); MAF: erase flash (sector in RAM)"]
    ERASE = 0x03,
}
impl P2omflenTrans {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P2omflenTrans {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P2omflenTrans {
    #[inline(always)]
    fn from(val: u8) -> P2omflenTrans {
        P2omflenTrans::from_bits(val)
    }
}
impl From<P2omflenTrans> for u8 {
    #[inline(always)]
    fn from(val: P2omflenTrans) -> u8 {
        P2omflenTrans::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P2statRdstat {
    #[doc = "Endpoint or index-and-data: Empty. Mailbox: Empty (host read to end). Master/Flash: No requests."]
    RDSTAT0 = 0x0,
    #[doc = "Endpoint or index-and-data: Data waiting from MCU. Mailbox: Started (by MCU). Master/Flash: Started. From-host: goes to Pending next. To-host: goes to Complete next. SAF: host made request: Started."]
    RDSTAT1 = 0x01,
    #[doc = "Mailbox: Complete (by MCU). Master/Flash: Complete. SAF: Complete."]
    RDSTAT2 = 0x02,
    #[doc = "Mailbox: Partially read (by host). Master/Flash: From-host only, Pending (request made); goes to Complete next. SAF: MCU has setup completion."]
    RDSTAT3 = 0x03,
}
impl P2statRdstat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P2statRdstat {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P2statRdstat {
    #[inline(always)]
    fn from(val: u8) -> P2statRdstat {
        P2statRdstat::from_bits(val)
    }
}
impl From<P2statRdstat> for u8 {
    #[inline(always)]
    fn from(val: P2statRdstat) -> u8 {
        P2statRdstat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P2statRpmc1Or2 {
    #[doc = "RPMC operation 1 has been chosen"]
    OPERATION_0 = 0x0,
    #[doc = "RPMC operation 2 has been chosen"]
    OPERATION_1 = 0x01,
}
impl P2statRpmc1Or2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P2statRpmc1Or2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P2statRpmc1Or2 {
    #[inline(always)]
    fn from(val: u8) -> P2statRpmc1Or2 {
        P2statRpmc1Or2::from_bits(val)
    }
}
impl From<P2statRpmc1Or2> for u8 {
    #[inline(always)]
    fn from(val: P2statRpmc1Or2) -> u8 {
        P2statRpmc1Or2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P2statRpmcFlashDevice {
    #[doc = "First device"]
    FIRST = 0x0,
    #[doc = "Second device"]
    SECOND = 0x01,
    #[doc = "Third device"]
    THIRD = 0x02,
    _RESERVED_3 = 0x03,
}
impl P2statRpmcFlashDevice {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P2statRpmcFlashDevice {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P2statRpmcFlashDevice {
    #[inline(always)]
    fn from(val: u8) -> P2statRpmcFlashDevice {
        P2statRpmcFlashDevice::from_bits(val)
    }
}
impl From<P2statRpmcFlashDevice> for u8 {
    #[inline(always)]
    fn from(val: P2statRpmcFlashDevice) -> u8 {
        P2statRpmcFlashDevice::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P2statWrstat {
    #[doc = "Endpoint: Empty. Index-and-data: Empty. Mailbox: Empty. Master/flash: Not last request."]
    WRSTAT0 = 0x0,
    #[doc = "Endpoint: data waiting from host. Index-and-data: data waiting from host (index may have been written before). Mailbox: Started (by host). Master/flash: last request from-host/read-flash (writes to MCU). SAF: Request from master was or is Flash Read."]
    WRSTAT1 = 0x01,
    #[doc = "Endpoint: Empty, but last was CMD. Index-and-data: wrote index. Mailbox: complete/last (host to end). Master/flash: last request to-host/write-flash (read from MCU). SAF: Request from Master was or is Flash Write."]
    WRSTAT2 = 0x02,
    #[doc = "Endpoint: CMD waiting from host. Index-and-data: Wrote Data then Index. Mailbox: Partially read (by MCU). Flash: Last Requested Erase. SAF: Request from Master was or is Flash Erase."]
    WRSTAT3 = 0x03,
}
impl P2statWrstat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P2statWrstat {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P2statWrstat {
    #[inline(always)]
    fn from(val: u8) -> P2statWrstat {
        P2statWrstat::from_bits(val)
    }
}
impl From<P2statWrstat> for u8 {
    #[inline(always)]
    fn from(val: P2statWrstat) -> u8 {
        P2statWrstat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P3addrBaseAsz {
    #[doc = "Offset from 0 in host memory or I/O space."]
    OFFSET_FROM_0 = 0x0,
    #[doc = "Uses BASE0 offset in host memory"]
    USE_BASE0 = 0x01,
    #[doc = "Uses BASE1 offset in host memory"]
    USE_BASE1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl P3addrBaseAsz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P3addrBaseAsz {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P3addrBaseAsz {
    #[inline(always)]
    fn from(val: u8) -> P3addrBaseAsz {
        P3addrBaseAsz::from_bits(val)
    }
}
impl From<P3addrBaseAsz> for u8 {
    #[inline(always)]
    fn from(val: P3addrBaseAsz) -> u8 {
        P3addrBaseAsz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P3addrIdx1st {
    #[doc = "Index is higher address than data (for example, data at OFF, index at OFF+IDXOFF)"]
    DATAOFF = 0x0,
    #[doc = "Index is lower address than data (for example, index at OFF, data at OFF+IDXOFF)"]
    IDXOFF = 0x01,
}
impl P3addrIdx1st {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P3addrIdx1st {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P3addrIdx1st {
    #[inline(always)]
    fn from(val: u8) -> P3addrIdx1st {
        P3addrIdx1st::from_bits(val)
    }
}
impl From<P3addrIdx1st> for u8 {
    #[inline(always)]
    fn from(val: P3addrIdx1st) -> u8 {
        P3addrIdx1st::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P3cfgDirection {
    #[doc = "Endpoint or index-and-data: bidirectional (normal). Mailbox Single: unenforced. Mailbox Split or Mailbox Shared: bidirectional."]
    DIR0 = 0x0,
    #[doc = "Endpoint or index-and-data: ignore read. Mailbox Single: write only. Mailbox Split or Mailbox Shared: ignore read."]
    DIR1 = 0x01,
    #[doc = "Endpoint or index-and-data: ignore write. Mailbox Single: read only. Mailbox Split or Mailbox Shared: ignore write."]
    DIR2 = 0x02,
    #[doc = "Endpoint or index-and-data: Ignore both. Mailbox Single: ignore both. Mailbox Split or Mailbox Shared: ignore both."]
    DIR3 = 0x03,
}
impl P3cfgDirection {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P3cfgDirection {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P3cfgDirection {
    #[inline(always)]
    fn from(val: u8) -> P3cfgDirection {
        P3cfgDirection::from_bits(val)
    }
}
impl From<P3cfgDirection> for u8 {
    #[inline(always)]
    fn from(val: P3cfgDirection) -> u8 {
        P3cfgDirection::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P3cfgErrorign {
    #[doc = "The host receives an error when trying to perform a read or write that is blocked by PnCFG\\[DIRECTION\\]."]
    ERRORIGIN0 = 0x0,
    #[doc = "Ignored silently. Reads return FFh for each byte if ignored (host property). In either case, PnSTAT\\[INTERR\\] bit becomes 1, causing an interrupt if masked for it."]
    ERRORIGIN1 = 0x01,
}
impl P3cfgErrorign {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P3cfgErrorign {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P3cfgErrorign {
    #[inline(always)]
    fn from(val: u8) -> P3cfgErrorign {
        P3cfgErrorign::from_bits(val)
    }
}
impl From<P3cfgErrorign> for u8 {
    #[inline(always)]
    fn from(val: P3cfgErrorign) -> u8 {
        P3cfgErrorign::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P3cfgType {
    #[doc = "Unconfigured (reset condition)"]
    UNCONFIGURED = 0x0,
    #[doc = "ACPI style Endpoint"]
    ACPI_END = 0x01,
    #[doc = "ACPI style index-and-data. Index and data byte locations. Index gives offset into implied space. Uses registers for data and index."]
    ACPI_INDEX = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Bus Master Memory Single"]
    BUS_M_MEM_S = 0x04,
    #[doc = "Bus Master Flash Single"]
    BUS_M_FLASH_S = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Mailbox Shared"]
    MAILBOX_SHARED = 0x08,
    #[doc = "Mailbox Single"]
    MAILBOX_SINGLE = 0x09,
    #[doc = "Mailbox Split"]
    MAILBOX_SPLIT = 0x0a,
    #[doc = "Mailbox OOB Split"]
    MAILBOX_OOB_SPLIT = 0x0b,
    #[doc = "Mailbox OEM"]
    MAILBOX_OEM = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl P3cfgType {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P3cfgType {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P3cfgType {
    #[inline(always)]
    fn from(val: u8) -> P3cfgType {
        P3cfgType::from_bits(val)
    }
}
impl From<P3cfgType> for u8 {
    #[inline(always)]
    fn from(val: P3cfgType) -> u8 {
        P3cfgType::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P3datainDir {
    #[doc = "Read by host"]
    ENABLE = 0x0,
    #[doc = "Write by host"]
    DISABLE = 0x01,
}
impl P3datainDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P3datainDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P3datainDir {
    #[inline(always)]
    fn from(val: u8) -> P3datainDir {
        P3datainDir::from_bits(val)
    }
}
impl From<P3datainDir> for u8 {
    #[inline(always)]
    fn from(val: P3datainDir) -> u8 {
        P3datainDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P3irulestatFlashCompletionType {
    #[doc = "Indicates the middle completion of a split completion sequence"]
    MIDDLE = 0x0,
    #[doc = "Indicates the first completion of a split completion sequence"]
    FIRST = 0x01,
    #[doc = "Indicates the last completion of a split completion sequence"]
    LAST = 0x02,
    #[doc = "Indicates the only completion for a split transaction."]
    ONLY = 0x03,
}
impl P3irulestatFlashCompletionType {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P3irulestatFlashCompletionType {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P3irulestatFlashCompletionType {
    #[inline(always)]
    fn from(val: u8) -> P3irulestatFlashCompletionType {
        P3irulestatFlashCompletionType::from_bits(val)
    }
}
impl From<P3irulestatFlashCompletionType> for u8 {
    #[inline(always)]
    fn from(val: P3irulestatFlashCompletionType) -> u8 {
        P3irulestatFlashCompletionType::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P3omflenTrans {
    #[doc = "OOB: to host; Master: to host 32 (host reads with 32-bit address); SAF: Completion fail"]
    OOB = 0x0,
    #[doc = "Master: to host 64 (host reads w/64-bit address); MAF: read flash (location in RAM); SAF: completion with data"]
    READ = 0x01,
    #[doc = "Master: from host 32 (host writes w/32-bit address); MAF: write flash (location in RAM); SAF: completion with no data"]
    WRITE = 0x02,
    #[doc = "Master: from host 64 (host writes w/64-bit address); MAF: erase flash (sector in RAM)"]
    ERASE = 0x03,
}
impl P3omflenTrans {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P3omflenTrans {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P3omflenTrans {
    #[inline(always)]
    fn from(val: u8) -> P3omflenTrans {
        P3omflenTrans::from_bits(val)
    }
}
impl From<P3omflenTrans> for u8 {
    #[inline(always)]
    fn from(val: P3omflenTrans) -> u8 {
        P3omflenTrans::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P3statRdstat {
    #[doc = "Endpoint or index-and-data: Empty. Mailbox: Empty (host read to end). Master/Flash: No requests."]
    RDSTAT0 = 0x0,
    #[doc = "Endpoint or index-and-data: Data waiting from MCU. Mailbox: Started (by MCU). Master/Flash: Started. From-host: goes to Pending next. To-host: goes to Complete next. SAF: host made request: Started."]
    RDSTAT1 = 0x01,
    #[doc = "Mailbox: Complete (by MCU). Master/Flash: Complete. SAF: Complete."]
    RDSTAT2 = 0x02,
    #[doc = "Mailbox: Partially read (by host). Master/Flash: From-host only, Pending (request made); goes to Complete next. SAF: MCU has setup completion."]
    RDSTAT3 = 0x03,
}
impl P3statRdstat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P3statRdstat {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P3statRdstat {
    #[inline(always)]
    fn from(val: u8) -> P3statRdstat {
        P3statRdstat::from_bits(val)
    }
}
impl From<P3statRdstat> for u8 {
    #[inline(always)]
    fn from(val: P3statRdstat) -> u8 {
        P3statRdstat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P3statRpmc1Or2 {
    #[doc = "RPMC operation 1 has been chosen"]
    OPERATION_0 = 0x0,
    #[doc = "RPMC operation 2 has been chosen"]
    OPERATION_1 = 0x01,
}
impl P3statRpmc1Or2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P3statRpmc1Or2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P3statRpmc1Or2 {
    #[inline(always)]
    fn from(val: u8) -> P3statRpmc1Or2 {
        P3statRpmc1Or2::from_bits(val)
    }
}
impl From<P3statRpmc1Or2> for u8 {
    #[inline(always)]
    fn from(val: P3statRpmc1Or2) -> u8 {
        P3statRpmc1Or2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P3statRpmcFlashDevice {
    #[doc = "First device"]
    FIRST = 0x0,
    #[doc = "Second device"]
    SECOND = 0x01,
    #[doc = "Third device"]
    THIRD = 0x02,
    _RESERVED_3 = 0x03,
}
impl P3statRpmcFlashDevice {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P3statRpmcFlashDevice {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P3statRpmcFlashDevice {
    #[inline(always)]
    fn from(val: u8) -> P3statRpmcFlashDevice {
        P3statRpmcFlashDevice::from_bits(val)
    }
}
impl From<P3statRpmcFlashDevice> for u8 {
    #[inline(always)]
    fn from(val: P3statRpmcFlashDevice) -> u8 {
        P3statRpmcFlashDevice::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P3statWrstat {
    #[doc = "Endpoint: Empty. Index-and-data: Empty. Mailbox: Empty. Master/flash: Not last request."]
    WRSTAT0 = 0x0,
    #[doc = "Endpoint: data waiting from host. Index-and-data: data waiting from host (index may have been written before). Mailbox: Started (by host). Master/flash: last request from-host/read-flash (writes to MCU). SAF: Request from master was or is Flash Read."]
    WRSTAT1 = 0x01,
    #[doc = "Endpoint: Empty, but last was CMD. Index-and-data: wrote index. Mailbox: complete/last (host to end). Master/flash: last request to-host/write-flash (read from MCU). SAF: Request from Master was or is Flash Write."]
    WRSTAT2 = 0x02,
    #[doc = "Endpoint: CMD waiting from host. Index-and-data: Wrote Data then Index. Mailbox: Partially read (by MCU). Flash: Last Requested Erase. SAF: Request from Master was or is Flash Erase."]
    WRSTAT3 = 0x03,
}
impl P3statWrstat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P3statWrstat {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P3statWrstat {
    #[inline(always)]
    fn from(val: u8) -> P3statWrstat {
        P3statWrstat::from_bits(val)
    }
}
impl From<P3statWrstat> for u8 {
    #[inline(always)]
    fn from(val: P3statWrstat) -> u8 {
        P3statWrstat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P4addrBaseAsz {
    #[doc = "Offset from 0 in host memory or I/O space."]
    OFFSET_FROM_0 = 0x0,
    #[doc = "Uses BASE0 offset in host memory"]
    USE_BASE0 = 0x01,
    #[doc = "Uses BASE1 offset in host memory"]
    USE_BASE1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl P4addrBaseAsz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P4addrBaseAsz {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P4addrBaseAsz {
    #[inline(always)]
    fn from(val: u8) -> P4addrBaseAsz {
        P4addrBaseAsz::from_bits(val)
    }
}
impl From<P4addrBaseAsz> for u8 {
    #[inline(always)]
    fn from(val: P4addrBaseAsz) -> u8 {
        P4addrBaseAsz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P4addrIdx1st {
    #[doc = "Index is higher address than data (for example, data at OFF, index at OFF+IDXOFF)"]
    DATAOFF = 0x0,
    #[doc = "Index is lower address than data (for example, index at OFF, data at OFF+IDXOFF)"]
    IDXOFF = 0x01,
}
impl P4addrIdx1st {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P4addrIdx1st {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P4addrIdx1st {
    #[inline(always)]
    fn from(val: u8) -> P4addrIdx1st {
        P4addrIdx1st::from_bits(val)
    }
}
impl From<P4addrIdx1st> for u8 {
    #[inline(always)]
    fn from(val: P4addrIdx1st) -> u8 {
        P4addrIdx1st::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P4cfgDirection {
    #[doc = "Endpoint or index-and-data: bidirectional (normal). Mailbox Single: unenforced. Mailbox Split or Mailbox Shared: bidirectional."]
    DIR0 = 0x0,
    #[doc = "Endpoint or index-and-data: ignore read. Mailbox Single: write only. Mailbox Split or Mailbox Shared: ignore read."]
    DIR1 = 0x01,
    #[doc = "Endpoint or index-and-data: ignore write. Mailbox Single: read only. Mailbox Split or Mailbox Shared: ignore write."]
    DIR2 = 0x02,
    #[doc = "Endpoint or index-and-data: Ignore both. Mailbox Single: ignore both. Mailbox Split or Mailbox Shared: ignore both."]
    DIR3 = 0x03,
}
impl P4cfgDirection {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P4cfgDirection {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P4cfgDirection {
    #[inline(always)]
    fn from(val: u8) -> P4cfgDirection {
        P4cfgDirection::from_bits(val)
    }
}
impl From<P4cfgDirection> for u8 {
    #[inline(always)]
    fn from(val: P4cfgDirection) -> u8 {
        P4cfgDirection::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P4cfgErrorign {
    #[doc = "The host receives an error when trying to perform a read or write that is blocked by PnCFG\\[DIRECTION\\]."]
    ERRORIGIN0 = 0x0,
    #[doc = "Ignored silently. Reads return FFh for each byte if ignored (host property). In either case, PnSTAT\\[INTERR\\] bit becomes 1, causing an interrupt if masked for it."]
    ERRORIGIN1 = 0x01,
}
impl P4cfgErrorign {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P4cfgErrorign {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P4cfgErrorign {
    #[inline(always)]
    fn from(val: u8) -> P4cfgErrorign {
        P4cfgErrorign::from_bits(val)
    }
}
impl From<P4cfgErrorign> for u8 {
    #[inline(always)]
    fn from(val: P4cfgErrorign) -> u8 {
        P4cfgErrorign::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P4cfgType {
    #[doc = "Unconfigured (reset condition)"]
    UNCONFIGURED = 0x0,
    #[doc = "ACPI style Endpoint"]
    ACPI_END = 0x01,
    #[doc = "ACPI style index-and-data. Index and data byte locations. Index gives offset into implied space. Uses registers for data and index."]
    ACPI_INDEX = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Bus Master Memory Single"]
    BUS_M_MEM_S = 0x04,
    #[doc = "Bus Master Flash Single"]
    BUS_M_FLASH_S = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Mailbox Shared"]
    MAILBOX_SHARED = 0x08,
    #[doc = "Mailbox Single"]
    MAILBOX_SINGLE = 0x09,
    #[doc = "Mailbox Split"]
    MAILBOX_SPLIT = 0x0a,
    #[doc = "Mailbox OOB Split"]
    MAILBOX_OOB_SPLIT = 0x0b,
    #[doc = "Mailbox OEM"]
    MAILBOX_OEM = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl P4cfgType {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P4cfgType {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P4cfgType {
    #[inline(always)]
    fn from(val: u8) -> P4cfgType {
        P4cfgType::from_bits(val)
    }
}
impl From<P4cfgType> for u8 {
    #[inline(always)]
    fn from(val: P4cfgType) -> u8 {
        P4cfgType::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P4datainDir {
    #[doc = "Read by host"]
    ENABLE = 0x0,
    #[doc = "Write by host"]
    DISABLE = 0x01,
}
impl P4datainDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P4datainDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P4datainDir {
    #[inline(always)]
    fn from(val: u8) -> P4datainDir {
        P4datainDir::from_bits(val)
    }
}
impl From<P4datainDir> for u8 {
    #[inline(always)]
    fn from(val: P4datainDir) -> u8 {
        P4datainDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P4irulestatFlashCompletionType {
    #[doc = "Indicates the middle completion of a split completion sequence"]
    MIDDLE = 0x0,
    #[doc = "Indicates the first completion of a split completion sequence"]
    FIRST = 0x01,
    #[doc = "Indicates the last completion of a split completion sequence"]
    LAST = 0x02,
    #[doc = "Indicates the only completion for a split transaction."]
    ONLY = 0x03,
}
impl P4irulestatFlashCompletionType {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P4irulestatFlashCompletionType {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P4irulestatFlashCompletionType {
    #[inline(always)]
    fn from(val: u8) -> P4irulestatFlashCompletionType {
        P4irulestatFlashCompletionType::from_bits(val)
    }
}
impl From<P4irulestatFlashCompletionType> for u8 {
    #[inline(always)]
    fn from(val: P4irulestatFlashCompletionType) -> u8 {
        P4irulestatFlashCompletionType::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P4omflenTrans {
    #[doc = "OOB: to host; Master: to host 32 (host reads with 32-bit address); SAF: Completion fail"]
    OOB = 0x0,
    #[doc = "Master: to host 64 (host reads w/64-bit address); MAF: read flash (location in RAM); SAF: completion with data"]
    READ = 0x01,
    #[doc = "Master: from host 32 (host writes w/32-bit address); MAF: write flash (location in RAM); SAF: completion with no data"]
    WRITE = 0x02,
    #[doc = "Master: from host 64 (host writes w/64-bit address); MAF: erase flash (sector in RAM)"]
    ERASE = 0x03,
}
impl P4omflenTrans {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P4omflenTrans {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P4omflenTrans {
    #[inline(always)]
    fn from(val: u8) -> P4omflenTrans {
        P4omflenTrans::from_bits(val)
    }
}
impl From<P4omflenTrans> for u8 {
    #[inline(always)]
    fn from(val: P4omflenTrans) -> u8 {
        P4omflenTrans::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P4statRdstat {
    #[doc = "Endpoint or index-and-data: Empty. Mailbox: Empty (host read to end). Master/Flash: No requests."]
    RDSTAT0 = 0x0,
    #[doc = "Endpoint or index-and-data: Data waiting from MCU. Mailbox: Started (by MCU). Master/Flash: Started. From-host: goes to Pending next. To-host: goes to Complete next. SAF: host made request: Started."]
    RDSTAT1 = 0x01,
    #[doc = "Mailbox: Complete (by MCU). Master/Flash: Complete. SAF: Complete."]
    RDSTAT2 = 0x02,
    #[doc = "Mailbox: Partially read (by host). Master/Flash: From-host only, Pending (request made); goes to Complete next. SAF: MCU has setup completion."]
    RDSTAT3 = 0x03,
}
impl P4statRdstat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P4statRdstat {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P4statRdstat {
    #[inline(always)]
    fn from(val: u8) -> P4statRdstat {
        P4statRdstat::from_bits(val)
    }
}
impl From<P4statRdstat> for u8 {
    #[inline(always)]
    fn from(val: P4statRdstat) -> u8 {
        P4statRdstat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P4statRpmc1Or2 {
    #[doc = "RPMC operation 1 has been chosen"]
    OPERATION_0 = 0x0,
    #[doc = "RPMC operation 2 has been chosen"]
    OPERATION_1 = 0x01,
}
impl P4statRpmc1Or2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P4statRpmc1Or2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P4statRpmc1Or2 {
    #[inline(always)]
    fn from(val: u8) -> P4statRpmc1Or2 {
        P4statRpmc1Or2::from_bits(val)
    }
}
impl From<P4statRpmc1Or2> for u8 {
    #[inline(always)]
    fn from(val: P4statRpmc1Or2) -> u8 {
        P4statRpmc1Or2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P4statRpmcFlashDevice {
    #[doc = "First device"]
    FIRST = 0x0,
    #[doc = "Second device"]
    SECOND = 0x01,
    #[doc = "Third device"]
    THIRD = 0x02,
    _RESERVED_3 = 0x03,
}
impl P4statRpmcFlashDevice {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P4statRpmcFlashDevice {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P4statRpmcFlashDevice {
    #[inline(always)]
    fn from(val: u8) -> P4statRpmcFlashDevice {
        P4statRpmcFlashDevice::from_bits(val)
    }
}
impl From<P4statRpmcFlashDevice> for u8 {
    #[inline(always)]
    fn from(val: P4statRpmcFlashDevice) -> u8 {
        P4statRpmcFlashDevice::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum P4statWrstat {
    #[doc = "Endpoint: Empty. Index-and-data: Empty. Mailbox: Empty. Master/flash: Not last request."]
    WRSTAT0 = 0x0,
    #[doc = "Endpoint: data waiting from host. Index-and-data: data waiting from host (index may have been written before). Mailbox: Started (by host). Master/flash: last request from-host/read-flash (writes to MCU). SAF: Request from master was or is Flash Read."]
    WRSTAT1 = 0x01,
    #[doc = "Endpoint: Empty, but last was CMD. Index-and-data: wrote index. Mailbox: complete/last (host to end). Master/flash: last request to-host/write-flash (read from MCU). SAF: Request from Master was or is Flash Write."]
    WRSTAT2 = 0x02,
    #[doc = "Endpoint: CMD waiting from host. Index-and-data: Wrote Data then Index. Mailbox: Partially read (by MCU). Flash: Last Requested Erase. SAF: Request from Master was or is Flash Erase."]
    WRSTAT3 = 0x03,
}
impl P4statWrstat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> P4statWrstat {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for P4statWrstat {
    #[inline(always)]
    fn from(val: u8) -> P4statWrstat {
        P4statWrstat::from_bits(val)
    }
}
impl From<P4statWrstat> for u8 {
    #[inline(always)]
    fn from(val: P4statWrstat) -> u8 {
        P4statWrstat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RtcIntBmc {
    #[doc = "BMC does not support an integrated RTC."]
    BMC_NOT_SUPPORT = 0x0,
    #[doc = "BMC supports an integrated RTC to which eSPI controller can forward RTC targeting IO cycles. (**ESPI slave regs: 0x8, bit 29**)."]
    BMC_SUPPORT = 0x01,
}
impl RtcIntBmc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RtcIntBmc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RtcIntBmc {
    #[inline(always)]
    fn from(val: u8) -> RtcIntBmc {
        RtcIntBmc::from_bits(val)
    }
}
impl From<RtcIntBmc> for u8 {
    #[inline(always)]
    fn from(val: RtcIntBmc) -> u8 {
        RtcIntBmc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Safera {
    #[doc = "2 kB"]
    MIN_2KB = 0x0,
    #[doc = "4 kB"]
    MIN_4KB = 0x01,
    #[doc = "8 kB"]
    MIN_8KB = 0x02,
    #[doc = "16 kB"]
    MIN_16KB = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Safera {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Safera {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Safera {
    #[inline(always)]
    fn from(val: u8) -> Safera {
        Safera::from_bits(val)
    }
}
impl From<Safera> for u8 {
    #[inline(always)]
    fn from(val: Safera) -> u8 {
        Safera::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spicap {
    #[doc = "SPI only"]
    SPI = 0x0,
    #[doc = "BiSPI and SPI"]
    BSPI_SPI = 0x01,
    #[doc = "FLEXSPI and SPI"]
    FLEXSPI_SPI = 0x02,
    #[doc = "Any"]
    ANY = 0x03,
}
impl Spicap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spicap {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spicap {
    #[inline(always)]
    fn from(val: u8) -> Spicap {
        Spicap::from_bits(val)
    }
}
impl From<Spicap> for u8 {
    #[inline(always)]
    fn from(val: Spicap) -> u8 {
        Spicap::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spimod {
    #[doc = "SPI"]
    SPI = 0x0,
    #[doc = "BiSPI"]
    BSPI = 0x01,
    #[doc = "FLEXSPI"]
    FLEXSPI = 0x02,
    _RESERVED_3 = 0x03,
}
impl Spimod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spimod {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spimod {
    #[inline(always)]
    fn from(val: u8) -> Spimod {
        Spimod::from_bits(val)
    }
}
impl From<Spimod> for u8 {
    #[inline(always)]
    fn from(val: Spimod) -> u8 {
        Spimod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spispd {
    #[doc = "20 MHz or less"]
    LESS_AND_20M = 0x0,
    #[doc = "25 MHz or 24 MHz"]
    FREQ_25M_24M = 0x01,
    #[doc = "33 MHz or 30 MHz"]
    FREQ_33M_30M = 0x02,
    #[doc = "50 MHz or 48 MHz"]
    FREQ_50M_48M = 0x03,
    #[doc = "66 MHz or 60 MHz"]
    FREQ_66M_60M = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Spispd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spispd {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spispd {
    #[inline(always)]
    fn from(val: u8) -> Spispd {
        Spispd::from_bits(val)
    }
}
impl From<Spispd> for u8 {
    #[inline(always)]
    fn from(val: Spispd) -> u8 {
        Spispd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TargetRpmcSupported {
    #[doc = "Target does not support Replay Protected Monotonic counter"]
    MIN_2KB = 0x0,
    #[doc = "Target supports up to 1 Replay Protected Monotonic counter"]
    MIN_4KB = 0x01,
    #[doc = "Target supports up to 2 Replay Protected Monotonic counters"]
    MIN_8KB = 0x02,
    #[doc = "Target supports up to 63 Replay Protected Monotonic counters. The value of this field is the total sum of Replay Protected Monotonic counters supported by all RPMC flash devices behind the target. If RPMC is not supported by the target, this field must indicate a value of 0h."]
    MIN_16KB = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl TargetRpmcSupported {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TargetRpmcSupported {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TargetRpmcSupported {
    #[inline(always)]
    fn from(val: u8) -> TargetRpmcSupported {
        TargetRpmcSupported::from_bits(val)
    }
}
impl From<TargetRpmcSupported> for u8 {
    #[inline(always)]
    fn from(val: TargetRpmcSupported) -> u8 {
        TargetRpmcSupported::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrgtReqSizeSupp {
    _RESERVED_0 = 0x0,
    #[doc = "64 bytes max read request size"]
    SIXTY_FOUR = 0x01,
    #[doc = "128 bytes max read request size"]
    BYTESDDDD = 0x02,
    #[doc = "256 bytes max read request size"]
    BYTESSSSSS = 0x03,
    #[doc = "512 bytes max read request size"]
    BYTES = 0x04,
    #[doc = "1024 bytes max read request size"]
    BYTESS = 0x05,
    #[doc = "2048 bytes max read request size"]
    BYTESSSS = 0x06,
    #[doc = "4096 bytes max read request size"]
    BYTESSSSSTTTS = 0x07,
}
impl TrgtReqSizeSupp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrgtReqSizeSupp {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrgtReqSizeSupp {
    #[inline(always)]
    fn from(val: u8) -> TrgtReqSizeSupp {
        TrgtReqSizeSupp::from_bits(val)
    }
}
impl From<TrgtReqSizeSupp> for u8 {
    #[inline(always)]
    fn from(val: TrgtReqSizeSupp) -> u8 {
        TrgtReqSizeSupp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WireinGpioLevel {
    #[doc = "Low"]
    DISABLE = 0x0,
    #[doc = "High"]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl WireinGpioLevel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WireinGpioLevel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WireinGpioLevel {
    #[inline(always)]
    fn from(val: u8) -> WireinGpioLevel {
        WireinGpioLevel::from_bits(val)
    }
}
impl From<WireinGpioLevel> for u8 {
    #[inline(always)]
    fn from(val: WireinGpioLevel) -> u8 {
        WireinGpioLevel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WireinGpioValid {
    #[doc = "Not valid"]
    NOT = 0x0,
    #[doc = "Valid"]
    VALID1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl WireinGpioValid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WireinGpioValid {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WireinGpioValid {
    #[inline(always)]
    fn from(val: u8) -> WireinGpioValid {
        WireinGpioValid::from_bits(val)
    }
}
impl From<WireinGpioValid> for u8 {
    #[inline(always)]
    fn from(val: WireinGpioValid) -> u8 {
        WireinGpioValid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WireoutGpioLevel {
    #[doc = "Low"]
    DISABLE = 0x0,
    #[doc = "High"]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl WireoutGpioLevel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WireoutGpioLevel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WireoutGpioLevel {
    #[inline(always)]
    fn from(val: u8) -> WireoutGpioLevel {
        WireoutGpioLevel::from_bits(val)
    }
}
impl From<WireoutGpioLevel> for u8 {
    #[inline(always)]
    fn from(val: WireoutGpioLevel) -> u8 {
        WireoutGpioLevel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WireoutGpioValid {
    #[doc = "Not valid"]
    NOT = 0x0,
    #[doc = "Valid"]
    VALID1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl WireoutGpioValid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WireoutGpioValid {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WireoutGpioValid {
    #[inline(always)]
    fn from(val: u8) -> WireoutGpioValid {
        WireoutGpioValid::from_bits(val)
    }
}
impl From<WireoutGpioValid> for u8 {
    #[inline(always)]
    fn from(val: WireoutGpioValid) -> u8 {
        WireoutGpioValid::to_bits(val)
    }
}
