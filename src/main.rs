/*
Tasks
  - refactor input into convinience methods (template)
  - add priority enumeration
  - add task data to save stuff

*/

use std::io::Write;
use std::io::prelude::*;


const INLINE_DELIMITER: &'static str = ";;";
const LINE_BREAK_DELIMITER: &'static str = "x\n";

struct Task{
  name:String,
  description:String,
  completed:bool,
}

impl Task{
  fn make_vec_from_cmd() -> Vec<Self>{
    let mut out: Vec<Self> = vec![];
    loop{
      let stdin = std::io::stdin();
      let mut n = String::new();
      let mut des = String::new();
      print!(" - task name: ");
      std::io::stdout().flush().ok();
      stdin.read_line(&mut n).ok();
      print!(" - task description: ");
      std::io::stdout().flush().ok();
      stdin.read_line(&mut des).ok();

      out.push(Task{
        name:n.trim().to_owned(),
        description:des.trim().to_owned(),
        completed:false});

      let mut ans = String::new();
      println!("Add another task? (y/n)");
      stdin.read_line(&mut ans).ok();
      ans = ans.to_lowercase();
      if !(ans == "y" || ans == "yes"){
        break;
      }
    }
    return out;
  }

  fn to_string(&self) -> String{
    return format!("{1}{0}{2}{0}{3}",INLINE_DELIMITER,self.name,self.description,self.completed)
  }
}

struct Project{
  name:String,
  tasks: Vec<Task>,
  priority: u8//Priority,
}
impl Project{
  fn view(list: &[Self]){
    
    if list.len() <= 0 {
      println!("No Projects in list");
      return
    }
    println!("Projects:");
    for i in list{
      println!(" - {}",i.name);
    }
  }

  fn make_from_cmd() -> Self{
    println!("Making a new project");
    let mut n = String::new();
    let mut p = String::new();
    let stdin = std::io::stdin();
    print!(" - name: ");
    std::io::stdout().flush().unwrap();
    stdin.read_line(&mut n).map_err(|err| println!("{}",err)).ok();
    print!(" - priority (0-10): ");
    std::io::stdout().flush().unwrap();
    stdin.read_line(&mut p).map_err(|err| println!("{}",err)).ok();

    let t = Task::make_vec_from_cmd();

    return Project { name: n.trim().to_owned(), tasks: t, priority: p.trim().parse::<u8>().unwrap() }
  }

  fn load(filename: &str)->Vec<Self>{
    let mut out:Vec<Self> = vec![];
    let mut file;
    match std::fs::File::open(&filename){
      Ok(f)=>{file = f},
      Err(_)=>{return out}
    };
    let mut filelines = String::new();
    match file.read_to_string(&mut filelines).map_err(|err| println!("{}",err)){
      Ok(_) => {
        let mut tasks:Vec<Task>=vec![];
        for line in filelines.split(LINE_BREAK_DELIMITER){
          if line.len()<=2 {continue}
          match &line[..2]{
            "t_" => {
              let ln = line[2..].to_owned();
              let part:Vec<&str> = ln.trim().split(INLINE_DELIMITER).collect();
              let name:String = part[0].trim().to_owned();
              let description: String = part[1].trim().to_owned();
              let completed:bool = part[2].trim().parse::<bool>().unwrap();
              tasks.push(Task{name,description,completed})
            }
            "p_" => {       
              let ln = line[2..].to_owned();   
              let part: Vec<&str> = ln.trim().split(INLINE_DELIMITER).collect();
              let name:String = part[0].trim().to_owned();
              let priority: u8 = part[1].trim().parse::<u8>().unwrap();
              out.push(Project {name,tasks,priority});
              tasks = vec![];
            }
            _=>{}
          }

      }},
      Err(_) => {}
    }
    
    return out;
  }
  fn save(list: &[Self],filename:&str){
    let mut filelines = String::new();
    for item in list{
      for task in &item.tasks{
        filelines += "t_";
        filelines += &task.to_string();
        filelines += LINE_BREAK_DELIMITER;
      }
      filelines += "p_";
      filelines += &item.to_string() as &str;
      filelines += LINE_BREAK_DELIMITER;
    }

    let mut file = std::fs::File::create(&filename).unwrap();
    file.write_all(filelines.as_bytes()).ok();

  }
  fn to_string(&self)->String{
    return format!("{1}{0}{2}{0}{3}",INLINE_DELIMITER,self.name,self.tasks.len(),self.priority);
  }
}


fn main() {
  let filename = "Test.proj".to_owned();
  let mut projects:Vec<Project> = Project::load(&filename);
  loop {
    println!("Please select an option:");
    println!(" 0 - save & quit");
    println!(" 1 - view projects");
    println!(" 2 - add new Project");
    //println!(" 3 - load from file");
    let mut line = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut line).map_err(|err| println!("{}",err)).ok();
    if let Ok(answer) = line.trim().parse::<i32>(){
      match answer{
        0 => {
          Project::save(&projects,&filename);
          return
        },
        1 => Project::view(&projects),
        2 => {
          let project = Project::make_from_cmd();
          println!("{}",project.to_string());
          projects.push(project); 
        },
        _ => println!("Command not recognized")
      }

    }else{
      println!("Command not recognized");
    }
   
  }
}
