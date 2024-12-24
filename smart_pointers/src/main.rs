use smart_pointers::box_pointer;
use smart_pointers::drop;
use smart_pointers::Rc;
use smart_pointers::Rc_RefCell;
use smart_pointers::RefCycle;
use smart_pointers::RefCycle2;

fn main() {
    println!("********** Box Pointer **********");
    box_pointer::main();
    println!("\n********** Drop **********");
    drop::main();
    println!("\n********** Reference count pointer **********");
    Rc::main();
    println!("\n********** RefCell Pointer **********");
    Rc_RefCell::main();
    println!("\n********** Reference Cycle **********");
    RefCycle::main();
    println!("\n********** Reference Cycle with weak pointer **********");
    RefCycle2::main();
}
