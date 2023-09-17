use regex::Regex;
use std::collections::HashMap;
fn main() -> (){ // Hacer una funcion para update y otra para eliminar

    let input_existing_modules: &str = "A1 O1 A2 L1 F1";
    let input_number_of_modules = "A:2 L:1 F:1";
    let input_connections_string: &str =
    "A1 >f O1
    A2 >p O1
    L1 >f O2
    O2 > F1
    ";
    let re_connections = Regex::new(r"([A-Z]\d+)\s[>]([a-z]?)\s([A-Z]\d+)").unwrap();
    let mut results = vec![];
    let mut module_ids_string = "".to_string();
    let mut connections: HashMap<&str, Vec<String>> = HashMap::new(); // Initialize the HashMap
    let vector_existing_modules: Vec<&str> = input_existing_modules.split(' ').collect();
    let mut number_of_modules = [0;8];
    for module_id in vector_existing_modules.iter() {
        //println!("Module ID: {}", module_id.chars().nth(0).unwrap());
        //
        let module_type = module_id.chars().nth(0).unwrap();
        match module_type {
            'O' => {
                number_of_modules[0] += 1;
                //modules.insert(module_id, Rc::new(RefCell::new(Oscillator::new())));
            },
            'A' => {
                number_of_modules[1] += 1;
                //modules.insert(module_id, Box::new(ADSR::new()));
            },
            'L' => {
                number_of_modules[2] += 1;
                //modules.insert(module_id, Box::new(LFO::new()));
            },
            //'S' => modules.insert(module_id, AudioModule::Sampler(Sampler::new())),
            'F' => {
                number_of_modules[4] += 1;
                //modules.insert(module_id, Box::new(Filter::new()));
            }
            //'E' => modules.insert(module_id, AudioModule::Effect(Effect::new())),
            //'I' => modules.insert(module_id, AudioModule::Input(Input::new())),
            //'U' => modules.insert(module_id, AudioModule::Output(Output::new())),
            //'B' => modules.insert(module_id, AudioModule::By(By::new())),
             _ => unreachable!()
        };
    }
    // hacer un hashmap con un vector de tamanho
    //el primer str representa el modulo que modifica a AudioModule, el segundo el parametro o si esta vacio implica que es su in


    //OALSFEIUB
    //Go through each right_op and their param(op) and keep adding their parents in a vector on a hashmap<right_op_ID, vec!(!format(op + left_op_ID))>

    for caps in re_connections.captures_iter(input_connections_string) {
        let left_op = caps.get(1).unwrap().as_str();
        let op = caps.get(2).unwrap().as_str();
        let right_op = caps.get(3).unwrap().as_str();

        results.push((left_op, op, right_op));
        module_ids_string = format!("{} {} {}", module_ids_string, left_op, right_op);

        if let Some(vector_of_connections) = connections.get_mut(right_op) {
            vector_of_connections.push(format!("{}{}", op, left_op));
            match right_op {
                "O" => {
                    match op {
                        "f" => {
                        },
                        "p" => {},
                        _ => {}
                    }
                }
                _ => unreachable!()
            }
        } else {
            connections.insert(right_op, vec![format!("{}{}", op, left_op)]);
        }

        println!("{}", right_op.chars().nth(0).unwrap());


    }
    println!("Results: {:?}", results);
    println!("Existing modules: {:?}", vector_existing_modules);
    println!("Connections: {:?}", connections);
}
