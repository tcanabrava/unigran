extern crate clap;

fn main() {
    let args = clap::App::new("IMC Calculations App")
        .version("1.0")
        .author("Tomaz Canabrava")
        .about("Calculates the IMC for a single person")
        .arg(clap::Arg::with_name("height")
            .short("h")
            .long("height")
            .help("Your height in meters")
            .takes_value(true)
            .required(true)
        )
        .arg(clap::Arg::with_name("weight")
            .short("w")
            .long("weight")
            .help("Your weight in kgs")
            .takes_value(true)
            .required(true)
        ).get_matches();
    println!("{:#?}", args);
}
