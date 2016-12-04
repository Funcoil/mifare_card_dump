extern crate pn532;
extern crate mifare;

use pn532::tags::{ISO14443AListOptions};
use mifare::{MifareTag, KeyOption};

fn main() {
    let i2c = pn532::bus::i2c::open("/dev/i2c-0").unwrap();
    let mut device = pn532::PN532::new(pn532::bus::BusyWait::new(i2c));
    device.sam_configure(pn532::SAMMode::Normal(None)).unwrap();
    let list_opts = ISO14443AListOptions {
        limit: pn532::tags::TagNumLimit::One,
        uid: None
    };
    let mut tag_buf = pn532::tags::TagBuffer::new();
    let tags = device.list_tags(list_opts, &mut tag_buf).unwrap();
    let tag = tags.first();
    let mut tag = MifareTag::new(tag).unwrap();

    let mut rbuf = [0u8; 16];
    for sector_number in 0..16 {
		println!("-------------------Sector {:02}-------------------", sector_number);
        let mut sector = tag.authenticate_sector(sector_number, KeyOption::KeyA, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]).unwrap();
        for block in 0..4 {
            sector.read_block(block, &mut rbuf).unwrap();
            for b in &rbuf {
                print!("{:02X} ", *b);
            }
            println!("");
        }
    }
}
