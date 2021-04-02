use crate::rom_lookup::{rom_lookup, RomIndex};
use crate::rom::{SF_Ctrl_Cfg_Type, BL_Err_Type, BL_Sts_Type, SF_Ctrl_IO_Type, SF_Ctrl_Mode_Type, SPI_Flash_Cfg_Type};

#[inline(always)]
pub fn SFlash_Init(sfCtrlCfg: *const SF_Ctrl_Cfg_Type) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*const SF_Ctrl_Cfg_Type)>(rom_lookup(
            RomIndex::SFlash_Init,
        ))
    };
    romdriver_func(sfCtrlCfg)
}

#[inline(always)]
pub fn SFlash_SetSPIMode(mode: SF_Ctrl_Mode_Type) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(SF_Ctrl_Mode_Type) -> u32>(rom_lookup(
            RomIndex::SFlash_SetSPIMode,
        ))
    };
    romdriver_func(mode)
}

#[inline(always)]
pub fn SFlash_Read_Reg(
    flashCfg: *mut SPI_Flash_Cfg_Type,
    regIndex: u8,
    regValue: *mut u8,
    regLen: u8,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, u8, *mut u8, u8) -> BL_Err_Type,
        >(rom_lookup(RomIndex::SFlash_Read_Reg))
    };
    romdriver_func(flashCfg, regIndex, regValue, regLen)
}

#[inline(always)]
pub fn SFlash_Write_Reg(
    flashCfg: *mut SPI_Flash_Cfg_Type,
    regIndex: u8,
    regValue: *mut u8,
    regLen: u8,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, u8, *mut u8, u8) -> BL_Err_Type,
        >(rom_lookup(RomIndex::SFlash_Write_Reg))
    };
    romdriver_func(flashCfg, regIndex, regValue, regLen)
}

#[inline(always)]
pub fn SFlash_Busy(flashCfg: *mut SPI_Flash_Cfg_Type) -> BL_Sts_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut SPI_Flash_Cfg_Type) -> BL_Sts_Type>(
            rom_lookup(RomIndex::SFlash_Busy),
        )
    };
    romdriver_func(flashCfg)
}

#[inline(always)]
pub fn SFlash_Write_Enable(flashCfg: *mut SPI_Flash_Cfg_Type) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut SPI_Flash_Cfg_Type) -> BL_Err_Type>(
            rom_lookup(RomIndex::SFlash_Write_Enable),
        )
    };
    romdriver_func(flashCfg)
}

#[inline(always)]
pub fn SFlash_Qspi_Enable(flashCfg: *mut SPI_Flash_Cfg_Type) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut SPI_Flash_Cfg_Type) -> BL_Err_Type>(
            rom_lookup(RomIndex::SFlash_Qspi_Enable),
        )
    };
    romdriver_func(flashCfg)
}

#[inline(always)]
pub fn SFlash_Volatile_Reg_Write_Enable(flashCfg: *mut SPI_Flash_Cfg_Type) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut SPI_Flash_Cfg_Type)>(rom_lookup(
            RomIndex::SFlash_Volatile_Reg_Write_Enable,
        ))
    };
    romdriver_func(flashCfg)
}

#[inline(always)]
pub fn SFlash_Chip_Erase(flashCfg: *mut SPI_Flash_Cfg_Type) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut SPI_Flash_Cfg_Type) -> BL_Err_Type>(
            rom_lookup(RomIndex::SFlash_Chip_Erase),
        )
    };
    romdriver_func(flashCfg)
}

#[inline(always)]
pub fn SFlash_Sector_Erase(flashCfg: *mut SPI_Flash_Cfg_Type, secNum: u32) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, u32) -> BL_Err_Type,
        >(rom_lookup(RomIndex::SFlash_Sector_Erase))
    };
    romdriver_func(flashCfg, secNum)
}

#[inline(always)]
pub fn SFlash_Blk32_Erase(flashCfg: *mut SPI_Flash_Cfg_Type, blkNum: u32) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, u32) -> BL_Err_Type,
        >(rom_lookup(RomIndex::SFlash_Blk32_Erase))
    };
    romdriver_func(flashCfg, blkNum)
}

#[inline(always)]
pub fn SFlash_Blk64_Erase(flashCfg: *mut SPI_Flash_Cfg_Type, blkNum: u32) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, u32) -> BL_Err_Type,
        >(rom_lookup(RomIndex::SFlash_Blk64_Erase))
    };
    romdriver_func(flashCfg, blkNum)
}

#[inline(always)]
pub fn SFlash_Erase(
    flashCfg: *mut SPI_Flash_Cfg_Type,
    startaddr: u32,
    endaddr: u32,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, u32, u32) -> BL_Err_Type,
        >(rom_lookup(RomIndex::SFlash_Erase))
    };
    romdriver_func(flashCfg, startaddr, endaddr)
}

#[inline(always)]
pub fn SFlash_Program(
    flashCfg: *mut SPI_Flash_Cfg_Type,
    ioMode: SF_Ctrl_IO_Type,
    addr: u32,
    data: *mut u8,
    len: u32,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(
                *mut SPI_Flash_Cfg_Type,
                SF_Ctrl_IO_Type,
                u32,
                *mut u8,
                u32,
            ) -> BL_Err_Type,
        >(rom_lookup(RomIndex::SFlash_Program))
    };
    romdriver_func(flashCfg, ioMode, addr, data, len)
}

#[inline(always)]
pub fn SFlash_GetUniqueId(data: *mut u8, idLen: u8) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut u8, u8)>(rom_lookup(
            RomIndex::SFlash_GetUniqueId,
        ))
    };
    romdriver_func(data, idLen)
}

#[inline(always)]
pub fn SFlash_GetJedecId(flashCfg: *mut SPI_Flash_Cfg_Type, data: *mut u8) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut SPI_Flash_Cfg_Type, *mut u8)>(
            rom_lookup(RomIndex::SFlash_GetJedecId),
        )
    };
    romdriver_func(flashCfg, data)
}

#[inline(always)]
pub fn SFlash_GetDeviceId(data: *mut u8) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut u8)>(rom_lookup(
            RomIndex::SFlash_GetDeviceId,
        ))
    };
    romdriver_func(data)
}

#[inline(always)]
pub fn SFlash_Powerdown() {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn()>(rom_lookup(
            RomIndex::SFlash_Powerdown,
        ))
    };
    romdriver_func()
}

#[inline(always)]
pub fn SFlash_Releae_Powerdown(flashCfg: *mut SPI_Flash_Cfg_Type) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut SPI_Flash_Cfg_Type)>(rom_lookup(
            RomIndex::SFlash_Releae_Powerdown,
        ))
    };
    romdriver_func(flashCfg)
}

#[inline(always)]
pub fn SFlash_SetBurstWrap(flashCfg: *mut SPI_Flash_Cfg_Type) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut SPI_Flash_Cfg_Type)>(rom_lookup(
            RomIndex::SFlash_SetBurstWrap,
        ))
    };
    romdriver_func(flashCfg)
}

#[inline(always)]
pub fn SFlash_DisableBurstWrap(flashCfg: *mut SPI_Flash_Cfg_Type) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut SPI_Flash_Cfg_Type)>(rom_lookup(
            RomIndex::SFlash_DisableBurstWrap,
        ))
    };
    romdriver_func(flashCfg)
}

#[inline(always)]
pub fn SFlash_Software_Reset(flashCfg: *mut SPI_Flash_Cfg_Type) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut SPI_Flash_Cfg_Type) -> BL_Err_Type>(
            rom_lookup(RomIndex::SFlash_Software_Reset),
        )
    };
    romdriver_func(flashCfg)
}

#[inline(always)]
pub fn SFlash_Reset_Continue_Read(flashCfg: *mut SPI_Flash_Cfg_Type) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut SPI_Flash_Cfg_Type)>(rom_lookup(
            RomIndex::SFlash_Reset_Continue_Read,
        ))
    };
    romdriver_func(flashCfg)
}

#[inline(always)]
pub fn SFlash_Set_IDbus_Cfg(
    flashCfg: *mut SPI_Flash_Cfg_Type,
    ioMode: SF_Ctrl_IO_Type,
    contRead: u8,
    addr: u32,
    len: u32,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(
                *mut SPI_Flash_Cfg_Type,
                SF_Ctrl_IO_Type,
                u8,
                u32,
                u32,
            ) -> BL_Err_Type,
        >(rom_lookup(RomIndex::SFlash_Set_IDbus_Cfg))
    };
    romdriver_func(flashCfg, ioMode, contRead, addr, len)
}

#[inline(always)]
pub fn SFlash_IDbus_Read_Enable(
    flashCfg: *mut SPI_Flash_Cfg_Type,
    ioMode: SF_Ctrl_IO_Type,
    contRead: u8,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, SF_Ctrl_IO_Type, u8) -> BL_Err_Type,
        >(rom_lookup(RomIndex::SFlash_IDbus_Read_Enable))
    };
    romdriver_func(flashCfg, ioMode, contRead)
}

#[inline(always)]
pub fn SFlash_Cache_Enable_Set(wayDisable: u8) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(u8) -> BL_Err_Type>(rom_lookup(
            RomIndex::SFlash_Cache_Enable_Set,
        ))
    };
    romdriver_func(wayDisable)
}

#[inline(always)]
pub fn SFlash_Cache_Flush() -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn() -> BL_Err_Type>(rom_lookup(
            RomIndex::SFlash_Cache_Flush,
        ))
    };
    romdriver_func()
}

#[inline(always)]
pub fn SFlash_Cache_Read_Enable(
    flashCfg: *mut SPI_Flash_Cfg_Type,
    ioMode: SF_Ctrl_IO_Type,
    contRead: u8,
    wayDisable: u8,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, SF_Ctrl_IO_Type, u8, u8) -> BL_Err_Type,
        >(rom_lookup(RomIndex::SFlash_Cache_Read_Enable))
    };
    romdriver_func(flashCfg, ioMode, contRead, wayDisable)
}

#[inline(always)]
pub fn SFlash_Cache_Hit_Count_Get(hitCountLow: *mut u32, hitCountHigh: *mut u32) {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn(*mut u32, *mut u32)>(rom_lookup(
            RomIndex::SFlash_Cache_Hit_Count_Get,
        ))
    };
    romdriver_func(hitCountLow, hitCountHigh)
}

#[inline(always)]
pub fn SFlash_Cache_Miss_Count_Get() -> u32 {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn() -> u32>(rom_lookup(
            RomIndex::SFlash_Cache_Miss_Count_Get,
        ))
    };
    romdriver_func()
}

#[inline(always)]
pub fn SFlash_Cache_Read_Disable() {
    let romdriver_func = unsafe {
        core::mem::transmute::<*const (), extern "C" fn()>(rom_lookup(
            RomIndex::SFlash_Cache_Read_Disable,
        ))
    };
    romdriver_func()
}

#[inline(always)]
pub fn SFlash_Read(
    flashCfg: *mut SPI_Flash_Cfg_Type,
    ioMode: SF_Ctrl_IO_Type,
    contRead: u8,
    addr: u32,
    data: *mut u8,
    len: u32,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(
                *mut SPI_Flash_Cfg_Type,
                SF_Ctrl_IO_Type,
                u8,
                u32,
                *mut u8,
                u32,
            ) -> BL_Err_Type,
        >(rom_lookup(RomIndex::SFlash_Read))
    };
    romdriver_func(flashCfg, ioMode, contRead, addr, data, len)
}

#[inline(always)]
pub fn SFlash_Read_Reg_With_Cmd(
    flashCfg: *mut SPI_Flash_Cfg_Type,
    readRegCmd: u8,
    regValue: *mut u8,
    regLen: u8,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, u8, *mut u8, u8) -> BL_Err_Type,
        >(rom_lookup(RomIndex::SFlash_Read_Reg_With_Cmd))
    };
    romdriver_func(flashCfg, readRegCmd, regValue, regLen)
}

#[inline(always)]
pub fn SFlash_Write_Reg_With_Cmd(
    flashCfg: *mut SPI_Flash_Cfg_Type,
    writeRegCmd: u8,
    regValue: *mut u8,
    regLen: u8,
) -> BL_Err_Type {
    let romdriver_func = unsafe {
        core::mem::transmute::<
            *const (),
            extern "C" fn(*mut SPI_Flash_Cfg_Type, u8, *mut u8, u8) -> BL_Err_Type,
        >(rom_lookup(RomIndex::SFlash_Write_Reg_With_Cmd))
    };
    romdriver_func(flashCfg, writeRegCmd, regValue, regLen)
}