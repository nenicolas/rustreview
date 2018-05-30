struct Sheep { naked: bool, name: &'static str }
struct Dog { hyper: bool, name: &'static  str}

trait Animal {
    // Static method signature; `Self` refers to the implementor type.
    // This creates a new animal
    fn new_animal(name: &'static str) -> Self;

    // Instance method signatures; these will return a string.
    // Any animal has a name and a noice
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions.
    // Unused? any animal can talk? Maybe
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Dog {
    fn is_hyper(&self) -> bool {
        self.hyper
    }

    fn walk(&mut self){
        if self.is_hyper(){
            println!("{} is tired and does not want to walk", self.name());
        } else {
            println!("{} needs to go for a walk! -- you take {} for a walk.", self.name(), self.name());

            //hyper becomes true therefore, we need to take the dog for a walk
            self.hyper = true;
        }

    }
}

impl Sheep {
    //I dont understand why this function works AND it means naked = false?
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // Implementor methods can use the implementor's trait methods.
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name); //I also don't understand why name is not name() 
                                                        //like the above println! line
                                                        //I checked both syntax works
            self.naked = true;
        }
    }
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    // `Self` is the implementor type: `Sheep`.
    fn new_animal(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false } //A new sheep created will ALWAYS have hair, 
                                        //since it is defaulted to NOT naked
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() { //I thought is_naked() returns a boolean... wtf
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }
    
    // Default trait methods can be overridden.
    fn talk(&self) {
        // For example, we can add some quiet contemplation.
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

impl Animal for Dog {
    fn new_animal(name: &'static str) -> Dog {
        Dog {hyper: false, name: name}
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_hyper() {
            "hmmmmphhh"
        } else {
            "BARK BARK"
        }
    }

    fn talk(&self) {
        println!("{} wiggles tail ... {}", self.name(), self.noise());
    }
}

fn main() {
    // Type annotation is necessary in this case.
    let mut dolly: Sheep = Animal::new_animal("Dolly");
    let mut damian: Dog = Animal::new_animal("Damian");
    // TODO ^ Try removing the type annotations.

    dolly.talk();
    dolly.shear();
    dolly.shear();
    dolly.talk();

    damian.talk();
    damian.walk();
    damian.walk();
    damian.talk();
}