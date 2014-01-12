#[crate_id = "rs-notes#0.0.1"];

extern mod extra;

use std::os;
use std::os::homedir;
use std::io::File;
use std::io::fs;
use extra::getopts::{optopt, optflag, getopts, Opt};
use extra::json;
use std::result;
use std::str;

struct Note {
  name: ~str,
  description: ~str,
  status: bool
}

struct NoteDB {
  path: Path,
  notes: ~[Note],
}

impl Note {
  fn to_json(&self) -> ~str {
    ~r#"{"name": &self.name, "description": &self.description}"#
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
      fail!("corrupted db! Resetting...");
      self.reset();
    }
  }

  fn reset(&self) {
    fs::unlink(&self.path);
    let mut file = File::create(&self.path);
    file.write_str("{}");
  }

  fn add_note(&self) {
    let note = Note {
      name: ~"test",
      description: ~"test",
      status: false
    };

    println!("{:?}", note.to_json());
    (~self.notes).push(note);
    //let mut db = File::open();
  }

  fn save_and_close(&self) {
    let notes = ~self.notes;

    for &note in notes.iter() {
      println!("note: {:?}", note);
    }
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
  
  let db = NoteDB::new();
  db.prepare();

  db.save_and_close();
  db.add_note();

  //println!("hmmm {:?}", args.tail());

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