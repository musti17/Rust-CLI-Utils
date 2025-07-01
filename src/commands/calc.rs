use clap::Args;

#[derive(Args)]
pub struct CalcArgs {
    pub left: i32,
    #[arg(value_parser=["+", "-", "*", "/"])]
    pub operator: String,
    pub right: i32,
}

pub fn run(args: &CalcArgs) {
    let result = match args.operator.as_str() {
        "+" => args.left + args.right,
        "-" => args.left - args.right,
        "*" => args.left * args.right,
        "/" => args.left / args.right,
        _ => unreachable!("Operator should have been validated by clap"),
    };

    println!("Result: {}", result);
}
