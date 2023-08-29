use std::{
    env,
    io::stdin,
    path::{Path, PathBuf},
};

pub fn confirm(text: &str) {
    println!("{} [y/n]: ", text);

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();

    if !(s == "y\n" || s == "yes\n") {
        println!("Exited");
        std::process::exit(0);
    }
}

pub struct FindWorkspace {
    pub workspace: PathBuf,
    pub relative: String,
}

pub fn find_workspace(path: &Path) -> FindWorkspace {
    let built = build_path(path);

    let mut relative = built.file_name().unwrap().to_str().unwrap().to_string();
    let mut path = built
        .parent()
        .expect("Cannot find a workspace at '/'")
        .to_path_buf();

    while !path.join("Cargo.toml").exists() {
        relative.insert(0, '/');
        relative.insert_str(0, path.file_name().unwrap().to_str().unwrap());
        path = path.parent().expect("Not in a workspace.").to_path_buf();
    }

    FindWorkspace {
        workspace: path.join("Cargo.toml"),
        relative,
    }
}

pub fn file_name(path: &Path) -> String {
    path.file_name()
        .unwrap_or(build_path(path).file_name().unwrap())
        .to_str()
        .unwrap()
        .to_string()
}

fn build_path(path: &Path) -> PathBuf {
    if path.starts_with("/") {
        path.to_path_buf()
    } else if path.starts_with("~/") {
        let home = home::home_dir().unwrap();
        home.join(&path.to_str().unwrap()[2..])
    } else if path.starts_with("./") {
        let pwd = env::current_dir().unwrap();
        pwd.join(&path.to_str().unwrap()[2..])
    } else {
        let pwd = env::current_dir().unwrap();
        pwd.join(path)
    }
}
