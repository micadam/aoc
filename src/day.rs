use core::fmt::Debug;

#[derive(Debug)]
pub struct Day {
    name: String,
    parts: Vec<Box<dyn Solveable + Sync>>,
}

impl Day {
    pub fn new(name: String, parts: Vec<Box<dyn Solveable + Sync>>) -> Day {
        Day { name, parts }
    }
}

pub trait Solveable {
    fn solve(&self, lines: &Vec<String>) -> String;
}

impl Debug for dyn Solveable + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Solveable")
    }
}

fn timed_solve(
    solve: &Box<dyn Solveable + Sync>,
    lines: &Vec<String>,
) -> (String, std::time::Duration) {
    let now = std::time::Instant::now();
    let solution = solve.solve(lines);
    let elapsed = now.elapsed();

    (solution, elapsed)
}

impl Day {
    pub fn solve(&self, lines_factory: &dyn Fn(&String) -> Vec<String>, censor: &bool) {
        let lines = lines_factory(&self.name);
        self.parts.iter().enumerate().for_each(|(idx, part)| {
            let (mut ans, time) = timed_solve(part, &lines);
            if *censor {
                ans = "XXX".to_string();
            }
            println!("Part {}: {}, took {} s", idx + 1, ans, time.as_secs_f32());
        });
    }
}
