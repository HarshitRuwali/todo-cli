use std::env;
use std::fs;
use std::io::Read;

mod res;
// mod db;

enum Command{
    Get,
    Add(String),
    Mark(usize),
    Remove(usize)
}


fn read(_:&res::TodoList) -> Result<(), std::io::Error>{

    let mut file = fs::File::open("db.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut index:usize = 0;
    for item in content.lines(){
        
        item.to_string().clone();
        let slice = &item[8..]; 
        res::TodoList::add_to_list(slice.to_string());
        println!("{}", slice);
        index += 1;
        println!("index : {}", index);
    }
    Ok(())
}

// static mut todo_list:res::TodoList = res::TodoList::new();

fn main() {
    let arguments : Vec<String> = env::args().collect();
    let mut todo_list = res::TodoList::new();

    let command = match arguments[1].as_str(){
        "get" => Command::Get,
        "add" => Command::Add(arguments[2].clone()),
        "mark" => Command::Mark(arguments[2].parse().expect("Error converting to int")),
        "remove" => Command::Remove(arguments[2].parse().expect("Error converting to int")),
        _=> panic!("provide a proper argument")
        }; 

    todo_list.add_to_list("Say Hi to Todo app".to_string());
    // todo_list.add_to_list("just another test sample".to_string());
    // todo_list.mark(0);
    
    match read(&todo_list){
        Ok(_) => println!("\n [+] opened"),
        Err(why) => println!("error {}", why)
    }

    // todo_list.read();

        // Ok(_) => println!("\n[+] Todo List Opened\n"),
        // Err(why) => println!("error {}", why)
    // }

    match command {
        Command::Get => todo_list.print_list(),
        Command::Add(task) => {
            todo_list.add_to_list(task);
            todo_list.print_list();
        },
        Command::Mark(index) => {
            todo_list.mark(index);
            todo_list.print_list();
        },
        Command::Remove(index) => {
            todo_list.remove(index);
            todo_list.print_list();
        }
    }
    match todo_list.save(){
        Ok(_) => println!("\n[+] Todo List Saved"),
        Err(why) => println!("error {}", why)
    }
    // println!("{:#?}", arguments);
}
