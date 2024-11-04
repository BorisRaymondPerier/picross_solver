#[macro_use]
extern crate criterion;

use criterion::measurement::WallTime;
use criterion::{BenchmarkId, BenchmarkGroup, Criterion, Throughput};

extern crate picross_solver;

use crate::picross_solver::display::*;
use crate::picross_solver::solver::*;

fn solve_unit_brut_force(picross_string : & String) {
    let mut picross = PicrossBoard::picross_from_clue_string(picross_string);
    solve_picross_board(&mut picross , LineSolveMethod::BrutForce);
}

fn group_step_brut_force(group : &mut BenchmarkGroup<WallTime>, n : usize, clues : & String) {
    group.throughput(Throughput::Elements(n as u64));
    group.bench_with_input(
        BenchmarkId::new(format!("Picross {}x{}", n, n), clues),
        &clues,
        |b, s| { b.iter(|| solve_unit_brut_force(&s));});
}

fn solve_fixed_bench_brut_force(c: &mut Criterion) {
    let mut group = c.benchmark_group("fixed picross brut force");

    group_step_brut_force(&mut group, 5, & CLUE_STRING_5X5.to_string());
    group_step_brut_force(&mut group, 6, & CLUE_STRING_6X6.to_string());
    group_step_brut_force(&mut group, 7, & CLUE_STRING_7X7.to_string());
    group_step_brut_force(&mut group, 8, & CLUE_STRING_8X8.to_string());
    group_step_brut_force(&mut group, 9, & CLUE_STRING_9X9.to_string());
    group_step_brut_force(&mut group, 10, & CLUE_STRING_10X10.to_string());
    group_step_brut_force(&mut group, 12, & CLUE_STRING_12X12.to_string());
    group_step_brut_force(&mut group, 15, & CLUE_STRING_15X15.to_string());
    group_step_brut_force(&mut group, 16, & CLUE_STRING_16X16.to_string());
    group_step_brut_force(&mut group, 20, & CLUE_STRING_20X20.to_string());
    group.sample_size(10);
    group_step_brut_force(&mut group, 25, & CLUE_STRING_25X25.to_string());
}

fn solve_unit_space_distribution(picross_string : & String) {
    let mut picross = PicrossBoard::picross_from_clue_string(picross_string);
    solve_picross_board(&mut picross , LineSolveMethod::SpaceDistribution);
}

fn group_step_space_distribution(group : &mut BenchmarkGroup<WallTime>, n : usize, clues : & String) {
    group.throughput(Throughput::Elements(n as u64));
    group.bench_with_input(
        BenchmarkId::new(format!("Picross {}x{}", n, n), clues),
        &clues,
        |b, s| { b.iter(|| solve_unit_space_distribution(&s));});
}

fn solve_fixed_bench_space_distribution(c: &mut Criterion) {
    let mut group = c.benchmark_group("fixed picross space distribution");

    group_step_space_distribution(&mut group, 5, & CLUE_STRING_5X5.to_string());
    group_step_space_distribution(&mut group, 6, & CLUE_STRING_6X6.to_string());
    group_step_space_distribution(&mut group, 7, & CLUE_STRING_7X7.to_string());
    group_step_space_distribution(&mut group, 8, & CLUE_STRING_8X8.to_string());
    group_step_space_distribution(&mut group, 9, & CLUE_STRING_9X9.to_string());
    group_step_space_distribution(&mut group, 10, & CLUE_STRING_10X10.to_string());
    group_step_space_distribution(&mut group, 12, & CLUE_STRING_12X12.to_string());
    group_step_space_distribution(&mut group, 15, & CLUE_STRING_15X15.to_string());
    group_step_space_distribution(&mut group, 16, & CLUE_STRING_16X16.to_string());
    group_step_space_distribution(&mut group, 20, & CLUE_STRING_20X20.to_string());
    group.sample_size(10);
    group_step_space_distribution(&mut group, 25, & CLUE_STRING_25X25.to_string());
    group_step_space_distribution(&mut group, 30, & CLUE_STRING_30X30.to_string());
}

criterion_group!(benches, solve_fixed_bench_brut_force, solve_fixed_bench_space_distribution);
criterion_main!(benches);


