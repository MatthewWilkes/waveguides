extern crate itertools_num;

use std::fs::File;
use std::io::{Read, Error};
use std::f64::consts::{PI};
use rustfft::num_complex::Complex;
use itertools_num::linspace;


use waveguides::radio::{IQSamples};


const SAMPLE_RATE: usize = 256000;

fn signal(frequency: f64) -> IQSamples {
    let mut count: usize = 0;
    let counter = std::iter::from_fn(move || {
        count += 1;
        let val = (count as f64) * 2.0 * PI * frequency / (SAMPLE_RATE as f64);
        Some(
            Complex {
                re: ((val.cos() * 30.0)+128.0) as u8,
                im: ((val.sin() * 30.0)+128.0) as u8
            }
        )
    });
    
    IQSamples::from_samples(
        counter.take(SAMPLE_RATE*100).collect()
    )
}

fn main() -> Result<(), Error> {
    let data = signal(1600.0);
    //println!("{:?}", data);
    data.to_file(None);
    return Ok(())
}
