
pub trait Register {
    fn register(&self) -> u8;
    fn reset(&self) -> u8;
}

#[derive(Debug)]
pub enum ControlRegister {

    ///[Address: 0] Silicon Revision
    REVISION,

    ///[Address: 1] Scratch Register
    SCRATCH,

    ///[Address: 2] Power Mode
    PWRMODE,

    ///[Address: 3] FIFO, Part 1
    XTALOSC,

    ///[Address: 4] FIFO Control
    FIFOCTRL,

    ///[Address: 5] FIFO Data
    FIFODATA,

    ///[Address: 6] IRQ Mask
    IRQMASK,

    ///[Address: 7] IRQ Request
    IRQREQUEST,

    ///[Address: C] Pin Configuration 1
    PINCFG1,

    ///[Address: D] Pin Configuration 2
    PINCFG2,

    ///[Address: E] Pin Configuration 3
    PINCFG3,

    ///[Address: F] IRQ Inversion
    IRQINVERSION,

    ///[Address: 10] Modulation
    MODULATION,

    ///[Address: 11] Encoder/Decoder Settings
    ENCODING,

    ///[Address: 12] Framing settings
    FRAMING,

    ///[Address: 14] CRC Initialization Data or Preamble
    CRCINIT3,

    ///[Address: 15] CRC Initialization Data or Preamble
    CRCINIT2,

    ///[Address: 16] CRC Initialization Data or Preamble
    CRCINIT1,

    ///[Address: 17] CRC Initialization Data or Preamble
    CRCINIT0,

    ///[Address: 1B] Voltage Regulator Status
    VREG,

    ///[Address: 1C] 2nd Synthesizer Frequency
    FREQB3,

    ///[Address: 1D] 2nd Synthesizer Frequency
    FREQB2,

    ///[Address: 1E] 2nd Synthesizer Frequency
    FREQB1,

    ///[Address: 1F] 2nd Synthesizer Frequency
    FREQB0,

    ///[Address: 20] Synthesizer Frequency
    FREQ3,

    ///[Address: 21] Synthesizer Frequency
    FREQ2,

    ///[Address: 22] Synthesizer Frequency
    FREQ1,

    ///[Address: 23] Synthesizer Frequency
    FREQ0,

    ///[Address: 25] FSK Frequency Deviation
    FSKDEV2,

    ///[Address: 26] FSK Frequency Deviation
    FSKDEV1,

    ///[Address: 27] FSK Frequency Deviation
    FSKDEV0,

    ///[Address: 2C] Synthesizer Loop Filter Settings
    PLLLOOP,

    ///[Address: 2D] Synthesizer VCO Auto-Ranging
    PLLRANGING,

    ///[Address: 30] Transmit Power
    TXPWR,

    ///[Address: 31] Transmitter Bitrate
    TXRATEHI,

    ///[Address: 32] Transmitter Bitrate
    TXRATEMID,

    ///[Address: 33] Transmitter Bitrate
    TXRATELO,

    ///[Address: 34] Misc RF Flags
    MODMISC,

    ///[Address: 35] FIFO Fill State
    FIFOCOUNT,

    ///[Address: 36] FIFO Threshold
    FIFOTHRESH,

    ///[Address: 37] Additional FIFO Control
    FIFOCONTROL,

    ///[Address: 4F] Crystal oscillator tuning capacitance
    XTALCAP,

    ///[Address: 50] 4-FSK Control
    FOURFSK,

}


impl Register for ControlRegister {

    fn register(&self) -> u8 {
        match self {
            ControlRegister::REVISION => 0x0,
            ControlRegister::SCRATCH => 0x1,
            ControlRegister::PWRMODE => 0x2,
            ControlRegister::XTALOSC => 0x3,
            ControlRegister::FIFOCTRL => 0x4,
            ControlRegister::FIFODATA => 0x5,
            ControlRegister::IRQMASK => 0x6,
            ControlRegister::IRQREQUEST => 0x7,
            ControlRegister::PINCFG1 => 0x0c,
            ControlRegister::PINCFG2 => 0x0d,
            ControlRegister::PINCFG3 => 0xe,
            ControlRegister::IRQINVERSION => 0xf,
            ControlRegister::MODULATION => 0x10,
            ControlRegister::ENCODING => 0x11,
            ControlRegister::FRAMING => 0x12,
            ControlRegister::CRCINIT3 => 0x14,
            ControlRegister::CRCINIT2 => 0x15,
            ControlRegister::CRCINIT1 => 0x16,
            ControlRegister::CRCINIT0 => 0x17,
            ControlRegister::VREG => 0x1b,
            ControlRegister::FREQB3 => 0x1c,
            ControlRegister::FREQB2 => 0x1d,
            ControlRegister::FREQB1 => 0x1e,
            ControlRegister::FREQB0 => 0x1f,
            ControlRegister::FREQ3 => 0x20,
            ControlRegister::FREQ2 => 0x21,
            ControlRegister::FREQ1 => 0x22,
            ControlRegister::FREQ0 => 0x23,
            ControlRegister::FSKDEV2 => 0x25,
            ControlRegister::FSKDEV1 => 0x26,
            ControlRegister::FSKDEV0 => 0x27,
            ControlRegister::PLLLOOP => 0x2c,
            ControlRegister::PLLRANGING => 0x2d,
            ControlRegister::TXPWR => 0x30,
            ControlRegister::TXRATEHI => 0x31,
            ControlRegister::TXRATEMID => 0x32,
            ControlRegister::TXRATELO => 0x33,
            ControlRegister::MODMISC => 0x34,
            ControlRegister::FIFOCOUNT => 0x35,
            ControlRegister::FIFOTHRESH => 0x36,
            ControlRegister::FIFOCONTROL => 0x37,
            ControlRegister::XTALCAP => 0x4f,
            ControlRegister::FOURFSK => 0x50,
            
        }
    }

    fn reset(&self) -> u8 {
        unimplemented!()
    }
}

pub enum PowerMode {
    PowerDown,
    VoltageRegulatorOn,
    Standby,
    SynthTx,
    FullTx,
}

#[derive(Debug)]
pub enum Modulation {
    ASK,
    FSK,
    MSK,
    PSK,
}

pub enum FramingMode {
    Raw
    // TODO: Add rest of framing modes
}

fn get_bit(byte: u8, n_bit: u8) -> bool {
    (byte >> n_bit & 1) == 1
}

#[derive(Debug)]
pub struct Status {
    pub pll_lock: bool,
    pub fifo_over: bool,
    pub fifo_under: bool,
    pub fifo_full: bool,
    pub fifo_empty: bool,
    pub fifo_status: u8,
}


impl Status {
    pub fn from_register(status_register: u8) -> Self {
        Status {
            pll_lock: get_bit(status_register, 1),
            fifo_over: get_bit(status_register, 1),
            fifo_under: get_bit(status_register, 1),
            fifo_full: get_bit(status_register, 1),
            fifo_empty: get_bit(status_register, 1),
            fifo_status: status_register >> 6 & 0x3
        }
    }
}
