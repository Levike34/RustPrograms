use std::fs;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::io::*;

fn main() {
    let files = fs::read_dir("assets").unwrap();
    change file names to lowercase json.
    files.filter_map(Result::ok)
        .filter(|d| d.path().extension() == Some(OsStr::from_bytes(b"Json")))
        .for_each(|f| {
            let mut path = f.path();
            path.set_extension("json");
            fs::rename(f.path(), path);
        }
        );

    //change png file names to numbers
    let files2 = fs::read_dir("assets").unwrap();
    files2.filter_map(Result::ok)
        .filter(|d| d.path().extension() == Some(OsStr::from_bytes(b"png")))
        .for_each(|f| {
            let mut path = f.path();         
            let s = path.into_os_string().into_string().unwrap();
            let w = "NFT_";
            let s2 = s.replace(w, "");
            let new_path = std::path::Path::new(&s2);
            fs::rename(f.path(), new_path);
        }
        );


    // change json file names to numbers
    let files2 = fs::read_dir("assets").unwrap();
    files2.filter_map(Result::ok)
        .filter(|d| d.path().extension() == Some(OsStr::from_bytes(b"json")))
        .for_each(|f| {
            let mut path = f.path();
           
            let s = path.into_os_string().into_string().unwrap();
            let w = "NFT_";
            let s2 = s.replace(w, "");
            let new_path = std::path::Path::new(&s2);
            fs::rename(f.path(), new_path);
        }
        );

    //subtract 1 from all file names json
    let files3 = fs::read_dir("assets").unwrap();
    files3.filter_map(Result::ok)
        .filter(|d| d.path().extension() == Some(OsStr::from_bytes(b"json")))
        .for_each(|f| {
            let mut path = f.path();
           
            let s = path.into_os_string().into_string().unwrap();
            let w = ".json";
            let y = "assets/";
            let s2 = s.replace(w, "");
            let s3 = s2.replace(y, "");

            let mut my_int: i32 = s3.parse().unwrap();
            let x = my_int - 1;
            let new_file_name = format!("assets/{}.json", x.to_string());
            
            let new_path = std::path::Path::new(&new_file_name);
            fs::rename(f.path(), new_path);
            // println!("{}", my_int);
            // let new_path = std::path::Path::new(&s2);
            // fs::rename(f.path(), new_path);
        }
        );

//subtract 1 from all file names png
let files4 = fs::read_dir("assets").unwrap();
files4.filter_map(Result::ok)
    .filter(|d| d.path().extension() == Some(OsStr::from_bytes(b"png")))
    .for_each(|f| {
        let mut path = f.path();
       
        let s = path.into_os_string().into_string().unwrap();
        let w = ".png";
        let y = "assets/";
        let s2 = s.replace(w, "");
        let s3 = s2.replace(y, "");

        let mut my_int: i32 = s3.parse().unwrap();
        let x = my_int - 1;
        let new_file_name = format!("assets/{}.png", x.to_string());
        
        let new_path = std::path::Path::new(&new_file_name);
        fs::rename(f.path(), new_path);
        // println!("{}", my_int);
        // let new_path = std::path::Path::new(&s2);
        // fs::rename(f.path(), new_path);
    }
    );
}
// ts-node ~/metaplex/js/packages/cli/src/candy-machine-v2-cli.ts upload \
//     -e devnet \
//     -k ~/.config/solana/id.json \
//     -cp config.json \
//     -c example \
//     ./assetsts