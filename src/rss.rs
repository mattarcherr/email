use std::error::Error;
use rss::Channel;
use rss::Item;



pub async fn request() -> Result<Channel, Box<dyn Error>> {
   let content = reqwest::get("https://feeds.bbci.co.uk/news/uk/rss.xml")
       .await?
       .bytes()
       .await?;
   let channel = Channel::read_from(&content[..])?;
   Ok(channel)
}

async fn get_feed() -> Result<&'static[Item], Box<dyn Error>> {
    let channel = request().await;
    match channel {
        Ok(channel) => { 
            let feed = channel.items();
            let test = feed.clone();
            Ok(test)

            // let mut i = 0;
            // loop {
            //     if i >= feed.len() { break; }
            //
            //     match feed[i].title() {
            //         Some(title) => { println!("{title}") },
            //         None => {}
            //     }
            //     i += 1;
            // }
        },
        Err(e) => Err(e),
    }
}
