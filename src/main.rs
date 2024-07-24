use sqlite;
use std::{collections::HashMap, env, vec};

pub fn get_table_names(database_name : String) -> Vec<String> {
    let connection = sqlite::open(database_name).unwrap(); 
    let mut return_vals : Vec<String> = vec![];
   
    connection.iterate("SELECT name FROM sqlite_master WHERE type='table'", |pairs| {
        for &(column, value) in pairs.iter() {
            return_vals.push(value.unwrap().to_string()); 
        }
        true
    }).unwrap();
    
    drop(connection);
    return return_vals;
}    

pub fn get_table(database_name : String, table_name : String) -> Vec<Vec<String>> {
    let connection = sqlite::open(database_name).unwrap(); 
    let mut data_enteries : Vec<Vec<String>> = vec![];     
    
    connection.iterate("SELECT * FROM ".to_string() + table_name.as_str(), |pairs| {
        let mut data : Vec<String> = vec![];
        for &(column, value) in pairs.iter() {
            data.push(value.unwrap().to_string()); 
        }
        data_enteries.push(data);
        true
    }).unwrap();
        
    drop(connection);
    return data_enteries;
}

pub fn print_data(data : HashMap<&str, Vec<Vec<String>>>) {
    
    let mut keys = data.keys();    
    
    for val in keys {
        println!("{}", val);
        let meta_data = data.get(val.to_owned()).unwrap();
        
        for i in meta_data {
            println!("     {:?}", i);
        } 

        println!("");
    }
    
}

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() == 2 {
        let database = args.get(1).unwrap();
        
        let names = get_table_names(database.to_string()); 
        
        let mut data : HashMap<&str, Vec<Vec<String>>> = HashMap::new();
        
        for i in 0..names.len() {
            data.insert(names.get(i).unwrap(), get_table(database.to_string(), names.get(i).unwrap().to_string())); 
        }
        
        print_data(data);   
        return;
    }     
    
    if args.contains(&"help".to_string()) {
        println!("Show Full DB:");
        println!("     ./db_visual filename.db");
        println!("");
        println!("Show Single Table");
        println!("     ./db_visual filename.db table_name");
    
        return;
    }

    if args.len() == 3 {
        let database = args.get(1).unwrap();
        let table_name = args.get(2).unwrap();
        
        let mut data : HashMap<&str, Vec<Vec<String>>> = HashMap::new();
        
        data.insert(table_name.as_str(), get_table(database.to_owned(), table_name.to_owned()));

        print_data(data);

        return;
    }

    println!("Show Full DB:");
    println!("     ./db_visual filename.db");
    println!("");
    println!("Show Single Table");
    println!("     ./db_visual filename.db table_name");

}

