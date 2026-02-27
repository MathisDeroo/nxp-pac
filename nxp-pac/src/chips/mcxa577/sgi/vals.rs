#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AesKeysize {
    #[doc = "128 0nly"]
    AES_128_ONLY = 0x0,
    #[doc = "192 only"]
    AES_192_ONLY = 0x01,
    #[doc = "256 only"]
    AES_256_ONLY = 0x02,
    #[doc = "All key sizes"]
    ALL_KEYSIZE = 0x03,
}
impl AesKeysize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AesKeysize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AesKeysize {
    #[inline(always)]
    fn from(val: u8) -> AesKeysize {
        AesKeysize::from_bits(val)
    }
}
impl From<AesKeysize> for u8 {
    #[inline(always)]
    fn from(val: AesKeysize) -> u8 {
        AesKeysize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AesNoKl {
    #[doc = "new AES key will be loaded"]
    NEW = 0x0,
    #[doc = "No AES key will be loaded, and previously loaded key will be used."]
    NO = 0x01,
}
impl AesNoKl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AesNoKl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AesNoKl {
    #[inline(always)]
    fn from(val: u8) -> AesNoKl {
        AesNoKl::from_bits(val)
    }
}
impl From<AesNoKl> for u8 {
    #[inline(always)]
    fn from(val: AesNoKl) -> u8 {
        AesNoKl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AesSel {
    #[doc = "First AES selected"]
    FIRST_AES = 0x0,
    #[doc = "Second AES selected (when enabled)"]
    SECOND_AES = 0x01,
}
impl AesSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AesSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AesSel {
    #[inline(always)]
    fn from(val: u8) -> AesSel {
        AesSel::from_bits(val)
    }
}
impl From<AesSel> for u8 {
    #[inline(always)]
    fn from(val: AesSel) -> u8 {
        AesSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AesUsed {
    #[doc = "Apollo"]
    APOLLO = 0x0,
    #[doc = "Aegis"]
    AEGIS = 0x01,
    #[doc = "Ayna"]
    AYNA = 0x02,
    #[doc = "Athenium"]
    ATHENIUM = 0x03,
    #[doc = "Ajax"]
    AJAX = 0x04,
    #[doc = "Aegis_hs"]
    AEGIS_HS = 0x05,
    #[doc = "Athenium_hs"]
    ATHENIUM_HS = 0x06,
    #[doc = "ATE"]
    ATE = 0x07,
    #[doc = "ATOM"]
    ATOM = 0x08,
    #[doc = "Asterix"]
    ASTERIX = 0x09,
    #[doc = "RFU"]
    RFU_10 = 0x0a,
    #[doc = "RFU"]
    RFU_11 = 0x0b,
    #[doc = "RFU"]
    RFU_12 = 0x0c,
    #[doc = "RFU"]
    RFU_13 = 0x0d,
    #[doc = "RFU"]
    RFU_14 = 0x0e,
    #[doc = "RFU"]
    RFU_15 = 0x0f,
}
impl AesUsed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AesUsed {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AesUsed {
    #[inline(always)]
    fn from(val: u8) -> AesUsed {
        AesUsed::from_bits(val)
    }
}
impl From<AesUsed> for u8 {
    #[inline(always)]
    fn from(val: AesUsed) -> u8 {
        AesUsed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aeskeysz {
    #[doc = "AES-128"]
    AES_128 = 0x0,
    #[doc = "AES-192"]
    AES_192 = 0x01,
    #[doc = "AES-256"]
    AES_256 = 0x02,
    #[doc = "RFU (defaults to AES-128)"]
    RFU = 0x03,
}
impl Aeskeysz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aeskeysz {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aeskeysz {
    #[inline(always)]
    fn from(val: u8) -> Aeskeysz {
        Aeskeysz::from_bits(val)
    }
}
impl From<Aeskeysz> for u8 {
    #[inline(always)]
    fn from(val: Aeskeysz) -> u8 {
        Aeskeysz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BytesOrder {
    #[doc = "Normal"]
    NORMAL_ORDER = 0x0,
    #[doc = "Swapped"]
    SWAPPED_ORDER = 0x01,
}
impl BytesOrder {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BytesOrder {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BytesOrder {
    #[inline(always)]
    fn from(val: u8) -> BytesOrder {
        BytesOrder::from_bits(val)
    }
}
impl From<BytesOrder> for u8 {
    #[inline(always)]
    fn from(val: BytesOrder) -> u8 {
        BytesOrder::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cmd(u8);
impl Cmd {
    #[doc = "ECB mode"]
    pub const ECB: Self = Self(0x0);
    #[doc = "CTR mode"]
    pub const CTR: Self = Self(0x01);
    #[doc = "CBC mode"]
    pub const CBC: Self = Self(0x02);
    #[doc = "CBCMAC mode"]
    pub const CBCMAC: Self = Self(0x03);
    #[doc = "Key Wrap/Unwrap (128 bit key data)"]
    pub const WRAP_128_BIT: Self = Self(0x10);
    #[doc = "Key Wrap/Unwrap (256 bit key data)"]
    pub const WRAP_256_BIT: Self = Self(0x11);
}
impl Cmd {
    pub const fn from_bits(val: u8) -> Cmd {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Cmd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ECB"),
            0x01 => f.write_str("CTR"),
            0x02 => f.write_str("CBC"),
            0x03 => f.write_str("CBCMAC"),
            0x10 => f.write_str("WRAP_128_BIT"),
            0x11 => f.write_str("WRAP_256_BIT"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ECB"),
            0x01 => defmt::write!(f, "CTR"),
            0x02 => defmt::write!(f, "CBC"),
            0x03 => defmt::write!(f, "CBCMAC"),
            0x10 => defmt::write!(f, "WRAP_128_BIT"),
            0x11 => defmt::write!(f, "WRAP_256_BIT"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Cmd {
    #[inline(always)]
    fn from(val: u8) -> Cmd {
        Cmd::from_bits(val)
    }
}
impl From<Cmd> for u8 {
    #[inline(always)]
    fn from(val: Cmd) -> u8 {
        Cmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CryptoOp {
    #[doc = "AES"]
    AES = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "GFMUL(If Included)"]
    GFMUL = 0x03,
    #[doc = "SHA2 (If Included)"]
    SHA = 0x04,
    #[doc = "CMAC (If Included)"]
    CMAC = 0x05,
    #[doc = "others - RFU (Defaults to 1st available OP)"]
    OTHERS_6 = 0x06,
    #[doc = "others - RFU (Defaults to 1st available OP)"]
    OTHERS_7 = 0x07,
}
impl CryptoOp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CryptoOp {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CryptoOp {
    #[inline(always)]
    fn from(val: u8) -> CryptoOp {
        CryptoOp::from_bits(val)
    }
}
impl From<CryptoOp> for u8 {
    #[inline(always)]
    fn from(val: CryptoOp) -> u8 {
        CryptoOp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DatinFlush {
    #[doc = "Clr has no effect"]
    NO_EFFECT = 0x0,
    #[doc = "Set to start flush"]
    FLUSH = 0x01,
}
impl DatinFlush {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DatinFlush {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DatinFlush {
    #[inline(always)]
    fn from(val: u8) -> DatinFlush {
        DatinFlush::from_bits(val)
    }
}
impl From<DatinFlush> for u8 {
    #[inline(always)]
    fn from(val: DatinFlush) -> u8 {
        DatinFlush::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DatoutRes {
    #[doc = "END_UP"]
    END_UP = 0x0,
    #[doc = "START_UP"]
    START_UP = 0x01,
    #[doc = "TRIGGER_UP"]
    TRIGGER_UP = 0x02,
    #[doc = "NO_UP"]
    NO_UP = 0x03,
}
impl DatoutRes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DatoutRes {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DatoutRes {
    #[inline(always)]
    fn from(val: u8) -> DatoutRes {
        DatoutRes::from_bits(val)
    }
}
impl From<DatoutRes> for u8 {
    #[inline(always)]
    fn from(val: DatoutRes) -> u8 {
        DatoutRes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Decrypt {
    #[doc = "Encryption"]
    ENC = 0x0,
    #[doc = "Decryption"]
    DEC = 0x01,
}
impl Decrypt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Decrypt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Decrypt {
    #[inline(always)]
    fn from(val: u8) -> Decrypt {
        Decrypt::from_bits(val)
    }
}
impl From<Decrypt> for u8 {
    #[inline(always)]
    fn from(val: Decrypt) -> u8 {
        Decrypt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DesUsed {
    #[doc = "Dakar"]
    DAKAR = 0x0,
    #[doc = "Danube"]
    DANUBE = 0x01,
    #[doc = "Depicta"]
    DEPICTA = 0x02,
    #[doc = "Digi"]
    DIGI = 0x03,
    #[doc = "Date"]
    DATE = 0x04,
    #[doc = "Desert"]
    DESERT = 0x05,
    #[doc = "RFU"]
    RFU_6 = 0x06,
    #[doc = "RFU"]
    RFU_7 = 0x07,
    #[doc = "RFU"]
    RFU_8 = 0x08,
    #[doc = "RFU"]
    RFU_9 = 0x09,
    #[doc = "RFU"]
    RFU_10 = 0x0a,
    #[doc = "RFU"]
    RFU_11 = 0x0b,
    #[doc = "RFU"]
    RFU_12 = 0x0c,
    #[doc = "RFU"]
    RFU_13 = 0x0d,
    #[doc = "RFU"]
    RFU_14 = 0x0e,
    #[doc = "RFU"]
    RFU_15 = 0x0f,
}
impl DesUsed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DesUsed {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DesUsed {
    #[inline(always)]
    fn from(val: u8) -> DesUsed {
        DesUsed::from_bits(val)
    }
}
impl From<DesUsed> for u8 {
    #[inline(always)]
    fn from(val: DesUsed) -> u8 {
        DesUsed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    #[doc = "Error (all values other than 0x05 indicate ERROR)"]
    ERROR = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "No error"]
    NO_ERROR = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Error {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Error {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Error {
    #[inline(always)]
    fn from(val: u8) -> Error {
        Error::from_bits(val)
    }
}
impl From<Error> for u8 {
    #[inline(always)]
    fn from(val: Error) -> u8 {
        Error::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flush {
    #[doc = "Clr has no effect"]
    NO_EFFECT = 0x0,
    #[doc = "Set to start flush"]
    FLUSH = 0x01,
}
impl Flush {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flush {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flush {
    #[inline(always)]
    fn from(val: u8) -> Flush {
        Flush::from_bits(val)
    }
}
impl From<Flush> for u8 {
    #[inline(always)]
    fn from(val: Flush) -> u8 {
        Flush::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IncrCin {
    #[doc = "Carry-In for INCR is 1"]
    INCR_ONE = 0x0,
    #[doc = "Carry-In for INCR is overflow from previous INCR operation"]
    INCR_PREVIOUS = 0x01,
}
impl IncrCin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IncrCin {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IncrCin {
    #[inline(always)]
    fn from(val: u8) -> IncrCin {
        IncrCin::from_bits(val)
    }
}
impl From<IncrCin> for u8 {
    #[inline(always)]
    fn from(val: IncrCin) -> u8 {
        IncrCin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IncrMode {
    #[doc = "2**32 increment mode"]
    INCR_MODE_32 = 0x0,
    #[doc = "2**64 increment mode"]
    INCR_MODE_64 = 0x01,
    #[doc = "2**96 increment mode"]
    INCR_MODE_96 = 0x02,
    #[doc = "2**128 increment mode"]
    INCR_MODE_128 = 0x03,
}
impl IncrMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IncrMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IncrMode {
    #[inline(always)]
    fn from(val: u8) -> IncrMode {
        IncrMode::from_bits(val)
    }
}
impl From<IncrMode> for u8 {
    #[inline(always)]
    fn from(val: IncrMode) -> u8 {
        IncrMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Insel {
    #[doc = "DATIN\\[0\\]"]
    DATIN0 = 0x0,
    #[doc = "DATIN\\[1\\]*"]
    DATIN1 = 0x01,
    #[doc = "DATIN\\[2\\]*"]
    DATIN2 = 0x02,
    #[doc = "DATIN\\[3\\]*"]
    DATIN3 = 0x03,
    #[doc = "DATIN\\[0\\] ^ DATOUT"]
    DATIN0_DATOUT = 0x04,
    #[doc = "DATIN\\[1\\] ^ DATOUT*"]
    DATIN1_DATOUT = 0x05,
    #[doc = "DATIN\\[2\\] ^ DATOUT*"]
    DATIN2_DATOUT = 0x06,
    #[doc = "DATIN\\[3\\] ^ DATOUT*"]
    DATIN3_DATOUT = 0x07,
    #[doc = "DATOUT"]
    DATOUT = 0x08,
    #[doc = "others - DATIN\\[0\\] * - only if DATIN\\[num\\] exists, else \\[0\\]"]
    OTHERS_9 = 0x09,
    #[doc = "others - DATIN\\[0\\] * - only if DATIN\\[num\\] exists, else \\[0\\]"]
    OTHERS_10 = 0x0a,
    #[doc = "others - DATIN\\[0\\] * - only if DATIN\\[num\\] exists, else \\[0\\]"]
    OTHERS_11 = 0x0b,
    #[doc = "others - DATIN\\[0\\] * - only if DATIN\\[num\\] exists, else \\[0\\]"]
    OTHERS_12 = 0x0c,
    #[doc = "others - DATIN\\[0\\] * - only if DATIN\\[num\\] exists, else \\[0\\]"]
    OTHERS_13 = 0x0d,
    #[doc = "others - DATIN\\[0\\] * - only if DATIN\\[num\\] exists, else \\[0\\]"]
    OTHERS_14 = 0x0e,
    #[doc = "others - DATIN\\[0\\] * - only if DATIN\\[num\\] exists, else \\[0\\]"]
    OTHERS_15 = 0x0f,
}
impl Insel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Insel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Insel {
    #[inline(always)]
    fn from(val: u8) -> Insel {
        Insel::from_bits(val)
    }
}
impl From<Insel> for u8 {
    #[inline(always)]
    fn from(val: Insel) -> u8 {
        Insel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KeyFlush {
    #[doc = "Clr has no effect"]
    NO_EFFECT = 0x0,
    #[doc = "Set to start flush"]
    FLUSH = 0x01,
}
impl KeyFlush {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KeyFlush {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KeyFlush {
    #[inline(always)]
    fn from(val: u8) -> KeyFlush {
        KeyFlush::from_bits(val)
    }
}
impl From<KeyFlush> for u8 {
    #[inline(always)]
    fn from(val: KeyFlush) -> u8 {
        KeyFlush::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Milestone {
    #[doc = "PREL"]
    PREL = 0x0,
    #[doc = "BR"]
    BR = 0x01,
    #[doc = "SI"]
    SI = 0x02,
    #[doc = "GO"]
    GO = 0x03,
}
impl Milestone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Milestone {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Milestone {
    #[inline(always)]
    fn from(val: u8) -> Milestone {
        Milestone::from_bits(val)
    }
}
impl From<Milestone> for u8 {
    #[inline(always)]
    fn from(val: Milestone) -> u8 {
        Milestone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NoAutoInit {
    #[doc = "SHA automatic HASH initialisation"]
    SHA_INIT = 0x0,
    #[doc = "No SHA automatic HASH initialisation"]
    NO_SHA_INIT = 0x01,
}
impl NoAutoInit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NoAutoInit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NoAutoInit {
    #[inline(always)]
    fn from(val: u8) -> NoAutoInit {
        NoAutoInit::from_bits(val)
    }
}
impl From<NoAutoInit> for u8 {
    #[inline(always)]
    fn from(val: NoAutoInit) -> u8 {
        NoAutoInit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Outsel {
    #[doc = "DATOUT = 'Kernel Res'"]
    DATOUT_KER_RES = 0x0,
    #[doc = "DATOUT = 'Kernel Res' ^ DATIN\\[0\\]"]
    DATOUT_DATIN0 = 0x01,
    #[doc = "DATOUT = 'Kernel Res' ^ DATIN\\[1\\]*"]
    DATOUT_DATIN1 = 0x02,
    #[doc = "DATOUT = 'Kernel Res' ^ DATIN\\[2\\]*"]
    DATOUT_DATIN2 = 0x03,
    #[doc = "DATOUT = 'Kernel Res' ^DATIN\\[3\\]*"]
    DATOUT_DATIN3 = 0x04,
    #[doc = "others - DATOUT = 'Kernel Res' * - only if DATIN\\[num\\] exists, else \\[0\\]"]
    OTHERS_5 = 0x05,
    #[doc = "others - DATOUT = 'Kernel Res' * - only if DATIN\\[num\\] exists, else \\[0\\]"]
    OTHERS_6 = 0x06,
    #[doc = "others - DATOUT = 'Kernel Res' * - only if DATIN\\[num\\] exists, else \\[0\\]"]
    OTHERS_7 = 0x07,
}
impl Outsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Outsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Outsel {
    #[inline(always)]
    fn from(val: u8) -> Outsel {
        Outsel::from_bits(val)
    }
}
impl From<Outsel> for u8 {
    #[inline(always)]
    fn from(val: Outsel) -> u8 {
        Outsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rkey {
    #[doc = "DATOUT register bank"]
    DATOUT = 0x0,
    #[doc = "KEY register bank"]
    KEY = 0x01,
}
impl Rkey {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rkey {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rkey {
    #[inline(always)]
    fn from(val: u8) -> Rkey {
        Rkey::from_bits(val)
    }
}
impl From<Rkey> for u8 {
    #[inline(always)]
    fn from(val: Rkey) -> u8 {
        Rkey::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sha2CountEn {
    #[doc = "SHA operation DOES NOT increment COUNT"]
    COUNT = 0x0,
    #[doc = "SHA operation DOES increment count"]
    NO_COUNT = 0x01,
}
impl Sha2CountEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sha2CountEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sha2CountEn {
    #[inline(always)]
    fn from(val: u8) -> Sha2CountEn {
        Sha2CountEn::from_bits(val)
    }
}
impl From<Sha2CountEn> for u8 {
    #[inline(always)]
    fn from(val: Sha2CountEn) -> u8 {
        Sha2CountEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sha2Mode {
    #[doc = "SHA NORM Mode"]
    NORMAL = 0x0,
    #[doc = "SHA AUTO Mode"]
    AUTO = 0x01,
}
impl Sha2Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sha2Mode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sha2Mode {
    #[inline(always)]
    fn from(val: u8) -> Sha2Mode {
        Sha2Mode::from_bits(val)
    }
}
impl From<Sha2Mode> for u8 {
    #[inline(always)]
    fn from(val: Sha2Mode) -> u8 {
        Sha2Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sha2Size {
    #[doc = "SHA-224"]
    SHA_224 = 0x0,
    #[doc = "SHA-256"]
    SHA_256 = 0x01,
    #[doc = "SHA-384(or SHA-224 if SHA-256 only)"]
    SHA_384 = 0x02,
    #[doc = "SHA-512 (or SHA-256 if SHA-256 only)"]
    SHA_512 = 0x03,
}
impl Sha2Size {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sha2Size {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sha2Size {
    #[inline(always)]
    fn from(val: u8) -> Sha2Size {
        Sha2Size::from_bits(val)
    }
}
impl From<Sha2Size> for u8 {
    #[inline(always)]
    fn from(val: Sha2Size) -> u8 {
        Sha2Size::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sha2Stop {
    #[doc = "Keep running"]
    RUNNING = 0x0,
    #[doc = "Stop auto mode"]
    STOP = 0x01,
}
impl Sha2Stop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sha2Stop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sha2Stop {
    #[inline(always)]
    fn from(val: u8) -> Sha2Stop {
        Sha2Stop::from_bits(val)
    }
}
impl From<Sha2Stop> for u8 {
    #[inline(always)]
    fn from(val: Sha2Stop) -> u8 {
        Sha2Stop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smaskstep {
    #[doc = "SFRSEED increments every regbank access"]
    REGBANK = 0x0,
    #[doc = "SFRSEED increments every regbank access PLUS when SFRSEED in read"]
    REGBANK_AND_PLUS = 0x01,
}
impl Smaskstep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smaskstep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smaskstep {
    #[inline(always)]
    fn from(val: u8) -> Smaskstep {
        Smaskstep::from_bits(val)
    }
}
impl From<Smaskstep> for u8 {
    #[inline(always)]
    fn from(val: Smaskstep) -> u8 {
        Smaskstep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smasksw {
    #[doc = "SFR MASK output directly controlled by HW mask generator"]
    HW = 0x0,
    #[doc = "SFR MASK output directly controlled by SW"]
    SW = 0x01,
}
impl Smasksw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smasksw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smasksw {
    #[inline(always)]
    fn from(val: u8) -> Smasksw {
        Smasksw::from_bits(val)
    }
}
impl From<Smasksw> for u8 {
    #[inline(always)]
    fn from(val: Smasksw) -> u8 {
        Smasksw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Start {
    #[doc = "Clr has no effect"]
    NO_EFFECT = 0x0,
    #[doc = "Set to start operation"]
    START_OP = 0x01,
}
impl Start {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Start {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Start {
    #[inline(always)]
    fn from(val: u8) -> Start {
        Start::from_bits(val)
    }
}
impl From<Start> for u8 {
    #[inline(always)]
    fn from(val: Start) -> u8 {
        Start::to_bits(val)
    }
}
