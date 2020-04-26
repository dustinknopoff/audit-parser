use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub mod abbreviations {
    use super::{Deserialize, Display, Serialize};
    use std::{borrow::Cow, convert::TryFrom};
    #[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
    pub enum NUPath {
        ND,
        EI,
        IC,
        FQ,
        SI,
        AD,
        DD,
        ER,
        WF,
        WD,
        WI,
        EX,
        CE,
    }

    impl Display for NUPath {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use NUPath::*;
            match self {
                ND => write!(f, "ND"),
                EI => write!(f, "EI"),
                IC => write!(f, "IC"),
                FQ => write!(f, "FQ"),
                SI => write!(f, "SI"),
                AD => write!(f, "AD"),
                DD => write!(f, "DD"),
                ER => write!(f, "ER"),
                WF => write!(f, "WF"),
                WD => write!(f, "WD"),
                WI => write!(f, "WI"),
                EX => write!(f, "EX"),
                CE => write!(f, "CE"),
            }
        }
    }

    impl TryFrom<String> for NUPath {
        type Error = String;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            use NUPath::*;
            match value.as_str() {
                "ND" => Ok(ND),
                "EI" => Ok(EI),
                "IC" => Ok(IC),
                "FQ" => Ok(FQ),
                "SI" => Ok(SI),
                "AD" => Ok(AD),
                "DD" => Ok(DD),
                "ER" => Ok(ER),
                "WF" => Ok(WF),
                "WD" => Ok(WD),
                "WI" => Ok(WI),
                "EX" => Ok(EX),
                "CE" => Ok(CE),
                _ => Err(String::from("ERROR")),
            }
        }
    }

    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    #[allow(unused)]
    pub enum Season {
        /// Fall
        FL,
        /// Spring
        SP,
        /// Summer 1
        S1,
        /// Summer 2
        S2,
        /// Full Summer Term
        SM,
    }

    impl Default for Season {
        fn default() -> Self {
            Self::FL
        }
    }

    impl Display for Season {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use Season::*;
            match self {
                FL => write!(f, "FL"),
                SP => write!(f, "SP"),
                S1 => write!(f, "S1"),
                S2 => write!(f, "S2"),
                SM => write!(f, "SM"),
            }
        }
    }

    impl<'a> TryFrom<Cow<'a, str>> for Season {
        type Error = String;
        fn try_from(value: Cow<'_, str>) -> Result<Self, Self::Error> {
            use Season::*;
            match value.as_ref() {
                "FL" => Ok(FL),
                "SP" => Ok(SP),
                "S1" => Ok(S1),
                "S2" => Ok(S2),
                "SM" => Ok(SM),
                _ => Err(String::from("ERROR")),
            }
        }
    }

    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    #[allow(unused)]
    pub enum Status {
        /// In Progress
        IP,
        /// Passed
        OK,
        /// Need to Take
        NO,
    }

    impl Display for Status {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use Status::*;
            match self {
                IP => write!(f, "IP"),
                OK => write!(f, "OK"),
                NO => write!(f, "NO"),
            }
        }
    }

    impl TryFrom<String> for Status {
        type Error = String;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            use Status::*;
            match value.as_str() {
                "IP" => Ok(IP),
                "OK" => Ok(OK),
                "NO" => Ok(NO),
                _ => Err(String::from("ERROR")),
            }
        }
    }

    #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
    #[allow(unused)]
    pub enum SeasonWord {
        Fall,
        Spring,
        Summer1,
        Summer2,
    }

    impl Display for SeasonWord {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use SeasonWord::*;
            match self {
                Fall => write!(f, "fall"),
                Spring => write!(f, "spring"),
                Summer1 => write!(f, "summer1"),
                Summer2 => write!(f, "summer2"),
            }
        }
    }
}

pub mod courses {
    use super::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Requirement {
        OrCourse(Vec<Requirement>),
        AndCourse(Vec<Requirement>),
        CourseRange(CourseRange),
        RequiredCourse(Course),
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CourseRange {
        credits_required: isize,
        ranges: Vec<SubjectRange>,
    }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SubjectRange {
        subject: String,
        id_range_start: isize,
        id_range_end: isize,
    }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Course {
        class_id: isize,
        subject: String,
        is_required: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum NeuPreqs {
        And(Vec<Prereq>),
        Or(Vec<Prereq>),
        One(Prereq),
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Prereq {
        class_id: String,
        subject: String,
        missing: Option<bool>,
    }
}

pub mod majors {
    use super::{
        abbreviations::NUPath,
        courses::{CourseRange, Requirement},
    };
    use super::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Major {
        name: String,
        requirement_groups: Vec<String>,
        requirement_group_map: HashMap<String, MajorRequirement>,
        year_version: isize,
        is_language_required: bool,
        total_credits_required: isize,
        nu_paths: Vec<NUPath>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Concentration {
        min_options: isize,
        max_options: isize,
        requirements_group_map: Vec<MajorRequirement>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum MajorRequirement {
        And(Section),
        Or(Section),
        Range(SectionRange),
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Section {
        requirements: Vec<Requirement>,
        num_credits_min: Option<isize>,
        num_credits_max: Option<isize>,
        name: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SectionRange {
        requirements: Vec<CourseRange>,
        num_credits_min: Option<isize>,
        num_credits_max: Option<isize>,
        name: String,
    }
}

pub mod schedule {
    use super::{abbreviations::Season, courses::Prereq, Display};
    use super::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Schedule {
        years: Vec<isize>,
        year_map: HashMap<isize, ScheduleYear>,
        id: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ScheduleYear {
        year: isize,
        fall: ScheduleTerm,
        spring: ScheduleTerm,
        summer1: ScheduleTerm,
        summer2: ScheduleTerm,
        is_summer_full: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum ScheduleTerm {
        Term(Term),
        /// Inner value is DndId
        Dnd(String),
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Term {
        season: Season,
        year: isize,
        term_id: isize,
        id: isize,
        status: Status,
        classes: Vec<ScheduleCourse>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Status {
        COOP,
        CLASSES,
        INACTIVE,
        HOVERINACTIVE,
        HOVERCOOP,
    }

    impl Display for Status {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use Status::*;
            match self {
                COOP => write!(f, "COOP"),
                CLASSES => write!(f, "CLASSES"),
                INACTIVE => write!(f, "INACTIVE"),
                HOVERINACTIVE => write!(f, "HOVERINACTIVE"),
                HOVERCOOP => write!(f, "HOVERCOOP"),
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum AllScheduleCourse {
        Course(ScheduleCourse),
        /// inner value is dnd id
        Dnd(String),
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ScheduleCourse {
        name: String,
        class_id: String,
        subject: String,
        pre_reqs: Option<Prereq>,
        co_reqs: Option<Prereq>,
        num_credits_min: isize,
        num_credits_max: isize,
    }
}

pub mod warnings {
    use super::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Warning {
        message: String,
        term_id: isize,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CourseWarning {
        subject: String,
        class_id: isize,
        warning: Warning,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WarningContainer {
        normal_warnings: Vec<Warning>,
        course_warnings: Vec<CourseWarning>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RequirementGroupWarning {
        message: String,
        requirement_group: isize,
    }
}

pub mod tracking {
    use super::majors::Major;
    use super::schedule::Schedule;
    use super::AllCourses;
    use super::{Deserialize, Serialize};

    pub trait CourseTakenTracker {
        fn contains(&self, input: &str) -> bool;
        fn add_courses(&mut self, to_add: Vec<AllCourses>, term_id: isize);
        fn add_course(&mut self, to_add: AllCourses, term_id: isize);
        fn get_term_ids(&self, course: String) -> Vec<isize>;
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserData {
        full_name: Option<String>,
        academic_year: Option<isize>,
        graduation_year: Option<isize>,
        major: Option<Major>,
        minors: Option<Vec<String>>,
        plan: Option<Schedule>,
    }
}

use parser_types::NEUCourse;
use schedule::AllScheduleCourse;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllCourses {
    Parser(NEUCourse),
    Model(AllScheduleCourse),
}

pub mod parser_types {
    use super::{
        abbreviations::{NUPath, Season},
        courses::Prereq,
    };
    use super::{Deserialize, Serialize};
    use chrono::NaiveDate;
    use std::collections::HashMap;

    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    pub struct CompleteCourse {
        /// True if course is Honors
        pub hon: bool,
        /// Subject i.e. "CS" or Psychology
        pub subject: String,
        pub class_id: isize,
        pub name: String,
        pub credit_hours: f32,
        pub season: Season,
        pub year: isize,
        /// Northeastern unique identifier
        pub term_id: isize,
    }

    impl PartialEq for CompleteCourse {
        fn eq(&self, other: &CompleteCourse) -> bool {
            return self.class_id == other.class_id
                && self.subject == other.subject
                && self.term_id == other.term_id
                && self.name == other.name;
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
    pub struct Requirement {
        pub class_id: isize,
        pub subject: Option<String>,
        pub num_required: Option<isize>,
        pub class_id_2: Option<isize>,
        pub list: Vec<isize>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Course {
        nupaths: Vec<NUPath>,
        courses: Vec<CompleteCourse>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Requirements {
        nupaths: Vec<Requirement>,
        courses: Vec<CompleteCourse>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Data {
        majors: Vec<String>,
        minors: Vec<String>,
        audit_year: isize,
        grad_date: NaiveDate,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct InitialScheduleRep {
        completed: Requirement,
        in_progress: Requirement,
        requirements: Requirements,
        data: Data,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NEUParentMap {
        most_recent_semester: isize,
        all_term_ids: Vec<isize>,
        class_map: HashMap<String, NEUClassMap>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NEUClassMap {
        term_id: isize,
        class_map: HashMap<String, NEUCourse>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NEUCourse {
        crns: Vec<String>,
        pre_reqs: Option<Prereq>,
        co_reqs: Option<Prereq>,
        max_credits: isize,
        min_credits: isize,
        desc: String,
        class_id: isize,
        pretty_url: String,
        name: String,
        url: String,
        last_update_time: isize,
        term_id: isize,
        host: String,
        subject: String,
        opt_prereqs_for: Option<Vec<Prereq>>,
        prereqs_for: Option<Vec<Prereq>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Schedule {
        completed: Vec<CompleteCourse>,
        scheduled: Vec<Vec<String>>,
    }
}
