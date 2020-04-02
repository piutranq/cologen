pub mod color;
pub mod replace;
pub mod scheme;

use getopt::prelude::*;
use replace::Replacer;

const APPNAME: &'static str = std::env!("CARGO_PKG_NAME");
const APPVER:  &'static str = std::env!("CARGO_PKG_VERSION");

fn print_version() {
    println!("{} v{}", APPNAME, APPVER);
}

fn print_help() {
    println!("{} v{}", APPNAME, APPVER);
    println!(
"Usage)
    [TEMPLATE] | {} [SCHEME] > [OUTPUT]

    [TEMPLATE]: Template configuration file for target.
                It must be input at stdin.
    [SCHEME]: Path of color scheme file. (YAML format)
    [OUTPUT]: The generated configuration is printed at stdout.

Example)
    cat $XDG_CONFIG_HOME/cologen/templates/rofi.template \\
    | cologen $XDG_CONFIG_HOME/cologen/schemes/gruvbox-dark.yaml \\
    > $XDG_CONFIG_HOME/rofi/color.rasi
"
    , APPNAME)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Vec<String> = std::env::args().collect();
    let mut opts = Parser::new(&args, "hv");

    // Parse opts
    loop {
        match opts.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('h', None) => {
                    print_help();
                    return Ok(())
                }
                Opt('v', None) => {
                    print_version();
                    return Ok(())
                },
                _ => unreachable!(),
            }
        }
    }
    let argv = args.split_off(opts.index());

    // Init the color scheme
    let scheme_path = argv[0].clone();
    let mut replacer = Replacer::new(scheme_path);

    replacer.execute();

    Ok(())
}
