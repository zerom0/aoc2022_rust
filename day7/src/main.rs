fn main() {
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(_) => {
            if line.starts_with("$ cd /") {
                let filesystem = enter_directory("");
                let free_space = 70_000_000 - filesystem.dir_size;
                let required_space = 30_000_000 - free_space;
                println!("required space = {}", required_space);
                println!("{:?}", walk(&filesystem, required_space));
            }
        }
        _ => (),
    }
}

fn walk(subtree: &Dir, required: usize) -> usize {
    let mut best_fit = 70_000_000;
    if subtree.dir_size >= required {
        best_fit = subtree.dir_size
    }
    for subdir in subtree.sub_dirs.iter() {
        let candidate_size = walk(&subdir, required);
        if candidate_size < best_fit {
            best_fit = candidate_size
        }
    }
    best_fit
}

#[derive(Debug)]
struct Dir {
    name: String,
    dir_size: usize,
    sub_dirs: Vec<Dir>,
}

fn enter_directory(name: &str) -> Dir {
    //    println!("Entered directory {}/", name);
    let mut dir_size = 0;
    let mut sub_dirs = Vec::<Dir>::new();
    loop {
        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => {
                if line.starts_with("$ cd ..") || line.is_empty() {
                    //                    println!("{}/ has size {}", &name, dir_size);
                    return Dir {
                        name: name.to_string(),
                        dir_size,
                        sub_dirs,
                    };
                } else if line.starts_with("$ cd ") {
                    let subdir_name = &line[5..].trim();
                    let subdir = enter_directory(&format!("{}/{}", name, &subdir_name));
                    dir_size = dir_size + subdir.dir_size;
                    sub_dirs.push(subdir);
                } else if line.starts_with("$ ls") {
                } else if line.starts_with("dir") {
                } else {
                    if let Some((size, _)) = line.split_once(' ') {
                        dir_size = dir_size + size.parse::<usize>().unwrap();
                    } else {
                    }
                }
            }
            _ => (),
        }
    }
}
