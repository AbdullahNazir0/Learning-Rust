// Task 1.
// Create a function that takes an i32 by value and an i32 reference. Modify one of the values and observe the impact on the caller. Explain the ownership and borrowing behavior.
// Experiment with passing different data types (e.g., String, structures) by value and reference, analyzing the changes and understanding the reasons.
fn task_1(mut one: i32, two: &mut i32) {
    one += 2;
    *two += 2;
    println!(
        "Inside Function: val1 {}, Val2 {}.",
        one,
        two,
    );
}

// Task 2.
// Box for Heap Allocation:
// Create a function that allocates a large data structure (e.g., an array) on the heap using Box::new. Access and modify elements of the allocated data. Discuss the memory management implications compared to stack allocation.
// Implement a simple linked list using Box nodes. Consider different ways to handle ownership and deallocation in this scenario.
fn allocate() {
    let my_array: Box<[i32]> = Box::new([5; 5]);
    println!("{:#?}", my_array);
}

#[derive(Debug)]
struct Node {
    data: usize,
    next: Box<Option<Node>>,
}

#[derive(Debug)]
struct List {
    head: Box<Option<Node>>,
}

// Task 3.
// Rc<T> for Shared Ownership:
// Create a struct containing a vector of i32s. Use Rc<Vec<i32>> to share ownership of the vector between multiple functions. Explore how changes in one function affect the value in another.
use std::rc::Rc;
struct SharedVector {
    data: Rc<Vec<i32>>,
}

impl SharedVector {
    fn new(data: Vec<i32>) -> Self {
        SharedVector {
            data: Rc::new(data),
        }
    }

    fn print_data(&self) {
        println!("Data: {:?}", self.data);
    }

    fn update_data(&mut self, new_data: Vec<i32>) {
        self.data = Rc::new(new_data);
    }
}

fn another_function(data: Rc<Vec<i32>>) {
    println!("Inside another function: {:?}", data);
}

// Task 4.
// Custom Smart Pointers:
// Design a smart pointer that provides a reference count and deallocates the underlying data when the count drops to zero. Compare its functionality and usage to standard smart pointers.
// Experiment with implementing features like thread safety, custom deallocation logic (e.g., zeroing memory), or error handling within your custom smart pointer.
use std::ops::Deref;

#[derive(Debug)]
struct CustomSmartPointer {
    reference_count: usize,
    data: i32, // For Simplicity.
}

impl CustomSmartPointer {
    fn new(data: i32) -> Self {
        CustomSmartPointer {
            reference_count: 1,
            data,
        }
    }
}

impl Deref for CustomSmartPointer {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.data
    }
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        self.reference_count -= 1;
        if self.reference_count == 0 {
            // drop();
        }
        println!("Currenct Reference Count: {}", self.reference_count);
    }
}

// Solution: 
use std::cell::Cell;
use std::ptr::Unique;

struct RcRef<T> {
    ptr: Unique<T>,
    count: Cell<usize>,
}

impl<T> RcRef<T> {
    fn new(value: T) -> Self {
        RcRef {
            ptr: Box::new(value),
            count: Cell::new(1),
        }
    }

    fn inc_ref(&self) {
        self.count.set(self.count.get() + 1);
    }

    fn dec_ref(&self) {
        let count = self.count.get() - 1;
        if count == 0 {
            let ptr = self.ptr.as_ptr();
            unsafe {
                Box::from_raw(ptr); // Deallocate using Box
            }
        } else {
            self.count.set(count);
        }
    }

    fn get(&self) -> &T {
        unsafe { &*self.ptr.as_ptr() }
    }
}

impl<T> Drop for RcRef<T> {
    fn drop(&mut self) {
        self.dec_ref();
    }
}


fn main() {
    // Task 1.
    let val1 = 5;
    let mut val2 = 10;
    println!(
        "Before Calling Function: val1 {}, Val2 {}.",
        val1,
        val2,
    );
    task_1(val1, &mut val2);
    println!(
        "After Calling Function: val1 {}, Val2 {}.",
        val1,
        val2,
    );

    // Task 2.
    allocate();

    let node1 = Node { data: 10, next: Box::new(None) };
    let node2 = Node { data: 20, next: Box::new(Some(node1)) };
    let node3 = Node { data: 30, next: Box::new(Some(node2)) };
    let list = List { head: Box::new(Some(node3)) };
    println!("{:#?}", list);

    // Task 3.
    let shared_vector = SharedVector::new(vec![1, 2, 3]);
    shared_vector.print_data();
    let shared_vector_clone = shared_vector.data.clone();
    another_function(shared_vector_clone);
    // shared_vector.update_data(vec![4, 5, 6]);

    // Task 4.
    let my_custom_smart_pointer1 = CustomSmartPointer::new(5);
    println!("my_custom_smart_pointer1: {:#?}", my_custom_smart_pointer1);
    let my_custom_smart_pointer2 = &my_custom_smart_pointer1;
    println!("my_custom_smart_pointer1: {:#?}", my_custom_smart_pointer1);
    println!("my_custom_smart_pointer2: {:#?}", my_custom_smart_pointer2);
      // Solution
    let data = RcRef::new(42);
    {
        let borrowed_data = data.get();
        println!("Borrowed data: {}", borrowed_data);
    } // `data` is automatically decremented here
    println!("Data still exists: {:?}", data); // Prints Optional<RcRef<T>>: Some(...)

}