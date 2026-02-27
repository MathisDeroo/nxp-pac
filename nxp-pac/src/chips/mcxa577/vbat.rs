#[doc = "VBAT"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbat {
    ptr: *mut u8,
}
unsafe impl Send for Vbat {}
unsafe impl Sync for Vbat {}
impl Vbat {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version ID"]
    #[inline(always)]
    pub const fn verid(self) -> crate::common::Reg<regs::Verid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Status A"]
    #[inline(always)]
    pub const fn statusa(self) -> crate::common::Reg<regs::Statusa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Interrupt Enable A"]
    #[inline(always)]
    pub const fn irqena(self) -> crate::common::Reg<regs::Irqena, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Wake-up Enable A"]
    #[inline(always)]
    pub const fn wakena(self) -> crate::common::Reg<regs::Wakena, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Lock A"]
    #[inline(always)]
    pub const fn locka(self) -> crate::common::Reg<regs::Locka, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Wake-up Configuration"]
    #[inline(always)]
    pub const fn wakecfg(self) -> crate::common::Reg<regs::Wakecfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Oscillator Control A"]
    #[inline(always)]
    pub const fn oscctla(self) -> crate::common::Reg<regs::Oscctla, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Oscillator Configuration A"]
    #[inline(always)]
    pub const fn osccfga(self) -> crate::common::Reg<regs::Osccfga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Oscillator Lock A"]
    #[inline(always)]
    pub const fn osclcka(self) -> crate::common::Reg<regs::Osclcka, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Oscillator Clock Enable"]
    #[inline(always)]
    pub const fn oscclke(self) -> crate::common::Reg<regs::Oscclke, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "FRO16K Control A"]
    #[inline(always)]
    pub const fn froctla(self) -> crate::common::Reg<regs::Froctla, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "FRO16K Lock A"]
    #[inline(always)]
    pub const fn frolcka(self) -> crate::common::Reg<regs::Frolcka, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "FRO16K Clock Enable"]
    #[inline(always)]
    pub const fn froclke(self) -> crate::common::Reg<regs::Froclke, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "LDO_RAM Control A"]
    #[inline(always)]
    pub const fn ldoctla(self) -> crate::common::Reg<regs::Ldoctla, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "LDO_RAM Lock A"]
    #[inline(always)]
    pub const fn ldolcka(self) -> crate::common::Reg<regs::Ldolcka, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0318usize) as _) }
    }
    #[doc = "RAM Control"]
    #[inline(always)]
    pub const fn ldoramc(self) -> crate::common::Reg<regs::Ldoramc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "Bandgap Timer 0"]
    #[inline(always)]
    pub const fn ldotimer0(self) -> crate::common::Reg<regs::Ldotimer0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0330usize) as _) }
    }
    #[doc = "Bandgap Timer 1"]
    #[inline(always)]
    pub const fn ldotimer1(self) -> crate::common::Reg<regs::Ldotimer1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0338usize) as _) }
    }
    #[doc = "CLKMON Control A"]
    #[inline(always)]
    pub const fn monctla(self) -> crate::common::Reg<regs::Monctla, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "CLKMON Configuration A"]
    #[inline(always)]
    pub const fn moncfga(self) -> crate::common::Reg<regs::Moncfga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize) as _) }
    }
    #[doc = "CLKMON Test A"]
    #[inline(always)]
    pub const fn montsta(self) -> crate::common::Reg<regs::Montsta, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "CLKMON Lock A"]
    #[inline(always)]
    pub const fn monlcka(self) -> crate::common::Reg<regs::Monlcka, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0418usize) as _) }
    }
    #[doc = "Switch Control A"]
    #[inline(always)]
    pub const fn swictla(self) -> crate::common::Reg<regs::Swictla, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "Switch Lock A"]
    #[inline(always)]
    pub const fn swilcka(self) -> crate::common::Reg<regs::Swilcka, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0618usize) as _) }
    }
    #[doc = "Array of registers: WAKEUPA"]
    #[inline(always)]
    pub const fn wakeup(self, n: usize) -> Wakeup {
        assert!(n < 2usize);
        unsafe { Wakeup::from_ptr(self.ptr.wrapping_add(0x0700usize + n * 8usize) as _) }
    }
    #[doc = "Wakeup Lock A"]
    #[inline(always)]
    pub const fn waklcka(self) -> crate::common::Reg<regs::Waklcka, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07f8usize) as _) }
    }
}
#[doc = "Array of registers: WAKEUPA"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakeup {
    ptr: *mut u8,
}
unsafe impl Send for Wakeup {}
unsafe impl Sync for Wakeup {}
impl Wakeup {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Wakeup 0 Register A"]
    #[inline(always)]
    pub const fn wakeupa(self) -> crate::common::Reg<regs::Wakeupa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
pub mod regs;
pub mod vals;
