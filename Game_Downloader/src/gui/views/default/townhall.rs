use std::process::Command;
use std::fs;
use std::io::prelude::*;
use std::str::from_utf8;
use error_chain::error_chain;
use std::fs::File;
use std::path::Path;
use reqwest::header::USER_AGENT;

   
error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

pub fn main(game_to_play: i32) {

    match game_to_play {
        1 => {
            let exists = Path::new(".VoxellandGame").exists();
            let exists2 = Path::new(".VoxellandGame/assets").exists();
            println!("exists?: {}, {}", exists, exists2);
            if !exists {
                download_game(game_to_play);
             
            }
            start_game(game_to_play);
         
        },
        2 => {
            let exists = Path::new(".PixellandGame").exists();
            println!("exists?: {}", exists);
            if !exists {
                download_game(game_to_play);
             
            }
            start_game(game_to_play);
        },
        3 => println!("Star Alliance is not available yet."),
        _ => println!("N/A")
    }

}

#[tokio::main]
async fn download_game(game_to_play: i32) -> Result<()> {
    //github repo must be public
    //get latest .zip file

    // Returns which game to play based on the number
    // 3 = voxelland
    // 4 = pixelland
   match game_to_play {
    1 => {
        let target = "https://github.com/RebornGames/Voxelland/releases/download/0.1.0/Voxelland.zip";

        let client = reqwest::Client::new();
        let res = client
            .get(target)
            .header(USER_AGENT, "My Rust Program 1.0")
            .send()
            .await?;

          
        let path = Path::new("Voxelland.zip");

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}", why),
            Ok(file) => file,
        };
        let content =  res.bytes().await?;
        file.write_all(&content)?;
        let fname = std::path::Path::new("Voxelland.zip");
        let file = fs::File::open(&fname).unwrap();
    
        let mut archive = zip::ZipArchive::new(file).unwrap();
        let target_path = ".VoxellandGame";
        archive.extract(&target_path);
        Ok(())
            
    },
    2 => {
        let target = "https://github.com/RebornGames/Pixelland/releases/download/0.1.0/Pixelland.zip";

        let client = reqwest::Client::new();
        let res = client
            .get(target)
            .header(USER_AGENT, "My Rust Program 1.0")
            .send()
            .await?;

          
        let path = Path::new("Pixelland.zip");

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}", why),
            Ok(file) => file,
        };
        let content =  res.bytes().await?;
        file.write_all(&content)?;
        let fname = std::path::Path::new("Pixelland.zip");
        let file = fs::File::open(&fname).unwrap();
    
        let mut archive = zip::ZipArchive::new(file).unwrap();
        let target_path = ".PixellandGame";
        archive.extract(&target_path);
        Ok(())
    }
    _ => panic!("couldn't create"),
   }
   

        
        // let game_to_run = real_main(".Voxelland.zip");
}
 

fn start_game(game_to_play: i32) {
    match game_to_play {
        1 => {
            Command::new(".VoxellandGame/veloren-voxygen.exe")
            .args(&["run"])
            .output()
            .expect("failed to execute process");
             println!("DONE");
        },
        2 => {
            Command::new(".PixellandGame/IAM.exe")
            .args(&["run"])
            .output()
            .expect("failed to execute process");
             println!("DONE");
        },
        _ => println!("Not available.")
    }


}