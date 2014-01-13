#[crate_id = "rs-notes#0.0.1"];

extern mod extra;

use std::os;
use std::os::homedir;
use std::io::File;
use std::io::fs;
use extra::getopts::{optopt, optflag, getopts, Opt};
use extra::json;

struct Note {
  id: uint,
  description: ~str,
  status: bool
}

struct NoteDB {
  path: Path,
  notes: ~[Note],
}

impl Note {
  fn to_json(&self) -> ~str {
    ~r#"{"id": self.id, "description": self.description}"#
  }
}

impl NoteDB {
  fn new() -> NoteDB {
    let mut db_path = homedir().unwrap();
    db_path.push(".rs-notes");
    
    if !db_path.exists() {
      File::create(&db_path);
    };

    NoteDB {
      path: db_path,
      notes: ~[]
    }
  }

  fn prepare(&self) {
    let mut db = File::open(&self.path);
    let json_notes = db.read_to_str();
    let parsed_notes = json::from_str(json_notes);

    if parsed_notes.is_err() {
      println!("corrupted db! Resetting...");
      self.reset();
    }
  }

  fn reset(&self) {
    fs::unlink(&self.path);
    let mut file = File::create(&self.path);
    file.write_str("{}");
  }

  fn add_note(& mut self) -> uint {
    let note_id = self.return_next_id();
    let note = Note {
      id: note_id,
      description: ~"test",
      status: false
    };

    self.notes.push(note);
    note_id
  }

  fn return_next_id(&self) -> uint {
    self.notes.len()
  }

  fn save_and_close(& mut self) {
    let mut db = File::create(&self.path);
    db.write_str(self.to_json());
    println!("{}", self.to_json());
  }

  fn to_json(& mut self) -> ~str {
    let mut notes_in_json = ~"{[";
    
    for note in self.notes.iter() {
      notes_in_json = notes_in_json + note.to_json();
    }

    notes_in_json + "]}"
  }
}

fn print_help(program: &str) {
  println!("Usage {} [options]", program);
  println("-h --help\t\tUsage");
  println("-a --add <note>\t\tAdd note with <name>");
  println("-d --delete <note>\t\tDelete note");
}

fn main() {
  let args = os::args();
  
  let mut db = NoteDB::new();
  db.prepare();

  // just tests
  db.add_note();
  //db.add_note();
  db.save_and_close();

  let commands = ~[
    optflag("h"),
    optflag("help"),
    optflag("a"),
    optflag("add"),
    optflag("d"),
    optflag("delete")
  ];

  let matches = match getopts(args.tail(), commands) {
    Ok(m) => m,
    Err(f) => {
      fail!(f.to_err_msg());
    }
  };

  if matches.opt_present("h") || matches.opt_present("help") {;
    print_help(args[0].clone());
  }
}
