#[doc = "INPUTMUX"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inputmux {
    ptr: *mut u8,
}
unsafe impl Send for Inputmux {}
unsafe impl Sync for Inputmux {}
impl Inputmux {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Capture select register for CTIMER inputs"]
    #[inline(always)]
    pub const fn ctimer0cap(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ctimer0cap, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "Trigger register for TIMER0"]
    #[inline(always)]
    pub const fn timer0trig(self) -> crate::common::Reg<regs::Timer0trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Capture select register for CTIMER inputs"]
    #[inline(always)]
    pub const fn ctimer1cap(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ctimer1cap, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "Trigger register for TIMER1"]
    #[inline(always)]
    pub const fn timer1trig(self) -> crate::common::Reg<regs::Timer1trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Capture select register for CTIMER inputs"]
    #[inline(always)]
    pub const fn ctimer2cap(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ctimer2cap, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 4usize) as _) }
    }
    #[doc = "Trigger register for TIMER2 inputs"]
    #[inline(always)]
    pub const fn timer2trig(self) -> crate::common::Reg<regs::Timer2trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "SmartDMA Trigger Input Connections"]
    #[inline(always)]
    pub const fn smart_dma_trig(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SmartDmaTrig, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 4usize) as _) }
    }
    #[doc = "LPSPI2 trigger input connections"]
    #[inline(always)]
    pub const fn lpspi2_trig(self) -> crate::common::Reg<regs::LpspiTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "LPSPI3 trigger input connections"]
    #[inline(always)]
    pub const fn lpspi3_trig(self) -> crate::common::Reg<regs::LpspiTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "LPSPI4 trigger input connections"]
    #[inline(always)]
    pub const fn lpspi4_trig(self) -> crate::common::Reg<regs::LpspiTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "LPSPI5 trigger input connections"]
    #[inline(always)]
    pub const fn lpspi5_trig(self) -> crate::common::Reg<regs::LpspiTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "ENET trigger input connections"]
    #[inline(always)]
    pub const fn enet_trig_in(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::EnetTrigIn, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize + n * 4usize) as _)
        }
    }
    #[doc = "10BASE-T1S Local Wake Up connections"]
    #[inline(always)]
    pub const fn t1s_wkup(self) -> crate::common::Reg<regs::T1sWkup, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Selection for frequency measurement reference clock"]
    #[inline(always)]
    pub const fn freqmeas_ref(self) -> crate::common::Reg<regs::FreqmeasRef, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Selection for frequency measurement reference clock"]
    #[inline(always)]
    pub const fn freqmeas_tar(self) -> crate::common::Reg<regs::FreqmeasTar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Capture select register for CTIMER inputs"]
    #[inline(always)]
    pub const fn ctimer3cap(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ctimer3cap, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize + n * 4usize) as _)
        }
    }
    #[doc = "Trigger register for TIMER3"]
    #[inline(always)]
    pub const fn timer3trig(self) -> crate::common::Reg<regs::Timer3trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "Capture select register for CTIMER inputs"]
    #[inline(always)]
    pub const fn ctimer4cap(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ctimer4cap, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize + n * 4usize) as _)
        }
    }
    #[doc = "Trigger register for TIMER4"]
    #[inline(always)]
    pub const fn timer4trig(self) -> crate::common::Reg<regs::Timer4trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "CMP0 input connections"]
    #[inline(always)]
    pub const fn cmp0_trig(self) -> crate::common::Reg<regs::CmpTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "ADC Trigger input connections"]
    #[inline(always)]
    pub const fn adc0_trig(self, n: usize) -> crate::common::Reg<regs::AdcTrig, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize + n * 4usize) as _)
        }
    }
    #[doc = "ADC Trigger input connections"]
    #[inline(always)]
    pub const fn adc1_trig(self, n: usize) -> crate::common::Reg<regs::AdcTrig, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize + n * 4usize) as _)
        }
    }
    #[doc = "DAC0 trigger"]
    #[inline(always)]
    pub const fn dac0_trig(self) -> crate::common::Reg<regs::DacTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "DAC1 trigger"]
    #[inline(always)]
    pub const fn dac1_trig(self) -> crate::common::Reg<regs::DacTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "AOI0 trigger input connections 0"]
    #[inline(always)]
    pub const fn aoi0_input(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::AoiInput, crate::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize + n * 4usize) as _)
        }
    }
    #[doc = "TSI0 trigger input connections"]
    #[inline(always)]
    pub const fn tsi0_trig_input(
        self,
    ) -> crate::common::Reg<regs::Tsi0TrigInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04a0usize) as _) }
    }
    #[doc = "EXT trigger connections"]
    #[inline(always)]
    pub const fn trig_out(self, n: usize) -> crate::common::Reg<regs::TrigOut, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c0usize + n * 4usize) as _)
        }
    }
    #[doc = "LPI2C2 trigger input connections"]
    #[inline(always)]
    pub const fn lpi2c2_trig(self) -> crate::common::Reg<regs::Lpi2cTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize) as _) }
    }
    #[doc = "LPI2C3 trigger input connections"]
    #[inline(always)]
    pub const fn lpi2c3_trig(self) -> crate::common::Reg<regs::Lpi2cTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0560usize) as _) }
    }
    #[doc = "LPI2C4 trigger input connections"]
    #[inline(always)]
    pub const fn lpi2c4_trig(self) -> crate::common::Reg<regs::Lpi2cTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0580usize) as _) }
    }
    #[doc = "LPI2C0 trigger input connections"]
    #[inline(always)]
    pub const fn lpi2c0_trig(self) -> crate::common::Reg<regs::Lpi2cTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a0usize) as _) }
    }
    #[doc = "LPI2C1 trigger input connections"]
    #[inline(always)]
    pub const fn lpi2c1_trig(self) -> crate::common::Reg<regs::Lpi2cTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
    }
    #[doc = "LPSPI0 trigger input connections"]
    #[inline(always)]
    pub const fn lpspi0_trig(self) -> crate::common::Reg<regs::LpspiTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e0usize) as _) }
    }
    #[doc = "LPSPI1 trigger input connections"]
    #[inline(always)]
    pub const fn lpspi1_trig(self) -> crate::common::Reg<regs::LpspiTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "LPUART0 trigger input connections"]
    #[inline(always)]
    pub const fn lpuart(self, n: usize) -> crate::common::Reg<regs::Lpuart, crate::common::RW> {
        assert!(n < 6usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0620usize + n * 32usize) as _)
        }
    }
    #[doc = "FlexIO Trigger Input Connections"]
    #[inline(always)]
    pub const fn flexio_trig(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::FlexioTrig, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06e0usize + n * 4usize) as _)
        }
    }
}
pub mod regs;
pub mod vals;
