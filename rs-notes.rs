extern crate getopts;
extern crate serialize;
extern crate collections;

use std::os;
use std::os::homedir;
use std::io::{File, Open, ReadWrite, Truncate};
use getopts::{optopt, optflag, getopts};
use serialize::{json, Decodable, Encodable};
use serialize::json::{Json, ToJson};
use collections::TreeMap;

#[deriving(Decodable, Encodable)]
pub struct Note {
  id: uint,
  description: ~str
}

#[deriving(Decodable, Encodable)]
pub struct Notes {
  data: ~[Note]
}

pub struct NoteDB {
  db: File,
  path: Path,
  notes: Notes
}

impl ToJson for Note {
  fn to_json(&self) -> json::Json {
    let mut d = ~TreeMap::new();

    d.insert(~"id", self.id.to_json());
    d.insert(~"description", self.description.to_json());

    json::Object(d)
  }
}

impl ToJson for Notes {
  fn to_json(&self) -> json::Json {
    let mut d = ~TreeMap::new();

    d.insert(~"data", self.data.to_json());

    json::Object(d)
  }
}

impl NoteDB {
  fn new() -> NoteDB {
    let mut db_path = match homedir() {
      Some(x) => x,
      None => fail!("error: failed to determine home directory.")
    };
    db_path.push(".rs-notes");

    let db = if db_path.exists() {
      match File::open_mode(&db_path, Open, ReadWrite) {
        Ok(x) => x,
        Err(e) => fail!("error: failed to open db, {}", e)
      }
    } else {
      match File::open_mode(&db_path, Truncate, ReadWrite) {
        Ok(x) => x,
        Err(e) => fail!("error: failed to create db, {}", e)
      }
    };

    NoteDB {
      db: db,
      path: db_path,
      notes: Notes { data: ~[] }
    }
  }

  fn load_notes(& mut self) {
    self.check();

    let raw_notes = match self.db.read_to_str() {
      Ok(x) => x,
      Err(e) => fail!("error: failed to read notes, {}", e)
    };

    let notes_in_json = match json::from_str(raw_notes) {
      Ok(x) => x,
      Err(e) => fail!("error: failed to parse raw notes, {}", e)
    };

    let mut decoder = json::Decoder::new(notes_in_json);
    self.notes = match Decodable::decode(&mut decoder) {
      Ok(v) => v,
      Err(e) => fail!("error: failed to convert string to json object, {}", e)
    };
  }

  fn truncate(& mut self) {
    self.db = match File::open_mode(&self.path, Truncate, ReadWrite) {
      Ok(x) => x,
      Err(e) => fail!("Failed to open db. {}", e)
    };
  }

  fn reopen(& mut self) {
    self.db = match File::open_mode(&self.path, Open, ReadWrite) {
      Ok(x) => x,
      Err(e) => fail!("error: failed to open db, {}", e)
    };
  }

  fn check(& mut self) {
    let data = match self.db.read_to_str() {
      Ok(x) => x,
      Err(e) => fail!("error: failed to read notes, {}", e)
    };

    if data.len() == 0 {
      self.save();
    }

    self.reopen();
  }

  fn add_note(& mut self, text: ~str) {
    let note_id = self.return_next_id();

    let note = Note {
      id: note_id,
      description: text
    };

    self.notes.data.push(note);

    println!("info: added note with id {}", note_id);
  }

  fn delete_at(& mut self, id: uint) {
    match self.notes.data.remove(id - 1) {
      Some(x) => {
        println!("info: deleted note at {} id", id);
        x
      },
      None => fail!("error: failed to delete note.")
    };
  }

  fn return_next_id(&self) -> uint {
    match self.notes.data.last() {
      Some(x) => x.id + 1,
      None => 0
    }
  }

  fn save(& mut self) {
    self.truncate();
    let notes = self.notes.to_json().to_str();

    match self.db.write_str(notes) {
      Ok(x) => x,
      Err(e) => fail!("error: failed to save database changes, {}", e)
    };

    self.reopen();
  }

  fn list(&self) {
    if self.notes.data.len() == 0 {
      println!("info: no notes found. Try --help");
    }

    for note in self.notes.data.iter() {
      println!("{0}: {1}", note.id, note.description);
    }
  }
}

fn print_help(program: &str) {
  println!("Usage {} [options]", program);
  println!("-h --help\t\tUsage");
  println!("-l --list\t\tList notes");
  println!("-a --add NAME\t\tAdd note with NAME");
  println!("-d --delete ID\t\tDelete note by ID");
}

fn main() {
  let args = os::args();
  let mut db = NoteDB::new();
  db.load_notes();

  let commands = ~[
    optflag("h", "help", "print help"),
    optopt("a", "add", "add note", "NAME"),
    optopt("d", "delete", "delete note", "NAME"),
    optflag("l", "list", "list notes"),
    optflag("r", "reset", "reset notes database")
  ];

  let matches = match getopts(args.tail(), commands) {
    Ok(m) => m,
    Err(f) => fail!(f.to_err_msg())
  };

  if matches.opt_present("h") {
    print_help(args[0].clone());
  }

  if matches.opt_present("l") {
    db.list();
  }

  if matches.opt_present("a") {
    match matches.opt_str("a") {
      Some(x) => db.add_note(x),
      None => fail!("Failed to add note.")
    };
  }

  if matches.opt_present("d") {
    match matches.opt_str("d") {
      Some(x) => {
        match from_str::<uint>(x) {
          Some(x) => db.delete_at(x),
          None => fail!("Failed to convert string to integer.")
        }
      },
      None => fail!("Failed to delete note.")
    };
  }

  if matches.opt_present("r") {
    db.check();
  }

  db.save();
}
