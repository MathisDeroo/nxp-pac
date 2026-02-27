#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbclkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl AhbclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> AhbclkdivUnstab {
        AhbclkdivUnstab::from_bits(val)
    }
}
impl From<AhbclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: AhbclkdivUnstab) -> u8 {
        AhbclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbmatprioCoolfluxYEspi {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl AhbmatprioCoolfluxYEspi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbmatprioCoolfluxYEspi {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbmatprioCoolfluxYEspi {
    #[inline(always)]
    fn from(val: u8) -> AhbmatprioCoolfluxYEspi {
        AhbmatprioCoolfluxYEspi::from_bits(val)
    }
}
impl From<AhbmatprioCoolfluxYEspi> for u8 {
    #[inline(always)]
    fn from(val: AhbmatprioCoolfluxYEspi) -> u8 {
        AhbmatprioCoolfluxYEspi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbmatprioCpu0Sbus {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl AhbmatprioCpu0Sbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbmatprioCpu0Sbus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbmatprioCpu0Sbus {
    #[inline(always)]
    fn from(val: u8) -> AhbmatprioCpu0Sbus {
        AhbmatprioCpu0Sbus::from_bits(val)
    }
}
impl From<AhbmatprioCpu0Sbus> for u8 {
    #[inline(always)]
    fn from(val: AhbmatprioCpu0Sbus) -> u8 {
        AhbmatprioCpu0Sbus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbmatprioDma0 {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl AhbmatprioDma0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbmatprioDma0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbmatprioDma0 {
    #[inline(always)]
    fn from(val: u8) -> AhbmatprioDma0 {
        AhbmatprioDma0::from_bits(val)
    }
}
impl From<AhbmatprioDma0> for u8 {
    #[inline(always)]
    fn from(val: AhbmatprioDma0) -> u8 {
        AhbmatprioDma0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbmatprioDma1 {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl AhbmatprioDma1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbmatprioDma1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbmatprioDma1 {
    #[inline(always)]
    fn from(val: u8) -> AhbmatprioDma1 {
        AhbmatprioDma1::from_bits(val)
    }
}
impl From<AhbmatprioDma1> for u8 {
    #[inline(always)]
    fn from(val: AhbmatprioDma1) -> u8 {
        AhbmatprioDma1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbmatprioPkcEls {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl AhbmatprioPkcEls {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbmatprioPkcEls {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbmatprioPkcEls {
    #[inline(always)]
    fn from(val: u8) -> AhbmatprioPkcEls {
        AhbmatprioPkcEls::from_bits(val)
    }
}
impl From<AhbmatprioPkcEls> for u8 {
    #[inline(always)]
    fn from(val: AhbmatprioPkcEls) -> u8 {
        AhbmatprioPkcEls::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbmatprioUsbFsEnet {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl AhbmatprioUsbFsEnet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbmatprioUsbFsEnet {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbmatprioUsbFsEnet {
    #[inline(always)]
    fn from(val: u8) -> AhbmatprioUsbFsEnet {
        AhbmatprioUsbFsEnet::from_bits(val)
    }
}
impl From<AhbmatprioUsbFsEnet> for u8 {
    #[inline(always)]
    fn from(val: AhbmatprioUsbFsEnet) -> u8 {
        AhbmatprioUsbFsEnet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbmatprioUsbHs {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl AhbmatprioUsbHs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbmatprioUsbHs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbmatprioUsbHs {
    #[inline(always)]
    fn from(val: u8) -> AhbmatprioUsbHs {
        AhbmatprioUsbHs::from_bits(val)
    }
}
impl From<AhbmatprioUsbHs> for u8 {
    #[inline(always)]
    fn from(val: AhbmatprioUsbHs) -> u8 {
        AhbmatprioUsbHs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BusclkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl BusclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BusclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BusclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> BusclkdivHalt {
        BusclkdivHalt::from_bits(val)
    }
}
impl From<BusclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: BusclkdivHalt) -> u8 {
        BusclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BusclkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl BusclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BusclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BusclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> BusclkdivReset {
        BusclkdivReset::from_bits(val)
    }
}
impl From<BusclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: BusclkdivReset) -> u8 {
        BusclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BusclkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl BusclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BusclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BusclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> BusclkdivUnstab {
        BusclkdivUnstab::from_bits(val)
    }
}
impl From<BusclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: BusclkdivUnstab) -> u8 {
        BusclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClrLpcac {
    #[doc = "Unclears the cache"]
    ENABLE = 0x0,
    #[doc = "Clears the cache"]
    DISABLE = 0x01,
}
impl ClrLpcac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClrLpcac {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClrLpcac {
    #[inline(always)]
    fn from(val: u8) -> ClrLpcac {
        ClrLpcac::from_bits(val)
    }
}
impl From<ClrLpcac> for u8 {
    #[inline(always)]
    fn from(val: ClrLpcac) -> u8 {
        ClrLpcac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0Cbus {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl Cpu0Cbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0Cbus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0Cbus {
    #[inline(always)]
    fn from(val: u8) -> Cpu0Cbus {
        Cpu0Cbus::from_bits(val)
    }
}
impl From<Cpu0Cbus> for u8 {
    #[inline(always)]
    fn from(val: Cpu0Cbus) -> u8 {
        Cpu0Cbus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0lockup {
    #[doc = "CPU is not in lockup"]
    AWAKE = 0x0,
    #[doc = "CPU is in lockup"]
    SLEEPING = 0x01,
}
impl Cpu0lockup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0lockup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0lockup {
    #[inline(always)]
    fn from(val: u8) -> Cpu0lockup {
        Cpu0lockup::from_bits(val)
    }
}
impl From<Cpu0lockup> for u8 {
    #[inline(always)]
    fn from(val: Cpu0lockup) -> u8 {
        Cpu0lockup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0sleeping {
    #[doc = "CPU is not sleeping"]
    AWAKE = 0x0,
    #[doc = "CPU is sleeping"]
    SLEEPING = 0x01,
}
impl Cpu0sleeping {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0sleeping {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0sleeping {
    #[inline(always)]
    fn from(val: u8) -> Cpu0sleeping {
        Cpu0sleeping::from_bits(val)
    }
}
impl From<Cpu0sleeping> for u8 {
    #[inline(always)]
    fn from(val: Cpu0sleeping) -> u8 {
        Cpu0sleeping::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1CbusSmartDmaI {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl Cpu1CbusSmartDmaI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1CbusSmartDmaI {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1CbusSmartDmaI {
    #[inline(always)]
    fn from(val: u8) -> Cpu1CbusSmartDmaI {
        Cpu1CbusSmartDmaI::from_bits(val)
    }
}
impl From<Cpu1CbusSmartDmaI> for u8 {
    #[inline(always)]
    fn from(val: Cpu1CbusSmartDmaI) -> u8 {
        Cpu1CbusSmartDmaI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1SbusSmartDmaD {
    #[doc = "level 0"]
    LEVEL0 = 0x0,
    #[doc = "level 1"]
    LEVEL1 = 0x01,
    #[doc = "level 2"]
    LEVEL2 = 0x02,
    #[doc = "level 3"]
    LEVEL3 = 0x03,
}
impl Cpu1SbusSmartDmaD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1SbusSmartDmaD {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1SbusSmartDmaD {
    #[inline(always)]
    fn from(val: u8) -> Cpu1SbusSmartDmaD {
        Cpu1SbusSmartDmaD::from_bits(val)
    }
}
impl From<Cpu1SbusSmartDmaD> for u8 {
    #[inline(always)]
    fn from(val: Cpu1SbusSmartDmaD) -> u8 {
        Cpu1SbusSmartDmaD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DeviceTypePkg {
    #[doc = "HLQFP"]
    HLQFP = 0x0,
    #[doc = "HTQFP"]
    HTQFP = 0x01,
    #[doc = "BGA"]
    BGA = 0x02,
    #[doc = "HDQFP"]
    HDQFP = 0x03,
    #[doc = "QFN"]
    QFN = 0x04,
    #[doc = "CSP"]
    CSP = 0x05,
    #[doc = "LQFP"]
    LQFP = 0x06,
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
impl DeviceTypePkg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DeviceTypePkg {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DeviceTypePkg {
    #[inline(always)]
    fn from(val: u8) -> DeviceTypePkg {
        DeviceTypePkg::from_bits(val)
    }
}
impl From<DeviceTypePkg> for u8 {
    #[inline(always)]
    fn from(val: DeviceTypePkg) -> u8 {
        DeviceTypePkg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DeviceTypeSec {
    #[doc = "Non Secure"]
    NON_SEC = 0x0,
    #[doc = "Secure"]
    SEC = 0x01,
}
impl DeviceTypeSec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DeviceTypeSec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DeviceTypeSec {
    #[inline(always)]
    fn from(val: u8) -> DeviceTypeSec {
        DeviceTypeSec::from_bits(val)
    }
}
impl From<DeviceTypeSec> for u8 {
    #[inline(always)]
    fn from(val: DeviceTypeSec) -> u8 {
        DeviceTypeSec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisDataSpec {
    #[doc = "Enables data speculation"]
    ENABLE = 0x0,
    #[doc = "Disables data speculation"]
    DISABLE = 0x01,
}
impl DisDataSpec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisDataSpec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisDataSpec {
    #[inline(always)]
    fn from(val: u8) -> DisDataSpec {
        DisDataSpec::from_bits(val)
    }
}
impl From<DisDataSpec> for u8 {
    #[inline(always)]
    fn from(val: DisDataSpec) -> u8 {
        DisDataSpec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisFlashSpec {
    #[doc = "Enables flash speculation"]
    ENABLE = 0x0,
    #[doc = "Disables flash speculation"]
    DISABLE = 0x01,
}
impl DisFlashSpec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisFlashSpec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisFlashSpec {
    #[inline(always)]
    fn from(val: u8) -> DisFlashSpec {
        DisFlashSpec::from_bits(val)
    }
}
impl From<DisFlashSpec> for u8 {
    #[inline(always)]
    fn from(val: DisFlashSpec) -> u8 {
        DisFlashSpec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisLpcac {
    #[doc = "Enabled"]
    ENABLE = 0x0,
    #[doc = "Disabled"]
    DISABLE = 0x01,
}
impl DisLpcac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisLpcac {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisLpcac {
    #[inline(always)]
    fn from(val: u8) -> DisLpcac {
        DisLpcac::from_bits(val)
    }
}
impl From<DisLpcac> for u8 {
    #[inline(always)]
    fn from(val: DisLpcac) -> u8 {
        DisLpcac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisMbeccErrData {
    #[doc = "Enables bus error on multi-bit ECC error for data"]
    ENABLE = 0x0,
    #[doc = "Disables bus error on multi-bit ECC error for data"]
    DISABLE = 0x01,
}
impl DisMbeccErrData {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisMbeccErrData {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisMbeccErrData {
    #[inline(always)]
    fn from(val: u8) -> DisMbeccErrData {
        DisMbeccErrData::from_bits(val)
    }
}
impl From<DisMbeccErrData> for u8 {
    #[inline(always)]
    fn from(val: DisMbeccErrData) -> u8 {
        DisMbeccErrData::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisMbeccErrInst {
    #[doc = "Enables bus error on multi-bit ECC error for instruction"]
    ENABLE = 0x0,
    #[doc = "Disables bus error on multi-bit ECC error for instruction"]
    DISABLE = 0x01,
}
impl DisMbeccErrInst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisMbeccErrInst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisMbeccErrInst {
    #[inline(always)]
    fn from(val: u8) -> DisMbeccErrInst {
        DisMbeccErrInst::from_bits(val)
    }
}
impl From<DisMbeccErrInst> for u8 {
    #[inline(always)]
    fn from(val: DisMbeccErrInst) -> u8 {
        DisMbeccErrInst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlashSize {
    #[doc = "32KB."]
    SIZE_32KB = 0x0,
    #[doc = "64KB."]
    SIZE_64KB = 0x01,
    #[doc = "128KB."]
    SIZE_128KB = 0x02,
    #[doc = "256KB."]
    SIZE_256KB = 0x03,
    #[doc = "512KB."]
    SIZE_512KB = 0x04,
    #[doc = "768KB."]
    SIZE_768KB = 0x05,
    #[doc = "1MB."]
    SIZE_1MB = 0x06,
    #[doc = "1.5MB."]
    SIZE_1P5MB = 0x07,
    #[doc = "2MB."]
    SIZE_2MB = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl FlashSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashSize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashSize {
    #[inline(always)]
    fn from(val: u8) -> FlashSize {
        FlashSize::from_bits(val)
    }
}
impl From<FlashSize> for u8 {
    #[inline(always)]
    fn from(val: FlashSize) -> u8 {
        FlashSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlashStallEn {
    #[doc = "No stall on FLASH busy"]
    ENABLE = 0x0,
    #[doc = "Stall on FLASH busy"]
    DISABLE = 0x01,
}
impl FlashStallEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashStallEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashStallEn {
    #[inline(always)]
    fn from(val: u8) -> FlashStallEn {
        FlashStallEn::from_bits(val)
    }
}
impl From<FlashStallEn> for u8 {
    #[inline(always)]
    fn from(val: FlashStallEn) -> u8 {
        FlashStallEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrcNoAlloc {
    #[doc = "Forces allocation"]
    ENABLE = 0x0,
    #[doc = "Forces no allocation"]
    DISABLE = 0x01,
}
impl FrcNoAlloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrcNoAlloc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrcNoAlloc {
    #[inline(always)]
    fn from(val: u8) -> FrcNoAlloc {
        FrcNoAlloc::from_bits(val)
    }
}
impl From<FrcNoAlloc> for u8 {
    #[inline(always)]
    fn from(val: FrcNoAlloc) -> u8 {
        FrcNoAlloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrohfdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl FrohfdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrohfdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrohfdivHalt {
    #[inline(always)]
    fn from(val: u8) -> FrohfdivHalt {
        FrohfdivHalt::from_bits(val)
    }
}
impl From<FrohfdivHalt> for u8 {
    #[inline(always)]
    fn from(val: FrohfdivHalt) -> u8 {
        FrohfdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrohfdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl FrohfdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrohfdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrohfdivReset {
    #[inline(always)]
    fn from(val: u8) -> FrohfdivReset {
        FrohfdivReset::from_bits(val)
    }
}
impl From<FrohfdivReset> for u8 {
    #[inline(always)]
    fn from(val: FrohfdivReset) -> u8 {
        FrohfdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrohfdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl FrohfdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrohfdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrohfdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> FrohfdivUnstab {
        FrohfdivUnstab::from_bits(val)
    }
}
impl From<FrohfdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: FrohfdivUnstab) -> u8 {
        FrohfdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrolfdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl FrolfdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrolfdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrolfdivHalt {
    #[inline(always)]
    fn from(val: u8) -> FrolfdivHalt {
        FrolfdivHalt::from_bits(val)
    }
}
impl From<FrolfdivHalt> for u8 {
    #[inline(always)]
    fn from(val: FrolfdivHalt) -> u8 {
        FrolfdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrolfdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl FrolfdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrolfdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrolfdivReset {
    #[inline(always)]
    fn from(val: u8) -> FrolfdivReset {
        FrolfdivReset::from_bits(val)
    }
}
impl From<FrolfdivReset> for u8 {
    #[inline(always)]
    fn from(val: FrolfdivReset) -> u8 {
        FrolfdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrolfdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl FrolfdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrolfdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrolfdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> FrolfdivUnstab {
        FrolfdivUnstab::from_bits(val)
    }
}
impl From<FrolfdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: FrolfdivUnstab) -> u8 {
        FrolfdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0SclFilt {
    #[doc = "Disabled filter function when I3C0 SDA_FILT=0b0000"]
    DISABLE = 0x0,
    #[doc = "Enable Spike filter on SCL input. Non_zero value means width of Glitch on SCL line to be filtered in number of half cycles of CLK_FLT."]
    FILT_CNT = 0x01,
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
impl I3c0SclFilt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0SclFilt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0SclFilt {
    #[inline(always)]
    fn from(val: u8) -> I3c0SclFilt {
        I3c0SclFilt::from_bits(val)
    }
}
impl From<I3c0SclFilt> for u8 {
    #[inline(always)]
    fn from(val: I3c0SclFilt) -> u8 {
        I3c0SclFilt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0SdaFilt {
    #[doc = "Disabled filter function when I3C0 SCL_FILT=0b0000"]
    DISABLE = 0x0,
    #[doc = "Enable Spike filter on SDA input. Non_zero value means width of Glitch on SDA line to be filtered in number of half cycles of CLK_FLT."]
    FILT_NUM = 0x01,
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
impl I3c0SdaFilt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0SdaFilt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0SdaFilt {
    #[inline(always)]
    fn from(val: u8) -> I3c0SdaFilt {
        I3c0SdaFilt::from_bits(val)
    }
}
impl From<I3c0SdaFilt> for u8 {
    #[inline(always)]
    fn from(val: I3c0SdaFilt) -> u8 {
        I3c0SdaFilt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1SclFilt {
    #[doc = "Disabled filter function when SDA_FILT=0b0000"]
    DISABLE = 0x0,
    #[doc = "Enable Spike filter on SCL input. Non_zero value means width of Glitch on SCL line to be filtered in number of half cycles of CLK_FLT."]
    FILT_NUM = 0x01,
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
impl I3c1SclFilt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1SclFilt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1SclFilt {
    #[inline(always)]
    fn from(val: u8) -> I3c1SclFilt {
        I3c1SclFilt::from_bits(val)
    }
}
impl From<I3c1SclFilt> for u8 {
    #[inline(always)]
    fn from(val: I3c1SclFilt) -> u8 {
        I3c1SclFilt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1SdaFilt {
    #[doc = "Disabled filter function when I3C1 SCL_FILT=0b0000"]
    DISABLE = 0x0,
    #[doc = "Enable Spike filter on SDA input. Non_zero value means width of Glitch on SDA line to be filtered in number of half cycles of CLK_FLT."]
    FILT_NUM = 0x01,
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
impl I3c1SdaFilt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1SdaFilt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1SdaFilt {
    #[inline(always)]
    fn from(val: u8) -> I3c1SdaFilt {
        I3c1SdaFilt::from_bits(val)
    }
}
impl From<I3c1SdaFilt> for u8 {
    #[inline(always)]
    fn from(val: I3c1SdaFilt) -> u8 {
        I3c1SdaFilt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c2SclFilt {
    #[doc = "Disabled filter function when I3C2 SDA_FILT=0b0000"]
    DISABLE = 0x0,
    #[doc = "Enable Spike filter on SCL input. Non_zero value means width of Glitch on SCL line to be filtered in number of half cycles of CLK_FLT."]
    FILT_NUM = 0x01,
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
impl I3c2SclFilt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c2SclFilt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c2SclFilt {
    #[inline(always)]
    fn from(val: u8) -> I3c2SclFilt {
        I3c2SclFilt::from_bits(val)
    }
}
impl From<I3c2SclFilt> for u8 {
    #[inline(always)]
    fn from(val: I3c2SclFilt) -> u8 {
        I3c2SclFilt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c2SdaFilt {
    #[doc = "Disabled filter function when I3C2 SCL_FILT=0b0000"]
    DISABLE = 0x0,
    #[doc = "Enable Spike filter on SDA input. Non_zero value means width of Glitch on SDA line to be filtered in number of half cycles of CLK_FLT."]
    FILT_NUM = 0x01,
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
impl I3c2SdaFilt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c2SdaFilt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c2SdaFilt {
    #[inline(always)]
    fn from(val: u8) -> I3c2SdaFilt {
        I3c2SdaFilt::from_bits(val)
    }
}
impl From<I3c2SdaFilt> for u8 {
    #[inline(always)]
    fn from(val: I3c2SdaFilt) -> u8 {
        I3c2SdaFilt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c3SclFilt {
    #[doc = "Disabled filter function when I3C3 SDA_FILT=0b0000"]
    DISABLE = 0x0,
    #[doc = "Enable Spike filter on SCL input. Non_zero value means width of Glitch on SCL line to be filtered in number of half cycles of CLK_FLT."]
    FILT_NUM = 0x01,
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
impl I3c3SclFilt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c3SclFilt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c3SclFilt {
    #[inline(always)]
    fn from(val: u8) -> I3c3SclFilt {
        I3c3SclFilt::from_bits(val)
    }
}
impl From<I3c3SclFilt> for u8 {
    #[inline(always)]
    fn from(val: I3c3SclFilt) -> u8 {
        I3c3SclFilt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c3SdaFilt {
    #[doc = "Disabled filter function when I3C3 SCL_FILT=0b0000"]
    DISABLE = 0x0,
    #[doc = "Enable Spike filter on SDA input. Non_zero value means width of Glitch on SDA line to be filtered in number of half cycles of CLK_FLT."]
    FILT_NUM = 0x01,
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
impl I3c3SdaFilt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c3SdaFilt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c3SdaFilt {
    #[inline(always)]
    fn from(val: u8) -> I3c3SdaFilt {
        I3c3SdaFilt::from_bits(val)
    }
}
impl From<I3c3SdaFilt> for u8 {
    #[inline(always)]
    fn from(val: I3c3SdaFilt) -> u8 {
        I3c3SdaFilt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interleave {
    #[doc = "RAM access is consecutive."]
    NORMAL = 0x0,
    #[doc = "RAM access is interleaved. This setting is need for PKC L0 memory access."]
    INTERLEAVE = 0x01,
}
impl Interleave {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Interleave {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Interleave {
    #[inline(always)]
    fn from(val: u8) -> Interleave {
        Interleave::from_bits(val)
    }
}
impl From<Interleave> for u8 {
    #[inline(always)]
    fn from(val: Interleave) -> u8 {
        Interleave::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Locknsmpu {
    #[doc = "Unlock these registers. privileged access to Nonsecure MPU memory regions is allowed."]
    ENABLE = 0x0,
    #[doc = "Disable writes to the MPU_CTRL_NS, MPU_RNR_NS, MPU_RBAR_NS, MPU_RLAR_NS, MPU_RBAR_A_NSn and MPU_RLAR_A_NSn. All writes to the registers are ignored."]
    DISABLE = 0x01,
}
impl Locknsmpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Locknsmpu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Locknsmpu {
    #[inline(always)]
    fn from(val: u8) -> Locknsmpu {
        Locknsmpu::from_bits(val)
    }
}
impl From<Locknsmpu> for u8 {
    #[inline(always)]
    fn from(val: Locknsmpu) -> u8 {
        Locknsmpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Noref {
    #[doc = "Reference clock is provided"]
    YES_REF = 0x0,
    #[doc = "No reference clock is provided"]
    NO_REF = 0x01,
}
impl Noref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Noref {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Noref {
    #[inline(always)]
    fn from(val: u8) -> Noref {
        Noref::from_bits(val)
    }
}
impl From<Noref> for u8 {
    #[inline(always)]
    fn from(val: Noref) -> u8 {
        Noref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhyIntf {
    #[doc = "Selects MII PHY Interface"]
    MII = 0x0,
    #[doc = "Selects RMII PHY Interface"]
    RMII = 0x01,
}
impl PhyIntf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhyIntf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhyIntf {
    #[inline(always)]
    fn from(val: u8) -> PhyIntf {
        PhyIntf::from_bits(val)
    }
}
impl From<PhyIntf> for u8 {
    #[inline(always)]
    fn from(val: PhyIntf) -> u8 {
        PhyIntf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhySel {
    #[doc = "Selects external PHY"]
    PHY = 0x0,
    #[doc = "Selects on-chip 10BASE-T1S"]
    T1S = 0x01,
}
impl PhySel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhySel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhySel {
    #[inline(always)]
    fn from(val: u8) -> PhySel {
        PhySel::from_bits(val)
    }
}
impl From<PhySel> for u8 {
    #[inline(always)]
    fn from(val: PhySel) -> u8 {
        PhySel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl Pll1clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Pll1clkdivHalt {
        Pll1clkdivHalt::from_bits(val)
    }
}
impl From<Pll1clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Pll1clkdivHalt) -> u8 {
        Pll1clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl Pll1clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Pll1clkdivReset {
        Pll1clkdivReset::from_bits(val)
    }
}
impl From<Pll1clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Pll1clkdivReset) -> u8 {
        Pll1clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl Pll1clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Pll1clkdivUnstab {
        Pll1clkdivUnstab::from_bits(val)
    }
}
impl From<Pll1clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Pll1clkdivUnstab) -> u8 {
        Pll1clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProtlvlLock {
    #[doc = "This register is not locked and can be altered."]
    ENABLE = 0x0,
    #[doc = "This register is locked and cannot be altered until a system reset."]
    DISABLE = 0x01,
}
impl ProtlvlLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ProtlvlLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ProtlvlLock {
    #[inline(always)]
    fn from(val: u8) -> ProtlvlLock {
        ProtlvlLock::from_bits(val)
    }
}
impl From<ProtlvlLock> for u8 {
    #[inline(always)]
    fn from(val: ProtlvlLock) -> u8 {
        ProtlvlLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamSize {
    #[doc = "8KB."]
    SIZE_8KB = 0x0,
    #[doc = "16KB."]
    SIZE_16KB = 0x01,
    #[doc = "32KB."]
    SIZE_32KB = 0x02,
    #[doc = "64KB."]
    SIZE_64KB = 0x03,
    #[doc = "96KB."]
    SIZE_96KB = 0x04,
    #[doc = "128KB."]
    SIZE_128KB = 0x05,
    #[doc = "160KB."]
    SIZE_160KB = 0x06,
    #[doc = "192KB."]
    SIZE_192KB = 0x07,
    #[doc = "256KB."]
    SIZE_256KB = 0x08,
    #[doc = "288KB."]
    SIZE_288KB = 0x09,
    #[doc = "352KB."]
    SIZE_352KB = 0x0a,
    #[doc = "512KB."]
    SIZE_512KB = 0x0b,
    #[doc = "640KB."]
    SIZE_640KB = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl RamSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamSize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamSize {
    #[inline(always)]
    fn from(val: u8) -> RamSize {
        RamSize::from_bits(val)
    }
}
impl From<RamSize> for u8 {
    #[inline(always)]
    fn from(val: RamSize) -> u8 {
        RamSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RemapCoolfluxYEspi {
    #[doc = "RAMX0: alias space is disabled."]
    DISABLE = 0x0,
    #[doc = "RAMX0: same alias space as CPU0_SBUS"]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl RemapCoolfluxYEspi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RemapCoolfluxYEspi {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RemapCoolfluxYEspi {
    #[inline(always)]
    fn from(val: u8) -> RemapCoolfluxYEspi {
        RemapCoolfluxYEspi::from_bits(val)
    }
}
impl From<RemapCoolfluxYEspi> for u8 {
    #[inline(always)]
    fn from(val: RemapCoolfluxYEspi) -> u8 {
        RemapCoolfluxYEspi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RemapCpu0Sbus {
    #[doc = "RAMX0: alias space is disabled."]
    DISABLE = 0x0,
    #[doc = "RAMX0: alias space is enabled. It's linear address space from bottom of system ram. The start address is 0x20000000 + (system ram size - RAMX size)*1024."]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl RemapCpu0Sbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RemapCpu0Sbus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RemapCpu0Sbus {
    #[inline(always)]
    fn from(val: u8) -> RemapCpu0Sbus {
        RemapCpu0Sbus::from_bits(val)
    }
}
impl From<RemapCpu0Sbus> for u8 {
    #[inline(always)]
    fn from(val: RemapCpu0Sbus) -> u8 {
        RemapCpu0Sbus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RemapDma0 {
    #[doc = "RAMX0: alias space is disabled."]
    DISABLE = 0x0,
    #[doc = "RAMX0: same alias space as CPU0_SBUS"]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl RemapDma0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RemapDma0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RemapDma0 {
    #[inline(always)]
    fn from(val: u8) -> RemapDma0 {
        RemapDma0::from_bits(val)
    }
}
impl From<RemapDma0> for u8 {
    #[inline(always)]
    fn from(val: RemapDma0) -> u8 {
        RemapDma0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RemapDma1 {
    #[doc = "RAMX0: alias space is disabled."]
    DISABLE = 0x0,
    #[doc = "RAMX0: same alias space as CPU0_SBUS"]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl RemapDma1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RemapDma1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RemapDma1 {
    #[inline(always)]
    fn from(val: u8) -> RemapDma1 {
        RemapDma1::from_bits(val)
    }
}
impl From<RemapDma1> for u8 {
    #[inline(always)]
    fn from(val: RemapDma1) -> u8 {
        RemapDma1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RemapLock {
    #[doc = "This register is not locked and can be altered."]
    ENABLE = 0x0,
    #[doc = "This register is locked and cannot be altered until a system reset."]
    DISABLE = 0x01,
}
impl RemapLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RemapLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RemapLock {
    #[inline(always)]
    fn from(val: u8) -> RemapLock {
        RemapLock::from_bits(val)
    }
}
impl From<RemapLock> for u8 {
    #[inline(always)]
    fn from(val: RemapLock) -> u8 {
        RemapLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RemapPkcEls {
    #[doc = "RAMX0: alias space is disabled."]
    DISABLE = 0x0,
    #[doc = "RAMX0: same alias space as CPU0_SBUS"]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl RemapPkcEls {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RemapPkcEls {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RemapPkcEls {
    #[inline(always)]
    fn from(val: u8) -> RemapPkcEls {
        RemapPkcEls::from_bits(val)
    }
}
impl From<RemapPkcEls> for u8 {
    #[inline(always)]
    fn from(val: RemapPkcEls) -> u8 {
        RemapPkcEls::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RemapUsbFsEnet {
    #[doc = "RAMX0: alias space is disabled."]
    DISABLE = 0x0,
    #[doc = "RAMX0: same alias space as CPU0_SBUS"]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl RemapUsbFsEnet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RemapUsbFsEnet {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RemapUsbFsEnet {
    #[inline(always)]
    fn from(val: u8) -> RemapUsbFsEnet {
        RemapUsbFsEnet::from_bits(val)
    }
}
impl From<RemapUsbFsEnet> for u8 {
    #[inline(always)]
    fn from(val: RemapUsbFsEnet) -> u8 {
        RemapUsbFsEnet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RemapUsbHs {
    #[doc = "RAMX0: alias space is disabled."]
    DISABLE = 0x0,
    #[doc = "RAMX0: same alias space as CPU0_SBUS"]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl RemapUsbHs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RemapUsbHs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RemapUsbHs {
    #[inline(always)]
    fn from(val: u8) -> RemapUsbHs {
        RemapUsbHs::from_bits(val)
    }
}
impl From<RemapUsbHs> for u8 {
    #[inline(always)]
    fn from(val: RemapUsbHs) -> u8 {
        RemapUsbHs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Security {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Secure version. (All values other than 1010b represent the secure version.)"]
    SEC = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Non secure version."]
    NON_SEC = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Security {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Security {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Security {
    #[inline(always)]
    fn from(val: u8) -> Security {
        Security::from_bits(val)
    }
}
impl From<Security> for u8 {
    #[inline(always)]
    fn from(val: Security) -> u8 {
        Security::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Skew {
    #[doc = "TENMS value is exact"]
    EXACT = 0x0,
    #[doc = "TENMS value is not exact or not given"]
    INEXACT = 0x01,
}
impl Skew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Skew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Skew {
    #[inline(always)]
    fn from(val: u8) -> Skew {
        Skew::from_bits(val)
    }
}
impl From<Skew> for u8 {
    #[inline(always)]
    fn from(val: Skew) -> u8 {
        Skew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SlowclkdivHalt {
    #[doc = "Divider clock is running"]
    RUN = 0x0,
    #[doc = "Divider clock is stopped"]
    HALT = 0x01,
}
impl SlowclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SlowclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SlowclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> SlowclkdivHalt {
        SlowclkdivHalt::from_bits(val)
    }
}
impl From<SlowclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: SlowclkdivHalt) -> u8 {
        SlowclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SlowclkdivReset {
    #[doc = "Divider is not reset"]
    RELEASED = 0x0,
    #[doc = "Divider is reset"]
    ASSERTED = 0x01,
}
impl SlowclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SlowclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SlowclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> SlowclkdivReset {
        SlowclkdivReset::from_bits(val)
    }
}
impl From<SlowclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: SlowclkdivReset) -> u8 {
        SlowclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SlowclkdivUnstab {
    #[doc = "Divider clock is stable"]
    STABLE = 0x0,
    #[doc = "Clock frequency is not stable"]
    ONGOING = 0x01,
}
impl SlowclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SlowclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SlowclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> SlowclkdivUnstab {
        SlowclkdivUnstab::from_bits(val)
    }
}
impl From<SlowclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: SlowclkdivUnstab) -> u8 {
        SlowclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmartDmaD {
    #[doc = "RAMX0: alias space is disabled."]
    DISABLE = 0x0,
    #[doc = "RAMX0: same alias space as CPU0_SBUS"]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl SmartDmaD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmartDmaD {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmartDmaD {
    #[inline(always)]
    fn from(val: u8) -> SmartDmaD {
        SmartDmaD::from_bits(val)
    }
}
impl From<SmartDmaD> for u8 {
    #[inline(always)]
    fn from(val: SmartDmaD) -> u8 {
        SmartDmaD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmartDmaI {
    #[doc = "RAMX0: alias space is disabled."]
    DISABLE = 0x0,
    #[doc = "RAMX0: same alias space as CPU0_SBUS"]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl SmartDmaI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmartDmaI {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmartDmaI {
    #[inline(always)]
    fn from(val: u8) -> SmartDmaI {
        SmartDmaI::from_bits(val)
    }
}
impl From<SmartDmaI> for u8 {
    #[inline(always)]
    fn from(val: SmartDmaI) -> u8 {
        SmartDmaI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Unlock {
    #[doc = "Updates are allowed to all clock configuration registers"]
    ENABLE = 0x0,
    #[doc = "Freezes all clock configuration registers update."]
    FREEZE = 0x01,
}
impl Unlock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Unlock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Unlock {
    #[inline(always)]
    fn from(val: u8) -> Unlock {
        Unlock::from_bits(val)
    }
}
impl From<Unlock> for u8 {
    #[inline(always)]
    fn from(val: Unlock) -> u8 {
        Unlock::to_bits(val)
    }
}
