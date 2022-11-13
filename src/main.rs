use clap::{ArgAction, Parser, ValueEnum};

#[derive(ValueEnum, Clone, Copy, Debug)]
#[repr(u64)]
enum Representation {
    B,
    D,
    H,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
#[repr(u64)]
enum Format {
    U32 = 32,
    U64 = 64,
}

fn display_detailed(value: u64, repr: Representation, format: Format) {
    match repr {
        Representation::B => {
            print!("bin: ");

            if let Format::U64 = format {
                for i in (32..=63).rev() {
                    if value & 1 << i != 0 {
                        print!("1")
                    } else {
                        print!("0")
                    }
                    if i % 4 == 0 {
                        print!(" ")
                    }
                }
                println!();
                println!("       60   56   52   48   44   40   36   32");
                print!("     ");
            }

            for i in (0..=31).rev() {
                if value & 1 << i != 0 {
                    print!("1")
                } else {
                    print!("0")
                }
                if i % 4 == 0 {
                    print!(" ")
                }
            }
            println!();
            println!("       28   24   20   16   12    8    4    0");
        }
        Representation::D => {
            println!("dec: {value:39}")
        }
        Representation::H => {
            println!("hex: {value:39x}")
        }
    }
}

fn display_simplified(value: u64, repr: Representation, format: Format) {
    match repr {
        Representation::B => (0..format as u64).rev().for_each(|i| {
            let c = if value & 1 << i != 0 { '1' } else { '0' };
            print!("{c}");
        }),
        Representation::D => print!("{value}"),
        Representation::H => print!("{value:x}"),
    }
    println!();
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct AppConfig {
    #[arg(
        short,
        long,
        help = "Show 64 bits for the binary form instead of 32",
        default_value = "u32"
    )]
    format: Format,

    #[arg(
        short,
        long = "repr",
        value_delimiter = ',',
        value_enum,
        help = "Representations to be printed",
        long_help = "Possible values are h for hex, d for decimal, b for binary. Put multiple between commas to print multiple representations.",
        default_value = "h,d,b"
    )]
    representation: Vec<Representation>,

    #[arg(
        short,
        long,
        help = "Only show the number, you may want to use it to pipe the output to something else"
    )]
    simplified: bool,

    num: String,
}

fn main() {
    let cfg = AppConfig::parse();

    let value = if cfg.num.starts_with("0b") {
        u64::from_str_radix(&cfg.num[2..], 2).expect("binary value")
    } else if cfg.num.starts_with("0x") {
        u64::from_str_radix(&cfg.num[2..], 16).expect("hex value")
    } else {
        u64::from_str_radix(&cfg.num, 10).expect("decimal value")
    };

    for repr in cfg.representation {
        if cfg.simplified {
            display_simplified(value, repr, cfg.format);
        } else {
            display_detailed(value, repr, cfg.format);
        }
    }
}
