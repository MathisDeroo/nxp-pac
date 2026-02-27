#[doc = "DMA Channel 0 Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0Control(pub u32);
impl DmaCh0Control {
    #[doc = "8xPBL mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pblx8(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "8xPBL mode"]
    #[inline(always)]
    pub const fn set_pblx8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Descriptor Skip Length"]
    #[must_use]
    #[inline(always)]
    pub const fn dsl(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "Descriptor Skip Length"]
    #[inline(always)]
    pub const fn set_dsl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "Descriptor Re-fetch when Own bit is 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dro(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Descriptor Re-fetch when Own bit is 0"]
    #[inline(always)]
    pub const fn set_dro(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for DmaCh0Control {
    #[inline(always)]
    fn default() -> DmaCh0Control {
        DmaCh0Control(0)
    }
}
impl core::fmt::Debug for DmaCh0Control {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0Control")
            .field("pblx8", &self.pblx8())
            .field("dsl", &self.dsl())
            .field("dro", &self.dro())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0Control {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0Control {{ pblx8: {=bool:?}, dsl: {=u8:?}, dro: {=bool:?} }}",
            self.pblx8(),
            self.dsl(),
            self.dro()
        )
    }
}
#[doc = "Channel 0 Current Application Receive Buffer Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0CurrentAppRxBuffer(pub u32);
impl DmaCh0CurrentAppRxBuffer {
    #[doc = "Application Receive Buffer Address Pointer"]
    #[must_use]
    #[inline(always)]
    pub const fn currbufaptr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Application Receive Buffer Address Pointer"]
    #[inline(always)]
    pub const fn set_currbufaptr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DmaCh0CurrentAppRxBuffer {
    #[inline(always)]
    fn default() -> DmaCh0CurrentAppRxBuffer {
        DmaCh0CurrentAppRxBuffer(0)
    }
}
impl core::fmt::Debug for DmaCh0CurrentAppRxBuffer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0CurrentAppRxBuffer")
            .field("currbufaptr", &self.currbufaptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0CurrentAppRxBuffer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0CurrentAppRxBuffer {{ currbufaptr: {=u32:?} }}",
            self.currbufaptr()
        )
    }
}
#[doc = "Channel 0 Current Application Receive Descriptor"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0CurrentAppRxDesc(pub u32);
impl DmaCh0CurrentAppRxDesc {
    #[doc = "Application Receive Descriptor Address Pointer"]
    #[must_use]
    #[inline(always)]
    pub const fn currdesaptr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Application Receive Descriptor Address Pointer"]
    #[inline(always)]
    pub const fn set_currdesaptr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DmaCh0CurrentAppRxDesc {
    #[inline(always)]
    fn default() -> DmaCh0CurrentAppRxDesc {
        DmaCh0CurrentAppRxDesc(0)
    }
}
impl core::fmt::Debug for DmaCh0CurrentAppRxDesc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0CurrentAppRxDesc")
            .field("currdesaptr", &self.currdesaptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0CurrentAppRxDesc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0CurrentAppRxDesc {{ currdesaptr: {=u32:?} }}",
            self.currdesaptr()
        )
    }
}
#[doc = "Channel 0 Current Application Transmit Buffer Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0CurrentAppTxBuffer(pub u32);
impl DmaCh0CurrentAppTxBuffer {
    #[doc = "Application Transmit Buffer Address Pointer"]
    #[must_use]
    #[inline(always)]
    pub const fn curtbufaptr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Application Transmit Buffer Address Pointer"]
    #[inline(always)]
    pub const fn set_curtbufaptr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DmaCh0CurrentAppTxBuffer {
    #[inline(always)]
    fn default() -> DmaCh0CurrentAppTxBuffer {
        DmaCh0CurrentAppTxBuffer(0)
    }
}
impl core::fmt::Debug for DmaCh0CurrentAppTxBuffer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0CurrentAppTxBuffer")
            .field("curtbufaptr", &self.curtbufaptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0CurrentAppTxBuffer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0CurrentAppTxBuffer {{ curtbufaptr: {=u32:?} }}",
            self.curtbufaptr()
        )
    }
}
#[doc = "Channel 0 Current Application Transmit Descriptor"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0CurrentAppTxDesc(pub u32);
impl DmaCh0CurrentAppTxDesc {
    #[doc = "Application Transmit Descriptor Address Pointer"]
    #[must_use]
    #[inline(always)]
    pub const fn curtdesaptr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Application Transmit Descriptor Address Pointer"]
    #[inline(always)]
    pub const fn set_curtdesaptr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DmaCh0CurrentAppTxDesc {
    #[inline(always)]
    fn default() -> DmaCh0CurrentAppTxDesc {
        DmaCh0CurrentAppTxDesc(0)
    }
}
impl core::fmt::Debug for DmaCh0CurrentAppTxDesc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0CurrentAppTxDesc")
            .field("curtdesaptr", &self.curtdesaptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0CurrentAppTxDesc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0CurrentAppTxDesc {{ curtdesaptr: {=u32:?} }}",
            self.curtdesaptr()
        )
    }
}
#[doc = "Channel 0 Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0InterruptEnable(pub u32);
impl DmaCh0InterruptEnable {
    #[doc = "Transmit Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit Stopped Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn txse(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Stopped Enable"]
    #[inline(always)]
    pub const fn set_txse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit Buffer Unavailable Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tbue(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Buffer Unavailable Enable"]
    #[inline(always)]
    pub const fn set_tbue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Receive Buffer Unavailable Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rbue(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Buffer Unavailable Enable"]
    #[inline(always)]
    pub const fn set_rbue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Receive Stopped Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rse(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Stopped Enable"]
    #[inline(always)]
    pub const fn set_rse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Receive Watchdog Timeout Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rwte(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Watchdog Timeout Enable"]
    #[inline(always)]
    pub const fn set_rwte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Early Transmit Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn etie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Early Transmit Interrupt Enable"]
    #[inline(always)]
    pub const fn set_etie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Early Receive Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn erie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Early Receive Interrupt Enable"]
    #[inline(always)]
    pub const fn set_erie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Fatal Bus Error Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fbee(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Fatal Bus Error Enable"]
    #[inline(always)]
    pub const fn set_fbee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Context Descriptor Error Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cdee(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Context Descriptor Error Enable"]
    #[inline(always)]
    pub const fn set_cdee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Abnormal Interrupt Summary Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn aie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Abnormal Interrupt Summary Enable"]
    #[inline(always)]
    pub const fn set_aie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Normal Interrupt Summary Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nie(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Normal Interrupt Summary Enable"]
    #[inline(always)]
    pub const fn set_nie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for DmaCh0InterruptEnable {
    #[inline(always)]
    fn default() -> DmaCh0InterruptEnable {
        DmaCh0InterruptEnable(0)
    }
}
impl core::fmt::Debug for DmaCh0InterruptEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0InterruptEnable")
            .field("tie", &self.tie())
            .field("txse", &self.txse())
            .field("tbue", &self.tbue())
            .field("rie", &self.rie())
            .field("rbue", &self.rbue())
            .field("rse", &self.rse())
            .field("rwte", &self.rwte())
            .field("etie", &self.etie())
            .field("erie", &self.erie())
            .field("fbee", &self.fbee())
            .field("cdee", &self.cdee())
            .field("aie", &self.aie())
            .field("nie", &self.nie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0InterruptEnable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0InterruptEnable {{ tie: {=bool:?}, txse: {=bool:?}, tbue: {=bool:?}, rie: {=bool:?}, rbue: {=bool:?}, rse: {=bool:?}, rwte: {=bool:?}, etie: {=bool:?}, erie: {=bool:?}, fbee: {=bool:?}, cdee: {=bool:?}, aie: {=bool:?}, nie: {=bool:?} }}",
            self.tie(),
            self.txse(),
            self.tbue(),
            self.rie(),
            self.rbue(),
            self.rse(),
            self.rwte(),
            self.etie(),
            self.erie(),
            self.fbee(),
            self.cdee(),
            self.aie(),
            self.nie()
        )
    }
}
#[doc = "Channel 0 Miss Frame Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0MissFrameCnt(pub u32);
impl DmaCh0MissFrameCnt {
    #[doc = "Dropped Packet Counters"]
    #[must_use]
    #[inline(always)]
    pub const fn mfc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Dropped Packet Counters"]
    #[inline(always)]
    pub const fn set_mfc(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Overflow status of the MFC Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn mfco(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Overflow status of the MFC Counter"]
    #[inline(always)]
    pub const fn set_mfco(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for DmaCh0MissFrameCnt {
    #[inline(always)]
    fn default() -> DmaCh0MissFrameCnt {
        DmaCh0MissFrameCnt(0)
    }
}
impl core::fmt::Debug for DmaCh0MissFrameCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0MissFrameCnt")
            .field("mfc", &self.mfc())
            .field("mfco", &self.mfco())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0MissFrameCnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0MissFrameCnt {{ mfc: {=u16:?}, mfco: {=bool:?} }}",
            self.mfc(),
            self.mfco()
        )
    }
}
#[doc = "DMA Channel 0 Receive Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0RxControl(pub u32);
impl DmaCh0RxControl {
    #[doc = "Start or Stop Receive"]
    #[must_use]
    #[inline(always)]
    pub const fn sr(&self) -> super::vals::Sr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sr::from_bits(val as u8)
    }
    #[doc = "Start or Stop Receive"]
    #[inline(always)]
    pub const fn set_sr(&mut self, val: super::vals::Sr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Buffer size Low"]
    #[must_use]
    #[inline(always)]
    pub const fn rbsz_x_0(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "Receive Buffer size Low"]
    #[inline(always)]
    pub const fn set_rbsz_x_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "Receive Buffer size High"]
    #[must_use]
    #[inline(always)]
    pub const fn rbsz_13_y(&self) -> u16 {
        let val = (self.0 >> 3usize) & 0x0fff;
        val as u16
    }
    #[doc = "Receive Buffer size High"]
    #[inline(always)]
    pub const fn set_rbsz_13_y(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 3usize)) | (((val as u32) & 0x0fff) << 3usize);
    }
    #[doc = "Receive Programmable Burst Length"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_pbl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Receive Programmable Burst Length"]
    #[inline(always)]
    pub const fn set_rx_pbl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Early Receive Interrupt Control"]
    #[must_use]
    #[inline(always)]
    pub const fn eric(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Early Receive Interrupt Control"]
    #[inline(always)]
    pub const fn set_eric(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Rx Packet Flush."]
    #[must_use]
    #[inline(always)]
    pub const fn rpf(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Rx Packet Flush."]
    #[inline(always)]
    pub const fn set_rpf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DmaCh0RxControl {
    #[inline(always)]
    fn default() -> DmaCh0RxControl {
        DmaCh0RxControl(0)
    }
}
impl core::fmt::Debug for DmaCh0RxControl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0RxControl")
            .field("sr", &self.sr())
            .field("rbsz_x_0", &self.rbsz_x_0())
            .field("rbsz_13_y", &self.rbsz_13_y())
            .field("rx_pbl", &self.rx_pbl())
            .field("eric", &self.eric())
            .field("rpf", &self.rpf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0RxControl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0RxControl {{ sr: {:?}, rbsz_x_0: {=u8:?}, rbsz_13_y: {=u16:?}, rx_pbl: {=u8:?}, eric: {=bool:?}, rpf: {=bool:?} }}",
            self.sr(),
            self.rbsz_x_0(),
            self.rbsz_13_y(),
            self.rx_pbl(),
            self.eric(),
            self.rpf()
        )
    }
}
#[doc = "Channel 0 Receive Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0RxControl2(pub u32);
impl DmaCh0RxControl2 {
    #[doc = "Receive Descriptor Ring Length"]
    #[must_use]
    #[inline(always)]
    pub const fn rdrl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Receive Descriptor Ring Length"]
    #[inline(always)]
    pub const fn set_rdrl(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Alternate Receive Buffer Size"]
    #[must_use]
    #[inline(always)]
    pub const fn arbs(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Alternate Receive Buffer Size"]
    #[inline(always)]
    pub const fn set_arbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for DmaCh0RxControl2 {
    #[inline(always)]
    fn default() -> DmaCh0RxControl2 {
        DmaCh0RxControl2(0)
    }
}
impl core::fmt::Debug for DmaCh0RxControl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0RxControl2")
            .field("rdrl", &self.rdrl())
            .field("arbs", &self.arbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0RxControl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0RxControl2 {{ rdrl: {=u16:?}, arbs: {=u8:?} }}",
            self.rdrl(),
            self.arbs()
        )
    }
}
#[doc = "Channel 0 Receive Descriptor List Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0RxDescListAddress(pub u32);
impl DmaCh0RxDescListAddress {
    #[doc = "Start of Receive List"]
    #[must_use]
    #[inline(always)]
    pub const fn rdesla(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Start of Receive List"]
    #[inline(always)]
    pub const fn set_rdesla(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for DmaCh0RxDescListAddress {
    #[inline(always)]
    fn default() -> DmaCh0RxDescListAddress {
        DmaCh0RxDescListAddress(0)
    }
}
impl core::fmt::Debug for DmaCh0RxDescListAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0RxDescListAddress")
            .field("rdesla", &self.rdesla())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0RxDescListAddress {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0RxDescListAddress {{ rdesla: {=u32:?} }}",
            self.rdesla()
        )
    }
}
#[doc = "Channel 0 Receive Descriptor Tail Pointer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0RxDescTailPointer(pub u32);
impl DmaCh0RxDescTailPointer {
    #[doc = "Receive Descriptor Tail Pointer"]
    #[must_use]
    #[inline(always)]
    pub const fn rdtp(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Receive Descriptor Tail Pointer"]
    #[inline(always)]
    pub const fn set_rdtp(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for DmaCh0RxDescTailPointer {
    #[inline(always)]
    fn default() -> DmaCh0RxDescTailPointer {
        DmaCh0RxDescTailPointer(0)
    }
}
impl core::fmt::Debug for DmaCh0RxDescTailPointer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0RxDescTailPointer")
            .field("rdtp", &self.rdtp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0RxDescTailPointer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0RxDescTailPointer {{ rdtp: {=u32:?} }}",
            self.rdtp()
        )
    }
}
#[doc = "Channel 0 Receive ERI Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0RxEriCnt(pub u32);
impl DmaCh0RxEriCnt {
    #[doc = "ERI Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn ecnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "ERI Counter"]
    #[inline(always)]
    pub const fn set_ecnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for DmaCh0RxEriCnt {
    #[inline(always)]
    fn default() -> DmaCh0RxEriCnt {
        DmaCh0RxEriCnt(0)
    }
}
impl core::fmt::Debug for DmaCh0RxEriCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0RxEriCnt")
            .field("ecnt", &self.ecnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0RxEriCnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DmaCh0RxEriCnt {{ ecnt: {=u16:?} }}", self.ecnt())
    }
}
#[doc = "Channel 0 Receive Interrupt Watchdog Timer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0RxInterruptWatchdogTimer(pub u32);
impl DmaCh0RxInterruptWatchdogTimer {
    #[doc = "Receive Interrupt Watchdog Timer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn rwt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Receive Interrupt Watchdog Timer Count"]
    #[inline(always)]
    pub const fn set_rwt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Rx Idle Timer Window"]
    #[must_use]
    #[inline(always)]
    pub const fn itw(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Rx Idle Timer Window"]
    #[inline(always)]
    pub const fn set_itw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Receive Interrupt Watchdog Timer Count Units"]
    #[must_use]
    #[inline(always)]
    pub const fn rwtu(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Receive Interrupt Watchdog Timer Count Units"]
    #[inline(always)]
    pub const fn set_rwtu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Rx Idle Timer Count Units"]
    #[must_use]
    #[inline(always)]
    pub const fn itcu(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Rx Idle Timer Count Units"]
    #[inline(always)]
    pub const fn set_itcu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Receive Byte Count Threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn rbct(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x03ff;
        val as u16
    }
    #[doc = "Receive Byte Count Threshold."]
    #[inline(always)]
    pub const fn set_rbct(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 20usize)) | (((val as u32) & 0x03ff) << 20usize);
    }
    #[doc = "Packet Count Interrupt Select."]
    #[must_use]
    #[inline(always)]
    pub const fn psel(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Packet Count Interrupt Select."]
    #[inline(always)]
    pub const fn set_psel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DmaCh0RxInterruptWatchdogTimer {
    #[inline(always)]
    fn default() -> DmaCh0RxInterruptWatchdogTimer {
        DmaCh0RxInterruptWatchdogTimer(0)
    }
}
impl core::fmt::Debug for DmaCh0RxInterruptWatchdogTimer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0RxInterruptWatchdogTimer")
            .field("rwt", &self.rwt())
            .field("itw", &self.itw())
            .field("rwtu", &self.rwtu())
            .field("itcu", &self.itcu())
            .field("rbct", &self.rbct())
            .field("psel", &self.psel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0RxInterruptWatchdogTimer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0RxInterruptWatchdogTimer {{ rwt: {=u8:?}, itw: {=u8:?}, rwtu: {=u8:?}, itcu: {=u8:?}, rbct: {=u16:?}, psel: {=bool:?} }}",
            self.rwt(),
            self.itw(),
            self.rwtu(),
            self.itcu(),
            self.rbct(),
            self.psel()
        )
    }
}
#[doc = "Channel 0 Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0Status(pub u32);
impl DmaCh0Status {
    #[doc = "Transmit Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ti(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Interrupt"]
    #[inline(always)]
    pub const fn set_ti(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit Process Stopped"]
    #[must_use]
    #[inline(always)]
    pub const fn tps(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Process Stopped"]
    #[inline(always)]
    pub const fn set_tps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit Buffer Unavailable"]
    #[must_use]
    #[inline(always)]
    pub const fn tbu(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Buffer Unavailable"]
    #[inline(always)]
    pub const fn set_tbu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ri(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Interrupt"]
    #[inline(always)]
    pub const fn set_ri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Receive Buffer Unavailable"]
    #[must_use]
    #[inline(always)]
    pub const fn rbu(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Buffer Unavailable"]
    #[inline(always)]
    pub const fn set_rbu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Receive Process Stopped"]
    #[must_use]
    #[inline(always)]
    pub const fn rps(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Process Stopped"]
    #[inline(always)]
    pub const fn set_rps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Receive Watchdog Timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn rwt(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Watchdog Timeout"]
    #[inline(always)]
    pub const fn set_rwt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Early Transmit Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn eti(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Early Transmit Interrupt"]
    #[inline(always)]
    pub const fn set_eti(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Early Receive Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn eri(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Early Receive Interrupt"]
    #[inline(always)]
    pub const fn set_eri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Fatal Bus Error"]
    #[must_use]
    #[inline(always)]
    pub const fn fbe(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Fatal Bus Error"]
    #[inline(always)]
    pub const fn set_fbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Context Descriptor Error"]
    #[must_use]
    #[inline(always)]
    pub const fn cde(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Context Descriptor Error"]
    #[inline(always)]
    pub const fn set_cde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Abnormal Interrupt Summary"]
    #[must_use]
    #[inline(always)]
    pub const fn ais(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Abnormal Interrupt Summary"]
    #[inline(always)]
    pub const fn set_ais(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Normal Interrupt Summary"]
    #[must_use]
    #[inline(always)]
    pub const fn nis(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Normal Interrupt Summary"]
    #[inline(always)]
    pub const fn set_nis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Tx DMA Error Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn teb(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Tx DMA Error Bits"]
    #[inline(always)]
    pub const fn set_teb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Rx DMA Error Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn reb(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x07;
        val as u8
    }
    #[doc = "Rx DMA Error Bits"]
    #[inline(always)]
    pub const fn set_reb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
    }
}
impl Default for DmaCh0Status {
    #[inline(always)]
    fn default() -> DmaCh0Status {
        DmaCh0Status(0)
    }
}
impl core::fmt::Debug for DmaCh0Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0Status")
            .field("ti", &self.ti())
            .field("tps", &self.tps())
            .field("tbu", &self.tbu())
            .field("ri", &self.ri())
            .field("rbu", &self.rbu())
            .field("rps", &self.rps())
            .field("rwt", &self.rwt())
            .field("eti", &self.eti())
            .field("eri", &self.eri())
            .field("fbe", &self.fbe())
            .field("cde", &self.cde())
            .field("ais", &self.ais())
            .field("nis", &self.nis())
            .field("teb", &self.teb())
            .field("reb", &self.reb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0Status {{ ti: {=bool:?}, tps: {=bool:?}, tbu: {=bool:?}, ri: {=bool:?}, rbu: {=bool:?}, rps: {=bool:?}, rwt: {=bool:?}, eti: {=bool:?}, eri: {=bool:?}, fbe: {=bool:?}, cde: {=bool:?}, ais: {=bool:?}, nis: {=bool:?}, teb: {=u8:?}, reb: {=u8:?} }}",
            self.ti(),
            self.tps(),
            self.tbu(),
            self.ri(),
            self.rbu(),
            self.rps(),
            self.rwt(),
            self.eti(),
            self.eri(),
            self.fbe(),
            self.cde(),
            self.ais(),
            self.nis(),
            self.teb(),
            self.reb()
        )
    }
}
#[doc = "DMA Channel 0 Transmit Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0TxControl(pub u32);
impl DmaCh0TxControl {
    #[doc = "Start or Stop Transmission Command"]
    #[must_use]
    #[inline(always)]
    pub const fn st(&self) -> super::vals::St {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::St::from_bits(val as u8)
    }
    #[doc = "Start or Stop Transmission Command"]
    #[inline(always)]
    pub const fn set_st(&mut self, val: super::vals::St) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Operate on Second Packet"]
    #[must_use]
    #[inline(always)]
    pub const fn osf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Operate on Second Packet"]
    #[inline(always)]
    pub const fn set_osf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmit Programmable Burst Length"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_pbl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Transmit Programmable Burst Length"]
    #[inline(always)]
    pub const fn set_tx_pbl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Early Transmit Interrupt Control"]
    #[must_use]
    #[inline(always)]
    pub const fn etic(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Early Transmit Interrupt Control"]
    #[inline(always)]
    pub const fn set_etic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for DmaCh0TxControl {
    #[inline(always)]
    fn default() -> DmaCh0TxControl {
        DmaCh0TxControl(0)
    }
}
impl core::fmt::Debug for DmaCh0TxControl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0TxControl")
            .field("st", &self.st())
            .field("osf", &self.osf())
            .field("tx_pbl", &self.tx_pbl())
            .field("etic", &self.etic())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0TxControl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0TxControl {{ st: {:?}, osf: {=bool:?}, tx_pbl: {=u8:?}, etic: {=bool:?} }}",
            self.st(),
            self.osf(),
            self.tx_pbl(),
            self.etic()
        )
    }
}
#[doc = "Channel 0 Transmit Descriptor List Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0TxDescListAddress(pub u32);
impl DmaCh0TxDescListAddress {
    #[doc = "Start of Transmit List"]
    #[must_use]
    #[inline(always)]
    pub const fn tdesla(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Start of Transmit List"]
    #[inline(always)]
    pub const fn set_tdesla(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for DmaCh0TxDescListAddress {
    #[inline(always)]
    fn default() -> DmaCh0TxDescListAddress {
        DmaCh0TxDescListAddress(0)
    }
}
impl core::fmt::Debug for DmaCh0TxDescListAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0TxDescListAddress")
            .field("tdesla", &self.tdesla())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0TxDescListAddress {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0TxDescListAddress {{ tdesla: {=u32:?} }}",
            self.tdesla()
        )
    }
}
#[doc = "Channel 0 Transmit Descriptor Ring Length"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0TxDescRingLength(pub u32);
impl DmaCh0TxDescRingLength {
    #[doc = "Transmit Descriptor Ring Length"]
    #[must_use]
    #[inline(always)]
    pub const fn tdrl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Transmit Descriptor Ring Length"]
    #[inline(always)]
    pub const fn set_tdrl(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for DmaCh0TxDescRingLength {
    #[inline(always)]
    fn default() -> DmaCh0TxDescRingLength {
        DmaCh0TxDescRingLength(0)
    }
}
impl core::fmt::Debug for DmaCh0TxDescRingLength {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0TxDescRingLength")
            .field("tdrl", &self.tdrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0TxDescRingLength {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0TxDescRingLength {{ tdrl: {=u16:?} }}",
            self.tdrl()
        )
    }
}
#[doc = "Channel 0 Transmit Descriptor Tail Pointer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0TxDescTailPointer(pub u32);
impl DmaCh0TxDescTailPointer {
    #[doc = "Transmit Descriptor Tail Pointer"]
    #[must_use]
    #[inline(always)]
    pub const fn tdtp(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Transmit Descriptor Tail Pointer"]
    #[inline(always)]
    pub const fn set_tdtp(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for DmaCh0TxDescTailPointer {
    #[inline(always)]
    fn default() -> DmaCh0TxDescTailPointer {
        DmaCh0TxDescTailPointer(0)
    }
}
impl core::fmt::Debug for DmaCh0TxDescTailPointer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0TxDescTailPointer")
            .field("tdtp", &self.tdtp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0TxDescTailPointer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0TxDescTailPointer {{ tdtp: {=u32:?} }}",
            self.tdtp()
        )
    }
}
