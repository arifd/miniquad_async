mod exec;
use miniquad::*;
use std::future::Future;
use std::pin::Pin;
use std::cell::RefCell;
use std::rc::Rc;

struct Game {
    screen_size: (f32, f32),
    r: f32,
    g: f32,
    b: f32,
    gfx: Context
}

impl Game {
    fn do_something_with_an_event(&mut self, x: f32, y: f32) {
        self.r = x / self.screen_size.0;
        self.g = y / self.screen_size.1;
        self.b += 0.001;
    }
}

fn main() {
    #[rustfmt::skip]
    miniquad::start(conf::Conf::default(), |ctx| {
        // Setup your Game struct here
        let game = Game {
            screen_size: ctx.screen_size(),
            r: 0.,
            g: 0.,
            b: 0.,
            gfx: ctx,
        };

        UserData::free(Stage {
            main_future: None,
            game: Rc::new(RefCell::new(game)),
        })
    });
}

fn update(game: &mut Game) {
    game.b = game.b.sin().abs();
}

async fn run(game: Rc<RefCell<Game>>) {
    // Anything you need to preload goes here,
    // ---------------------------------------
    let raw_data = load_file("assets/test.png").await;
    // dbg!(raw_data); // here it is loaded, before we have hit our main draw loop.

    // This is your main draw loop
    // ---------------------------
    loop {
        // Here you can reference your game,
        // but always drop the reference before the end of the loop!!
        let mut game = game.borrow_mut();
        
        update(&mut game);

        game.gfx.clear(Some((game.r, game.g, game.b, 1.0)), None, None);

        std::mem::drop(game);
        next_frame().await
    }
}

pub fn next_frame() -> Pin<Box<exec::FrameFuture>> {
    Box::pin(exec::FrameFuture)
}

struct Stage<'a> {
    main_future: Option<Pin<Box<dyn Future<Output = ()> + 'a>>>,
    game: Rc<RefCell<Game>>,
}

impl<'a> EventHandlerFree for Stage<'a> {
    fn resize_event(&mut self, width: f32, height: f32) {
        self.game.borrow_mut().screen_size = (width, height);
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.game.borrow_mut().do_something_with_an_event(x, y);
    }

    fn update(&mut self) {}

    #[rustfmt::skip]
    fn draw(&mut self) {
        if let Some(future) = self.main_future.as_mut() {
            if exec::resume(future) {
                self.main_future = None;
                // self.ctx.as_ref().unwrap().quit(); // this will always panic. Self, doesn't have ctx anymore
                return;
            }
        } else {
            let game = self.game.clone();
            self.main_future = Some(Box::pin(run(game)));
        }
    }
}

pub fn load_file(path: &str) -> exec::FutureWithReturn<Vec<u8>> {
    let item = Rc::new(RefCell::new(None));
    let path = path.to_owned();

    {
        let item = item.clone();

        miniquad::fs::load_file(&path, move |bytes| {
            let bytes = bytes.expect("file not found");
            *item.borrow_mut() = Some(bytes);
        });
    }

    exec::FutureWithReturn { item }
}
