use std::fs::File;
use std::io::prelude::*;
use std::thread;

fn write_data_to_file(data_chunk: Vec<(i32, i32, i32)>, file_index: usize) {
    let filename = format!("data_{}.csv", file_index);
    let mut file = File::create(filename).expect("Unable to create file");

    for data in data_chunk {
        // Simulate a data line in CSV format
        let line = format!("{},{},{}\n", data.0, data.1, data.2);
        file.write_all(line.as_bytes()).expect("Unable to write data");
    }

    println!("Thread for file {} has finished writing.", file_index);
}

fn generate_data(num_items: i32) -> Vec<(i32, i32, i32)> {
    (0..num_items).map(|i| (i, i * 2, i * 3)).collect()
}

fn main() {
    let total_items = 1000; // Total number of data items to generate
    let items_per_file = 100; // Number of data items per file
    let num_threads = total_items / items_per_file; // Number of threads/files

    // Generate the data
    let data = generate_data(total_items);

    let mut handles = vec![];

    for i in 0..num_threads {
        // Determine the chunk of data for this thread
        let start_index = (i * items_per_file) as usize;
        let end_index = ((i + 1) * items_per_file) as usize;
        let data_chunk = data[start_index..end_index].to_vec();

        // Spawn a thread for each chunk of data
        let handle = thread::spawn(move || {
            write_data_to_file(data_chunk, i as usize);
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All data has been written to files.");
}

use std::fs::File;
use std::io::{self, Write, BufReader, BufRead};

fn merge_files(output_file: &str, input_files: Vec<&str>) -> io::Result<()> {
    let mut outfile = File::create(output_file)?;

    for file_name in input_files {
        let infile = File::open(file_name)?;
        let reader = BufReader::new(infile);

        for line in reader.lines() {
            writeln!(outfile, "{}", line?)?;
        }
        // Optionally, write a newline between files' contents
        writeln!(outfile)?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let input_files = vec!["data_0.csv", "data_1.csv", "data_2.csv"];  // Add your file names here
    let output_file = "merged_data.csv";

    merge_files(output_file, input_files)
}
