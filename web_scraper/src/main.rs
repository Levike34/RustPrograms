use std::collections::HashMap;
use std::io::*;

fn main() {

    // let response = reqwest::blocking::get(
    //     "https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=100",
    // )
    // .unwrap()
    // .text()
    // .unwrap();

    // let document = scraper::Html::parse_document(&response);

    // let title_selector = scraper::Selector::parse("h3.lister-item-header>a").unwrap();

    // let titles = document.select(&title_selector).map(|x| x.inner_html());

    // titles
    //     .zip(1..101).for_each(|(item, number)| println!("{}, {}", number, item));

    get_mate();
}

pub fn get_btc_price() {
    let response = reqwest::blocking::get(
        "https://www.forbes.com/advisor/investing/cryptocurrency/top-10-cryptocurrencies/",
    )
    .unwrap()
    .text()
    .unwrap();

    let document = scraper::Html::parse_document(&response);

    let coin_selector = scraper::Selector::parse("h3").unwrap();

    let price_selector = scraper::Selector::parse("li > strong").unwrap();

    let coin = document.select(&coin_selector).map(|x| x.inner_html());

     let price = document.select(&price_selector).map(|x| x.inner_html());

     const EMPTY_STRING: String = String::new();
    let mut arr: [String; 126] = [EMPTY_STRING; 126];

        coin
        .zip(1..11).for_each(|(item, number)| arr[number] = item);

         price
        .zip(1..11).for_each(|(item, number)| 
        println!("{}:  {}", arr[number], item)

    );//https://www.amazon.com/s?k=mate+1kg&crid=1R85U1IRXU29R&sprefix=mate+1%2Caps%2C432&ref=nb_sb_noss_2
     

}


pub fn get_mate() {
    let response = reqwest::blocking::get(
        "https://www.amazon.com/s?k=mate+1kg&crid=1R85U1IRXU29R&sprefix=mate+1%2Caps%2C432&ref=nb_sb_noss_2",
    )
    .unwrap()
    .text()
    .unwrap();

    let document = scraper::Html::parse_document(&response);

    let coin_selector = scraper::Selector::parse("a.a-link-normal > span.a-size-base-plus").unwrap();


    let coin = document.select(&coin_selector).map(|x| x.inner_html());


     const EMPTY_STRING: String = String::new();
    let mut arr: [String; 126] = [EMPTY_STRING; 126];


         coin 
        .zip(1..101).for_each(|(item, number)| 
        println!("{}:  {}", number, item)

    );
     

}
