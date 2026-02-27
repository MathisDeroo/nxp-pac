#[doc = "no description available"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enet {
    ptr: *mut u8,
}
unsafe impl Send for Enet {}
unsafe impl Sync for Enet {}
impl Enet {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "MAC Configuration"]
    #[inline(always)]
    pub const fn mac_configuration(
        self,
    ) -> crate::common::Reg<regs::MacConfiguration, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "MAC Extended Configuration"]
    #[inline(always)]
    pub const fn mac_ext_configuration(
        self,
    ) -> crate::common::Reg<regs::MacExtConfiguration, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "MAC Packet Filter"]
    #[inline(always)]
    pub const fn mac_packet_filter(
        self,
    ) -> crate::common::Reg<regs::MacPacketFilter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Watchdog and Jabber Timeout"]
    #[inline(always)]
    pub const fn mac_wd_jb_timeout(
        self,
    ) -> crate::common::Reg<regs::MacWdJbTimeout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "VLAN Tag"]
    #[inline(always)]
    pub const fn mac_vlan_tag(self) -> crate::common::Reg<regs::MacVlanTag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "VLAN Tag Inclusion or Replacement"]
    #[inline(always)]
    pub const fn mac_vlan_incl(self) -> crate::common::Reg<regs::MacVlanIncl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Inner VLAN Tag Inclusion or Replacement"]
    #[inline(always)]
    pub const fn mac_inner_vlan_incl(
        self,
    ) -> crate::common::Reg<regs::MacInnerVlanIncl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Queue 0 Transmit Flow Control"]
    #[inline(always)]
    pub const fn mac_q0_tx_flow_ctrl(
        self,
    ) -> crate::common::Reg<regs::MacQ0TxFlowCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Receive Flow Control"]
    #[inline(always)]
    pub const fn mac_rx_flow_ctrl(
        self,
    ) -> crate::common::Reg<regs::MacRxFlowCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub const fn mac_interrupt_status(
        self,
    ) -> crate::common::Reg<regs::MacInterruptStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub const fn mac_interrupt_enable(
        self,
    ) -> crate::common::Reg<regs::MacInterruptEnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Receive Transmit Status"]
    #[inline(always)]
    pub const fn mac_rx_tx_status(
        self,
    ) -> crate::common::Reg<regs::MacRxTxStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "PMT Control and Status"]
    #[inline(always)]
    pub const fn mac_pmt_control_status(
        self,
    ) -> crate::common::Reg<regs::MacPmtControlStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Version"]
    #[inline(always)]
    pub const fn mac_version(self) -> crate::common::Reg<regs::MacVersion, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Debug"]
    #[inline(always)]
    pub const fn mac_debug(self) -> crate::common::Reg<regs::MacDebug, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "HW Features 0"]
    #[inline(always)]
    pub const fn mac_hw_feature0(
        self,
    ) -> crate::common::Reg<regs::MacHwFeature0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "HW Features 1"]
    #[inline(always)]
    pub const fn mac_hw_feature1(
        self,
    ) -> crate::common::Reg<regs::MacHwFeature1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "HW Features 2"]
    #[inline(always)]
    pub const fn mac_hw_feature2(
        self,
    ) -> crate::common::Reg<regs::MacHwFeature2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "HW Features 3"]
    #[inline(always)]
    pub const fn mac_hw_feature3(
        self,
    ) -> crate::common::Reg<regs::MacHwFeature3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "HW Features 4"]
    #[inline(always)]
    pub const fn mac_hw_feature4(
        self,
    ) -> crate::common::Reg<regs::MacHwFeature4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "MDIO Address"]
    #[inline(always)]
    pub const fn mac_mdio_address(
        self,
    ) -> crate::common::Reg<regs::MacMdioAddress, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "MDIO Data"]
    #[inline(always)]
    pub const fn mac_mdio_data(self) -> crate::common::Reg<regs::MacMdioData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "CSR Software Control"]
    #[inline(always)]
    pub const fn mac_csr_sw_ctrl(
        self,
    ) -> crate::common::Reg<regs::MacCsrSwCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "MAC Address0 High"]
    #[inline(always)]
    pub const fn mac_address0_high(
        self,
    ) -> crate::common::Reg<regs::MacAddress0High, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "MAC Address0 Low"]
    #[inline(always)]
    pub const fn mac_address0_low(
        self,
    ) -> crate::common::Reg<regs::MacAddress0Low, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "Timestamp Control"]
    #[inline(always)]
    pub const fn mac_timestamp_control(
        self,
    ) -> crate::common::Reg<regs::MacTimestampControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b00usize) as _) }
    }
    #[doc = "Sub Second Increment"]
    #[inline(always)]
    pub const fn mac_sub_second_increment(
        self,
    ) -> crate::common::Reg<regs::MacSubSecondIncrement, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b04usize) as _) }
    }
    #[doc = "System Time Seconds"]
    #[inline(always)]
    pub const fn mac_system_time_seconds(
        self,
    ) -> crate::common::Reg<regs::MacSystemTimeSeconds, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b08usize) as _) }
    }
    #[doc = "System Time Nanoseconds"]
    #[inline(always)]
    pub const fn mac_system_time_nanoseconds(
        self,
    ) -> crate::common::Reg<regs::MacSystemTimeNanoseconds, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b0cusize) as _) }
    }
    #[doc = "System Time Seconds Update"]
    #[inline(always)]
    pub const fn mac_system_time_seconds_update(
        self,
    ) -> crate::common::Reg<regs::MacSystemTimeSecondsUpdate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b10usize) as _) }
    }
    #[doc = "MAC System Time Nanoseconds Update"]
    #[inline(always)]
    pub const fn mac_system_time_nanoseconds_update(
        self,
    ) -> crate::common::Reg<regs::MacSystemTimeNanosecondsUpdate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b14usize) as _) }
    }
    #[doc = "Timestamp Addend"]
    #[inline(always)]
    pub const fn mac_timestamp_addend(
        self,
    ) -> crate::common::Reg<regs::MacTimestampAddend, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b18usize) as _) }
    }
    #[doc = "Timestamp Status"]
    #[inline(always)]
    pub const fn mac_timestamp_status(
        self,
    ) -> crate::common::Reg<regs::MacTimestampStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b20usize) as _) }
    }
    #[doc = "Receive Domain TIme Increment"]
    #[inline(always)]
    pub const fn mac_rx_domain_time_incr(
        self,
    ) -> crate::common::Reg<regs::MacRxDomainTimeIncr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b24usize) as _) }
    }
    #[doc = "Transmit Domain TIme Increment"]
    #[inline(always)]
    pub const fn mac_tx_domain_time_incr(
        self,
    ) -> crate::common::Reg<regs::MacTxDomainTimeIncr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b28usize) as _) }
    }
    #[doc = "Transmit Timestamp Status Nanoseconds"]
    #[inline(always)]
    pub const fn mac_tx_timestamp_status_nanoseconds(
        self,
    ) -> crate::common::Reg<regs::MacTxTimestampStatusNanoseconds, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b30usize) as _) }
    }
    #[doc = "Transmit Timestamp Status Seconds"]
    #[inline(always)]
    pub const fn mac_tx_timestamp_status_seconds(
        self,
    ) -> crate::common::Reg<regs::MacTxTimestampStatusSeconds, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b34usize) as _) }
    }
    #[doc = "Auxiliary Timestamp Control"]
    #[inline(always)]
    pub const fn mac_auxiliary_control(
        self,
    ) -> crate::common::Reg<regs::MacAuxiliaryControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b40usize) as _) }
    }
    #[doc = "Auxiliary Timestamp Nanoseconds"]
    #[inline(always)]
    pub const fn mac_auxiliary_timestamp_nanoseconds(
        self,
    ) -> crate::common::Reg<regs::MacAuxiliaryTimestampNanoseconds, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b48usize) as _) }
    }
    #[doc = "Auxiliary Timestamp Seconds"]
    #[inline(always)]
    pub const fn mac_auxiliary_timestamp_seconds(
        self,
    ) -> crate::common::Reg<regs::MacAuxiliaryTimestampSeconds, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b4cusize) as _) }
    }
    #[doc = "MAC Timestamp Ingress Correction Nanosecond"]
    #[inline(always)]
    pub const fn mac_timestamp_ingress_corr_nanosecond(
        self,
    ) -> crate::common::Reg<regs::MacTimestampIngressCorrNanosecond, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b58usize) as _) }
    }
    #[doc = "MAC Timestamp Egress Correction Nanosecond"]
    #[inline(always)]
    pub const fn mac_timestamp_egress_corr_nanosecond(
        self,
    ) -> crate::common::Reg<regs::MacTimestampEgressCorrNanosecond, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b5cusize) as _) }
    }
    #[doc = "MAC Timestamp Ingress Latency"]
    #[inline(always)]
    pub const fn mac_timestamp_ingress_latency(
        self,
    ) -> crate::common::Reg<regs::MacTimestampIngressLatency, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b68usize) as _) }
    }
    #[doc = "MAC Timestamp Egress Latency"]
    #[inline(always)]
    pub const fn mac_timestamp_egress_latency(
        self,
    ) -> crate::common::Reg<regs::MacTimestampEgressLatency, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b6cusize) as _) }
    }
    #[doc = "PPS Control"]
    #[inline(always)]
    pub const fn mac_pps_control(
        self,
    ) -> crate::common::Reg<regs::MacPpsControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b70usize) as _) }
    }
    #[doc = "PPS Target Time Seconds"]
    #[inline(always)]
    pub const fn mac_pps0_target_time_seconds(
        self,
    ) -> crate::common::Reg<regs::MacPps0TargetTimeSeconds, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b80usize) as _) }
    }
    #[doc = "PPS0 Target Time Nanoseconds"]
    #[inline(always)]
    pub const fn mac_pps0_target_time_nanoseconds(
        self,
    ) -> crate::common::Reg<regs::MacPps0TargetTimeNanoseconds, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b84usize) as _) }
    }
}
pub mod regs;
pub mod vals;
