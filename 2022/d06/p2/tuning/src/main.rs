use std::collections::{HashSet, VecDeque};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};

const MARKER_LEN: usize = 14;

struct SignalBuf {
    buf: VecDeque<u8>,
    set: HashSet<u8>,
    read: usize,
}

impl SignalBuf {
    fn new(init: Vec<u8>) -> SignalBuf {
        SignalBuf {
            buf: VecDeque::from_iter(init.iter().cloned()),
            set: HashSet::from_iter(init.iter().cloned()),
            read: init.len(),
        }
    }

    fn read(&mut self, c: u8) {
        let old = self.buf.pop_front().unwrap();
        if !self.buf.contains(&old) {
            self.set.remove(&old);
        }

        self.buf.push_back(c);
        self.set.insert(c);

        self.read += 1;
    }

    fn unique(&self) -> bool {
        self.buf.len() == self.set.len()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;

    let reader = io::BufReader::new(input);

    let chars = &mut reader.bytes().filter_map(|x| x.ok());

    let init = chars.by_ref().take(MARKER_LEN).collect::<Vec<u8>>();
    let mut buf = SignalBuf::new(init);

    for c in chars {
        if buf.unique() {
            break;
        }
        buf.read(c)
    }

    println!("{}", buf.read);

    Ok(())
}
