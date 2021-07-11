use std::env;

use web_view::*;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    web_view::builder()
        .title("Windy")
        .content(Content::Url(
            args.get(1).expect("No URL argument - panicking"),
        ))
        .size(800, 300)
        .resizable(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
