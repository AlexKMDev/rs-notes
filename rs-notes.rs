#[crate_id = "rs-notes#0.0.1"];

extern mod extra;

use std::os;
use std::os::homedir;
use std::io::{File, io_error, Open, ReadWrite, Truncate};
use std::io::fs;
use extra::getopts::{optopt, optflag, getopts, Opt};
use extra::json;
use extra::json::{Json, Decoder};

struct Note {
  id: uint,
  description: ~str,
  status: bool
}

struct NoteDB {
  db: File,
  path: Path,
  notes: ~[Note],
}

impl Note {
  fn to_json(&self) -> ~str {
    "{\"id\": " + self.id.to_str() + ", \"description\": \"" + self.description + "\"}"
  }
}

impl NoteDB {
  fn new() -> NoteDB {
    let mut db_path = homedir().unwrap();
    db_path.push(".rs-notes");

    println!("path: {}", db_path.display());
    let db = if db_path.exists() {
      match File::open_mode(&db_path, Open, ReadWrite) {
        Some(x) => x,
        None => {
          fail!("Failed to open db.")
        }
      }
    } else {
      match File::open_mode(&db_path, Truncate, ReadWrite) {
        Some(x) => x,
        None => {
          fail!("Failed to create db.")
        }
      }
    };

    NoteDB {
      db: db,
      path: db_path,
      notes: ~[]
    }
  }

  fn prepare(& mut self) {
    let json_notes = self.db.read_to_str();

    println!("decode: {:?}", json::from_str(json_notes).unwrap());
    let parsed_notes = match json::from_str(json_notes) {
      Ok(x) => {
        x
      },
      Err(e) => {
        self.reset();
        fail!("Failed to parse db");
      }
    };

    //println!("pretty: {:?}", parsed_notes.to_pretty_str());
    //let decoder = Decoder::new(parsed_notes);
  }

  fn reset(& mut self) {
    self.truncate();
    self.db.write_str("{}");
  }

  fn truncate(& mut self) {
    self.db = match File::open_mode(&self.path, Truncate, ReadWrite) {
      Some(x) => x,
      None => {
        fail!("Failed to open db.");
      }
    };
  }

  fn add_note(& mut self) -> uint {
    let note_id = self.return_next_id();

    println!("next id: {}", note_id);
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
    self.truncate();
    let notes = self.to_json();

    self.db.write_str(notes);
  }

  fn to_json(& mut self) -> ~str {
    let mut notes_in_json = ~"[";
    
    for note in self.notes.iter() {
      notes_in_json = notes_in_json + note.to_json();
    }

    notes_in_json + "]"
  }

  fn list(&self) {
    for note in self.notes.iter() {
      println!("{}: {}", note.id, note.description);
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
  
  let mut db = NoteDB::new();
  db.prepare();

  // just tests
  db.add_note();
  //db.add_note();
  db.list();
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
