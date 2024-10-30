fn main() {
    let mut s = "initial string".to_string();
    // can concat strings with the + operator
    println!("{s}");
    s += " is now longer!";
    println!("{s}");
    s.push('!'); // adds a single character
    println!("{s}");
    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{s3}");
    // I don't really care about strings... skipping this!
}
