#[doc = "SPI FILTER"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiFilter {
    ptr: *mut u8,
}
unsafe impl Send for SpiFilter {}
unsafe impl Sync for SpiFilter {}
impl SpiFilter {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Interrupt Register"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(self) -> crate::common::Reg<regs::Imr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Port Status Register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Test Register"]
    #[inline(always)]
    pub const fn tr(self) -> crate::common::Reg<regs::Tr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "PORT0 Filter Address Region 1"]
    #[inline(always)]
    pub const fn p0far1(self) -> crate::common::Reg<regs::P0far1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "PORT0 Filter Address Region 2"]
    #[inline(always)]
    pub const fn p0far2(self) -> crate::common::Reg<regs::P0far2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "PORT0 Filter Address Region 3"]
    #[inline(always)]
    pub const fn p0far3(self) -> crate::common::Reg<regs::P0far3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "PORT0 Filter Address Region 4"]
    #[inline(always)]
    pub const fn p0far4(self) -> crate::common::Reg<regs::P0far4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "PORT0 Filter Address Region 5"]
    #[inline(always)]
    pub const fn p0far5(self) -> crate::common::Reg<regs::P0far5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "PORT0 Filter Address Region 6"]
    #[inline(always)]
    pub const fn p0far6(self) -> crate::common::Reg<regs::P0far6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "PORT1 Filter Address Region 1"]
    #[inline(always)]
    pub const fn p1far1(self) -> crate::common::Reg<regs::P1far1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "PORT1 Filter Address Region 2"]
    #[inline(always)]
    pub const fn p1far2(self) -> crate::common::Reg<regs::P1far2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "PORT1 Filter Address Region 3"]
    #[inline(always)]
    pub const fn p1far3(self) -> crate::common::Reg<regs::P1far3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "PORT1 Filter Address Region 4"]
    #[inline(always)]
    pub const fn p1far4(self) -> crate::common::Reg<regs::P1far4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "PORT1 Filter Address Region 5"]
    #[inline(always)]
    pub const fn p1far5(self) -> crate::common::Reg<regs::P1far5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "PORT1 Filter Address Region 6"]
    #[inline(always)]
    pub const fn p1far6(self) -> crate::common::Reg<regs::P1far6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Programmable OP Code0"]
    #[inline(always)]
    pub const fn popcode0(self) -> crate::common::Reg<regs::Popcode0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Programmable OP Code1"]
    #[inline(always)]
    pub const fn popcode1(self) -> crate::common::Reg<regs::Popcode1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Programmable OP Code2"]
    #[inline(always)]
    pub const fn popcode2(self) -> crate::common::Reg<regs::Popcode2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Programmable OP Code3"]
    #[inline(always)]
    pub const fn popcode3(self) -> crate::common::Reg<regs::Popcode3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Programmable OP Code4"]
    #[inline(always)]
    pub const fn popcode4(self) -> crate::common::Reg<regs::Popcode4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "P0 Blocked Op Code"]
    #[inline(always)]
    pub const fn p0boc(self) -> crate::common::Reg<regs::P0boc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "P1 Blocked Op Code"]
    #[inline(always)]
    pub const fn p1boc(self) -> crate::common::Reg<regs::P1boc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "PORT0 Max Address Mask"]
    #[inline(always)]
    pub const fn p0mam(self) -> crate::common::Reg<regs::P0mam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "PORT1 Max Address Mask"]
    #[inline(always)]
    pub const fn p1mam(self) -> crate::common::Reg<regs::P1mam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
}
pub mod regs;
pub mod vals;
