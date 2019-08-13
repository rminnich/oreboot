/* Copyright (c) 2018 SiFive, Inc */
/* SPDX-License-Identifier: Apache-2.0 */
/* SPDX-License-Identifier: GPL-2.0-or-later */
/* See the file LICENSE for further information */

use core::ptr;
use crate::reg;
use crate::ddrregs;

// #define _REG32((DDR_CTRL,p, i) (*(volatile uint32_t *)((p) + (i)))
pub const DRAM_CLASS_OFFSET: u32=8;
pub const DRAM_CLASS_DDR4: u32=0xA;
pub const OPTIMAL_RMODW_EN_OFFSET: u32=0;
pub const DISABLE_RD_INTERLEAVE_OFFSET: u32=16;
pub const OUT_OF_RANGE_OFFSET: u32=1;
pub const MULTIPLE_OUT_OF_RANGE_OFFSET: u32=2;
pub const PORT_COMMAND_CHANNEL_ERROR_OFFSET: u32=7;
pub const MC_INIT_COMPLETE_OFFSET: u32=8;
pub const LEVELING_OPERATION_COMPLETED_OFFSET: u32=22;
pub const DFI_PHY_WRLELV_MODE_OFFSET: u32=24;
pub const DFI_PHY_RDLVL_MODE_OFFSET: u32=24;
pub const DFI_PHY_RDLVL_GATE_MODE_OFFSET: u32=0;
pub const VREF_EN_OFFSET: u32=24;
pub const PORT_ADDR_PROTECTION_EN_OFFSET: u32=0;
pub const AXI0_ADDRESS_RANGE_ENABLE: u32=8;
pub const AXI0_RANGE_PROT_BITS_0_OFFSET: u32=24;
pub const RDLVL_EN_OFFSET: u32=16;
pub const RDLVL_GATE_EN_OFFSET: u32=24;
pub const WRLVL_EN_OFFSET: u32=0;

pub const PHY_RX_CAL_DQ0_0_OFFSET: u64=             0;
pub const PHY_RX_CAL_DQ1_0_OFFSET: u64 =              16;

// This is nasty but at the same time, the way the code is written, we need to
// make this change slowly.

// This is a 64-bit machine but all this action seems to be on 32-bit values.
// No idea why this is.
fn poke(Pointer:u32, Index: u32, Value: u32) -> () {
    let addr = (Pointer + Index) as *mut u32;
    unsafe {
        ptr::write_volatile(addr, Value.into());
    }
}

fn peek(Pointer:u32, Index: u32) -> u32 {
    let addr = (Pointer + Index) as *const u32;
    unsafe { ptr::read_volatile(addr) }
}

// #define _REG32((DDR_CTRL,p, i) (*(volatile uint32_t *)((p) + (i)))

fn phy_reset() {
    for i in 1152..1214 {
        poke(reg::DDR_PHY, i as u32, ddrregs::DenaliPhyData[i] );
        //uint32_t physet = DenaliPhyData[i];
        // /*if (physet!=0)*/ DDR_PHY[i] = physet;
    }
    for i in 0..1151 {
        poke(reg::DDR_PHY, i as u32, ddrregs::DenaliPhyData[i] );
        //for (i=0;i<=1151;i++) {
        //    uint32_t physet = DenaliPhyData[i];
        //if (physet!=0)*/ DDR_PHY[i] = physet;
    }
}


fn ux00ddr_writeregmap() {
    for i in 0..265 {
        //  for (i=0;i<=264;i++) {
        poke(reg::DDR_CTRL, i as u32,ddrregs::DenaliCtlData[i]  );
        // uint32_t ctlset = DenaliCtlData[i];
        // /*if (ctlset!=0)*/ DDR_CTRL[i] = ctlset;
  }

    phy_reset();
}

// static inline void ux00ddr_start(size_t DDR_CTRL, size_t filteraddr, size_t ddrend) {
pub fn ux00ddr_start(filteraddr: usize, ddrend: usize){
//   // START register at ddrctl register base offset 0
//   uint32_t regdata = _REG32((DDR_CTRL,0);
//   regdata |= 0x1;
//   _REG32((DDR_CTRL,0) = regdata;
//   // WAIT for initialization complete : bit 8 of INT_STATUS (DENALI_CTL_132) 0x210
//   while ((_REG32((DDR_CTRL,132) & (1<<MC_INIT_COMPLETE_OFFSET)) == 0) {}

//   // Disable the BusBlocker in front of the controller AXI slave ports
//   volatile uint64_t *filterreg = (volatile uint64_t *)filteraddr;
//   filterreg[0] = 0x0f00000000000000UL | (ddrend >> 2);
//   //                ^^ RWX + TOR
// }
}

// static inline void ux00ddr_mask_mc_init_complete_interrupt(size_t DDR_CTRL) {
//   // Mask off Bit 8 of Interrupt Status
//   // Bit [8] The MC initialization has been completed
//   _REG32((DDR_CTRL,136) |= (1<<MC_INIT_COMPLETE_OFFSET);
// }

// static inline void ux00ddr_mask_outofrange_interrupts(size_t DDR_CTRL) {
//   // Mask off Bit 8, Bit 2 and Bit 1 of Interrupt Status
//   // Bit [2] Multiple accesses outside the defined PHYSICAL memory space have occured
//   // Bit [1] A memory access outside the defined PHYSICAL memory space has occured
//   _REG32((DDR_CTRL,136) |= ((1<<OUT_OF_RANGE_OFFSET) | (1<<MULTIPLE_OUT_OF_RANGE_OFFSET));
// }

// static inline void ux00ddr_mask_port_command_error_interrupt(size_t DDR_CTRL) {
//   // Mask off Bit 7 of Interrupt Status
//   // Bit [7] An error occured on the port command channel
//   _REG32((DDR_CTRL,136) |= (1<<PORT_COMMAND_CHANNEL_ERROR_OFFSET);
// }

// static inline void ux00ddr_mask_leveling_completed_interrupt(size_t DDR_CTRL) {
//   // Mask off Bit 22 of Interrupt Status
//   // Bit [22] The leveling operation has completed
//   _REG32((DDR_CTRL,136) |= (1<<LEVELING_OPERATION_COMPLETED_OFFSET);
// }

// static inline void ux00ddr_setuprangeprotection(size_t DDR_CTRL, size_t end_addr) {
//   _REG32((DDR_CTRL,209) = 0x0;
//   size_t end_addr_16Kblocks = ((end_addr >> 14) & 0x7FFFFF)-1;
//   _REG32((DDR_CTRL,210) = ((uint32_t) end_addr_16Kblocks);
//   _REG32((DDR_CTRL,212) = 0x0;
//   _REG32((DDR_CTRL,214) = 0x0;
//   _REG32((DDR_CTRL,216) = 0x0;
//   _REG32((DDR_CTRL,224) |= (0x3 << AXI0_RANGE_PROT_BITS_0_OFFSET);
//   _REG32((DDR_CTRL,225) = 0xFFFFFFFF;
//   _REG32((DDR_CTRL,208) |= (1 << AXI0_ADDRESS_RANGE_ENABLE);
//   _REG32((DDR_CTRL,208) |= (1 << PORT_ADDR_PROTECTION_EN_OFFSET);

// }

// static inline void ux00ddr_disableaxireadinterleave(size_t DDR_CTRL) {
//   _REG32((DDR_CTRL,120) |= (1<<DISABLE_RD_INTERLEAVE_OFFSET);
// }

// static inline void ux00ddr_disableoptimalrmodw(size_t DDR_CTRL) {
//   _REG32((DDR_CTRL,21) &= (~(1<<OPTIMAL_RMODW_EN_OFFSET));
// }

// static inline void ux00ddr_enablewriteleveling(size_t DDR_CTRL) {
//   _REG32((DDR_CTRL,170) |= ((1<<WRLVL_EN_OFFSET) | (1<<DFI_PHY_WRLELV_MODE_OFFSET));
// }

// static inline void ux00ddr_enablereadleveling(size_t DDR_CTRL) {
//   _REG32((DDR_CTRL,181) |= (1<<DFI_PHY_RDLVL_MODE_OFFSET);
//   _REG32((DDR_CTRL,260) |= (1<<RDLVL_EN_OFFSET);
// }

// static inline void ux00ddr_enablereadlevelinggate(size_t DDR_CTRL) {
//   _REG32((DDR_CTRL,260) |= (1<<RDLVL_GATE_EN_OFFSET);
//   _REG32((DDR_CTRL,182) |= (1<<DFI_PHY_RDLVL_GATE_MODE_OFFSET);
// }

// static inline void ux00ddr_enablevreftraining(size_t DDR_CTRL) {
//   _REG32((DDR_CTRL,184) |= (1<<VREF_EN_OFFSET);
// }

// static inline uint32_t ux00ddr_getdramclass(size_t DDR_CTRL) {
//   return ((_REG32((DDR_CTRL,0) >> DRAM_CLASS_OFFSET) & 0xF);
// }

// static inline uint64_t ux00ddr_phy_fixup(size_t DDR_CTRL) {
//   // return bitmask of failed lanes

//   size_t ddrphyreg = DDR_CTRL + 0x2000;

//   uint64_t fails=0;
//   uint32_t slicebase = 0;
//   uint32_t dq = 0;

//   // check errata condition
//   for (uint32_t slice = 0; slice < 8; slice++) {
//     uint32_t regbase = slicebase + 34;
//     for (uint32_t reg = 0 ; reg < 4; reg++) {
//       uint32_t updownreg = _REG32((DDR_CTRL,(regbase+reg), ddrphyreg);
//       for (uint32_t bit = 0; bit < 2; bit++) {
//         uint32_t phy_rx_cal_dqn_0_offset;

//         if (bit==0) {
//           phy_rx_cal_dqn_0_offset = PHY_RX_CAL_DQ0_0_OFFSET;
//         }else{
//           phy_rx_cal_dqn_0_offset = PHY_RX_CAL_DQ1_0_OFFSET;
//         }

//         uint32_t down = (updownreg >> phy_rx_cal_dqn_0_offset) & 0x3F;
//         uint32_t up = (updownreg >> (phy_rx_cal_dqn_0_offset+6)) & 0x3F;

//         uint8_t failc0 = ((down == 0) && (up == 0x3F));
//         uint8_t failc1 = ((up == 0) && (down == 0x3F));

//         // print error message on failure
//         if (failc0 || failc1) {
//           //if (fails==0) uart_puts((void*) UART0_CTRL_ADDR, "DDR error in fixing up \n");
//           fails |= (1<<dq);
//           char slicelsc = '0';
//           char slicemsc = '0';
//           slicelsc += (dq % 10);
//           slicemsc += (dq / 10);
//           //uart_puts((void*) UART0_CTRL_ADDR, "S ");
//           //uart_puts((void*) UART0_CTRL_ADDR, &slicemsc);
//           //uart_puts((void*) UART0_CTRL_ADDR, &slicelsc);
//           //if (failc0) uart_puts((void*) UART0_CTRL_ADDR, "U");
//           //else uart_puts((void*) UART0_CTRL_ADDR, "D");
//           //uart_puts((void*) UART0_CTRL_ADDR, "\n");
//         }
//         dq++;
//       }
//     }
//     slicebase+=128;
//   }
//   return (0);
// }

// #endif

// #endif
