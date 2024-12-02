use std::io;
use std::collections::HashMap;

//construct a struct
struct Contact{
    name: String,
    phone: String,
}

fn main() {
    let mut contacts: HashMap<String, Contact> = HashMap::new();
     
    loop {    
            println!("\nContact Manager");
            println!("1. Add a new contact");
            println!("2. View all contacts");
            println!("3. Search for a contact");
            println!("4. Quit");
            println!("Choose an option: ");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        match choice.trim(){
            "1" => add_contact(&mut contacts),
            "2" => view_contacts(&contacts),
            "3" => search_contact(&contacts),
            "4" => {
                println!("Goodbye!");
                break;

            }

            _ => println!("Invalid option, please try again"),
        }

    }
}

fn add_contact(contacts: &mut HashMap<String, Contact>){
    println!("Enter the name of the contact:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim().to_string();

    println!("Enter the phone number of the contact: ");
    let mut phone = String::new();
    io::stdin().read_line(&mut phone).expect("Failed to read line");
    let phone = phone.trim().to_string();

    let contact = Contact { name: name.clone(), phone };
    contacts.insert(name.clone(), contact);
    println!("Contact added successfully!");

}

fn view_contacts(contacts: &HashMap<String, Contact>){
    if contacts.is_empty(){
        println!("No contacts available");
    }else{
        println!("\nContacts");
        for contact in contacts.values(){
            println!("Name:{}, phone: {}", contact.name, contact.phone);
        }
    }
}

fn search_contact(contacts: &HashMap<String, Contact>){
    println!("Enter the name of the contact to search");
    let mut name =  String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim();

    match contacts.get(name){
        Some(contact) => println!("Found contact name: {}, phone: {}", contact.name, contact.phone),
        None => println!("Contact not found")
    }
}

