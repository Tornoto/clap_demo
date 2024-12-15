use clap::{command, Arg, Command};

fn main() {
    process();
}

/// 如下是运行 help 的展示 注意理解如下概念
///
/// Command Argument Option
///
/// ### Command
/// 对于每个 sub-command 来说，都是一个命令 Command，命令内部的参数仅在子命令上下文可见
///
/// ### (position) Argument
/// 位置参数顺序敏感(敏感是相对于其他位置参数)
///
/// ### (named argument) Option
/// 如果参数设置了 short 或 long 后，位置参数会变成 Option。Option 是全局的。子命令也可以使用这些选项。
///
/// ```bash
/// my_cli -lowercase true flfy echo 'some user input'
/// ```
///
/// 在上面的的例子中
/// - flfy 是位置参数对应的值 (不需要想其他arg那样写参数id)（位置敏感是相对其他位置参数）
/// - -lowercase 是 named argument，上送的 true
/// - echo 是子命令 'some user input' 是上送的值
///
/// ``` bash
/// Usage: my_cli [OPTIONS] [fluffy] [COMMAND]
/// Commands:
/// echo          echo use input
/// echo-reverse  echo user input in reverse
/// help          Print this message or the help of the given subcommand(s)
///
/// Arguments:
/// [fluffy]
///
/// Options:
///     --lowercase <lowercase>  [possible values: true, false]
///     --uppercase
/// -h, --help                   Print help
/// -V, --version                Print version
/// ```
fn process() {
    let match_result = command!()
        // 注意理解子命令和位置
        .subcommand(
            Command::new("echo")
                .about("echo use input")
                .arg(Arg::new("input")),
        )
        .subcommand(
            Command::new("echo-reverse")
                .about("echo user input in reverse")
                .arg(Arg::new("input")),
        )
        // 注意 lowercase 和 uppercase 的取值方式
        // lowercase 需要输入 --lcase true 或 --lcase false
        // 通过 value_parser 设置
        // 而 uppercase 利用 action 设置，只要输出 --ucase 即可
        .arg(
            Arg::new("lowercase")
                .long("lowercase")
                // 设置别名，可以输入 --lowercase true / --lower-case false / lcase true
                .aliases(["lc", "lower-case", "lcase"])
                // 设置解析方式 如果输入不是合法的 bool 值 (true/false) 则报错
                .value_parser(clap::value_parser!(bool)),
        )
        .arg(
            Arg::new("uppercase")
                .long("uppercase")
                .aliases(["uc", "upper-case", "ucase"])
                // 设置为标志位 可以仅输入 --uppercase 即可
                .action(clap::ArgAction::SetTrue),
        )
        .arg(Arg::new("fluffy").index(1))
        .get_matches();
    // println!(">> {:?}", match_result);

    let is_lower_case = match_result.get_flag("lowercase");
    let is_upper_case = match_result.get_one::<bool>("uppercase").unwrap_or(&false);
    let fluffy = match_result.get_one::<String>("fluffy");

    println!("user input fluffy: {:?}", fluffy);

    match match_result.subcommand() {
        Some(("echo", sub_m)) => {
            if let Some(input) = sub_m.get_one::<String>("input") {
                if is_lower_case {
                    println!("{:?}", input.to_lowercase());
                    return;
                }
                if *is_upper_case {
                    println!("{:?}", input.to_uppercase());
                    return;
                }
                println!("{:?}", input.to_uppercase());
            }
        }
        Some(("echo-reverse", sub_m)) => {
            if let Some(input) = sub_m.get_one::<String>("input") {
                let input_rev: String = input
                    .split(" ")
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect::<Vec<_>>()
                    .join(" ");
                println!("reverse echo: {:?}", input_rev)
            }
        }
        None => todo!(),
        _ => todo!(),
    }
}
