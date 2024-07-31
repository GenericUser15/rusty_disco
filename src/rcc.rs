use core::ptr::{read_volatile, write_volatile};

const RCCBASE_ADDR: u32 = 0x40021000;
const RCC_AHBNER_OFFSET: u32 = 0x14;

pub struct RCC {
    rcc_gpio_addr: *mut u32,
}

impl RCC {
    pub fn init_rcc() -> RCC {
        RCC {
            rcc_gpio_addr: (RCCBASE_ADDR + RCC_AHBNER_OFFSET) as *mut u32,
        }
    }

    pub fn init_rcc_for_gpio_bank(&self, gpio_bank_shit_amount: u8) {
        unsafe {
            write_volatile(
                self.rcc_gpio_addr,
                read_volatile(self.rcc_gpio_addr) | (1 << gpio_bank_shit_amount),
            );
        }
    }
}
