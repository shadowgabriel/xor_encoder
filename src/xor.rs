pub mod xorencoding {

    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;
    use std::io::BufReader;
    use std::vec::Vec;

    // xor's every entry in byte with the key
    pub fn encode_array(data: Vec<&u8>, key: u8) -> Vec<u8> {
        let mut out = vec![];
        for data_byte in data {
            out.push(data_byte ^ key);
        }
        out
    }

    /*  - path_in = path to input file */
    pub fn encode_file(path_in: &Path, path_out: &Path, key : u8) {

        
        let file_in = File::open(path_in)
            .expect("can't open");
        
        let meta = match file_in.metadata() {
            Ok(ok)      => ok,
            Err(err)    => {
                println!("File size can't be read");
                std::process::exit(0);
            },
        };
        let file_size = meta.len() as usize;

        let mut reader = BufReader::with_capacity(file_size, file_in);
        reader.fill_buf()
            .expect("Unable to fill buffer");

        let mut data = vec![];

        for data_byte in reader.buffer() {
            data.push(data_byte);
        }
        
        let encoded_data = encode_array(data, key);

        let mut file_out = File::create(path_out)
            .expect("Unable to open");

        file_out.write_all(&encoded_data)
            .expect("Unable to write to file");

    }

}