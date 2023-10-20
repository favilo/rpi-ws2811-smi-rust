use rpi_ws2811_smi::Ws2811;

fn main() -> anyhow::Result<()> {
    let mut ws2811 = Ws2811::new(100)?;
    ws2811.clear()?;

    let channel = 1;
    for led in 0..8 {
        println!("Setting Pixel: {} on channel {}", led, channel);
        ws2811.set_pixel(
            channel,
            led,
            rpi_ws2811_smi::Rgba {
                r: 64,
                g: 0,
                b: 0,
                a: 255,
            },
        )?;
    }
    // println!("Buffer: {:?}", ws2811.buffer);

    ws2811.send()?;
    // loop {
    //      ws2811.send()?;
    // }

    Ok(())
}
