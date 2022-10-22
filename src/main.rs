use warp::{http::Uri, Filter};

#[tokio::main]
async fn main() {
    let hello = warp::path!(String).map(|link: String| {
        warp::redirect(Uri::from_static(match link.as_str() {
            "discord" => "https://discord.gg/SZhzv6z",
            "donate" => "https://www.patreon.com/synixe",
            _ => "https://synixe.contractors",
        }))
    });

    // match
    warp::serve(hello).run(([0, 0, 0, 0], 8080)).await;
}
