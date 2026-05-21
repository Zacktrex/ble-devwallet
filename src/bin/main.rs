#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use bt_hci::controller::ExternalController;
use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::rng::{Trng, TrngSource};
use esp_hal::timer::timg::TimerGroup;
// use esp_println as _;
use esp_radio::ble::controller::BleConnector;
use esp_storage::FlashStorage;
// use trouble_host::prelude::*;
use {esp_alloc as _, esp_backtrace as _};

use ble_devwallet::ble;

use esp_hal::{
    gpio::{Input, Level, Output, Pull},
    spi::master::{Config as SpiConfig, Spi},
};

use esp_hal::gpio::InputConfig;
use esp_hal::gpio::OutputConfig;

use ble_devwallet::display;

extern crate alloc;

// #[panic_handler]
// fn panic(_: &core::panic::PanicInfo) -> ! {
//     loop {}
// }

// const CONNECTIONS_MAX: usize = 1;
// const L2CAP_CHANNELS_MAX: usize = 1;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(_spawner: Spawner) -> ! {
    // esp_println::logger::init_logger_from_env();
    // generator version: 1.3.0
    // generator parameters: --chip esp32c3 -o unstable-hal -o alloc -o embassy -o ble-trouble -o defmt -o ci -o wokwi -o vscode

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 66320);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    let _trng_source = TrngSource::new(peripherals.RNG, peripherals.ADC1);
    let mut trng = Trng::try_new().unwrap();

    info!("Embassy initialized!");

    // find more examples https://github.com/embassy-rs/trouble/tree/main/examples/esp32
    let transport = BleConnector::new(peripherals.BT, Default::default()).unwrap();
    let ble_controller = ExternalController::<_, 1>::new(transport);
    // let mut resources: HostResources<DefaultPacketPool, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX> =
    //     HostResources::new();
    // let _stack = trouble_host::new(ble_controller, &mut resources);

    let flash_storage = FlashStorage::new(peripherals.FLASH);
    use embedded_storage::nor_flash::{NorFlash, ReadNorFlash};
    let erase_size = <FlashStorage as NorFlash>::ERASE_SIZE as u32;
    let capacity = flash_storage.capacity() as u32;
    let storage_range = (capacity - erase_size * 2)..capacity;
    let mut flash = embassy_embedded_hal::adapter::BlockingAsync::new(flash_storage);

    let spi = Spi::new(peripherals.SPI2, SpiConfig::default())
        .unwrap()
        .with_sck(peripherals.GPIO6)
        .with_mosi(peripherals.GPIO7);

    let cs = Output::new(peripherals.GPIO10, Level::High, OutputConfig::default());

    let dc = Output::new(peripherals.GPIO4, Level::Low, OutputConfig::default());

    let rst = Output::new(peripherals.GPIO5, Level::High, OutputConfig::default());

    let busy = Input::new(
        peripherals.GPIO3,
        InputConfig::default().with_pull(Pull::None),
    );

    display::init_display(spi, cs, dc, rst, busy);

    info!("E-paper display initialized");

    ble::run(ble_controller, &mut trng, &mut flash, storage_range).await;

    // TODO: Spawn some tasks
    // let _ = spawner;

    loop {
        info!("Hello world!");
        Timer::after(Duration::from_secs(1)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.1.0/examples
}
