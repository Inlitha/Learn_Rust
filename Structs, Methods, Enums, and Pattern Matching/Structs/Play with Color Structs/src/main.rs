struct ColorClassicStruct {
    name: String,
    hex: String,
}

struct ColorTupleStruct(String, String);

fn classic_c_structs() -> ColorClassicStruct {
    let green = ColorClassicStruct{
        name: "green".to_string(),
        hex: "#00FF00".to_string(),
    };
    return green
}

fn tuple_structs() -> ColorTupleStruct {
    let green = ColorTupleStruct("green".to_string(), "#00FF00".to_string());
    return green
}

fn main() {
    let cl_str = classic_c_structs();
    println!("{}", "Classic Struct:");
    println!("Name: {}", cl_str.name);
    println!("Hex: {}", cl_str.hex);

    let tup_str = tuple_structs();
    println!("{}", "Tuple Struct:");
    println!("Name: {}", tup_str.0);
    println!("Hex: {}", tup_str.1);
}
