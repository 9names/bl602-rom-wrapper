#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// use num_enum::IntoPrimitive;
#[allow(dead_code)]
pub type ROM_API_INDEX_e = usize;
#[allow(dead_code)]
pub const ROM_APITABLE_ADDR: usize = 0x2101_0800;

#[allow(dead_code)]
#[repr(usize)]
pub enum RomIndex {
    VERSION = 0,
    RSVD_0 = 1,
    RSVD_1 = 2,
    RSVD_LAST = 3,
    AON_Power_On_MBG = 4,
    AON_Power_Off_MBG = 5,
    AON_Power_On_XTAL = 6,
    AON_Set_Xtal_CapCode = 7,
    AON_Get_Xtal_CapCode = 8,
    AON_Power_Off_XTAL = 9,
    AON_Power_On_BG = 10,
    AON_Power_Off_BG = 11,
    AON_Power_On_LDO11_SOC = 12,
    AON_Power_Off_LDO11_SOC = 13,
    AON_Power_On_LDO15_RF = 14,
    AON_Power_Off_LDO15_RF = 15,
    AON_Power_On_SFReg = 16,
    AON_Power_Off_SFReg = 17,
    AON_LowPower_Enter_PDS0 = 18,
    AON_LowPower_Exit_PDS0 = 19,
    ASM_Delay_Us = 20,
    BL602_Delay_US = 21,
    BL602_Delay_MS = 22,
    BL602_MemCpy = 23,
    BL602_MemCpy4 = 24,
    BL602_MemCpy_Fast = 25,
    BL602_MemSet = 26,
    BL602_MemSet4 = 27,
    BL602_MemCmp = 28,
    EF_Ctrl_Sw_AHB_Clk_0 = 29,
    EF_Ctrl_Program_Efuse_0 = 30,
    EF_Ctrl_Load_Efuse_R0 = 31,
    EF_Ctrl_Busy = 32,
    EF_Ctrl_AutoLoad_Done = 33,
    EF_Ctrl_Get_Trim_Parity = 34,
    EF_Ctrl_Read_RC32M_Trim = 35,
    EF_Ctrl_Read_RC32K_Trim = 36,
    EF_Ctrl_Clear = 37,
    GLB_Get_Root_CLK_Sel = 38,
    GLB_Set_System_CLK_Div = 39,
    GLB_Get_BCLK_Div = 40,
    GLB_Get_HCLK_Div = 41,
    Update_SystemCoreClockWith_XTAL = 42,
    GLB_Set_System_CLK = 43,
    System_Core_Clock_Update_From_RC32M = 44,
    GLB_Set_SF_CLK = 45,
    GLB_Set_PKA_CLK_Sel = 46,
    GLB_SW_System_Reset = 47,
    GLB_SW_CPU_Reset = 48,
    GLB_SW_POR_Reset = 49,
    GLB_Select_Internal_Flash = 50,
    GLB_Select_External_Flash = 51,
    GLB_Deswap_Flash_Pin = 52,
    GLB_Swap_Flash_Pin = 53,
    GLB_GPIO_Init = 54,
    GLB_GPIO_OUTPUT_Enable = 55,
    GLB_GPIO_OUTPUT_Disable = 56,
    GLB_GPIO_Set_HZ = 57,
    GLB_GPIO_Get_Fun = 58,
    HBN_Mode_Enter = 59,
    HBN_Power_Down_Flash = 60,
    HBN_Enable = 61,
    HBN_Reset = 62,
    HBN_Set_Ldo11_Aon_Vout = 63,
    HBN_Set_Ldo11_Rt_Vout = 64,
    HBN_Set_Ldo11_Soc_Vout = 65,
    HBN_32K_Sel = 66,
    HBN_Set_ROOT_CLK_Sel = 67,
    HBN_Power_On_Xtal_32K = 68,
    HBN_Power_Off_Xtal_32K = 69,
    HBN_Power_On_RC32K = 70,
    HBN_Power_Off_RC32K = 71,
    HBN_Trim_RC32K = 72,
    HBN_Hw_Pu_Pd_Cfg = 73,
    HBN_Pin_WakeUp_Mask = 74,
    HBN_GPIO7_Dbg_Pull_Cfg = 75,
    HBN_Set_Embedded_Flash_Pullup = 76,
    L1C_Set_Wrap = 77,
    L1C_Set_Way_Disable = 78,
    L1C_IROM_2T_Access_Set = 79,
    PDS_Reset = 80,
    PDS_Enable = 81,
    PDS_Force_Config = 82,
    PDS_RAM_Config = 83,
    PDS_Default_Level_Config = 84,
    PDS_Trim_RC32M = 85,
    PDS_Select_RC32M_As_PLL_Ref = 86,
    PDS_Select_XTAL_As_PLL_Ref = 87,
    PDS_Power_On_PLL = 88,
    PDS_Enable_PLL_All_Clks = 89,
    PDS_Disable_PLL_All_Clks = 90,
    PDS_Enable_PLL_Clk = 91,
    PDS_Disable_PLL_Clk = 92,
    PDS_Power_Off_PLL = 93,
    SEC_Eng_Turn_On_Sec_Ring = 94,
    SEC_Eng_Turn_Off_Sec_Ring = 95,
    SFlash_Init = 96,
    SFlash_SetSPIMode = 97,
    SFlash_Read_Reg = 98,
    SFlash_Write_Reg = 99,
    SFlash_Busy = 100,
    SFlash_Write_Enable = 101,
    SFlash_Qspi_Enable = 102,
    SFlash_Volatile_Reg_Write_Enable = 103,
    SFlash_Chip_Erase = 104,
    SFlash_Sector_Erase = 105,
    SFlash_Blk32_Erase = 106,
    SFlash_Blk64_Erase = 107,
    SFlash_Erase = 108,
    SFlash_Program = 109,
    SFlash_GetUniqueId = 110,
    SFlash_GetJedecId = 111,
    SFlash_GetDeviceId = 112,
    SFlash_Powerdown = 113,
    SFlash_Releae_Powerdown = 114,
    SFlash_SetBurstWrap = 115,
    SFlash_DisableBurstWrap = 116,
    SFlash_Software_Reset = 117,
    SFlash_Reset_Continue_Read = 118,
    SFlash_Set_IDbus_Cfg = 119,
    SFlash_IDbus_Read_Enable = 120,
    SFlash_Cache_Enable_Set = 121,
    SFlash_Cache_Flush = 122,
    SFlash_Cache_Read_Enable = 123,
    SFlash_Cache_Hit_Count_Get = 124,
    SFlash_Cache_Miss_Count_Get = 125,
    SFlash_Cache_Read_Disable = 126,
    SFlash_Read = 127,
    SFlash_Read_Reg_With_Cmd = 128,
    SFlash_Write_Reg_With_Cmd = 129,
    SFlash_Restore_From_Powerdown = 130,
    SF_Cfg_Init_Ext_Flash_Gpio = 131,
    SF_Cfg_Init_Internal_Flash_Gpio = 132,
    SF_Cfg_Deinit_Ext_Flash_Gpio = 133,
    SF_Cfg_Restore_GPIO17_Fun = 134,
    SF_Cfg_Get_Flash_Cfg_Need_Lock = 135,
    SF_Cfg_Init_Flash_Gpio = 136,
    SF_Cfg_Flash_Identify = 137,
    SF_Ctrl_Enable = 138,
    SF_Ctrl_Select_Pad = 139,
    SF_Ctrl_Set_Owner = 140,
    SF_Ctrl_Disable = 141,
    SF_Ctrl_AES_Enable_BE = 142,
    SF_Ctrl_AES_Enable_LE = 143,
    SF_Ctrl_AES_Set_Region = 144,
    SF_Ctrl_AES_Set_Key = 145,
    SF_Ctrl_AES_Set_Key_BE = 146,
    SF_Ctrl_AES_Set_IV = 147,
    SF_Ctrl_AES_Set_IV_BE = 148,
    SF_Ctrl_AES_Enable = 149,
    SF_Ctrl_AES_Disable = 150,
    SF_Ctrl_Set_Flash_Image_Offset = 151,
    SF_Ctrl_Get_Flash_Image_Offset = 152,
    SF_Ctrl_Select_Clock = 153,
    SF_Ctrl_SendCmd = 154,
    SF_Ctrl_Icache_Set = 155,
    SF_Ctrl_Icache2_Set = 156,
    SF_Ctrl_GetBusyState = 157,
    SF_Ctrl_Is_AES_Enable = 158,
    SF_Ctrl_Get_Clock_Delay = 159,
    SF_Ctrl_Set_Clock_Delay = 160,
    XIP_SFlash_State_Save = 161,
    XIP_SFlash_State_Restore = 162,
    XIP_SFlash_Erase_Need_Lock = 163,
    XIP_SFlash_Write_Need_Lock = 164,
    XIP_SFlash_Read_Need_Lock = 165,
    XIP_SFlash_GetJedecId_Need_Lock = 166,
    XIP_SFlash_GetDeviceId_Need_Lock = 167,
    XIP_SFlash_GetUniqueId_Need_Lock = 168,
    XIP_SFlash_Read_Via_Cache_Need_Lock = 169,
    XIP_SFlash_Read_With_Lock = 170,
    XIP_SFlash_Write_With_Lock = 171,
    XIP_SFlash_Erase_With_Lock = 172,
    XIP_SFlash_Opt_Enter = 173,
    XIP_SFlash_Opt_Exit = 174,
    BFLB_Soft_CRC32 = 175,
    FUNC_EMPTY_START = 176,
    FUNC_EMPTY_END = 511,
}

#[inline(always)]
pub fn rom_lookup(index: RomIndex) -> *const () {
    let rom_function_table_index = ROM_APITABLE_ADDR + (index as usize * 4);
    let rom_function_addr = unsafe { (rom_function_table_index as *mut usize).read_volatile() };
    rom_function_addr as *const ()
}

#[cfg(test)]
fn pds_power_on_pll(xtal_src: u32) -> usize {
    let romdriver_pds_power_on_pll = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(usize) -> usize>(rom_lookup(
            RomIndex::PDS_Power_On_PLL,
        ))
    };

    romdriver_pds_power_on_pll(xtal_src as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        pds_power_on_pll(32000);
    }
}
