use std::io;

fn main() {
    let mut todos = Vec::new();
    let mut input = String::new();
    loop {
        println!("Enter your command: ");
        io::stdin().read_line(&mut input).unwrap();

        let mut words: Vec<&str> = input.split(" ").collect();
        if words[0].trim() == "q" {
            break;
        } else if words[0].trim() == "help" {
            print_help();
        } else if words[0].trim() == "add" {
            add_todo(&mut words, &mut todos);
        } else if words[0].trim() == "print" {
            print_todos(&mut todos);
        } else if words[0].trim() == "delete" {
            delete_todo(words[1], &mut todos);
        } else if words[0].trim() == "update" {
            update_todo(words[1], &mut words, &mut todos)
        } else{
            println!("This action doesnt exist!");
        }

        input.clear();
    }
}

fn print_help() {
    println!("Actions you can perform:");
    println!("1. Add new todo, syntax: add [new_todo]");
    println!("2. Delete todo, syntax: delete [todo_number]");
    println!("3. Update todo, syntax: update [todo_number] [new_todo]");
    println!("4. Print all todos, syntax: print");
    println!("5. Quit the program, syntax: q ");
}

fn add_todo(todo_name: &mut Vec<&str>, todos: &mut Vec<String>) {
    let trimmed_parts: Vec<&str> = todo_name.iter().skip(1).map(|s| s.trim()).collect();
    let todo_to_add = trimmed_parts.join(" ");
    todos.push(todo_to_add.clone());
    println!("Todo {} added!", todo_to_add);
}

fn print_todos(todos: &mut Vec<String>) {
    if todos.len() == 0 {
        println!("You dont have any todos!");
    } else {
        println!("Your Todos:");
        for (index, todo) in todos.iter().enumerate() {
            println!("{}: {}", index + 1, todo.trim());
        }
    }
}

fn delete_todo(todo_index: &str, todos: &mut Vec<String>) {
    match todo_index.trim().parse::<usize>() {
        Ok(index) => {
            if index != 0 && index - 1 < todos.len() {
                todos.remove(index - 1);
                println!("Removed todo!");
            } else {
                println!("Index out of range");
            }
        }
        Err(_) => {
            println!("Failed to parse index");
        }
    }
}

fn update_todo(todo_index: &str, todo_name: &mut Vec<&str>, todos: &mut Vec<String>) {
    match todo_index.trim().parse::<usize>() {
        Ok(index) => {
            if index != 0 && index - 1 < todos.len(){
                let trimmed_parts: Vec<&str> = todo_name.iter().skip(2).map(|s| s.trim()).collect();
                let todo_to_update = trimmed_parts.join(" ");
                todos[index-1] = todo_to_update.clone();
                println!("Todo {} updated!", todo_to_update);
            } else {
                println!("Index out of range");
            }
        }
        Err(_) => {
            println!("Failed to parse index");
        }
    }
}
