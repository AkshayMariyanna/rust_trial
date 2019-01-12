use oop::avg::AveragedCollection;

fn main() {
    let mut s = AveragedCollection::new();
    s.add(12);
    s.add(20);

    println!("Average of {:?} is {}", vec![12, 20], s.average());
}
