struct Laptop{
    brand: String,
    price: u32
}

impl Laptop {
    fn total_three(&self, quantity: u32) -> u32{
        return self.price * quantity;
    }
}

fn main() {

    let quantity:u32 = 3;
    let hp  = Laptop {
       brand : "hp".to_string(),
       price: 650000

    };

    let ibm  = Laptop {
       brand : "ibm".to_string(),
       price: 755000

    };

    let toshiba  = Laptop {
       brand : "Toshiba".to_string(),
       price: 550000

    };

    let dell  = Laptop {
       brand : "dell".to_string(),
       price: 850000
    };

    let brands:[Laptop; 4] = [hp, ibm, toshiba, dell];

    let mut final_cost:u32 = 0;

    for i in brands{
        println!("The total cost for {} is {}", i.brand, i.total_three(quantity));
        final_cost += i.total_three(quantity)
    };

    println!("The final cost is {} for all the laptops.", final_cost );
}