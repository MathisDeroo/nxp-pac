#[doc = "no description available"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enet0_eqosMtlQ {
    ptr: *mut u8,
}
unsafe impl Send for Enet0_eqosMtlQ {}
unsafe impl Sync for Enet0_eqosMtlQ {}
impl Enet0_eqosMtlQ {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Queue 0 Transmit Operation Mode"]
    #[inline(always)]
    pub const fn mtl_tx_q0_operation_mode(
        self,
    ) -> crate::common::Reg<regs::MtlTxQ0OperationMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Queue 0 Underflow Counter"]
    #[inline(always)]
    pub const fn mtl_tx_q0_underflow(
        self,
    ) -> crate::common::Reg<regs::MtlTxQ0Underflow, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Queue 0 Transmit Debug"]
    #[inline(always)]
    pub const fn mtl_tx_q0_debug(self) -> crate::common::Reg<regs::MtlTxQ0Debug, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Queue 0 Interrupt Control Status"]
    #[inline(always)]
    pub const fn mtl_q0_interrupt_control_status(
        self,
    ) -> crate::common::Reg<regs::MtlQ0InterruptControlStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Queue 0 Receive Operation Mode"]
    #[inline(always)]
    pub const fn mtl_rx_q0_operation_mode(
        self,
    ) -> crate::common::Reg<regs::MtlRxQ0OperationMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Queue 0 Missed Packet and Overflow Counter"]
    #[inline(always)]
    pub const fn mtl_rx_q0_missed_packet_overflow_cnt(
        self,
    ) -> crate::common::Reg<regs::MtlRxQ0MissedPacketOverflowCnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Queue 0 Receive Debug"]
    #[inline(always)]
    pub const fn mtl_rx_q0_debug(self) -> crate::common::Reg<regs::MtlRxQ0Debug, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
}
pub mod regs;
pub mod vals;
