
pub use self::private::Profiler;

#[cfg(not(feature = "profiler"))]
mod private {
    pub struct Profiler;

    impl Profiler {
        pub fn new() -> Profiler { Profiler }
        pub fn start_frame(&self) {}
        pub fn end_frame(&self) {}
        pub fn enter(&self, _: &str) {}
        pub fn leave(&self) {}
        pub fn print_summary(&self) {}
    }
}

#[cfg(feature = "profiler")]
mod private {

    extern crate hprof;

    use self::hprof::ProfileNode;
    use std::fmt::{self, Display, Formatter};
    use std::mem;
    use std::borrow::Borrow;
    use std::rc::Rc;

    struct TimingNode {
        name: &'static str,
        total_times: Vec<u64>,
        current_index: usize,
        min_time: u64,
        max_time: u64,
        children: Vec<TimingNode>,
    }

    impl TimingNode {
        pub fn new(name: &'static str) -> TimingNode {
            TimingNode {
                name: name,
                total_times: Vec::with_capacity(10_000),
                current_index: 0,
                children: Vec::new(),
                min_time: u64::max_value(),
                max_time: 0,
            }
        }

        pub fn push(&mut self, total_time: u64) {
            if self.max_time < total_time {
                self.max_time = total_time;
            }
            if self.min_time > total_time {
                self.min_time = total_time;
            }
            if self.total_times.len() != 10_000 {
                self.current_index += 1;
                self.total_times.push(total_time);
            } else {
                self.current_index = (self.current_index + 1) % 10_000;
                self.total_times[self.current_index] = total_time;
            }
        }

        pub fn print_timing(&self) {
            self.print(0);
        }

        pub fn print(&self, indent: u32) {
            for _ in 0..indent {
                print!(" ");
            }
            let avg = self.total_times.iter().sum::<u64>() / (self.total_times.len() as u64);
            println!("{name} - avg({avg}), +{max}/-{min}",
                name = self.name,
                avg = Nanoseconds(avg),
                max = Nanoseconds(self.max_time),
                min = Nanoseconds(self.min_time),
            );
            for c in &self.children {
                c.print(indent+2);
            }
        }
    }

    // used to do a pretty printing of time
    struct Nanoseconds(u64);

    impl Display for Nanoseconds {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            if self.0 < 1_000 {
                write!(f, "{}ns", self.0)
            } else if self.0 < 1_000_000 {
                write!(f, "{:.1}us", self.0 as f64 / 1_000.)
            } else if self.0 < 1_000_000_000 {
                write!(f, "{:.1}ms", self.0 as f64 / 1_000_000.)
            } else {
                write!(f, "{:.1}s", self.0 as f64 / 1_000_000_000.)
            }
        }
    }

    pub struct Profiler {
        p: hprof::Profiler,
        timings: Option<TimingNode>,
    }

    fn create_timing_tree(profile_node: &Rc<ProfileNode>) -> TimingNode {
        let mut timing_node = TimingNode::new(profile_node.name);
        timing_node.push(profile_node.total_time.get());
        for child in profile_node.children.borrow().iter() {
            timing_node.children.push(create_timing_tree(child));
        }
        timing_node
    }

    fn traverse_trees(node: &mut TimingNode, profile_node: &Rc<ProfileNode>) {

        node.push(profile_node.total_time.get());
        for (child, p_child) in node.children.iter_mut().zip(profile_node.children.borrow().iter()) {
            traverse_trees(child, p_child);
        }
    }

    impl Profiler {

        pub fn new() -> Profiler {
            Profiler {
                p: hprof::Profiler::new("Sarosa"),
                timings: None,
            }
        }

        #[inline]
        pub fn start_frame(&self) {
            self.p.start_frame();
        }

        #[inline]
        pub fn end_frame(&mut self) {
            self.p.end_frame();
            let new_timings = {
                let old_timings = mem::replace(&mut self.timings, None);
                match old_timings {
                    Some(mut node) => {
                        traverse_trees(&mut node, &self.p.root());
                        Some(node)
                    }
                    None => Some(create_timing_tree(&self.p.root())),
                }
            };
            mem::replace(&mut self.timings, new_timings);
        }

        #[inline]
        pub fn enter(&self, node_name: &'static str) {
            self.p.enter_noguard(node_name);
        }

        #[inline]
        pub fn leave(&self) {
            self.p.leave();
        }

        #[inline]
        pub fn print_summary(&self) {
            self.timings.as_ref().unwrap().print_timing();
        }
    }
}
