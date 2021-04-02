use crate::rom_lookup::{rom_lookup, RomIndex};
use crate::sflash::SPI_Flash_Cfg_Type;
use crate::sflash::BL_Err_Type;

extern "C" {
    #[doc = " @defgroup  XIP_SFLASH_Public_Functions"]
    #[doc = "  @{"]
    pub fn XIP_SFlash_State_Save(
        pFlashCfg: *mut SPI_Flash_Cfg_Type,
        offset: *mut u32,
    ) -> BL_Err_Type;


    pub fn XIP_SFlash_State_Restore(pFlashCfg: *mut SPI_Flash_Cfg_Type, offset: u32)
        -> BL_Err_Type;


    pub fn XIP_SFlash_Erase_Need_Lock(
        pFlashCfg: *mut SPI_Flash_Cfg_Type,
        startaddr: u32,
        endaddr: u32,
    ) -> BL_Err_Type;


    pub fn XIP_SFlash_Write_Need_Lock(
        pFlashCfg: *mut SPI_Flash_Cfg_Type,
        addr: u32,
        data: *mut u8,
        len: u32,
    ) -> BL_Err_Type;


    pub fn XIP_SFlash_Read_Need_Lock(
        pFlashCfg: *mut SPI_Flash_Cfg_Type,
        addr: u32,
        data: *mut u8,
        len: u32,
    ) -> BL_Err_Type;


    pub fn XIP_SFlash_GetJedecId_Need_Lock(
        pFlashCfg: *mut SPI_Flash_Cfg_Type,
        data: *mut u8,
    ) -> BL_Err_Type;


    pub fn XIP_SFlash_GetDeviceId_Need_Lock(
        pFlashCfg: *mut SPI_Flash_Cfg_Type,
        data: *mut u8,
    ) -> BL_Err_Type;


    pub fn XIP_SFlash_GetUniqueId_Need_Lock(
        pFlashCfg: *mut SPI_Flash_Cfg_Type,
        data: *mut u8,
        idLen: u8,
    ) -> BL_Err_Type;


    pub fn XIP_SFlash_Read_Via_Cache_Need_Lock(addr: u32, data: *mut u8, len: u32) -> BL_Err_Type;


    pub fn XIP_SFlash_Read_With_Lock(
        pFlashCfg: *mut SPI_Flash_Cfg_Type,
        addr: u32,
        dst: *mut u8,
        len: u32,
    ) -> u32;


    pub fn XIP_SFlash_Write_With_Lock(
        pFlashCfg: *mut SPI_Flash_Cfg_Type,
        addr: u32,
        src: *mut u8,
        len: u32,
    ) -> u32;


    pub fn XIP_SFlash_Erase_With_Lock(
        pFlashCfg: *mut SPI_Flash_Cfg_Type,
        addr: u32,
        len: u32,
    ) -> u32;


    pub fn XIP_SFlash_Opt_Enter(aesEnable: *mut u8);


    pub fn XIP_SFlash_Opt_Exit(aesEnable: u8);
}