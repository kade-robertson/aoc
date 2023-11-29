use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::{Rc, Weak},
};

use anyhow::{anyhow, Context, Result};

use common::{Problem, Solution};
/// \--- Day 7: No Space Left On Device ---
/// ----------
///
/// You can hear birds chirping and raindrops hitting leaves as the expedition
/// proceeds. Occasionally, you can even hear much louder sounds in the
/// distance; how big do the animals get out here, anyway?
///
/// The device the Elves gave you has problems with more than just its
/// communication system. You try to run a system update:
///
/// ```
/// $ system-update --please --pretty-please-with-sugar-on-top
/// Error: No space left on device
/// ```
///
/// Perhaps you can delete some files to make space for the update?
///
/// You browse around the filesystem to assess the situation and save the
/// resulting terminal output (your puzzle input). For example:
///
/// ```
/// $ cd /
/// $ ls
/// dir a
/// 14848514 b.txt
/// 8504156 c.dat
/// dir d
/// $ cd a
/// $ ls
/// dir e
/// 29116 f
/// 2557 g
/// 62596 h.lst
/// $ cd e
/// $ ls
/// 584 i
/// $ cd ..
/// $ cd ..
/// $ cd d
/// $ ls
/// 4060174 j
/// 8033020 d.log
/// 5626152 d.ext
/// 7214296 k
/// ```
///
/// The filesystem consists of a tree of files (plain data) and directories
/// (which can contain other directories or files). The outermost directory is
/// called `/`. You can navigate around the filesystem, moving into or out of
/// directories and listing the contents of the directory you're currently in.
///
/// Within the terminal output, lines that begin with `$` are *commands you
/// executed*, very much like some modern computers:
///
/// * `cd` means *change directory*. This changes which directory is the current
///   directory, but the specific result depends on the argument:
///   * `cd x` moves *in* one level: it looks in the current directory for the
///     directory named `x` and makes it the current directory.
///   * `cd ..` moves *out* one level: it finds the directory that contains the
///     current directory, then makes that directory the current directory.
///   * `cd /` switches the current directory to the outermost directory, `/`.
///
/// * `ls` means *list*. It prints out all of the files and directories
///   immediately contained by the current directory:
///   * `123 abc` means that the current directory contains a file named `abc`
///     with size `123`.
///   * `dir xyz` means that the current directory contains a directory named
///     `xyz`.
///
/// Given the commands and output in the example above, you can determine that
/// the filesystem looks visually like this:
///
/// ```
/// - / (dir)
///   - a (dir)
///     - e (dir)
///       - i (file, size=584)
///     - f (file, size=29116)
///     - g (file, size=2557)
///     - h.lst (file, size=62596)
///   - b.txt (file, size=14848514)
///   - c.dat (file, size=8504156)
///   - d (dir)
///     - j (file, size=4060174)
///     - d.log (file, size=8033020)
///     - d.ext (file, size=5626152)
///     - k (file, size=7214296)
/// ```
///
/// Here, there are four directories: `/` (the outermost directory), `a` and `d`
/// (which are in `/`), and `e` (which is in `a`). These directories also
/// contain files of various sizes.
///
/// Since the disk is full, your first step should probably be to find
/// directories that are good candidates for deletion. To do this, you need to
/// determine the *total size* of each directory. The total size of a directory
/// is the sum of the sizes of the files it contains, directly or indirectly.
/// (Directories themselves do not count as having any intrinsic size.)
///
/// The total sizes of the directories above can be found as follows:
///
/// * The total size of directory `e` is *584* because it contains a single file
///   `i` of size 584 and no other directories.
/// * The directory `a` has total size *94853* because it contains files `f`
///   (size 29116), `g` (size 2557), and `h.lst` (size 62596), plus file `i`
///   indirectly (`a` contains `e` which contains `i`).
/// * Directory `d` has total size *24933642*.
/// * As the outermost directory, `/` contains every file. Its total size is
///   *48381165*, the sum of the size of every file.
///
/// To begin, find all of the directories with a total size of *at most 100000*,
/// then calculate the sum of their total sizes. In the example above, these
/// directories are `a` and `e`; the sum of their total sizes is `*95437*`
/// (94853 + 584). (As in this example, this process can count files more than
/// once!)
///
/// Find all of the directories with a total size of at most 100000. *What is
/// the sum of the total sizes of those directories?*
///
/// \--- Part Two ---
/// ----------
///
/// Now, you're ready to choose a directory to delete.
///
/// The total disk space available to the filesystem is `*70000000*`. To run the
/// update, you need unused space of at least `*30000000*`. You need to find a
/// directory you can delete that will *free up enough space* to run the update.
///
/// In the example above, the total size of the outermost directory (and thus
/// the total amount of used space) is `48381165`; this means that the size of
/// the *unused* space must currently be `21618835`, which isn't quite the
/// `30000000` required by the update. Therefore, the update still requires a
/// directory with total size of at least `8381165` to be deleted before it can
/// run.
///
/// To achieve this, you have the following options:
///
/// * Delete directory `e`, which would increase unused space by `584`.
/// * Delete directory `a`, which would increase unused space by `94853`.
/// * Delete directory `d`, which would increase unused space by `24933642`.
/// * Delete directory `/`, which would increase unused space by `48381165`.
///
/// Directories `e` and `a` are both too small; deleting them would not free up
/// enough space. However, directories `d` and `/` are both big enough! Between
/// these, choose the *smallest*: `d`, increasing unused space by `*24933642*`.
///
/// Find the smallest directory that, if deleted, would free up enough space on
/// the filesystem to run the update. *What is the total size of that
/// directory?*
static PROBLEM_INPUT: &str = include_str!("input.txt");
pub struct Day07;

struct Directory {
    parent: Option<Weak<RefCell<Directory>>>,
    file_size: u64,
    directories: HashMap<String, Rc<RefCell<Directory>>>,
}

impl Directory {
    pub fn dir_size(&self) -> u64 {
        let subdirectory_size = self
            .directories
            .values()
            .map(|d| if let Ok(dir) = d.try_borrow() { dir.dir_size() } else { 0 })
            .sum::<u64>();

        self.file_size + subdirectory_size
    }
}

struct Filesystem {
    root: Rc<RefCell<Directory>>,
    current_directory: Weak<RefCell<Directory>>,
}

impl Filesystem {
    pub fn new() -> Self {
        let root_directory = Rc::new(RefCell::new(Directory {
            parent: None,
            file_size: 0,
            directories: HashMap::new(),
        }));
        Self { root: root_directory, current_directory: Weak::new() }
    }

    pub fn init(&mut self) {
        self.current_directory = Rc::downgrade(&self.root);
    }

    pub fn mkdir(&self, name: &str) -> Result<()> {
        let new_directory = Rc::new(RefCell::new(Directory {
            parent: Some(Weak::clone(&self.current_directory)),
            file_size: 0,
            directories: HashMap::new(),
        }));

        self.current_directory
            .upgrade()
            .context("Could not access current directory")?
            .try_borrow_mut()?
            .directories
            .insert(name.to_owned(), new_directory);

        Ok(())
    }

    pub fn chdir(&mut self, name: &str) -> Result<()> {
        if name == ".." {
            if let Some(parent_dir) = &self
                .current_directory
                .upgrade()
                .context("Could not access current directory")?
                .try_borrow()?
                .parent
            {
                self.current_directory = Weak::clone(parent_dir);
                Ok(())
            } else {
                Err(anyhow!("Could not access parent of current directory"))
            }
        } else if name == "/" {
            self.current_directory = Rc::downgrade(&self.root);
            Ok(())
        } else if let Some(new_dir) = self
            .current_directory
            .upgrade()
            .context("Could not access current directory")?
            .try_borrow_mut()?
            .directories
            .get(name)
        {
            self.current_directory = Rc::downgrade(new_dir);
            Ok(())
        } else {
            Err(anyhow!("Could not change directory"))
        }
    }

    pub fn mkfile(&mut self, size: u64) -> Result<()> {
        self.current_directory
            .upgrade()
            .context("Could not access current directory")?
            .try_borrow_mut()?
            .file_size += size;

        Ok(())
    }
}

impl Day07 {
    fn parse(&self, data: &str) -> Filesystem {
        let mut fs = Filesystem::new();
        fs.init();

        data.lines().for_each(|l| {
            if l.starts_with("$ cd") {
                fs.chdir(&l[5..]).unwrap()
            } else if l.starts_with("dir") {
                fs.mkdir(&l[4..]).unwrap();
            } else if l.starts_with(|c: char| c.is_numeric()) {
                let mut file_split = l.split(' ');
                fs.mkfile(file_split.next().unwrap().parse::<u64>().unwrap()).unwrap();
            }
        });

        fs
    }

    fn solve_actual(&self, filesystem: &Filesystem) -> u64 {
        let root_directory = filesystem.root.borrow();
        let mut dirs_to_walk: VecDeque<Rc<RefCell<Directory>>> =
            VecDeque::from_iter(root_directory.directories.values().map(Rc::clone));
        let mut total_size = 0;

        while let Some(dir) = dirs_to_walk.pop_front() {
            let dir_ref = dir.borrow();
            let dir_size = dir_ref.dir_size();

            if dir_size <= 100000 {
                total_size += dir_size;
            }

            dirs_to_walk.extend(dir_ref.directories.values().map(Rc::clone));
        }

        total_size
    }

    fn solve_actual_part2(&self, filesystem: &Filesystem) -> u64 {
        let root_directory = filesystem.root.borrow();
        let root_size = root_directory.dir_size();
        let space_remaining = 70_000_000 - root_size;
        let space_to_free = 30_000_000 - space_remaining;
        let mut dirs_to_walk: VecDeque<Rc<RefCell<Directory>>> =
            VecDeque::from_iter(root_directory.directories.values().map(Rc::clone));

        let mut smallest_candidate = root_size;

        while let Some(dir) = dirs_to_walk.pop_front() {
            let dir_ref = dir.borrow();
            let dir_size = dir_ref.dir_size();

            // Only directories that are already large enough to be candidates
            // can possibly have children that are also candidates.
            if dir_size >= space_to_free {
                if dir_size <= smallest_candidate {
                    smallest_candidate = dir_size;
                }
                dirs_to_walk.extend(dir_ref.directories.values().map(Rc::clone));
            }
        }

        smallest_candidate
    }
}

impl Problem for Day07 {
    fn day(&self) -> u8 {
        7u8
    }
    fn name(&self) -> &str {
        "Day 7: No Space Left On Device"
    }
    fn solve(&self) -> Solution {
        Solution::U64(self.solve_actual(&self.parse(PROBLEM_INPUT)))
    }
    fn solve_part2(&self) -> Solution {
        Solution::U64(self.solve_actual_part2(&self.parse(PROBLEM_INPUT)))
    }
}
