struct Person {
    name: String,
    age: i32,
}

impl Person {

    //Asosciated functions are generally used as constructor
    fn new() -> Person {
        Person{name: "Ejaz".to_string(), age: 29}
    }

    fn display(self) {
        println!("Person name {} and age is {}", self.name, self.age);
    }

    //This method required caller object to be mutable.
    fn display_mutable(&mut self) {
        println!("Mutable person name {} and age is {}", self.name, self.age);
    }
}

fn main(){
    let person = Person::new();
    person.display();

    let mut mutPerson = Person::new();
    mutPerson.display_mutable();
}