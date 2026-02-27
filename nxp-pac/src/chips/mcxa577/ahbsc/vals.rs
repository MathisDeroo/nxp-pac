#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Adc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0 {
    #[inline(always)]
    fn from(val: u8) -> Adc0 {
        Adc0::from_bits(val)
    }
}
impl From<Adc0> for u8 {
    #[inline(always)]
    fn from(val: Adc0) -> u8 {
        Adc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Adc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc1 {
    #[inline(always)]
    fn from(val: u8) -> Adc1 {
        Adc1::from_bits(val)
    }
}
impl From<Adc1> for u8 {
    #[inline(always)]
    fn from(val: Adc1) -> u8 {
        Adc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSecureCtrlMemRule0Rule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl AhbSecureCtrlMemRule0Rule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSecureCtrlMemRule0Rule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSecureCtrlMemRule0Rule0 {
    #[inline(always)]
    fn from(val: u8) -> AhbSecureCtrlMemRule0Rule0 {
        AhbSecureCtrlMemRule0Rule0::from_bits(val)
    }
}
impl From<AhbSecureCtrlMemRule0Rule0> for u8 {
    #[inline(always)]
    fn from(val: AhbSecureCtrlMemRule0Rule0) -> u8 {
        AhbSecureCtrlMemRule0Rule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSecureCtrlMemRule0Rule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl AhbSecureCtrlMemRule0Rule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSecureCtrlMemRule0Rule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSecureCtrlMemRule0Rule1 {
    #[inline(always)]
    fn from(val: u8) -> AhbSecureCtrlMemRule0Rule1 {
        AhbSecureCtrlMemRule0Rule1::from_bits(val)
    }
}
impl From<AhbSecureCtrlMemRule0Rule1> for u8 {
    #[inline(always)]
    fn from(val: AhbSecureCtrlMemRule0Rule1) -> u8 {
        AhbSecureCtrlMemRule0Rule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSecureCtrlMemRule0Rule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl AhbSecureCtrlMemRule0Rule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSecureCtrlMemRule0Rule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSecureCtrlMemRule0Rule2 {
    #[inline(always)]
    fn from(val: u8) -> AhbSecureCtrlMemRule0Rule2 {
        AhbSecureCtrlMemRule0Rule2::from_bits(val)
    }
}
impl From<AhbSecureCtrlMemRule0Rule2> for u8 {
    #[inline(always)]
    fn from(val: AhbSecureCtrlMemRule0Rule2) -> u8 {
        AhbSecureCtrlMemRule0Rule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSecureCtrlMemRule0Rule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl AhbSecureCtrlMemRule0Rule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSecureCtrlMemRule0Rule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSecureCtrlMemRule0Rule3 {
    #[inline(always)]
    fn from(val: u8) -> AhbSecureCtrlMemRule0Rule3 {
        AhbSecureCtrlMemRule0Rule3::from_bits(val)
    }
}
impl From<AhbSecureCtrlMemRule0Rule3> for u8 {
    #[inline(always)]
    fn from(val: AhbSecureCtrlMemRule0Rule3) -> u8 {
        AhbSecureCtrlMemRule0Rule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AipsBridgeGroup1MemRule1Usb1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl AipsBridgeGroup1MemRule1Usb1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AipsBridgeGroup1MemRule1Usb1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AipsBridgeGroup1MemRule1Usb1 {
    #[inline(always)]
    fn from(val: u8) -> AipsBridgeGroup1MemRule1Usb1 {
        AipsBridgeGroup1MemRule1Usb1::from_bits(val)
    }
}
impl From<AipsBridgeGroup1MemRule1Usb1> for u8 {
    #[inline(always)]
    fn from(val: AipsBridgeGroup1MemRule1Usb1) -> u8 {
        AipsBridgeGroup1MemRule1Usb1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aoi0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Aoi0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aoi0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aoi0 {
    #[inline(always)]
    fn from(val: u8) -> Aoi0 {
        Aoi0::from_bits(val)
    }
}
impl From<Aoi0> for u8 {
    #[inline(always)]
    fn from(val: Aoi0) -> u8 {
        Aoi0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApbPeripheralGroup0MemRule1Smartdma {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl ApbPeripheralGroup0MemRule1Smartdma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApbPeripheralGroup0MemRule1Smartdma {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApbPeripheralGroup0MemRule1Smartdma {
    #[inline(always)]
    fn from(val: u8) -> ApbPeripheralGroup0MemRule1Smartdma {
        ApbPeripheralGroup0MemRule1Smartdma::from_bits(val)
    }
}
impl From<ApbPeripheralGroup0MemRule1Smartdma> for u8 {
    #[inline(always)]
    fn from(val: ApbPeripheralGroup0MemRule1Smartdma) -> u8 {
        ApbPeripheralGroup0MemRule1Smartdma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can0Region0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Can0Region0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can0Region0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can0Region0 {
    #[inline(always)]
    fn from(val: u8) -> Can0Region0 {
        Can0Region0::from_bits(val)
    }
}
impl From<Can0Region0> for u8 {
    #[inline(always)]
    fn from(val: Can0Region0) -> u8 {
        Can0Region0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can0Region1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Can0Region1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can0Region1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can0Region1 {
    #[inline(always)]
    fn from(val: u8) -> Can0Region1 {
        Can0Region1::from_bits(val)
    }
}
impl From<Can0Region1> for u8 {
    #[inline(always)]
    fn from(val: Can0Region1) -> u8 {
        Can0Region1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can0Region2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Can0Region2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can0Region2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can0Region2 {
    #[inline(always)]
    fn from(val: u8) -> Can0Region2 {
        Can0Region2::from_bits(val)
    }
}
impl From<Can0Region2> for u8 {
    #[inline(always)]
    fn from(val: Can0Region2) -> u8 {
        Can0Region2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can0Region3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Can0Region3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can0Region3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can0Region3 {
    #[inline(always)]
    fn from(val: u8) -> Can0Region3 {
        Can0Region3::from_bits(val)
    }
}
impl From<Can0Region3> for u8 {
    #[inline(always)]
    fn from(val: Can0Region3) -> u8 {
        Can0Region3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can1Region0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Can1Region0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can1Region0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can1Region0 {
    #[inline(always)]
    fn from(val: u8) -> Can1Region0 {
        Can1Region0::from_bits(val)
    }
}
impl From<Can1Region0> for u8 {
    #[inline(always)]
    fn from(val: Can1Region0) -> u8 {
        Can1Region0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can1Region1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Can1Region1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can1Region1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can1Region1 {
    #[inline(always)]
    fn from(val: u8) -> Can1Region1 {
        Can1Region1::from_bits(val)
    }
}
impl From<Can1Region1> for u8 {
    #[inline(always)]
    fn from(val: Can1Region1) -> u8 {
        Can1Region1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can1Region2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Can1Region2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can1Region2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can1Region2 {
    #[inline(always)]
    fn from(val: u8) -> Can1Region2 {
        Can1Region2::from_bits(val)
    }
}
impl From<Can1Region2> for u8 {
    #[inline(always)]
    fn from(val: Can1Region2) -> u8 {
        Can1Region2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can1Region3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Can1Region3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can1Region3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can1Region3 {
    #[inline(always)]
    fn from(val: u8) -> Can1Region3 {
        Can1Region3::from_bits(val)
    }
}
impl From<Can1Region3> for u8 {
    #[inline(always)]
    fn from(val: Can1Region3) -> u8 {
        Can1Region3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cdog0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Cdog0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cdog0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cdog0 {
    #[inline(always)]
    fn from(val: u8) -> Cdog0 {
        Cdog0::from_bits(val)
    }
}
impl From<Cdog0> for u8 {
    #[inline(always)]
    fn from(val: Cdog0) -> u8 {
        Cdog0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cdog1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Cdog1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cdog1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cdog1 {
    #[inline(always)]
    fn from(val: u8) -> Cdog1 {
        Cdog1::from_bits(val)
    }
}
impl From<Cdog1> for u8 {
    #[inline(always)]
    fn from(val: Cdog1) -> u8 {
        Cdog1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmc {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Cmc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmc {
    #[inline(always)]
    fn from(val: u8) -> Cmc {
        Cmc::from_bits(val)
    }
}
impl From<Cmc> for u8 {
    #[inline(always)]
    fn from(val: Cmc) -> u8 {
        Cmc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Cmp0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp0 {
    #[inline(always)]
    fn from(val: u8) -> Cmp0 {
        Cmp0::from_bits(val)
    }
}
impl From<Cmp0> for u8 {
    #[inline(always)]
    fn from(val: Cmp0) -> u8 {
        Cmp0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crc0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Crc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crc0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crc0 {
    #[inline(always)]
    fn from(val: u8) -> Crc0 {
        Crc0::from_bits(val)
    }
}
impl From<Crc0> for u8 {
    #[inline(always)]
    fn from(val: Crc0) -> u8 {
        Crc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ctimer0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer0 {
    #[inline(always)]
    fn from(val: u8) -> Ctimer0 {
        Ctimer0::from_bits(val)
    }
}
impl From<Ctimer0> for u8 {
    #[inline(always)]
    fn from(val: Ctimer0) -> u8 {
        Ctimer0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ctimer1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer1 {
    #[inline(always)]
    fn from(val: u8) -> Ctimer1 {
        Ctimer1::from_bits(val)
    }
}
impl From<Ctimer1> for u8 {
    #[inline(always)]
    fn from(val: Ctimer1) -> u8 {
        Ctimer1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ctimer2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer2 {
    #[inline(always)]
    fn from(val: u8) -> Ctimer2 {
        Ctimer2::from_bits(val)
    }
}
impl From<Ctimer2> for u8 {
    #[inline(always)]
    fn from(val: Ctimer2) -> u8 {
        Ctimer2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ctimer3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer3 {
    #[inline(always)]
    fn from(val: u8) -> Ctimer3 {
        Ctimer3::from_bits(val)
    }
}
impl From<Ctimer3> for u8 {
    #[inline(always)]
    fn from(val: Ctimer3) -> u8 {
        Ctimer3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ctimer4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer4 {
    #[inline(always)]
    fn from(val: u8) -> Ctimer4 {
        Ctimer4::from_bits(val)
    }
}
impl From<Ctimer4> for u8 {
    #[inline(always)]
    fn from(val: Ctimer4) -> u8 {
        Ctimer4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dac0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac0 {
    #[inline(always)]
    fn from(val: u8) -> Dac0 {
        Dac0::from_bits(val)
    }
}
impl From<Dac0> for u8 {
    #[inline(always)]
    fn from(val: Dac0) -> u8 {
        Dac0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dac1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac1 {
    #[inline(always)]
    fn from(val: u8) -> Dac1 {
        Dac1::from_bits(val)
    }
}
impl From<Dac1> for u8 {
    #[inline(always)]
    fn from(val: Dac1) -> u8 {
        Dac1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugMailbox {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl DebugMailbox {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugMailbox {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugMailbox {
    #[inline(always)]
    fn from(val: u8) -> DebugMailbox {
        DebugMailbox::from_bits(val)
    }
}
impl From<DebugMailbox> for u8 {
    #[inline(always)]
    fn from(val: DebugMailbox) -> u8 {
        DebugMailbox::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dgdet0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dgdet0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dgdet0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dgdet0 {
    #[inline(always)]
    fn from(val: u8) -> Dgdet0 {
        Dgdet0::from_bits(val)
    }
}
impl From<Dgdet0> for u8 {
    #[inline(always)]
    fn from(val: Dgdet0) -> u8 {
        Dgdet0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Ch0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma0Ch0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Ch0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Ch0 {
    #[inline(always)]
    fn from(val: u8) -> Dma0Ch0 {
        Dma0Ch0::from_bits(val)
    }
}
impl From<Dma0Ch0> for u8 {
    #[inline(always)]
    fn from(val: Dma0Ch0) -> u8 {
        Dma0Ch0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Ch1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma0Ch1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Ch1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Ch1 {
    #[inline(always)]
    fn from(val: u8) -> Dma0Ch1 {
        Dma0Ch1::from_bits(val)
    }
}
impl From<Dma0Ch1> for u8 {
    #[inline(always)]
    fn from(val: Dma0Ch1) -> u8 {
        Dma0Ch1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Ch10 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma0Ch10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Ch10 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Ch10 {
    #[inline(always)]
    fn from(val: u8) -> Dma0Ch10 {
        Dma0Ch10::from_bits(val)
    }
}
impl From<Dma0Ch10> for u8 {
    #[inline(always)]
    fn from(val: Dma0Ch10) -> u8 {
        Dma0Ch10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Ch11 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma0Ch11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Ch11 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Ch11 {
    #[inline(always)]
    fn from(val: u8) -> Dma0Ch11 {
        Dma0Ch11::from_bits(val)
    }
}
impl From<Dma0Ch11> for u8 {
    #[inline(always)]
    fn from(val: Dma0Ch11) -> u8 {
        Dma0Ch11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Ch2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma0Ch2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Ch2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Ch2 {
    #[inline(always)]
    fn from(val: u8) -> Dma0Ch2 {
        Dma0Ch2::from_bits(val)
    }
}
impl From<Dma0Ch2> for u8 {
    #[inline(always)]
    fn from(val: Dma0Ch2) -> u8 {
        Dma0Ch2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Ch3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma0Ch3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Ch3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Ch3 {
    #[inline(always)]
    fn from(val: u8) -> Dma0Ch3 {
        Dma0Ch3::from_bits(val)
    }
}
impl From<Dma0Ch3> for u8 {
    #[inline(always)]
    fn from(val: Dma0Ch3) -> u8 {
        Dma0Ch3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Ch4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma0Ch4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Ch4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Ch4 {
    #[inline(always)]
    fn from(val: u8) -> Dma0Ch4 {
        Dma0Ch4::from_bits(val)
    }
}
impl From<Dma0Ch4> for u8 {
    #[inline(always)]
    fn from(val: Dma0Ch4) -> u8 {
        Dma0Ch4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Ch5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma0Ch5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Ch5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Ch5 {
    #[inline(always)]
    fn from(val: u8) -> Dma0Ch5 {
        Dma0Ch5::from_bits(val)
    }
}
impl From<Dma0Ch5> for u8 {
    #[inline(always)]
    fn from(val: Dma0Ch5) -> u8 {
        Dma0Ch5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Ch6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma0Ch6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Ch6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Ch6 {
    #[inline(always)]
    fn from(val: u8) -> Dma0Ch6 {
        Dma0Ch6::from_bits(val)
    }
}
impl From<Dma0Ch6> for u8 {
    #[inline(always)]
    fn from(val: Dma0Ch6) -> u8 {
        Dma0Ch6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Ch7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma0Ch7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Ch7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Ch7 {
    #[inline(always)]
    fn from(val: u8) -> Dma0Ch7 {
        Dma0Ch7::from_bits(val)
    }
}
impl From<Dma0Ch7> for u8 {
    #[inline(always)]
    fn from(val: Dma0Ch7) -> u8 {
        Dma0Ch7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Ch8 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma0Ch8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Ch8 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Ch8 {
    #[inline(always)]
    fn from(val: u8) -> Dma0Ch8 {
        Dma0Ch8::from_bits(val)
    }
}
impl From<Dma0Ch8> for u8 {
    #[inline(always)]
    fn from(val: Dma0Ch8) -> u8 {
        Dma0Ch8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Ch9 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma0Ch9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Ch9 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Ch9 {
    #[inline(always)]
    fn from(val: u8) -> Dma0Ch9 {
        Dma0Ch9::from_bits(val)
    }
}
impl From<Dma0Ch9> for u8 {
    #[inline(always)]
    fn from(val: Dma0Ch9) -> u8 {
        Dma0Ch9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Mp {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma0Mp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Mp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Mp {
    #[inline(always)]
    fn from(val: u8) -> Dma0Mp {
        Dma0Mp::from_bits(val)
    }
}
impl From<Dma0Mp> for u8 {
    #[inline(always)]
    fn from(val: Dma0Mp) -> u8 {
        Dma0Mp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma1Ch0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma1Ch0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma1Ch0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma1Ch0 {
    #[inline(always)]
    fn from(val: u8) -> Dma1Ch0 {
        Dma1Ch0::from_bits(val)
    }
}
impl From<Dma1Ch0> for u8 {
    #[inline(always)]
    fn from(val: Dma1Ch0) -> u8 {
        Dma1Ch0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma1Ch1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma1Ch1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma1Ch1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma1Ch1 {
    #[inline(always)]
    fn from(val: u8) -> Dma1Ch1 {
        Dma1Ch1::from_bits(val)
    }
}
impl From<Dma1Ch1> for u8 {
    #[inline(always)]
    fn from(val: Dma1Ch1) -> u8 {
        Dma1Ch1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma1Ch2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma1Ch2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma1Ch2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma1Ch2 {
    #[inline(always)]
    fn from(val: u8) -> Dma1Ch2 {
        Dma1Ch2::from_bits(val)
    }
}
impl From<Dma1Ch2> for u8 {
    #[inline(always)]
    fn from(val: Dma1Ch2) -> u8 {
        Dma1Ch2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma1Ch3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma1Ch3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma1Ch3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma1Ch3 {
    #[inline(always)]
    fn from(val: u8) -> Dma1Ch3 {
        Dma1Ch3::from_bits(val)
    }
}
impl From<Dma1Ch3> for u8 {
    #[inline(always)]
    fn from(val: Dma1Ch3) -> u8 {
        Dma1Ch3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma1Mp {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Dma1Mp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma1Mp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma1Mp {
    #[inline(always)]
    fn from(val: u8) -> Dma1Mp {
        Dma1Mp::from_bits(val)
    }
}
impl From<Dma1Mp> for u8 {
    #[inline(always)]
    fn from(val: Dma1Mp) -> u8 {
        Dma1Mp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ESpi {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl ESpi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ESpi {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ESpi {
    #[inline(always)]
    fn from(val: u8) -> ESpi {
        ESpi::from_bits(val)
    }
}
impl From<ESpi> for u8 {
    #[inline(always)]
    fn from(val: ESpi) -> u8 {
        ESpi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eim {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Eim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eim {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eim {
    #[inline(always)]
    fn from(val: u8) -> Eim {
        Eim::from_bits(val)
    }
}
impl From<Eim> for u8 {
    #[inline(always)]
    fn from(val: Eim) -> u8 {
        Eim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet00 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Enet00 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet00 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet00 {
    #[inline(always)]
    fn from(val: u8) -> Enet00 {
        Enet00::from_bits(val)
    }
}
impl From<Enet00> for u8 {
    #[inline(always)]
    fn from(val: Enet00) -> u8 {
        Enet00::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet01 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Enet01 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet01 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet01 {
    #[inline(always)]
    fn from(val: u8) -> Enet01 {
        Enet01::from_bits(val)
    }
}
impl From<Enet01> for u8 {
    #[inline(always)]
    fn from(val: Enet01) -> u8 {
        Enet01::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Erm {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Erm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Erm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Erm {
    #[inline(always)]
    fn from(val: u8) -> Erm {
        Erm::from_bits(val)
    }
}
impl From<Erm> for u8 {
    #[inline(always)]
    fn from(val: Erm) -> u8 {
        Erm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ewm0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ewm0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ewm0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ewm0 {
    #[inline(always)]
    fn from(val: u8) -> Ewm0 {
        Ewm0::from_bits(val)
    }
}
impl From<Ewm0> for u8 {
    #[inline(always)]
    fn from(val: Ewm0) -> u8 {
        Ewm0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash00MemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash00MemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash00MemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash00MemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Flash00MemRuleRule0 {
        Flash00MemRuleRule0::from_bits(val)
    }
}
impl From<Flash00MemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Flash00MemRuleRule0) -> u8 {
        Flash00MemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash00MemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash00MemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash00MemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash00MemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Flash00MemRuleRule1 {
        Flash00MemRuleRule1::from_bits(val)
    }
}
impl From<Flash00MemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Flash00MemRuleRule1) -> u8 {
        Flash00MemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash00MemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash00MemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash00MemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash00MemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Flash00MemRuleRule2 {
        Flash00MemRuleRule2::from_bits(val)
    }
}
impl From<Flash00MemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Flash00MemRuleRule2) -> u8 {
        Flash00MemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash00MemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash00MemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash00MemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash00MemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Flash00MemRuleRule3 {
        Flash00MemRuleRule3::from_bits(val)
    }
}
impl From<Flash00MemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Flash00MemRuleRule3) -> u8 {
        Flash00MemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash00MemRuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash00MemRuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash00MemRuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash00MemRuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Flash00MemRuleRule4 {
        Flash00MemRuleRule4::from_bits(val)
    }
}
impl From<Flash00MemRuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Flash00MemRuleRule4) -> u8 {
        Flash00MemRuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash00MemRuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash00MemRuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash00MemRuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash00MemRuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Flash00MemRuleRule5 {
        Flash00MemRuleRule5::from_bits(val)
    }
}
impl From<Flash00MemRuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Flash00MemRuleRule5) -> u8 {
        Flash00MemRuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash00MemRuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash00MemRuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash00MemRuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash00MemRuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Flash00MemRuleRule6 {
        Flash00MemRuleRule6::from_bits(val)
    }
}
impl From<Flash00MemRuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Flash00MemRuleRule6) -> u8 {
        Flash00MemRuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash00MemRuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash00MemRuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash00MemRuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash00MemRuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Flash00MemRuleRule7 {
        Flash00MemRuleRule7::from_bits(val)
    }
}
impl From<Flash00MemRuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Flash00MemRuleRule7) -> u8 {
        Flash00MemRuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash01MemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash01MemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash01MemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash01MemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Flash01MemRuleRule0 {
        Flash01MemRuleRule0::from_bits(val)
    }
}
impl From<Flash01MemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Flash01MemRuleRule0) -> u8 {
        Flash01MemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash01MemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash01MemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash01MemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash01MemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Flash01MemRuleRule1 {
        Flash01MemRuleRule1::from_bits(val)
    }
}
impl From<Flash01MemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Flash01MemRuleRule1) -> u8 {
        Flash01MemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash01MemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash01MemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash01MemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash01MemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Flash01MemRuleRule2 {
        Flash01MemRuleRule2::from_bits(val)
    }
}
impl From<Flash01MemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Flash01MemRuleRule2) -> u8 {
        Flash01MemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash01MemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash01MemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash01MemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash01MemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Flash01MemRuleRule3 {
        Flash01MemRuleRule3::from_bits(val)
    }
}
impl From<Flash01MemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Flash01MemRuleRule3) -> u8 {
        Flash01MemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash01MemRuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash01MemRuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash01MemRuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash01MemRuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Flash01MemRuleRule4 {
        Flash01MemRuleRule4::from_bits(val)
    }
}
impl From<Flash01MemRuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Flash01MemRuleRule4) -> u8 {
        Flash01MemRuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash01MemRuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash01MemRuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash01MemRuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash01MemRuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Flash01MemRuleRule5 {
        Flash01MemRuleRule5::from_bits(val)
    }
}
impl From<Flash01MemRuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Flash01MemRuleRule5) -> u8 {
        Flash01MemRuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash01MemRuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash01MemRuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash01MemRuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash01MemRuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Flash01MemRuleRule6 {
        Flash01MemRuleRule6::from_bits(val)
    }
}
impl From<Flash01MemRuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Flash01MemRuleRule6) -> u8 {
        Flash01MemRuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash01MemRuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash01MemRuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash01MemRuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash01MemRuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Flash01MemRuleRule7 {
        Flash01MemRuleRule7::from_bits(val)
    }
}
impl From<Flash01MemRuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Flash01MemRuleRule7) -> u8 {
        Flash01MemRuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash02MemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash02MemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash02MemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash02MemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Flash02MemRuleRule0 {
        Flash02MemRuleRule0::from_bits(val)
    }
}
impl From<Flash02MemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Flash02MemRuleRule0) -> u8 {
        Flash02MemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash02MemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash02MemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash02MemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash02MemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Flash02MemRuleRule1 {
        Flash02MemRuleRule1::from_bits(val)
    }
}
impl From<Flash02MemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Flash02MemRuleRule1) -> u8 {
        Flash02MemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash02MemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash02MemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash02MemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash02MemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Flash02MemRuleRule2 {
        Flash02MemRuleRule2::from_bits(val)
    }
}
impl From<Flash02MemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Flash02MemRuleRule2) -> u8 {
        Flash02MemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash02MemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash02MemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash02MemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash02MemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Flash02MemRuleRule3 {
        Flash02MemRuleRule3::from_bits(val)
    }
}
impl From<Flash02MemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Flash02MemRuleRule3) -> u8 {
        Flash02MemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash03MemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash03MemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash03MemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash03MemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Flash03MemRuleRule0 {
        Flash03MemRuleRule0::from_bits(val)
    }
}
impl From<Flash03MemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Flash03MemRuleRule0) -> u8 {
        Flash03MemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash03MemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash03MemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash03MemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash03MemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Flash03MemRuleRule1 {
        Flash03MemRuleRule1::from_bits(val)
    }
}
impl From<Flash03MemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Flash03MemRuleRule1) -> u8 {
        Flash03MemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash03MemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash03MemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash03MemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash03MemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Flash03MemRuleRule2 {
        Flash03MemRuleRule2::from_bits(val)
    }
}
impl From<Flash03MemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Flash03MemRuleRule2) -> u8 {
        Flash03MemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flash03MemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flash03MemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flash03MemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flash03MemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Flash03MemRuleRule3 {
        Flash03MemRuleRule3::from_bits(val)
    }
}
impl From<Flash03MemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Flash03MemRuleRule3) -> u8 {
        Flash03MemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexio {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio {
    #[inline(always)]
    fn from(val: u8) -> Flexio {
        Flexio::from_bits(val)
    }
}
impl From<Flexio> for u8 {
    #[inline(always)]
    fn from(val: Flexio) -> u8 {
        Flexio::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0 {
        Flexspi0::from_bits(val)
    }
}
impl From<Flexspi0> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0) -> u8 {
        Flexspi0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region0MemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region0MemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region0MemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region0MemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region0MemRuleRule0 {
        Flexspi0Region0MemRuleRule0::from_bits(val)
    }
}
impl From<Flexspi0Region0MemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region0MemRuleRule0) -> u8 {
        Flexspi0Region0MemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region0MemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region0MemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region0MemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region0MemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region0MemRuleRule1 {
        Flexspi0Region0MemRuleRule1::from_bits(val)
    }
}
impl From<Flexspi0Region0MemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region0MemRuleRule1) -> u8 {
        Flexspi0Region0MemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region0MemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region0MemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region0MemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region0MemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region0MemRuleRule2 {
        Flexspi0Region0MemRuleRule2::from_bits(val)
    }
}
impl From<Flexspi0Region0MemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region0MemRuleRule2) -> u8 {
        Flexspi0Region0MemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region0MemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region0MemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region0MemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region0MemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region0MemRuleRule3 {
        Flexspi0Region0MemRuleRule3::from_bits(val)
    }
}
impl From<Flexspi0Region0MemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region0MemRuleRule3) -> u8 {
        Flexspi0Region0MemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region0MemRuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region0MemRuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region0MemRuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region0MemRuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region0MemRuleRule4 {
        Flexspi0Region0MemRuleRule4::from_bits(val)
    }
}
impl From<Flexspi0Region0MemRuleRule4> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region0MemRuleRule4) -> u8 {
        Flexspi0Region0MemRuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region0MemRuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region0MemRuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region0MemRuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region0MemRuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region0MemRuleRule5 {
        Flexspi0Region0MemRuleRule5::from_bits(val)
    }
}
impl From<Flexspi0Region0MemRuleRule5> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region0MemRuleRule5) -> u8 {
        Flexspi0Region0MemRuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region0MemRuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region0MemRuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region0MemRuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region0MemRuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region0MemRuleRule6 {
        Flexspi0Region0MemRuleRule6::from_bits(val)
    }
}
impl From<Flexspi0Region0MemRuleRule6> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region0MemRuleRule6) -> u8 {
        Flexspi0Region0MemRuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0Region0MemRuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0Region0MemRuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0Region0MemRuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0Region0MemRuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0Region0MemRuleRule7 {
        Flexspi0Region0MemRuleRule7::from_bits(val)
    }
}
impl From<Flexspi0Region0MemRuleRule7> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0Region0MemRuleRule7) -> u8 {
        Flexspi0Region0MemRuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0RegionMemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0RegionMemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0RegionMemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0RegionMemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0RegionMemRuleRule0 {
        Flexspi0RegionMemRuleRule0::from_bits(val)
    }
}
impl From<Flexspi0RegionMemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0RegionMemRuleRule0) -> u8 {
        Flexspi0RegionMemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0RegionMemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0RegionMemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0RegionMemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0RegionMemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0RegionMemRuleRule1 {
        Flexspi0RegionMemRuleRule1::from_bits(val)
    }
}
impl From<Flexspi0RegionMemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0RegionMemRuleRule1) -> u8 {
        Flexspi0RegionMemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0RegionMemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0RegionMemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0RegionMemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0RegionMemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0RegionMemRuleRule2 {
        Flexspi0RegionMemRuleRule2::from_bits(val)
    }
}
impl From<Flexspi0RegionMemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0RegionMemRuleRule2) -> u8 {
        Flexspi0RegionMemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi0RegionMemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Flexspi0RegionMemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi0RegionMemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi0RegionMemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> Flexspi0RegionMemRuleRule3 {
        Flexspi0RegionMemRuleRule3::from_bits(val)
    }
}
impl From<Flexspi0RegionMemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: Flexspi0RegionMemRuleRule3) -> u8 {
        Flexspi0RegionMemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fmc {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Fmc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fmc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fmc {
    #[inline(always)]
    fn from(val: u8) -> Fmc {
        Fmc::from_bits(val)
    }
}
impl From<Fmc> for u8 {
    #[inline(always)]
    fn from(val: Fmc) -> u8 {
        Fmc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fmu {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Fmu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fmu {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fmu {
    #[inline(always)]
    fn from(val: u8) -> Fmu {
        Fmu::from_bits(val)
    }
}
impl From<Fmu> for u8 {
    #[inline(always)]
    fn from(val: Fmu) -> u8 {
        Fmu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Freqme {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Freqme {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Freqme {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Freqme {
    #[inline(always)]
    fn from(val: u8) -> Freqme {
        Freqme::from_bits(val)
    }
}
impl From<Freqme> for u8 {
    #[inline(always)]
    fn from(val: Freqme) -> u8 {
        Freqme::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Glikey0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Glikey0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Glikey0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Glikey0 {
    #[inline(always)]
    fn from(val: u8) -> Glikey0 {
        Glikey0::from_bits(val)
    }
}
impl From<Glikey0> for u8 {
    #[inline(always)]
    fn from(val: Glikey0) -> u8 {
        Glikey0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Gpio0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio0 {
    #[inline(always)]
    fn from(val: u8) -> Gpio0 {
        Gpio0::from_bits(val)
    }
}
impl From<Gpio0> for u8 {
    #[inline(always)]
    fn from(val: Gpio0) -> u8 {
        Gpio0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio0Alias {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Gpio0Alias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio0Alias {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio0Alias {
    #[inline(always)]
    fn from(val: u8) -> Gpio0Alias {
        Gpio0Alias::from_bits(val)
    }
}
impl From<Gpio0Alias> for u8 {
    #[inline(always)]
    fn from(val: Gpio0Alias) -> u8 {
        Gpio0Alias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Gpio1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio1 {
    #[inline(always)]
    fn from(val: u8) -> Gpio1 {
        Gpio1::from_bits(val)
    }
}
impl From<Gpio1> for u8 {
    #[inline(always)]
    fn from(val: Gpio1) -> u8 {
        Gpio1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio1Alias {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Gpio1Alias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio1Alias {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio1Alias {
    #[inline(always)]
    fn from(val: u8) -> Gpio1Alias {
        Gpio1Alias::from_bits(val)
    }
}
impl From<Gpio1Alias> for u8 {
    #[inline(always)]
    fn from(val: Gpio1Alias) -> u8 {
        Gpio1Alias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Gpio2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio2 {
    #[inline(always)]
    fn from(val: u8) -> Gpio2 {
        Gpio2::from_bits(val)
    }
}
impl From<Gpio2> for u8 {
    #[inline(always)]
    fn from(val: Gpio2) -> u8 {
        Gpio2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio2Alias {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Gpio2Alias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio2Alias {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio2Alias {
    #[inline(always)]
    fn from(val: u8) -> Gpio2Alias {
        Gpio2Alias::from_bits(val)
    }
}
impl From<Gpio2Alias> for u8 {
    #[inline(always)]
    fn from(val: Gpio2Alias) -> u8 {
        Gpio2Alias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Gpio3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio3 {
    #[inline(always)]
    fn from(val: u8) -> Gpio3 {
        Gpio3::from_bits(val)
    }
}
impl From<Gpio3> for u8 {
    #[inline(always)]
    fn from(val: Gpio3) -> u8 {
        Gpio3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio3Alias {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Gpio3Alias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio3Alias {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio3Alias {
    #[inline(always)]
    fn from(val: u8) -> Gpio3Alias {
        Gpio3Alias::from_bits(val)
    }
}
impl From<Gpio3Alias> for u8 {
    #[inline(always)]
    fn from(val: Gpio3Alias) -> u8 {
        Gpio3Alias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Gpio4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio4 {
    #[inline(always)]
    fn from(val: u8) -> Gpio4 {
        Gpio4::from_bits(val)
    }
}
impl From<Gpio4> for u8 {
    #[inline(always)]
    fn from(val: Gpio4) -> u8 {
        Gpio4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio4Alias {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Gpio4Alias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio4Alias {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio4Alias {
    #[inline(always)]
    fn from(val: u8) -> Gpio4Alias {
        Gpio4Alias::from_bits(val)
    }
}
impl From<Gpio4Alias> for u8 {
    #[inline(always)]
    fn from(val: Gpio4Alias) -> u8 {
        Gpio4Alias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Gpio5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio5 {
    #[inline(always)]
    fn from(val: u8) -> Gpio5 {
        Gpio5::from_bits(val)
    }
}
impl From<Gpio5> for u8 {
    #[inline(always)]
    fn from(val: Gpio5) -> u8 {
        Gpio5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpio5Alias {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Gpio5Alias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio5Alias {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio5Alias {
    #[inline(always)]
    fn from(val: u8) -> Gpio5Alias {
        Gpio5Alias::from_bits(val)
    }
}
impl From<Gpio5Alias> for u8 {
    #[inline(always)]
    fn from(val: Gpio5Alias) -> u8 {
        Gpio5Alias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl I3c0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0 {
    #[inline(always)]
    fn from(val: u8) -> I3c0 {
        I3c0::from_bits(val)
    }
}
impl From<I3c0> for u8 {
    #[inline(always)]
    fn from(val: I3c0) -> u8 {
        I3c0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl I3c1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1 {
    #[inline(always)]
    fn from(val: u8) -> I3c1 {
        I3c1::from_bits(val)
    }
}
impl From<I3c1> for u8 {
    #[inline(always)]
    fn from(val: I3c1) -> u8 {
        I3c1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl I3c2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c2 {
    #[inline(always)]
    fn from(val: u8) -> I3c2 {
        I3c2::from_bits(val)
    }
}
impl From<I3c2> for u8 {
    #[inline(always)]
    fn from(val: I3c2) -> u8 {
        I3c2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl I3c3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c3 {
    #[inline(always)]
    fn from(val: u8) -> I3c3 {
        I3c3::from_bits(val)
    }
}
impl From<I3c3> for u8 {
    #[inline(always)]
    fn from(val: I3c3) -> u8 {
        I3c3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Inputmux {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Inputmux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inputmux {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inputmux {
    #[inline(always)]
    fn from(val: u8) -> Inputmux {
        Inputmux::from_bits(val)
    }
}
impl From<Inputmux> for u8 {
    #[inline(always)]
    fn from(val: Inputmux) -> u8 {
        Inputmux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Itrc0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Itrc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Itrc0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Itrc0 {
    #[inline(always)]
    fn from(val: u8) -> Itrc0 {
        Itrc0::from_bits(val)
    }
}
impl From<Itrc0> for u8 {
    #[inline(always)]
    fn from(val: Itrc0) -> u8 {
        Itrc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockNsMpu {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCK_NS_MPU is 1"]
    LOCK_NS_MPU_EQ_1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_NS_MPU is 0"]
    LOCK_NS_MPU_EQ_0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockNsMpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockNsMpu {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockNsMpu {
    #[inline(always)]
    fn from(val: u8) -> LockNsMpu {
        LockNsMpu::from_bits(val)
    }
}
impl From<LockNsMpu> for u8 {
    #[inline(always)]
    fn from(val: LockNsMpu) -> u8 {
        LockNsMpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockNsVtor {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCKNSVTOR is 1"]
    LOCK_NS_VTOR_EQ_1 = 0x01,
    #[doc = "CM33 (CPU0) LOCKNSVTOR is 0"]
    LOCK_NS_VTOR_EQ_0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockNsVtor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockNsVtor {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockNsVtor {
    #[inline(always)]
    fn from(val: u8) -> LockNsVtor {
        LockNsVtor::from_bits(val)
    }
}
impl From<LockNsVtor> for u8 {
    #[inline(always)]
    fn from(val: LockNsVtor) -> u8 {
        LockNsVtor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockSMpu {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCK_S_MPU is 1"]
    LOCK_S_MPU_EQ_1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_S_MPU is 0"]
    LOCK_S_MPU_EQ_0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockSMpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockSMpu {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockSMpu {
    #[inline(always)]
    fn from(val: u8) -> LockSMpu {
        LockSMpu::from_bits(val)
    }
}
impl From<LockSMpu> for u8 {
    #[inline(always)]
    fn from(val: LockSMpu) -> u8 {
        LockSMpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockSVtaircr {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCK_S_VTAIRCR is 1"]
    LOCK_S_VTAIRCR_EQ_1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_S_VTAIRCR is 0"]
    LOCK_S_VTAIRCR_EQ_0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockSVtaircr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockSVtaircr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockSVtaircr {
    #[inline(always)]
    fn from(val: u8) -> LockSVtaircr {
        LockSVtaircr::from_bits(val)
    }
}
impl From<LockSVtaircr> for u8 {
    #[inline(always)]
    fn from(val: LockSVtaircr) -> u8 {
        LockSVtaircr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockSau {
    _RESERVED_0 = 0x0,
    #[doc = "CM33 (CPU0) LOCK_SAU is 1"]
    LOCK_SAU_EQ_1 = 0x01,
    #[doc = "CM33 (CPU0) LOCK_SAU is 0"]
    LOCK_SAU_EQ_0 = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockSau {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockSau {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockSau {
    #[inline(always)]
    fn from(val: u8) -> LockSau {
        LockSau::from_bits(val)
    }
}
impl From<LockSau> for u8 {
    #[inline(always)]
    fn from(val: LockSau) -> u8 {
        LockSau::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpi2c0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c0 {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c0 {
        Lpi2c0::from_bits(val)
    }
}
impl From<Lpi2c0> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c0) -> u8 {
        Lpi2c0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpi2c1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c1 {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c1 {
        Lpi2c1::from_bits(val)
    }
}
impl From<Lpi2c1> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c1) -> u8 {
        Lpi2c1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpi2c2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c2 {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c2 {
        Lpi2c2::from_bits(val)
    }
}
impl From<Lpi2c2> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c2) -> u8 {
        Lpi2c2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpi2c3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c3 {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c3 {
        Lpi2c3::from_bits(val)
    }
}
impl From<Lpi2c3> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c3) -> u8 {
        Lpi2c3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpi2c4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c4 {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c4 {
        Lpi2c4::from_bits(val)
    }
}
impl From<Lpi2c4> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c4) -> u8 {
        Lpi2c4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpspi0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi0 {
    #[inline(always)]
    fn from(val: u8) -> Lpspi0 {
        Lpspi0::from_bits(val)
    }
}
impl From<Lpspi0> for u8 {
    #[inline(always)]
    fn from(val: Lpspi0) -> u8 {
        Lpspi0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpspi1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi1 {
    #[inline(always)]
    fn from(val: u8) -> Lpspi1 {
        Lpspi1::from_bits(val)
    }
}
impl From<Lpspi1> for u8 {
    #[inline(always)]
    fn from(val: Lpspi1) -> u8 {
        Lpspi1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpspi2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi2 {
    #[inline(always)]
    fn from(val: u8) -> Lpspi2 {
        Lpspi2::from_bits(val)
    }
}
impl From<Lpspi2> for u8 {
    #[inline(always)]
    fn from(val: Lpspi2) -> u8 {
        Lpspi2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpspi3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi3 {
    #[inline(always)]
    fn from(val: u8) -> Lpspi3 {
        Lpspi3::from_bits(val)
    }
}
impl From<Lpspi3> for u8 {
    #[inline(always)]
    fn from(val: Lpspi3) -> u8 {
        Lpspi3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpspi4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi4 {
    #[inline(always)]
    fn from(val: u8) -> Lpspi4 {
        Lpspi4::from_bits(val)
    }
}
impl From<Lpspi4> for u8 {
    #[inline(always)]
    fn from(val: Lpspi4) -> u8 {
        Lpspi4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpspi5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi5 {
    #[inline(always)]
    fn from(val: u8) -> Lpspi5 {
        Lpspi5::from_bits(val)
    }
}
impl From<Lpspi5> for u8 {
    #[inline(always)]
    fn from(val: Lpspi5) -> u8 {
        Lpspi5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lptmr {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lptmr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lptmr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lptmr {
    #[inline(always)]
    fn from(val: u8) -> Lptmr {
        Lptmr::from_bits(val)
    }
}
impl From<Lptmr> for u8 {
    #[inline(always)]
    fn from(val: Lptmr) -> u8 {
        Lptmr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpuart0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart0 {
    #[inline(always)]
    fn from(val: u8) -> Lpuart0 {
        Lpuart0::from_bits(val)
    }
}
impl From<Lpuart0> for u8 {
    #[inline(always)]
    fn from(val: Lpuart0) -> u8 {
        Lpuart0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpuart1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart1 {
    #[inline(always)]
    fn from(val: u8) -> Lpuart1 {
        Lpuart1::from_bits(val)
    }
}
impl From<Lpuart1> for u8 {
    #[inline(always)]
    fn from(val: Lpuart1) -> u8 {
        Lpuart1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpuart2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart2 {
    #[inline(always)]
    fn from(val: u8) -> Lpuart2 {
        Lpuart2::from_bits(val)
    }
}
impl From<Lpuart2> for u8 {
    #[inline(always)]
    fn from(val: Lpuart2) -> u8 {
        Lpuart2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpuart3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart3 {
    #[inline(always)]
    fn from(val: u8) -> Lpuart3 {
        Lpuart3::from_bits(val)
    }
}
impl From<Lpuart3> for u8 {
    #[inline(always)]
    fn from(val: Lpuart3) -> u8 {
        Lpuart3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpuart4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart4 {
    #[inline(always)]
    fn from(val: u8) -> Lpuart4 {
        Lpuart4::from_bits(val)
    }
}
impl From<Lpuart4> for u8 {
    #[inline(always)]
    fn from(val: Lpuart4) -> u8 {
        Lpuart4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Lpuart5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart5 {
    #[inline(always)]
    fn from(val: u8) -> Lpuart5 {
        Lpuart5::from_bits(val)
    }
}
impl From<Lpuart5> for u8 {
    #[inline(always)]
    fn from(val: Lpuart5) -> u8 {
        Lpuart5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegDma0 {
    #[doc = "Non-secure and non-privileged Master"]
    NONSECURE_NONPRIV_MASTER = 0x0,
    #[doc = "Non-secure and privileged Master"]
    NONSECURE_PRIV_MASTER = 0x01,
    #[doc = "Secure and non-privileged Master"]
    SECURE_NONPRIV_MASTER = 0x02,
    #[doc = "Secure and privileged Master"]
    SECURE_PRIV_MASTER = 0x03,
}
impl MasterSecAntiPolRegDma0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegDma0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegDma0 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegDma0 {
        MasterSecAntiPolRegDma0::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegDma0> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegDma0) -> u8 {
        MasterSecAntiPolRegDma0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegDma1 {
    #[doc = "Non-secure and non-privileged Master"]
    NONSECURE_NONPRIV_MASTER = 0x0,
    #[doc = "Non-secure and privileged Master"]
    NONSECURE_PRIV_MASTER = 0x01,
    #[doc = "Secure and non-privileged Master"]
    SECURE_NONPRIV_MASTER = 0x02,
    #[doc = "Secure and privileged Master"]
    SECURE_PRIV_MASTER = 0x03,
}
impl MasterSecAntiPolRegDma1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegDma1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegDma1 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegDma1 {
        MasterSecAntiPolRegDma1::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegDma1> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegDma1) -> u8 {
        MasterSecAntiPolRegDma1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegEnet0 {
    #[doc = "Non-secure and non-privileged Master"]
    NONSECURE_NONPRIV_MASTER = 0x0,
    #[doc = "Non-secure and privileged Master"]
    NONSECURE_PRIV_MASTER = 0x01,
    #[doc = "Secure and non-privileged Master"]
    SECURE_NONPRIV_MASTER = 0x02,
    #[doc = "Secure and privileged Master"]
    SECURE_PRIV_MASTER = 0x03,
}
impl MasterSecAntiPolRegEnet0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegEnet0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegEnet0 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegEnet0 {
        MasterSecAntiPolRegEnet0::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegEnet0> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegEnet0) -> u8 {
        MasterSecAntiPolRegEnet0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegPkc {
    #[doc = "Non-secure and non-privileged Master"]
    NONSECURE_NONPRIV_MASTER = 0x0,
    #[doc = "Non-secure and privileged Master"]
    NONSECURE_PRIV_MASTER = 0x01,
    #[doc = "Secure and non-privileged Master"]
    SECURE_NONPRIV_MASTER = 0x02,
    #[doc = "Secure and privileged Master"]
    SECURE_PRIV_MASTER = 0x03,
}
impl MasterSecAntiPolRegPkc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegPkc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegPkc {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegPkc {
        MasterSecAntiPolRegPkc::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegPkc> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegPkc) -> u8 {
        MasterSecAntiPolRegPkc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegSmartdma {
    #[doc = "Non-secure and non-privileged Master"]
    NONSECURE_NONPRIV_MASTER = 0x0,
    #[doc = "Non-secure and privileged Master"]
    NONSECURE_PRIV_MASTER = 0x01,
    #[doc = "Secure and non-privileged Master"]
    SECURE_NONPRIV_MASTER = 0x02,
    #[doc = "Secure and privileged Master"]
    SECURE_PRIV_MASTER = 0x03,
}
impl MasterSecAntiPolRegSmartdma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegSmartdma {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegSmartdma {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegSmartdma {
        MasterSecAntiPolRegSmartdma::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegSmartdma> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegSmartdma) -> u8 {
        MasterSecAntiPolRegSmartdma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecAntiPolRegUsb1 {
    #[doc = "Non-secure and non-privileged Master"]
    NONSECURE_NONPRIV_MASTER = 0x0,
    #[doc = "Non-secure and privileged Master"]
    NONSECURE_PRIV_MASTER = 0x01,
    #[doc = "Secure and non-privileged Master"]
    SECURE_NONPRIV_MASTER = 0x02,
    #[doc = "Secure and privileged Master"]
    SECURE_PRIV_MASTER = 0x03,
}
impl MasterSecAntiPolRegUsb1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecAntiPolRegUsb1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecAntiPolRegUsb1 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecAntiPolRegUsb1 {
        MasterSecAntiPolRegUsb1::from_bits(val)
    }
}
impl From<MasterSecAntiPolRegUsb1> for u8 {
    #[inline(always)]
    fn from(val: MasterSecAntiPolRegUsb1) -> u8 {
        MasterSecAntiPolRegUsb1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelDma0 {
    #[doc = "Non-secure and non-privileged Master"]
    NONSECURE_NONPRIV_MASTER = 0x0,
    #[doc = "Non-secure and privileged Master"]
    NONSECURE_PRIV_MASTER = 0x01,
    #[doc = "Secure and non-privileged Master"]
    SECURE_NONPRIV_MASTER = 0x02,
    #[doc = "Secure and privileged Master"]
    SECURE_PRIV_MASTER = 0x03,
}
impl MasterSecLevelDma0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelDma0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelDma0 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelDma0 {
        MasterSecLevelDma0::from_bits(val)
    }
}
impl From<MasterSecLevelDma0> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelDma0) -> u8 {
        MasterSecLevelDma0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelDma1 {
    #[doc = "Non-secure and non-privileged Master"]
    NONSECURE_NONPRIV_MASTER = 0x0,
    #[doc = "Non-secure and privileged Master"]
    NONSECURE_PRIV_MASTER = 0x01,
    #[doc = "Secure and non-privileged Master"]
    SECURE_NONPRIV_MASTER = 0x02,
    #[doc = "Secure and privileged Master"]
    SECURE_PRIV_MASTER = 0x03,
}
impl MasterSecLevelDma1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelDma1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelDma1 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelDma1 {
        MasterSecLevelDma1::from_bits(val)
    }
}
impl From<MasterSecLevelDma1> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelDma1) -> u8 {
        MasterSecLevelDma1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelEnet0 {
    #[doc = "Non-secure and non-privileged Master"]
    NONSECURE_NONPRIV_MASTER = 0x0,
    #[doc = "Non-secure and privileged Master"]
    NONSECURE_PRIV_MASTER = 0x01,
    #[doc = "Secure and non-privileged Master"]
    SECURE_NONPRIV_MASTER = 0x02,
    #[doc = "Secure and privileged Master"]
    SECURE_PRIV_MASTER = 0x03,
}
impl MasterSecLevelEnet0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelEnet0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelEnet0 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelEnet0 {
        MasterSecLevelEnet0::from_bits(val)
    }
}
impl From<MasterSecLevelEnet0> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelEnet0) -> u8 {
        MasterSecLevelEnet0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelPkc {
    #[doc = "Non-secure and non-privileged Master"]
    NONSECURE_NONPRIV_MASTER = 0x0,
    #[doc = "Non-secure and privileged Master"]
    NONSECURE_PRIV_MASTER = 0x01,
    #[doc = "Secure and non-privileged Master"]
    SECURE_NONPRIV_MASTER = 0x02,
    #[doc = "Secure and privileged Master"]
    SECURE_PRIV_MASTER = 0x03,
}
impl MasterSecLevelPkc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelPkc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelPkc {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelPkc {
        MasterSecLevelPkc::from_bits(val)
    }
}
impl From<MasterSecLevelPkc> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelPkc) -> u8 {
        MasterSecLevelPkc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelSmartdma {
    #[doc = "Non-secure and non-privileged Master"]
    NONSECURE_NONPRIV_MASTER = 0x0,
    #[doc = "Non-secure and privileged Master"]
    NONSECURE_PRIV_MASTER = 0x01,
    #[doc = "Secure and non-privileged Master"]
    SECURE_NONPRIV_MASTER = 0x02,
    #[doc = "Secure and privileged Master"]
    SECURE_PRIV_MASTER = 0x03,
}
impl MasterSecLevelSmartdma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelSmartdma {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelSmartdma {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelSmartdma {
        MasterSecLevelSmartdma::from_bits(val)
    }
}
impl From<MasterSecLevelSmartdma> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelSmartdma) -> u8 {
        MasterSecLevelSmartdma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterSecLevelUsb1 {
    #[doc = "Non-secure and non-privileged Master"]
    NONSECURE_NONPRIV_MASTER = 0x0,
    #[doc = "Non-secure and privileged Master"]
    NONSECURE_PRIV_MASTER = 0x01,
    #[doc = "Secure and non-privileged Master"]
    SECURE_NONPRIV_MASTER = 0x02,
    #[doc = "Secure and privileged Master"]
    SECURE_PRIV_MASTER = 0x03,
}
impl MasterSecLevelUsb1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterSecLevelUsb1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterSecLevelUsb1 {
    #[inline(always)]
    fn from(val: u8) -> MasterSecLevelUsb1 {
        MasterSecLevelUsb1::from_bits(val)
    }
}
impl From<MasterSecLevelUsb1> for u8 {
    #[inline(always)]
    fn from(val: MasterSecLevelUsb1) -> u8 {
        MasterSecLevelUsb1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mau0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Mau0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mau0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mau0 {
    #[inline(always)]
    fn from(val: u8) -> Mau0 {
        Mau0::from_bits(val)
    }
}
impl From<Mau0> for u8 {
    #[inline(always)]
    fn from(val: Mau0) -> u8 {
        Mau0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbc {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Mbc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbc {
    #[inline(always)]
    fn from(val: u8) -> Mbc {
        Mbc::from_bits(val)
    }
}
impl From<Mbc> for u8 {
    #[inline(always)]
    fn from(val: Mbc) -> u8 {
        Mbc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegDisableStrictMode {
    _RESERVED_0 = 0x0,
    #[doc = "Master can access memories and peripherals at the same level or below that level."]
    AHBTM = 0x01,
    #[doc = "Master can access memories and peripherals at same level only"]
    AHBSM1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegDisableStrictMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegDisableStrictMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegDisableStrictMode {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegDisableStrictMode {
        MiscCtrlDpRegDisableStrictMode::from_bits(val)
    }
}
impl From<MiscCtrlDpRegDisableStrictMode> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegDisableStrictMode) -> u8 {
        MiscCtrlDpRegDisableStrictMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegDisableViolationAbort {
    _RESERVED_0 = 0x0,
    #[doc = "The violation detected by the secure checker will not cause an abort, but a secure_violation_irq (interrupt request) will still be asserted and serviced by ISR."]
    NO_ABORT = 0x01,
    #[doc = "The violation detected by the secure checker will cause an abort."]
    ABORT = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegDisableViolationAbort {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegDisableViolationAbort {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegDisableViolationAbort {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegDisableViolationAbort {
        MiscCtrlDpRegDisableViolationAbort::from_bits(val)
    }
}
impl From<MiscCtrlDpRegDisableViolationAbort> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegDisableViolationAbort) -> u8 {
        MiscCtrlDpRegDisableViolationAbort::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegEnableNsPrivCheck {
    _RESERVED_0 = 0x0,
    #[doc = "Enables the privilege checking of non-secure mode access."]
    ENABLED = 0x01,
    #[doc = "Disables the privilege checking of non-secure mode access."]
    DISABLED = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegEnableNsPrivCheck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegEnableNsPrivCheck {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegEnableNsPrivCheck {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegEnableNsPrivCheck {
        MiscCtrlDpRegEnableNsPrivCheck::from_bits(val)
    }
}
impl From<MiscCtrlDpRegEnableNsPrivCheck> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegEnableNsPrivCheck) -> u8 {
        MiscCtrlDpRegEnableNsPrivCheck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegEnableSPrivCheck {
    _RESERVED_0 = 0x0,
    #[doc = "Enables the privilege checking of secure mode access."]
    ENABLED = 0x01,
    #[doc = "Disables the privilege checking of secure mode access."]
    DISABLED = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegEnableSPrivCheck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegEnableSPrivCheck {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegEnableSPrivCheck {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegEnableSPrivCheck {
        MiscCtrlDpRegEnableSPrivCheck::from_bits(val)
    }
}
impl From<MiscCtrlDpRegEnableSPrivCheck> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegEnableSPrivCheck) -> u8 {
        MiscCtrlDpRegEnableSPrivCheck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegEnableSecureChecking {
    _RESERVED_0 = 0x0,
    #[doc = "Enables secure checking. Violation can be detected when the security level of a transaction does not meet the security rule of the slave or memory to be accessed."]
    ENABLED = 0x01,
    #[doc = "Disables secure checking. Even if the security level of a transaction does not conform to the security rule of the slave or memory, it will not be detected as a violation."]
    DISABLED = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegEnableSecureChecking {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegEnableSecureChecking {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegEnableSecureChecking {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegEnableSecureChecking {
        MiscCtrlDpRegEnableSecureChecking::from_bits(val)
    }
}
impl From<MiscCtrlDpRegEnableSecureChecking> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegEnableSecureChecking) -> u8 {
        MiscCtrlDpRegEnableSecureChecking::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegIdauAllNs {
    _RESERVED_0 = 0x0,
    #[doc = "IDAU is disabled, which means that all memories are attributed as non-secure memory."]
    DISABLED = 0x01,
    #[doc = "IDAU is enabled (restrictive mode)"]
    ENABLED = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegIdauAllNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegIdauAllNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegIdauAllNs {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegIdauAllNs {
        MiscCtrlDpRegIdauAllNs::from_bits(val)
    }
}
impl From<MiscCtrlDpRegIdauAllNs> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegIdauAllNs) -> u8 {
        MiscCtrlDpRegIdauAllNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlDpRegWriteLock {
    _RESERVED_0 = 0x0,
    #[doc = "Writes to this register and to the Memory and Peripheral RULE registers are not allowed"]
    LOCKED = 0x01,
    #[doc = "Writes to this register and to the Memory and Peripheral RULE registers are allowed"]
    NOT_LOCKED = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlDpRegWriteLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlDpRegWriteLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlDpRegWriteLock {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlDpRegWriteLock {
        MiscCtrlDpRegWriteLock::from_bits(val)
    }
}
impl From<MiscCtrlDpRegWriteLock> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlDpRegWriteLock) -> u8 {
        MiscCtrlDpRegWriteLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegDisableStrictMode {
    _RESERVED_0 = 0x0,
    #[doc = "Master strict mode is disabled/off and can access memories and peripherals at the same level or below that level"]
    AHBTM = 0x01,
    #[doc = "Master strict mode is enabled/on and can access memories and peripherals at same level only"]
    AHBSM1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegDisableStrictMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegDisableStrictMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegDisableStrictMode {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegDisableStrictMode {
        MiscCtrlRegDisableStrictMode::from_bits(val)
    }
}
impl From<MiscCtrlRegDisableStrictMode> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegDisableStrictMode) -> u8 {
        MiscCtrlRegDisableStrictMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegDisableViolationAbort {
    _RESERVED_0 = 0x0,
    #[doc = "The violation detected by the secure checker will not cause an abort, but a secure_violation_irq (interrupt request) will still be asserted and serviced by ISR."]
    NO_ABORT = 0x01,
    #[doc = "The violation detected by the secure checker will cause an abort."]
    ABORT = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegDisableViolationAbort {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegDisableViolationAbort {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegDisableViolationAbort {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegDisableViolationAbort {
        MiscCtrlRegDisableViolationAbort::from_bits(val)
    }
}
impl From<MiscCtrlRegDisableViolationAbort> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegDisableViolationAbort) -> u8 {
        MiscCtrlRegDisableViolationAbort::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegEnableNsPrivCheck {
    _RESERVED_0 = 0x0,
    #[doc = "Enables privilege checking of non-secure mode access."]
    ENABLED = 0x01,
    #[doc = "Disables privilege checking of non-secure mode access is disabled."]
    DISABLED = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegEnableNsPrivCheck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegEnableNsPrivCheck {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegEnableNsPrivCheck {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegEnableNsPrivCheck {
        MiscCtrlRegEnableNsPrivCheck::from_bits(val)
    }
}
impl From<MiscCtrlRegEnableNsPrivCheck> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegEnableNsPrivCheck) -> u8 {
        MiscCtrlRegEnableNsPrivCheck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegEnableSPrivCheck {
    _RESERVED_0 = 0x0,
    #[doc = "Enables privilege checking of secure mode access."]
    ENABLED = 0x01,
    #[doc = "Disables privilege checking of secure mode access."]
    DISABLED = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegEnableSPrivCheck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegEnableSPrivCheck {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegEnableSPrivCheck {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegEnableSPrivCheck {
        MiscCtrlRegEnableSPrivCheck::from_bits(val)
    }
}
impl From<MiscCtrlRegEnableSPrivCheck> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegEnableSPrivCheck) -> u8 {
        MiscCtrlRegEnableSPrivCheck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegEnableSecureChecking {
    _RESERVED_0 = 0x0,
    #[doc = "Enables secure checking. Violation can be detected when the security level of a transaction does not meet the security rule of the slave or memory to be accessed."]
    ENABLED = 0x01,
    #[doc = "Disables secure checking. Even if the security level of a transaction does not conform to the security rule of the slave or memory, it will not be detected as a violation."]
    DISABLED = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegEnableSecureChecking {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegEnableSecureChecking {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegEnableSecureChecking {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegEnableSecureChecking {
        MiscCtrlRegEnableSecureChecking::from_bits(val)
    }
}
impl From<MiscCtrlRegEnableSecureChecking> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegEnableSecureChecking) -> u8 {
        MiscCtrlRegEnableSecureChecking::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegIdauAllNs {
    _RESERVED_0 = 0x0,
    #[doc = "IDAU is disabled, which means that all memories are attributed as non-secure memory."]
    DISABLED = 0x01,
    #[doc = "IDAU is enabled (restrictive mode)"]
    ENABLED = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegIdauAllNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegIdauAllNs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegIdauAllNs {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegIdauAllNs {
        MiscCtrlRegIdauAllNs::from_bits(val)
    }
}
impl From<MiscCtrlRegIdauAllNs> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegIdauAllNs) -> u8 {
        MiscCtrlRegIdauAllNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscCtrlRegWriteLock {
    _RESERVED_0 = 0x0,
    #[doc = "Writes to this register and to the Memory and Peripheral RULE registers are not allowed"]
    LOCKED = 0x01,
    #[doc = "Writes to this register and to the Memory and Peripheral RULE registers are allowed"]
    NOT_LOCKED = 0x02,
    _RESERVED_3 = 0x03,
}
impl MiscCtrlRegWriteLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscCtrlRegWriteLock {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscCtrlRegWriteLock {
    #[inline(always)]
    fn from(val: u8) -> MiscCtrlRegWriteLock {
        MiscCtrlRegWriteLock::from_bits(val)
    }
}
impl From<MiscCtrlRegWriteLock> for u8 {
    #[inline(always)]
    fn from(val: MiscCtrlRegWriteLock) -> u8 {
        MiscCtrlRegWriteLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ostimer {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Ostimer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ostimer {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ostimer {
    #[inline(always)]
    fn from(val: u8) -> Ostimer {
        Ostimer::from_bits(val)
    }
}
impl From<Ostimer> for u8 {
    #[inline(always)]
    fn from(val: Ostimer) -> u8 {
        Ostimer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pkc0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Pkc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pkc0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pkc0 {
    #[inline(always)]
    fn from(val: u8) -> Pkc0 {
        Pkc0::from_bits(val)
    }
}
impl From<Pkc0> for u8 {
    #[inline(always)]
    fn from(val: Pkc0) -> u8 {
        Pkc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Port0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port0 {
    #[inline(always)]
    fn from(val: u8) -> Port0 {
        Port0::from_bits(val)
    }
}
impl From<Port0> for u8 {
    #[inline(always)]
    fn from(val: Port0) -> u8 {
        Port0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Port1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port1 {
    #[inline(always)]
    fn from(val: u8) -> Port1 {
        Port1::from_bits(val)
    }
}
impl From<Port1> for u8 {
    #[inline(always)]
    fn from(val: Port1) -> u8 {
        Port1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Port2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port2 {
    #[inline(always)]
    fn from(val: u8) -> Port2 {
        Port2::from_bits(val)
    }
}
impl From<Port2> for u8 {
    #[inline(always)]
    fn from(val: Port2) -> u8 {
        Port2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Port3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port3 {
    #[inline(always)]
    fn from(val: u8) -> Port3 {
        Port3::from_bits(val)
    }
}
impl From<Port3> for u8 {
    #[inline(always)]
    fn from(val: Port3) -> u8 {
        Port3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Port4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port4 {
    #[inline(always)]
    fn from(val: u8) -> Port4 {
        Port4::from_bits(val)
    }
}
impl From<Port4> for u8 {
    #[inline(always)]
    fn from(val: Port4) -> u8 {
        Port4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Port5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Port5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Port5 {
    #[inline(always)]
    fn from(val: u8) -> Port5 {
        Port5::from_bits(val)
    }
}
impl From<Port5> for u8 {
    #[inline(always)]
    fn from(val: Port5) -> u8 {
        Port5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamaMemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamaMemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamaMemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamaMemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> RamaMemRuleRule0 {
        RamaMemRuleRule0::from_bits(val)
    }
}
impl From<RamaMemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: RamaMemRuleRule0) -> u8 {
        RamaMemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamaMemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamaMemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamaMemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamaMemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> RamaMemRuleRule1 {
        RamaMemRuleRule1::from_bits(val)
    }
}
impl From<RamaMemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: RamaMemRuleRule1) -> u8 {
        RamaMemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamaMemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamaMemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamaMemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamaMemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> RamaMemRuleRule2 {
        RamaMemRuleRule2::from_bits(val)
    }
}
impl From<RamaMemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: RamaMemRuleRule2) -> u8 {
        RamaMemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamaMemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamaMemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamaMemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamaMemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> RamaMemRuleRule3 {
        RamaMemRuleRule3::from_bits(val)
    }
}
impl From<RamaMemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: RamaMemRuleRule3) -> u8 {
        RamaMemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamaMemRuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamaMemRuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamaMemRuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamaMemRuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> RamaMemRuleRule4 {
        RamaMemRuleRule4::from_bits(val)
    }
}
impl From<RamaMemRuleRule4> for u8 {
    #[inline(always)]
    fn from(val: RamaMemRuleRule4) -> u8 {
        RamaMemRuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamaMemRuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamaMemRuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamaMemRuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamaMemRuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> RamaMemRuleRule5 {
        RamaMemRuleRule5::from_bits(val)
    }
}
impl From<RamaMemRuleRule5> for u8 {
    #[inline(always)]
    fn from(val: RamaMemRuleRule5) -> u8 {
        RamaMemRuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamaMemRuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamaMemRuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamaMemRuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamaMemRuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> RamaMemRuleRule6 {
        RamaMemRuleRule6::from_bits(val)
    }
}
impl From<RamaMemRuleRule6> for u8 {
    #[inline(always)]
    fn from(val: RamaMemRuleRule6) -> u8 {
        RamaMemRuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamaMemRuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamaMemRuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamaMemRuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamaMemRuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> RamaMemRuleRule7 {
        RamaMemRuleRule7::from_bits(val)
    }
}
impl From<RamaMemRuleRule7> for u8 {
    #[inline(always)]
    fn from(val: RamaMemRuleRule7) -> u8 {
        RamaMemRuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RambMemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RambMemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RambMemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RambMemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> RambMemRuleRule0 {
        RambMemRuleRule0::from_bits(val)
    }
}
impl From<RambMemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: RambMemRuleRule0) -> u8 {
        RambMemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RambMemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RambMemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RambMemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RambMemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> RambMemRuleRule1 {
        RambMemRuleRule1::from_bits(val)
    }
}
impl From<RambMemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: RambMemRuleRule1) -> u8 {
        RambMemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RambMemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RambMemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RambMemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RambMemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> RambMemRuleRule2 {
        RambMemRuleRule2::from_bits(val)
    }
}
impl From<RambMemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: RambMemRuleRule2) -> u8 {
        RambMemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RambMemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RambMemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RambMemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RambMemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> RambMemRuleRule3 {
        RambMemRuleRule3::from_bits(val)
    }
}
impl From<RambMemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: RambMemRuleRule3) -> u8 {
        RambMemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RambMemRuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RambMemRuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RambMemRuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RambMemRuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> RambMemRuleRule4 {
        RambMemRuleRule4::from_bits(val)
    }
}
impl From<RambMemRuleRule4> for u8 {
    #[inline(always)]
    fn from(val: RambMemRuleRule4) -> u8 {
        RambMemRuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RambMemRuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RambMemRuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RambMemRuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RambMemRuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> RambMemRuleRule5 {
        RambMemRuleRule5::from_bits(val)
    }
}
impl From<RambMemRuleRule5> for u8 {
    #[inline(always)]
    fn from(val: RambMemRuleRule5) -> u8 {
        RambMemRuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RambMemRuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RambMemRuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RambMemRuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RambMemRuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> RambMemRuleRule6 {
        RambMemRuleRule6::from_bits(val)
    }
}
impl From<RambMemRuleRule6> for u8 {
    #[inline(always)]
    fn from(val: RambMemRuleRule6) -> u8 {
        RambMemRuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RambMemRuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RambMemRuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RambMemRuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RambMemRuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> RambMemRuleRule7 {
        RambMemRuleRule7::from_bits(val)
    }
}
impl From<RambMemRuleRule7> for u8 {
    #[inline(always)]
    fn from(val: RambMemRuleRule7) -> u8 {
        RambMemRuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamxMemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamxMemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamxMemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamxMemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> RamxMemRuleRule0 {
        RamxMemRuleRule0::from_bits(val)
    }
}
impl From<RamxMemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: RamxMemRuleRule0) -> u8 {
        RamxMemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamxMemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamxMemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamxMemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamxMemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> RamxMemRuleRule1 {
        RamxMemRuleRule1::from_bits(val)
    }
}
impl From<RamxMemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: RamxMemRuleRule1) -> u8 {
        RamxMemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamxMemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamxMemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamxMemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamxMemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> RamxMemRuleRule2 {
        RamxMemRuleRule2::from_bits(val)
    }
}
impl From<RamxMemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: RamxMemRuleRule2) -> u8 {
        RamxMemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamxMemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamxMemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamxMemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamxMemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> RamxMemRuleRule3 {
        RamxMemRuleRule3::from_bits(val)
    }
}
impl From<RamxMemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: RamxMemRuleRule3) -> u8 {
        RamxMemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamxMemRuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamxMemRuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamxMemRuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamxMemRuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> RamxMemRuleRule4 {
        RamxMemRuleRule4::from_bits(val)
    }
}
impl From<RamxMemRuleRule4> for u8 {
    #[inline(always)]
    fn from(val: RamxMemRuleRule4) -> u8 {
        RamxMemRuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamxMemRuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamxMemRuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamxMemRuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamxMemRuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> RamxMemRuleRule5 {
        RamxMemRuleRule5::from_bits(val)
    }
}
impl From<RamxMemRuleRule5> for u8 {
    #[inline(always)]
    fn from(val: RamxMemRuleRule5) -> u8 {
        RamxMemRuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamxMemRuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamxMemRuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamxMemRuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamxMemRuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> RamxMemRuleRule6 {
        RamxMemRuleRule6::from_bits(val)
    }
}
impl From<RamxMemRuleRule6> for u8 {
    #[inline(always)]
    fn from(val: RamxMemRuleRule6) -> u8 {
        RamxMemRuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamxMemRuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RamxMemRuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamxMemRuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamxMemRuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> RamxMemRuleRule7 {
        RamxMemRuleRule7::from_bits(val)
    }
}
impl From<RamxMemRuleRule7> for u8 {
    #[inline(always)]
    fn from(val: RamxMemRuleRule7) -> u8 {
        RamxMemRuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RomMemRuleRule0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule0 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule0 {
        RomMemRuleRule0::from_bits(val)
    }
}
impl From<RomMemRuleRule0> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule0) -> u8 {
        RomMemRuleRule0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RomMemRuleRule1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule1 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule1 {
        RomMemRuleRule1::from_bits(val)
    }
}
impl From<RomMemRuleRule1> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule1) -> u8 {
        RomMemRuleRule1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule2 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RomMemRuleRule2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule2 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule2 {
        RomMemRuleRule2::from_bits(val)
    }
}
impl From<RomMemRuleRule2> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule2) -> u8 {
        RomMemRuleRule2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule3 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RomMemRuleRule3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule3 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule3 {
        RomMemRuleRule3::from_bits(val)
    }
}
impl From<RomMemRuleRule3> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule3) -> u8 {
        RomMemRuleRule3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule4 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RomMemRuleRule4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule4 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule4 {
        RomMemRuleRule4::from_bits(val)
    }
}
impl From<RomMemRuleRule4> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule4) -> u8 {
        RomMemRuleRule4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule5 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RomMemRuleRule5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule5 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule5 {
        RomMemRuleRule5::from_bits(val)
    }
}
impl From<RomMemRuleRule5> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule5) -> u8 {
        RomMemRuleRule5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule6 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RomMemRuleRule6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule6 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule6 {
        RomMemRuleRule6::from_bits(val)
    }
}
impl From<RomMemRuleRule6> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule6) -> u8 {
        RomMemRuleRule6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RomMemRuleRule7 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl RomMemRuleRule7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomMemRuleRule7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomMemRuleRule7 {
    #[inline(always)]
    fn from(val: u8) -> RomMemRuleRule7 {
        RomMemRuleRule7::from_bits(val)
    }
}
impl From<RomMemRuleRule7> for u8 {
    #[inline(always)]
    fn from(val: RomMemRuleRule7) -> u8 {
        RomMemRuleRule7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Romcp {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Romcp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Romcp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Romcp {
    #[inline(always)]
    fn from(val: u8) -> Romcp {
        Romcp::from_bits(val)
    }
}
impl From<Romcp> for u8 {
    #[inline(always)]
    fn from(val: Romcp) -> u8 {
        Romcp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rtc0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Rtc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtc0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtc0 {
    #[inline(always)]
    fn from(val: u8) -> Rtc0 {
        Rtc0::from_bits(val)
    }
}
impl From<Rtc0> for u8 {
    #[inline(always)]
    fn from(val: Rtc0) -> u8 {
        Rtc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scg {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Scg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scg {
    #[inline(always)]
    fn from(val: u8) -> Scg {
        Scg::from_bits(val)
    }
}
impl From<Scg> for u8 {
    #[inline(always)]
    fn from(val: Scg) -> u8 {
        Scg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecVioInfoDataAccess {
    #[doc = "Code"]
    CODE = 0x0,
    #[doc = "Data"]
    DATA = 0x01,
}
impl SecVioInfoDataAccess {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecVioInfoDataAccess {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecVioInfoDataAccess {
    #[inline(always)]
    fn from(val: u8) -> SecVioInfoDataAccess {
        SecVioInfoDataAccess::from_bits(val)
    }
}
impl From<SecVioInfoDataAccess> for u8 {
    #[inline(always)]
    fn from(val: SecVioInfoDataAccess) -> u8 {
        SecVioInfoDataAccess::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecVioInfoMaster {
    #[doc = "CM33 Code"]
    CPU0_CODE = 0x0,
    #[doc = "CM33 System"]
    CPU0_SYS = 0x01,
    #[doc = "SMARTDMA Instruction"]
    SDMA_INSTR = 0x02,
    #[doc = "SMARTDMA Data"]
    SDMA_DATA = 0x03,
    #[doc = "eDMA1"]
    E_DMA1 = 0x04,
    #[doc = "eDMA0"]
    E_DMA0 = 0x05,
    #[doc = "USB HS"]
    USB_HS = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "eSPI"]
    ESPI = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "PKC"]
    PKC = 0x0c,
    #[doc = "Ethernet"]
    ENET = 0x0d,
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
impl SecVioInfoMaster {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecVioInfoMaster {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecVioInfoMaster {
    #[inline(always)]
    fn from(val: u8) -> SecVioInfoMaster {
        SecVioInfoMaster::from_bits(val)
    }
}
impl From<SecVioInfoMaster> for u8 {
    #[inline(always)]
    fn from(val: SecVioInfoMaster) -> u8 {
        SecVioInfoMaster::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecVioInfoWrite {
    #[doc = "Read access"]
    READ = 0x0,
    #[doc = "Write access"]
    WRITE = 0x01,
}
impl SecVioInfoWrite {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecVioInfoWrite {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecVioInfoWrite {
    #[inline(always)]
    fn from(val: u8) -> SecVioInfoWrite {
        SecVioInfoWrite::from_bits(val)
    }
}
impl From<SecVioInfoWrite> for u8 {
    #[inline(always)]
    fn from(val: SecVioInfoWrite) -> u8 {
        SecVioInfoWrite::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Seccon {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Seccon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Seccon {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Seccon {
    #[inline(always)]
    fn from(val: u8) -> Seccon {
        Seccon::from_bits(val)
    }
}
impl From<Seccon> for u8 {
    #[inline(always)]
    fn from(val: Seccon) -> u8 {
        Seccon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sgi0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Sgi0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sgi0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sgi0 {
    #[inline(always)]
    fn from(val: u8) -> Sgi0 {
        Sgi0::from_bits(val)
    }
}
impl From<Sgi0> for u8 {
    #[inline(always)]
    fn from(val: Sgi0) -> u8 {
        Sgi0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spc {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Spc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spc {
    #[inline(always)]
    fn from(val: u8) -> Spc {
        Spc::from_bits(val)
    }
}
impl From<Spc> for u8 {
    #[inline(always)]
    fn from(val: Spc) -> u8 {
        Spc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpiFileter0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl SpiFileter0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpiFileter0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpiFileter0 {
    #[inline(always)]
    fn from(val: u8) -> SpiFileter0 {
        SpiFileter0::from_bits(val)
    }
}
impl From<SpiFileter0> for u8 {
    #[inline(always)]
    fn from(val: SpiFileter0) -> u8 {
        SpiFileter0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Syscon {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Syscon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Syscon {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Syscon {
    #[inline(always)]
    fn from(val: u8) -> Syscon {
        Syscon::from_bits(val)
    }
}
impl From<Syscon> for u8 {
    #[inline(always)]
    fn from(val: Syscon) -> u8 {
        Syscon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum T1s0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl T1s0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> T1s0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for T1s0 {
    #[inline(always)]
    fn from(val: u8) -> T1s0 {
        T1s0::from_bits(val)
    }
}
impl From<T1s0> for u8 {
    #[inline(always)]
    fn from(val: T1s0) -> u8 {
        T1s0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tdet0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Tdet0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tdet0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tdet0 {
    #[inline(always)]
    fn from(val: u8) -> Tdet0 {
        Tdet0::from_bits(val)
    }
}
impl From<Tdet0> for u8 {
    #[inline(always)]
    fn from(val: Tdet0) -> u8 {
        Tdet0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trng0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Trng0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trng0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trng0 {
    #[inline(always)]
    fn from(val: u8) -> Trng0 {
        Trng0::from_bits(val)
    }
}
impl From<Trng0> for u8 {
    #[inline(always)]
    fn from(val: Trng0) -> u8 {
        Trng0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsi0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Tsi0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsi0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsi0 {
    #[inline(always)]
    fn from(val: u8) -> Tsi0 {
        Tsi0::from_bits(val)
    }
}
impl From<Tsi0> for u8 {
    #[inline(always)]
    fn from(val: Tsi0) -> u8 {
        Tsi0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Udf0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Udf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Udf0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Udf0 {
    #[inline(always)]
    fn from(val: u8) -> Udf0 {
        Udf0::from_bits(val)
    }
}
impl From<Udf0> for u8 {
    #[inline(always)]
    fn from(val: Udf0) -> u8 {
        Udf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1Phy {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Usb1Phy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1Phy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1Phy {
    #[inline(always)]
    fn from(val: u8) -> Usb1Phy {
        Usb1Phy::from_bits(val)
    }
}
impl From<Usb1Phy> for u8 {
    #[inline(always)]
    fn from(val: Usb1Phy) -> u8 {
        Usb1Phy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Utick {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Utick {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Utick {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Utick {
    #[inline(always)]
    fn from(val: u8) -> Utick {
        Utick::from_bits(val)
    }
}
impl From<Utick> for u8 {
    #[inline(always)]
    fn from(val: Utick) -> u8 {
        Utick::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vbat {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Vbat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vbat {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vbat {
    #[inline(always)]
    fn from(val: u8) -> Vbat {
        Vbat::from_bits(val)
    }
}
impl From<Vbat> for u8 {
    #[inline(always)]
    fn from(val: Vbat) -> u8 {
        Vbat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vref0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Vref0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vref0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vref0 {
    #[inline(always)]
    fn from(val: u8) -> Vref0 {
        Vref0::from_bits(val)
    }
}
impl From<Vref0> for u8 {
    #[inline(always)]
    fn from(val: Vref0) -> u8 {
        Vref0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WakeTimer {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl WakeTimer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WakeTimer {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WakeTimer {
    #[inline(always)]
    fn from(val: u8) -> WakeTimer {
        WakeTimer::from_bits(val)
    }
}
impl From<WakeTimer> for u8 {
    #[inline(always)]
    fn from(val: WakeTimer) -> u8 {
        WakeTimer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wuu {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Wuu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wuu {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wuu {
    #[inline(always)]
    fn from(val: u8) -> Wuu {
        Wuu::from_bits(val)
    }
}
impl From<Wuu> for u8 {
    #[inline(always)]
    fn from(val: Wuu) -> u8 {
        Wuu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wwdt0 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Wwdt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wwdt0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wwdt0 {
    #[inline(always)]
    fn from(val: u8) -> Wwdt0 {
        Wwdt0::from_bits(val)
    }
}
impl From<Wwdt0> for u8 {
    #[inline(always)]
    fn from(val: Wwdt0) -> u8 {
        Wwdt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wwdt1 {
    #[doc = "Non-secure and non-privilege user access allowed"]
    NONSECURE_NONPRIV_USER_ALLOWED = 0x0,
    #[doc = "Non-secure and privilege access allowed"]
    NONSECURE_PRIV_USER_ALLOWED = 0x01,
    #[doc = "Secure and non-privilege user access allowed"]
    SECURE_NONPRIV_USER_ALLOWED = 0x02,
    #[doc = "Secure and privilege user access allowed"]
    SECURE_PRIV_USER_ALLOWED = 0x03,
}
impl Wwdt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wwdt1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wwdt1 {
    #[inline(always)]
    fn from(val: u8) -> Wwdt1 {
        Wwdt1::from_bits(val)
    }
}
impl From<Wwdt1> for u8 {
    #[inline(always)]
    fn from(val: Wwdt1) -> u8 {
        Wwdt1::to_bits(val)
    }
}
