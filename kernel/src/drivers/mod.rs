use crate::mm::virt_to_phys;

pub mod interrupt;
pub mod misc;
pub mod timer;
pub mod uart;
pub mod virtio;

pub fn init_early() {
    uart::init_early();
}

pub fn init() {
    println!("Initializing drivers...");
    interrupt::init();
    uart::init();
    timer::init();
    println!("virtio-blk-init initializing");
    virtio::virtio_blk_init();
    println!("virtio-blk-init succeed");
    let buf: [u8; 512] = [12; 512];
    let buf2: [u8; 512] = [0; 512];
    virtio::write(0, 1, virt_to_phys(&buf as *const u8 as usize));
    virtio::read(0, 1, virt_to_phys(&buf2 as *const u8 as usize));
    assert_eq!(buf[0], buf2[0]);
    assert_eq!(buf[500], buf2[500]);
    println!("buf2[0] is {}", buf2[0]);

}
