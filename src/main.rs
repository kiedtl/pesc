mod pesc;
mod errors;
use crate::pesc::*;

fn main() {
    let mut pesc = Pesc::new();
    let code = &std::env::args()
        .collect::<Vec<String>>()[1];

    let parsed = match pesc.parse(code) {
        Ok(r) => r,
        Err(e) => {
            println!("error: {}", e);
            return;
        },
    };

    match pesc.eval(&parsed.1) {
        Ok(()) => pesc.print(),
        Err(e) => println!("error: {}", e),
    }
}
