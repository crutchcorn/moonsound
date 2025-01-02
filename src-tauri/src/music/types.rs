use rodio::Decoder;
use std::fs::File;
use std::io::BufReader;

pub type PeriodicCallback = Box<dyn FnMut(&mut Decoder<BufReader<File>>) + Send + 'static>;
