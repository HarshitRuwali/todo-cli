use std::env;
mod res;

enum Command{
    Get,
    Add(String),
    Mark(usize),
    Remove(usize)
}


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
    todo_list.add_to_list("just another test sample".to_string());
    todo_list.mark(0);

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
    // println!("{:#?}", arguments);
}
