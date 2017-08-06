
extern crate ggez;
extern crate rand;
extern crate time;

extern crate omn_sprites;


use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;

use ggez::conf;
use ggez::event;
use ggez::{GameResult, Context};
use ggez::graphics::{self, DrawParam, Image};

use omn_sprites::{SpriteSheetData, AnimationClip, ClipStore, PlayMode};


struct MainState {
    clip: Option<RefCell<AnimationClip>>,
    sheet: Rc<SpriteSheetData>,
    clips: Rc<ClipStore>,
    image: Image,
}


impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        ctx.print_resource_stats();
        let sheet = Rc::new(SpriteSheetData::from_file(
            "resources/numbers-matrix-tags.array.json",
        ));
        let clips = Rc::new(ClipStore::new(sheet.as_ref()));
        let mut s = MainState {
            clips: clips,
            clip: None,
            sheet: sheet.clone(),
            image: Image::new(ctx, "/numbers-matrix.png").unwrap(),
        };
        s.clip = Some(RefCell::new(
            s.clips.create("Alpha", PlayMode::Loop).unwrap(),
        ));
        Ok(s)
    }
}


impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {

        if let Some(ref clip) = self.clip {
            let delta_millis = _dt.subsec_nanos() as f32 / 1e6;
            clip.borrow_mut().update(delta_millis)
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        if let Some(ref clip) = self.clip {

            let w = self.image.width() as f32;
            let h = self.image.height() as f32;

            let idx = clip.borrow().get_cell().unwrap();
            let cell = &self.sheet.frames[idx];
            let param = DrawParam {
                src: graphics::Rect::new(
                    cell.bbox.x as f32 / w,
                    cell.bbox.y as f32 / h,
                    cell.bbox.width as f32 / w,
                    cell.bbox.height as f32 / h,
                ),
                dest: graphics::Point::new(160., 120.),
                scale: graphics::Point::new(1.5, 1.5),
                ..Default::default()
            };
            graphics::draw_ex(ctx, &self.image, param)?;
        }

        graphics::present(ctx);
        Ok(())
    }
}


fn main() {

    let mut conf = conf::Conf::new();
    conf.window_width = 320;
    conf.window_height = 240;
    conf.window_title = "Omn Sprites".to_string();

    println!("Starting with default config: {:#?}", conf);
    let ctx = &mut Context::load_from_conf("Omn Sprites", "omn_sprites", conf).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
