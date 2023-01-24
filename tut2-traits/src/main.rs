use std::str::FromStr;

struct Equation {
    left: String,
    right: String,
    is_equal: bool,
}

// 1 + 1 = 2
// 10 = 1 + 2 + 3 + 4
// 10 = 6 + 7

struct ParseEquationError;

fn add_s(x: &str) -> usize {
    x.split("+")
        .map(|y| -> usize { y.trim().parse::<usize>().unwrap_or(0) })
        .sum()
}

impl FromStr for Equation {
    type Err = ParseEquationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // implementation one
        /*
        match s.split_once("=") {
            Some((l, r)) => Ok(Equation {
                left: l.trim().to_string(),
                right: r.trim().to_string(),
                is_equal: add_s(l) == add_s(r),
            }),
            None => Err(ParseEquationError),
        }
        */
        s.split_once("=")
            .and_then(|(l, r)| {
                Some(Equation {
                    left: l.trim().to_string(),
                    right: r.trim().to_string(),
                    is_equal: add_s(l) == add_s(r),
                })
            })
            .ok_or(ParseEquationError)
    }
}

fn main() {
    parse_print("124".to_string());
    parse_print("one".to_string());
    parse_print_eq("1 + 1 = 2".to_string());
    parse_print_eq("1 + 1 = 3".to_string());
}

fn parse_print(num_str: String) {
    let num = num_str.parse::<usize>();

    match num {
        Ok(n) => println!("The number is {n}."),
        Err(_) => println!("Failed finding number in '{num_str}'."),
    }
}

fn parse_print_eq(num_str: String) {
    let num = num_str.parse::<Equation>();

    match num {
        Ok(Equation {
            left,
            right,
            is_equal,
        }) => {
            println!(
                "{} is{} {}.",
                left,
                if is_equal { "" } else { " not" },
                right,
            )
        }
        Err(_) => println!("Failed finding an equation in '{num_str}'."),
    }
}
