pub mod xorencoding {

    use std::fs::File;
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    use std::path::Path;
    use std::io::BufReader;
    use std::vec::Vec;
    use std::thread;
    use std::time;
    use std::sync::*;

    const BUFFERSIZE : usize = 65536*2*2*2;

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

        if path_out.is_file() {
            std::fs::remove_file(path_out).expect("Error: Could not remove existing encoded file");
        }

        let mut file_out = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path_out)
            .expect("Error: Unable to create encoded file");

        let meta = match file_in.metadata() {
            Ok(ok)      => ok,
            Err(err)    => {
                println!("Error: File size can't be read");
                std::process::exit(0);
            },
        };
        let file_size = meta.len() as usize;
        
        let mut remaining = file_size as u64;
        let mut status : u64 = 100;
        let mut buffsize = BUFFERSIZE;

        if file_size < BUFFERSIZE {
            buffsize = file_size;
        }
        let mut reader = BufReader::with_capacity(buffsize, file_in);

        let bol = Arc::new(AtomicBool::new(false));

        thread::spawn(move || {

            let halfsec = time::Duration::from_millis(500);
            loop {
                
                std::thread::sleep(halfsec);                
            }
        });

        let mut len = reader.fill_buf().expect("Unable to fill buffer").len();
        while len != 0 {
            
            remaining = remaining - (len as u64);
            status = 100 - (remaining * 100) / (file_size as u64);

            tx.send(status).unwrap();

            println!("\tstatus = {} |\tremaining = {}", status, remaining);

            let mut data = vec![];

            for data_byte in reader.buffer() {
                data.push(data_byte);
            }
            
            let encoded_data = encode_array(data, key);

            
            file_out.write_all(&encoded_data)
                .expect("Unable to write to file");
            
            reader.consume(len);
            len = reader.fill_buf().expect("Unable to fill buffer").len();
        }
    }

}