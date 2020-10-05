mod exec;
use miniquad::*;
use std::future::Future;
use std::pin::Pin;
use std::cell::RefCell;
use std::rc::Rc;

static mut GAME: Game = Game {
    screen_size: (0., 0.),
    r: 0.,
    g: 0.,
    b: 0.,
};

struct Game {
    screen_size: (f32, f32),
    r: f32,
    g: f32,
    b: f32,
}

impl Game {
    fn do_something_with_an_event(&mut self, x: f32, y: f32) {
        self.r = x / self.screen_size.0;
        self.g = y / self.screen_size.1;
        self.b += 0.001;
    }
}

impl Game {
    async fn draw(&mut self, ctx: Context) {
        // Anything you need to preload goes here,
        // ---------------------------------------
        let raw_data = load_file("assets/test.png").await;
        // dbg!(raw_data); // here it is loaded, before we have hit our main draw loop.
    
        // This is your main draw loop
        // ---------------------------
        loop {    
            ctx.clear(Some((self.r, self.g, self.b, 1.0)), None, None);
            
            next_frame().await
        }
    }
}

fn main() {
    #[rustfmt::skip]
    miniquad::start(conf::Conf::default(), |ctx| {
        // give your game knowledge of the initial screen size
        unsafe { GAME.screen_size = ctx.screen_size(); }

        UserData::free(Stage {
            ctx: Some(ctx),
            main_future: None,
        })
    });
}

pub fn next_frame() -> Pin<Box<exec::FrameFuture>> {
    Box::pin(exec::FrameFuture)
}

struct Stage<'a> {
    ctx: Option<Context>,
    main_future: Option<Pin<Box<dyn Future<Output = ()> + 'a>>>,
}

impl<'a> EventHandlerFree for Stage<'a> {
    fn resize_event(&mut self, width: f32, height: f32) {
        unsafe { GAME.screen_size = (width, height); }
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        unsafe { GAME.do_something_with_an_event(x, y); }
    }

    fn update(&mut self) {
        unsafe { GAME.b = GAME.b.sin().abs(); }
    }

    #[rustfmt::skip]
    fn draw(&mut self) {
        if let Some(future) = self.main_future.as_mut() {
            if exec::resume(future) {
                self.main_future = None;
                self.ctx.as_ref().unwrap().quit();
                return;
            }
        } else {
            unsafe {
                self.main_future = Some(Box::pin(GAME.draw(self.ctx.take().unwrap())));
            }
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
