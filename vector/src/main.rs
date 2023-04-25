fn main() {
    let mut vector = Vec::new();
    vector.push(5);
    vector.push(7);
    println!("Vector: {:?}", vector);

    let first_element = vector[0];
    println!("First element: {}", first_element);

    let third_element = vector.get(2);
    match third_element {
        Some(value) => println!("Third element: {}", value),
        None => println!("No third element"),
    }

    println!("---------------------Iterating over vector---------------------");
    for element in vector {
        println!("Element: {}", element);
    }

    // for (let i = 0; i < vector.length; i++) {
    //     console.log(vector[i]);
    // }

    // for i in range:
    //     print(i)

    let mut vector2 = vec![1, 2, 3, 4, 5];
    for i in &mut vector2 {
        *i += 1;
        println!("Element: {}", i);
    }
    
    for i in &vector2 {
        println!("Element: {}", i);
    }

}
