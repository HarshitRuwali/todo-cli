use std::env;
mod res;

fn main() {
    let arguments : Vec<String> = env::args().collect();
    let command = arguments[1].clone();

    let mut todo_list = res::TodoList::new();

    todo_list.add_to_list("Say Hi to Todo app".to_string());

    if command == "get"{
        todo_list.print_list();
    }else if command == "add"{
        println!("We got a add");
    }

    // println!("{:#?}", arguments);
}
