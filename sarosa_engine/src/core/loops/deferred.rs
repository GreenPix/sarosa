
use super::clock_ticks;

use std::thread;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use loader::Loader;

pub struct DeferredLoader<R> {
    rx_resource: Receiver<R>,
}

impl<R> DeferredLoader<R>
    where R: Send + 'static
{

    pub fn new(mut loader: Box<Loader<Resources=R>>) -> DeferredLoader<R> {

        let (tx, rx) = channel();

        thread::spawn(move|| {
            let r = loader.load_resources();
            tx.send(r).unwrap();
        });

        DeferredLoader {
            rx_resource: rx,
        }
    }

    pub fn while_waiting<F>(self, mut actions: F) -> R
        where F: FnMut(u64)
    {
        debug!("Deferred Started");

        let mut previous_clock = clock_ticks::precise_time_ns();

        loop {
            let now = clock_ticks::precise_time_ns();

            if let Ok(r) = self.rx_resource.try_recv() {
                debug!("Deferred Finished");
                return r;
            }

            actions(now - previous_clock);

            previous_clock = now;
            thread::sleep_ms(8);
        }
    }
}
