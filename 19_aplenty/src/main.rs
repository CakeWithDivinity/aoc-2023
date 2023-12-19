use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Error},
    str::FromStr,
};

#[derive(Debug)]
enum RuleOperation {
    LessThan,
    GreaterThan,
}

#[derive(Debug)]
enum Rule {
    AlwaysTrue(String),
    Conditional {
        attr: PartAttribute,
        operation: RuleOperation,
        value: usize,
        next: String,
    },
}

impl Rule {
    fn apply(&self, part_rating: &PartRating) -> Option<&str> {
        match self {
            Self::AlwaysTrue(next) => Some(next),
            Self::Conditional {
                attr,
                operation,
                value,
                next,
            } => {
                let actual_val = match attr {
                    PartAttribute::X => part_rating.x,
                    PartAttribute::M => part_rating.m,
                    PartAttribute::A => part_rating.a,
                    PartAttribute::S => part_rating.s,
                };

                let applies = match operation {
                    RuleOperation::LessThan => actual_val < *value,
                    RuleOperation::GreaterThan => actual_val > *value,
                };

                if applies {
                    Some(next)
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn apply(&self, part_rating: &PartRating) -> &str {
        let rules = self.rules.iter();

        for rule in rules {
            if let Some(next) = rule.apply(part_rating) {
                return next;
            }
        }

        unreachable!()
    }
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rules = s
            .split(',')
            .map(|r| {
                let Some((condition, result)) = r.split_once(':') else {
                    return Rule::AlwaysTrue(r.to_string());
                };

                let mut cond_chars = condition.chars();

                let attr = match cond_chars.next().expect("attribute is present") {
                    'x' => PartAttribute::X,
                    'm' => PartAttribute::M,
                    'a' => PartAttribute::A,
                    's' => PartAttribute::S,
                    x => panic!("unexpected attr {x}"),
                };

                let operation = match cond_chars.next().expect("condition type") {
                    '<' => RuleOperation::LessThan,
                    '>' => RuleOperation::GreaterThan,
                    x => panic!("unexpected condition type {x}"),
                };

                let value: usize = cond_chars
                    .collect::<String>()
                    .parse()
                    .expect("condition value is number");

                Rule::Conditional {
                    attr,
                    operation,
                    value,
                    next: result.to_string(),
                }
            })
            .collect();

        Ok(Workflow { rules })
    }
}

#[derive(Debug)]
enum PartAttribute {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
struct PartRating {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl FromStr for PartRating {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split(',');

        let x: usize = splits
            .next()
            .unwrap()
            .split_once('=')
            .unwrap()
            .1
            .parse()
            .unwrap();

        let m: usize = splits
            .next()
            .unwrap()
            .split_once('=')
            .unwrap()
            .1
            .parse()
            .unwrap();

        let a: usize = splits
            .next()
            .unwrap()
            .split_once('=')
            .unwrap()
            .1
            .parse()
            .unwrap();

        let s: usize = splits
            .next()
            .unwrap()
            .split_once('=')
            .unwrap()
            .1
            .parse()
            .unwrap();

        Ok(PartRating { x, m, a, s })
    }
}

fn parse_input(reader: BufReader<File>) -> (HashMap<String, Workflow>, Vec<PartRating>) {
    let mut lines = reader.lines();
    let mut workflows: HashMap<String, Workflow> = HashMap::new();

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }

        let (workflow_name, rest) = line.split_once('{').expect("workflow is seperated by {");

        workflows.insert(
            workflow_name.to_string(),
            rest[..rest.len() - 1].parse().unwrap(),
        );
    }

    let part_ratings: Vec<PartRating> = lines
        .map(|l| l.unwrap())
        .map(|l| l[1..l.len() - 1].parse().unwrap())
        .collect();

    (workflows, part_ratings)
}

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);

    let (workflows, part_ratings) = parse_input(reader);

    let result_part_1: usize = part_ratings
        .iter()
        .filter(|part_rating| {
            let mut next_workflow_name = "in";

            while let Some(workflow) = workflows.get(next_workflow_name) {
                match workflow.apply(part_rating) {
                    "A" => return true,
                    "R" => return false,
                    next => next_workflow_name = next,
                };
            }

            unreachable!()
        })
        .map(|part_rating| part_rating.x + part_rating.m + part_rating.a + part_rating.s)
        .sum();

    println!("Result 1: {result_part_1}");

    Ok(())
}
