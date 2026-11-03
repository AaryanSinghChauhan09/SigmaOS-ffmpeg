use anyhow::Result;
use clap::Parser;
use ffmpeg::ffmpeg;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    no_hw_accel: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = ffmpeg::Config { 
        hardware_acceleration: !cli.no_hw_accel, 
        ..Default::default() 
    };
    let mut component = ffmpeg::new(config);
    
    component.initialize()?;
    component.run()?;
    
    println!("SigmaOS-ffmpeg ready");
    Ok(())
}
