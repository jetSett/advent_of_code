#![feature(iter_map_while)]
#![feature(map_first_last)]
use std::{collections::BTreeSet, collections::HashMap, io::BufRead, ops::RangeInclusive};

type Int = u64;

#[derive(Debug, PartialEq)]
pub struct Field(String, RangeInclusive<Int>, RangeInclusive<Int>);

impl Field {
    fn contains(&self, x: &Int) -> bool {
        self.1.contains(&x) || self.2.contains(&x)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ticket(Vec<Int>);

#[derive(Debug, PartialEq)]
struct ProblemData {
    fields: Vec<Field>,
    my_ticket: Ticket,
    all_tickets: Vec<Ticket>,

    number_fields: usize,
}

impl ProblemData {
    fn from_strings(lines: &[String]) -> anyhow::Result<Self> {
        let mut fields = vec![];
        let mut i = 0;
        while let Ok(field) = ticket_parser::rule_field(&lines[i]) {
            fields.push(field);
            i += 1;
        }
        i += 2;
        let my_ticket = ticket_parser::ticket(&lines[i])?;
        i += 3;
        let mut nearby_tickets = vec![];

        while i < lines.len() {
            if let Ok(ticket) = ticket_parser::ticket(&lines[i]) {
                nearby_tickets.push(ticket);
            } else {
                break;
            }
            i += 1;
        }
        Ok(ProblemData {
            number_fields: fields.len(),
            fields,
            my_ticket,
            all_tickets: nearby_tickets,
        })
    }

    fn error_rate_ticket(&self, ticket: &Ticket) -> Int {
        let mut error_rate = 0;

        for value in &ticket.0 {
            if self
                .fields
                .iter()
                .filter(|field| field.contains(value))
                .count()
                == 0
            {
                error_rate += value;
            }
        }
        error_rate
    }

    fn scanning_error_rate(&self) -> Int {
        self.all_tickets
            .iter()
            .map(|ticket| self.error_rate_ticket(ticket))
            .sum()
    }

    fn remove_error_tickets(&mut self) {
        self.all_tickets = self
            .all_tickets
            .iter()
            .filter(|ticket| self.error_rate_ticket(ticket) == 0)
            .cloned()
            .collect();
    }

    fn id_possible_for_field(&self, field: &Field, id: usize) -> bool {
        self.all_tickets
            .iter()
            .filter(|ticket| !field.contains(&ticket.0[id]))
            .count()
            == 0
    }

    fn determine_fields_values(self) -> HashMap<String, Int> {
        // Vec<field_id> -> <my_field_ticket>
        let mut possible_fields = Vec::new();

        for field_id in 0..self.number_fields {
            possible_fields.push(Vec::new());
            for (i, field) in self.fields.iter().enumerate() {
                if self.id_possible_for_field(field, field_id) {
                    possible_fields[field_id].push(i);
                }
            }
        }

        let mut fixed_fields = BTreeSet::new();

        let mut modification = true;

        let mut association = HashMap::new();

        while modification {
            // println!("{:?}", possible_fields);

            modification = false;
            for (field_id, ticket_field_id) in possible_fields
                .iter()
                .enumerate()
                .filter(|(_, possible_ids)| possible_ids.len() == 1)
                .map(|x| (x.0, x.1[0]))
            {
                // println!("Fixed {}-{}", ticket_field_id, field_id);
                fixed_fields.insert(ticket_field_id);
                association.insert(ticket_field_id, field_id);
            }
            for i in 0..possible_fields.len() {
                let new_possible: Vec<usize> = possible_fields[i]
                    .iter()
                    .cloned()
                    .filter(|x| !fixed_fields.contains(x))
                    .collect();
                if new_possible.len() != possible_fields[i].len() {
                    modification = true;
                    possible_fields[i] = new_possible;
                }
            }
        }

        // println!("{:?}", association);
        let mut final_repartition = HashMap::new();
        for (field_id, associated_id) in association {
            final_repartition.insert(
                self.fields[field_id].0.clone(),
                self.my_ticket.0[associated_id],
            );
        }
        // final_repartition.insert(, v)
        final_repartition
    }
}

peg::parser! {
    grammar ticket_parser() for str {
        rule number() -> Int
            = n:$(['0'..='9']+) { n.parse().unwrap() }
        rule range() -> RangeInclusive<Int>
            = a:number() "-" b:number() {a..=b}
        pub rule rule_field() -> Field
            = name:$(['a'..='z' | ' ']+)": " r1:range() " or " r2:range() {Field(name.into(), r1, r2)}
        pub rule ticket() -> Ticket
            = l:(number()**",") {Ticket(l)}
    }
}

fn main() -> anyhow::Result<()> {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    let mut problem_data = ProblemData::from_strings(&lines)?;
    println!("{}", problem_data.scanning_error_rate());
    problem_data.remove_error_tickets();
    let fields = problem_data.determine_fields_values();
    let score: Int = fields
        .iter()
        .filter(|(name, _)| name.starts_with("departure"))
        .map(|x| x.1)
        .product();
    println!("{}", score);
    Ok(())
}

#[test]
fn test_parser() {
    assert_eq!(
        ticket_parser::rule_field("departure location: 30-828 or 839-971").unwrap(),
        Field("departure location".into(), 30..=828, 839..=971)
    );
    assert_eq!(
        ticket_parser::ticket("7,3,47").unwrap(),
        Ticket(vec![7, 3, 47])
    );
}
#[test]
fn test_parser_problem_data() -> anyhow::Result<()> {
    assert_eq!(
        ProblemData {
            my_ticket: Ticket(vec![7, 1, 14]),
            fields: vec![
                Field("class".into(), 1..=3, 5..=7),
                Field("row".into(), 6..=11, 33..=44),
                Field("seat".into(), 13..=40, 45..=50),
            ],
            all_tickets: vec![
                Ticket(vec![7, 3, 47]),
                Ticket(vec![40, 4, 50]),
                Ticket(vec![55, 2, 20]),
                Ticket(vec![38, 6, 12])
            ],
            number_fields: 3
        },
        ProblemData::from_strings(&[
            "class: 1-3 or 5-7".into(),
            "row: 6-11 or 33-44".into(),
            "seat: 13-40 or 45-50".into(),
            "".into(),
            "your ticket:".into(),
            "7,1,14".into(),
            "".into(),
            "nearby tickets:".into(),
            "7,3,47".into(),
            "40,4,50".into(),
            "55,2,20".into(),
            "38,6,12".into(),
        ])?
    );
    Ok(())
}

#[test]
fn test_exo1() -> anyhow::Result<()> {
    let data = ProblemData::from_strings(&[
        "class: 1-3 or 5-7".into(),
        "row: 6-11 or 33-44".into(),
        "seat: 13-40 or 45-50".into(),
        "".into(),
        "your ticket:".into(),
        "7,1,14".into(),
        "".into(),
        "nearby tickets:".into(),
        "7,3,47".into(),
        "40,4,50".into(),
        "55,2,20".into(),
        "38,6,12".into(),
    ])?;

    assert_eq!(data.scanning_error_rate(), 71);

    Ok(())
}
#[test]
fn test_determine_fields_possible() -> anyhow::Result<()> {
    let mut data = ProblemData::from_strings(&[
        "class: 0-1 or 4-19".into(),
        "row: 0-5 or 8-19".into(),
        "seat: 0-13 or 16-19".into(),
        "".into(),
        "your ticket:".into(),
        "11,12,13".into(),
        "".into(),
        "nearby tickets:".into(),
        "3,9,18".into(),
        "15,1,5".into(),
        "5,14,9".into(),
    ])?;

    data.remove_error_tickets();
    dbg!(&data);
    let fields = data.determine_fields_values();
    dbg!(&fields);

    let class = &fields[&"class".to_string()];
    let row = &fields[&"row".to_string()];
    let seat = &fields[&"seat".to_string()];

    let mut results = [*class, *row, *seat];
    results.sort_unstable();

    assert_eq!(results, [11, 12, 13]);

    assert!((0..=1).contains(class) || (4..=19).contains(class));
    assert!((0..=5).contains(row) || (8..=19).contains(row));
    assert!((0..=13).contains(seat) || (16..=19).contains(seat));

    Ok(())
}
#[test]
fn test_determine_fields() -> anyhow::Result<()> {
    let mut data = ProblemData::from_strings(&[
        "class: 0-1 or 4-19".into(),
        "row: 0-5 or 8-19".into(),
        "seat: 0-13 or 16-19".into(),
        "".into(),
        "your ticket:".into(),
        "11,12,13".into(),
        "".into(),
        "nearby tickets:".into(),
        "3,9,18".into(),
        "15,1,5".into(),
        "5,14,9".into(),
    ])?;

    data.remove_error_tickets();
    let fields = data.determine_fields_values();

    assert_eq!(fields.get("class"), Some(&12));
    assert_eq!(fields.get("row"), Some(&11));
    assert_eq!(fields.get("seat"), Some(&13));

    Ok(())
}
