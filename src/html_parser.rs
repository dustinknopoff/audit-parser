use crate::constants::{
    abbreviations::{NUPath, Season, Status},
    parser_types::{CompleteCourse, Requirement},
};
use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditToJson<'a> {
    majors: Vec<Cow<'a, str>>,
    minors: Vec<Cow<'a, str>>,
    audit_year: isize,
    grad_date: NaiveDate,
    complete_nupaths: Vec<NUPath>,
    complete_courses: Vec<CompleteCourse>,
    ip_nupaths: Vec<NUPath>,
    ip_courses: Vec<CompleteCourse>,
    required_nupaths: Vec<NUPath>,
    required_courses: Vec<Requirement>,
    earned_hours: f32,
    courses_taken: isize,
    attempted_hours: f32,
    points: f32,
    gpa: f32,
}

impl AuditToJson<'_> {
    pub fn new() -> Self {
        Self {
            majors: vec![],
            minors: vec![],
            audit_year: 2020,
            grad_date: NaiveDate::from_ymd(2020, 1, 1),
            complete_courses: vec![],
            complete_nupaths: vec![],
            ip_courses: vec![],
            ip_nupaths: vec![],
            required_courses: vec![],
            required_nupaths: vec![],
            earned_hours: 0_f32,
            courses_taken: 0,
            attempted_hours: 0_f32,
            points: 0_f32,
            gpa: 0_f32,
        }
    }
}

use pest::error::Error as PestError;
use pest::iterators::Pair;
use pest::Parser;
use std::convert::TryInto;

#[derive(Parser)]
#[grammar = "audit.pest"]
pub struct AuditParser;

impl AuditParser {
    pub fn parse_audit(file: &'_ str) -> Result<AuditToJson<'_>, PestError<Rule>> {
        let main = Self::parse(Rule::main, file)?.next().unwrap();
        let mut out = AuditToJson::new();
        fn parse_inner<'a>(mut out: &mut AuditToJson<'a>, rule: Pair<'a, Rule>) {
            match rule.as_rule() {
                Rule::GRAD_PARSER => {
                    let date = rule
                        .into_inner()
                        .next() // Move in to GRAD_PARSER Steps
                        .unwrap()
                        .into_inner()
                        .next() // Skip GRAD_STRING
                        .unwrap();
                    parse_inner(out, date);
                }
                Rule::CATALOG_PARSER => {
                    // Reach in to parser and get CATALOG_NUM
                    let year = rule.into_inner().next().unwrap();
                    parse_inner(out, year);
                }
                Rule::MAJOR => {
                    // Simple rule, just convert to string.
                    let mut majors = rule
                        .as_str()
                        .split('\n')
                        .map(Cow::from)
                        .collect::<Vec<Cow<'_, str>>>();
                    out.majors.append(&mut majors);
                }
                Rule::DATE => {
                    let date = NaiveDate::parse_from_str(rule.as_str(), "%D").unwrap();
                    out.grad_date = date;
                }
                Rule::CATALOG_NUM => {
                    let date = rule.as_str().parse::<isize>().unwrap();
                    out.audit_year = date;
                }
                Rule::COURSE_OPTION => {
                    // Reach into rule and recurse to NUPATH_PARSER, COURSE_LIST_PARSER, or COURSE_PARSER
                    parse_inner(out, rule.into_inner().next().unwrap());
                }
                Rule::NUPATH_PARSER => {
                    // Reach in to rule and get STATUS
                    let info = AuditParser::extract_nupath(rule);
                    match info {
                        (Some(Status::OK), Some(val)) => out.complete_nupaths.push(val),
                        (Some(Status::IP), Some(val)) => out.ip_nupaths.push(val),
                        (Some(Status::NO), Some(val)) => out.required_nupaths.push(val),
                        _ => eprintln!("WARNING: Incorrect parsing/rule for NUPath"),
                    }
                }
                Rule::COURSE_LIST_PARSER => {
                    let mut required_courses = AuditParser::extract_course_list(rule);
                    out.required_courses.append(&mut required_courses);
                }
                Rule::COURSE_PARSER => {
                    let (course, is_in_progress) = AuditParser::extract_course(rule);
                    if is_in_progress {
                        out.ip_courses.push(course);
                    } else {
                        out.complete_courses.push(course);
                    }
                }
                Rule::INFO => {
                    AuditParser::extract_info(&mut out, rule);
                }
                _ => eprintln!("SKipping"),
            }
        }
        main.into_inner().for_each(|rule| {
            parse_inner(&mut out, rule);
        });
        Ok(out)
    }

    fn extract_nupath(rules: Pair<'_, Rule>) -> (Option<Status>, Option<NUPath>) {
        // NUPATH has 3 significant Rules: STATUS, NUPATH_NAME, NUPATH_ID.
        // Right now, we only care about STATUS and NUPATH_ID.
        // So, we iterate through all children of NUPATH, skipping the ones we don't use.
        rules.into_inner().fold((None, None), |acc, pair| {
            let (status, id): (Option<Status>, Option<NUPath>) = acc;
            match pair.as_rule() {
                Rule::STATUS => {
                    let status_str = pair.as_str().to_string();
                    (Some(status_str.try_into().unwrap()), id)
                }
                Rule::NUPATH_ID => {
                    let id_str = pair.as_str().to_string();
                    (status, Some(id_str.try_into().unwrap()))
                }
                _ => (status, id),
            }
        })
    }

    /// Returns complete course and bool representing `ifInProgress`
    fn extract_course(rules: Pair<'_, Rule>) -> (CompleteCourse, bool) {
        let mut course = CompleteCourse::default();
        let mut in_progress = false;
        rules.into_inner().for_each(|pair| match pair.as_rule() {
            Rule::YEAR => {
                let year_str = pair.as_str();
                let season_str: Cow<'_, str> = year_str.chars().take(2).collect();
                let year_str: Cow<'_, str> = year_str.chars().skip(2).collect();
                course.season = season_str.try_into().unwrap();
                course.year = year_str.parse::<isize>().unwrap();
                course.term_id = Self::get_termid(course.season, course.year);
            }
            Rule::COURSE => {
                let subject = pair.as_str().to_string();
                course.subject = subject;
            }
            Rule::CREDITS => {
                let credits = pair.as_str();
                course.credit_hours = credits.parse::<f32>().unwrap();
            }
            Rule::COURSE_NAME => {
                let name = pair.as_str().to_string();
                course.name = name;
            }
            Rule::MAYBE_IP => {
                let as_str = pair.as_str().to_string();
                if as_str.contains("IP") {
                    in_progress = false;
                }
                if as_str.contains("(HON)") {
                    course.hon = true;
                }
            }
            _ => unreachable!(),
        });
        (course, in_progress)
    }

    fn extract_course_list(rules: Pair<'_, Rule>) -> Vec<Requirement> {
        let mut requirements = Vec::new();
        let mut prev_id: bool = false;
        let mut last_subject = None;

        rules.into_inner().for_each(|pair| match pair.as_rule() {
            Rule::COURSE => {
                let mut requirement = Requirement::default();
                pair.into_inner().for_each(|pair| match pair.as_rule() {
                    Rule::COURSE_NUMBER if !prev_id => {
                        requirement.class_id = AuditParser::to_num(pair.as_str()).unwrap();
                    }
                    Rule::COURSE_NUMBER if prev_id => {
                        requirement = requirements.pop().unwrap();
                        requirement.class_id_2 = Some(AuditParser::to_num(pair.as_str()).unwrap());
                    }
                    Rule::ID => {
                        requirement.subject = Some(pair.as_str().trim().to_string());
                        last_subject = requirement.subject.clone();
                    }
                    _ => unreachable!(),
                });
                if requirement.subject.is_none() {
                    requirement.subject = last_subject.clone();
                }
                requirements.push(requirement);
                prev_id = false;
            }
            Rule::TO => {
                prev_id = true;
            }
            _ => unreachable!(),
        });

        requirements
    }

    fn extract_info(audit: &mut AuditToJson<'_>, rule: Pair<'_, Rule>) {
        rule.into_inner().for_each(|pair| {
            match pair.as_rule() {
                Rule::EARNED_HOURS => {
                    audit.earned_hours = pair
                        .into_inner()
                        .next()
                        .unwrap() // Reach in for FLOAT
                        .as_str()
                        .parse::<f32>()
                        .unwrap();
                }
                Rule::COURSES_TAKEN => {
                    audit.courses_taken = pair
                        .into_inner()
                        .next()
                        .unwrap() // Reach in for FLOAT
                        .as_str()
                        .parse::<isize>()
                        .unwrap();
                }
                Rule::ATTEMPTED_HOURS => {
                    audit.attempted_hours = pair
                        .into_inner()
                        .next()
                        .unwrap() // Reach in for FLOAT
                        .as_str()
                        .parse::<f32>()
                        .unwrap();
                }
                Rule::POINTS => {
                    audit.points = pair
                        .into_inner()
                        .next()
                        .unwrap() // Reach in for FLOAT
                        .as_str()
                        .parse::<f32>()
                        .unwrap();
                }
                Rule::GPA => {
                    audit.gpa = pair
                        .into_inner()
                        .next()
                        .unwrap() // Reach in for FLOAT
                        .as_str()
                        .parse::<f32>()
                        .unwrap();
                }
                _ => unreachable!(),
            }
        });
    }

    fn get_termid(season: Season, year: isize) -> isize {
        let mut term_id = String::from("20");
        term_id.push_str(year.to_string().as_str());
        use Season::*;
        match season {
            FL => {
                term_id = String::from("20");
                term_id.push_str((year + 1).to_string().as_str());
                term_id.push_str("10");
            }
            SP => term_id.push_str("30"),
            S1 => term_id.push_str("40"),
            S2 => term_id.push_str("60"),
            SM => term_id.push_str("50"),
        }
        Self::to_num(term_id.as_str()).unwrap()
    }

    #[allow(clippy::wrong_self_convention)]
    fn to_num(input: &str) -> Result<isize, std::num::ParseIntError> {
        isize::from_str_radix(input, 10)
    }
}
