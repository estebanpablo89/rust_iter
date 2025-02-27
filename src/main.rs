fn print_elements1(elements: &Vec<String>){
    for element in elements {
        println!("{}", element);
    }
}

fn print_elements2(elements: &Vec<String>){
    elements.iter().for_each(|el|println!("{}", el));
}

fn print_elements3(elements: &[String]){
    elements
        .iter()
        .map(|el|format!("{} {}", el, el))
        .for_each(|el|println!("{}", el));

}

fn shorten_string(elements: &mut [String])  {
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements.iter()
    .map(|el| el.to_uppercase())
    .collect::<Vec<String>>()
}

fn main() {
    let mut colors = vec![String::from("red"), String::from("green"), String::from("blue")];

    print_elements2(&colors);

    shorten_string(&mut colors);

    print_elements2(&colors);

    let upper_colors = to_uppercase(&colors);

    print_elements2(&upper_colors);

    // print_elements1(&colors);

    // let mut colors_iter = colors.iter();

    // println!("{:?}", colors_iter.next());
    // println!("{:?}", colors_iter.next());
    // println!("{:?}", colors_iter.next());
    // println!("{:?}", colors_iter.next());
}
