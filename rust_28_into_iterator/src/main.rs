#[derive(Debug)]
struct Project {
    name: String,
}

#[derive(Debug)]
struct Solution {
    projects: Vec<Project>,
}

impl IntoIterator for Solution {
    type Item = Project;
    type IntoIter = std::vec::IntoIter<Project>; // std doc: struct.Vec.html#impl-IntoIterator

    fn into_iter(self) -> Self::IntoIter {
        self.projects.into_iter()
    }
}

impl<'s> IntoIterator for &'s Solution {
    type Item = &'s Project;
    type IntoIter = std::slice::Iter<'s, Project>; // std doc: struct.Vec.html#method.iter

    fn into_iter(self) -> Self::IntoIter {
        self.projects.iter()
    }
}

impl<'s> IntoIterator for &'s mut Solution {
    type Item = &'s mut Project;
    type IntoIter = std::slice::IterMut<'s, Project>; // std doc: struct.Vec.html#method.iter

    fn into_iter(self) -> Self::IntoIter {
        self.projects.iter_mut()
    }
}

fn main() {
    let d1 = [99, 98, 97, 96, 95, 94, 93, 92, 91];

    // func1 can only take array
    func1(&d1);

    // func2_1 && func2_2 are better: can take array, Vec, iterator
    func2_1(d1);
    func2_2(vec![100, 101, 102, 103, 104, 105, 106]);
    func2_2(std::iter::repeat(999));

    // Now let's implement IntoIterator for Solution
    // Note: 3 ways to iterate
    // move: for item in collection { ... } -> impl IntoIterator for Solution { ... }
    // shared ref: for item in &collection { ... } -> impl IntoIterator<'s> for &'s Solution { ... }
    // mutable ref: for item in &mut collection { ... } -> impl IntoIterator<'s> for &'s mut Solution { ... }

    let p1 = Project {
        name: String::from("a1"),
    };
    let p2 = Project {
        name: String::from("b22"),
    };
    let solution_from_m1 = Solution {
        projects: vec![p1, p2],
    };

    let p3 = Project {
        name: String::from("a2"),
    };
    let p4 = Project {
        name: String::from("b59"),
    };
    let mut solution_from_m2 = Solution {
        projects: vec![p3, p4],
    };

    for project in solution_from_m1 {
        println!("solution for m1, project: {:?}", project);
    }

    // Note: this is not valid, 'solution_from_m1' has been moved in previous for loop
    // println!("solution_for_m1: {:?}", solution_from_m1);

    for project in &solution_from_m2 {
        println!("solution for m2, project: {:?}", project);
    }

    for project in &mut solution_from_m2 {
        project.name.push_str(" +reviewed");
        println!("solution for m2, project: {:?}", project);
    }

    /* for project in &solution_from_m2 {
        println!("solution for m2, project: {:?}", project);
    } */
}

fn func1(data: &[i32]) {
    for i in 0..5 {
        println!("data[{}]: {:?}", i, data.get(i));
    }
}

fn func2_1<C>(data: C)
where
    C: IntoIterator<Item = i32>,
{
    for (i, v) in data.into_iter().enumerate().take(5) {
        println!("data[{}]: {:?}", i, v);
    }
}

fn func2_2<C>(data: C)
where
    C: IntoIterator<Item = i32>,
{
    for (i, v) in (0..5).zip(data.into_iter()) {
        println!("data[{}]: {:?}", i, v);
    }
}
