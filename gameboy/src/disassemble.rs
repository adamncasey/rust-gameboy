

pub struct Disassembly {
    addr: u16,
    data: Vec<u8>,
    desc: [15; char]
}