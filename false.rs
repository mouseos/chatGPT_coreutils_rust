use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // 環境変数 POSIXLY_CORRECT が設定されている場合、 --help と --version を無視する
    let ignore_help_and_version = env::var("POSIXLY_CORRECT").is_ok();

    // 引数に "--help" が指定されているか
    let is_help = args.iter().any(|x| x == "--help");

    // 引数に "--version" が指定されているか
    let is_version = args.iter().any(|x| x == "--version");

    if !ignore_help_and_version && is_help {
        // "--help" が指定されている場合は、使用方法のメッセージを標準出力に出力して正常終了
        println!("Usage: false [--help] [--version]");
        std::process::exit(0);
    } else if !ignore_help_and_version && is_version {
        // "--version" が指定されている場合は、バージョン情報を標準出力に出力して正常終了
        println!("false version 1.0.0");
        std::process::exit(0);
    }

    // 失敗ステータスとして「失敗」を意味する 1 を返す
    std::process::exit(1);
}
