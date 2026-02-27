#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockRecoverEn {
    #[doc = "Disable"]
    DIS_CLK_RECOVER = 0x0,
    #[doc = "Enable"]
    EN_CLK_RECOVER = 0x01,
}
impl ClockRecoverEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClockRecoverEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClockRecoverEn {
    #[inline(always)]
    fn from(val: u8) -> ClockRecoverEn {
        ClockRecoverEn::from_bits(val)
    }
}
impl From<ClockRecoverEn> for u8 {
    #[inline(always)]
    fn from(val: ClockRecoverEn) -> u8 {
        ClockRecoverEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FsIsoB2bFixen {
    #[doc = "Disabled"]
    FS_ISO_B2B_FIXEN_0 = 0x0,
    #[doc = "Enabled"]
    FS_ISO_B2B_FIXEN_1 = 0x01,
}
impl FsIsoB2bFixen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FsIsoB2bFixen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FsIsoB2bFixen {
    #[inline(always)]
    fn from(val: u8) -> FsIsoB2bFixen {
        FsIsoB2bFixen::from_bits(val)
    }
}
impl From<FsIsoB2bFixen> for u8 {
    #[inline(always)]
    fn from(val: FsIsoB2bFixen) -> u8 {
        FsIsoB2bFixen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IrcEn {
    #[doc = "Disable"]
    DIS_IRC = 0x0,
    #[doc = "Enable"]
    EN_IRC = 0x01,
}
impl IrcEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IrcEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IrcEn {
    #[inline(always)]
    fn from(val: u8) -> IrcEn {
        IrcEn::from_bits(val)
    }
}
impl From<IrcEn> for u8 {
    #[inline(always)]
    fn from(val: IrcEn) -> u8 {
        IrcEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmAutoPhcd {
    #[doc = "Disable"]
    LPM_AUTO_PHCD0 = 0x0,
    #[doc = "Enable"]
    LPM_AUTO_PHCD1 = 0x01,
}
impl LpmAutoPhcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmAutoPhcd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmAutoPhcd {
    #[inline(always)]
    fn from(val: u8) -> LpmAutoPhcd {
        LpmAutoPhcd::from_bits(val)
    }
}
impl From<LpmAutoPhcd> for u8 {
    #[inline(always)]
    fn from(val: LpmAutoPhcd) -> u8 {
        LpmAutoPhcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmDevBeslthres {
    #[doc = "75 us, if LPM_ERRATA_EN = 1; 50 us, if LPM_ERRATA_EN = 0"]
    LPM_DEV_BESLTHRES0 = 0x0,
    #[doc = "100 us, if LPM_ERRATA_EN = 1; 125 us, if LPM_ERRATA_EN = 0"]
    LPM_DEV_BESLTHRES1 = 0x01,
    #[doc = "150 us, if LPM_ERRATA_EN = 1; 200 us, if LPM_ERRATA_EN = 0"]
    LPM_DEV_BESLTHRES2 = 0x02,
    #[doc = "250 us, if LPM_ERRATA_EN = 1; 275 us, if LPM_ERRATA_EN = 0"]
    LPM_DEV_BESLTHRES3 = 0x03,
    #[doc = "350 us, if LPM_ERRATA_EN = 1; 350 us, if LPM_ERRATA_EN = 0"]
    LPM_DEV_BESLTHRES4 = 0x04,
    #[doc = "450 us, if LPM_ERRATA_EN = 1; 425 us, if LPM_ERRATA_EN = 0"]
    LPM_DEV_BESLTHRES5 = 0x05,
    #[doc = "950 us, if LPM_ERRATA_EN = 1; 500 us, if LPM_ERRATA_EN = 0"]
    LPM_DEV_BESLTHRES6 = 0x06,
    #[doc = "1950 us, if LPM_ERRATA_EN = 1; 575 us, if LPM_ERRATA_EN = 0"]
    LPM_DEV_BESLTHRES7 = 0x07,
    #[doc = "2950 us, if LPM_ERRATA_EN = 1; 650 us, if LPM_ERRATA_EN = 0"]
    LPM_DEV_BESLTHRES8 = 0x08,
    #[doc = "3950 us, if LPM_ERRATA_EN = 1; 725 us, if LPM_ERRATA_EN = 0"]
    LPM_DEV_BESLTHRES9 = 0x09,
    #[doc = "4950 us, if LPM_ERRATA_EN = 1; 800 us, if LPM_ERRATA_EN = 0"]
    LPM_DEV_BESLTHRESA = 0x0a,
    #[doc = "5950 us, if LPM_ERRATA_EN = 1; 875 us, if LPM_ERRATA_EN = 0"]
    LPM_DEV_BESLTHRESB = 0x0b,
    #[doc = "6950 us, if LPM_ERRATA_EN = 1; 950 us, if LPM_ERRATA_EN = 0"]
    LPM_DEV_BESLTHRESC = 0x0c,
    #[doc = "7950 us, if LPM_ERRATA_EN = 1; 1025 us, if LPM_ERRATA_EN = 0"]
    LPM_DEV_BESLTHRESD = 0x0d,
    #[doc = "8950 us, if LPM_ERRATA_EN = 1; 1100 us, if LPM_ERRATA_EN = 0"]
    LPM_DEV_BESLTHRESE = 0x0e,
    #[doc = "9950 us, if LPM_ERRATA_EN = 1; 1175 us, if LPM_ERRATA_EN = 0"]
    LPM_DEV_BESLTHRESF = 0x0f,
}
impl LpmDevBeslthres {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmDevBeslthres {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmDevBeslthres {
    #[inline(always)]
    fn from(val: u8) -> LpmDevBeslthres {
        LpmDevBeslthres::from_bits(val)
    }
}
impl From<LpmDevBeslthres> for u8 {
    #[inline(always)]
    fn from(val: LpmDevBeslthres) -> u8 {
        LpmDevBeslthres::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmDevDp {
    #[doc = "Not pending"]
    LPM_DEV_DP0 = 0x0,
    #[doc = "Pending"]
    LPM_DEV_DP1 = 0x01,
}
impl LpmDevDp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmDevDp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmDevDp {
    #[inline(always)]
    fn from(val: u8) -> LpmDevDp {
        LpmDevDp::from_bits(val)
    }
}
impl From<LpmDevDp> for u8 {
    #[inline(always)]
    fn from(val: LpmDevDp) -> u8 {
        LpmDevDp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmDevLnkstrcvd {
    _RESERVED_0 = 0x0,
    #[doc = "L1 (Sleep mode)"]
    LPM_DEV_LNKSTRCVD1 = 0x01,
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
impl LpmDevLnkstrcvd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmDevLnkstrcvd {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmDevLnkstrcvd {
    #[inline(always)]
    fn from(val: u8) -> LpmDevLnkstrcvd {
        LpmDevLnkstrcvd::from_bits(val)
    }
}
impl From<LpmDevLnkstrcvd> for u8 {
    #[inline(always)]
    fn from(val: LpmDevLnkstrcvd) -> u8 {
        LpmDevLnkstrcvd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmDevRes {
    #[doc = "Fourth condition not needed"]
    LPM_DEV_RES0 = 0x0,
    #[doc = "Fourth condition needed"]
    LPM_DEV_RES1 = 0x01,
}
impl LpmDevRes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmDevRes {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmDevRes {
    #[inline(always)]
    fn from(val: u8) -> LpmDevRes {
        LpmDevRes::from_bits(val)
    }
}
impl From<LpmDevRes> for u8 {
    #[inline(always)]
    fn from(val: LpmDevRes) -> u8 {
        LpmDevRes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmDevRspsts {
    #[doc = "Invalid"]
    LPM_DEV_RSPSTS0 = 0x0,
    #[doc = "ACK"]
    LPM_DEV_RSPSTS1 = 0x01,
    #[doc = "NYET"]
    LPM_DEV_RSPSTS2 = 0x02,
    #[doc = "STALL"]
    LPM_DEV_RSPSTS3 = 0x03,
}
impl LpmDevRspsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmDevRspsts {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmDevRspsts {
    #[inline(always)]
    fn from(val: u8) -> LpmDevRspsts {
        LpmDevRspsts::from_bits(val)
    }
}
impl From<LpmDevRspsts> for u8 {
    #[inline(always)]
    fn from(val: LpmDevRspsts) -> u8 {
        LpmDevRspsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmDevRwkenrcvd {
    #[doc = "0"]
    LPM_DEV_RWKENRCVD0 = 0x0,
    #[doc = "1"]
    LPM_DEV_RWKENRCVD1 = 0x01,
}
impl LpmDevRwkenrcvd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmDevRwkenrcvd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmDevRwkenrcvd {
    #[inline(always)]
    fn from(val: u8) -> LpmDevRwkenrcvd {
        LpmDevRwkenrcvd::from_bits(val)
    }
}
impl From<LpmDevRwkenrcvd> for u8 {
    #[inline(always)]
    fn from(val: LpmDevRwkenrcvd) -> u8 {
        LpmDevRwkenrcvd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmEn {
    #[doc = "Disable"]
    LPM_EN0 = 0x0,
    #[doc = "Enable"]
    LPM_EN1 = 0x01,
}
impl LpmEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmEn {
    #[inline(always)]
    fn from(val: u8) -> LpmEn {
        LpmEn::from_bits(val)
    }
}
impl From<LpmEn> for u8 {
    #[inline(always)]
    fn from(val: LpmEn) -> u8 {
        LpmEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmEnEndpChk {
    #[doc = "Disabled"]
    LPM_EN_ENDP_CHK_0 = 0x0,
    #[doc = "Enabled"]
    LPM_EN_ENDP_CHK_1 = 0x01,
}
impl LpmEnEndpChk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmEnEndpChk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmEnEndpChk {
    #[inline(always)]
    fn from(val: u8) -> LpmEnEndpChk {
        LpmEnEndpChk::from_bits(val)
    }
}
impl From<LpmEnEndpChk> for u8 {
    #[inline(always)]
    fn from(val: LpmEnEndpChk) -> u8 {
        LpmEnEndpChk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmErrataEn {
    #[doc = "Disable"]
    LPM_ERRATA_EN0 = 0x0,
    #[doc = "Enable"]
    LPM_ERRATA_EN1 = 0x01,
}
impl LpmErrataEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmErrataEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmErrataEn {
    #[inline(always)]
    fn from(val: u8) -> LpmErrataEn {
        LpmErrataEn::from_bits(val)
    }
}
impl From<LpmErrataEn> for u8 {
    #[inline(always)]
    fn from(val: LpmErrataEn) -> u8 {
        LpmErrataEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmHstRwken {
    #[doc = "Disable"]
    LPM_HST_RWKEN0 = 0x0,
    #[doc = "Enable"]
    LPM_HST_RWKEN1 = 0x01,
}
impl LpmHstRwken {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmHstRwken {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmHstRwken {
    #[inline(always)]
    fn from(val: u8) -> LpmHstRwken {
        LpmHstRwken::from_bits(val)
    }
}
impl From<LpmHstRwken> for u8 {
    #[inline(always)]
    fn from(val: LpmHstRwken) -> u8 {
        LpmHstRwken::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmHstSend {
    #[doc = "LPM transaction did not happen or is complete"]
    LPM_HST_SEND0 = 0x0,
    #[doc = "LPM transaction is ongoing"]
    LPM_HST_SEND1 = 0x01,
}
impl LpmHstSend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmHstSend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmHstSend {
    #[inline(always)]
    fn from(val: u8) -> LpmHstSend {
        LpmHstSend::from_bits(val)
    }
}
impl From<LpmHstSend> for u8 {
    #[inline(always)]
    fn from(val: LpmHstSend) -> u8 {
        LpmHstSend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmHstStsrcvd {
    #[doc = "Invalid"]
    LPM_HST_STSRCVD0 = 0x0,
    #[doc = "ACK"]
    LPM_HST_STSRCVD1 = 0x01,
    #[doc = "NYET"]
    LPM_HST_STSRCVD2 = 0x02,
    #[doc = "STALL"]
    LPM_HST_STSRCVD3 = 0x03,
    #[doc = "Timeout"]
    LPM_HST_STSRCVD4 = 0x04,
    #[doc = "ERR"]
    LPM_HST_STSRCVD5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl LpmHstStsrcvd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmHstStsrcvd {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmHstStsrcvd {
    #[inline(always)]
    fn from(val: u8) -> LpmHstStsrcvd {
        LpmHstStsrcvd::from_bits(val)
    }
}
impl From<LpmHstStsrcvd> for u8 {
    #[inline(always)]
    fn from(val: LpmHstStsrcvd) -> u8 {
        LpmHstStsrcvd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmL1Active {
    #[doc = "Inactive"]
    LPM_L1_ACTIVE0 = 0x0,
    #[doc = "Active"]
    LPM_L1_ACTIVE1 = 0x01,
}
impl LpmL1Active {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmL1Active {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmL1Active {
    #[inline(always)]
    fn from(val: u8) -> LpmL1Active {
        LpmL1Active::from_bits(val)
    }
}
impl From<LpmL1Active> for u8 {
    #[inline(always)]
    fn from(val: LpmL1Active) -> u8 {
        LpmL1Active::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpmResumeok {
    #[doc = "Cannot resume"]
    LPM_RESUMEOK0 = 0x0,
    #[doc = "Can resume"]
    LPM_RESUMEOK1 = 0x01,
}
impl LpmResumeok {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpmResumeok {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpmResumeok {
    #[inline(always)]
    fn from(val: u8) -> LpmResumeok {
        LpmResumeok::from_bits(val)
    }
}
impl From<LpmResumeok> for u8 {
    #[inline(always)]
    fn from(val: LpmResumeok) -> u8 {
        LpmResumeok::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OverCurDis {
    #[doc = "Enable"]
    OVRCRNT_DETCT_EN = 0x0,
    #[doc = "Disable"]
    OVRCRNT_DETCT_DIS = 0x01,
}
impl OverCurDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OverCurDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OverCurDis {
    #[inline(always)]
    fn from(val: u8) -> OverCurDis {
        OverCurDis::from_bits(val)
    }
}
impl From<OverCurDis> for u8 {
    #[inline(always)]
    fn from(val: OverCurDis) -> u8 {
        OverCurDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OverCurPol {
    #[doc = "Active high"]
    ACTIVE_HI_OVRCRNT = 0x0,
    #[doc = "Active low"]
    ACTIVE_LOW_OVRCRNT = 0x01,
}
impl OverCurPol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OverCurPol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OverCurPol {
    #[inline(always)]
    fn from(val: u8) -> OverCurPol {
        OverCurPol::from_bits(val)
    }
}
impl From<OverCurPol> for u8 {
    #[inline(always)]
    fn from(val: OverCurPol) -> u8 {
        OverCurPol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OvfError {
    #[doc = "Interrupt did not occur"]
    INT_NO = 0x0,
    #[doc = "Unmasked interrupt occurred"]
    INT_YES = 0x01,
}
impl OvfError {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OvfError {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OvfError {
    #[inline(always)]
    fn from(val: u8) -> OvfError {
        OvfError::from_bits(val)
    }
}
impl From<OvfError> for u8 {
    #[inline(always)]
    fn from(val: OvfError) -> u8 {
        OvfError::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OvfErrorEn {
    #[doc = "The interrupt is masked"]
    MASK_OVF_ERR_INT = 0x0,
    #[doc = "The interrupt is enabled"]
    EN_OVF_ERR_INT = 0x01,
}
impl OvfErrorEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OvfErrorEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OvfErrorEn {
    #[inline(always)]
    fn from(val: u8) -> OvfErrorEn {
        OvfErrorEn::from_bits(val)
    }
}
impl From<OvfErrorEn> for u8 {
    #[inline(always)]
    fn from(val: OvfErrorEn) -> u8 {
        OvfErrorEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwrPol {
    #[doc = "Active low"]
    ACTIVE_LO_PMIC = 0x0,
    #[doc = "Active high"]
    ACTIVE_HI_PMIC = 0x01,
}
impl PwrPol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwrPol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwrPol {
    #[inline(always)]
    fn from(val: u8) -> PwrPol {
        PwrPol::from_bits(val)
    }
}
impl From<PwrPol> for u8 {
    #[inline(always)]
    fn from(val: PwrPol) -> u8 {
        PwrPol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RemoteWakeupEn {
    #[doc = "Disable"]
    REMOTE_WKUP_DIS = 0x0,
    #[doc = "Enable"]
    REMOTE_WKUP_EN = 0x01,
}
impl RemoteWakeupEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RemoteWakeupEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RemoteWakeupEn {
    #[inline(always)]
    fn from(val: u8) -> RemoteWakeupEn {
        RemoteWakeupEn::from_bits(val)
    }
}
impl From<RemoteWakeupEn> for u8 {
    #[inline(always)]
    fn from(val: RemoteWakeupEn) -> u8 {
        RemoteWakeupEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ResetResumeRoughEn {
    #[doc = "Always works in tracking phase after the first time rough phase, to track transition."]
    KEEP_TRIM_FINE_ON_RESET = 0x0,
    #[doc = "Go back to rough stage whenever a bus reset or bus resume occurs."]
    USE_IFR_TRIM_FINE_ON_RESET = 0x01,
}
impl ResetResumeRoughEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ResetResumeRoughEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ResetResumeRoughEn {
    #[inline(always)]
    fn from(val: u8) -> ResetResumeRoughEn {
        ResetResumeRoughEn::from_bits(val)
    }
}
impl From<ResetResumeRoughEn> for u8 {
    #[inline(always)]
    fn from(val: ResetResumeRoughEn) -> u8 {
        ResetResumeRoughEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RestartIfrtrimEn {
    #[doc = "Trim fine adjustment always works based on the previous updated trim fine value."]
    LOAD_TRIM_FINE_MID = 0x0,
    #[doc = "Trim fine restarts from the IFR trim value whenever you detect bus_reset or bus_resume or deassert module enable."]
    LOAD_TRIM_FINE_IFR = 0x01,
}
impl RestartIfrtrimEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RestartIfrtrimEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RestartIfrtrimEn {
    #[inline(always)]
    fn from(val: u8) -> RestartIfrtrimEn {
        RestartIfrtrimEn::from_bits(val)
    }
}
impl From<RestartIfrtrimEn> for u8 {
    #[inline(always)]
    fn from(val: RestartIfrtrimEn) -> u8 {
        RestartIfrtrimEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrimInitValSel {
    #[doc = "Mid-scale"]
    INIT_TRIM_FINE_MID = 0x0,
    #[doc = "IFR"]
    INIT_TRIM_FINE_IFR = 0x01,
}
impl TrimInitValSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrimInitValSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrimInitValSel {
    #[inline(always)]
    fn from(val: u8) -> TrimInitValSel {
        TrimInitValSel::from_bits(val)
    }
}
impl From<TrimInitValSel> for u8 {
    #[inline(always)]
    fn from(val: TrimInitValSel) -> u8 {
        TrimInitValSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusSourceSel {
    #[doc = "vbus_valid"]
    VBUS_VALID = 0x0,
    #[doc = "sess_valid"]
    SESS_VALID_1 = 0x01,
    #[doc = "sess_valid"]
    SESS_VALID_2 = 0x02,
    #[doc = "sess_valid"]
    SESS_VALID_3 = 0x03,
}
impl VbusSourceSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VbusSourceSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VbusSourceSel {
    #[inline(always)]
    fn from(val: u8) -> VbusSourceSel {
        VbusSourceSel::from_bits(val)
    }
}
impl From<VbusSourceSel> for u8 {
    #[inline(always)]
    fn from(val: VbusSourceSel) -> u8 {
        VbusSourceSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wie {
    #[doc = "Disable"]
    INT_DIS = 0x0,
    #[doc = "Enable"]
    INT_EN = 0x01,
}
impl Wie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wie {
    #[inline(always)]
    fn from(val: u8) -> Wie {
        Wie::from_bits(val)
    }
}
impl From<Wie> for u8 {
    #[inline(always)]
    fn from(val: Wie) -> u8 {
        Wie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WkupDpdmEn {
    #[doc = "Disable only when VBUS is invalid"]
    DPDM_WKUP_DIS = 0x0,
    #[doc = "Enable (default)"]
    DPDM_WKUP_EN = 0x01,
}
impl WkupDpdmEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WkupDpdmEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WkupDpdmEn {
    #[inline(always)]
    fn from(val: u8) -> WkupDpdmEn {
        WkupDpdmEn::from_bits(val)
    }
}
impl From<WkupDpdmEn> for u8 {
    #[inline(always)]
    fn from(val: WkupDpdmEn) -> u8 {
        WkupDpdmEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WkupIdEn {
    #[doc = "Disable"]
    WKUP_ID_DIS = 0x0,
    #[doc = "Enable"]
    WKUP_ID_EN = 0x01,
}
impl WkupIdEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WkupIdEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WkupIdEn {
    #[inline(always)]
    fn from(val: u8) -> WkupIdEn {
        WkupIdEn::from_bits(val)
    }
}
impl From<WkupIdEn> for u8 {
    #[inline(always)]
    fn from(val: WkupIdEn) -> u8 {
        WkupIdEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WkupSw {
    #[doc = "Inactive"]
    INACTIVE = 0x0,
    #[doc = "Force wake-up"]
    FORCE_WKUP = 0x01,
}
impl WkupSw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WkupSw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WkupSw {
    #[inline(always)]
    fn from(val: u8) -> WkupSw {
        WkupSw::from_bits(val)
    }
}
impl From<WkupSw> for u8 {
    #[inline(always)]
    fn from(val: WkupSw) -> u8 {
        WkupSw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WkupSwEn {
    #[doc = "Disable"]
    SW_WKUP_DIS = 0x0,
    #[doc = "Enable"]
    SW_WKUP_EN = 0x01,
}
impl WkupSwEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WkupSwEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WkupSwEn {
    #[inline(always)]
    fn from(val: u8) -> WkupSwEn {
        WkupSwEn::from_bits(val)
    }
}
impl From<WkupSwEn> for u8 {
    #[inline(always)]
    fn from(val: WkupSwEn) -> u8 {
        WkupSwEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WkupVbusEn {
    #[doc = "Disable"]
    WKUP_VBUS_DIS = 0x0,
    #[doc = "Enable"]
    WKUP_VBUS_EN = 0x01,
}
impl WkupVbusEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WkupVbusEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WkupVbusEn {
    #[inline(always)]
    fn from(val: u8) -> WkupVbusEn {
        WkupVbusEn::from_bits(val)
    }
}
impl From<WkupVbusEn> for u8 {
    #[inline(always)]
    fn from(val: WkupVbusEn) -> u8 {
        WkupVbusEn::to_bits(val)
    }
}
