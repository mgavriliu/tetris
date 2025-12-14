use serde::{Deserialize, Serialize};
use std::ffi::{CStr, CString};
use std::fs;
use std::os::raw::c_char;
use std::path::Path;

const MAX_SCORES: usize = 100;
const TOP_SCORES: usize = 10;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Score {
    pub name: String,
    pub score: u32,
    pub level: u32,
    pub lines: u32,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct ScoreStore {
    scores: Vec<Score>,
}

impl ScoreStore {
    fn new() -> Self {
        Self { scores: Vec::new() }
    }

    fn load(path: &Path) -> Self {
        match fs::read_to_string(path) {
            Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| Self::new()),
            Err(_) => Self::new(),
        }
    }

    fn save(&self, path: &Path) -> Result<(), std::io::Error> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let data = serde_json::to_string_pretty(&self)?;
        fs::write(path, data)
    }

    fn add_score(&mut self, score: Score) {
        self.scores.push(score);
        self.scores.sort_by(|a, b| b.score.cmp(&a.score));
        self.scores.truncate(MAX_SCORES);
    }

    fn get_top(&self) -> Vec<&Score> {
        self.scores.iter().take(TOP_SCORES).collect()
    }
}

// FFI exports for Deno

static mut DATA_PATH: Option<String> = None;

/// Initialize the score store with a data file path
/// Returns 0 on success, -1 on failure
#[no_mangle]
pub extern "C" fn scores_init(path: *const c_char) -> i32 {
    if path.is_null() {
        return -1;
    }

    let path_str = unsafe {
        match CStr::from_ptr(path).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return -1,
        }
    };

    unsafe {
        DATA_PATH = Some(path_str);
    }

    0
}

/// Get top scores as JSON
/// Caller must free the returned string with scores_free_string
#[no_mangle]
pub extern "C" fn scores_get_top() -> *mut c_char {
    let path = match unsafe { &DATA_PATH } {
        Some(p) => Path::new(p),
        None => return std::ptr::null_mut(),
    };

    let store = ScoreStore::load(path);
    let top = store.get_top();

    match serde_json::to_string(&top) {
        Ok(json) => match CString::new(json) {
            Ok(cstr) => cstr.into_raw(),
            Err(_) => std::ptr::null_mut(),
        },
        Err(_) => std::ptr::null_mut(),
    }
}

/// Add a new score
/// score_json should be a JSON object with name, score, level, lines fields
/// Returns 0 on success, -1 on failure
#[no_mangle]
pub extern "C" fn scores_add(score_json: *const c_char) -> i32 {
    if score_json.is_null() {
        return -1;
    }

    let path = match unsafe { &DATA_PATH } {
        Some(p) => Path::new(p),
        None => return -1,
    };

    let json_str = unsafe {
        match CStr::from_ptr(score_json).to_str() {
            Ok(s) => s,
            Err(_) => return -1,
        }
    };

    #[derive(Deserialize)]
    struct ScoreInput {
        name: String,
        score: u32,
        level: u32,
        lines: u32,
    }

    let input: ScoreInput = match serde_json::from_str(json_str) {
        Ok(s) => s,
        Err(_) => return -1,
    };

    let score = Score {
        name: input.name.chars().take(20).collect(),
        score: input.score,
        level: input.level,
        lines: input.lines,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0),
    };

    let mut store = ScoreStore::load(path);
    store.add_score(score);

    match store.save(path) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

/// Free a string returned by this library
#[no_mangle]
pub extern "C" fn scores_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            drop(CString::from_raw(ptr));
        }
    }
}

/// Check if a score qualifies for top 10
/// Returns 1 if qualifies, 0 if not, -1 on error
#[no_mangle]
pub extern "C" fn scores_qualifies(score: u32) -> i32 {
    let path = match unsafe { &DATA_PATH } {
        Some(p) => Path::new(p),
        None => return -1,
    };

    let store = ScoreStore::load(path);

    if store.scores.len() < TOP_SCORES {
        return 1;
    }

    match store.scores.get(TOP_SCORES - 1) {
        Some(lowest) => {
            if score > lowest.score {
                1
            } else {
                0
            }
        }
        None => 1,
    }
}
