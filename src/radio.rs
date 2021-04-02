use rustfft::num_complex::Complex;

const SAMPLE_BLOCK_SIZE = 2048;

pub struct RealSamples {
    samples: Vec<u8>
}

pub struct IQSamples {
    samples: Vec<Complex<u8>>
}

pub struct SampleFile {
    pub filename: String,
    pub mut buffer: [0; SAMPLE_BLOCK_SIZE],
    pub mut buffer_fill: int
}


impl IQSamples {
    pub fn new() -> Self {
        Self {
            samples: vec![]
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
        return &(self.samples)
    }
    
    fn from_file() -> std::io::Result<[u8; FFT_SAMPLE_COUNT]> {
        let mut file = File::open("data.dat")?;
        
        let mut buffer = [0; FFT_SAMPLE_COUNT];
    
        file.read(&mut buffer)?;
    
        return Ok(buffer);
    }
    
}