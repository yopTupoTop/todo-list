use std::{any::Any, fs::read, io, u32};

#[derive(Debug)]
struct ToDoItem {
    id: u32,
    name: String,
    completed: bool,
}

fn main() {
    let mut todo_list: Vec<ToDoItem> = Vec::new();

    loop {
        println!("What do you want?");
        println!("1. Add new todo");
        println!("2. Complete todo");
        println!("3. Display all todo's");
        println!("4. Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("failed to read");

        let choice = choice.trim().parse::<u32>().expect("invalid input");

        match choice {
            1 => {
                println!("name of todo?");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("failed to read");
                let name = name.trim().to_string();

                let id = todo_list.len() as u32 + 1;

                add_item(id, name, &mut todo_list);
            }

            2 => {
                println!("id of todo?");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("failed to read");
                let id = id.trim().parse::<u32>().expect("invalid input");

                let item = todo_list.iter_mut().find(|i| i.id == id).unwrap();

                complete_item(item);
            }

            3 => {
                display_items(&todo_list);
            }

            4 => {
                println!("goodbye!");
                return;
            }

            _ => {
                println!("invalid choice");
            }
        }
    }
}

fn add_item(_id: u32, _name: String, _todo_list: &mut Vec<ToDoItem>) {
    let item = ToDoItem {
        id: _id,
        name: _name,
        completed: false,
    };

    _todo_list.push(item);
}

fn complete_item(item: &mut ToDoItem) {
    item.completed = true;
}

fn display_items(items: &Vec<ToDoItem>) {
    for item in items {
        println!("{} - {} ({})", item.id, item.name, item.completed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_todo() {
        let mut todo_list: Vec<ToDoItem> = Vec::new();
        add_item(1, "first".to_string(), &mut todo_list);
        assert_eq!(todo_list.len(), 1 as usize);
    }
    #[test]
    fn complete_todo() {
        let mut todo_list: Vec<ToDoItem> = Vec::new();
        add_item(1, "first".to_string(), &mut todo_list);
        let item = todo_list.iter_mut().find(|i| i.id == 1).unwrap();
        complete_item(item);
        assert_eq!(item.completed, true);
    }
}