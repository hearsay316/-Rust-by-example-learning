use reqwest::Error;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let start_time = Instant::now();
    // let mut async_arr = vec![];
    // let a = reqwest::get(url);
    // async_arr.push(a);
    // let b = reqwest::get(url);
    // async_arr.push(b);
    // let c = reqwest::get(url);
    // async_arr.push(c);
    // let d = reqwest::get(url);
    // async_arr.push(d);
    //
    // for i in async_arr {
    //     let _ = i.await?;
    // }

    tokio::join!(
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url)
    );
    // let _ = tokio::join!(a, b, c, d);
    println!("{:?}", start_time.elapsed());
    Ok(())
}
