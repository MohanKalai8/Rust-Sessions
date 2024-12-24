# Smart Pointers in Rust

## Box
- **Purpose**: Allows for heap allocation of values.
- **Usage**: Use `Box<T>` when you have a value that needs to be stored on the heap rather than the stack.
- **Example**:
    ```rust
    let b = Box::new(5);
    println!("b = {}", b);
    ```

## Rc (Reference Counted)
- **Purpose**: Enables multiple ownership of data by keeping track of the number of references to the data.
- **Usage**: Use `Rc<T>` when you need to share read-only data between multiple parts of your program.
- **Example**:
    ```rust
    use std::rc::Rc;

    let a = Rc::new(5);
    let b = Rc::clone(&a);
    println!("a = {}, b = {}", a, b);
    ```

## RefCell
- **Purpose**: Allows for interior mutability, enabling mutation of data even when the `RefCell` itself is immutable.
- **Usage**: Use `RefCell<T>` when you need to mutate data inside an immutable structure.
- **Example**:
    ```rust
    use std::cell::RefCell;

    let x = RefCell::new(5);
    *x.borrow_mut() += 1;
    println!("x = {:?}", x.borrow());
    ```

## Memory Leaks and Weak Pointers
- **Memory Leaks**: Occur when references create a cycle, preventing the reference count from ever reaching zero.
- **Prevention**: Use `Weak<T>` to break cycles and prevent memory leaks.
- **Usage**: `Weak<T>` is a non-owning reference that does not increment the reference count.
- **Example**:
    ```rust
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::rc::Weak;

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    pub fn main() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    }
    ```
