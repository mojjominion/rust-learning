#![allow(unused_variables, dead_code)]
mod exercises;

fn main() {
    // matrix_transpose::run_transpose();
    // matrix_transpose::run_transpose_generic();
    // lifetime::life_time();
    // lifetime::life_time_different_scope();

    // exercises::library::library_runner();
    // exercises::health_stats::health_stats();
    // exercises::gui_library::run();
    // exercises::safe_ffi::run().expect("msg");
    // exercises::dining_philosophers::run();
    exercises::link_checker::run_parallel();
}
