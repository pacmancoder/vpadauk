use std::{
    path::Path,
    time::Duration
};
use vpadauk_plotter::plotting_host::{PlottingHost, PlottingHostBuilder};

fn main() {
    let mut host = PlottingHostBuilder::new()
        .plot_width(2048)
        .plot_height(2048)
        .plot_path("out.png")
        .build();

    host.run(Path::new("examples/blink/blink.fw"), Duration::from_millis(500));
}
