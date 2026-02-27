#[doc = "Enhanced Serial Peripheral Interface"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Espi {
    ptr: *mut u8,
}
unsafe impl Send for Espi {}
unsafe impl Sync for Espi {}
impl Espi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Master Control"]
    #[inline(always)]
    pub const fn mctrl(self) -> crate::common::Reg<regs::Mctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Master Status"]
    #[inline(always)]
    pub const fn mstat(self) -> crate::common::Reg<regs::Mstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Interrupt Enable Set"]
    #[inline(always)]
    pub const fn intenset(self) -> crate::common::Reg<regs::Intenset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Interrupt Clear"]
    #[inline(always)]
    pub const fn intenclr(self) -> crate::common::Reg<regs::Intenclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Masked Interrupt Status"]
    #[inline(always)]
    pub const fn intstat(self) -> crate::common::Reg<regs::Intstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "DMA Control"]
    #[inline(always)]
    pub const fn dmactrl(self) -> crate::common::Reg<regs::Dmactrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "RAM Base"]
    #[inline(always)]
    pub const fn rambase(self) -> crate::common::Reg<regs::Rambase, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Mapped Base"]
    #[inline(always)]
    pub const fn mapbase(self) -> crate::common::Reg<regs::Mapbase, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "IRQ Push"]
    #[inline(always)]
    pub const fn irqpush(self) -> crate::common::Reg<regs::Irqpush, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Virtual Wire MCU-to-host"]
    #[inline(always)]
    pub const fn wirewo(self) -> crate::common::Reg<regs::Wirewo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Virtual Wire Host-to-MCU"]
    #[inline(always)]
    pub const fn wirero(self) -> crate::common::Reg<regs::Wirero, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Port 80 Status"]
    #[inline(always)]
    pub const fn p80stat(self) -> crate::common::Reg<regs::P80stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Status Block Address"]
    #[inline(always)]
    pub const fn stataddr(self) -> crate::common::Reg<regs::Stataddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "eSPI Capabilities"]
    #[inline(always)]
    pub const fn espicap(self) -> crate::common::Reg<regs::Espicap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "eSPI Configuration"]
    #[inline(always)]
    pub const fn espicfg(self) -> crate::common::Reg<regs::Espicfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "eSPI Miscellaneous"]
    #[inline(always)]
    pub const fn espimisc(self) -> crate::common::Reg<regs::Espimisc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "RPMC Support 1"]
    #[inline(always)]
    pub const fn rpmc_support1(self) -> crate::common::Reg<regs::RpmcSupport1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "RPMC Support 2"]
    #[inline(always)]
    pub const fn rpmc_support2(self) -> crate::common::Reg<regs::RpmcSupport2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "WIREIN_GPIO"]
    #[inline(always)]
    pub const fn wirein_gpio(self) -> crate::common::Reg<regs::WireinGpio, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "WIREOUT_GPIO"]
    #[inline(always)]
    pub const fn wireout_gpio(self) -> crate::common::Reg<regs::WireoutGpio, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Port Configuration"]
    #[inline(always)]
    pub const fn p0cfg(self) -> crate::common::Reg<regs::P0cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Port Status"]
    #[inline(always)]
    pub const fn p0stat(self) -> crate::common::Reg<regs::P0stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Set Interrupt Rules and User Status"]
    #[inline(always)]
    pub const fn p0irulestat(self) -> crate::common::Reg<regs::P0irulestat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Port Address"]
    #[inline(always)]
    pub const fn p0addr(self) -> crate::common::Reg<regs::P0addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Port OOB, Mastering, and Flash Length"]
    #[inline(always)]
    pub const fn p0omflen(self) -> crate::common::Reg<regs::P0omflen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn p0datain(self) -> crate::common::Reg<regs::P0datain, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Port Data Out"]
    #[inline(always)]
    pub const fn p0dataout(self) -> crate::common::Reg<regs::P0dataout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Port RAM Use"]
    #[inline(always)]
    pub const fn p0ramuse(self) -> crate::common::Reg<regs::P0ramuse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Port Configuration"]
    #[inline(always)]
    pub const fn p1cfg(self) -> crate::common::Reg<regs::P1cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Port Status"]
    #[inline(always)]
    pub const fn p1stat(self) -> crate::common::Reg<regs::P1stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Set Interrupt Rules and User Status"]
    #[inline(always)]
    pub const fn p1irulestat(self) -> crate::common::Reg<regs::P1irulestat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Port Address"]
    #[inline(always)]
    pub const fn p1addr(self) -> crate::common::Reg<regs::P1addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "Port OOB, Mastering, and Flash Length"]
    #[inline(always)]
    pub const fn p1omflen(self) -> crate::common::Reg<regs::P1omflen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn p1datain(self) -> crate::common::Reg<regs::P1datain, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Port Data Out"]
    #[inline(always)]
    pub const fn p1dataout(self) -> crate::common::Reg<regs::P1dataout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Port RAM Use"]
    #[inline(always)]
    pub const fn p1ramuse(self) -> crate::common::Reg<regs::P1ramuse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Port Configuration"]
    #[inline(always)]
    pub const fn p2cfg(self) -> crate::common::Reg<regs::P2cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Port Status"]
    #[inline(always)]
    pub const fn p2stat(self) -> crate::common::Reg<regs::P2stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Set Interrupt Rules and User Status"]
    #[inline(always)]
    pub const fn p2irulestat(self) -> crate::common::Reg<regs::P2irulestat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "Port Address"]
    #[inline(always)]
    pub const fn p2addr(self) -> crate::common::Reg<regs::P2addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "Port OOB, Mastering, and Flash Length"]
    #[inline(always)]
    pub const fn p2omflen(self) -> crate::common::Reg<regs::P2omflen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn p2datain(self) -> crate::common::Reg<regs::P2datain, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Port Data Out"]
    #[inline(always)]
    pub const fn p2dataout(self) -> crate::common::Reg<regs::P2dataout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Port RAM Use"]
    #[inline(always)]
    pub const fn p2ramuse(self) -> crate::common::Reg<regs::P2ramuse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Port Configuration"]
    #[inline(always)]
    pub const fn p3cfg(self) -> crate::common::Reg<regs::P3cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Port Status"]
    #[inline(always)]
    pub const fn p3stat(self) -> crate::common::Reg<regs::P3stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Set Interrupt Rules and User Status"]
    #[inline(always)]
    pub const fn p3irulestat(self) -> crate::common::Reg<regs::P3irulestat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "Port Address"]
    #[inline(always)]
    pub const fn p3addr(self) -> crate::common::Reg<regs::P3addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "Port OOB, Mastering, and Flash Length"]
    #[inline(always)]
    pub const fn p3omflen(self) -> crate::common::Reg<regs::P3omflen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn p3datain(self) -> crate::common::Reg<regs::P3datain, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "Port Data Out"]
    #[inline(always)]
    pub const fn p3dataout(self) -> crate::common::Reg<regs::P3dataout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "Port RAM Use"]
    #[inline(always)]
    pub const fn p3ramuse(self) -> crate::common::Reg<regs::P3ramuse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "Port Configuration"]
    #[inline(always)]
    pub const fn p4cfg(self) -> crate::common::Reg<regs::P4cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Port Status"]
    #[inline(always)]
    pub const fn p4stat(self) -> crate::common::Reg<regs::P4stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Set Interrupt Rules and User Status"]
    #[inline(always)]
    pub const fn p4irulestat(self) -> crate::common::Reg<regs::P4irulestat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Port Address"]
    #[inline(always)]
    pub const fn p4addr(self) -> crate::common::Reg<regs::P4addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "Port OOB, Mastering, and Flash Length"]
    #[inline(always)]
    pub const fn p4omflen(self) -> crate::common::Reg<regs::P4omflen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "Port Data Input"]
    #[inline(always)]
    pub const fn p4datain(self) -> crate::common::Reg<regs::P4datain, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Port Data Out"]
    #[inline(always)]
    pub const fn p4dataout(self) -> crate::common::Reg<regs::P4dataout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "Port RAM Use"]
    #[inline(always)]
    pub const fn p4ramuse(self) -> crate::common::Reg<regs::P4ramuse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
}
pub mod regs;
pub mod vals;
