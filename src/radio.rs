use std::fs::File;
use std::io::{Write,stdout};
use std::io;
use std::path::Path;
use std::io::BufWriter;
use rustfft::num_complex::Complex;

const SAMPLE_BLOCK_SIZE: usize = 2048;

pub struct RealSamples {
    samples: Vec<u8>
}

#[derive(Debug)]
pub struct IQSamples {
    samples: Vec<Complex<u8>>
}

/*pub struct SampleFile {
    pub filename: String,
    pub buffer: [0; SAMPLE_BLOCK_SIZE],
    pub buffer_fill: u64
}*/


impl IQSamples {
    pub fn new() -> Self {
        Self {
            samples: vec![]
        }
    }
    pub fn from_samples(samples: Vec<Complex<u8>>) -> Self {
        Self {
            samples: samples
        }
    }
    pub fn from_IQ_stream(data: &[u8]) -> Self {
        Self {
            samples: data.chunks(2)
                         .map(
                             |coord| Complex {re: coord[0], im:coord[1]}
                         )
                         .collect()
       }
    }
    
    pub fn get_samples(&self) -> Vec<Complex<u8>> {
        self.samples.clone()
    }
    
    
    pub fn to_file(&self, filename: Option<String>) -> std::io::Result<()> {
        let mut out_writer = match filename {
            Some(x) => {
                let path = Path::new(&x);
                Box::new(File::create(&path).unwrap()) as Box<Write>
            }
            None => Box::new(io::stdout()) as Box<Write>,
        };
        for sample in self.samples.iter() {
            out_writer.write(&[sample.re, sample.im])?;
        }
        
        
        Ok(())
        
    }
/*    fn from_file() -> std::io::Result<[u8; SAMPLE_BLOCK_SIZE]> {
        let mut file = File::open("data.dat")?;
        
        let mut buffer = [0; SAMPLE_BLOCK_SIZE];
    
        file.read(&mut buffer)?;
    
        return Ok(buffer);
    }*/
    
}