use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (rads, ids) = parse_config(&args);
    println!("{} Intersections", how_many_intersections((rads, ids)));
}

fn parse_config(args: &[String]) -> (Vec<f32>, Vec<String>){
    // Initialize empty vectors to store radians and identifiers
    let mut rads: Vec<f32> = Vec::new();
    let mut ids: Vec<String> = Vec::new();

    // -a: expecting a full list of radian measurements and identifiers
    if args[1] == String::from("-a") {
        // Stripping the argument of enclosing brackets and comma separators
        let data: Vec<&str> = args[2].split(&['[', '(', ',', ')', ']' ][..]).collect();
        for d in data{
            let d = d.trim(); // Trim surrounding whitespace
            // only looking to process radians and ids, not whitespaces
            if d != String::from(" ") && d != String::from(""){
                // cleaning and processing identifiers
                if d[0..1] == String::from("\""){
                    ids.push(String::from(d.trim_matches('\"')));
                } else{
                    rads.push(d.parse::<f32>().unwrap()); // converting radians to float type
                }
            }
        }
    }
    (rads, ids)
}

fn how_many_intersections(list_of_chords: (Vec<f32>, Vec<String>)) -> i32{
    if list_of_chords.0.len() != list_of_chords.1.len() { // Not accepting lists of different sizes
        -1
    } else if list_of_chords.0.len() == 0 || list_of_chords.1.len() == 0{ // Not accepting empty lists
        -1
    } else{
        count_intersections(connect_chords(&list_of_chords.0, &list_of_chords.1))
    }
}

fn count_intersections(chords: Vec<(&f32, &f32)>) -> i32 {
    // Store for the final count of intersections
    let mut count: i32 = 0;
    // Two pointers to check intersecting chords
    // Once the major chord has checked for intersection of minor chords
    // minor chords do not need to go back and check major chord
    for i in 0..chords.len(){
        for j in i+1..chords.len(){
            if is_intersection(chords[i], chords[j]){ count += 1; }
        }
    }
    // returning the count of intersections
    count
}

// Identifying any intersections with the current "major" chord with the rest of the "minor" chords
fn is_intersection(major: (&f32, &f32), minor: (&f32, &f32)) -> bool{
    // Interpreting the major chord as dividing the circle in half
    // Checking if the points of the minor chord both exists on the same half circle
    if (major.0 < minor.0 && minor.0 < major.1) && (major.0 < minor.1 && minor.1 < major.1){
        false
    } else if (minor.0 < major.0 || minor.0 > major.1) && (minor.1 < major.0 || minor.1 > major.1){
        false
    } else{
        true
    }
}

// Mapping of radian measures and identifiers using a hashmap for quick retrieval and search
fn connect_chords<'a>(point_rads: &'a Vec<f32>, point_ids: &'a Vec<String>) -> Vec<(&'a f32, &'a f32)>{
    // Mapping the identifiers and radian measures of the points into a hashmap
    let points: HashMap<&String, &f32> = point_ids.into_iter().zip(point_rads.into_iter()).collect();
    // Initializing Vector to store tuples representing the chords
    let mut chords: Vec<(&f32, &f32)> = Vec::new();
    // Iterating through the map of points to group the start and end of the chords in a tuple
    for (id, rad) in &points{
        // Only care about finding the start points for each chord
        if id[0..1] == String::from("s") {
            // String concatenation to form key for end point
            let mut end = String::from("e");
            end.push_str(&id[1..2]);
            let end_rad: &f32 = points.get(&end).unwrap();
            // Ordering the tuple so the lower radian measure is first
            if rad > &end_rad{
                chords.push((end_rad, rad));
            } else{
                chords.push((rad, end_rad));
            }
        }
    }
    // returning the vector of chords
    chords
}




// TESTING SUITE
#[test]
fn single_chord(){
    let rads = vec![0.9, 1.7];
    let ids = vec![String::from("s1"),String::from("e1")];
    assert_eq!(how_many_intersections((rads, ids)), 0);
}

#[test]
fn pair_non_intersecting(){
    let rads = vec![0.9, 1.7, 1.3, 2.92];
    let ids = vec![String::from("s1"), String::from("s2"), String::from("e1"), String::from("e2")];
    assert_eq!(how_many_intersections((rads, ids)), 0);
}

#[test]
fn pair_intersecting(){
    let rads = vec![0.78, 1.47, 1.77, 3.92];
    let ids = vec![String::from("s1"), String::from("s2"), String::from("e1"), String::from("e2")];
    assert_eq!(how_many_intersections((rads, ids)), 1);
}

#[test]
fn multiple_non_intersecting(){
    let rads = vec![0.28, 0.73, 1.12, 1.47, 1.84, 2.14, 2.77, 3.92];
    let ids = vec![String::from("s1"), String::from("e1"), String::from("s2"), String::from("e2"), String::from("s3"),
                    String::from("e3"), String::from("s4"), String::from("e4")];
    assert_eq!(how_many_intersections((rads, ids)), 0);
}

#[test]
fn multiple_intersecting(){
    let rads = vec![0.28, 0.73, 1.12, 1.47, 1.84, 2.14, 2.77, 3.92];
    let ids = vec![String::from("s1"), String::from("s2"), String::from("s3"), String::from("s4"), String::from("e1"),
                    String::from("e2"), String::from("e3"), String::from("e4")];
    assert_eq!(how_many_intersections((rads, ids)), 6);
}

#[test]
fn unordered_identifiers(){
    let rads = vec![1.04, 1.25, 1.57, 3.92, 4.18, 4.72, 4.77, 5.49, 5.65, 5.75];
    let ids = vec![String::from("s1"), String::from("s2"), String::from("s3"), String::from("e4"), String::from("e1"),
                    String::from("s4"), String::from("e3"), String::from("s5"), String::from("e2"), String::from("e5")];
    assert_eq!(how_many_intersections((rads, ids)), 4);
}

#[test]
fn empty_radian_measurements(){
    let rads: Vec<f32> = Vec::new();
    let ids = vec![String::from("s1"), String::from("s2"), String::from("e1"), String::from("e2")];
    assert_eq!(how_many_intersections((rads, ids)), -1);
}

#[test]
fn empty_identifiers(){
    let rads = vec![0.9, 1.7, 1.3, 2.92];
    let ids: Vec<String> = Vec::new();
    assert_eq!(how_many_intersections((rads, ids)), -1);
}

#[test]
fn mismatch_radians_and_identifiers(){
    let rads = vec![0.28, 0.73, 1.12, 1.47, 1.84, 2.14];
    let ids = vec![String::from("s1"), String::from("s2"), String::from("s3"), String::from("s4"), String::from("e1"),
                    String::from("e2"), String::from("e3"), String::from("e4")];
    assert_eq!(how_many_intersections((rads, ids)), -1);
}





