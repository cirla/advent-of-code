use std::collections::{HashMap, VecDeque};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};

struct SignalBuf {
    buf: VecDeque<u8>,
    counter: HashMap<u8, usize>,
    bytes_read: usize,
}

impl SignalBuf {
    fn new(init: Vec<u8>) -> SignalBuf {
        SignalBuf {
            buf: VecDeque::from_iter(init.iter().cloned()),
            counter: HashMap::from_iter(init.iter().cloned().map(|c| (c, 1))),
            bytes_read: init.len(),
        }
    }

    fn read(&mut self, c: u8) {
        // update circular buffer
        let old = self.buf.pop_front().unwrap();
        self.buf.push_back(c);

        // update counter
        if *self
            .counter
            .entry(old)
            .and_modify(|count| *count -= 1)
            .or_default() // never used as old is in set
            == 0
        {
            self.counter.remove(&old);
        }

        self.counter
            .entry(c)
            .and_modify(|count| *count += 1)
            .or_insert(1);

        // update read count
        self.bytes_read += 1;
    }

    fn unique(&self) -> bool {
        self.buf.len() == self.counter.len()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let marker_len: usize = args[2].parse()?;

    let reader = io::BufReader::new(input);
    let chars = &mut reader.bytes().filter_map(|x| x.ok());

    let init = chars.by_ref().take(marker_len).collect::<Vec<u8>>();
    let mut buf = SignalBuf::new(init);

    // while let Some(c) = chars.next() && !buf.unique()
    while let Some((c, false)) = chars.next().zip(Some(buf.unique())) {
        buf.read(c);
    }

    println!("{}", buf.bytes_read);

    Ok(())
}
