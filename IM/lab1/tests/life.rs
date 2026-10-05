//! Игра «Жизнь»: правило «Хищник и добыча» и замкнутое поле.

use lab1::sim::{Life, Species};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

fn world(cells: &[(usize, usize)], w: usize, h: usize) -> Life {
    let mut life = Life::new(w, h);
    for &(x, y) in cells {
        life.set_alive(x, y, true);
    }
    life
}

/// Sorted live coordinates, the shape of the world.
fn alive(life: &Life) -> Vec<(usize, usize)> {
    let mut v: Vec<(usize, usize)> = (0..life.grid.height)
        .flat_map(|y| (0..life.grid.width).map(move |x| (x, y)))
        .filter(|&(x, y)| life.grid.get(x, y).is_alive())
        .collect();
    v.sort_unstable();
    v
}

#[test]
fn a_block_is_a_still_life() {
    let mut life = world(&[(4, 4), (5, 4), (4, 5), (5, 5)], 12, 12);
    life.step();
    assert_eq!(alive(&life), vec![(4, 4), (4, 5), (5, 4), (5, 5)]);
    assert_eq!(life.generation, 1);
    assert_eq!(life.alive, 4);
}

#[test]
fn a_blinker_toggles_between_horizontal_and_vertical() {
    let mut life = world(&[(4, 4), (5, 4), (6, 4)], 12, 12);
    life.step();
    assert_eq!(alive(&life), vec![(5, 3), (5, 4), (5, 5)]);
    life.step();
    assert_eq!(alive(&life), vec![(4, 4), (5, 4), (6, 4)]);
    assert_eq!(life.generation, 2);
}

#[test]
fn a_cell_with_fewer_than_two_neighbours_dies() {
    let mut life = world(&[(7, 7)], 15, 15);
    life.step();
    assert_eq!(alive(&life), Vec::<(usize, usize)>::new());
    assert_eq!(life.alive, 0);
    assert_eq!(life.generation, 1);
}

#[test]
fn a_live_cell_with_two_or_three_neighbours_survives() {
    // Exactly two: the cell keeps its place.
    let mut two = world(&[(4, 4), (3, 4), (5, 4)], 9, 9);
    two.step();
    assert!(two.grid.get(4, 4).is_alive());

    // Exactly three: it survives as well, and a neighbour is born.
    let mut three = world(&[(4, 4), (3, 4), (5, 4), (4, 3)], 9, 9);
    three.step();
    assert!(three.grid.get(4, 4).is_alive());
    assert!(three.grid.get(4, 5).is_alive());
}

#[test]
fn a_live_cell_with_four_neighbours_dies_of_overpopulation() {
    let mut life = world(&[(4, 4), (3, 4), (5, 4), (4, 3), (4, 5)], 9, 9);
    life.step();
    assert!(!life.grid.get(4, 4).is_alive());
}

#[test]
fn a_dead_cell_is_born_only_with_exactly_three_neighbours() {
    // Three neighbours: born.
    let mut born = world(&[(5, 4), (5, 5), (6, 5)], 9, 9);
    born.step();
    assert!(born.grid.get(6, 4).is_alive());

    // Two neighbours: stays dead.
    let mut two = world(&[(7, 7), (7, 8)], 11, 11);
    two.step();
    assert!(!two.grid.get(8, 7).is_alive());

    // Four neighbours: stays dead.
    let mut four = world(&[(7, 7), (7, 8), (9, 7), (8, 8)], 11, 11);
    four.step();
    assert!(!four.grid.get(8, 7).is_alive());
}

#[test]
fn a_blinker_straddling_the_seam_wraps_around_the_torus() {
    // Three in a row across the left/right edge of an 8x10 field.
    let mut life = world(&[(7, 5), (0, 5), (1, 5)], 8, 10);
    life.step();
    assert_eq!(alive(&life), vec![(0, 4), (0, 5), (0, 6)]);
}

#[test]
fn a_glider_crosses_the_seam_and_returns_after_a_full_lap() {
    // On a 16x16 torus a glider shifts by one cell every four generations, so
    // after 4 * 16 steps it must sit exactly where it started.
    let start = world(&[(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)], 16, 16);
    let initial = alive(&start);
    let mut life = start;
    let mut crossed = false;
    for _ in 0..64 {
        life.step();
        assert_eq!(life.alive, 5, "a glider never gains or loses cells");
        if alive(&life).iter().any(|&(x, _)| x == 15) {
            crossed = true;
        }
    }
    assert!(crossed, "the glider should travel through the far edge");
    assert_eq!(alive(&life), initial);
}

#[test]
fn step_advances_the_generation_even_on_an_empty_world() {
    let mut life = Life::new(10, 10);
    life.step();
    assert_eq!(life.generation, 1);
    assert_eq!(life.alive, 0);
    assert!(life.grid.cells.iter().all(|c| !c.is_alive()));
}

#[test]
fn clear_kills_everything_and_resets_the_generation() {
    let mut life = world(&[(1, 1), (2, 2), (3, 3)], 8, 8);
    life.step();
    life.clear();
    assert_eq!(life.generation, 0);
    assert_eq!(life.alive, 0);
    assert!(life.grid.cells.iter().all(|c| !c.is_alive()));
}

#[test]
fn set_alive_keeps_the_counter_in_sync() {
    let mut life = Life::new(8, 8);
    life.set_alive(2, 3, true);
    life.set_alive(2, 3, true); // no double count
    assert_eq!(life.alive, 1);
    life.set_alive(4, 4, false); // killing a dead cell counts nothing
    assert_eq!(life.alive, 1);
    life.set_alive(2, 3, false);
    assert_eq!(life.alive, 0);
    assert!(!life.grid.get(2, 3).is_alive());
}

#[test]
fn fill_is_reproducible_from_a_seed() {
    let mut a = Life::new(20, 20);
    let mut b = Life::new(20, 20);
    a.fill(0.4, &mut ChaCha8Rng::seed_from_u64(7));
    b.fill(0.4, &mut ChaCha8Rng::seed_from_u64(7));
    assert_eq!(a.grid.cells, b.grid.cells);
    assert_eq!(a.generation, 0);
    assert_eq!(
        a.alive,
        a.grid.cells.iter().filter(|c| c.is_alive()).count()
    );
}

#[test]
fn fill_honours_the_density_bounds() {
    let mut empty = Life::new(12, 12);
    empty.fill(0.0, &mut ChaCha8Rng::seed_from_u64(1));
    assert_eq!(empty.alive, 0);

    let mut full = Life::new(12, 12);
    full.fill(1.0, &mut ChaCha8Rng::seed_from_u64(1));
    assert_eq!(full.alive, 144);
}

#[test]
fn resize_recenters_the_world_and_recounts_the_live_cells() {
    let mut life = world(&[(0, 0), (1, 1), (4, 4)], 6, 6);
    life.resize(4, 4);
    // The 6x6 world is cropped to its centre: column and row 0 and 5 are gone.
    assert_eq!(alive(&life), vec![(0, 0), (3, 3)]);
    assert_eq!(life.alive, 2);
    assert!(life.cycle.is_none(), "resizing starts a fresh watch");
}

// ---------------------------------------------------------- циклы (детекция)

#[test]
fn a_repeating_world_reports_the_length_of_its_cycle() {
    let mut life = world(&[(4, 4), (5, 4), (6, 4)], 12, 12);
    life.step();
    assert!(life.cycle.is_none(), "the world has not repeated yet");
    life.step();
    assert_eq!(life.cycle, Some(2), "a blinker repeats every 2 generations");
}

#[test]
fn a_still_life_reports_a_cycle_of_one_generation() {
    let mut life = world(&[(4, 4), (5, 4), (4, 5), (5, 5)], 12, 12);
    life.step();
    assert_eq!(life.cycle, Some(1));
}

#[test]
fn an_empty_world_cycles_on_the_first_step() {
    let mut life = Life::new(10, 10);
    life.step();
    assert_eq!(life.cycle, Some(1));
    assert_eq!(life.alive, 0);
}

#[test]
fn any_edit_of_the_world_starts_a_new_watch() {
    let mut life = world(&[(4, 4), (5, 4), (4, 5), (5, 5)], 12, 12);
    life.step();
    assert_eq!(life.cycle, Some(1));

    life.set_alive(9, 9, true);
    assert!(life.cycle.is_none(), "painting forgets the old cycle");

    life.step();
    life.step();
    assert!(life.cycle.is_some());

    life.clear();
    assert!(life.cycle.is_none(), "clearing starts a fresh watch");

    life.fill(0.5, &mut ChaCha8Rng::seed_from_u64(3));
    assert!(life.cycle.is_none(), "refilling starts a fresh watch");
}

#[test]
fn watching_the_same_cycle_again_needs_a_fresh_history() {
    let mut life = world(&[(4, 4), (5, 4), (6, 4)], 12, 12);
    life.step();
    life.step();
    assert_eq!(life.cycle, Some(2));

    let generation = life.generation;
    life.forget_cycles();
    assert!(life.cycle.is_none());
    assert_eq!(life.generation, generation, "the world itself is untouched");

    life.step();
    assert!(life.cycle.is_none());
    life.step();
    assert_eq!(life.cycle, Some(2), "the same cycle is found once more");
}

#[test]
fn every_finite_world_eventually_reports_a_cycle() {
    // A 3x3 world has 3^9 = 19683 distinct states (empty, prey, predator), so
    // by the pigeonhole principle the trajectory must repeat within 19684
    // steps — and the watch must say so.
    let mut life = Life::new(3, 3);
    life.fill(0.5, &mut ChaCha8Rng::seed_from_u64(42));
    for _ in 0..20_000 {
        life.step();
        if life.cycle.is_some() {
            break;
        }
    }
    assert!(life.cycle.is_some(), "no cycle after the whole state space");
}

// --------------------------------------------------- два вида клеток

#[test]
fn a_predator_eats_an_adjacent_prey_and_stays_alive() {
    // Блок из добычи, хищник стоит в N4 к ячейке (5, 4) — та съедается.
    let mut life = world(&[(4, 4), (5, 4), (4, 5), (5, 5)], 12, 12);
    life.set_cell(6, 4, Some(Species::Predator));
    life.step();

    assert!(!life.grid.get(5, 4).is_alive(), "съедена добыча в N4");
    assert_eq!(life.grid.get(6, 4).species, Some(Species::Predator));
    assert_eq!(life.prey, 3);
    assert_eq!(life.predator, 1);
    assert_eq!(life.alive, life.prey + life.predator);
}

#[test]
fn a_lone_predator_starves_and_the_world_dies() {
    let mut life = Life::new(8, 8);
    life.set_cell(3, 3, Some(Species::Predator));
    life.step();
    assert_eq!(life.predator, 0, "без добычи в N4 хищник гибнет");
    assert_eq!(life.alive, 0);
    life.step();
    assert_eq!(life.cycle, Some(1), "пустой мир циклится, колония вымерла");
}

#[test]
fn a_predator_is_born_only_with_a_pack_and_prey_nearby() {
    // Добыча (5,5), хищники (4,5) и (6,5): пустые (5,6) и (5,4) видят обоих
    // хищников в N8 и добычу в N4 — рождаются ещё двое, добыча съедается.
    let mut born = Life::new(11, 11);
    born.set_cell(5, 5, Some(Species::Prey));
    born.set_cell(4, 5, Some(Species::Predator));
    born.set_cell(6, 5, Some(Species::Predator));
    born.step();
    assert_eq!(born.grid.get(5, 6).species, Some(Species::Predator));
    assert_eq!(born.grid.get(5, 4).species, Some(Species::Predator));
    assert_eq!((born.prey, born.predator), (0, 4));

    // Без второй пары хищников рождения нет.
    let mut pairless = Life::new(11, 11);
    pairless.set_cell(5, 5, Some(Species::Prey));
    pairless.set_cell(4, 5, Some(Species::Predator));
    pairless.step();
    assert!(
        !pairless.grid.get(5, 6).is_alive(),
        "стая меньше двух не рожает"
    );

    // Без добычи рядом рождения нет тоже.
    let mut hungry = Life::new(11, 11);
    hungry.set_cell(4, 5, Some(Species::Predator));
    hungry.set_cell(6, 5, Some(Species::Predator));
    hungry.step();
    assert!(
        !hungry.grid.get(5, 6).is_alive(),
        "без добычи рожать незачем"
    );
}

#[test]
fn prey_is_not_born_beside_a_predator() {
    // Три добычи над (5,5) как раз дают B3, но хищник в N4 запрещает
    // рождение — хищник отпугивает размножение.
    let mut life = Life::new(11, 11);
    life.set_cell(4, 4, Some(Species::Prey));
    life.set_cell(5, 4, Some(Species::Prey));
    life.set_cell(6, 4, Some(Species::Prey));
    life.set_cell(5, 6, Some(Species::Predator));
    life.step();
    assert!(
        !life.grid.get(5, 5).is_alive(),
        "рождение рядом с хищником заблокировано"
    );
}

#[test]
fn a_starving_predator_dies_and_the_prey_block_cycles() {
    // Блок + голодный хищник: съедает (5,4), на втором шаге умирает, на
    // третьем блок восстанавливается — и мир зацикливается с длиной 1.
    let mut life = world(&[(4, 4), (5, 4), (4, 5), (5, 5)], 12, 12);
    life.set_cell(6, 4, Some(Species::Predator));

    life.step();
    assert_eq!((life.prey, life.predator), (3, 1), "съел одну, сам сыт");
    assert!(life.cycle.is_none());

    life.step();
    assert_eq!((life.prey, life.predator), (3, 0), "хищник умер от голода");
    assert!(life.cycle.is_none());

    life.step();
    assert_eq!(
        (life.prey, life.predator),
        (4, 0),
        "съеденная клетка родилась заново"
    );
    assert!(life.cycle.is_none());

    life.step();
    assert_eq!(life.cycle, Some(1), "стабильный блок повторяется");
}

#[test]
fn set_cell_paints_a_species_and_starts_a_fresh_watch() {
    let mut life = world(&[(4, 4), (5, 4), (4, 5), (5, 5)], 12, 12);
    life.step();
    assert_eq!(life.cycle, Some(1));

    life.set_cell(8, 8, Some(Species::Predator));
    assert!(life.cycle.is_none(), "правка мира начинает новый отсчёт");
    assert_eq!(life.grid.get(8, 8).species, Some(Species::Predator));
    assert_eq!((life.prey, life.predator, life.alive), (4, 1, 5));

    life.set_cell(8, 8, None);
    assert_eq!((life.prey, life.predator, life.alive), (4, 0, 4));
}

#[test]
fn the_species_counters_always_add_up_to_alive() {
    let mut life = Life::new(16, 16);
    life.fill(0.6, &mut ChaCha8Rng::seed_from_u64(5));
    assert_eq!(life.alive, life.prey + life.predator);

    for _ in 0..10 {
        life.step();
        assert_eq!(life.alive, life.prey + life.predator);
        assert_eq!(
            life.alive,
            life.grid.cells.iter().filter(|c| c.is_alive()).count()
        );
    }

    life.set_cell(0, 0, Some(Species::Predator));
    life.resize(8, 8);
    assert_eq!(life.alive, life.prey + life.predator);
    life.clear();
    assert_eq!((life.alive, life.prey, life.predator), (0, 0, 0));
}

#[test]
fn fill_spawns_both_species_reproducibly() {
    let mut a = Life::new(20, 20);
    let mut b = Life::new(20, 20);
    a.fill(1.0, &mut ChaCha8Rng::seed_from_u64(11));
    b.fill(1.0, &mut ChaCha8Rng::seed_from_u64(11));
    assert_eq!(a.grid.cells, b.grid.cells, "один seed — один мир");
    assert_eq!(a.alive, 400);
    assert!(a.prey > 0, "добыча должна встретиться");
    assert!(a.predator > 0, "хищники должны встретиться");
    assert_eq!(a.prey + a.predator, a.alive);
}

#[test]
fn the_species_colours_match_the_spec() {
    use lab1::ui::palette;
    assert_eq!(palette::PREY, egui::Color32::BLACK);
    assert_eq!(palette::PREDATOR, egui::Color32::from_rgb(0xD3, 0x2F, 0x2F));
}
