extern crate tui;
extern crate itertools_num;
extern crate rustfft;
extern crate rand;

use rand::prelude::*;

use std::fs::File;
use std::io::Read;

use std::{io, thread, time};
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, BarChart, Borders};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Style, Color, Modifier};


use itertools_num::linspace;
use rustfft::{FftPlanner, num_complex::Complex};

use waveguides::radio::{IQSamples};


const FFT_SAMPLE_COUNT: usize = 256000;

fn signal(data: [u8; FFT_SAMPLE_COUNT]) -> IQSamples {
    //let x = IQSamples::new();
    return IQSamples::from_IQ_stream(&data);
    //let complexes = data.chunks(2).map(|coord| Complex {re: coord[0], im:coord[1]}).collect();
    //return complexes;
    
    /*
    let n: usize = 1024;
    let mut rng = rand::thread_rng();

    let t: Vec<f64> = linspace(0., 10., n).collect();
    let : Vec<f64> = t.iter().map(|x| x.sin()).collect();
    let y2: Vec<f64> = t.iter().map(|x| x.mul_add(rng.gen(), 3.0).sin()).collect();
    let y3: Vec<f64> = t.iter().map(|x| x.mul_add(24.5, 3.0).sin()).collect();

    let y4: Vec<f64> = y1.iter().zip(y2.iter()).map(|(a,b)| a+b).collect();
    let y: Vec<f64> = y3.iter().zip(y4.iter()).map(|(a,b)| a+b).collect();
    return (t, y);*/
}

fn fft(y: Vec<Complex<u8>>, FFT_BIN_COUNT: usize) -> Vec<Complex<f64>> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(FFT_BIN_COUNT);
    let mut b: Vec<Complex<f64>> = y.iter().map(|y| Complex{re: y.re as f64 +0.0, im: y.im as f64+0.0}).collect();
    fft.process(&mut b);
    return b;
}


//let (x, y) = signal();
//let ffty: Vec<f64> = fft(y).iter().map(|val| val.norm()).collect();
//simple_bar_plot(x[0..512].to_vec(), ffty[0..512].to_vec());


fn read_a_file() -> std::io::Result<[u8; FFT_SAMPLE_COUNT]> {
    let mut file = File::open("data.dat")?;
    
    let mut buffer = [0; FFT_SAMPLE_COUNT];

    file.read(&mut buffer)?;

    return Ok(buffer);
}


fn calc_fft_bins(width: u16) -> usize {
    let ALLOWED_SIZES: [usize; 11] = [8, 16, 25, 32, 40, 50, 64, 100, 125, 128, 160];
    return *ALLOWED_SIZES.iter().filter(|x| x < &&(width as usize)).last().unwrap()
}

fn main() -> Result<(), io::Error> {
    let mut FFT_BIN_COUNT: usize = 128;

    
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    //let mut file = File::open("data.dat")?;
    let mut file = io::stdin();
    let mut buffer = [0; FFT_SAMPLE_COUNT];
    let ten_millis = time::Duration::from_millis(1);


    
    let mut count = 0u64;
    loop {
        
        file.read(&mut buffer)?;
        let ffty: Vec<f64> = fft(signal(buffer).get_samples(), FFT_BIN_COUNT).iter().map(|val| val.norm()).collect();
    
        let odata: Vec<(String, u64)> = ffty.iter().map(|y| (String::from(""), y.round() as u64)).collect();
    
    
        
        let mut data = vec![];
        let correction_factor = (FFT_BIN_COUNT as f64).sqrt();
        for i in 1..FFT_BIN_COUNT {
            data.insert(
                i-1, ("", (ffty[i]/correction_factor) as u64)
            );
        }
        data.reverse();
        terminal.draw(|f| {
            let size = f.size();
            let block = BarChart::default()
                .block(Block::default()
                .title("FFT")
                .borders(Borders::ALL)
            ).bar_style(Style::default().fg(Color::Red))
            .label_style(Style::default().fg(Color::White))
            .data(data.as_slice())
            .bar_gap(0)
            .max(128);
            f.render_widget(block, size);
        });
        
        FFT_BIN_COUNT = calc_fft_bins(terminal.size().unwrap().width - 2);
        
        thread::sleep(ten_millis);
        
        count += 1;
        if count > 1000 {
            break;
        }
    };

    //println!("{:?}", ffty);
    Ok(())
}
