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

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|el| vec_b.push(el));
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|el| el.chars().map(|c| c.to_string()).collect())
        .collect()
}

fn find_color_or(elements: &[String], search: &str , fallback: &str)-> String {
    elements
        .iter()
        .find(|el| el.contains(search))
        .map_or(String::from(fallback), |el| el.to_string())
}

fn main() {
    let mut colors = vec![String::from("red"), String::from("green"), String::from("blue")];

    print_elements2(&colors);

    shorten_string(&mut colors);

    print_elements2(&colors);

    let upper_colors = to_uppercase(&colors);

    print_elements2(&upper_colors);

    let mut colors2 = vec![String::from("red"), String::from("green"), String::from("blue")];
    
    let mut destination = vec![];

    move_elements(colors2, &mut destination);

    println!("{:#?}", destination);

    let mut colors3 = vec![String::from("red"), String::from("green"), String::from("blue")];

    let exploded_colors = explode(&colors3);

    println!("{:#?}", exploded_colors);

    let color = find_color_or(&colors3, "gr", "orange");

    println!("{}", color);

    // let mut colors_iter = colors.iter();

    // println!("{:?}", colors_iter.next());
    // println!("{:?}", colors_iter.next());
    // println!("{:?}", colors_iter.next());
    // println!("{:?}", colors_iter.next());
}
