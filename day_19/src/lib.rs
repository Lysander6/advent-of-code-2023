use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, bail};

#[derive(Debug, Hash, PartialEq, Eq)]
enum Property {
    X,
    M,
    A,
    S,
}

impl TryFrom<char> for Property {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'x' => Ok(Self::X),
            'm' => Ok(Self::M),
            'a' => Ok(Self::A),
            's' => Ok(Self::S),
            c => bail!("couldn't parse property from {:?}", c),
        }
    }
}

impl From<&Property> for usize {
    fn from(val: &Property) -> Self {
        match val {
            Property::X => 0,
            Property::M => 1,
            Property::A => 2,
            Property::S => 3,
        }
    }
}

impl FromStr for Property {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .next()
            .and_then(|c| c.try_into().ok())
            .ok_or_else(|| anyhow!("couldn't parse property from {:?}", s))
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Relation {
    GT,
    LT,
}

impl TryFrom<char> for Relation {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '>' => Ok(Self::GT),
            '<' => Ok(Self::LT),
            c => bail!("couldn't parse Relation from {:?}", c),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Rule {
    property: Property,
    relation: Relation,
    value: u64,
    target_workflow_name: String,
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (check, target_workflow_name) = s
            .split_once(':')
            .ok_or_else(|| anyhow!("couldn't split rule at ':'"))?;

        let target_workflow_name = target_workflow_name.to_string();

        let contains_greater_than = check.contains(|c| c == '>');

        let (property, value) = if contains_greater_than {
            check.split_once('>')
        } else {
            check.split_once('<')
        }
        .ok_or_else(|| anyhow!("couldn't split at '>' or '<'"))?;

        let property = property
            .chars()
            .nth(0)
            .and_then(|c| c.try_into().ok())
            .ok_or_else(|| anyhow!("couldn't parse Property"))?;

        let relation = if contains_greater_than {
            Relation::GT
        } else {
            Relation::LT
        };

        let value = value.parse()?;

        Ok(Self {
            property,
            relation,
            value,
            target_workflow_name,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    fallback_workflow_name: String,
}

impl FromStr for Workflow {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_terminator(&['{', '}'][..]);

        let name = parts
            .next()
            .ok_or_else(|| anyhow!("missing name"))?
            .to_string();

        let mut rules = parts
            .next()
            .ok_or_else(|| anyhow!("missing rules"))?
            .split(',')
            .rev();

        let fallback_workflow_name = rules
            .next()
            .ok_or_else(|| anyhow!("missing fallback workflow"))?
            .to_string();

        let rules = rules.rev().map(str::parse).collect::<Result<_, _>>()?;

        Ok(Self {
            name,
            rules,
            fallback_workflow_name,
        })
    }
}

#[derive(Debug)]
struct PartRating([u64; 4]);

impl FromStr for PartRating {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ratings = s
            .split_terminator(&['{', '}', ','][..])
            .filter(|p| !p.is_empty())
            .map(|property_value_pair| -> Result<u64, anyhow::Error> {
                let (_, value) = property_value_pair
                    .split_once('=')
                    .ok_or_else(|| anyhow!("couldn't split at '='"))?;

                let value = value.parse::<u64>()?;

                Ok(value)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let ratings: Result<[u64; 4], _> = ratings.try_into();
        let ratings = ratings.unwrap();

        Ok(Self(ratings))
    }
}

#[derive(Debug)]
pub struct Problem {
    workflows: HashMap<String, Workflow>,
    part_ratings: Vec<PartRating>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (workflows, part_ratings) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("couldn't split at double newline"))?;

        let workflows = workflows
            .lines()
            .map(|l| l.parse::<Workflow>().map(|w| (w.name.clone(), w)))
            .collect::<Result<HashMap<String, Workflow>, _>>()?;

        let part_ratings = part_ratings
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<PartRating>, _>>()?;

        Ok(Self {
            workflows,
            part_ratings,
        })
    }
}

fn next_workflow(part: &PartRating, workflow: &Workflow) -> String {
    let Workflow {
        rules,
        fallback_workflow_name,
        ..
    } = workflow;

    for Rule {
        property,
        relation,
        value,
        target_workflow_name,
    } in rules
    {
        let property_idx: usize = property.into();
        let property_value: u64 = part.0[property_idx];

        match relation {
            Relation::GT => {
                if property_value > *value {
                    return target_workflow_name.to_string();
                }
            }
            Relation::LT => {
                if property_value < *value {
                    return target_workflow_name.to_string();
                }
            }
        };
    }

    fallback_workflow_name.to_string()
}

#[derive(Debug, PartialEq, Eq)]
enum EvaluationResult {
    Accepted,
    Rejected,
}

fn evaluate_part(part: &PartRating, workflows: &HashMap<String, Workflow>) -> EvaluationResult {
    let mut current_workflow_name = "in".to_string();

    while current_workflow_name != "A" && current_workflow_name != "R" {
        current_workflow_name = next_workflow(part, &workflows[&current_workflow_name]);
    }

    if current_workflow_name == "A" {
        EvaluationResult::Accepted
    } else {
        EvaluationResult::Rejected
    }
}

#[must_use]
pub fn solve_part_1(p: &Problem) -> u64 {
    let Problem {
        workflows,
        part_ratings,
    } = p;

    part_ratings
        .iter()
        .filter(|&part| evaluate_part(part, workflows) == EvaluationResult::Accepted)
        .map(|part| part.0[0] + part.0[1] + part.0[2] + part.0[3])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_workflow_parsing() {
        let w: Workflow = "px{a<2006:qkq,m>2090:A,rfg}".parse().unwrap();
        assert_eq!(
            w,
            Workflow {
                name: "px".to_string(),
                rules: vec![
                    Rule {
                        property: Property::A,
                        relation: Relation::LT,
                        value: 2006,
                        target_workflow_name: "qkq".to_string(),
                    },
                    Rule {
                        property: Property::M,
                        relation: Relation::GT,
                        value: 2090,
                        target_workflow_name: "A".to_string()
                    },
                ],
                fallback_workflow_name: "rfg".to_string(),
            }
        );
    }

    #[test]
    fn test_solve_part_1() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(solve_part_1(&p), 19114);
    }
}
