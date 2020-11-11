extern crate reqwest;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{env, io};

fn clean_href(href: &str) -> String {
    String::from(href)
        .replace(r#"""#, "")
        .replace("href=//", "https://")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    use regex::Regex;
    let re = Regex::new("href=\\S+?\\.pdf").unwrap();
    let link_special = Regex::new("[^\\w\\.]").unwrap();

    let url = &args[1];
    match reqwest::blocking::get(url) {
        Ok(mut resp) => {
            if resp.status() == reqwest::StatusCode::OK {
                print!("success!\n")
            }

            let text = resp.text().expect("failed to extract html text");
            for (i, matched) in re.find_iter(&text).enumerate() {
                let pdf_link = clean_href(matched.as_str());
                let pdf_link = pdf_link.replace("href=", "");
                print!("{}", pdf_link);
                print!("\n");

                match reqwest::blocking::get(&pdf_link) {
                    Ok(mut pdf_rsp) => {
                        if pdf_rsp.status() == reqwest::StatusCode::OK {
                            let path = &*link_special.replace_all(&pdf_link, "");
                            let mut file = match File::create(path) {
                                Err(why) => panic!("couldn't create {} {}", path, why),
                                Ok(file) => file,
                            };

                            file.write_all(
                                &pdf_rsp
                                    .bytes()
                                    .expect(&format!("filed to save pdf: {}", &pdf_link)),
                            );
                        } else {
                            print!("bad status code")
                        }
                    }
                    _ => {
                        panic!(format!("failed to download pdf: {}", &pdf_link))
                    }
                }
            }
        }
        _ => panic!(format!("failed to download main url: {}", url)),
    }
    // let mut out = File::create("rustup-init.sh").expect("failed to create file");
    // io::copy(&mut resp, &mut out).expect("failed to copy content");
}
