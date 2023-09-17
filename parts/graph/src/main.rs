use petgraph::graphmap::DiGraphMap;

#[derive(Clone, Copy)]
struct Oscillator<'a> {
    private_value: [i32; 5],
    id: &'a str,
}

impl<'a> Oscillator<'a> {
    fn new(value: [i32; 5], id: &'a str) -> Self {
        Oscillator {
            private_value: value,
            id,
        }
    }

    fn shared_function(&self) {
        println!("This is shared function of Oscillator");
    }

    fn get_value(&self) -> [i32; 5] {
        self.private_value
    }

    fn get_id(&self) -> &'a str {
        self.id
    }

    fn set_value(&mut self, new_value: [i32; 5]) {
        self.private_value = new_value;
    }
}

#[derive(Clone, Copy)]
struct ADSR<'a> {
    private_value: [i32; 5],
    id: &'a str,
}

impl<'a> ADSR<'a> {
    fn new(value: [i32; 5], id: &'a str) -> Self {
        ADSR {
            private_value: value,
            id,
        }
    }

    fn shared_function(&self) {
        println!("This is shared function of ADSR");
    }

    fn get_value(&self) -> [i32; 5] {
        self.private_value
    }

    fn get_id(&self) -> &'a str {
        self.id
    }


    fn set_value(&mut self, new_value: [i32; 5]) {
        self.private_value = new_value;
    }
}

// Define a trait that specifies the shared function
trait AudioModule {
    fn shared_function(&self);
    fn get_value(&self) -> [i32; 5];
    fn set_value(&mut self, new_value: [i32; 5]);
}

// Implement the trait for your structs
impl<'a> AudioModule for Oscillator<'a> {
    fn shared_function(&self) {
        self.shared_function();
    }

    fn get_value(&self) -> [i32; 5] {
        self.get_value()
    }

    fn set_value(&mut self, new_value: [i32; 5]) {
        self.set_value(new_value);
    }
}

impl<'a> AudioModule for ADSR<'a> {
    fn shared_function(&self) {
        self.shared_function();
    }

    fn get_value(&self) -> [i32; 5] {
        self.get_value()
    }

    fn set_value(&mut self, new_value: [i32; 5]) {
        self.set_value(new_value);
    }
}

fn main() {
    //Create all the existing AudioModules
    let mut g  = DiGraphMap::new();
    //Connect them according to a... json file?
    g.add_edge("", "", 1);
    let id1 = "oscil1";
    let struct1 = Oscillator::new([0; 5], id1);

    let id2 = "adsr1";
    let struct2 = ADSR::new([0; 5], id2);

    let mut structs: Vec<Box<dyn AudioModule>> = vec![Box::new(struct1), Box::new(struct2)];

    for s in structs.iter_mut() {
        s.shared_function();
        println!("Value: {:?}", s.get_value());
        s.set_value([0; 5]);
        s.shared_function();
    }


    println!("Oscillator<'a> ID: {}", struct1.get_id());
    println!("ADSR<'a> ID: {}", struct2.get_id());
}
