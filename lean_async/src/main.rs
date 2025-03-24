use trpl::{Either, Html};

async fn page_title(url: &str) -> (&str, Option<String>) {
    // let response = trpl::get(url).await; // both these steps are async
    // let response_text = response.text().await;
    // the above code can also be chained and simplified to:
    let response_text = trpl::get(url).await.text().await;
    // can only use await within async functions / blocks
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|element| element.inner_html());

    (url, title)
}

fn main() {
    // cargo run -- https://www.rust-lang.org https://www.github.com
    let args: Vec<String> = std::env::args().collect();
    // marking something as async means it returns a future
    trpl::run(async {
        let title1 = page_title(&args[1]);
        let title2 = page_title(&args[2]);
        let (url, possible_title) = match trpl::race(title1, title2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };
        println!("{url} returned first");
        match possible_title {
            Some(title) => println!("Title: {title}"),
            None => println!("No url found!"),
        };

    });

    println!("Hello, world!");
}
