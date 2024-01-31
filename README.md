# Structify OA

Problem: Counting the number of intersections of a list of chords in a cicle.

## Usage
1) You must have [Rust & Cargo](https://www.rust-lang.org/) installed and up to date.  
```
rustc --version
```
2) Clone repo locally
```
git clone https://github.com/JShand18/structify_oa.git
```
3) Go to the project's root directory
```
cd /my/path/to/structify_oa
```

5) Run `main.rs` with `cargo run` to ensure compiler is wokring, then add `-- -a` flag with the list of radian measurements and identifers
```
cargo run -- -a [(0.78, 1.47, 1.77, 3.92), ("s1", "s2", "e1", "e2")]
```

6) Run tests from test suite in `main.rs`
```
cargo test
```

## Runtime Analysis

Total Estimated Runtime Complexity: O(nlogn)

## Algorithm
Everything needed and used to count the number of instersections is located in the `main.rs` file with the `main` function being the driver of the program.



### 1) Parser – O(n)
The list is parsed from the command line using `parse_config` to seperate the radian measures and identifers into two seperate vectors of float and string values respectifully.

The two vectors are packaged into a tuple and are sent back to the driver.

### 2) Connecting Chords - O(n)
In order to connect the chords by matching to the correct idenitifers `connect_chords` map each radian measure to the corresponding identifer into a hashmap with the identifer as the key.

Then, the function iterates through the hashmap seeking the starting `s_x` identifer for each chord and retrives the ending `e_x` radian measure. The two radian measures are then packaged together in tuple in ascending order.

All of the chords are represented as `(f32, f32)`, a tuple of two float values, then are packaged together in a vector and returned back to the driver.

### 3) Finding Intersections - O(nlogn)
Intersections are determined by thinking of a chord as disecting a cicle into two parts, this chord is the major chord. 

If another chord, a minor chord, were to intersect that means that the start and end points of the minor chord must lie on opposite parts of the two parts of the circle created by the major chord.

This determination is the logic used in `is_intersection`, where the radian measures of each point of the minor chord are compared to the radian measures of the major chord. 

If both points of the minor chord are between or outside of the major chord then the chords do not intersect and the funcion returns `false`. If one point of the minor chord is between the points of the major chord, while the other is outside the chords intersect and function returns `true`.

Once the determination of intersection can be calcuated `count_intersections` loops through all of the vector of chords given as a parameter from the driver and compares each chord.

Each chord's relation with the other chords can be visiualized as symmetric matrix. Therefore is no need to double calculuate whether two chords intersect, as in, chord A(minor) crossing chord B(major) is the same as chord B(minor) crossing chord A(major).

Only the top half of the matrix needs to be checked for intersections, which is accomplished by using two pointers to iterate through the vector.

A running count of the intersections that occur is tracked and returned to the driver.


