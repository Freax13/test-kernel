//! "CMOS" is a tiny bit of very low power static memory that lives on the same chip as the Real-Time Clock (RTC)
//! Implementation mostly copied from https://github.com/deadblackclover/CMOS/blob/main/src/lib.rs
//! CMOS port and register config see: https://wiki.osdev.org/CMOS

use x86_64::instructions::port::Port;

/// Selecting a CMOS register port
const CMOS_ADDRESS: u16 = 0x70;

/// Data receiving port
const CMOS_DATA: u16 = 0x71;

/// Struct for storage time
pub struct Time {
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
    pub day: u8,
    pub month: u8,
    pub year: u16,
    pub century: u8,
}

/// Struct for storage ports, current year and century register
struct ReadRTC {
    cmos_address: Port<u8>,
    cmos_data: Port<u8>,
    century_register: u8,
}

impl ReadRTC {
    /// Creates a new `ReadRTC`.
    const fn new(century_register: u8) -> ReadRTC {
        ReadRTC {
            cmos_address: Port::new(CMOS_ADDRESS),
            cmos_data: Port::new(CMOS_DATA),
            century_register,
        }
    }

    /// Lets you know if a time update is in progress
    fn get_update_in_progress_flag(&mut self) -> u8 {
        unsafe {
            self.cmos_address.write(0x0A);
            self.cmos_data.read() & 0x80
        }
    }

    /// Retrieves a value from a time register
    fn get_rtc_register(&mut self, reg: u8) -> u8 {
        unsafe {
            self.cmos_address.write(reg);
            self.cmos_data.read()
        }
    }

    /// Updating our time
    fn update_time(&mut self) -> Time {
        // Make sure an update isn't in progress
        while self.get_update_in_progress_flag() != 0 {}
        Time {
            second: self.get_rtc_register(0x00),
            minute: self.get_rtc_register(0x02),
            hour: self.get_rtc_register(0x04),
            day: self.get_rtc_register(0x07),
            month: self.get_rtc_register(0x08),
            year: self.get_rtc_register(0x09) as u16,
            century: if self.century_register != 0 {
                self.get_rtc_register(self.century_register)
            } else {
                0
            },
        }
    }

    /// Gets the time without regard to the time zone
    fn read(&mut self) -> Time {
        let mut last_time: Time;
        let mut time: Time = self.update_time();

        loop {
            last_time = time;
            time = self.update_time();

            if (last_time.second == time.second)
                && (last_time.minute == time.minute)
                && (last_time.hour == time.hour)
                && (last_time.day == time.day)
                && (last_time.month == time.month)
                && (last_time.year == time.year)
                && (last_time.century == time.century)
            {
                break;
            }
        }

        let register_b = self.get_rtc_register(0x0B);

        if !(register_b & 0x04 != 0) {
            time.second = (time.second & 0x0F) + ((time.second / 16) * 10);
            time.minute = (time.minute & 0x0F) + ((time.minute / 16) * 10);
            time.hour =
                ((time.hour & 0x0F) + (((time.hour & 0x70) / 16) * 10)) | (time.hour & 0x80);
            time.day = (time.day & 0x0F) + ((time.day / 16) * 10);
            time.month = (time.month & 0x0F) + ((time.month / 16) * 10);
            time.year = (time.year & 0x0F) + ((time.year / 16) * 10);
            if self.century_register != 0 {
                time.century = (time.century & 0x0F) + ((time.century / 16) * 10);
            }
        }

        // Convert 12 hour clock to 24 hour clock

        if !(register_b & 0x02 != 0) && (time.hour & 0x80 != 0) {
            time.hour = ((time.hour & 0x7F) + 12) % 24;
        }

        // Calculate the full (4-digit) year

        if self.century_register != 0 {
            time.year += (time.century as u16) * 100;
        } else {
            time.year += 2000;
        }

        time
    }
}

pub fn now() -> Time {
    let mut cmos = ReadRTC::new(0x32);
    cmos.read()
}
