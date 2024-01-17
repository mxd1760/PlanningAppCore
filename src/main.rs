/*
Tasks
  - add priority enumeration
  - add options to pick an individual project to project view
  - figure out how to beter handle error cases in get_input 
  - impliment some validation layers across app
  
  - way to edit tasks within projects
  - way to delete projects
  - might want some prioritization for tasks.
  - view learning goals
  - add learning goals

  - completing tasks workflow

  - priority should be visible on higher order lists
  - at the point we're sorting tasks projects can be sorted by priority too

*/

use std::io::Write;
use std::io::prelude::*;
use std::str::FromStr;
use std::collections::HashMap;

// enum Priority{
//   NeedToDoIt,
//   ReallyWantToDoIt,
//   WantToDoIt,
//   WouldLikeItToGetDone,
//   Indifferent,
//   HardToJustify,
//   WillHate,
// }

const INLINE_DELIMITER: &'static str = ";;";
const LINE_BREAK_DELIMITER: &'static str = "x\n";

fn get_input<T: FromStr>(text:&str) -> T{
  print!("{}",text);
  std::io::stdout().flush().ok();
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).ok();
  return input.trim().parse::<T>().ok().unwrap();
}

struct Task{
  name:String,
  description:String,
  completed:bool,
  priority: u8,
  relevant_learning_goal_id:u32,
}
impl Task{
  fn make_vec_from_cmd(data:&mut AppData) -> Vec<Self>{
    let mut out: Vec<Self> = vec![];
    loop{
      out.push(Task{
        name:get_input::<String>(" - task name: "),
        description:get_input::<String>(" - task description: "),
        completed:false,
        relevant_learning_goal_id:data.pick_learning_goal(),
        priority:get_input::<u8>(" - priority (0-10): "),
      });

      let ans = get_input::<String>("Add another task? (y/n): ").to_lowercase();
      if !(ans == "y" || ans == "yes"){
        break;
      }
    }
    return out;
  }

  fn to_string(&self) -> String{
    return format!("{1}{0}{2}{0}{3}{0}{4},{0},{5}",INLINE_DELIMITER,self.name,self.description,self.completed,self.relevant_learning_goal_id,self.priority)
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

  fn make_from_cmd(data:&mut AppData) -> Self{
    println!("Making a new project");
    let name = get_input::<String>(" - name: ");
    let priority = get_input::<u8>(" - priority (0-10): ");

    let tasks = Task::make_vec_from_cmd(data);

    return Project { name, 
      tasks, 
      priority}
  }

  fn to_string(&self)->String{
    return format!("{1}{0}{2}{0}{3}",INLINE_DELIMITER,self.name,self.tasks.len(),self.priority);
  }
}

struct LearningGoal{
  id: u32,
  parent: u32, // do we need more than one?
  goal: String,
  priority:u8,
  tasks_completed: u32,
  //resources: Vec<String>,
}
impl LearningGoal{
  fn to_string(&self) -> String{
    return format!("{1},{0},{2},{0},{3},{0},{4},{0},{5}",INLINE_DELIMITER,self.id,self.parent,self.goal,self.priority,self.tasks_completed);
  }
}

struct AppData{
  filename:String,
  projects:Vec<Project>,
  learning_goals:HashMap<u32,LearningGoal>
}
impl AppData{
  fn load(&mut self){
    let mut file;
    match std::fs::File::open(&self.filename){
      Ok(f)=>{file = f},
      Err(_)=>{todo!()} // TODO better error handling
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
              let priority:u8 = part[3].trim().parse::<u8>().unwrap();
              let relevant_learning_goal_id:u32 = part[4].trim().parse::<u32>().unwrap();
              tasks.push(Task{name,description,completed,priority,relevant_learning_goal_id})
            },
            "p_" => {       
              let ln = line[2..].to_owned();   
              let part: Vec<&str> = ln.trim().split(INLINE_DELIMITER).collect();
              let name:String = part[0].trim().to_owned();
              let priority: u8 = part[1].trim().parse::<u8>().unwrap();
              self.projects.push(Project{name,tasks,priority});
              tasks = vec![];
            },
            "l_" => {
              let ln = line[2..].to_owned();
              let part: Vec<&str> = ln.trim().split(INLINE_DELIMITER).collect();
              let id = part[0].trim().parse::<u32>().unwrap();
              let parent = part[1].trim().parse::<u32>().unwrap();
              let goal = part[2].trim().to_owned();
              let priority = part[3].trim().parse::<u8>().unwrap();
              let tasks_completed = part[4].trim().parse::<u32>().unwrap();
              self.learning_goals.insert(id,LearningGoal{id,parent,goal,priority,tasks_completed});
            }
            _=>{}
          }

      }},
      Err(_) => {}
    }
  }
  fn save(&mut self){
    let mut filelines = String::new();
    for item in &self.projects{
      for task in &item.tasks{
        filelines += "t_";
        filelines += &task.to_string();
        filelines += LINE_BREAK_DELIMITER;
      }
      filelines += "p_";
      filelines += &item.to_string();
      filelines += LINE_BREAK_DELIMITER;
    }
    for (_, item) in &self.learning_goals{
      filelines += "l_";
      filelines += &(item.to_string());
      filelines += LINE_BREAK_DELIMITER;
    }

    let mut file = std::fs::File::create(&self.filename).unwrap();
    file.write_all(filelines.as_bytes()).ok();
  }
  fn pick_learning_goal(&mut self) -> u32{
    println!("Pick a Goal:");
    println!(" 0 - None");
    for (id,goal) in &self.learning_goals{
      println!(" {} - {}",id,goal.goal);
    }
    return get_input::<u32>("");
  }
}

fn main() {
  let filename = "Test.proj".to_owned();
  let mut data:AppData = AppData{filename,projects:vec![],learning_goals:HashMap::new()};
  data.load();
  loop {
    println!("Please select an option:");
    println!(" 0 - save & quit");
    println!(" 1 - view projects");
    println!(" 2 - add new Project");
    println!(" 3 - view learning goals");
    println!(" 4 - add new learning goal");
    println!(" 5 - save");
    let mut line = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut line).map_err(|err| println!("{}",err)).ok();
    if let Ok(answer) = line.trim().parse::<i32>(){
      match answer{
        0 => {
          data.save();
          return
        },
        1 => Project::view(&data.projects),
        2 => {
          let project = Project::make_from_cmd(&mut data);
          println!("{}",project.to_string());
          data.projects.push(project);
          
        },
        3 => {},
        4 => {},
        5 => {
          data.save();
        }
        _ => println!("Command not recognized")
      }

    }else{
      println!("Command not recognized");
    }
   
  }
}
