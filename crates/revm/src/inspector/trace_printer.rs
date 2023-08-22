use serde_json::{self, Value};
use std::io::Write;

struct StdOutPrinter {
    opcode_filter: u8,
}

impl Write for StdOutPrinter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match serde_json::from_slice::<Value>(buf) {
            Ok(v) => {
                let opcode = v
                    .get("op")
                    .expect("'op' field missing")
                    .as_u64()
                    .expect("opcode parsing") as u8;
                if opcode == self.opcode_filter {
                    println!("{}", v)
                }
                Ok(buf.len())
            }
            Err(_) => Ok(0),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}
