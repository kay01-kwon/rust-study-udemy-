const STARTING_MISSILES:i32 = 8;
const READY_AMOUNT:i32 = 2;

fn main() {
    let mut missiles = STARTING_MISSILES;
    let ready = READY_AMOUNT;


    println!("Firing {} of my {} missiles..."
    ,ready, missiles);
    
    missiles = missiles - ready;

    println!("{} misiles left",missiles);
    

    // Extra challenges

    let (mut missiles_, ready_): (i32, i32) = 
    (STARTING_MISSILES, READY_AMOUNT);

    println!("Firing {} of my {} missiles..."
    ,ready_, missiles_);
    
    missiles_ = missiles_ - ready_;

    println!("{} misiles left",missiles_);


}