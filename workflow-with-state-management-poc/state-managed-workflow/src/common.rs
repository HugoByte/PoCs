#![allow(unused_imports)]
use super::*;
use alloc::task;
use paste::paste;
#[derive(Debug)]
pub struct WorkflowGraph {
    edges: Vec<(usize, usize)>,
    nodes: Vec<Box<dyn Execute>>,
    pub workflow_id: String,
    pub state_manger: StateManager,
}

impl WorkflowGraph {
    pub fn new(size: usize, workflow_id: &str) -> Self {
        WorkflowGraph {
            nodes: Vec::with_capacity(size),
            edges: Vec::new(),
            workflow_id: workflow_id.to_string(),
            state_manger: StateManager::init(),
        }
    }
}

impl WorkflowGraph {
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn add_node(&mut self, task: Box<dyn Execute>) -> usize {
        let len = self.nodes.len();
        self.nodes.push(task);
        len
    }

    pub fn add_edge(&mut self, parent: usize, child: usize) {
        self.edges.push((parent, child));
    }

    pub fn add_edges(&mut self, edges: &[(usize, usize)]) {
        edges
            .iter()
            .for_each(|(source, destination)| self.add_edge(*source, *destination));
    }

    pub fn get_task(&self, index: usize) -> &Box<dyn Execute> {
        self.nodes.get(index).unwrap()
    }

    pub fn get_task_as_mut(&mut self, index: usize) -> &mut Box<dyn Execute> {
        self.nodes.get_mut(index).unwrap()
    }

    pub fn node_indices(&self) -> Vec<usize> {
        (0..self.node_count()).collect::<Vec<_>>()
    }

    // pub fn init(&mut self) -> Result<&mut Self, String> {
    //     match self.get_task_as_mut(0).execute() {
    //         Ok(()) => Ok(self),
    //         Err(err) => Err(err),
    //     }
    // }

    // pub fn term(&mut self, task_index: Option<usize>) -> Result<Value, String> {

    //     match task_index {
    //         Some(index) => {
    //             let mut list = Vec::new();
    //             let edges_list = self.edges.clone();
    //             edges_list.iter().for_each(|(source, destination)| {
    //                 if destination == &index {
    //                     list.push(source)
    //                 }
    //             });
    //             match list.len() {
    //                 0 => {
    //                     let current_task = self.get_task_as_mut(index);
    //                     match current_task.execute() {
    //                         Ok(()) => Ok(current_task.get_task_output()),
    //                         Err(err) => Err(err),
    //                     }
    //                 }
    //                 1 => {
    //                     let previous_task_output = self.get_task(*list[0]).get_task_output();
    //                     let current_task = self.get_task_as_mut(index);
    //                     current_task.set_output_to_task(previous_task_output);
    //                     match current_task.execute() {
    //                         Ok(()) => Ok(current_task.get_task_output()),
    //                         Err(err) => Err(err),
    //                     }
    //                 }
    //                 _ => {
    //                     let res: Vec<Value> = list
    //                         .iter()
    //                         .map(|index| {
    //                             let previous_task = self.get_task(**index);
    //                             let previous_task_output = previous_task.get_task_output();
    //                             previous_task_output
    //                         })
    //                         .collect();

    //                     let s: Value = res.into();
    //                     let current_task = self.get_task_as_mut(index);
    //                     current_task.set_output_to_task(s);

    //                     match current_task.execute() {
    //                         Ok(()) => Ok(current_task.get_task_output()),
    //                         Err(err) => Err(err),
    //                     }
    //                 }
    //             }
    //         }
    //         None => {
    //             let len = self.node_count();
    //             Ok(self.get_task(len - 1).get_task_output())
    //         }
    //     }
    // }

    pub fn pipe(&mut self, task_index: usize) -> Result<&mut Self, String> {
        let len = self.nodes.len() - 1;

        // let redis_cache: RedisCache<String, Value> = RedisCache::new(self.workflow_id.clone(), 60*60)
        // .set_connection_string("redis://127.0.0.1:6379")
        // .set_refresh(true)
        // .set_namespace("workflows")
        // .build()
        // .unwrap();

        let task = self.get_task(task_index);
        // let key = digest(serde_json::to_string(&task.get_json_string()).unwrap());
        let action_name = task.get_action_name();

        self.state_manger.update_running(&action_name, task_index as isize);
        
        // let output = redis_cache.cache_get(&key).unwrap();
        // if output.is_some() {
        //     println!("cache hit for task {}", action_name);
        //     let output = output.unwrap();
        //     let task = self.get_task_as_mut(task_index);
        //     task.set_output_to_task(output);
        // }

        let result = {

            let mut list = Vec::new();
            let edges_list = self.edges.clone();
            edges_list.iter().for_each(|(source, destination)| {
                if destination == &task_index {
                    list.push(source)
                }
            });

            match list.len() {
                0 => {

                    let task = self.get_task_as_mut(task_index);
                        match task.execute() {
                        Ok(()) => Ok(task.get_task_output()),
                        Err(err) => Err(err),
                        }
            },
                1 => {
                    let previous_task_output = self.get_task(*list[0]).get_task_output();
                    let current_task = self.get_task_as_mut(task_index);
                    current_task.set_output_to_task(previous_task_output);
                    match current_task.execute() {
                        Ok(()) => Ok(current_task.get_task_output()),
                        Err(err) => Err(err),
                    }
                }
                _ => {
                    let mut res: Vec<Value> = list
                        .iter()
                        .map(|index| {
                            let previous_task = self.get_task(**index);
                            let previous_task_output = previous_task.get_task_output();
                            previous_task_output
                        })
                        .collect();

                    let s: Value = res.into();
                    let current_task = self.get_task_as_mut(task_index);
                    current_task.set_output_to_task(s);

                    match current_task.execute() {
                        Ok(()) => Ok(current_task.get_task_output()),
                        Err(err) => Err(err),
                    }
                }
            }
        };

        // let result =           if let 3 = task_index {
        //     Err("error in task".to_string())
        // }else{
        //     result
        // };

        match result {
            Ok(output) => {
                self.state_manger.update_success(output);
                Ok(self)
            }
            Err(err) => {
                self.state_manger.update_err(&err);
                Err(err)
            }
        }
    }
}

#[macro_export]
macro_rules! impl_execute_trait {
    ($ ($struct : ty), *) => {

    paste!{$(
    impl Execute for $struct {
            fn execute(&mut self) -> Result<(),String>{
                self.run()
            }

            fn get_task_output(&self) -> Value {
                self.output().clone().into()
            }

            fn set_output_to_task(&mut self, input: Value) {
                self.setter(input)
            }

            fn get_action_name(&self) -> String{
                self.action_name.clone()
            }

            fn get_json_string(&self) -> String{
                serde_json::to_string(&self).unwrap()
            }

        }
    )*}
    };
}

#[allow(dead_code, unused)]
pub fn join_hashmap<T: PartialEq + std::hash::Hash + Eq + Clone, U: Clone, V: Clone>(
    first: HashMap<T, U>,
    second: HashMap<T, V>,
) -> HashMap<T, (U, V)> {
    let mut data: HashMap<T, (U, V)> = HashMap::new();
    for (key, value) in first {
        for (s_key, s_value) in &second {
            if key.clone() == *s_key {
                data.insert(key.clone(), (value.clone(), s_value.clone()));
            }
        }
    }
    data
}

#[no_mangle]
pub unsafe extern "C" fn free_memory(ptr: *mut u8, size: u32, alignment: u32) {
    let layout = Layout::from_size_align_unchecked(size as usize, alignment as usize);
    alloc::alloc::dealloc(ptr, layout);
}

#[link(wasm_import_module = "host")]
extern "C" {
    pub fn set_output(ptr: i32, size: i32);
}

#[link(wasm_import_module = "host")]
extern "C" {
    pub fn set_state(ptr: i32, size: i32);
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Output {
    pub result: Value,
}

#[no_mangle]
pub unsafe extern "C" fn memory_alloc(size: u32, alignment: u32) -> *mut u8 {
    let layout = Layout::from_size_align_unchecked(size as usize, alignment as usize);
    alloc::alloc::alloc(layout)
}
