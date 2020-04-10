use bio::io::fasta;
use rosalind2::alignment::edta;
use rosalind2::alignment::glob;
use rosalind2::alignment::ksim;
use rosalind2::alignment::loca;
use rosalind2::alignment::sims;
use rosalind2::combinatorics::cat;
use rosalind2::combinatorics::mmch;
use rosalind2::combinatorics::motz;
use rosalind2::dynamicp::lcsq;
use rosalind2::graph::inod;
use rosalind2::graph::lrep;
use rosalind2::graph::suff;
use rosalind2::graph::trie;
use rosalind2::string::edit;
use rosalind2::string::itwv;
use rosalind2::string::kmer;
use rosalind2::string::kmp;
use rosalind2::string::lcsq as recursive_lcsq;
use rosalind2::string::lexv;
use rosalind2::string::ling;
use rosalind2::string::mrep;
use rosalind2::string::scsp;

fn main() {
    //loca::solve();
    //glob::solve();
    ksim::solve();
}
