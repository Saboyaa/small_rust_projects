use std::process::Command;

pub fn git_add() {
    Command::new("git")
        .args(["add","."])
        .output()
        .expect("Error in adding");
}

pub fn git_commit(text:String) {
    Command::new("git")
        .args(["commit","-m",&("\"".to_owned()+&text+"\"")])
        .output()
        .expect("Error in commiting");
}

pub fn git_push() {
    Command::new("git")
        .arg("push")
        .output()
        .expect("Error in pushing");
}


fn main() {
    git_add();
    git_commit("tessting".to_string());
    git_push();
    println!("Just added to the repository");
}
