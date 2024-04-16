use bxes::read::single_file_bxes_reader::read_bxes;

pub fn main() {
    let log = read_bxes("/Users/aero/Programming/pmide/bxes/output/procfiler_logs/loh_allocations_75/loh_allocations_75.bxes").ok().unwrap();
}
