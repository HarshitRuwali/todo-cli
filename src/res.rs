pub struct TodoItem {
    pub name: String,
    pub completed: char
}

impl TodoItem{
    pub fn new(name: String) -> TodoItem{
        return TodoItem{
            name: name,
            completed: ' '
        };
    }
}


pub struct TodoList{
    pub list: Vec<TodoItem>
}

impl TodoList{
    pub fn new() -> TodoList{
        return TodoList{
            list: Vec::new()
        };
    }

    pub fn add_to_list(&mut self, name:String){
        let todo_item = TodoItem::new(name);
        self.list.push(todo_item);
    }

    pub fn print_list(&self){
        for item in &self.list{
            println!("[{}] - {}", item.completed, item.name);
        }
    } 
}