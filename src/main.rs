// TODO:
// 1. cli: 接收命令
// 2. 定义结构
// 3. 存成本地的 json 数据
use std::fs;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

use clap::{Arg, App, SubCommand};
use chrono;
use chrono::prelude::*;

use serde::{self, Deserialize, Serialize};
use serde_json;

type LocalDT = chrono::DateTime<chrono::Local>; 

#[derive(Debug, Deserialize, Serialize)]
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


struct DB<T: AsRef<Path>>{
    path: T,
    items: Vec<TODOItem>,
}

impl<T: AsRef<Path>> DB<T> {
    pub fn new(path: T) -> Self{
        DB{
            path,
            items: vec![],
        }
    }

    pub fn load(&mut self){
        if !self.path.as_ref().exists(){
            fs::File::create(&self.path).expect("create DB file failed");
        } 
        let db_file = fs::File::open(&self.path).expect("read DB file faild");
        let buf = BufReader::new(db_file);
        for line in buf.lines(){
            let item: TODOItem = serde_json::from_str(&line.unwrap())
                .expect("invalid string in DB file");
            self.items.push(item);
        }

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
