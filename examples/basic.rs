use screen_size::get_primary_screen_size;

fn main() {
    let (w, h) = get_primary_screen_size().expect("Screen size");
    println!("Main screen is {w}x{h} px");
}
