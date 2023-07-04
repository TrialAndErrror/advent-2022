trait HasSize {
    fn get_size(&self) -> usize {
        0
    }
}

pub struct File {
    name: String,
    size: usize
}

impl File {
    fn from_string(entry: &str) -> File {
        let parts = entry.split(" ").collect::<Vec<&str>>();
        File {
            name: String::from(parts[1]),
            size: parts[0].parse().unwrap()
        }
    }
}

impl HasSize for File {
    fn get_size(&self) -> usize {
        self.size
    }
}

pub struct Directory<'a> {
    name: String,
    files: Vec<File>,
    parent: &'a Directory<'a>,
}

impl Directory {
    fn from_string(entry: &str, parent: &Directory) -> Directory<'a> {
        let parts = entry.split(" ").collect::<Vec<&str>>();
        Directory {
            name: String::from(parts[1]),
            files: vec![],
            parent
        }
    }
}

impl HasSize for Directory<'a> {
    fn get_size(&self) -> usize {
        self.files.iter().map(|file|file.get_size()).collect::<Vec<usize>>().iter().sum()
    }
}