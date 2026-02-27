#[doc = "no description available"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enet0_eqosDmaCh {
    ptr: *mut u8,
}
unsafe impl Send for Enet0_eqosDmaCh {}
unsafe impl Sync for Enet0_eqosDmaCh {}
impl Enet0_eqosDmaCh {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DMA Channel 0 Control"]
    #[inline(always)]
    pub const fn dma_ch0_control(
        self,
    ) -> crate::common::Reg<regs::DmaCh0Control, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "DMA Channel 0 Transmit Control"]
    #[inline(always)]
    pub const fn dma_ch0_tx_control(
        self,
    ) -> crate::common::Reg<regs::DmaCh0TxControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "DMA Channel 0 Receive Control"]
    #[inline(always)]
    pub const fn dma_ch0_rx_control(
        self,
    ) -> crate::common::Reg<regs::DmaCh0RxControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Channel 0 Transmit Descriptor List Address"]
    #[inline(always)]
    pub const fn dma_ch0_tx_desc_list_address(
        self,
    ) -> crate::common::Reg<regs::DmaCh0TxDescListAddress, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Channel 0 Receive Descriptor List Address"]
    #[inline(always)]
    pub const fn dma_ch0_rx_desc_list_address(
        self,
    ) -> crate::common::Reg<regs::DmaCh0RxDescListAddress, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Channel 0 Transmit Descriptor Tail Pointer"]
    #[inline(always)]
    pub const fn dma_ch0_tx_desc_tail_pointer(
        self,
    ) -> crate::common::Reg<regs::DmaCh0TxDescTailPointer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Channel 0 Receive Descriptor Tail Pointer"]
    #[inline(always)]
    pub const fn dma_ch0_rx_desc_tail_pointer(
        self,
    ) -> crate::common::Reg<regs::DmaCh0RxDescTailPointer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Channel 0 Transmit Descriptor Ring Length"]
    #[inline(always)]
    pub const fn dma_ch0_tx_desc_ring_length(
        self,
    ) -> crate::common::Reg<regs::DmaCh0TxDescRingLength, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Channel 0 Receive Control"]
    #[inline(always)]
    pub const fn dma_ch0_rx_control2(
        self,
    ) -> crate::common::Reg<regs::DmaCh0RxControl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Channel 0 Interrupt Enable"]
    #[inline(always)]
    pub const fn dma_ch0_interrupt_enable(
        self,
    ) -> crate::common::Reg<regs::DmaCh0InterruptEnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Channel 0 Receive Interrupt Watchdog Timer"]
    #[inline(always)]
    pub const fn dma_ch0_rx_interrupt_watchdog_timer(
        self,
    ) -> crate::common::Reg<regs::DmaCh0RxInterruptWatchdogTimer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Channel 0 Current Application Transmit Descriptor"]
    #[inline(always)]
    pub const fn dma_ch0_current_app_tx_desc(
        self,
    ) -> crate::common::Reg<regs::DmaCh0CurrentAppTxDesc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Channel 0 Current Application Receive Descriptor"]
    #[inline(always)]
    pub const fn dma_ch0_current_app_rx_desc(
        self,
    ) -> crate::common::Reg<regs::DmaCh0CurrentAppRxDesc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Channel 0 Current Application Transmit Buffer Address"]
    #[inline(always)]
    pub const fn dma_ch0_current_app_tx_buffer(
        self,
    ) -> crate::common::Reg<regs::DmaCh0CurrentAppTxBuffer, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Channel 0 Current Application Receive Buffer Address"]
    #[inline(always)]
    pub const fn dma_ch0_current_app_rx_buffer(
        self,
    ) -> crate::common::Reg<regs::DmaCh0CurrentAppRxBuffer, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Channel 0 Status"]
    #[inline(always)]
    pub const fn dma_ch0_status(self) -> crate::common::Reg<regs::DmaCh0Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Channel 0 Miss Frame Count"]
    #[inline(always)]
    pub const fn dma_ch0_miss_frame_cnt(
        self,
    ) -> crate::common::Reg<regs::DmaCh0MissFrameCnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Channel 0 Receive ERI Count"]
    #[inline(always)]
    pub const fn dma_ch0_rx_eri_cnt(
        self,
    ) -> crate::common::Reg<regs::DmaCh0RxEriCnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
