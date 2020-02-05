// TODO:
// 1. cli: 接收命令
// 2. 定义结构
// 3. 存成本地的 json 数据

use clap::{Arg, App, SubCommand};
use chrono;
use chrono::prelude::*;

type LocalDT = chrono::DateTime<chrono::Local>; 

#[derive(Debug)]
struct TODOItem{
    create_time: LocalDT,
    check_time: Option<LocalDT>,
    content: String,
}

#[allow(dead_code)]
impl TODOItem{
    pub fn new(content: String)-> Self{
        TODOItem{
            create_time: Local::now(),
            check_time: None,
            content, 
        }
    }

    pub fn check(&mut self){
        let now = Local::now();
        self.check_time = Some(now);
    }

}


struct DB{
    path: Box<std::path::Path>,
    items: Option<Vec<TODOItem>>,
}

impl DB {
    pub fn load(&self){
        
    }

    pub fn save(&self){
    
    }
}

fn main() {
    let matchs = App::new("todo-cli")
        .version("0.1")
        .subcommand(
            SubCommand::with_name("add")
            .arg(Arg::with_name("item").required(true))
        )
        .get_matches();
    
    if let Some(ref matchs) = matchs.subcommand_matches("add"){
        let content = matchs.value_of("item").unwrap();
        let item = TODOItem::new(content.to_owned());
        println!("add item: {}", content);
        println!("{:?}", item);
    }

}
