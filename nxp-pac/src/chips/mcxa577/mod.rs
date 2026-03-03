#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6a8c2aa 2026-01-27))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - RESERVED16"]
    RESERVED16 = 0,
    #[doc = "1 - CMC"]
    CMC = 1,
    #[doc = "2 - DMA_CH0"]
    DMA_CH0 = 2,
    #[doc = "3 - DMA_CH1"]
    DMA_CH1 = 3,
    #[doc = "4 - DMA_CH2"]
    DMA_CH2 = 4,
    #[doc = "5 - DMA_CH3"]
    DMA_CH3 = 5,
    #[doc = "6 - DMA_CH4"]
    DMA_CH4 = 6,
    #[doc = "7 - DMA_CH5"]
    DMA_CH5 = 7,
    #[doc = "8 - DMA_CH6"]
    DMA_CH6 = 8,
    #[doc = "9 - DMA_CH7"]
    DMA_CH7 = 9,
    #[doc = "10 - ERM0_SINGLE_BIT"]
    ERM0_SINGLE_BIT = 10,
    #[doc = "11 - ERM0_MULTI_BIT"]
    ERM0_MULTI_BIT = 11,
    #[doc = "12 - FMU0"]
    FMU0 = 12,
    #[doc = "13 - GLIKEY0"]
    GLIKEY0 = 13,
    #[doc = "14 - MBC0"]
    MBC0 = 14,
    #[doc = "15 - SCG0"]
    SCG0 = 15,
    #[doc = "16 - SPC0"]
    SPC0 = 16,
    #[doc = "17 - TDET"]
    TDET = 17,
    #[doc = "18 - WUU0"]
    WUU0 = 18,
    #[doc = "19 - CAN0"]
    CAN0 = 19,
    #[doc = "20 - CAN1"]
    CAN1 = 20,
    #[doc = "23 - FLEXIO"]
    FLEXIO = 23,
    #[doc = "24 - I3C0"]
    I3C0 = 24,
    #[doc = "25 - I3C1"]
    I3C1 = 25,
    #[doc = "26 - LPI2C0"]
    LPI2C0 = 26,
    #[doc = "27 - LPI2C1"]
    LPI2C1 = 27,
    #[doc = "28 - LPSPI0"]
    LPSPI0 = 28,
    #[doc = "29 - LPSPI1"]
    LPSPI1 = 29,
    #[doc = "30 - LPSPI2"]
    LPSPI2 = 30,
    #[doc = "31 - LPUART0"]
    LPUART0 = 31,
    #[doc = "32 - LPUART1"]
    LPUART1 = 32,
    #[doc = "33 - LPUART2"]
    LPUART2 = 33,
    #[doc = "34 - LPUART3"]
    LPUART3 = 34,
    #[doc = "35 - LPUART4"]
    LPUART4 = 35,
    #[doc = "38 - CDOG0"]
    CDOG0 = 38,
    #[doc = "39 - CTIMER0"]
    CTIMER0 = 39,
    #[doc = "40 - CTIMER1"]
    CTIMER1 = 40,
    #[doc = "41 - CTIMER2"]
    CTIMER2 = 41,
    #[doc = "42 - CTIMER3"]
    CTIMER3 = 42,
    #[doc = "43 - CTIMER4"]
    CTIMER4 = 43,
    #[doc = "54 - FREQME0"]
    FREQME0 = 54,
    #[doc = "55 - LPTMR0"]
    LPTMR0 = 55,
    #[doc = "57 - OS_EVENT"]
    OS_EVENT = 57,
    #[doc = "58 - WAKETIMER0"]
    WAKETIMER0 = 58,
    #[doc = "59 - UTICK0"]
    UTICK0 = 59,
    #[doc = "60 - WWDT0"]
    WWDT0 = 60,
    #[doc = "61 - WWDT1"]
    WWDT1 = 61,
    #[doc = "62 - ADC0"]
    ADC0 = 62,
    #[doc = "63 - ADC1"]
    ADC1 = 63,
    #[doc = "64 - CMP0"]
    CMP0 = 64,
    #[doc = "67 - DAC0"]
    DAC0 = 67,
    #[doc = "68 - DAC1"]
    DAC1 = 68,
    #[doc = "71 - GPIO0"]
    GPIO0 = 71,
    #[doc = "72 - GPIO1"]
    GPIO1 = 72,
    #[doc = "73 - GPIO2"]
    GPIO2 = 73,
    #[doc = "74 - GPIO3"]
    GPIO3 = 74,
    #[doc = "75 - GPIO4"]
    GPIO4 = 75,
    #[doc = "76 - GPIO5"]
    GPIO5 = 76,
    #[doc = "77 - LPI2C2"]
    LPI2C2 = 77,
    #[doc = "78 - LPI2C3"]
    LPI2C3 = 78,
    #[doc = "89 - ESPI"]
    ESPI = 89,
    #[doc = "90 - ETHERNET"]
    ETHERNET = 90,
    #[doc = "91 - ETHERNET_PMT"]
    ETHERNET_PMT = 91,
    #[doc = "93 - TENBASET_PHY0"]
    TENBASET_PHY0 = 93,
    #[doc = "94 - I3C2"]
    I3C2 = 94,
    #[doc = "95 - LPUART5"]
    LPUART5 = 95,
    #[doc = "97 - LPSPI3"]
    LPSPI3 = 97,
    #[doc = "98 - LPSPI4"]
    LPSPI4 = 98,
    #[doc = "99 - LPSPI5"]
    LPSPI5 = 99,
    #[doc = "100 - LPI2C4"]
    LPI2C4 = 100,
    #[doc = "101 - I3C3"]
    I3C3 = 101,
    #[doc = "103 - USB1_HS"]
    USB1_HS = 103,
    #[doc = "104 - USB1_HS_PHY"]
    USB1_HS_PHY = 104,
    #[doc = "106 - FLEXSPI0"]
    FLEXSPI0 = 106,
    #[doc = "108 - SMARTDMA"]
    SMARTDMA = 108,
    #[doc = "109 - CDOG1"]
    CDOG1 = 109,
    #[doc = "110 - PKC"]
    PKC = 110,
    #[doc = "111 - SGI"]
    SGI = 111,
    #[doc = "112 - SPI_FILTER"]
    SPI_FILTER = 112,
    #[doc = "113 - TRNG0"]
    TRNG0 = 113,
    #[doc = "114 - SECURE_ERR"]
    SECURE_ERR = 114,
    #[doc = "115 - SEC_HYPERVISOR_CALL"]
    SEC_HYPERVISOR_CALL = 115,
    #[doc = "119 - RTC"]
    RTC = 119,
    #[doc = "122 - GDET"]
    GDET = 122,
    #[doc = "123 - EWM0"]
    EWM0 = 123,
    #[doc = "124 - TSI_END_OF_SCAN"]
    TSI_END_OF_SCAN = 124,
    #[doc = "125 - TSI_OUT_OF_SCAN"]
    TSI_OUT_OF_SCAN = 125,
    #[doc = "126 - GPIO0_1"]
    GPIO0_1 = 126,
    #[doc = "127 - GPIO1_1"]
    GPIO1_1 = 127,
    #[doc = "128 - GPIO2_1"]
    GPIO2_1 = 128,
    #[doc = "129 - GPIO3_1"]
    GPIO3_1 = 129,
    #[doc = "130 - GPIO4_1"]
    GPIO4_1 = 130,
    #[doc = "131 - GPIO5_1"]
    GPIO5_1 = 131,
    #[doc = "133 - ITRC0"]
    ITRC0 = 133,
    #[doc = "134 - DMA_CH8"]
    DMA_CH8 = 134,
    #[doc = "135 - DMA_CH9"]
    DMA_CH9 = 135,
    #[doc = "136 - DMA0_CH10"]
    DMA0_CH10 = 136,
    #[doc = "137 - DMA0_CH11"]
    DMA0_CH11 = 137,
    #[doc = "142 - DMA1_CH0"]
    DMA1_CH0 = 142,
    #[doc = "143 - DMA1_CH1"]
    DMA1_CH1 = 143,
    #[doc = "144 - DMA1_CH2"]
    DMA1_CH2 = 144,
    #[doc = "145 - DMA1_CH3"]
    DMA1_CH3 = 145,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors;
#[doc = "INPUTMUX"]
pub const INPUTMUX0: inputmux::Inputmux =
    unsafe { inputmux::Inputmux::from_ptr(0x4000_1000usize as _) };
#[doc = "Improved Inter-Integrated Circuit"]
pub const I3C0: i3c::I3c = unsafe { i3c::I3c::from_ptr(0x4000_2000usize as _) };
#[doc = "Improved Inter-Integrated Circuit"]
pub const I3C1: i3c1::I3c1 = unsafe { i3c1::I3c1::from_ptr(0x4000_3000usize as _) };
#[doc = "Standard Counter or Timer"]
pub const CTIMER0: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4000_4000usize as _) };
#[doc = "Standard Counter or Timer"]
pub const CTIMER1: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4000_5000usize as _) };
#[doc = "Standard Counter or Timer"]
pub const CTIMER2: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4000_6000usize as _) };
#[doc = "Standard Counter or Timer"]
pub const CTIMER3: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4000_7000usize as _) };
#[doc = "Standard Counter or Timer"]
pub const CTIMER4: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4000_8000usize as _) };
#[doc = "Frequency Measurement"]
pub const FREQME0: freqme::Freqme = unsafe { freqme::Freqme::from_ptr(0x4000_9000usize as _) };
#[doc = "Micro-Tick Timer"]
pub const UTICK0: utick::Utick = unsafe { utick::Utick::from_ptr(0x4000_b000usize as _) };
#[doc = "Windowed Watchdog Timer"]
pub const WWDT0: wwdt::Wwdt = unsafe { wwdt::Wwdt::from_ptr(0x4000_c000usize as _) };
#[doc = "Windowed Watchdog Timer"]
pub const WWDT1: wwdt::Wwdt = unsafe { wwdt::Wwdt::from_ptr(0x4000_d000usize as _) };
#[doc = "Smart DMA Controller"]
pub const SMARTDMA: smartdma::Smartdma =
    unsafe { smartdma::Smartdma::from_ptr(0x4000_e000usize as _) };
#[doc = "External Watchdog Monitor"]
pub const EWM: ewm::Ewm = unsafe { ewm::Ewm::from_ptr(0x4001_0000usize as _) };
#[doc = "no description available"]
pub const PKC0: pkc::Pkc = unsafe { pkc::Pkc::from_ptr(0x4001_2000usize as _) };
#[doc = "DMA MP"]
pub const DMA1: dma1::Dma1 = unsafe { dma1::Dma1::from_ptr(0x4001_3000usize as _) };
#[doc = "DMA TCD"]
pub const EDMA1_TCD0: edma1_tcd::Edma1Tcd =
    unsafe { edma1_tcd::Edma1Tcd::from_ptr(0x4001_4000usize as _) };
#[doc = "no description available"]
pub const ENET0: enet::Enet = unsafe { enet::Enet::from_ptr(0x4001_c000usize as _) };
#[doc = "no description available"]
pub const ENET0__EQOS_MTL: enet0__eqos_mtl::Enet0_eqosMtl =
    unsafe { enet0__eqos_mtl::Enet0_eqosMtl::from_ptr(0x4001_cc00usize as _) };
#[doc = "no description available"]
pub const ENET0__EQOS_MTL_Q0: enet0__eqos_mtl_q::Enet0_eqosMtlQ =
    unsafe { enet0__eqos_mtl_q::Enet0_eqosMtlQ::from_ptr(0x4001_cd00usize as _) };
#[doc = "no description available"]
pub const ENET0__EQOS_DMA: enet0__eqos_dma::Enet0_eqosDma =
    unsafe { enet0__eqos_dma::Enet0_eqosDma::from_ptr(0x4001_d000usize as _) };
#[doc = "no description available"]
pub const ENET0__EQOS_DMA_CH0: enet0__eqos_dma_ch::Enet0_eqosDmaCh =
    unsafe { enet0__eqos_dma_ch::Enet0_eqosDmaCh::from_ptr(0x4001_d100usize as _) };
#[doc = "Enhanced Serial Peripheral Interface"]
pub const ESPI0: espi::Espi = unsafe { espi::Espi::from_ptr(0x4001_f000usize as _) };
#[doc = "Flexible Serial Peripheral Interface"]
pub const FLEXSPI0: flexspi::Flexspi = unsafe { flexspi::Flexspi::from_ptr(0x4002_0000usize as _) };
#[doc = "Low-Power Serial Peripheral Interface"]
pub const LPSPI2: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4002_1000usize as _) };
#[doc = "Low-Power Serial Peripheral Interface"]
pub const LPSPI3: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4002_2000usize as _) };
#[doc = "Low-Power Serial Peripheral Interface"]
pub const LPSPI4: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4002_3000usize as _) };
#[doc = "Low-Power Serial Peripheral Interface"]
pub const LPSPI5: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4002_4000usize as _) };
#[doc = "SPI FILTER"]
pub const SPI_FILTER: spi_filter::SpiFilter =
    unsafe { spi_filter::SpiFilter::from_ptr(0x4002_c000usize as _) };
#[doc = "TENBASET_PHY"]
pub const T1S0: t1s::T1s = unsafe { t1s::T1s::from_ptr(0x4002_d000usize as _) };
#[doc = "USBC"]
pub const USB0: usb::Usb = unsafe { usb::Usb::from_ptr(0x4002_e000usize as _) };
#[doc = "USBNC"]
pub const USBNC: usbnc::Usbnc = unsafe { usbnc::Usbnc::from_ptr(0x4002_e200usize as _) };
#[doc = "Universal Serial Bus 2.0 Integrated PHY"]
pub const USBPHY0: usbphy::Usbphy = unsafe { usbphy::Usbphy::from_ptr(0x4002_f000usize as _) };
#[doc = "Code Watchdog Timer"]
pub const CDOG0: cdog::Cdog = unsafe { cdog::Cdog::from_ptr(0x4004_0000usize as _) };
#[doc = "Code Watchdog Timer"]
pub const CDOG1: cdog::Cdog = unsafe { cdog::Cdog::from_ptr(0x4004_1000usize as _) };
#[doc = "Debug Mailbox"]
pub const DBGMAILBOX: dbgmailbox::Dbgmailbox =
    unsafe { dbgmailbox::Dbgmailbox::from_ptr(0x4004_2000usize as _) };
#[doc = "AHBSC"]
pub const AHBSC: ahbsc::Ahbsc = unsafe { ahbsc::Ahbsc::from_ptr(0x4004_4000usize as _) };
#[doc = "GPIO"]
pub const GPIO0: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4004_8000usize as _) };
#[doc = "GPIO"]
pub const GPIO1: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4004_a000usize as _) };
#[doc = "GPIO"]
pub const GPIO2: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4004_c000usize as _) };
#[doc = "GPIO"]
pub const GPIO3: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4004_e000usize as _) };
#[doc = "GPIO"]
pub const GPIO4: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4005_0000usize as _) };
#[doc = "DMA MP"]
pub const DMA0: dma::Dma = unsafe { dma::Dma::from_ptr(0x4008_0000usize as _) };
#[doc = "DMA TCD"]
pub const EDMA0_TCD0: edma0_tcd::Edma0Tcd =
    unsafe { edma0_tcd::Edma0Tcd::from_ptr(0x4008_1000usize as _) };
#[doc = "SYSCON"]
pub const SYSCON: syscon::Syscon = unsafe { syscon::Syscon::from_ptr(0x4009_1000usize as _) };
#[doc = "MRCC"]
pub const MRCC0: mrcc::Mrcc = unsafe { mrcc::Mrcc::from_ptr(0x4009_1800usize as _) };
#[doc = "Low-Leakage Wakeup Unit"]
pub const WUU0: wuu::Wuu = unsafe { wuu::Wuu::from_ptr(0x4009_2000usize as _) };
#[doc = "VBAT"]
pub const VBAT0: vbat::Vbat = unsafe { vbat::Vbat::from_ptr(0x4009_3000usize as _) };
#[doc = "FMC"]
pub const FMC0: fmc::Fmc = unsafe { fmc::Fmc::from_ptr(0x4009_4000usize as _) };
#[doc = "Flash"]
pub const FMU0: fmu::Fmu = unsafe { fmu::Fmu::from_ptr(0x4009_5000usize as _) };
#[doc = "Flexible I/O"]
pub const FLEXIO0: flexio::Flexio = unsafe { flexio::Flexio::from_ptr(0x4009_9000usize as _) };
#[doc = "Low-Power Inter-Integrated Circuit"]
pub const LPI2C0: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x4009_a000usize as _) };
#[doc = "Low-Power Inter-Integrated Circuit"]
pub const LPI2C1: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x4009_b000usize as _) };
#[doc = "Low-Power Serial Peripheral Interface"]
pub const LPSPI0: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4009_c000usize as _) };
#[doc = "Low-Power Serial Peripheral Interface"]
pub const LPSPI1: lpspi::Lpspi = unsafe { lpspi::Lpspi::from_ptr(0x4009_d000usize as _) };
#[doc = "Improved Inter-Integrated Circuit"]
pub const I3C2: i3c::I3c = unsafe { i3c::I3c::from_ptr(0x4009_e000usize as _) };
#[doc = "LPUART"]
pub const LPUART0: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x4009_f000usize as _) };
#[doc = "LPUART"]
pub const LPUART1: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400a_0000usize as _) };
#[doc = "LPUART"]
pub const LPUART2: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400a_1000usize as _) };
#[doc = "LPUART"]
pub const LPUART3: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400a_2000usize as _) };
#[doc = "LPUART"]
pub const LPUART4: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400a_3000usize as _) };
#[doc = "LPTMR"]
pub const LPTMR0: lptmr::Lptmr = unsafe { lptmr::Lptmr::from_ptr(0x400a_b000usize as _) };
#[doc = "OS Event Timer"]
pub const OSTIMER0: ostimer::Ostimer = unsafe { ostimer::Ostimer::from_ptr(0x400a_d000usize as _) };
#[doc = "WAKE_TIMER"]
pub const WAKETIMER0: waketimer::Waketimer =
    unsafe { waketimer::Waketimer::from_ptr(0x400a_e000usize as _) };
#[doc = "ADC"]
pub const ADC0: adc::Adc = unsafe { adc::Adc::from_ptr(0x400a_f000usize as _) };
#[doc = "ADC"]
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x400b_0000usize as _) };
#[doc = "LPCMP"]
pub const CMP0: cmp::Cmp = unsafe { cmp::Cmp::from_ptr(0x400b_1000usize as _) };
#[doc = "12-bit DAC"]
pub const DAC0: dac::Dac = unsafe { dac::Dac::from_ptr(0x400b_4000usize as _) };
#[doc = "12-bit DAC"]
pub const DAC1: dac::Dac = unsafe { dac::Dac::from_ptr(0x400b_5000usize as _) };
#[doc = "Voltage Reference"]
pub const VREF0: vref::Vref = unsafe { vref::Vref::from_ptr(0x400b_b000usize as _) };
#[doc = "Port Control"]
pub const PORT0: port::Port = unsafe { port::Port::from_ptr(0x400b_c000usize as _) };
#[doc = "Port Control"]
pub const PORT1: port::Port = unsafe { port::Port::from_ptr(0x400b_d000usize as _) };
#[doc = "Port Control"]
pub const PORT2: port::Port = unsafe { port::Port::from_ptr(0x400b_e000usize as _) };
#[doc = "Port Control"]
pub const PORT3: port::Port = unsafe { port::Port::from_ptr(0x400b_f000usize as _) };
#[doc = "Port Control"]
pub const PORT4: port::Port = unsafe { port::Port::from_ptr(0x400c_0000usize as _) };
#[doc = "DA_IP_TSI_UG_3V_CLN40ULP"]
pub const TSI0: tsi::Tsi = unsafe { tsi::Tsi::from_ptr(0x400c_3000usize as _) };
#[doc = "AOI"]
pub const AOI0: aoi::Aoi = unsafe { aoi::Aoi::from_ptr(0x400c_4000usize as _) };
#[doc = "CRC"]
pub const CRC0: crc::Crc = unsafe { crc::Crc::from_ptr(0x400c_5000usize as _) };
#[doc = "Core Mode Controller"]
pub const CMC: cmc::Cmc = unsafe { cmc::Cmc::from_ptr(0x400c_6000usize as _) };
#[doc = "Error Injection Module"]
pub const EIM0: eim::Eim = unsafe { eim::Eim::from_ptr(0x400c_7000usize as _) };
#[doc = "Error Reporting Module"]
pub const ERM0: erm::Erm = unsafe { erm::Erm::from_ptr(0x400c_8000usize as _) };
#[doc = "TRDC"]
pub const MBC0: mbc::Mbc = unsafe { mbc::Mbc::from_ptr(0x400c_9000usize as _) };
#[doc = "System Clock Generator"]
pub const SCG0: scg::Scg = unsafe { scg::Scg::from_ptr(0x400c_a000usize as _) };
#[doc = "System Power Control"]
pub const SPC0: spc::Spc = unsafe { spc::Spc::from_ptr(0x400c_b000usize as _) };
#[doc = "Flexible Data Rate CAN"]
pub const CAN0: can::Can = unsafe { can::Can::from_ptr(0x400c_c000usize as _) };
#[doc = "Flexible Data Rate CAN"]
pub const CAN1: can::Can = unsafe { can::Can::from_ptr(0x400d_0000usize as _) };
#[doc = "Low-Power Inter-Integrated Circuit"]
pub const LPI2C2: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x400d_4000usize as _) };
#[doc = "Low-Power Inter-Integrated Circuit"]
pub const LPI2C3: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x400d_5000usize as _) };
#[doc = "Low-Power Inter-Integrated Circuit"]
pub const LPI2C4: lpi2c::Lpi2c = unsafe { lpi2c::Lpi2c::from_ptr(0x400d_6000usize as _) };
#[doc = "LPUART"]
pub const LPUART5: lpuart::Lpuart = unsafe { lpuart::Lpuart::from_ptr(0x400d_a000usize as _) };
#[doc = "Improved Inter-Integrated Circuit"]
pub const I3C3: i3c::I3c = unsafe { i3c::I3c::from_ptr(0x400d_e000usize as _) };
#[doc = "GPIO"]
pub const GPIO5: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x400d_f000usize as _) };
#[doc = "Port Control"]
pub const PORT5: port::Port = unsafe { port::Port::from_ptr(0x400e_3000usize as _) };
#[doc = "no description available"]
pub const DGDET0: dgdet::Dgdet = unsafe { dgdet::Dgdet::from_ptr(0x400e_5000usize as _) };
#[doc = "ITRC"]
pub const ITRC0: itrc::Itrc = unsafe { itrc::Itrc::from_ptr(0x400e_7000usize as _) };
#[doc = "GLIKEY"]
pub const GLIKEY0: glikey::Glikey = unsafe { glikey::Glikey::from_ptr(0x400e_8000usize as _) };
#[doc = "TDET"]
pub const TDET0: tdet::Tdet = unsafe { tdet::Tdet::from_ptr(0x400e_9000usize as _) };
#[doc = "SECCON"]
pub const SECCON: seccon::Seccon = unsafe { seccon::Seccon::from_ptr(0x400e_a000usize as _) };
#[doc = "no description available"]
pub const SGI0: sgi::Sgi = unsafe { sgi::Sgi::from_ptr(0x400e_b000usize as _) };
#[doc = "pd_main.trng0"]
pub const TRNG0: trng::Trng = unsafe { trng::Trng::from_ptr(0x400e_c000usize as _) };
#[doc = "no description available"]
pub const UDF0: udf::Udf = unsafe { udf::Udf::from_ptr(0x400e_d000usize as _) };
#[doc = "Real Time Clock"]
pub const RTC0: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x400e_e000usize as _) };
pub const ADC2: adc::Adc = unsafe { adc::Adc::from_ptr(0x400f_0000usize as _) };
pub const ADC3: adc::Adc = unsafe { adc::Adc::from_ptr(0x400f_1000usize as _) };
#[doc = "System Control not in System Control Block"]
pub const SCNSCB: scn_scb::ScnScb = unsafe { scn_scb::ScnScb::from_ptr(0xe000_e000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
pub mod adc;
pub mod ahbsc;
pub mod aoi;
pub mod can;
pub mod cdog;
pub mod cmc;
pub mod cmp;
pub mod common;
pub mod crc;
pub mod ctimer;
pub mod dac;
pub mod dbgmailbox;
pub mod dgdet;
pub mod dma;
pub mod dma1;
pub mod edma0_tcd;
pub mod edma1_tcd;
pub mod edma_0_tcd;
pub mod eim;
pub mod enet;
pub mod enet0__eqos_dma;
pub mod enet0__eqos_dma_ch;
pub mod enet0__eqos_mtl;
pub mod enet0__eqos_mtl_q;
pub mod erm;
pub mod espi;
pub mod ewm;
pub mod flexio;
pub mod flexspi;
pub mod fmc;
pub mod fmu;
pub mod freqme;
pub mod glikey;
pub mod gpio;
pub mod i3c;
pub mod i3c1;
pub mod inputmux;
pub mod itrc;
pub mod lpi2c;
pub mod lpspi;
pub mod lptmr;
pub mod lpuart;
pub mod mbc;
pub mod mrcc;
pub mod ostimer;
pub mod pkc;
pub mod port;
pub mod rtc;
pub mod scg;
pub mod scn_scb;
pub mod seccon;
pub mod sgi;
pub mod smartdma;
pub mod spc;
pub mod spi_filter;
pub mod syscon;
pub mod t1s;
pub mod tdet;
pub mod trng;
pub mod tsi;
pub mod udf;
pub mod usb;
pub mod usbnc;
pub mod usbphy;
pub mod utick;
pub mod vbat;
pub mod vref;
pub mod waketimer;
pub mod wuu;
pub mod wwdt;
