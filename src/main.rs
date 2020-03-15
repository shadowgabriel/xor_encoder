mod xor;
pub use crate::xor::xorencoding;

use std::cmp;
use std::path::Path;
use std::ffi::OsString;
use std::env;

fn compare_array(a: &[u8], b: &[u8]) {
    let min = cmp::min(a.len(), b.len());
    for i in 0..min {
        println!("{:#08b} -> {:#08b}", a[i], b[i]);
    }
}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let number_of_args = args.len();
    if number_of_args == 1 {
        println!("Error: No input file specified");
        println!("try running \"xor_encoder -h\" for help");
        std::process::exit(0);
    } else if number_of_args == 2 {
        if args[1] == "-h" {
            println!("");
            println!("xor_encoder <path_to_file> <key>\n");
            println!("\t<path_to_file> should be an existing file");
            println!("\t\t- the encoded file will be created at this location with _encoded in the name");
            println!("\t<key> should be a numeric 8bit value [0..255]");
            println!("\t\t- Note: the value 0 will result in no encoding");
            println!("\tEncoding a file two times with the same key will return the original file");
        } else {
            println!("Error: No key specified");
            println!("try running \"xor_encoder -h\" for help");
        };        
        std::process::exit(0);
    } else if number_of_args > 3 {
        println!("Error: More than two arguments specified");
        println!("try running \"xor_encoder -h\" for help");
        std::process::exit(0);
    }
    
    let key = match args[2].parse::<u8>() {
        Ok(ok)  => ok,
        Err(er) => {
            println!("Error: Invalid key");
            println!("try running \"xor_encoder -h\" for help");
            std::process::exit(0);
        },
    };
    
    let path_in = Path::new(&args[1]);

    if !path_in.is_file() {
        println!("Error: Specified path is not a file");
        println!("try running \"xor_encoder -h\" for help");
        std::process::exit(0);
    }

    let parent = match path_in.parent() {
        Some(x) => x,
        None    => Path::new(""),
    };
    
    let file_stem = path_in.file_stem();

    let file_stem_str = match file_stem {
        Some(x) => match x.to_str() {
            Some(x) => x,
            None    => "",
            },
        None => "",
    };

    let extension = match path_in.extension() {
        Some(x) => match x.to_str() {
            Some(x) => x,
            None    => "",
        },
        None => "",
    };

   

    let file_stem_encode_str = format!("{}_encoded.{}", file_stem_str, extension);
    let file_stem_decoded_str = format!("{}_decoded.{}", file_stem_str, extension);

    
    let path_out = parent.clone().join(file_stem_encode_str);
    let path_dec = parent.clone().join(file_stem_decoded_str);

    xorencoding::encode_file(&path_in, &path_out, key);
    //xorencoding::encode_file(&path_out, &path_dec, key);
}
