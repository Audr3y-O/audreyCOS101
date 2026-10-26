struct Device {
    brand:String,
    quantity:u32,
    price_each:u32

}

impl Device {
    fn cost(&self, r_n:u32)->u32 {
        self.price_each * r_n
    }
}

fn main() {
    let dev1 = Device {
        brand:String::from("HP"),
        quantity:10,
        price_each:650_000

    };
    
    let dev2 = Device {
        brand:String::from("IBM"),
        quantity:6,
        price_each:755_000

    };

    let dev3 = Device {
        brand:String::from("Toshiba"),
        quantity:10,
        price_each:550_000

    };

    let dev4 = Device {
        brand:String::from("Dell"),
        quantity:4,
        price_each:850_000

    };

    let s = dev1.cost(3) + dev2.cost(3) + dev3.cost(3) + dev4.cost(4);

    println!("Total cost for {} is {} \n Total cost for {} is {} \n Total cost for {} is {} \n Total cost for {} is {} \n Total cost for 4 brands is {}",dev1.brand,dev1.cost(3),dev2.brand,dev2.cost(3),dev3.brand,dev3.cost(3),dev4.brand,dev4.cost(3),s);

}
