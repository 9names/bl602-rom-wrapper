use crate::rom::SPI_Flash_Cfg_Type;

#[allow(dead_code)]
pub fn winbond_80_dv_cfg() -> SPI_Flash_Cfg_Type {
    SPI_Flash_Cfg_Type {
        ioMode: 4,
        cReadSupport: 1,
        clkDelay: 1,
        clkInvert: 1,
        resetEnCmd: 0x66,
        resetCmd: 0x99,
        resetCreadCmd: 0xFF,
        resetCreadCmdSize: 3,
        jedecIdCmd: 0x9F,
        jedecIdCmdDmyClk: 0,
        qpiJedecIdCmd: 0x9F,
        qpiJedecIdCmdDmyClk: 0,
        sectorSize: 4,
        mid: 0xEF,
        pageSize: 256,
        chipEraseCmd: 0xC7,
        sectorEraseCmd: 0x20,
        blk32EraseCmd: 0x52,
        blk64EraseCmd: 0xD8,
        writeEnableCmd: 0x06,
        pageProgramCmd: 0x02,
        qpageProgramCmd: 0x32,
        qppAddrMode: 0,
        fastReadCmd: 0x0B,
        frDmyClk: 1,
        qpiFastReadCmd: 0x0B,
        qpiFrDmyClk: 1,
        fastReadDoCmd: 0x3B,
        frDoDmyClk: 1,
        fastReadDioCmd: 0xBB,
        frDioDmyClk: 0,
        fastReadQoCmd: 0x6B,
        frQoDmyClk: 1,
        fastReadQioCmd: 0xEB,
        frQioDmyClk: 1,
        qpiFastReadQioCmd: 0xEB,
        qpiFrQioDmyClk: 2,
        qpiPageProgramCmd: 0x02,
        writeVregEnableCmd: 0x50,
        wrEnableIndex: 0,
        qeIndex: 1,
        busyIndex: 0,
        wrEnableBit: 1,
        qeBit: 1,
        busyBit: 0,
        wrEnableWriteRegLen: 2,
        wrEnableReadRegLen: 1,
        qeWriteRegLen: 1,
        qeReadRegLen: 0xAB,
        releasePowerDown: 1,
        busyReadRegLen: 1,
        readRegCmd: [0x05, 0x35, 0x00, 0x00],
        writeRegCmd: [0x01, 0x31, 0x00, 0x00],
        enterQpi: 0x38,
        exitQpi: 0xFF,
        cReadMode: 0x20,
        cRExit: 0xFF,
        burstWrapCmd: 0x77,
        burstWrapCmdDmyClk: 0x03,
        burstWrapDataMode: 2,
        burstWrapData: 0x40,
        deBurstWrapCmd: 0x77,
        deBurstWrapCmdDmyClk: 0x03,
        deBurstWrapDataMode: 2,
        deBurstWrapData: 0xF0,
        timeEsector: 300,
        timeE32k: 1200,
        timeE64k: 1200,
        timePagePgm: 5,
        timeCe: 20000,
        pdDelay: 3,
        qeData: 0,
    }
}

#[allow(dead_code)]
pub fn winbond_80_ew_cfg() -> SPI_Flash_Cfg_Type {
    SPI_Flash_Cfg_Type {
        ioMode: 4, // SF_CTRL_QIO_MODE
        cReadSupport: 1,
        clkDelay: 1,
        //clkInvert: 1,
        clkInvert: 0x3f,
        resetEnCmd: 0x66,
        resetCmd: 0x99,
        resetCreadCmd: 0xFF,
        resetCreadCmdSize: 3,
        jedecIdCmd: 0x9F,
        jedecIdCmdDmyClk: 0,
        qpiJedecIdCmd: 0x9F,
        qpiJedecIdCmdDmyClk: 0,
        sectorSize: 4,
        mid: 0xEF,
        pageSize: 256,
        chipEraseCmd: 0xC7,
        sectorEraseCmd: 0x20,
        blk32EraseCmd: 0x52,
        blk64EraseCmd: 0xD8,
        writeEnableCmd: 0x06,
        pageProgramCmd: 0x02,
        qpageProgramCmd: 0x32,
        qppAddrMode: 0, // SF_CTRL_ADDR_1_LINE
        fastReadCmd: 0x0B,
        frDmyClk: 1, // 8/8
        qpiFastReadCmd: 0x0B,
        qpiFrDmyClk: 1, // 8/8
        fastReadDoCmd: 0x3B,
        frDoDmyClk: 1, // 8/8
        fastReadDioCmd: 0xBB,
        frDioDmyClk: 0,
        fastReadQoCmd: 0x6B,
        frQoDmyClk: 1, // 8/8
        fastReadQioCmd: 0xEB,
        frQioDmyClk: 2, // 16/8
        qpiFastReadQioCmd: 0xEB,
        qpiFrQioDmyClk: 2, // 16/8
        qpiPageProgramCmd: 0x02,
        writeVregEnableCmd: 0x50,
        wrEnableIndex: 0,
        qeIndex: 1,
        busyIndex: 0,
        wrEnableBit: 1,
        qeBit: 1,
        busyBit: 0,
        wrEnableWriteRegLen: 0,
        wrEnableReadRegLen: 1,
        qeWriteRegLen: 1,
        qeReadRegLen: 0x1,
        releasePowerDown: 0xab,
        busyReadRegLen: 1,
        readRegCmd: [0x05, 0x35, 0x00, 0x00],
        writeRegCmd: [0x01, 0x31, 0x00, 0x00],
        enterQpi: 0x38,
        exitQpi: 0xFF,
        cReadMode: 0x20,
        cRExit: 0xFF,
        burstWrapCmd: 0x77,
        burstWrapCmdDmyClk: 0x03,
        burstWrapDataMode: 2, // SF_CTRL_DATA_4_LINES
        burstWrapData: 0x40,
        deBurstWrapCmd: 0x77,
        deBurstWrapCmdDmyClk: 0x03,
        deBurstWrapDataMode: 2, // SF_CTRL_DATA_4_LINES
        deBurstWrapData: 0xF0,
        timeEsector: 400,
        timeE32k: 1600,
        timeE64k: 2000,
        timePagePgm: 5,
        timeCe: 20000, // 20*1000
        pdDelay: 3,
        qeData: 0,
    }
}
