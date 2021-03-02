use std::collections::HashMap;

// stop in 1h.26M

struct Database {
    //map: std::collections::HashMap<String, String> // If we bring the scope like above in that
    //case here no need full syntax only below is fine
    map: HashMap<String, String>,
}

//Below is methode for above structure
impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        // Read the data from File
        let contents = match std::fs::read_to_string("satya.db") {
            Ok(Readdata) => { Readdata }
            Err(error) => {
                return Err(error);
            }
        }; // If we assign match to varibale at that time semicolon req. Becz now it is a statment. Other wise
        // match is a expression
        //let contents = std::fs::read_to_string("satya.db")?; // This one statement same as all
        //above error checking. GOOD FOR RUST
        // Parse the data
        // Populate out data
        for line in contents.lines() {
            let mut chunk = line.splitn(2, '\t');
            let key: &str = chunk.next().expect("No key");
            let value: &str = chunk.next().expect("No value");
            map.insert(key.to_owned(), value.to_owned());// Inset key and value inside hashmap
            //let v = map.get(key);
            //println!("v = {:?}", v);
        }
        Ok(Database { map: map }) // This new is present inside HashMap and it is just intialization
    }
}

fn main() {
    println!("Welcome To Database Project");
    let mut arguments  = std::env::args().skip(1); // command line arument and skip 1st argument
    //let key: String = arguments.next().unwrap();
    let key: String = arguments.next().expect("key was not passed"); // Insted of unwrap we can use expect. 
                                                    //This expect will give some message at the time of panic
    let value: String = arguments.next().unwrap();
    println!("Key = {} and  value = {}", key, value);
    // Creating key value inside a string using format macro
    // format macro takes varibale number of argument
    let content: String = format!("{}\t{}\n", key, value);
    let write_result = std::fs::write("satya.db", content);// We can use unwrap() also to handle error.
    // Below is pattern matching method
    match write_result {
        Ok(()) => {
            println!("file write success");
        } Err(e) => {
            println!("file write failed");
        }
    }
    let database = Database::new().expect("data base file not present"); // Calling new function present in Database methode
    let v = database.map.get(&key);
    match v {
        Some(v) => println!("key = {} and value = {}", key, v),
        None => println!("Invalid Key name"),
    }
    
}
