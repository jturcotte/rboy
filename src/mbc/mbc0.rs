use mbc::MBC;

pub struct MBC0 {
	rom: Vec<u8>,
}

impl MBC0 {
	pub fn new(data: Vec<u8>) -> Option<MBC0> {
		Some(MBC0 { rom: data })
	}
}

impl MBC for MBC0 {
	fn readrom(&self, a: u16) -> u8 { self.rom[a as uint] }
	fn readram(&self, _a: u16) -> u8 { 0 }
	fn writerom(&mut self, _a: u16, _v: u8) { () }
	fn writeram(&mut self, _a: u16, _v: u8) { () }
}
