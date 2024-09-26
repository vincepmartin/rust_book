fn main() {
    // Let's make a tuple.
    let tup = (1.0, 'x', "tacos");
    println!("tup values: {}, {}, {}", tup.0, tup.1, tup.2);

    /*
     * We can't loop over a tuple, because it is essentially a struct.
     * I'm guessing it's something like
     *
     * struct Tupple {
     *     0: 1.0;
     *     1: 'x';
     *     2: "tacos"
     * }
     *
     * The different types make it impossible to have a value in the loop.
     */

    // Let's make an array.
    let array: [i32; 4] = [1, 2, 3, 4];

    for ele in array {
        println!("{}", ele);
    }

    // Here is a bool.
    let truthy: bool = true;
    println!("truthy: {}", truthy);
    println!("!truthy: {}", !truthy);
}
