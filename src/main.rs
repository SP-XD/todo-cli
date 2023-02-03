use std::io::Read;
use std::env::args;
use std::collections::HashMap;
use std::str::FromStr;

fn main(){
    let action = args().nth(1).expect("Please specify an action!");

//    println!("{:?}, {:?}", action, item);

    let mut todo= Todo::new().expect("Initialisation of db failed!");

    if action == "add" {
        let item = args().nth(2).expect("Please specify an item!");
        todo.insert(item);
        match todo.save(){
            Ok(_) => println!("Todo saved!"),
            Err(why) => println!("An error occurred: {}", why),
        }
    } else if action == "complete" {
        let item = args().nth(2).expect("Please specify an item!");
        match todo.complete(&item){
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save(){
                Ok(_) => println!("Todo saved!"),
                Err(why)=> println!("An error occurred: {}", why),
            }
        }
        //fix ownership
        //todo.list();
    } else if action == "list"{
        todo.list();
    } else if action == "help"{
        todo.help();
    }
}

struct Todo{
    map: HashMap<String, bool>,
}

impl Todo{
    fn new() -> Result<Todo, std::io::Error> {
        //open the db file
        let file=std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt");
        // read all todo lines into a new string
        let mut content= String::new();
        file?.read_to_string(&mut content)?;

        //allocate an empty Hashmap
        let mut map=HashMap::new();

        //iterate over each lines of the file
        for entries in content.lines(){
            //split and bind values
            let mut values=entries.split('\t');
            let key=values.next().expect("No Key");
            let val=values.next().expect("No Value");

            //insert them into HashMap
            map.insert(String::from(key), bool::from_str(val).unwrap());
        } 
        // return 
        Ok(Todo {map})
    }
}

impl Todo{
    fn insert(&mut self, key: String){
        self.map.insert(key, true);
    }
}

impl Todo{
    fn save(self) -> Result<(), std::io::Error> {
    let mut content = String::new();
    for (k, v) in self.map{
        let record =format!("{}\t{}\n", k, v);
        content.push_str(&record);
    }
    std::fs::write("db.txt", content)
    }
}

impl Todo{
    fn complete(&mut self, key: &String ) -> Option<()>{
        match self.map.get_mut(key){
            Some(v)=> Some(*v=false),
            None=>None
        }    
    }
}

impl Todo{
    fn list(self){
        println!("-------------------------------------------------------------");
        println!("TASKS LIST");
        println!("-------------------------------------------------------------");
        for(key, value) in self.map{
            println!("{} {}", if value {"[ ]".to_string()} else {"[x]".to_string()}, key);
        }
}}

impl Todo{
    fn help(self){
        println!("-------------------------------------------------------------");
        println!("HELP");
        println!("-------------------------------------------------------------");
        println!("Cmd template : cargo run -- $actions [optional]($parameter)");
        println!("-------------------------------------------------------------");
        println!("Actions");
        println!("\t- add $task - To add task");
        println!("\t- complete $task - To mark a task complete");
        println!("\t- list - To list all tasks");
        println!("\t- help - To list the help menu");
    }
}
