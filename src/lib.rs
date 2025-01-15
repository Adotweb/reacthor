use type_lib::{ValueType, Value, ThorLangError, stringify_value, Environment, Overloadings};

use execution_lib::eval_function;

use std::collections::HashMap;

use std::sync::{Arc, Mutex, OnceLock, RwLock, atomic::AtomicBool};

use std::thread::{spawn, JoinHandle};

static THREAD_MANAGER : OnceLock<Arc<Mutex<ThreadManager>>> = OnceLock::new();

struct ThreadManager{
    threads : HashMap<usize, JoinHandle<()>>,
    count : usize
}

impl ThreadManager{
    fn new() -> Self{
        Self{
            threads : HashMap::new(),
            count : 0
        } 
    }

    fn add_job(&mut self, job : JoinHandle<()>){
        self.threads.insert(self.count, job);
        self.count += 1;
    }
}


#[no_mangle]
pub extern "Rust" fn start_thread(arguments : HashMap<String, Value>, enclosing : Arc<Mutex<Environment>>, overloadings : &mut Overloadings) -> Value{

    let mut job_id = 0; 

    let mut tm_lock = THREAD_MANAGER.get().unwrap().lock().unwrap();

    job_id = tm_lock.count;

    let func = arguments.get("func").unwrap().clone();

    if let ValueType::Function(_) = func.value.clone(){
       
        let mut easy_overloadings = overloadings.clone();

        let handle = spawn(move ||{
            let _ = eval_function(func.clone(), Vec::new(), enclosing.clone(), &mut easy_overloadings);
        });


        tm_lock.add_job(handle);
    }


    Value::object(HashMap::from([
        ("stop", Value::lib_function("stop", vec![], Some(Box::new(Value::number(job_id as f64)))))
    ])) 
}

#[no_mangle]
pub extern "Rust" fn stop(arguments : HashMap<String, Value>) -> Value{

    let mut tm_lock = THREAD_MANAGER.get().unwrap().lock().unwrap();

    let job_id = arguments.get("self_value").unwrap().to_f64().unwrap() as usize;

    let job = tm_lock.threads.get(&job_id).unwrap();

    let _ = job;

    tm_lock.threads.remove(&job_id);
    Value::nil()
}


#[no_mangle]
pub extern "Rust" fn value_map() -> HashMap<String, Value>{
    let mut map = HashMap::new();

    THREAD_MANAGER.get_or_init(||{
        Arc::new(Mutex::new(ThreadManager::new()))
    });

    Value::mut_lib_function("start_thread", vec!["func"], None).insert_to(&mut map);


    map
}
