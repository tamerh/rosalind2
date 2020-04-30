use rosalind2::graph::bf;
use rosalind2::graph::cte;
use rosalind2::graph::dij;
use rosalind2::graph::nwc;
use rosalind2::graph::sdag;
use rosalind2::graph::ts;
use rosalind2::graph::bfs;

fn main() {
    bf::solve();
    dij::solve();
    cte::solve();
    bfs::solve();
}
