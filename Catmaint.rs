use serde::{Serialize, Deserialize};
use std::fs::{File, read_to_string};
use std::io::{self, Write};

#[derive(Serialize, Deserialize)]
struct Book 
{
    name: String,
    category: String,
    cost: f64,
    npc_location: String,
}

fn main() 
{
	CheckFileExists()
}

fn CheckFileExists() 
{
    // Check if catalogue.json exists
    if let Ok(_) = read_to_string("catalogue.json") 
	{
        println!("Catalogue file found.");
    }
	else 
	{
		println!("Catalogue file not found - creating new file.")
		CreateNewCatFile()
	}
}

fn CreateNewCatFile()
{
    // Prompt the user for a pasted block of text
    println!("Please paste the skillbook information from all Dojo hangars (name, category, cost, NPC location):");
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("Failed to read input.");

    // Split the input into separate variables based on commas
    let book_info: Vec<&str> = input_text.trim().split(',').map(|s| s.trim()).collect();

    // Extract variables (assuming specific order: name, category, cost, NPC location)
    let name = book_info.get(0).unwrap_or(&"").to_string();
    let category = book_info.get(1).unwrap_or(&"").to_string();
    let cost: f64 = book_info.get(2).unwrap_or(&"0").parse().unwrap_or(0.0);
    let npc_location = book_info.get(3).unwrap_or(&"").to_string();

    // Create a Book instance
    let book = Book 
    {
        name,
        category,
        cost,
        npc_location,
    };

    // Serialize the Book struct to JSON format
    let serialized = serde_json::to_string(&book).expect("Failed to serialize to JSON.");

    // Save the serialized data to a file called catalogue.json
    let mut file = File::create("catalogue.json").expect("Failed to create file.");
    file.write_all(serialized.as_bytes()).expect("Failed to write to file.");

    println!("Book information saved to catalogue.");
}
