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
        for (index, item) in self.list.iter().enumerate(){
            println!("{} [{}] - {}", index, item.completed, item.name);
        }
    } 

    pub fn mark(&mut self, index: usize){
        if self.list[index].completed == ' '{
            self.list[index].completed = 'X';
        }else{
            self.list[index].completed = ' ';
        }
    }

    pub fn remove(&mut self, index: usize){
        self.list.remove(index);
    }

    pub fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (index, item) in self.list.iter().enumerate(){
            let entry = format!("{} [{}] - {}\n", index, item.completed, item.name);
            content.push_str(&entry);
        }
        std::fs::write("db.txt", content)
    }
}