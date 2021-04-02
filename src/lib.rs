#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod rom_lookup;
use rom_lookup::{rom_lookup, RomIndex};

pub mod sflash {
    use super::*;
    pub mod flashconfig;
    pub mod sf_cfg;
    pub mod sf_ctrl;
    pub mod xip_sflash;
    pub const BL_Err_Type_SUCCESS: BL_Err_Type = 0;
    pub const BL_Err_Type_ERROR: BL_Err_Type = 1;
    pub const BL_Err_Type_TIMEOUT: BL_Err_Type = 2;

    pub const RESET: BL_Sts_Type = 0;
    pub const SET: BL_Sts_Type = 1;
    pub type BL_Sts_Type = u32;

    #[doc = " @brief Error type definition"]
    pub type BL_Err_Type = u32;
    pub type sf_ctrl_aes_region_reg_t = u32;
    #[doc = "< Embedded flash select"]
    pub const SF_Ctrl_Pad_Sel_SF_CTRL_EMBEDDED_SEL: SF_Ctrl_Pad_Sel = 0;
    #[doc = "< External flash select gpio 17-22"]
    pub const SF_Ctrl_Pad_Sel_SF_CTRL_EXTERNAL_17TO22_SEL: SF_Ctrl_Pad_Sel = 1;
    #[doc = "< External flash select gpio 0-2 and 20-22"]
    pub const SF_Ctrl_Pad_Sel_SF_CTRL_EXTERNAL_0TO2_20TO22_SEL: SF_Ctrl_Pad_Sel = 2;
    #[doc = "  @brief Serial flash pad select type definition"]
    pub type SF_Ctrl_Pad_Sel = u32;
    #[doc = "< System AHB bus control serial flash controller"]
    pub const SF_Ctrl_Owner_Type_SF_CTRL_OWNER_SAHB: SF_Ctrl_Owner_Type = 0;
    #[doc = "< I-Code AHB bus control serial flash controller"]
    pub const SF_Ctrl_Owner_Type_SF_CTRL_OWNER_IAHB: SF_Ctrl_Owner_Type = 1;
    #[doc = "  @brief Serial flash controller owner type definition"]
    pub type SF_Ctrl_Owner_Type = u32;
    #[doc = "< Serial flash controller select default sahb clock"]
    pub const SF_Ctrl_Sahb_Type_SF_CTRL_SAHB_CLOCK: SF_Ctrl_Sahb_Type = 0;
    #[doc = "< Serial flash controller select flash clock"]
    pub const SF_Ctrl_Sahb_Type_SF_CTRL_FLASH_CLOCK: SF_Ctrl_Sahb_Type = 1;
    #[doc = "  @brief Serial flash controller select clock type definition"]
    pub type SF_Ctrl_Sahb_Type = u32;
    #[doc = "< Serial flash controller high speed mode clk_ahb>clk_sf"]
    pub const SF_Ctrl_Ahb2sif_Type_HIGH_SPEED_MODE_CLOCK: SF_Ctrl_Ahb2sif_Type = 0;
    #[doc = "< Serial flash controller remove clock constrain"]
    pub const SF_Ctrl_Ahb2sif_Type_REMOVE_CLOCK_CONSTRAIN: SF_Ctrl_Ahb2sif_Type = 1;
    #[doc = "  @brief Serial flash controller owner type definition"]
    pub type SF_Ctrl_Ahb2sif_Type = u32;
    #[doc = "< Serail flash read command flag"]
    pub const SF_Ctrl_RW_Type_SF_CTRL_READ: SF_Ctrl_RW_Type = 0;
    #[doc = "< Serail flash write command flag"]
    pub const SF_Ctrl_RW_Type_SF_CTRL_WRITE: SF_Ctrl_RW_Type = 1;
    #[doc = "  @brief Read and write type definition"]
    pub type SF_Ctrl_RW_Type = u32;
    #[doc = "< Normal IO mode define"]
    pub const SF_Ctrl_IO_Type_SF_CTRL_NIO_MODE: SF_Ctrl_IO_Type = 0;
    #[doc = "< Dual Output mode define"]
    pub const SF_Ctrl_IO_Type_SF_CTRL_DO_MODE: SF_Ctrl_IO_Type = 1;
    #[doc = "< Quad Output mode define"]
    pub const SF_Ctrl_IO_Type_SF_CTRL_QO_MODE: SF_Ctrl_IO_Type = 2;
    #[doc = "< Dual IO mode define"]
    pub const SF_Ctrl_IO_Type_SF_CTRL_DIO_MODE: SF_Ctrl_IO_Type = 3;
    #[doc = "< Quad IO mode define"]
    pub const SF_Ctrl_IO_Type_SF_CTRL_QIO_MODE: SF_Ctrl_IO_Type = 4;
    #[doc = "  @brief Serail flash interface IO type definition"]
    pub type SF_Ctrl_IO_Type = u32;
    #[doc = "< SPI mode define"]
    pub const SF_Ctrl_Mode_Type_SF_CTRL_SPI_MODE: SF_Ctrl_Mode_Type = 0;
    #[doc = "< QPI mode define"]
    pub const SF_Ctrl_Mode_Type_SF_CTRL_QPI_MODE: SF_Ctrl_Mode_Type = 1;
    #[doc = "  @brief Serail flash controller interface mode type definition"]
    pub type SF_Ctrl_Mode_Type = u32;
    #[doc = "< Command in one line mode"]
    pub const SF_Ctrl_Cmd_Mode_Type_SF_CTRL_CMD_1_LINE: SF_Ctrl_Cmd_Mode_Type = 0;
    #[doc = "< Command in four lines mode"]
    pub const SF_Ctrl_Cmd_Mode_Type_SF_CTRL_CMD_4_LINES: SF_Ctrl_Cmd_Mode_Type = 1;
    #[doc = "  @brief Serail flash controller command mode type definition"]
    pub type SF_Ctrl_Cmd_Mode_Type = u32;
    #[doc = "< Address in one line mode"]
    pub const SF_Ctrl_Addr_Mode_Type_SF_CTRL_ADDR_1_LINE: SF_Ctrl_Addr_Mode_Type = 0;
    #[doc = "< Address in two lines mode"]
    pub const SF_Ctrl_Addr_Mode_Type_SF_CTRL_ADDR_2_LINES: SF_Ctrl_Addr_Mode_Type = 1;
    #[doc = "< Address in four lines mode"]
    pub const SF_Ctrl_Addr_Mode_Type_SF_CTRL_ADDR_4_LINES: SF_Ctrl_Addr_Mode_Type = 2;
    #[doc = "  @brief Serail flash controller address mode type definition"]
    pub type SF_Ctrl_Addr_Mode_Type = u32;
    #[doc = "< Dummy in one line mode"]
    pub const SF_Ctrl_Dmy_Mode_Type_SF_CTRL_DUMMY_1_LINE: SF_Ctrl_Dmy_Mode_Type = 0;
    #[doc = "< Dummy in two lines mode"]
    pub const SF_Ctrl_Dmy_Mode_Type_SF_CTRL_DUMMY_2_LINES: SF_Ctrl_Dmy_Mode_Type = 1;
    #[doc = "< Dummy in four lines mode"]
    pub const SF_Ctrl_Dmy_Mode_Type_SF_CTRL_DUMMY_4_LINES: SF_Ctrl_Dmy_Mode_Type = 2;
    #[doc = "  @brief Serail flash controller dummy mode type definition"]
    pub type SF_Ctrl_Dmy_Mode_Type = u32;
    #[doc = "< Data in one line mode"]
    pub const SF_Ctrl_Data_Mode_Type_SF_CTRL_DATA_1_LINE: SF_Ctrl_Data_Mode_Type = 0;
    #[doc = "< Data in two lines mode"]
    pub const SF_Ctrl_Data_Mode_Type_SF_CTRL_DATA_2_LINES: SF_Ctrl_Data_Mode_Type = 1;
    #[doc = "< Data in four lines mode"]
    pub const SF_Ctrl_Data_Mode_Type_SF_CTRL_DATA_4_LINES: SF_Ctrl_Data_Mode_Type = 2;
    #[doc = "  @brief Serail flash controller data mode type definition"]
    pub type SF_Ctrl_Data_Mode_Type = u32;
    #[doc = "< Serail flash AES key 128 bits length"]
    pub const SF_Ctrl_AES_Key_Type_SF_CTRL_AES_128BITS: SF_Ctrl_AES_Key_Type = 0;
    #[doc = "< Serail flash AES key 256 bits length"]
    pub const SF_Ctrl_AES_Key_Type_SF_CTRL_AES_256BITS: SF_Ctrl_AES_Key_Type = 1;
    #[doc = "< Serail flash AES key 192 bits length"]
    pub const SF_Ctrl_AES_Key_Type_SF_CTRL_AES_192BITS: SF_Ctrl_AES_Key_Type = 2;
    #[doc = "< Serail flash AES key 128 bits length double key"]
    pub const SF_Ctrl_AES_Key_Type_SF_CTRL_AES_128BITS_DOUBLE_KEY: SF_Ctrl_AES_Key_Type = 3;
    #[doc = "  @brief Serail flash controller AES type definition"]
    pub type SF_Ctrl_AES_Key_Type = u32;

    #[doc = "  @brief Serial flash configuration structure type definition"]
    #[repr(C, packed)]
    #[derive(Debug, Copy, Clone, Default)]
    pub struct SPI_Flash_Cfg_Type {
        #[doc = "< Serail flash interface mode,bit0-3:IF mode,bit4:unwrap"]
        pub ioMode: u8,
        #[doc = "< Support continuous read mode,bit0:continuous read mode support,bit1:read mode cfg"]
        pub cReadSupport: u8,
        #[doc = "< SPI clock delay,bit0-3:delay,bit4-6:pad delay"]
        pub clkDelay: u8,
        #[doc = "< SPI clock phase invert,bit0:clck invert,bit1:rx invert,bit2-4:pad delay,bit5-7:pad delay"]
        pub clkInvert: u8,
        #[doc = "< Flash enable reset command"]
        pub resetEnCmd: u8,
        #[doc = "< Flash reset command"]
        pub resetCmd: u8,
        #[doc = "< Flash reset continuous read command"]
        pub resetCreadCmd: u8,
        #[doc = "< Flash reset continuous read command size"]
        pub resetCreadCmdSize: u8,
        #[doc = "< JEDEC ID command"]
        pub jedecIdCmd: u8,
        #[doc = "< JEDEC ID command dummy clock"]
        pub jedecIdCmdDmyClk: u8,
        #[doc = "< QPI JEDEC ID comamnd"]
        pub qpiJedecIdCmd: u8,
        #[doc = "< QPI JEDEC ID command dummy clock"]
        pub qpiJedecIdCmdDmyClk: u8,
        #[doc = "< *1024bytes"]
        pub sectorSize: u8,
        #[doc = "< Manufacturer ID"]
        pub mid: u8,
        #[doc = "< Page size"]
        pub pageSize: u16,
        #[doc = "< Chip erase cmd"]
        pub chipEraseCmd: u8,
        #[doc = "< Sector erase command"]
        pub sectorEraseCmd: u8,
        #[doc = "< Block 32K erase command,some Micron not support"]
        pub blk32EraseCmd: u8,
        #[doc = "< Block 64K erase command"]
        pub blk64EraseCmd: u8,
        #[doc = "< Need before every erase or program"]
        pub writeEnableCmd: u8,
        #[doc = "< Page program cmd"]
        pub pageProgramCmd: u8,
        #[doc = "< QIO page program cmd"]
        pub qpageProgramCmd: u8,
        #[doc = "< QIO page program address mode"]
        pub qppAddrMode: u8,
        #[doc = "< Fast read command"]
        pub fastReadCmd: u8,
        #[doc = "< Fast read command dummy clock"]
        pub frDmyClk: u8,
        #[doc = "< QPI fast read command"]
        pub qpiFastReadCmd: u8,
        #[doc = "< QPI fast read command dummy clock"]
        pub qpiFrDmyClk: u8,
        #[doc = "< Fast read dual output command"]
        pub fastReadDoCmd: u8,
        #[doc = "< Fast read dual output command dummy clock"]
        pub frDoDmyClk: u8,
        #[doc = "< Fast read dual io comamnd"]
        pub fastReadDioCmd: u8,
        #[doc = "< Fast read dual io command dummy clock"]
        pub frDioDmyClk: u8,
        #[doc = "< Fast read quad output comamnd"]
        pub fastReadQoCmd: u8,
        #[doc = "< Fast read quad output comamnd dummy clock"]
        pub frQoDmyClk: u8,
        #[doc = "< Fast read quad io comamnd"]
        pub fastReadQioCmd: u8,
        #[doc = "< Fast read quad io comamnd dummy clock"]
        pub frQioDmyClk: u8,
        #[doc = "< QPI fast read quad io comamnd"]
        pub qpiFastReadQioCmd: u8,
        #[doc = "< QPI fast read QIO dummy clock"]
        pub qpiFrQioDmyClk: u8,
        #[doc = "< QPI program command"]
        pub qpiPageProgramCmd: u8,
        #[doc = "< Enable write reg"]
        pub writeVregEnableCmd: u8,
        #[doc = "< Write enable register index"]
        pub wrEnableIndex: u8,
        #[doc = "< Quad mode enable register index"]
        pub qeIndex: u8,
        #[doc = "< Busy status register index"]
        pub busyIndex: u8,
        #[doc = "< Write enable bit pos"]
        pub wrEnableBit: u8,
        #[doc = "< Quad enable bit pos"]
        pub qeBit: u8,
        #[doc = "< Busy status bit pos"]
        pub busyBit: u8,
        #[doc = "< Register length of write enable"]
        pub wrEnableWriteRegLen: u8,
        #[doc = "< Register length of write enable status"]
        pub wrEnableReadRegLen: u8,
        #[doc = "< Register length of contain quad enable"]
        pub qeWriteRegLen: u8,
        #[doc = "< Register length of contain quad enable status"]
        pub qeReadRegLen: u8,
        #[doc = "< Release power down command"]
        pub releasePowerDown: u8,
        #[doc = "< Register length of contain busy status"]
        pub busyReadRegLen: u8,
        #[doc = "< Read register command buffer"]
        pub readRegCmd: [u8; 4usize],
        #[doc = "< Write register command buffer"]
        pub writeRegCmd: [u8; 4usize],
        #[doc = "< Enter qpi command"]
        pub enterQpi: u8,
        #[doc = "< Exit qpi command"]
        pub exitQpi: u8,
        #[doc = "< Config data for continuous read mode"]
        pub cReadMode: u8,
        #[doc = "< Config data for exit continuous read mode"]
        pub cRExit: u8,
        #[doc = "< Enable burst wrap command"]
        pub burstWrapCmd: u8,
        #[doc = "< Enable burst wrap command dummy clock"]
        pub burstWrapCmdDmyClk: u8,
        #[doc = "< Data and address mode for this command"]
        pub burstWrapDataMode: u8,
        #[doc = "< Data to enable burst wrap"]
        pub burstWrapData: u8,
        #[doc = "< Disable burst wrap command"]
        pub deBurstWrapCmd: u8,
        #[doc = "< Disable burst wrap command dummy clock"]
        pub deBurstWrapCmdDmyClk: u8,
        #[doc = "< Data and address mode for this command"]
        pub deBurstWrapDataMode: u8,
        #[doc = "< Data to disable burst wrap"]
        pub deBurstWrapData: u8,
        #[doc = "< 4K erase time"]
        pub timeEsector: u16,
        #[doc = "< 32K erase time"]
        pub timeE32k: u16,
        #[doc = "< 64K erase time"]
        pub timeE64k: u16,
        #[doc = "< Page program time"]
        pub timePagePgm: u16,
        #[doc = "< Chip erase time in ms"]
        pub timeCe: u16,
        #[doc = "< Release power down command delay time for wake up"]
        pub pdDelay: u8,
        #[doc = "< QE set data"]
        pub qeData: u8,
    }

    #[doc = "  @brief Serail flash controller configuration structure type definition"]
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct SF_Ctrl_Cfg_Type {
        #[doc = "< Sflash interface bus owner"]
        pub owner: SF_Ctrl_Owner_Type,
        #[doc = "< Sflash clock sahb sram select"]
        pub sahbClock: SF_Ctrl_Sahb_Type,
        #[doc = "< Sflash ahb2sif mode"]
        pub ahb2sifMode: SF_Ctrl_Ahb2sif_Type,
        #[doc = "< Clock count for read due to pad delay"]
        pub clkDelay: u8,
        #[doc = "< Clock invert"]
        pub clkInvert: u8,
        #[doc = "< RX clock invert"]
        pub rxClkInvert: u8,
        #[doc = "< Data out delay"]
        pub doDelay: u8,
        #[doc = "< Data in delay"]
        pub diDelay: u8,
        #[doc = "< Output enable delay"]
        pub oeDelay: u8,
    }

    // Rust wrappers for C functions
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut buffer: [u8; 20] = [0; 20];
        sflash::SFlash_GetDeviceId(buffer.as_mut_ptr());
    }
}
