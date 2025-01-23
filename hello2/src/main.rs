fn greet_world() {
    println!("Hello, world!");
    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let regions = [southern_germany, japan];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();

    // csv dataset
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 { continue; }
        // dynamic collection, _ to infer type
        let fields: Vec<_> = record
        .split(',')
        .map(|field| field.trim())
        .collect();
        
        //println!("{}", record);
        
        if cfg!(debug_assertions) {
            // conditional compilation based on --release flag
            eprintln!("debug: {:?} -> {:?}",
            record, fields);
        }

        let name = fields[0];
        // if fields[1] parses as f32 then assign it to length
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
}