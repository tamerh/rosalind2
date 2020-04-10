use prettytable::{Cell, Row, Table};
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn print_table(seq1: &str, seq2: &str, dyna_table: &Vec<Vec<i32>>) {
  // Create the table
  let mut table = Table::new();
  let mut header_cells = Vec::<Cell>::new();
  header_cells.push(Cell::new(""));
  header_cells.push(Cell::new("-"));
  for j in 0..seq2.len() {
    header_cells.push(Cell::new(&seq2[j..j + 1]));
  }
  table.add_row(Row::new(header_cells));
  for i in 0..seq1.len() + 1 {
    let mut cells = Vec::<Cell>::new();
    if i == 0 {
      cells.push(Cell::new("-"));
    } else {
      cells.push(Cell::new(&seq1[i - 1..i]));
    }
    for j in 0..seq2.len() + 1 {
      cells.push(Cell::new(&dyna_table[i][j].to_string()));
    }
    table.add_row(Row::new(cells));
  }
  table.printstd();
}

pub fn print_table_tuple(seq1: &str, seq2: &str, dyna_table: &Vec<Vec<(i32, &str)>>, file: bool) {
  // Create the table
  let mut table = Table::new();
  let mut header_cells = Vec::<Cell>::new();
  header_cells.push(Cell::new(""));
  header_cells.push(Cell::new("-"));
  for j in 0..seq2.len() {
    header_cells.push(Cell::new(&seq2[j..j + 1]));
  }
  table.add_row(Row::new(header_cells));
  for i in 0..seq1.len() + 1 {
    let mut cells = Vec::<Cell>::new();
    if i == 0 {
      cells.push(Cell::new("-"));
    } else {
      cells.push(Cell::new(&seq1[i - 1..i]));
    }
    for j in 0..seq2.len() + 1 {
      let mut s = "".to_owned();
      s.push_str(&dyna_table[i][j].0.to_string());
      s.push_str(&dyna_table[i][j].1);
      cells.push(Cell::new(&s));
    }
    table.add_row(Row::new(cells));
  }
  if file {
    let mut file = std::fs::File::create("table.txt").expect("create failed");
    let mut f = BufWriter::new(file);
    table.print(&mut f);
  } else {
    table.printstd();
  }
}
