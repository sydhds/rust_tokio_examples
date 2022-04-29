struct EvenNumber {
    curr: u32,
    // next: u32,
}

impl Iterator for EvenNumber {

    type Item = u32; // can refer as Self::Item

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let i = self.curr;
            self.curr += 1;
            if i % 2 == 0 {
                return Some(i);
            }
        }
    }
}

fn even_number_0() -> EvenNumber {
    EvenNumber { curr: 0 }
}

fn even_number(start: Option<u32>) -> EvenNumber {

    // an attempt to provide (optional/default) argument to even_number

    match start {
        Some(x) => EvenNumber { curr: x },
        None => EvenNumber { curr: 0 },
    }
    // EvenNumber { curr: start }
}

fn main() {

    println!("And now using our even number iterator... ;-)");

    for i in even_number(None) {
        println!("i: {}", i);
        if i > 50 {
            break;
        }
    }

    println!("And now start to look for even number starting from 3...");
    for i in even_number(Some(3)) {
        println!("i: {}", i);
        if i > 50 {
            break;
        }
    }

}
