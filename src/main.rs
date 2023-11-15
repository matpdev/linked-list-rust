fn main() {
    println!("Hello, world!");
    let mut head = MyList {
        value: 8,
        next: Address::Nil,
    };
    head.append(8);
    head.append(9);
    head.list();
    println!("This size of the list is {}", head.count());
    head.update(0, 6);
    head.list();
}

#[derive(Clone)]
enum Address {
    Address(Box<MyList>),
    Nil,
}

#[derive(Clone)]
struct MyList {
    value: u32,
    next: Address,
}

trait MyListTrait {
    fn append(&mut self, elem: u32);

    fn delete(&mut self, elem: u32);

    fn count(&self) -> u32;

    fn list(&self);

    fn update(&mut self, index: u32, elem: u32);
}

impl MyListTrait for MyList {
    fn append(&mut self, elem: u32) {
        match self.next {
            Address::Address(ref mut next_address) => {
                next_address.append(elem);
            }
            Address::Nil => {
                let node = MyList {
                    value: elem,
                    next: Address::Nil,
                };
                self.next = Address::Address(Box::new(node));
            }
        }
    }

    fn delete(&mut self, elem: u32) {
        match self.next {
            Address::Address(ref mut next_address) => {
                if next_address.value == elem {
                    println!("Deleting value {}", next_address.value);
                    self.next = next_address.next.clone();
                } else {
                    next_address.delete(elem);
                }
            }
            Address::Nil => {
                if self.value == elem {
                    self.value = 0;
                } else {
                    println!("Element {} does not exist in the linked list", elem);
                }
            }
        }
    }

    fn count(&self) -> u32 {
        match self.next {
            Address::Address(ref newaddress) => 1 + newaddress.count(),
            Address::Nil => 0,
        }
    }

    fn list(&self) {
        if self.value == 0 {
            println!("This list is empty");
        } else {
            println!("{}", self.value);
            match self.next {
                Address::Address(ref next_address) => next_address.list(),
                Address::Nil => {}
            }
        }
    }

    fn update(&mut self, index: u32, elem: u32) {
        let mut i = 0;
        let mut j = self;
        if i == index {
            j.value = elem;
        }
        while i < index {
            match j.next {
                Address::Address(ref mut next_address) => {
                    j = next_address;
                }
                Address::Nil => {}
            }
            i = i + 1;
        }
        j.value = elem;
    }
}
