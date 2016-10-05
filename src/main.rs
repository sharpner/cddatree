use std::fs;
use std::path::Path;
use std::io;

static RECIPES_FOLDER: &'static str="vendor/cdda/recipes";

fn visit_dirs(dir: &Path, cb: &Fn(&fs::DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            let path = entry.path();
            if path.is_dir() {
                try!(visit_dirs(&path, cb));
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn entry(cb: &fs::DirEntry) -> () {
	println!("{:?}", cb.path());
}

fn main() {
	let _ = visit_dirs(Path::new(RECIPES_FOLDER), &entry);
}

