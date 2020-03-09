use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::fmt;

use clap::{Arg, App, SubCommand};
use chrono::{prelude::*, Local, Duration};
use anyhow::Result;
use anyhow::anyhow;
use ansi_term::Color::Red;

use serde::{self, Deserialize, Serialize};
use dirs::home_dir;


type LocalDT = chrono::DateTime<chrono::Local>; 

#[derive(Debug, Deserialize, Serialize)]
struct TODOItem{
    create_time: LocalDT,
    check_time: Option<LocalDT>,
    content: String,
}

impl fmt::Display for TODOItem{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.content)
    }
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
            File::create(&self.path).expect("create DB file failed");
        } 
        let db_file = File::open(&self.path).expect("read DB file faild");
        let buf = BufReader::new(db_file);
        for line in buf.lines(){
            let item: TODOItem = serde_json::from_str(&line.unwrap())
                .expect("invalid string in DB file");
            self.items.push(item);
        }
    }

    pub fn save(&self){
        let mut file = OpenOptions::new()
                    .write(true)
                    .open(&self.path)
                    .expect("failed to open DB file");
        self.items.iter()
                  .for_each(|x| {
                      serde_json::to_writer(&file, x).unwrap();
                      file.write("\n".as_bytes()).unwrap();
                   });
    }

    pub fn insert(&mut self, item: TODOItem){
        self.items.push(item);
    }

    pub fn show_todo_today(&mut self){
        self.get_todays_todo()
                   .iter()
                   .enumerate()
                   .for_each(|(i, x)| {
                       let status = if x.check_time.is_some() { 
                                        Red.paint("★").to_string() 
                                    } else {
                                        "☆".to_string()
                                    };
                       println!("{0:>5}. {2}  {1}", i+1,x, status);
                    });
    }
    
    pub fn check_todo(&mut self, index: usize) -> Result<()>{
        let mut todos = self.get_todays_todo();
        let item = todos.iter_mut().nth(index);
        match item{
            Some(v) => v.check(),
            None =>  return Err(anyhow!("invalid index")),
        }
        Ok(())
    }

    fn get_todays_todo(&mut self) -> Vec<&mut TODOItem>{
        self.get_todos_in_last_n_days(1)    
    }

    fn get_todos_in_last_n_days(&mut self, n: i64) -> Vec<&mut TODOItem>{
        assert!(n >= 1);
        let today = Local::today()
                    .and_time(NaiveTime::from_hms(0,0,0))
                    .unwrap();
        let n = n-1;
        let duration = Duration::days(-n);
        let start_day = today - duration;

        self.items
            .iter_mut()
            .filter(|x| x.create_time > start_day)
            .collect()
    }
}


fn main() {
    let matchs = App::new("todo-cli")
        .version("0.1")
        .subcommand(
            SubCommand::with_name("add")
            .arg(Arg::with_name("item").required(true))
        )
        .subcommand(
            SubCommand::with_name("check")
            .arg(Arg::with_name("index").required(true))
        )
        .get_matches();
    
    let mut home_dir = home_dir().expect("can't get home dir");
    home_dir.push("db_file");

    let mut db = DB::new(home_dir);
    db.load();

    if let Some(ref matchs) = matchs.subcommand_matches("add"){
        let content = matchs.value_of("item").unwrap();
        let item = TODOItem::new(content.to_owned());
        db.insert(item);
    }

    if let Some(ref matchs) = matchs.subcommand_matches("check"){
        let index = matchs.value_of("index")
                          .unwrap()
                          .parse::<usize>()
                          .unwrap_or_else(|_| panic!("invalid index"));
        let index = index - 1;
        db.check_todo(index).unwrap();
    }
    db.show_todo_today();
    db.save();
}
