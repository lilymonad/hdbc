use clap::{builder::TypedValueParser, error::ErrorKind, Parser, ValueEnum};
use std::{collections::HashSet, hash::Hash, marker::PhantomData};

#[derive(ValueEnum, Clone, Copy, Hash, PartialEq, Eq, Debug)]
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

#[derive(Clone)]
struct MultiReprU64Parser;

impl TypedValueParser for MultiReprU64Parser {
    type Value = u64;
    fn parse_ref(
        &self,
        cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        raw_value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let value = raw_value
            .to_str()
            .ok_or_else(|| clap::Error::new(ErrorKind::InvalidUtf8).with_cmd(cmd))?;

        let value = if value.starts_with("0b") {
            u64::from_str_radix(&value[2..], 2)
        } else if value.starts_with("0x") {
            u64::from_str_radix(&value[2..], 16)
        } else {
            u64::from_str_radix(&value, 10)
        }
        .map_err(|_| clap::Error::new(ErrorKind::InvalidValue).with_cmd(cmd))?;

        Ok(value)
    }
}

#[derive(Clone, Default)]
struct EnumSetParser<T> {
    sep_char: char,
    _phantom: PhantomData<T>,
}

impl<T> EnumSetParser<T> {
    pub fn new(sep_char: char) -> Self {
        Self {
            sep_char,
            _phantom: PhantomData,
        }
    }
}

impl<T: ValueEnum + Hash + Eq + Send + Sync + 'static> TypedValueParser for EnumSetParser<T> {
    type Value = HashSet<T>;
    fn parse_ref(
        &self,
        cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        raw_value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let value = raw_value
            .to_str()
            .ok_or_else(|| clap::Error::new(ErrorKind::InvalidUtf8).with_cmd(cmd))?;

        let mut set = HashSet::new();
        for arg in value.split(|c| c == self.sep_char) {
            let arg_enum = <T as ValueEnum>::from_str(arg, true)
                .map_err(|_| clap::Error::new(ErrorKind::Io).with_cmd(cmd))?;
            set.insert(arg_enum);
        }

        Ok(set)
    }
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
        value_parser = EnumSetParser::<Representation>::new(','),
        help = "Representations to be printed",
        long_help = "Possible values are h for hex, d for decimal, b for binary. Put multiple between commas to print multiple representations.",
        default_value = "h,d,b"
    )]
    representation: HashSet<Representation>,

    #[arg(
        short,
        long,
        help = "Only show the number, you may want to use it to pipe the output to something else"
    )]
    simplified: bool,

    #[arg(value_parser=MultiReprU64Parser, help = "The number to convert (prefixed with 0x if hexadecimal, or 0b of binary)")]
    value: u64,
}

fn main() {
    let cfg = AppConfig::parse();
    for repr in cfg.representation {
        if cfg.simplified {
            display_simplified(cfg.value, repr, cfg.format);
        } else {
            display_detailed(cfg.value, repr, cfg.format);
        }
    }
}
