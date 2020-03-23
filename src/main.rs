use bio::io::fasta;
use rosalind2::combinatorics::cat;
use rosalind2::combinatorics::inod;
use rosalind2::combinatorics::mmch;
use rosalind2::combinatorics::motz;
use rosalind2::string::kmer;
use rosalind2::string::kmp;
use rosalind2::string::lcsq;
use rosalind2::string::lexv;
use rosalind2::string::scsp;
fn main() {
    lcsq::solve();
    scsp::solve();
    inod::solve();
}
