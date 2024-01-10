use diff::Diff;
use serde::{Deserialize, Serialize};

pub mod prototype;
pub mod runtime;

pub trait DiffPrint<T: Diff> {
    fn diff_print(&self, old: &T, new: &T, indent: usize, name: &str);
}

impl<T> DiffPrint<T> for Option<T>
where
    T: Diff<Repr = Option<T>> + PartialEq + std::fmt::Debug,
{
    fn diff_print(&self, _old: &T, _new: &T, indent: usize, name: &str) {
        let indent_str = " ".repeat(indent);

        if let Some(new) = self {
            println!("{indent_str}{name}: {new:?}",);
        }
    }
}

impl<T> DiffPrint<T> for T
where
    T: Diff<Repr = T> + PartialEq + std::fmt::Debug + ?Sized,
{
    fn diff_print(&self, old: &T, new: &T, indent: usize, name: &str) {
        let indent_str = " ".repeat(indent);

        if old != new {
            println!("{indent_str}{name}: {self:?}");
        }
    }
}

impl<T> DiffPrint<Option<T>> for diff::OptionDiff<T>
where
    T: Diff + PartialEq + std::fmt::Debug,
{
    fn diff_print(&self, _old: &Option<T>, new: &Option<T>, indent: usize, name: &str) {
        let indent_str = " ".repeat(indent);

        match self {
            diff::OptionDiff::Some(_) => {
                println!("{indent_str}{name}: {:?}", new.as_ref())
            }
            diff::OptionDiff::None => println!("{indent_str}{name}: None"),
            diff::OptionDiff::NoChange => {}
        }
    }
}

impl DiffPrint<Vec<String>> for diff::VecDiff<String> {
    fn diff_print(&self, old: &Vec<String>, new: &Vec<String>, indent: usize, name: &str) {
        let indent_str = " ".repeat(indent);
        if self.0.is_empty() {
            return;
        }

        println!("{indent_str}{name}:");
        for diff in &self.0 {
            match diff {
                diff::VecDiffType::Inserted { changes, .. } => {
                    for ins in changes.iter().flatten() {
                        ins.lines().for_each(|l| {
                            println!("{indent_str}  +{l}");
                        });
                    }
                }
                diff::VecDiffType::Removed { index, .. } => {
                    let old_val = old.get(*index).cloned().unwrap_or_default();
                    old_val.lines().for_each(|l| {
                        println!("{indent_str}  -{l}");
                    });
                }
                diff::VecDiffType::Altered { index, changes } => {
                    let (o, n) = if old.get(*index).is_none() || new.get(*index).is_none() {
                        (Default::default(), Default::default())
                    } else {
                        (old[*index].clone(), new[*index].clone())
                    };

                    println!("{indent_str}  *");
                    for diff in changes {
                        diff.diff_print(&o, &n, indent + 4, "");
                    }
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff)]
#[diff(attr(
    #[derive(Debug, Serialize, Deserialize)]
))]
#[serde(rename_all = "lowercase")]
pub enum Application {
    Factorio,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff)]
#[diff(attr(
    #[derive(Debug, Serialize, Deserialize)]
))]
#[serde(rename_all = "lowercase")]
pub enum Stage {
    Prototype,
    Runtime,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Diff)]
#[diff(attr(
    #[derive(Debug, Serialize, Deserialize)]
))]
pub struct Common {
    pub application: Application,
    pub stage: Stage,
    pub application_version: String,
    pub api_version: u8,
}

impl DiffPrint<Common> for CommonDiff {
    fn diff_print(&self, old: &Common, new: &Common, indent: usize, _name: &str) {
        let indent_str = " ".repeat(indent);
        println!(
            "{indent_str}application version: {} => {}",
            old.application_version, new.application_version
        );
    }
}
