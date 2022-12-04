// yesコマンドの実装
use std::env; // 引数を取得するために必要
use std::process; // プロセスを終了するために必要

// メイン関数
fn main() {
// 引数を取得する
let args: Vec<String> = env::args().collect();


// 引数がない場合は、デフォルトの文字列を出力し続ける
if args.len() == 1 {
    loop {
        println!("y");
    }
}

// 引数が1つの場合は、オプションの処理を行う
if args.len() == 2 {
    // 引数が"--help"の場合は、使用方法を表示して正常終了する
    if args[1] == "--help" {
        println!("Usage: yes [string...]");
        println!("Repeatedly output a line with all specified STRING(s), or 'y'");
        process::exit(0);
    }
    // 引数が"--version"の場合は、バージョン情報を表示して正常終了する
    if args[1] == "--version" {
        println!("yes version 0.1.0");
        process::exit(0);
    }
}

// 引数が2つ以上の場合は、指定された文字列を出力し続ける
if args.len() >= 2 {
    let mut output_string = String::new();
    // 2つ目以降の引数をすべて結合する
    for i in 2..args.len() {
        output_string.push_str(&args[i]);
        output_string.push_str(" ");
    }
    // 改行を追加する
    output_string.push_str("\n");

    // 指定された文字列を出力し続ける
    loop {
        print!("{}", output_string);
    }
}
}