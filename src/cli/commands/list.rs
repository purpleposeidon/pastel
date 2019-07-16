use std::io::{self, Write};

use crate::commands::prelude::*;
use crate::x11colors::{NamedColor, X11_COLORS};

pub struct ListCommand;

impl GenericCommand for ListCommand {
    fn run(&self, matches: &ArgMatches, config: &Config) -> Result<()> {
        let sort_order = matches.value_of("sort").expect("required argument");

        let mut colors: Vec<&NamedColor> = X11_COLORS.iter().map(|r| r).collect();
        if sort_order == "brightness" {
            colors.sort_by_key(|nc| (-nc.color.brightness() * 1000.0) as i32);
        } else if sort_order == "luminance" {
            colors.sort_by_key(|nc| (-nc.color.luminance() * 1000.0) as i32);
        } else if sort_order == "hue" {
            colors.sort_by_key(|nc| (nc.color.to_lch().h * 1000.0) as i32);
        } else if sort_order == "chroma" {
            colors.sort_by_key(|nc| (nc.color.to_lch().c * 1000.0) as i32);
        }
        colors.dedup_by(|n1, n2| n1.color == n2.color);

        if config.interactive_mode {
            for nc in colors {
                let bg = &nc.color;
                let fg = bg.text_color();
                println!(
                    "{}",
                    fg.to_termcolor()
                        .on(bg.to_termcolor())
                        .paint(format!(" {:24}", nc.name))
                );
            }
        } else {
            let stdout = io::stdout();
            let mut out = stdout.lock();
            for nc in colors {
                let res = writeln!(out, "{}", nc.name);
                if res.is_err() {
                    break;
                }
            }
        }

        Ok(())
    }
}