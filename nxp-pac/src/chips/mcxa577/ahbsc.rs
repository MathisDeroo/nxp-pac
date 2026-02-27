#[doc = "AHBSC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbsc {
    ptr: *mut u8,
}
unsafe impl Send for Ahbsc {}
unsafe impl Sync for Ahbsc {}
impl Ahbsc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Flash Memory Rule"]
    #[inline(always)]
    pub const fn flash00_mem_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Flash00MemRule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 4usize) as _) }
    }
    #[doc = "Flash Memory Rule"]
    #[inline(always)]
    pub const fn flash01_mem_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Flash01MemRule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "Flash IFR0 Rule register"]
    #[inline(always)]
    pub const fn flash02_mem_rule(
        self,
    ) -> crate::common::Reg<regs::Flash02MemRule, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Flash Memory Rule"]
    #[inline(always)]
    pub const fn flash03_mem_rule(
        self,
    ) -> crate::common::Reg<regs::Flash03MemRule, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "ROM Memory Rule"]
    #[inline(always)]
    pub const fn rom_mem_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::RomMemRule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 4usize) as _) }
    }
    #[doc = "RAMX Memory Rule"]
    #[inline(always)]
    pub const fn ramx_mem_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::RamxMemRule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "RAMA Memory Rule 0"]
    #[inline(always)]
    pub const fn rama_mem_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::RamaMemRule, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 4usize) as _) }
    }
    #[doc = "RAMB Memory Rule 0"]
    #[inline(always)]
    pub const fn ramb_mem_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::RambMemRule, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize + n * 4usize) as _) }
    }
    #[doc = "AHB Slave Port 5 Rule Register"]
    #[inline(always)]
    pub const fn ahb_slave_port_p5_slave_rule0(
        self,
    ) -> crate::common::Reg<regs::AhbSlavePortP5SlaveRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "APB Bridge Group 0 Memory Rule Register 0"]
    #[inline(always)]
    pub const fn apb_peripheral_group0_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::ApbPeripheralGroup0MemRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "APB Bridge Group 0 Memory Rule Register 1"]
    #[inline(always)]
    pub const fn apb_peripheral_group0_mem_rule1(
        self,
    ) -> crate::common::Reg<regs::ApbPeripheralGroup0MemRule1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "AIPS Bridge Group 0 Memory Rule 0"]
    #[inline(always)]
    pub const fn aips_bridge_group0_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup0MemRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "AIPS Bridge Group 0 Memory Rule 1"]
    #[inline(always)]
    pub const fn aips_bridge_group0_mem_rule1(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup0MemRule1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "AIPS Bridge Group 1 Memory Rule 0"]
    #[inline(always)]
    pub const fn aips_bridge_group1_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup1MemRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "AIPS Bridge Group 1 Memory Rule 1"]
    #[inline(always)]
    pub const fn aips_bridge_group1_mem_rule1(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup1MemRule1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "AHB Secure Control Peripheral Rule"]
    #[inline(always)]
    pub const fn ahb_secure_ctrl_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::AhbSecureCtrlMemRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "AHB Peripheral 0 Memory Rule 1"]
    #[inline(always)]
    pub const fn ahb_peripheral0_mem_rule1(
        self,
    ) -> crate::common::Reg<regs::AhbPeripheral0MemRule1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "AHB Peripheral 0 Memory Rule 2"]
    #[inline(always)]
    pub const fn ahb_peripheral0_mem_rule2(
        self,
    ) -> crate::common::Reg<regs::AhbPeripheral0MemRule2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "AHB Peripheral 0 Memory Rule 3"]
    #[inline(always)]
    pub const fn ahb_peripheral0_mem_rule3(
        self,
    ) -> crate::common::Reg<regs::AhbPeripheral0MemRule3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "AHB Peripheral 0 Memory Rule 4"]
    #[inline(always)]
    pub const fn ahb_peripheral0_mem_rule4(
        self,
    ) -> crate::common::Reg<regs::AhbPeripheral0MemRule4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "AHB Peripheral 0 Memory Rule 5"]
    #[inline(always)]
    pub const fn ahb_peripheral0_mem_rule5(
        self,
    ) -> crate::common::Reg<regs::AhbPeripheral0MemRule5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 0"]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule0(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup2MemRule0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 1"]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule1(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup2MemRule1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 2"]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule2(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup2MemRule2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 3"]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule3(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup2MemRule3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 4"]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule4(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup2MemRule4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 5"]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule5(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup2MemRule5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 6"]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule6(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup2MemRule6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 7"]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule7(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup2MemRule7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 8"]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule8(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup2MemRule8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 9"]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule9(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup2MemRule9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 10"]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule10(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup2MemRule10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 11"]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule11(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup2MemRule11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 12"]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule12(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup2MemRule12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "AIPS Bridge Group 2 Rule 13"]
    #[inline(always)]
    pub const fn aips_bridge_group2_mem_rule13(
        self,
    ) -> crate::common::Reg<regs::AipsBridgeGroup2MemRule13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "FLEXSPI0 Region 0 Memory Rule"]
    #[inline(always)]
    pub const fn flexspi0_region0_mem_rule(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Flexspi0Region0MemRule, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize + n * 4usize) as _)
        }
    }
    #[doc = "Array of registers: FLEXSPI0_REGION_MEM_RULE"]
    #[inline(always)]
    pub const fn flexspi0_region1_6_mem_rule(self, n: usize) -> Flexspi0Region16MemRule {
        assert!(n < 6usize);
        unsafe {
            Flexspi0Region16MemRule::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 16usize) as _)
        }
    }
    #[doc = "Security Violation Address"]
    #[inline(always)]
    pub const fn sec_vio_addr(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SecVioAddr, crate::common::R> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e00usize + n * 4usize) as _)
        }
    }
    #[doc = "Security Violation Miscellaneous Information at Address"]
    #[inline(always)]
    pub const fn sec_vio_misc_info(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SecVioMiscInfo, crate::common::R> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e80usize + n * 4usize) as _)
        }
    }
    #[doc = "Security Violation Info Validity for Address"]
    #[inline(always)]
    pub const fn sec_vio_info_valid(
        self,
    ) -> crate::common::Reg<regs::SecVioInfoValid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f00usize) as _) }
    }
    #[doc = "Secure general purpose registers"]
    #[inline(always)]
    pub const fn sec_gp_reg0(self) -> crate::common::Reg<regs::SecGpReg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f80usize) as _) }
    }
    #[doc = "Secure general purpose registers"]
    #[inline(always)]
    pub const fn sec_gp_reg1(self) -> crate::common::Reg<regs::SecGpReg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f84usize) as _) }
    }
    #[doc = "Secure general purpose registers"]
    #[inline(always)]
    pub const fn sec_gp_reg2(self) -> crate::common::Reg<regs::SecGpReg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f88usize) as _) }
    }
    #[doc = "Secure general purpose registers"]
    #[inline(always)]
    pub const fn sec_gp_reg3(self) -> crate::common::Reg<regs::SecGpReg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f8cusize) as _) }
    }
    #[doc = "Secure general purpose registers"]
    #[inline(always)]
    pub const fn sec_gp_reg4(self) -> crate::common::Reg<regs::SecGpReg4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f90usize) as _) }
    }
    #[doc = "Secure general purpose registers"]
    #[inline(always)]
    pub const fn sec_gp_reg5(self) -> crate::common::Reg<regs::SecGpReg5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f94usize) as _) }
    }
    #[doc = "Secure general purpose registers"]
    #[inline(always)]
    pub const fn sec_gp_reg6(self) -> crate::common::Reg<regs::SecGpReg6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f98usize) as _) }
    }
    #[doc = "Secure general purpose registers"]
    #[inline(always)]
    pub const fn sec_gp_reg7(self) -> crate::common::Reg<regs::SecGpReg7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f9cusize) as _) }
    }
    #[doc = "Secure general purpose registers"]
    #[inline(always)]
    pub const fn sec_gp_reg8(self) -> crate::common::Reg<regs::SecGpReg8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa0usize) as _) }
    }
    #[doc = "Secure general purpose registers"]
    #[inline(always)]
    pub const fn sec_gp_reg9(self) -> crate::common::Reg<regs::SecGpReg9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa4usize) as _) }
    }
    #[doc = "Master Secure Level"]
    #[inline(always)]
    pub const fn master_sec_level(
        self,
    ) -> crate::common::Reg<regs::MasterSecLevel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd0usize) as _) }
    }
    #[doc = "Master Secure Level"]
    #[inline(always)]
    pub const fn master_sec_anti_pol_reg(
        self,
    ) -> crate::common::Reg<regs::MasterSecAntiPolReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd4usize) as _) }
    }
    #[doc = "Miscellaneous CPU0 Control Signals"]
    #[inline(always)]
    pub const fn cpu0_lock_reg(self) -> crate::common::Reg<regs::Cpu0LockReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fecusize) as _) }
    }
    #[doc = "Secure Control Duplicate"]
    #[inline(always)]
    pub const fn misc_ctrl_dp_reg(
        self,
    ) -> crate::common::Reg<regs::MiscCtrlDpReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
    #[doc = "Secure Control"]
    #[inline(always)]
    pub const fn misc_ctrl_reg(self) -> crate::common::Reg<regs::MiscCtrlReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
#[doc = "Array of registers: FLEXSPI0_REGION_MEM_RULE"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi0Region16MemRule {
    ptr: *mut u8,
}
unsafe impl Send for Flexspi0Region16MemRule {}
unsafe impl Sync for Flexspi0Region16MemRule {}
impl Flexspi0Region16MemRule {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "FLEXSPI0 Region index Memory Rule"]
    #[inline(always)]
    pub const fn flexspi0_region_mem_rule(
        self,
    ) -> crate::common::Reg<regs::Flexspi0RegionMemRule, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
pub mod regs;
pub mod vals;
