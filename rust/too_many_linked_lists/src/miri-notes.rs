// This is just a scratchpad of code to remember how rustc and miri interpret the borrow stack

// The borrowchecker doesn't like this:
fn main() {
    let mut data = 10;
    let ref1 = &mut data;
    let ref2 = &mut *ref1;
    
    // ORDER SWAPPED!
    *ref1 += 1;
    *ref2 += 2;

    println!("{}", data);
}

// If we make ref2 a raw pointer:
// rustc compiles without warning (unsafe)
// but miri catches it
fn main() {
    unsafe {
        let mut data = 10;
        let ref1 = &mut data;
        let ptr2 = ref1 as *mut _;

        // ORDER SWAPPED!
        *ref1 += 1;
        *ptr2 += 2;

        println!("{}", data);
    }
}

// Here is a more complicated 'bad' example
// i.e. miri (with MIRIFLAGS="-Zmiri-tag-raw-pointers")
// will catch this error:
fn main() {
    unsafe {
        let mut data = 10;
        let ref1 = &mut data;
        let ptr2 = ref1 as *mut _;
        let ref3 = &mut *ptr2;
        let ptr4 = ref3 as *mut _;

        // Access the first raw pointer first
        *ptr2 += 2;

        // Then access things in "borrow stack" order
        *ptr4 += 4;
        *ref3 += 3;
        *ptr2 += 2;
        *ref1 += 1;

        println!("{}", data);
    }
}

// Finally, an example where we do things right
// Accessing everything in the "borrow stack" order
fn main() {
    unsafe {
        let mut data = 10;
        let ref1 = &mut data;
        let ptr2 = ref1 as *mut _;
        let ref3 = &mut *ptr2;
        let ptr4 = ref3 as *mut _;

        // Access things in "borrow stack" order
        *ptr4 += 4;
        *ref3 += 3;
        *ptr2 += 2;
        *ref1 += 1;

        println!("{}", data);
    }
}

// -- Arrays -------------------------------
// Let's go a little further and test arrays:

// This one doesn't work, since ptr3 is never given mutable borrow access
// b/c ref1_at_0 only borrows the first element
// -- when we then add to the ptr, it hasn't borrowed data[1]
unsafe {
    let mut data = [0; 10];
    let ref1_at_0 = &mut data[0];
    let ptr2_at_0 = ref1_at_0 as *mut i32;
    let ptr3_at_1 = ptr2_at_0.add(1); // <-- here is the issue

    *ptr3_at_1 += 3; // <-- miri will complain here
    *ptr2_at_0 += 2;
    *ref1_at_0 += 1;

    // Should be [3, 3, 0, ...]
    println!("{:?}", &data[..]);
}

// But this does work
unsafe {
    let mut data = [0; 10];
    let ref1_at_0 = &mut data[0];
    let ptr2_at_0 = ref1_at_0 as *mut i32;
    let ptr3_at_0 = ptr2_at_0;
    let ptr4_at_0 = ptr2_at_0.add(0);
    let ptr5_at_0 = ptr3_at_0.add(1).sub(1);

    *ptr3_at_0 += 3;
    *ptr2_at_0 += 2;
    *ptr4_at_0 += 4;
    *ptr5_at_0 += 5;
    *ptr3_at_0 += 3;
    *ptr2_at_0 += 2;
    *ref1_at_0 += 1;

    // Should be [20, 0, 0, ...]
    println!("{:?}", &data[..]);
    
}

// Here, we can't make ref2_at_1 because 'data' has already been mutably borrowed
unsafe {
    let mut data = [0; 10];
    let ref1_at_0 = &mut data[0];
    let ref2_at_1 = &mut data[1];
    let ptr3_at_0 = ref1_at_0 as *mut i32;
    let ptr4_at_1 = ref2_at_1 as *mut i32;

    *ptr4_at_1 += 4;
    *ptr3_at_0 += 3;
    *ref2_at_1 += 2;
    *ref1_at_0 += 1;

    // Should be [4, 6, 0, ...]
    println!("{:?}", &data[..]);
}

// So, we have to use .split_at_mut()
unsafe {
    let mut data = [0; 10];
    
    // Recall that slices are essentially (explicitly?) pointers with a length & size
    let slice1 = &mut data[..];
    let (slice2_at_0, slice3_at_1) = slice1.split_at_mut(1);

    let ref4_at_0 = &mut slice2_at_0[0];
    let ref5_at_1 = &mut slice3_at_1[0];
    let ptr6_at_0 = ref4_at_0 as *mut i32;
    let ptr7_at_1 = ref5_at_1 as *mut i32;

    *ptr7_at_1 += 7;
    *ptr6_at_0 += 6;
    *ref5_at_1 += 5;
    *ref4_at_0 += 4;

    // Should be [10, 12, 0, ...]
    println!("{:?}", &data[..]);
}
// Question: can we still access data after all of this (probably)
// -- but we probably can't access data before all the other refs & ptrs in the borrow stack

// Let's test directly turning a slice into a pointer
unsafe {
    let mut data = [0; 10];

    let slice1_all = &mut data[..];
    let ptr2_all = slic1_all.as_mut_ptr();
    
    let ptr3_at_0 = ptr2_all;
    let ptr4_at_1 = ptr2_all.add(1);
    let ref5_at_0 = &mut *ptr3_at_0;
    let ref6_at_1 &mut *ptr4_at_1;

    *ref6_at_1 += 6;
    *ref5_at_0 += 5;
    *ref4_at_1 += 4;
    *ptr3_at_0 += 3;
    
    // unsafe ptr arithmetic
    for idx in 0..10 {
        *ptr2_all.add(idx) += idx;
    }

    // safe iterator/slice version
    for (idx, elem_ref) in slice1_all.iter_mut().enumerate() {
        *elem_ref += idx;
    }

    // Should be [8, 12, 4, 6, 8, 10, 12, 14, 16, 18]
    println!("{:?}", &data[..]);
}
// This shows that pointers are not only integers
// they have a size and a range of memory associated with them
// (and that we can narrow that range with add?)
//
// More accurately, each pointer/reference has a tag associated to it
// and each location in memory has a borrow stack
// The borrow stacks keep track of the tags of pointers which have access
//
// In this above example, the tag associated with slice1_all
// is on the borrow access stack for all the memory locations of data[..],
// so then .mut_as_ptr() puts ptr2 on the borrow stacks as well
// (rather, I think it's more like there is one borrow stack associated with
// the entirety of data[..] which ptr1 is on, which we then put ptr2 on as well).
//
// This then allows us to do a .add(1) to ptr2, since we're effectively moving
// the ptr4 address to 1 position up, but on the borrow stack side, we're just adding
// ptr4's tag to the stack associated with data[..]
//
// (This is also because arrays are essentially just sequences of memory in Rust?)

// -- Testing Shared References -----------------------------

// Here we are going to test shared read-only references
// We will learn that mutable borrows (and specifically writing to them)
// 'cut-up' the borrow stack at a memory location
// (recall that rustc only allows either one mutable reference or
// multiple read-only/shared references)

// Helper function
fn opaque_read(val: &i32) {
    println!("{}", val);
}

// This is okay:
unsafe {
    let mut data = 10;
    let mref1 = &mut data;
    let ptr2 = mref1 as *mut i32;
    let sref3 = &*mref1;
    let ptr4 = sref3 as *const i32 as *mut i32;

    opaque_read(&*ptr4);
    opaque_read(sref3);
    *ptr2 += 2;
    *mref1 += 1;

    opaque_read(&data);
}

// Here, we pop everything above ptr2
// so, miri gives us an error
unsafe {
    let mut data = 10;
    let mref1 = &mut data;
    let ptr2 = mref1 as *mut i32;
    let sref3 = &*mref1;

    *ptr2 += 2;
    opaque_read(sref3);
    *mref1 += 1;

    opaque_read(&data);
}

// -- Interior Mutability ---------------------------
// TODO: write this portion
// -- Box -------------------------------------------
// TODO: write this portion
