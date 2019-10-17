#![allow(unused_must_use)]
use crate::registers::{ControlRegister, Register, PowerMode, Modulation, FramingMode, Status};

use crate::nb::block;

enum SpiAction {
    Read(ControlRegister),
    Write(ControlRegister, u8)
}

#[derive(Debug)]
pub enum Ax5031Error {
    Any,
    AutoRangingTimeout,
    AutoRangingError
}

impl From<Ax5031Error> for core::fmt::Error {
    fn from(_err: Ax5031Error) -> Self {
        core::fmt::Error
    }
}

pub struct Ax5031DigitalPin<'a, SPI, PIN>
{
    ax5031: &'a Ax5031<SPI, PIN>
}

impl<'a, SPI, PIN> embedded_hal::digital::OutputPin for Ax5031DigitalPin<'a, SPI, PIN>
where SPI: embedded_hal::spi::FullDuplex<u8>,
      PIN: embedded_hal::digital::v2::OutputPin {

    fn set_low(&mut self) {
        //self.ax5031.set_sysclk();
    }

    fn set_high(&mut self) {
        //self.ax5031.set_sysclk();
    }
}

pub struct Ax5031<SPI, PIN> {
    spi: SPI,
    enable_line: PIN
}

impl<SPI, PIN> Ax5031<SPI, PIN>
where SPI: embedded_hal::spi::FullDuplex<u8>,
      PIN: embedded_hal::digital::v2::OutputPin {

    pub fn new(spi: SPI, enable_line: PIN) -> Self {
        Ax5031 {
            spi,
            enable_line
        }
    }

    fn create_frame(action: SpiAction) -> u16 {
        match action {
            SpiAction::Read(addr) =>
                0 << 15 | ((addr.register() as u16) & 0x7F) << 8,
            SpiAction::Write(addr, data) =>
                1 << 15 | ((addr.register() as u16) & 0x7F) << 8 | (data as u16) & 0xFF,
        }
    }

    //fn write_then_read(&mut self, frame: u16) -> Result<[u8; 2], Ax5031Error> {
    //}

    fn set_register(&mut self, reg: ControlRegister, val: u8) -> Result<[u8; 2], Ax5031Error> {
        let frame = Self::create_frame(SpiAction::Write(reg, val));
        self.send(frame)
    }

    fn get_register(&mut self, reg: ControlRegister) -> Result<u8, Ax5031Error> {
        self.begin_transmission();
        let ret = self.send(Self::create_frame(SpiAction::Read(reg))).map_err(|_e| Ax5031Error::Any)?;
//        let ret = Ok([
//            block!(self.spi.read()).map_err(|_| Ax5031Error::Any)?,
//            block!(self.spi.read()).map_err(|_| Ax5031Error::Any)?,
//        ]);
        self.end_transmission();
        Ok(ret[1])
    }

    fn begin_transmission(&mut self) {
        self.enable_line.set_low().map_err(|_| Ax5031Error::Any).unwrap();
    }

    fn end_transmission(&mut self) {
        self.enable_line.set_high().map_err(|_| Ax5031Error::Any).unwrap();
        //for _ in 0..10_000 {
        //}
    }

    fn send(&mut self, frame: u16) -> Result<[u8; 2], Ax5031Error> {
        let mut ret: [u8; 2] = [0; 2];

        self.begin_transmission();
        self.spi.send((frame >> 8) as u8).map_err(|_| Ax5031Error::Any)?;
        ret[0] = block!(self.spi.read()).map_err(|_| Ax5031Error::Any)?;
        self.spi.send((frame & 0xFF) as u8).map_err(|_| Ax5031Error::Any)?;
        ret[1] = block!(self.spi.read()).map_err(|_| Ax5031Error::Any)?;
        self.end_transmission();

        Ok(ret)
    }

    pub fn get_scratch(&mut self) -> Result<u8, Ax5031Error> {
        self.get_register(ControlRegister::SCRATCH)
    }

    pub fn set_scratch(&mut self, val: u8) -> Result<(), Ax5031Error> {
        self.set_register(ControlRegister::SCRATCH, val);
        Ok(())
    }

    pub fn set_power_mode(&mut self, power_mode: PowerMode) -> Result<(), Ax5031Error> {
        let bits = match power_mode {
            PowerMode::PowerDown => 0x0,
            PowerMode::VoltageRegulatorOn => 0x4,
            PowerMode::Standby => 0x5,
            PowerMode::SynthTx => 0xC,
            PowerMode::FullTx => 0xD
        };

        self.set_register(ControlRegister::PWRMODE, bits);
        Ok(())
    }

    pub fn set_pll_loop(&mut self, flt: u8, pllcpi: u8, band_sel: u8, freq_sel: u8) -> Result<(), ()> {
        let reg = (band_sel & 1) << 5
            | (freq_sel & 1)  << 7
            | (pllcpi & 0x3) << 2 
            | (flt & 0x3);

        self.set_register(ControlRegister::PLLLOOP, reg);
        Ok(())
    }

    pub fn set_frequency(&mut self, carrier_frequency: u32) -> Result<(), Ax5031Error> {

        let crystal_frequency = 16_000_000;
        let freq = f64::from((carrier_frequency as u64 * 2u64.pow(24) / crystal_frequency) as u32) + 0.5;

        // Poor mans `.ceil`:
        let freq = (freq + 1.0) as u32;

        let p3 = (freq >> 24 & 0xFF) as u8;
        let p2 = (freq >> 16 & 0xFF) as u8;
        let p1 = (freq >> 8 & 0xFF) as u8;
        let p0 = (freq >> 0 & 0xFF) as u8;

        self.set_register(ControlRegister::FREQ3, p3)?;
        self.set_register(ControlRegister::FREQ2, p2)?;
        self.set_register(ControlRegister::FREQ1, p1)?;
        self.set_register(ControlRegister::FREQ0, p0)?;

        Ok(())
    }

    pub fn get_frequency(&mut self) -> Result<u32, Ax5031Error> {
        let p3 = self.get_register(ControlRegister::FREQ3)? as u32;
        let p2 = self.get_register(ControlRegister::FREQ2)? as u32;
        let p1 = self.get_register(ControlRegister::FREQ1)? as u32;
        let p0 = self.get_register(ControlRegister::FREQ0)? as u32;

        Ok(p3 << 24 | p2 << 16 | p1 << 8 | p0)
    }

    pub fn set_transmit_power(&mut self) -> Result<(), ()> {
        let txrng = 0xF;
        self.set_register(ControlRegister::TXPWR, txrng);
        Ok(())
    }

    pub fn set_transmit_bitrate(&mut self, bitrate: u16) -> Result<(), ()> {
        let crystal_frequency: u32 = 16_000_000;
        let txrate = f64::from((bitrate as u64 * 2u64.pow(24) / crystal_frequency as u64) as u32) + 0.5;
        let txrate = (txrate + 1.0) as u32;

        let txrate = txrate & 0x001FFFFF;

        self.set_register(ControlRegister::TXRATEHI, (txrate >> 16) as u8);
        self.set_register(ControlRegister::TXRATEMID, (txrate >> 8) as u8);
        self.set_register(ControlRegister::TXRATELO, (txrate >> 16) as u8);

        Ok(())
    }

    pub fn set_modulation(&mut self, modulation_type: Modulation) -> Result<(), ()> {
        let modulation = match modulation_type {
            Modulation::ASK => 0x00,
            _ => unimplemented!("Only ASK at this point")
        };

        let frame = modulation & 0x3F;

        self.set_register(ControlRegister::MODULATION, frame);
        Ok(())
    }

    pub fn get_modulation(&mut self) -> Result<Modulation, Ax5031Error> {
        let modulation_type = self.get_register(ControlRegister::MODULATION)?;

        let modulation = match modulation_type {
            0x00 => Modulation::ASK,
            _ => unimplemented!("Only ASK at this point")
        };

        Ok(modulation)
    }

    pub fn set_framing_mode(&mut self, _framing_mode: FramingMode) -> Result<(), ()> {
        Err(())
    }

    pub fn autoranging(&mut self) -> Result<u16, Ax5031Error> {
        let start_pattern = 8;

        self.set_register(ControlRegister::PLLRANGING, start_pattern);

        for i in 0..3000 {
            let pllranging = self.get_register(ControlRegister::PLLRANGING)?;

            let rngstart = pllranging >> 4 & 1;
            let rngerr = pllranging >> 5 & 1;

            if rngstart == 1 {
                continue
            } 
            if rngerr == 1 {
                return Err(Ax5031Error::AutoRangingError);
            } else {
                return Ok(i)
            }
        }
        Err(Ax5031Error::AutoRangingTimeout)
    }

    pub fn sysclk_led<'a>(&'a mut self, on: bool) {
        let data = if on { 1 } else { 0 };
        self.set_register(ControlRegister::PINCFG1, data);
    }
//    pub fn sysclk_led<'a>(&'a mut self) -> Ax5031DigitalPin<'a, SPI, PIN> {
//        let frame = Self::create_frame(SpiAction::Write(ControlRegister::PINCFG1, data));
//        self.send(frame);
//        block!(self.spi.read());
//        block!(self.spi.read());
//        self.end_transmission();
//
//        Ax5031DigitalPin {
//           ax5031: &'a self
//        }
//    }

    pub fn get_pincfg1(&mut self) -> Result<u8, Ax5031Error> {
        self.get_register(ControlRegister::PINCFG1)
    }
    pub fn get_pincfg2(&mut self) -> Result<u8, Ax5031Error> {
        self.get_register(ControlRegister::PINCFG2)
    }

    pub fn transmit(&mut self, packet: u8) -> Result<Status, Ax5031Error> {
        self.set_register(ControlRegister::FIFODATA, packet)
            .map(|ret| ret[0])
            .map(Status::from_register)
    }
}
