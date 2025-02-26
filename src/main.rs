fn print_elements1(elements: &Vec<String>){
    for element in elements {
        println!("{}", element);
    }
}

fn print_elements2(elements: &Vec<String>){
    elements.iter().for_each(|el|println!("{}", el));
}

fn print_elements3(elements: &Vec<String>){
    elements
        .iter()
        .map(|el|format!("{} {}", el, el))
        .for_each(|el|println!("{}", el));

}

fn main() {
    let colors = vec![String::from("red"), String::from("green"), String::from("blue")];

    print_elements3(&colors);

    // print_elements1(&colors);

    // let mut colors_iter = colors.iter();

    // println!("{:?}", colors_iter.next());
    // println!("{:?}", colors_iter.next());
    // println!("{:?}", colors_iter.next());
    // println!("{:?}", colors_iter.next());
}
