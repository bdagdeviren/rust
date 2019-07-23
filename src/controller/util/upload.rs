extern crate reqwest;
extern crate zip;

use std::io::Read;
use reqwest::Client;
use std::error::Error;
use reqwest::header::CONTENT_TYPE;
use std::fs;
use std::io::{self, Write};

const BOUNDARY: &'static str = "------------------------ea3bbcf87c101592";

pub fn upload() -> Result<(), Box<dyn Error>> {
    let vec = un_zip();

    for each in vec.iter()
    {
        let paste_api = "http://10.150.0.247:8081/service/rest/v1/components?repository=deneme1";
        let data = image_data(each.to_string()).unwrap();

        let mut response = Client::new().post(paste_api).header(CONTENT_TYPE, &*format!("multipart/form-data; boundary={}", BOUNDARY)).body(data).send()?;
        let mut response_body = String::new();
        response.read_to_string(&mut response_body)?;
        println!("Your paste is located at: {}", response_body);
        //println!("{:?}", each);
    }  
    
    Ok(())
}

fn un_zip() -> Vec<String> {
    let mut vec = Vec::new();

    let fname = std::path::Path::new("Package.zip");
    let file = fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = file.sanitized_name();
        
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }
        
        if (&*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.as_path().display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            
            println!("File {} extracted to \"{}\" ({} bytes)", i, outpath.as_path().display(), file.size());
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
        vec.push(outpath.as_os_str().to_str().unwrap().to_string());
    }
    return vec;
}

fn image_data(filename: String) -> io::Result<Vec<u8>> {
    let mut data = Vec::new();
    write!(&mut data, "--{}\r\n", BOUNDARY)?;
    write!(&mut data, "Content-Disposition: form-data; name=\"smfile\"; filename=\"11.jpg\"\r\n")?;
    write!(&mut data, "Content-Type: application/x-compressed\r\n")?;
    write!(&mut data, "\r\n")?;

    let mut f = fs::File::open(filename).unwrap();
    f.read_to_end(&mut data)?;

    write!(&mut data, "\r\n")?;
    write!(&mut data, "--{}--\r\n", BOUNDARY)?;

    Ok(data)
}