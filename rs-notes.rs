extern crate getopts;
extern crate serialize;

use std::os;
use std::os::homedir;
use std::io::{File, Open, ReadWrite, Truncate};
use std::io::fs;
use getopts::{optopt, optflag, getopts, Opt};
use serialize::json;
use serialize::json::{Json, Decoder};

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
    let mut db_path = match homedir() {
      Some(x) => x,
      None => fail!("Failed to get home directory.")
    };
    db_path.push(".rs-notes");

    println!("path: {}", db_path.display());

    let db = if db_path.exists() {
      match File::open_mode(&db_path, Open, ReadWrite) {
        Ok(x) => x,
        Err(e) => fail!("Failed to open db. {}", e)
      }
    } else {
      match File::open_mode(&db_path, Truncate, ReadWrite) {
        Ok(x) => x,
        Err(e) => fail!("Failed to create db. {}", e)
      }
    };

    NoteDB {
      db: db,
      path: db_path,
      notes: ~[]
    }
  }

  fn prepare(& mut self) {
    let json_notes = match self.db.read_to_str() {
      Ok(f) => f,
      Err(e) => fail!("Failed to read db. {}", e)
    };

    println!("notes: {}", json_notes);

    let parsed_notes = match json::from_str(json_notes) {
      Ok(x) => x,
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
      Ok(x) => x,
      Err(e) => fail!("Failed to open db. {}", e)
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
    //println!("notes {:?}", self.notes);

    for n in range(0, self.notes.len()) {
      //println!("{:?}, {}, {}", self.notes[n], n, self.notes.len());
      notes_in_json.push_str(self.notes[n].to_json());

      if n != self.notes.len() - 1 {
        notes_in_json.push_str(~",");
      }
    }

    notes_in_json + "]"
  }

  fn list(&self) {
    for note in self.notes.iter() {
      println!("{0}: {1}", note.id, note.description);
    }
  }
}

fn print_help(program: &str) {
  println!("Usage {} [options]", program);
  println!("-h --help\t\tUsage");
  println!("-a --add NAME\t\tAdd note with NAME");
  println!("-d --delete NAME\t\tDelete note by NAME");
}

fn main() {
  let args = os::args();
  println!("started!!!");
  
  let mut db = NoteDB::new();

  db.prepare();

  // just tests
  db.add_note();
  db.add_note();
  db.add_note();
  db.list();
  db.save_and_close();

  let commands = ~[
    optflag("h", "help", "print help"),
    optopt("a", "add", "add note", "NAME"),
    optopt("d", "delete", "delete note", "NAME")
  ];

  let matches = match getopts(args.tail(), commands) {
    Ok(m) => m,
    Err(f) => fail!(f.to_err_msg())
  };

  if matches.opt_present("h") {
    print_help(args[0].clone());
  }
}
