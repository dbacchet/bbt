
use std::process::Command;
// use regex::Regex;

#[derive(PartialEq, Default, Clone, Debug)]
struct Commit {
    hash: String,
    message: String,
}

#[derive(Debug)]
struct EntryTree {
    name: String,
    fullpath: String,
    children: Vec<EntryTree>,
}

impl EntryTree {
    // create a tree
    fn new(root_name: &str) -> EntryTree {
        EntryTree {
            name : String::from(root_name),
            fullpath : String::from(root_name),
            children : Vec::new(),
        }
    }
    // builder function for the fullpath
    fn fullpath(mut self, fullpath: String) -> Self {
        self.fullpath = fullpath;
        self
    }

    // get or create an entry 
    fn get_or_create(&mut self, name: &str) -> &mut EntryTree {
        if self.name == name {
            return self
        }
        let pos = self.children.iter().position(|x| { x.name == name });
        match pos {
            Some(idx) => &mut self.children[idx],
            None => { self.children.push(EntryTree::new(name).fullpath([&self.fullpath, "/", name].concat())); 
                      &mut self.children[0] }
        }
    }

    fn print(&self, depth: u32) {
        for _ in 0..depth {
            print!("   ");
        }
        let indicator: &str = if self.children.len()>0 { "+" } else { "-" };
        println!("{} {} [{}]", indicator, self.name, self.fullpath);
        // print children
        for c in self.children.iter() {
            c.print(depth+1);
        }
    }
}

fn main() {
    let output = Command::new("bazel").arg("query").arg("//...").arg("--output").arg("package").output().unwrap();

    if !output.status.success() {
        println!("Command executed with failing error code");
        return;
    }

    let data = String::from_utf8(output.stdout).unwrap();

    let mut tree = EntryTree::new("/");
    data.lines().for_each(|s| {
        // println!("{}", s);
        let tokens = s.split("/");
        let mut node = tree.get_or_create("/");
        for t in tokens {
            node = node.get_or_create(t);
        }
    });

    tree.print(0);

    // // another one
    //
    // let output = Command::new("git").arg("log").arg("--oneline").output().unwrap();
    //
    // if !output.status.success() {
    //     println!("Command executed with failing error code");
    //     return;
    // }
    //
    // // println!("{}", String::from_utf8(output.stdout).unwrap());
    //
    // let pattern = Regex::new(r"(?x)
    //                            ([0-9a-fA-F]+) # commit hash
    //                            (.*)           # The commit message").unwrap();
    //
    // String::from_utf8(output.stdout).unwrap()
    //     .lines()
    //     .filter_map(|line| pattern.captures(line))
    //     .map(|cap| {
    //              Commit {
    //                  hash: cap[1].to_string(),
    //                  message: cap[2].trim().to_string(),
    //              }
    //          })
    //     .take(5)
    //     .for_each(|x| println!("{:?}", x));

}
