use crate::*;

pub struct TaskSchedule {
    tasks: Vec<(f64, Box<dyn Fn(&mut Game, f64, f64) + 'static>)>,
}

impl TaskSchedule {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn push(&mut self, time: f64, func: impl Fn(&mut Game, f64, f64) + 'static) {
        let boxed_func = Box::new(func);
        for idx in (0..self.tasks.len()).rev() {
            if self.tasks[idx].0 < time {
                self.tasks.insert(idx + 1, (time, boxed_func));
                return;
            }
        }
        self.tasks.insert(0, (time, boxed_func));
    }

    pub fn push_multiple(
        &mut self,
        time: f64,
        mut funcs: Vec<Box<dyn Fn(&mut Game, f64, f64) + 'static>>,
    ) {
        for idx in (0..self.tasks.len()).rev() {
            if self.tasks[idx].0 <= time {
                self.tasks.splice(
                    (idx + 1)..(idx + 1),
                    funcs.drain(..).map(|func| (time, func)),
                );
                return;
            }
        }
        self.tasks
            .splice(0..0, funcs.drain(..).map(|func| (time, func)));
    }

    pub fn execute(&mut self, game: &mut Game, delta_time: f64, current_time: f64) {
        let mut removed = 0;
        for (task_time, func) in self.tasks.iter() {
            if *task_time <= current_time {
                func(game, delta_time, current_time);
                removed += 1;
            } else {
                break;
            }
        }
        self.tasks = self.tasks.drain(removed..).collect();
    }
}
