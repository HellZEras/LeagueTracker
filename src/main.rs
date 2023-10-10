#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{egui,IconData};
use reqwest::{Error};
use scraper::{Html, Selector};
use std::sync::mpsc::sync_channel;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    let icon = image::open("D:\\practice\\icon.ico").expect("Failed to open icon path").to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();
    env_logger::init();
    let options = eframe::NativeOptions {
        icon_data: Some(IconData {
            rgba: icon.into_raw(),
            width: icon_width,
            height: icon_height,
        }),
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    let mut bool = 0;
    let mut level = String::new();
    let mut win = String::new();
    let mut wins_losses = String::new();
    let mut rank = String::new();
    let mut lp = String::new();
    let mut username = "".to_owned();
    let client = reqwest::Client::new();
    eframe::run_simple_native("League Tracker", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui|{
            ui.heading("League tracker");
            });
            ui.vertical_centered(|ui| {
                let username_label = ui.label("Your Username: ");
                ui.text_edit_singleline(&mut username)
                    .labelled_by(username_label.id);
            });
            ui.vertical_centered(|ui|{
                let check_button = ui.button("Check");
                if check_button.clicked() {
                    let (tx, rx) = sync_channel::<String>(0);
                    let client = client.clone();
                    let username = username.clone();
                    tokio::spawn(async move {
                        let response = request(username, client).await.unwrap();
                        tx.send(response).unwrap();
                    });
                    let response_new = rx.recv().unwrap();
                    let extracted_data = data_extractor(response_new);
                    level = extracted_data.get(0).unwrap_or(&"Couldnt get Level".to_string()).to_string();
                    win = extracted_data.get(1).unwrap_or(&"Couldnt get Winrate".to_string()).to_string();
                    wins_losses = extracted_data.get(2).unwrap_or(&"Couldnt get Wins and Losses".to_string()).to_string();
                    rank = extracted_data.get(3).unwrap_or(&"Couldnt get Rank".to_string()).to_string();
                    lp = extracted_data.get(4).unwrap_or(&"Couldnt get LP".to_string()).to_string();
                    bool = 1;
                }
            });


            if bool ==1 {
                ui.vertical_centered(|ui|{
                ui.label(format!("Level is: {level}"));
                ui.label(format!("Winrate is: {win}"));
                ui.label(format!("Wins and Losses are: {wins_losses}"));
                ui.label(format!("Rank is: {rank}"));
                ui.label(format!("LP is: {lp}"));
            });}

        });
    })
}

async fn request(username: String,client: reqwest::Client) -> Result<String, Error> {
    let url = format!("https://blitz.gg/lol/profile/euw1/{}", username);
    let response = client.get(url)
        .send()
        .await?
        .text()
        .await?;
    Ok(response)
}
fn data_extractor(body:String)-> Vec<String>{
    let mut data_vec = vec![];
    let document = Html::parse_document(&body);
    let level_selector = Selector::parse("span.type-caption--bold.accent-pill").unwrap();
    if let Some(element5) = document.select(&level_selector).next(){
        let level = element5.text().collect::<Vec<_>>().join(" ");
        data_vec.push(level);
    }
    let winrate_selector = Selector::parse("div.bottom-line.type-caption.shade2 span").unwrap();
    if let Some(element) = document.select(&winrate_selector).next() {
        let winrate = element.text().collect::<Vec<_>>().join(" ");
        data_vec.push(winrate);
    }
    let wins_losses_selector = Selector::parse("div.bottom-line.type-caption.shade2 span:nth-child(2)").unwrap();
    if let Some(element2) = document.select(&wins_losses_selector).next(){
        let wins_losses = element2.text().collect::<Vec<_>>().join(" ");
        data_vec.push(wins_losses);
    }
    let rank_selector = Selector::parse("div.top-line span").unwrap();
    if let Some(element3) = document.select(&rank_selector).next(){
        let rank = element3.text().collect::<Vec<_>>().join(" ");
        data_vec.push(rank);
    }
    let lp_selector = Selector::parse("div.top-line span:nth-child(2)").unwrap();
    if let Some(element4) = document.select(&lp_selector).next(){
        let lp = element4.text().collect::<Vec<_>>().join(" ");
        data_vec.push(lp);
    }
    data_vec
}
