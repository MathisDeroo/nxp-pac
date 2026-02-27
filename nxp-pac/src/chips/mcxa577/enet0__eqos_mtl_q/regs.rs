#[doc = "Queue 0 Interrupt Control Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlQ0InterruptControlStatus(pub u32);
impl MtlQ0InterruptControlStatus {
    #[doc = "Transmit Queue Underflow Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn txunfis(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Queue Underflow Interrupt Status"]
    #[inline(always)]
    pub const fn set_txunfis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit Queue Underflow Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn txuie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Queue Underflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_txuie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Receive Queue Overflow Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn rxovfis(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Queue Overflow Interrupt Status"]
    #[inline(always)]
    pub const fn set_rxovfis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Receive Queue Overflow Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rxoie(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Queue Overflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rxoie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for MtlQ0InterruptControlStatus {
    #[inline(always)]
    fn default() -> MtlQ0InterruptControlStatus {
        MtlQ0InterruptControlStatus(0)
    }
}
impl core::fmt::Debug for MtlQ0InterruptControlStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlQ0InterruptControlStatus")
            .field("txunfis", &self.txunfis())
            .field("txuie", &self.txuie())
            .field("rxovfis", &self.rxovfis())
            .field("rxoie", &self.rxoie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlQ0InterruptControlStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlQ0InterruptControlStatus {{ txunfis: {=bool:?}, txuie: {=bool:?}, rxovfis: {=bool:?}, rxoie: {=bool:?} }}",
            self.txunfis(),
            self.txuie(),
            self.rxovfis(),
            self.rxoie()
        )
    }
}
#[doc = "Queue 0 Receive Debug"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlRxQ0Debug(pub u32);
impl MtlRxQ0Debug {
    #[doc = "MTL Rx Queue Write Controller Active Status"]
    #[must_use]
    #[inline(always)]
    pub const fn rwcsts(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "MTL Rx Queue Write Controller Active Status"]
    #[inline(always)]
    pub const fn set_rwcsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "MTL Rx Queue Read Controller State"]
    #[must_use]
    #[inline(always)]
    pub const fn rrcsts(&self) -> super::vals::Rrcsts {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Rrcsts::from_bits(val as u8)
    }
    #[doc = "MTL Rx Queue Read Controller State"]
    #[inline(always)]
    pub const fn set_rrcsts(&mut self, val: super::vals::Rrcsts) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "MTL Rx Queue Fill-Level Status"]
    #[must_use]
    #[inline(always)]
    pub const fn rxqsts(&self) -> super::vals::Rxqsts {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Rxqsts::from_bits(val as u8)
    }
    #[doc = "MTL Rx Queue Fill-Level Status"]
    #[inline(always)]
    pub const fn set_rxqsts(&mut self, val: super::vals::Rxqsts) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Number of Packets in Receive Queue"]
    #[must_use]
    #[inline(always)]
    pub const fn prxq(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x3fff;
        val as u16
    }
    #[doc = "Number of Packets in Receive Queue"]
    #[inline(always)]
    pub const fn set_prxq(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
    }
}
impl Default for MtlRxQ0Debug {
    #[inline(always)]
    fn default() -> MtlRxQ0Debug {
        MtlRxQ0Debug(0)
    }
}
impl core::fmt::Debug for MtlRxQ0Debug {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlRxQ0Debug")
            .field("rwcsts", &self.rwcsts())
            .field("rrcsts", &self.rrcsts())
            .field("rxqsts", &self.rxqsts())
            .field("prxq", &self.prxq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlRxQ0Debug {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlRxQ0Debug {{ rwcsts: {=bool:?}, rrcsts: {:?}, rxqsts: {:?}, prxq: {=u16:?} }}",
            self.rwcsts(),
            self.rrcsts(),
            self.rxqsts(),
            self.prxq()
        )
    }
}
#[doc = "Queue 0 Missed Packet and Overflow Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlRxQ0MissedPacketOverflowCnt(pub u32);
impl MtlRxQ0MissedPacketOverflowCnt {
    #[doc = "Overflow Packet Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn ovfpktcnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Overflow Packet Counter"]
    #[inline(always)]
    pub const fn set_ovfpktcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Overflow Counter Overflow Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn ovfcntovf(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Overflow Counter Overflow Bit"]
    #[inline(always)]
    pub const fn set_ovfcntovf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Missed Packet Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn mispktcnt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "Missed Packet Counter"]
    #[inline(always)]
    pub const fn set_mispktcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
    #[doc = "Missed Packet Counter Overflow Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn miscntovf(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Missed Packet Counter Overflow Bit"]
    #[inline(always)]
    pub const fn set_miscntovf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for MtlRxQ0MissedPacketOverflowCnt {
    #[inline(always)]
    fn default() -> MtlRxQ0MissedPacketOverflowCnt {
        MtlRxQ0MissedPacketOverflowCnt(0)
    }
}
impl core::fmt::Debug for MtlRxQ0MissedPacketOverflowCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlRxQ0MissedPacketOverflowCnt")
            .field("ovfpktcnt", &self.ovfpktcnt())
            .field("ovfcntovf", &self.ovfcntovf())
            .field("mispktcnt", &self.mispktcnt())
            .field("miscntovf", &self.miscntovf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlRxQ0MissedPacketOverflowCnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlRxQ0MissedPacketOverflowCnt {{ ovfpktcnt: {=u16:?}, ovfcntovf: {=bool:?}, mispktcnt: {=u16:?}, miscntovf: {=bool:?} }}",
            self.ovfpktcnt(),
            self.ovfcntovf(),
            self.mispktcnt(),
            self.miscntovf()
        )
    }
}
#[doc = "Queue 0 Receive Operation Mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlRxQ0OperationMode(pub u32);
impl MtlRxQ0OperationMode {
    #[doc = "Receive Queue Threshold Control"]
    #[must_use]
    #[inline(always)]
    pub const fn rtc(&self) -> super::vals::Rtc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Rtc::from_bits(val as u8)
    }
    #[doc = "Receive Queue Threshold Control"]
    #[inline(always)]
    pub const fn set_rtc(&mut self, val: super::vals::Rtc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Forward Undersized Good Packets"]
    #[must_use]
    #[inline(always)]
    pub const fn fup(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Forward Undersized Good Packets"]
    #[inline(always)]
    pub const fn set_fup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Forward Error Packets"]
    #[must_use]
    #[inline(always)]
    pub const fn fep(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Forward Error Packets"]
    #[inline(always)]
    pub const fn set_fep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Receive Queue Store and Forward"]
    #[must_use]
    #[inline(always)]
    pub const fn rsf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Queue Store and Forward"]
    #[inline(always)]
    pub const fn set_rsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Disable Dropping of TCP/IP Checksum Error Packets"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_tcp_ef(&self) -> super::vals::DisTcpEf {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::DisTcpEf::from_bits(val as u8)
    }
    #[doc = "Disable Dropping of TCP/IP Checksum Error Packets"]
    #[inline(always)]
    pub const fn set_dis_tcp_ef(&mut self, val: super::vals::DisTcpEf) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
}
impl Default for MtlRxQ0OperationMode {
    #[inline(always)]
    fn default() -> MtlRxQ0OperationMode {
        MtlRxQ0OperationMode(0)
    }
}
impl core::fmt::Debug for MtlRxQ0OperationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlRxQ0OperationMode")
            .field("rtc", &self.rtc())
            .field("fup", &self.fup())
            .field("fep", &self.fep())
            .field("rsf", &self.rsf())
            .field("dis_tcp_ef", &self.dis_tcp_ef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlRxQ0OperationMode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlRxQ0OperationMode {{ rtc: {:?}, fup: {=bool:?}, fep: {=bool:?}, rsf: {=bool:?}, dis_tcp_ef: {:?} }}",
            self.rtc(),
            self.fup(),
            self.fep(),
            self.rsf(),
            self.dis_tcp_ef()
        )
    }
}
#[doc = "Queue 0 Transmit Debug"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlTxQ0Debug(pub u32);
impl MtlTxQ0Debug {
    #[doc = "Transmit Queue in Pause"]
    #[must_use]
    #[inline(always)]
    pub const fn txqpaused(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Queue in Pause"]
    #[inline(always)]
    pub const fn set_txqpaused(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "MTL Tx Queue Read Controller Status"]
    #[must_use]
    #[inline(always)]
    pub const fn trcsts(&self) -> super::vals::Trcsts {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Trcsts::from_bits(val as u8)
    }
    #[doc = "MTL Tx Queue Read Controller Status"]
    #[inline(always)]
    pub const fn set_trcsts(&mut self, val: super::vals::Trcsts) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "MTL Tx Queue Write Controller Status"]
    #[must_use]
    #[inline(always)]
    pub const fn twcsts(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "MTL Tx Queue Write Controller Status"]
    #[inline(always)]
    pub const fn set_twcsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "MTL Tx Queue Not Empty Status"]
    #[must_use]
    #[inline(always)]
    pub const fn txqsts(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "MTL Tx Queue Not Empty Status"]
    #[inline(always)]
    pub const fn set_txqsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "MTL Tx Status FIFO Full Status"]
    #[must_use]
    #[inline(always)]
    pub const fn txstsfsts(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "MTL Tx Status FIFO Full Status"]
    #[inline(always)]
    pub const fn set_txstsfsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Number of Packets in the Transmit Queue"]
    #[must_use]
    #[inline(always)]
    pub const fn ptxq(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Number of Packets in the Transmit Queue"]
    #[inline(always)]
    pub const fn set_ptxq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Number of Status Words in Tx Status FIFO of Queue"]
    #[must_use]
    #[inline(always)]
    pub const fn stxstsf(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Number of Status Words in Tx Status FIFO of Queue"]
    #[inline(always)]
    pub const fn set_stxstsf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
}
impl Default for MtlTxQ0Debug {
    #[inline(always)]
    fn default() -> MtlTxQ0Debug {
        MtlTxQ0Debug(0)
    }
}
impl core::fmt::Debug for MtlTxQ0Debug {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlTxQ0Debug")
            .field("txqpaused", &self.txqpaused())
            .field("trcsts", &self.trcsts())
            .field("twcsts", &self.twcsts())
            .field("txqsts", &self.txqsts())
            .field("txstsfsts", &self.txstsfsts())
            .field("ptxq", &self.ptxq())
            .field("stxstsf", &self.stxstsf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlTxQ0Debug {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlTxQ0Debug {{ txqpaused: {=bool:?}, trcsts: {:?}, twcsts: {=bool:?}, txqsts: {=bool:?}, txstsfsts: {=bool:?}, ptxq: {=u8:?}, stxstsf: {=u8:?} }}",
            self.txqpaused(),
            self.trcsts(),
            self.twcsts(),
            self.txqsts(),
            self.txstsfsts(),
            self.ptxq(),
            self.stxstsf()
        )
    }
}
#[doc = "Queue 0 Transmit Operation Mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlTxQ0OperationMode(pub u32);
impl MtlTxQ0OperationMode {
    #[doc = "Timestamp Sub Seconds"]
    #[must_use]
    #[inline(always)]
    pub const fn ftq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp Sub Seconds"]
    #[inline(always)]
    pub const fn set_ftq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit Store and Forward"]
    #[must_use]
    #[inline(always)]
    pub const fn tsf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Store and Forward"]
    #[inline(always)]
    pub const fn set_tsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit Threshold Control"]
    #[must_use]
    #[inline(always)]
    pub const fn ttc(&self) -> super::vals::Ttc {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Ttc::from_bits(val as u8)
    }
    #[doc = "Transmit Threshold Control"]
    #[inline(always)]
    pub const fn set_ttc(&mut self, val: super::vals::Ttc) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
}
impl Default for MtlTxQ0OperationMode {
    #[inline(always)]
    fn default() -> MtlTxQ0OperationMode {
        MtlTxQ0OperationMode(0)
    }
}
impl core::fmt::Debug for MtlTxQ0OperationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlTxQ0OperationMode")
            .field("ftq", &self.ftq())
            .field("tsf", &self.tsf())
            .field("ttc", &self.ttc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlTxQ0OperationMode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlTxQ0OperationMode {{ ftq: {=bool:?}, tsf: {=bool:?}, ttc: {:?} }}",
            self.ftq(),
            self.tsf(),
            self.ttc()
        )
    }
}
#[doc = "Queue 0 Underflow Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlTxQ0Underflow(pub u32);
impl MtlTxQ0Underflow {
    #[doc = "Underflow Packet Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn uffrmcnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Underflow Packet Counter"]
    #[inline(always)]
    pub const fn set_uffrmcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Overflow Bit for Underflow Packet Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn ufcntovf(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Overflow Bit for Underflow Packet Counter"]
    #[inline(always)]
    pub const fn set_ufcntovf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for MtlTxQ0Underflow {
    #[inline(always)]
    fn default() -> MtlTxQ0Underflow {
        MtlTxQ0Underflow(0)
    }
}
impl core::fmt::Debug for MtlTxQ0Underflow {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlTxQ0Underflow")
            .field("uffrmcnt", &self.uffrmcnt())
            .field("ufcntovf", &self.ufcntovf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlTxQ0Underflow {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlTxQ0Underflow {{ uffrmcnt: {=u16:?}, ufcntovf: {=bool:?} }}",
            self.uffrmcnt(),
            self.ufcntovf()
        )
    }
}
