use crate::{
    address::Address,
    io_reg_constant::{
        BG_OBJ_PALETTES_END, BG_OBJ_PALETTES_START, BOOT_ROM_START, COMMUNICATION_END,
        COMMUNICATION_START, CONTROLLER_START, DIV_TIMER_START, LCD_END, LCD_START, SOUND_END,
        SOUND_START, TIMER_CONTROL_START, TIMER_COUNTER_START, TIMER_MODULO_START, VRAM_BANK_START,
        VRAM_DMA_END, VRAM_DMA_START, WAVEFORM_RAM_END, WAVEFORM_RAM_START, WRAM_BANK_START,
    },
    Address as PseudoAddress, Area, Error, FileOperation, IORegArea,
};
use std::{cell::RefCell, rc::Rc};

pub struct IORegBus {
    pub controller: Rc<RefCell<dyn FileOperation<IORegArea>>>,
    pub communication: Rc<RefCell<dyn FileOperation<IORegArea>>>,
    pub div_timer: Rc<RefCell<dyn FileOperation<IORegArea>>>,
    pub sound: Rc<RefCell<dyn FileOperation<IORegArea>>>,
    pub waveform_ram: Rc<RefCell<dyn FileOperation<IORegArea>>>,
    pub lcd: Rc<RefCell<dyn FileOperation<IORegArea>>>,
    pub vram_bank: Rc<RefCell<dyn FileOperation<IORegArea>>>,
    pub boot_rom: Rc<RefCell<dyn FileOperation<IORegArea>>>,
    pub vram_dma: Rc<RefCell<dyn FileOperation<IORegArea>>>,
    pub bg_obj_palettes: Rc<RefCell<dyn FileOperation<IORegArea>>>,
    pub wram_bank: Rc<RefCell<dyn FileOperation<IORegArea>>>,
}

impl FileOperation<Area> for IORegBus {
    fn read(&self, address: Box<dyn PseudoAddress<Area>>) -> Result<u8, Error> {
        let addr: u16 = address.into();
        match addr {
            CONTROLLER_START => self.controller.borrow().read(Box::new(Address::from_offset(
                IORegArea::Controller,
                addr,
                COMMUNICATION_START,
            ))),
            COMMUNICATION_START..=COMMUNICATION_END => {
                self.communication
                    .borrow()
                    .read(Box::new(Address::from_offset(
                        IORegArea::Communication,
                        addr,
                        COMMUNICATION_START,
                    )))
            }
            DIV_TIMER_START..=DIV_TIMER_END => self.div_timer.borrow().read(Box::new(
                Address::from_offset(IORegArea::DivTimer, addr, DIV_TIMER_START),
            )),
            DIV_TIMER_START => self.div_timer.borrow().read(Box::new(Address::from_offset(
                IORegArea::DivTimer,
                addr,
                DIV_TIMER_START,
            ))),
            TIMER_COUNTER_START => self
                .tima
                .borrow()
                .read(Box::new(Address::byte_reg(IORegArea::TimerCounter, addr))),
            TIMER_MODULO_START => self
                .tma
                .borrow()
                .read(Box::new(Address::byte_reg(IORegArea::TimerModulo, addr))),
            TIMER_CONTROL_START => self
                .tac
                .borrow()
                .read(Box::new(Address::byte_reg(IORegArea::TimerControl, addr))),
            SOUND_START..=SOUND_END => self.sound.borrow().read(Box::new(Address::from_offset(
                IORegArea::Sound,
                addr,
                SOUND_START,
            ))),
            WAVEFORM_RAM_START..=WAVEFORM_RAM_END => {
                self.waveform_ram
                    .borrow()
                    .read(Box::new(Address::from_offset(
                        IORegArea::WaveformRam,
                        addr,
                        WAVEFORM_RAM_START,
                    )))
            }
            LCD_START..=LCD_END => self.lcd.borrow().read(Box::new(Address::from_offset(
                IORegArea::Lcd,
                addr,
                LCD_START,
            ))),
            VRAM_BANK_START => self.vram_bank.borrow().read(Box::new(Address::from_offset(
                IORegArea::VRamBank,
                addr,
                VRAM_BANK_START,
            ))),
            BOOT_ROM_START => self.boot_rom.borrow().read(Box::new(Address::from_offset(
                IORegArea::BootRom,
                addr,
                BOOT_ROM_START,
            ))),
            VRAM_DMA_START..=VRAM_DMA_END => self.vram_dma.borrow().read(Box::new(
                Address::from_offset(IORegArea::VramDma, addr, VRAM_DMA_START),
            )),
            BG_OBJ_PALETTES_START..=BG_OBJ_PALETTES_END => {
                self.bg_obj_palettes
                    .borrow()
                    .read(Box::new(Address::from_offset(
                        IORegArea::BgObjPalettes,
                        addr,
                        BG_OBJ_PALETTES_START,
                    )))
            }
            WRAM_BANK_START => self.wram_bank.borrow().read(Box::new(Address::from_offset(
                IORegArea::WRamBank,
                addr,
                WRAM_BANK_START,
            ))),
            _ => Err(Error::BusError(addr)),
        }
    }

    fn write(&mut self, v: u8, address: Box<dyn PseudoAddress<Area>>) -> Result<(), Error> {
        let addr: u16 = address.into();
        match addr {
            CONTROLLER_START => self.controller.borrow_mut().write(
                v,
                Box::new(Address::from_offset(
                    IORegArea::Controller,
                    addr,
                    CONTROLLER_START,
                )),
            ),
            COMMUNICATION_START..=COMMUNICATION_END => self.communication.borrow_mut().write(
                v,
                Box::new(Address::from_offset(
                    IORegArea::Communication,
                    addr,
                    COMMUNICATION_START,
                )),
            ),
            DIV_TIMER_START => self
                .div_timer
                .borrow_mut()
                .write(v, Box::new(Address::byte_reg(IORegArea::DivTimer, addr))),
            TIMER_COUNTER_START => self.tima.borrow_mut().write(
                v,
                Box::new(Address::byte_reg(IORegArea::TimerCounter, addr)),
            ),
            TIMER_MODULO_START => self
                .tma
                .borrow_mut()
                .write(v, Box::new(Address::byte_reg(IORegArea::TimerModulo, addr))),
            TIMER_CONTROL_START => self.tac.borrow_mut().write(
                v,
                Box::new(Address::byte_reg(IORegArea::TimerControl, addr)),
            ),
            SOUND_START..=SOUND_END => self.sound.borrow_mut().write(
                v,
                Box::new(Address::from_offset(IORegArea::Sound, addr, SOUND_START)),
            ),
            WAVEFORM_RAM_START..=WAVEFORM_RAM_END => self.waveform_ram.borrow_mut().write(
                v,
                Box::new(Address::from_offset(
                    IORegArea::WaveformRam,
                    addr,
                    WAVEFORM_RAM_START,
                )),
            ),
            LCD_START..=LCD_END => self.lcd.borrow_mut().write(
                v,
                Box::new(Address::from_offset(IORegArea::Lcd, addr, LCD_START)),
            ),
            VRAM_BANK_START => self.vram_bank.borrow_mut().write(
                v,
                Box::new(Address::from_offset(
                    IORegArea::VRamBank,
                    addr,
                    VRAM_BANK_START,
                )),
            ),
            BOOT_ROM_START => self.boot_rom.borrow_mut().write(
                v,
                Box::new(Address::from_offset(
                    IORegArea::BootRom,
                    addr,
                    BOOT_ROM_START,
                )),
            ),
            VRAM_DMA_START..=VRAM_DMA_END => self.vram_dma.borrow_mut().write(
                v,
                Box::new(Address::from_offset(
                    IORegArea::VramDma,
                    addr,
                    VRAM_BANK_START,
                )),
            ),
            BG_OBJ_PALETTES_START..=BG_OBJ_PALETTES_END => self.bg_obj_palettes.borrow_mut().write(
                v,
                Box::new(Address::from_offset(
                    IORegArea::BgObjPalettes,
                    addr,
                    BG_OBJ_PALETTES_START,
                )),
            ),
            WRAM_BANK_START => self.wram_bank.borrow_mut().write(
                v,
                Box::new(Address::from_offset(
                    IORegArea::WRamBank,
                    addr,
                    WRAM_BANK_START,
                )),
            ),
            _ => Err(Error::BusError(addr)),
        }
    }
}
