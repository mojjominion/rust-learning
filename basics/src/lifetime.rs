#[derive(Debug)]
struct Point(i32, i32);

fn right_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
    if p1.0 < p2.0 {
        p2
    } else {
        p1
    }
}

fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
    if p1.0 < p2.0 {
        p1
    } else {
        p2
    }
}

pub(crate) fn life_time() {
    let p1: Point = Point(10, 10);
    let p2: Point = Point(20, 20);
    let p3: &Point = left_most(&p1, &p2);
    let p4: &Point = right_most(&p1, &p2);
    println!("left-most point: {:?}", p3);
    println!("right-most point: {:?}", p4);
}

// pub(crate) fn life_time_different_scope() {
//     let p1: Point = Point(10, 10);
//     let p3: &Point;
//     {
//         let p2: Point = Point(20, 20);
//         p3 = left_most(&p1, &p2);
//     }
//     println!("left-most point: {:?}", p3);
// }
