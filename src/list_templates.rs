use std::{
    collections::HashMap,
    fmt::Write,
    path::{Path, PathBuf},
};

use crate::template;

pub fn run_list() {
    let templates_dir = template::templates_dir();
    let templates = template::list_templates_relative(&templates_dir);
    let templates_dir_str = templates_dir
        .to_str()
        .unwrap_or("ERROR Expanding Config Dir");

    if Vec::is_empty(&templates) {
        println!("No templates found in: {}", templates_dir_str);
        return;
    }

    println!("Listing template dir: {}", templates_dir_str);
    let template_tree: TemplateTree = templates.into();
    template_tree.print();
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TemplateFile(PathBuf);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dirname(PathBuf);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TemplateNode {
    Dir(Dirname, TemplateTree),
    File(TemplateFile),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TemplateTree(Vec<TemplateNode>);

impl TemplateTree {
    fn merge(&self, other: Self) -> Self {
        let mut dirs: HashMap<Dirname, TemplateTree> = HashMap::new();
        let mut elements = vec![];
        let iterator = self.0.iter().chain(other.0.iter());

        for el in iterator {
            match el.clone() {
                TemplateNode::Dir(dir, tree) => match dirs.get(&dir) {
                    None => {
                        dirs.insert(dir, tree);
                    }
                    Some(other_tree) => {
                        let new_tree: TemplateTree = other_tree.merge(tree);
                        dirs.insert(dir, new_tree);
                    }
                },
                TemplateNode::File(..) => elements.push(el.clone()),
            }
        }
        for (dir, tree) in dirs.into_iter() {
            elements.push(TemplateNode::Dir(dir, tree))
        }
        TemplateTree(elements)
    }
}

impl From<PathBuf> for TemplateTree {
    fn from(value: PathBuf) -> Self {
        let mut saw_file = false;
        let mut node: Option<TemplateNode> = None;
        fn part(pb: &Path) -> Option<PathBuf> {
            pb.to_path_buf().file_name().map(PathBuf::from)
        }
        for parent in value.ancestors() {
            let part = part(parent);
            if part.is_none() {
                continue;
            }
            let part = part.unwrap();

            if !saw_file {
                saw_file = true;
                node = Some(TemplateNode::File(TemplateFile(part)));
                continue;
            }

            let prev_tree = TemplateTree(vec![node.unwrap()]);
            let new_node = Some(TemplateNode::Dir(Dirname(part), prev_tree));
            node = new_node;
        }
        TemplateTree(vec![node.unwrap()])
    }
}

impl From<Vec<PathBuf>> for TemplateTree {
    fn from(input: Vec<PathBuf>) -> Self {
        let mut value = input.clone();
        value.sort();
        let trees: Vec<TemplateTree> = value.iter().map(|p| p.clone().into()).collect();
        trees
            .iter()
            .fold(TemplateTree(vec![]), |acc, el| acc.merge(el.clone()))
    }
}

pub trait DepthPrint {
    fn string_at_depth(&self, depth: u16) -> String;
    fn print_depth(&self, depth: u16) {
        println!("{}", self.string_at_depth(depth));
    }
    fn print(&self) {
        self.print_depth(0)
    }
}

impl DepthPrint for Dirname {
    fn string_at_depth(&self, depth: u16) -> String {
        let mut result = String::new();
        if depth == 0 {
            result.push('\n');
        }
        result.push_str(&" ".repeat(depth as usize));
        result.push_str(self.0.to_str().unwrap());
        result.push('/');

        result
    }
}

impl DepthPrint for TemplateNode {
    fn string_at_depth(&self, depth: u16) -> String {
        let mut result = String::new();
        match self {
            TemplateNode::Dir(dirname, t) => {
                result.push_str(&dirname.string_at_depth(depth));
                result.push('\n');
                result.push_str(&t.string_at_depth(depth + 2));
            }

            TemplateNode::File(t) => {
                result.push_str(&t.string_at_depth(depth));
                result.push('\n');
            }
        }
        result
    }
}

impl DepthPrint for TemplateTree {
    fn string_at_depth(&self, depth: u16) -> String {
        let mut result = String::new();
        let mut items = self.0.clone();
        items.sort_unstable();

        for n in items.iter() {
            result.push_str(&n.string_at_depth(depth));
        }
        result
    }
}

impl DepthPrint for TemplateFile {
    fn string_at_depth(&self, depth: u16) -> String {
        let mut result = String::new();
        let _ = result.write_str(&" ".repeat(depth as usize));
        let _ = result.write_str("- ");
        let _ = result.write_str(
            self.0
                .file_name()
                .expect("Can't access PathBuf filename")
                .to_str()
                .expect("Can't cast &OsStr to &str"),
        );
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn single_child(node: TemplateNode) -> TemplateTree {
        TemplateTree(vec![node])
    }
    fn t_dir(s: &str, tt: TemplateTree) -> TemplateNode {
        TemplateNode::Dir(Dirname(PathBuf::from(s)), tt)
    }
    fn t_file(s: &str) -> TemplateNode {
        TemplateNode::File(TemplateFile(PathBuf::from(s)))
    }
    #[test]
    fn template_tree_from_pathbuf() -> Result<(), Box<dyn std::error::Error>> {
        let got: TemplateTree = PathBuf::from("a/b/c.tmplr").into();
        let expected: TemplateTree = single_child(t_dir(
            "a",
            single_child(t_dir("b", single_child(t_file("c.tmplr")))),
        ));

        assert_eq!(got, expected);
        Ok(())
    }
    #[test]
    fn template_tree_merge() -> Result<(), Box<dyn std::error::Error>> {
        let a: TemplateTree = PathBuf::from("a/b/c.tmplr").into();
        let b: TemplateTree = PathBuf::from("a/b/d.tmplr").into();
        let c: TemplateTree = PathBuf::from("a/b/e.tmplr").into();
        let got = a.merge(b).merge(c);
        let expected: TemplateTree = single_child(t_dir(
            "a",
            single_child(t_dir(
                "b",
                TemplateTree(vec![
                    t_file("c.tmplr"),
                    t_file("d.tmplr"),
                    t_file("e.tmplr"),
                ]),
            )),
        ));

        assert_eq!(got, expected);
        Ok(())
    }
    #[test]
    fn template_tree_to_string() -> Result<(), Box<dyn std::error::Error>> {
        let a: TemplateTree = PathBuf::from("a/b/c.tmplr").into();
        let b: TemplateTree = PathBuf::from("a/b/d.tmplr").into();
        let c: TemplateTree = PathBuf::from("a/b/e.tmplr").into();
        let got = a.merge(b).merge(c).string_at_depth(0);
        let expected: String = String::from(
            r#"
a/
  b/
    - c.tmplr
    - d.tmplr
    - e.tmplr
            "#,
        )
        .trim_end_matches(' ')
        .to_owned();

        assert_eq!(got, expected);
        Ok(())
    }
}
