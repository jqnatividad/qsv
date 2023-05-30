use std::{
    env, fmt, fs,
    fs::File,
    io::{self, Read, Write},
    path::{Path, PathBuf},
    process,
    str::FromStr,
    sync::atomic,
    time::Duration,
};

use uuid::Uuid;

use crate::Csv;

static QSV_INTEGRATION_TEST_DIR: &str = "xit";

static NEXT_ID: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

pub struct Workdir {
    root:     PathBuf,
    dir:      PathBuf,
    flexible: bool,
}

impl Drop for Workdir {
    fn drop(&mut self) {
        if let Err(err) = fs::remove_dir_all(&self.dir) {
            panic!("Could not remove '{:?}': {err}", self.dir);
        }
    }
}

impl Workdir {
    pub fn new(name: &str) -> Workdir {
        let id = NEXT_ID.fetch_add(1, atomic::Ordering::SeqCst);
        let mut root = env::current_exe()
            .unwrap()
            .parent()
            .expect("executable's directory")
            .to_path_buf();
        if root.ends_with("deps") {
            root.pop();
        }
        let dir = root
            .join(QSV_INTEGRATION_TEST_DIR)
            .join(name)
            .join(format!("test-{id}-{}", Uuid::new_v4()));
        if let Err(err) = create_dir_all(&dir) {
            panic!("Could not create '{dir:?}': {err}");
        }
        Workdir {
            root,
            dir,
            flexible: false,
        }
    }

    pub fn flexible(mut self, yes: bool) -> Workdir {
        self.flexible = yes;
        self
    }

    pub fn create<T: Csv>(&self, name: &str, rows: T) {
        self.create_with_delim(name, rows, b',')
    }

    pub fn create_with_delim<T: Csv>(&self, name: &str, rows: T, delim: u8) {
        let mut wtr = csv::WriterBuilder::new()
            .flexible(self.flexible)
            .delimiter(delim)
            .from_path(self.path(name))
            .unwrap();
        for row in rows.to_vecs() {
            wtr.write_record(row).unwrap();
        }
        wtr.flush().unwrap();
    }

    pub fn create_indexed<T: Csv>(&self, name: &str, rows: T) {
        self.create(name, rows);

        let mut cmd = self.command("index");
        cmd.arg(name);
        self.run(&mut cmd);
    }

    pub fn create_from_string(&self, name: &str, data: &str) {
        let filename = &self.path(name);
        let mut file = File::create(filename).unwrap();
        file.write_all(data.as_bytes()).unwrap();
        file.flush().unwrap();
    }

    pub fn read_to_string(&self, filename: &str) -> String {
        let mut file = File::open(self.path(filename)).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap_or_default();
        contents
    }

    pub fn read_stdout<T: Csv>(&self, cmd: &mut process::Command) -> T {
        let stdout: String = self.stdout(cmd);
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_reader(io::Cursor::new(stdout));

        let records: Vec<Vec<String>> = rdr
            .records()
            .collect::<Result<Vec<csv::StringRecord>, _>>()
            .unwrap()
            .into_iter()
            .map(|r| r.iter().map(std::string::ToString::to_string).collect())
            .collect();
        Csv::from_vecs(records)
    }

    pub fn command(&self, sub_command: &str) -> process::Command {
        let mut cmd = process::Command::new(self.qsv_bin());
        if sub_command.is_empty() {
            cmd.current_dir(&self.dir);
        } else {
            cmd.current_dir(&self.dir).arg(sub_command);
        }
        cmd
    }

    pub fn output(&self, cmd: &mut process::Command) -> process::Output {
        cmd.output().unwrap()
    }

    pub fn run(&self, cmd: &mut process::Command) {
        self.output(cmd);
    }

    pub fn stdout<T: FromStr>(&self, cmd: &mut process::Command) -> T {
        let o = self.output(cmd);
        let stdout = String::from_utf8_lossy(&o.stdout);
        stdout
            .trim_matches(&['\r', '\n'][..])
            .parse()
            .ok()
            .unwrap_or_else(|| panic!("Could not convert from string: '{stdout}'"))
    }

    pub fn output_stderr(&self, cmd: &mut process::Command) -> String {
        {
            // ensures stderr has been flushed before we run our cmd
            let mut _stderr = io::stderr();
            _stderr.flush().unwrap();
        }
        let o = cmd.output().unwrap();
        let o_utf8 = String::from_utf8_lossy(&o.stderr).to_string();
        if !o.status.success() || !o_utf8.is_empty() {
            o_utf8
        } else {
            "No error".to_string()
        }
    }

    pub fn assert_success(&self, cmd: &mut process::Command) {
        let o = cmd.output().unwrap();
        assert!(
            o.status.success(),
            "\n\n===== {:?} =====\ncommand failed but expected success!\n\ncwd: {}\n\nstatus: \
             {}\n\nstdout: {}\n\nstderr: {}\n\n=====\n",
            cmd,
            self.dir.display(),
            o.status,
            String::from_utf8_lossy(&o.stdout),
            String::from_utf8_lossy(&o.stderr)
        );
    }

    pub fn assert_err(&self, cmd: &mut process::Command) {
        let o = cmd.output().unwrap();
        assert!(
            !o.status.success(),
            "\n\n===== {:?} =====\ncommand succeeded but expected failure!\n\ncwd: {}\n\nstatus: \
             {}\n\nstdout: {}\n\nstderr: {}\n\n=====\n",
            cmd,
            self.dir.display(),
            o.status,
            String::from_utf8_lossy(&o.stdout),
            String::from_utf8_lossy(&o.stderr)
        );
    }

    /// returns contents of specified file in resources/test directory
    pub fn load_test_resource(&self, filename: &str) -> String {
        // locate resources/test relative to crate base dir
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/test/");
        path.push(filename);

        self.from_str::<String>(path.as_path())
    }

    /// copy the file in resources/test directory to the working directory
    /// returns absolute file path of the copied file
    pub fn load_test_file(&self, filename: &str) -> String {
        // locate resources/test relative to crate base dir
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/test/");
        path.push(filename);

        let resource_file_path = path.into_os_string().into_string().unwrap();

        let mut wrkdir_path = self.dir.clone();
        wrkdir_path.push(filename);

        fs::copy(resource_file_path, wrkdir_path.clone()).unwrap();

        wrkdir_path.into_os_string().into_string().unwrap()
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_str<T: FromStr>(&self, name: &Path) -> T {
        log::debug!("reading file: {name:?}");
        let mut o = String::new();
        fs::File::open(name)
            .unwrap()
            .read_to_string(&mut o)
            .unwrap();
        o.parse().ok().expect("fromstr")
    }

    pub fn path(&self, name: &str) -> PathBuf {
        self.dir.join(name)
    }

    #[cfg(feature = "feature_capable")]
    pub fn qsv_bin(&self) -> PathBuf {
        self.root.join("qsv")
    }

    #[cfg(feature = "lite")]
    pub fn qsv_bin(&self) -> PathBuf {
        self.root.join("qsvlite")
    }

    #[cfg(feature = "datapusher_plus")]
    pub fn qsv_bin(&self) -> PathBuf {
        self.root.join("qsvdp")
    }

    // clear all files in directory
    pub fn clear_contents(&self) -> io::Result<()> {
        for entry in fs::read_dir(&self.dir)? {
            fs::remove_file(entry?.path())?;
        }
        Ok(())
    }
}

impl fmt::Debug for Workdir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "path={}", self.dir.display())
    }
}

// For whatever reason, `fs::create_dir_all` fails intermittently on CI
// with a weird "file exists" error. Despite my best efforts to get to the
// bottom of it, I've decided a try-wait-and-retry hack is good enough.
fn create_dir_all<P: AsRef<Path>>(p: P) -> io::Result<()> {
    let mut last_err = None;
    for _ in 0..10 {
        if let Err(err) = fs::create_dir_all(&p) {
            last_err = Some(err);
            ::std::thread::sleep(Duration::from_millis(500));
        } else {
            return Ok(());
        }
    }
    Err(last_err.unwrap())
}

#[cfg(all(feature = "to", feature = "feature_capable"))]
pub fn is_same_file(file1: &Path, file2: &Path) -> Result<bool, std::io::Error> {
    use std::io::BufReader;

    let f1 = File::open(file1)?;
    let f2 = File::open(file2)?;

    // Check if file sizes are different
    if f1.metadata().unwrap().len() != f2.metadata().unwrap().len() {
        return Ok(false);
    }

    // Use buf readers since they are much faster
    let f1 = BufReader::new(f1);
    let f2 = BufReader::new(f2);

    // Do a byte to byte comparison of the two files
    for (b1, b2) in f1.bytes().zip(f2.bytes()) {
        if b1.unwrap() != b2.unwrap() {
            return Ok(false);
        }
    }

    return Ok(true);
}
