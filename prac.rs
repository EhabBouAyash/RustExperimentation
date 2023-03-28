use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

//well use BufRead because its more efficient for smaller amounts of data
fn main() -> std::io::Result<()>{
    let file = File::open("kennedy_copy.txt")?;
    let reader = BufReader::new(file);


    let outputfile = File::create("output.txt")?;
    let mut writer = BufWriter::new(outputfile); /*mut keyword is used  
    variables to be updated*/
    

    for line in reader.lines() {
        let line = line?; /* ? checks if theres errors, and sends them to the stack, 
        then stops the loop. */
        let words:Vec<&str> = line.split_whitespace().collect();
        for x in words{
            if x.to_lowercase()=="of" || x.to_lowercase()=="well"{
                writeln!(writer, "{}", line)?;//we propagate errors here as well. 
                continue; 
            }
        }
    }

    writer.flush()?; //ensures that all the data is written with no errors. 
    Ok(())// Result module returns Ok if there are no erroes and Err otherwise. the () represents an empty tuple.
}