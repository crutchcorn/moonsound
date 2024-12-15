use std::fs::File;
use std::io::BufReader;
use rodio::Decoder;

pub type PeriodicCallback = Box<dyn FnMut(&mut Decoder<BufReader<File>>) + Send + 'static>;
