use rand::{seq::SliceRandom, thread_rng};
use std::{
    cell::RefCell, mem::swap, sync::{Arc, Mutex}, thread, usize
};

mod bot;
pub use bot::Bot;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Iteration {
    // TODO: come up with a more effective way to protect against multiple updates
    Even,
    Odd,
}

#[derive(Debug)]
pub enum Cell {
    None,
    Bot(Iteration, Bot),
}

pub struct WorldConfig {
    pub h: usize,
    pub w: usize,
    pub thread_cnt: usize,
}

pub struct World {
    cfg: WorldConfig,
    cells: Arc<Vec<Mutex<Vec<RefCell<Cell>>>>>,
    current_iter: Iteration,
}

struct WordAccessor<'a> {
    // TODO: come up with a more efficient way to provide access
    pub cells: [&'a mut Cell; 8],
}

impl World {
    pub fn new(cfg: WorldConfig) -> Self {
        let mut cells = Vec::new();
        for _ in 0..cfg.h {
            let mut row = Vec::new();
            for _ in 0..cfg.w {
                row.push(RefCell::new(Cell::None));
            }
            cells.push(Mutex::new(row));
        }
        *cells[3].lock().unwrap()[3].borrow_mut() = Cell::Bot(Iteration::Even, Bot::new());
        Self {
            cells: Arc::new(cells),
            cfg,
            current_iter: Iteration::Odd,
        }
    }

    pub fn update(&mut self) {
        let mut colums_index = (0..self.cfg.w).collect::<Vec<_>>();
        colums_index.shuffle(&mut thread_rng());
        let colums_index = Arc::new(colums_index);

        let mut rows_index = (0..self.cfg.h).collect::<Vec<_>>();
        rows_index.shuffle(&mut thread_rng());
        let rows_index = Arc::new(rows_index);

        let cells_per_row = colums_index.len();
        let job_count = rows_index.len();
        let imposible_job = job_count;

        let is_completed = Arc::new(Mutex::new(vec![false; self.cfg.h]));
        let first_uncompleted = Arc::new(Mutex::new(0));
        let in_work = Arc::new(Mutex::new(vec![imposible_job; self.cfg.thread_cnt]));

        let current_iter = self.current_iter;
        let next_iter = match current_iter {
            Iteration::Even => Iteration::Odd,
            Iteration::Odd => Iteration::Even,
        };
        self.current_iter = next_iter;

        // let mut threads = Vec::new();
        for ti in 0..self.cfg.thread_cnt {
            let cells = self.cells.clone();
            let jobs = rows_index.clone();
            let is_completed_c = is_completed.clone();
            let first_uncompleted_c = first_uncompleted.clone();
            let in_work_c = in_work.clone();

            web_sys::console::log_1(&format!("spawn {ti}").into());
            // threads.push(thread::spawn(move || {
                let thread_id = 0;//ti;
                loop {
                    let mut found = false;
                    let mut job = 0;
                    // search for free job
                    {
                        let mut is_completed = is_completed_c.lock().unwrap();
                        let mut first_uncompleted = first_uncompleted_c.lock().unwrap();
                        let mut in_work = in_work_c.lock().unwrap();

                        in_work[thread_id] = imposible_job;

                        for i in *first_uncompleted..job_count {
                            let tmp_job = jobs[i] as isize;
                            if !is_completed[i]
                                && !in_work.contains(
                                    &((tmp_job - 2).rem_euclid(job_count as isize) as usize),
                                )
                                && !in_work.contains(
                                    &((tmp_job - 1).rem_euclid(job_count as isize) as usize),
                                )
                                && !in_work
                                    .contains(&((tmp_job as usize + 1).rem_euclid(job_count)))
                                && !in_work
                                    .contains(&((tmp_job as usize + 2).rem_euclid(job_count)))
                            {
                                is_completed[i] = true;
                                in_work[thread_id] = jobs[i];
                                found = true;
                                job = tmp_job;
                                break;
                            }
                        }
                        while *first_uncompleted < job_count && is_completed[*first_uncompleted] {
                            *first_uncompleted = *first_uncompleted + 1;
                        }
                        if !found {
                            // there is no more free work, the thread ends
                            break;
                        }
                    }
                    // do job
                    {
                        let y = job;
                        let prev_rows = cells[(y - 1).rem_euclid(job_count as isize) as usize]
                            .try_lock()
                            .unwrap();
                        let current_row = cells[y.rem_euclid(job_count as isize) as usize]
                            .try_lock()
                            .unwrap();
                        let next_row = cells[(y + 1).rem_euclid(job_count as isize) as usize]
                            .try_lock()
                            .unwrap();
                        for x in 0..cells_per_row {
                            let prev_col =
                                (x as isize - 1).rem_euclid(cells_per_row as isize) as usize;
                            let current_col = x.rem_euclid(cells_per_row) as usize;
                            let next_col = (x + 1).rem_euclid(cells_per_row) as usize;

                            let current_cell = &mut *current_row[current_col].borrow_mut();
                            if let Cell::Bot(iter, b) = &mut *current_cell {
                                if *iter == current_iter {
                                    *iter = next_iter;
                                    let mut wa = WordAccessor {
                                        cells: [
                                            &mut *prev_rows[current_col].borrow_mut(),
                                            &mut *prev_rows[next_col].borrow_mut(),
                                            &mut *current_row[next_col].borrow_mut(),
                                            &mut *next_row[next_col].borrow_mut(),
                                            &mut *next_row[current_col].borrow_mut(),
                                            &mut *next_row[prev_col].borrow_mut(),
                                            &mut *current_row[prev_col].borrow_mut(),
                                            &mut *prev_rows[prev_col].borrow_mut(),
                                        ],
                                    };
                                    match b.update(&mut wa) {
                                        bot::Action::Ok => {}
                                        bot::Action::Die => *current_cell = Cell::None,
                                        bot::Action::Mov(dir) => {
                                            swap(current_cell, wa.cells[dir as usize]);
                                        }
                                    };
                                }
                            }
                        }
                    }
                }
            // }));
            web_sys::console::log_1(&format!("spawn ok{ti}").into());

        }
        // threads.into_iter().for_each(|th| th.join().unwrap());
    }

    pub fn for_each_cell<F>(&self, mut func: F)
    where
        F: FnMut(usize, usize, &Cell),
    {
        self.cells.iter().enumerate().for_each(|(y, row)| {
            row.lock().unwrap().iter().enumerate().for_each(|(x, c)| {
                func(x, y, &*c.borrow());
            });
        });
    }
}
