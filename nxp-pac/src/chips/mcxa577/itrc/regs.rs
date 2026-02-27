#[doc = "Trigger Source IN0 to IN15 selector"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutSel(pub u32);
impl OutSel {
    #[doc = "Selects digital glitch detector as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in0_seln(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Selects digital glitch detector as a trigger source."]
    #[inline(always)]
    pub const fn set_in0_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Selects TDET event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in1_seln(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Selects TDET event as a trigger source."]
    #[inline(always)]
    pub const fn set_in1_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Selects Code Watchdog 0 event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in2_seln(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Selects Code Watchdog 0 event as a trigger source."]
    #[inline(always)]
    pub const fn set_in2_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Selects low-voltage event on VDD_CORE rail as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in4_seln(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Selects low-voltage event on VDD_CORE rail as a trigger source."]
    #[inline(always)]
    pub const fn set_in4_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Selects Watchdog 0 timer event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in5_seln(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Selects Watchdog 0 timer event as a trigger source."]
    #[inline(always)]
    pub const fn set_in5_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Selects Flash ECC mismatch event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in6_seln(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Selects Flash ECC mismatch event as a trigger source."]
    #[inline(always)]
    pub const fn set_in6_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Secure violation interrupt (Memory Block Checker (MBC) interrupt or secure AHB (AHBSC) matrix violation, and SYSCON XEN violation)."]
    #[must_use]
    #[inline(always)]
    pub const fn in7_seln(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Secure violation interrupt (Memory Block Checker (MBC) interrupt or secure AHB (AHBSC) matrix violation, and SYSCON XEN violation)."]
    #[inline(always)]
    pub const fn set_in7_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "SPC VDD_CORE glitch detect and VDD glitch detect (SPC_VDD_CORE_GLITCH_DETECT_SC\\[GLITCH_DETECT_FLAG\\] OR SPC_VDD _GLITCH_DETECT_SC\\[GLITCH_DETECT_FLAG\\])."]
    #[must_use]
    #[inline(always)]
    pub const fn in9_seln(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "SPC VDD_CORE glitch detect and VDD glitch detect (SPC_VDD_CORE_GLITCH_DETECT_SC\\[GLITCH_DETECT_FLAG\\] OR SPC_VDD _GLITCH_DETECT_SC\\[GLITCH_DETECT_FLAG\\])."]
    #[inline(always)]
    pub const fn set_in9_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Selects PKC error event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in10_seln(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Selects PKC error event as a trigger source."]
    #[inline(always)]
    pub const fn set_in10_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Selects Code Watchdog 1 event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in11_seln(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Selects Code Watchdog 1 event as a trigger source."]
    #[inline(always)]
    pub const fn set_in11_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Selects Watchdog 1 timer event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in12_seln(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Selects Watchdog 1 timer event as a trigger source."]
    #[inline(always)]
    pub const fn set_in12_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Selects FREQME out of range status output as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in13_seln(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "Selects FREQME out of range status output as a trigger source."]
    #[inline(always)]
    pub const fn set_in13_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "Selects software event 0 as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in14_seln(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Selects software event 0 as a trigger source."]
    #[inline(always)]
    pub const fn set_in14_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Selects software event 1 as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in15_seln(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Selects software event 1 as a trigger source."]
    #[inline(always)]
    pub const fn set_in15_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for OutSel {
    #[inline(always)]
    fn default() -> OutSel {
        OutSel(0)
    }
}
impl core::fmt::Debug for OutSel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OutSel")
            .field("in0_seln", &self.in0_seln())
            .field("in1_seln", &self.in1_seln())
            .field("in2_seln", &self.in2_seln())
            .field("in4_seln", &self.in4_seln())
            .field("in5_seln", &self.in5_seln())
            .field("in6_seln", &self.in6_seln())
            .field("in7_seln", &self.in7_seln())
            .field("in9_seln", &self.in9_seln())
            .field("in10_seln", &self.in10_seln())
            .field("in11_seln", &self.in11_seln())
            .field("in12_seln", &self.in12_seln())
            .field("in13_seln", &self.in13_seln())
            .field("in14_seln", &self.in14_seln())
            .field("in15_seln", &self.in15_seln())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OutSel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OutSel {{ in0_seln: {=u8:?}, in1_seln: {=u8:?}, in2_seln: {=u8:?}, in4_seln: {=u8:?}, in5_seln: {=u8:?}, in6_seln: {=u8:?}, in7_seln: {=u8:?}, in9_seln: {=u8:?}, in10_seln: {=u8:?}, in11_seln: {=u8:?}, in12_seln: {=u8:?}, in13_seln: {=u8:?}, in14_seln: {=u8:?}, in15_seln: {=u8:?} }}",
            self.in0_seln(),
            self.in1_seln(),
            self.in2_seln(),
            self.in4_seln(),
            self.in5_seln(),
            self.in6_seln(),
            self.in7_seln(),
            self.in9_seln(),
            self.in10_seln(),
            self.in11_seln(),
            self.in12_seln(),
            self.in13_seln(),
            self.in14_seln(),
            self.in15_seln()
        )
    }
}
#[doc = "Trigger Source IN16 to IN31 selector"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutSel1(pub u32);
impl OutSel1 {
    #[doc = "SPC VDD low voltage detect."]
    #[must_use]
    #[inline(always)]
    pub const fn in16_seln(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "SPC VDD low voltage detect."]
    #[inline(always)]
    pub const fn set_in16_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn in18_seln(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_in18_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Selects VDD_MAIN clock tamper output event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in20_seln(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Selects VDD_MAIN clock tamper output event as a trigger source."]
    #[inline(always)]
    pub const fn set_in20_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "SOCTRIM_ERR to SOCTRIM_ERR OR together."]
    #[must_use]
    #[inline(always)]
    pub const fn in25_seln(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "SOCTRIM_ERR to SOCTRIM_ERR OR together."]
    #[inline(always)]
    pub const fn set_in25_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "ERM multibit error interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn in27_seln(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "ERM multibit error interrupt."]
    #[inline(always)]
    pub const fn set_in27_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
}
impl Default for OutSel1 {
    #[inline(always)]
    fn default() -> OutSel1 {
        OutSel1(0)
    }
}
impl core::fmt::Debug for OutSel1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OutSel1")
            .field("in16_seln", &self.in16_seln())
            .field("in18_seln", &self.in18_seln())
            .field("in20_seln", &self.in20_seln())
            .field("in25_seln", &self.in25_seln())
            .field("in27_seln", &self.in27_seln())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OutSel1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OutSel1 {{ in16_seln: {=u8:?}, in18_seln: {=u8:?}, in20_seln: {=u8:?}, in25_seln: {=u8:?}, in27_seln: {=u8:?} }}",
            self.in16_seln(),
            self.in18_seln(),
            self.in20_seln(),
            self.in25_seln(),
            self.in27_seln()
        )
    }
}
#[doc = "Trigger source IN32 to IN47 selector"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutSel2(pub u32);
impl OutSel2 {
    #[doc = "DGDET error."]
    #[must_use]
    #[inline(always)]
    pub const fn in33_seln(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "DGDET error."]
    #[inline(always)]
    pub const fn set_in33_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Selects SPC VDD_CORE_HVD as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in34_seln(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Selects SPC VDD_CORE_HVD as a trigger source."]
    #[inline(always)]
    pub const fn set_in34_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "SPC VDD high voltage detect (SPC_VD_STAT\\[VDD_HVDF\\])."]
    #[must_use]
    #[inline(always)]
    pub const fn in35_seln(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "SPC VDD high voltage detect (SPC_VD_STAT\\[VDD_HVDF\\])."]
    #[inline(always)]
    pub const fn set_in35_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Selects VBAT OFF (vbat0. iso_vbat_to_1p8v OR VBAT Detector Status) as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in37_seln(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Selects VBAT OFF (vbat0. iso_vbat_to_1p8v OR VBAT Detector Status) as a trigger source."]
    #[inline(always)]
    pub const fn set_in37_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Selects FLEXSPI GCM error (FLEXSPI_INTR\\[AHBGCMERR\\]) as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in38_seln(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Selects FLEXSPI GCM error (FLEXSPI_INTR\\[AHBGCMERR\\]) as a trigger source."]
    #[inline(always)]
    pub const fn set_in38_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Selects UDF Error error as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in39_seln(&self) -> u16 {
        let val = (self.0 >> 14usize) & 0x3fff;
        val as u16
    }
    #[doc = "Selects UDF Error error as a trigger source."]
    #[inline(always)]
    pub const fn set_in39_seln(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 14usize)) | (((val as u32) & 0x3fff) << 14usize);
    }
    #[doc = "Selects TRNG HW Error (TRNG_INT_STATUS\\[HW_ERR\\]) as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in47_seln(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Selects TRNG HW Error (TRNG_INT_STATUS\\[HW_ERR\\]) as a trigger source."]
    #[inline(always)]
    pub const fn set_in47_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for OutSel2 {
    #[inline(always)]
    fn default() -> OutSel2 {
        OutSel2(0)
    }
}
impl core::fmt::Debug for OutSel2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OutSel2")
            .field("in33_seln", &self.in33_seln())
            .field("in34_seln", &self.in34_seln())
            .field("in35_seln", &self.in35_seln())
            .field("in37_seln", &self.in37_seln())
            .field("in38_seln", &self.in38_seln())
            .field("in39_seln", &self.in39_seln())
            .field("in47_seln", &self.in47_seln())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OutSel2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OutSel2 {{ in33_seln: {=u8:?}, in34_seln: {=u8:?}, in35_seln: {=u8:?}, in37_seln: {=u8:?}, in38_seln: {=u8:?}, in39_seln: {=u16:?}, in47_seln: {=u8:?} }}",
            self.in33_seln(),
            self.in34_seln(),
            self.in35_seln(),
            self.in37_seln(),
            self.in38_seln(),
            self.in39_seln(),
            self.in47_seln()
        )
    }
}
#[doc = "ITRC outputs and IN0 to IN15 Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "DGDET0 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn in0_status(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DGDET0 interrupt."]
    #[inline(always)]
    pub const fn set_in0_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TDET tamper output."]
    #[must_use]
    #[inline(always)]
    pub const fn in1_status(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TDET tamper output."]
    #[inline(always)]
    pub const fn set_in1_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Code Watchdog 0 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn in2_status(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Code Watchdog 0 interrupt."]
    #[inline(always)]
    pub const fn set_in2_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "SPC VDD_CORE_LVD detect."]
    #[must_use]
    #[inline(always)]
    pub const fn in4_status(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "SPC VDD_CORE_LVD detect."]
    #[inline(always)]
    pub const fn set_in4_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Watch Dog timer event occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn in5_status(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Watch Dog timer event occurred."]
    #[inline(always)]
    pub const fn set_in5_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Flash ECC mismatch event occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn in6_status(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Flash ECC mismatch event occurred."]
    #[inline(always)]
    pub const fn set_in6_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Secure violation interrupt (Memory Block Checker (MBC) interrupt or secure AHB (AHBSC) matrix violation, and SYSCON XEN violation)."]
    #[must_use]
    #[inline(always)]
    pub const fn in7_status(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Secure violation interrupt (Memory Block Checker (MBC) interrupt or secure AHB (AHBSC) matrix violation, and SYSCON XEN violation)."]
    #[inline(always)]
    pub const fn set_in7_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "SPC VDD_CORE glitch detect event occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn in9_status(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SPC VDD_CORE glitch detect event occurred."]
    #[inline(always)]
    pub const fn set_in9_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "PKC module detected an error event."]
    #[must_use]
    #[inline(always)]
    pub const fn in10_status(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PKC module detected an error event."]
    #[inline(always)]
    pub const fn set_in10_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Code Watchdog 1 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn in11_status(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Code Watchdog 1 interrupt."]
    #[inline(always)]
    pub const fn set_in11_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Watchdog 1 timer event interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn in112_status(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog 1 timer event interrupt."]
    #[inline(always)]
    pub const fn set_in112_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "FREQME out of range status output."]
    #[must_use]
    #[inline(always)]
    pub const fn in113_status(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "FREQME out of range status output."]
    #[inline(always)]
    pub const fn set_in113_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Software event 0 occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn in14_status(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Software event 0 occurred."]
    #[inline(always)]
    pub const fn set_in14_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Software event 1 occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn in15_status(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Software event 1 occurred."]
    #[inline(always)]
    pub const fn set_in15_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ITRC triggered ITRC_IRQ output."]
    #[must_use]
    #[inline(always)]
    pub const fn out0_status(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ITRC triggered ITRC_IRQ output."]
    #[inline(always)]
    pub const fn set_out0_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ITRC triggered SGI_UDF_RESET."]
    #[must_use]
    #[inline(always)]
    pub const fn out1_status(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "ITRC triggered SGI_UDF_RESET."]
    #[inline(always)]
    pub const fn set_out1_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "OUT2_STATUS can initiate PoR reset. CORE domain is reset completely. System domain must be reset completely, except the registers in SPC which reset by PoR. VBAT domain is not affected. ."]
    #[must_use]
    #[inline(always)]
    pub const fn out2_status(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "OUT2_STATUS can initiate PoR reset. CORE domain is reset completely. System domain must be reset completely, except the registers in SPC which reset by PoR. VBAT domain is not affected. ."]
    #[inline(always)]
    pub const fn set_out2_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "ITRC triggered RAM_ZEROIZE to clear retention and PKC RAM contents."]
    #[must_use]
    #[inline(always)]
    pub const fn out3_status(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "ITRC triggered RAM_ZEROIZE to clear retention and PKC RAM contents."]
    #[inline(always)]
    pub const fn set_out3_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "ITRC triggered CHIP_RESET to reset the chip after all other response process finished."]
    #[must_use]
    #[inline(always)]
    pub const fn out4_status(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "ITRC triggered CHIP_RESET to reset the chip after all other response process finished."]
    #[inline(always)]
    pub const fn set_out4_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ITRC triggered TMPR_OUT0 internal signal connected to various on-chip multiplexers."]
    #[must_use]
    #[inline(always)]
    pub const fn out5_status(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "ITRC triggered TMPR_OUT0 internal signal connected to various on-chip multiplexers."]
    #[inline(always)]
    pub const fn set_out5_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "ITRC triggered TMPR_OUT1 internal signal connected to various on-chip multiplexers."]
    #[must_use]
    #[inline(always)]
    pub const fn out6_status(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "ITRC triggered TMPR_OUT1 internal signal connected to various on-chip multiplexers."]
    #[inline(always)]
    pub const fn set_out6_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status")
            .field("in0_status", &self.in0_status())
            .field("in1_status", &self.in1_status())
            .field("in2_status", &self.in2_status())
            .field("in4_status", &self.in4_status())
            .field("in5_status", &self.in5_status())
            .field("in6_status", &self.in6_status())
            .field("in7_status", &self.in7_status())
            .field("in9_status", &self.in9_status())
            .field("in10_status", &self.in10_status())
            .field("in11_status", &self.in11_status())
            .field("in112_status", &self.in112_status())
            .field("in113_status", &self.in113_status())
            .field("in14_status", &self.in14_status())
            .field("in15_status", &self.in15_status())
            .field("out0_status", &self.out0_status())
            .field("out1_status", &self.out1_status())
            .field("out2_status", &self.out2_status())
            .field("out3_status", &self.out3_status())
            .field("out4_status", &self.out4_status())
            .field("out5_status", &self.out5_status())
            .field("out6_status", &self.out6_status())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{ in0_status: {=bool:?}, in1_status: {=bool:?}, in2_status: {=bool:?}, in4_status: {=bool:?}, in5_status: {=bool:?}, in6_status: {=bool:?}, in7_status: {=bool:?}, in9_status: {=bool:?}, in10_status: {=bool:?}, in11_status: {=bool:?}, in112_status: {=bool:?}, in113_status: {=bool:?}, in14_status: {=bool:?}, in15_status: {=bool:?}, out0_status: {=bool:?}, out1_status: {=bool:?}, out2_status: {=bool:?}, out3_status: {=bool:?}, out4_status: {=bool:?}, out5_status: {=bool:?}, out6_status: {=bool:?} }}",
            self.in0_status(),
            self.in1_status(),
            self.in2_status(),
            self.in4_status(),
            self.in5_status(),
            self.in6_status(),
            self.in7_status(),
            self.in9_status(),
            self.in10_status(),
            self.in11_status(),
            self.in112_status(),
            self.in113_status(),
            self.in14_status(),
            self.in15_status(),
            self.out0_status(),
            self.out1_status(),
            self.out2_status(),
            self.out3_status(),
            self.out4_status(),
            self.out5_status(),
            self.out6_status()
        )
    }
}
#[doc = "ITRC IN16 to IN47 Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status1(pub u32);
impl Status1 {
    #[doc = "SPC VDD low voltage detect event occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn in16_status(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SPC VDD low voltage detect event occurred."]
    #[inline(always)]
    pub const fn set_in16_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn in18_status(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_in18_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn in19_status(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_in19_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "VDD_MAIN clock tamper output event occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn in20_status(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VDD_MAIN clock tamper output event occurred."]
    #[inline(always)]
    pub const fn set_in20_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IN25_STATUS - SOCTRIM0_ERR to SOCTRIM7_ERR OR together; IN26_STATUS - Reserved IN27_STATUS - ERM Multibit Error Interrupt IN28_32_STATUS - reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn in32_25_status(&self) -> super::vals::In3225Status {
        let val = (self.0 >> 9usize) & 0xff;
        super::vals::In3225Status::from_bits(val as u8)
    }
    #[doc = "IN25_STATUS - SOCTRIM0_ERR to SOCTRIM7_ERR OR together; IN26_STATUS - Reserved IN27_STATUS - ERM Multibit Error Interrupt IN28_32_STATUS - reserved"]
    #[inline(always)]
    pub const fn set_in32_25_status(&mut self, val: super::vals::In3225Status) {
        self.0 = (self.0 & !(0xff << 9usize)) | (((val.to_bits() as u32) & 0xff) << 9usize);
    }
    #[doc = "DGDET error event occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn in33_status(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DGDET error event occurred."]
    #[inline(always)]
    pub const fn set_in33_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "SPC VDD_CORE high voltage detect event occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn in34_status(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "SPC VDD_CORE high voltage detect event occurred."]
    #[inline(always)]
    pub const fn set_in34_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "SPC VDD high voltage detect event occurred (SPC_VD_STAT\\[VDD_HVDF\\])."]
    #[must_use]
    #[inline(always)]
    pub const fn in35_status(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "SPC VDD high voltage detect event occurred (SPC_VD_STAT\\[VDD_HVDF\\])."]
    #[inline(always)]
    pub const fn set_in35_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "VBAT OFF (vbat0. iso_vbat_to_1p8v OR VBAT Detector Status)."]
    #[must_use]
    #[inline(always)]
    pub const fn in37_status(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "VBAT OFF (vbat0. iso_vbat_to_1p8v OR VBAT Detector Status)."]
    #[inline(always)]
    pub const fn set_in37_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "FLEXSPI GCM error (FLEXSPI_INTR\\[AHBGCMERR\\])."]
    #[must_use]
    #[inline(always)]
    pub const fn in38_status(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXSPI GCM error (FLEXSPI_INTR\\[AHBGCMERR\\])."]
    #[inline(always)]
    pub const fn set_in38_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "TRNG HW Error (TRNG_INT_STATUS\\[HW_ERR\\])"]
    #[must_use]
    #[inline(always)]
    pub const fn in47_status(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG HW Error (TRNG_INT_STATUS\\[HW_ERR\\])"]
    #[inline(always)]
    pub const fn set_in47_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Status1 {
    #[inline(always)]
    fn default() -> Status1 {
        Status1(0)
    }
}
impl core::fmt::Debug for Status1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status1")
            .field("in16_status", &self.in16_status())
            .field("in18_status", &self.in18_status())
            .field("in19_status", &self.in19_status())
            .field("in20_status", &self.in20_status())
            .field("in32_25_status", &self.in32_25_status())
            .field("in33_status", &self.in33_status())
            .field("in34_status", &self.in34_status())
            .field("in35_status", &self.in35_status())
            .field("in37_status", &self.in37_status())
            .field("in38_status", &self.in38_status())
            .field("in47_status", &self.in47_status())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status1 {{ in16_status: {=bool:?}, in18_status: {=bool:?}, in19_status: {=bool:?}, in20_status: {=bool:?}, in32_25_status: {:?}, in33_status: {=bool:?}, in34_status: {=bool:?}, in35_status: {=bool:?}, in37_status: {=bool:?}, in38_status: {=bool:?}, in47_status: {=bool:?} }}",
            self.in16_status(),
            self.in18_status(),
            self.in19_status(),
            self.in20_status(),
            self.in32_25_status(),
            self.in33_status(),
            self.in34_status(),
            self.in35_status(),
            self.in37_status(),
            self.in38_status(),
            self.in47_status()
        )
    }
}
#[doc = "Software event 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwEvent0(pub u32);
impl SwEvent0 {
    #[doc = "Trigger software event 0."]
    #[must_use]
    #[inline(always)]
    pub const fn trigger_sw_event_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Trigger software event 0."]
    #[inline(always)]
    pub const fn set_trigger_sw_event_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SwEvent0 {
    #[inline(always)]
    fn default() -> SwEvent0 {
        SwEvent0(0)
    }
}
impl core::fmt::Debug for SwEvent0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwEvent0")
            .field("trigger_sw_event_0", &self.trigger_sw_event_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwEvent0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SwEvent0 {{ trigger_sw_event_0: {=u32:?} }}",
            self.trigger_sw_event_0()
        )
    }
}
#[doc = "Software event 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwEvent1(pub u32);
impl SwEvent1 {
    #[doc = "Trigger software event 1."]
    #[must_use]
    #[inline(always)]
    pub const fn trigger_sw_event_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Trigger software event 1."]
    #[inline(always)]
    pub const fn set_trigger_sw_event_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SwEvent1 {
    #[inline(always)]
    fn default() -> SwEvent1 {
        SwEvent1(0)
    }
}
impl core::fmt::Debug for SwEvent1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwEvent1")
            .field("trigger_sw_event_1", &self.trigger_sw_event_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwEvent1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SwEvent1 {{ trigger_sw_event_1: {=u32:?} }}",
            self.trigger_sw_event_1()
        )
    }
}
