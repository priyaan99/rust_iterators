fn main() {
    iter_dif();
}

fn iter_dif() {
    let mut ls = vec![1, 2, 3];

    // value will not moved to i
    for i in &mut ls {
        // gives reference to element of ls
        // hence i can be used to modify the element
        *i += 10;
    }
    // [ for i in &ls ] == [ for i in ls.iter() ]
    // [ for i in &mut ls ] == [ for i in ls.iter_mut() ]

    // the value is moved to i
    // we cannot use ls after the for loop as value is moved to i's
    for i in ls {
        // give owened value to element of ls
        // hence cannot modify the content
        print!("{i}\n");
    }

    // print!("{}", ls[0]);
    // ^^^^ this produces error as value is moved
}
