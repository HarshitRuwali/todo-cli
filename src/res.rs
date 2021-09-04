use std::fs;
use std::io::Read;

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
        fs::write("db.txt", content)
    }

    pub fn read(&mut self) -> Result<(), std::io::Error>{
    
        // let mut todo_list = TodoList::new();
        // todo_list.add_to_list("Say Hi to Todo app".to_string());        
        let mut file = fs::File::open("db.txt")?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        

        // let mut todo_list = TodoList::new();
        let mut index:usize = 0;
        for item in content.lines(){

            // println!("{}", item);
            item.to_string();


            let slice = &item[8..]; 
            // let char_val = values.next().expect("err");

            println!("{}", slice);
            // println!("{}", item.to_string.slice(1..6));
            // let mut values = item.split(']');
            // let mark_sym = values.next().expect("err");
            // println!("{}", mark_sym);

            // if mark_sym == "X".to_string(){
            //     TodoList::mark(self, index);
            // }

            index += 1;
            // println!("{}", index);

            
        //     // println!("---");
        //     // println!("{}", content);
        //     // println!("+++");
        //     // let mut values = item.split(' ');
        //     // let mark = values.next().expect("no mark");
        //     // println!("{}", mark);
        //     let mut values = item.split('-');
        //     let task = values.next().expect("no task");
        //     println!("{}", task);
        //     // todo_list = TodoList::new();
        //     // todo_list.mark(index);
        //     // todo_list.add_to_list(task.to_string());

        //     // index += 1;
        //     // let todo_item = TodoList::new();
        }

        Ok(())
    }
}