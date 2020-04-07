use prettytable::{Cell, Row, Table};

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
