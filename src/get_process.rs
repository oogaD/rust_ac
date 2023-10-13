use sysinfo::{ProcessExt, System, SystemExt};

pub fn get_proc(processname: String) -> usize{

    let mut system = System::new_all();
    let mut process_id: usize = 0;
    system.refresh_all();
 
    for process in system.processes_by_name(&processname){
    //   println!("{}", process.pid());
       process_id = process.pid().into();
     //  println!("Process: {}, PID: {}", process.name(), process.pid()); //process.root().display());
    }
    if process_id == 0 {
       return process_id;
    }
    return process_id;
 
 }
