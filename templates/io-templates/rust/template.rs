/*

Github (Feel Free to add a star): https://github.com/Sharp-Software-Engineering/Competitive-Programmimng
Rust template: https://github.com/Sharp-Software-Engineering/Competitive-Programmimng/templates/io-templates/rust/template.rs

Content Posts (after the contest) Follow: https://www.youtube.com/@SharpSoftwareEngineering

Repository contents free to modify and use.

*/

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]

use std::io::{self, Read, Write};
use std::collections::{HashSet, HashMap, VecDeque, BinaryHeap};

type ll = i64;
type int = i32;
type uint = usize;
type float = f64;

type set<T> = HashSet<T>;
type dict<K, V> = HashMap<K, V>;
type deque<T> = VecDeque<T>;
type priority<T> = BinaryHeap<T>;

const MOD: uint = 1_000_000_007;

static mut MIO: Option<Mio> = None;

struct Mio {
  lines: deque<String>,
  line: deque<String>,
  output: String,
}

impl Mio {
  fn new() -> Self {
    Mio {
      lines: deque::new(),
      line: deque::new(),
      output: String::new(),
    }
  }

  fn input(&mut self) {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let lines: deque<String> = input.lines().map(String::from).collect();

    self.lines = lines;

  }
  // Used for interactive inputs.
  fn interin(&mut self) -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    String::from(input.trim())
  }

  fn gline(&mut self) -> String {
    if self.lines.is_empty() {
      panic!("No more lines to read");
    }
    self.lines.pop_front().expect("Failed to get next")
  }

  fn rline(&mut self) -> deque<String> {
    self.gline().split_whitespace().map(String::from).collect()
  }

  fn eline(&mut self) {
    if self.line.len()==0{
      self.line = self.rline();
    }
  }

  fn nint(&mut self) -> int {
    self.eline();
    self.line.pop_front().expect("Failed to get next").parse().expect("Failed to parse int")
  }

  fn nll(&mut self) -> ll {
    self.eline();
    self.line.pop_front().expect("Failed to get next").parse().expect("Failed to parse int")
  }

  fn nuint(&mut self) -> uint {
    self.eline();
    self.line.pop_front().expect("Failed to get next").parse().expect("Failed to parse int")
  }

  fn nstring(&mut self) -> String {
    self.eline();
    self.line.pop_front().expect("Failed to get next")
  }

  fn rint(&mut self) -> int {
    self.gline().trim().parse().expect("Failed to parse int")
  }

  fn rll(&mut self) -> ll {
    self.gline().trim().parse().expect("Failed to parse int")
  }

  fn ruint(&mut self) -> uint {
    self.gline().trim().parse().expect("Failed to parse usize")
  }

  fn rintv(&mut self) -> Vec<int> {
    self.gline()
      .trim()
      .split_whitespace()
      .map(|s| s.parse().expect("Failed to parse int"))
      .collect()
  }

  fn rllv(&mut self) -> Vec<ll> {
    self.gline()
      .trim()
      .split_whitespace()
      .map(|s| s.parse().expect("Failed to parse int"))
      .collect()
  }

  fn ruintv(&mut self) -> Vec<uint> {
    self.gline()
      .trim()
      .split_whitespace()
      .map(|s| s.parse().expect("Failed to parse int"))
      .collect()
  }

  fn rstring(&mut self) -> String {
    self.gline().to_string()
  }

  fn rstringv(&mut self) -> Vec<String> {
    self.gline()
      .trim()
      .split_whitespace()
      .map(String::from)
      .collect()
  }

  fn out<T>(&mut self, my_str: T)
  where T: ToString {
    self.output.push_str(&format!("{}\n", my_str.to_string()));
  }

  fn printall(&self) {
    io::stdout()
      .write_all(self.output.as_bytes())
      .expect("Failed to write output");
  }
  // Used to flush interactive outputs.
  fn flush(&mut self) {
    self.printall();
    io::stdout().flush().expect("Failed to flush output");
    self.clear();
  }
  fn clear(&mut self) {
    self.output = String::new();
  }
}

fn init_mio() {
  unsafe {
    if MIO.is_none() {
      MIO = Some(Mio::new());
    }
  }
}

fn with_mio<T>(f: impl FnOnce(&mut Mio) -> T) -> T {
  unsafe { f(MIO.as_mut().expect("Mio not initialized")) }
}

fn input() {
  with_mio(|mio| mio.input())
}

fn nint() -> int {
  with_mio(|mio| mio.nint())
}

fn nuint() -> uint{
  with_mio(|mio| mio.nuint())
}

fn nll() -> ll {
  with_mio(|mio| mio.nll())
}

fn rint() -> int {
  with_mio(|mio| mio.rint())
}

fn rll() -> ll {
  with_mio(|mio| mio.rll())
}

fn ruint() -> uint {
  with_mio(|mio| mio.ruint())
}

fn rintv() -> Vec<int> {
  with_mio(|mio| mio.rintv())
}

fn rllv() -> Vec<ll> {
  with_mio(|mio| mio.rllv())
}

fn ruintv() -> Vec<uint> {
  with_mio(|mio| mio.ruintv())
}

fn rstring() -> String {
  with_mio(|mio| mio.rstring())
}

fn rstringv() -> Vec<String> {
  with_mio(|mio| mio.rstringv())
}

fn out<T>(my_str: T) 
where
  T: ToString,
{
  with_mio(|mio| mio.out(my_str));
}

fn outv<T>(vec: Vec<T>)
where
    T: ToString,  // T must implement ToString
{
  out(vec.into_iter()
    .map(|item| item.to_string())
    .collect::<Vec<String>>()
    .join(" "));
}

fn outvl<T>(vec: Vec<T>)
where
  T: ToString,
{
  out(vec.into_iter()
    .map(|item| item.to_string())
    .collect::<Vec<String>>()
    .join("\n"));
}

fn printall() {
    with_mio(|mio| mio.printall())
}

fn flush() {
  with_mio(|mio| mio.flush())
}

fn clear() {
  with_mio(|mio| mio.clear())
}

fn interin() -> String{
  with_mio(|mio| mio.interin())
}

// --- Main Function ---

fn main(){
  init_mio();
  input();

  let mut tt = rint();
  for _ in 0..tt {
    let n = ruint();
    out(n);
  }

  printall();
}