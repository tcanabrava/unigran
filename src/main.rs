extern crate clap;

fn imc_calculation(mass: f64, height: f64) -> f64 {
    let result = mass / f64::powf(height, 2.0);
    return result;
}

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

    let weight = clap::value_t!(args.value_of("weight"), f64)
        .expect("You need to enter your weight for the IMC calculation.");

    let height = clap::value_t!(args.value_of("height"), f64)
        .expect("You need to enter your height for the IMC calculation");

    let message = match imc_calculation(weight, height) {
        x if x > 25.0 => "You are overweight",
        x if x < 18.0 => "You are underweight",
        _ => "You are ok"
    };
    println!("{}", message);
}
