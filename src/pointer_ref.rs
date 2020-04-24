/**
 * @author Dominik Dorfstetter
 * 
 * Pointers and references
 * Reference Pointers -> Point to a resource in memory
 * 
 **/
 pub fn run() {
     // Primitive array
     let arr1 = [1, 2, 3];
     let arr2 = arr1;

     // With non-primitives, if you assign another variable to a piece of data, the first
     // Variable will no longer hold that value. You'll need to use a reference (&) to point to the ressource.
     // Non-Primitive array (Vector)
     let arr3 = vec![1, 2, 3];
     let arr4 = &arr3; // if you aren't using (&) it will generate an error

     println!("{:?}", (arr1, arr2, &arr3, arr4));


 }

