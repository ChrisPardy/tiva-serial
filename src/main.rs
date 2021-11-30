#![no_std]
#![no_main]

mod passthrough;
use passthrough::SerialForwarder;

use core::panic::PanicInfo;
use cortex_m_rt::entry;
use tm4c123x_hal::{self as hal, prelude::*};

fn go() -> ! {
    let p = hal::Peripherals::take().unwrap();

    let mut sc = p.SYSCTL.constrain();
    sc.clock_setup.oscillator = hal::sysctl::Oscillator::Main(
        hal::sysctl::CrystalFrequency::_16mhz,
        hal::sysctl::SystemClock::UsePll(hal::sysctl::PllOutputFrequency::_80_00mhz),
    );
    let clocks = sc.clock_setup.freeze();

    let mut porta = p.GPIO_PORTA.split(&sc.power_control);
    let mut portb = p.GPIO_PORTB.split(&sc.power_control);
    let mut portc = p.GPIO_PORTC.split(&sc.power_control);

    // create uart objects
    let uart0 = hal::serial::Serial::uart0(
        p.UART0,
        porta
            .pa1
            .into_af_push_pull::<hal::gpio::AF1>(&mut porta.control),
        porta
            .pa0
            .into_af_push_pull::<hal::gpio::AF1>(&mut porta.control),
        (),
        (),
        115200_u32.bps(),
        hal::serial::NewlineMode::SwapLFtoCRLF,
        &clocks,
        &sc.power_control,
    );

    let uart1 = hal::serial::Serial::uart1(
        p.UART1,
        portb
            .pb1
            .into_af_push_pull::<hal::gpio::AF1>(&mut portb.control),
        portb
            .pb0
            .into_af_push_pull::<hal::gpio::AF1>(&mut portb.control),
        (),
        (),
        115200_u32.bps(),
        hal::serial::NewlineMode::SwapLFtoCRLF,
        &clocks,
        &sc.power_control,
    );

    let uart4 = hal::serial::Serial::uart4(
        p.UART4,
        portc
            .pc5
            .into_af_push_pull::<hal::gpio::AF1>(&mut portc.control),
        portc
            .pc4
            .into_af_push_pull::<hal::gpio::AF1>(&mut portc.control),
        (),
        (),
        115200_u32.bps(),
        hal::serial::NewlineMode::SwapLFtoCRLF,
        &clocks,
        &sc.power_control,
    );
    let mut chars = "Hello World!\r\n".chars().cycle();

    let (uart0_tx, _uart0_rx) = uart0.split();
    let (mut uart1_tx, uart1_rx) = uart1.split();
    let (uart4_tx, uart4_rx) = uart4.split();
    let mut u1_to_u4 = SerialForwarder::new(uart1_rx, uart4_tx);
    let mut u4_to_u0 = SerialForwarder::new(uart4_rx, uart0_tx);

    let mut c = chars.next().unwrap();
    loop {
        match uart1_tx.write(c as u8) {
            Ok(_) => {
                c = chars.next().unwrap();
            }
            Err(_) => {}
        }

        u1_to_u4.poll_and_forward();
        u4_to_u0.poll_and_forward();
    }
}

#[entry]
fn main() -> ! {
    go()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
