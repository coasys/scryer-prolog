fn main() {
    use scryer_prolog::atom_table::Atom;
    use scryer_prolog::*;
    use std::sync::atomic::Ordering;

    ctrlc::set_handler(move || {
        scryer_prolog::machine::INTERRUPT.store(true, Ordering::Relaxed);
    })
    .unwrap();

    let rt = tokio::runtime::Builder::new_multi_thread()
        .build()
        .expect("Failed to create Tokio runtime");
    let _guard = rt.enter();
    rt.block_on(async move {
        let mut wam = machine::Machine::new(Default::default());
        wam.run_top_level(atom!("$toplevel"), (atom!("$repl"), 1));
    });
}
