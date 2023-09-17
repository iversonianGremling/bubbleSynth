use rand::Rng;

fn main() {
    println!("{:?}", generate_sound());
}
fn generate_sound() -> Vec<f32> {
    let mut result : Vec<f32> = vec![]; //Create empty vector
    let mut rng = rand::thread_rng();

    for _ in 0..10
    {
      let mut processed_sample = rng.gen_range(0.0..1.0);
      processed_sample *= 2.0;
      processed_sample -= 1.0;
      result.push(processed_sample);
    }

    result //Return the vector
}
