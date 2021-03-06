use std::ops::Div;
use std::time::Instant;

use bitvec::vec::BitVec;

use crate::bmfont_renderer::BMFontRenderer;
use crate::caret::{Caret, CaretType};
use crate::common::{ControlFlags, Direction, FadeState, KeyState};
use crate::engine_constants::EngineConstants;
use crate::ggez::{Context, filesystem, GameResult, graphics};
use crate::npc::{NPC, NPCTable};
use crate::rng::RNG;
use crate::scene::Scene;
use crate::sound::SoundManager;
use crate::stage::StageData;
use crate::str;
use crate::text_script::TextScriptVM;
use crate::texture_set::TextureSet;

pub struct SharedGameState {
    pub control_flags: ControlFlags,
    pub game_flags: BitVec,
    pub fade_state: FadeState,
    pub game_rng: RNG,
    pub effect_rng: RNG,
    pub quake_counter: u16,
    pub carets: Vec<Caret>,
    pub key_state: KeyState,
    pub key_trigger: KeyState,
    pub font: BMFontRenderer,
    pub texture_set: TextureSet,
    pub base_path: String,
    pub npc_table: NPCTable,
    pub stages: Vec<StageData>,
    pub sound_manager: SoundManager,
    pub constants: EngineConstants,
    pub new_npcs: Vec<NPC>,
    pub scale: f32,
    pub god_mode: bool,
    pub speed_hack: bool,
    pub canvas_size: (f32, f32),
    pub screen_size: (f32, f32),
    pub next_scene: Option<Box<dyn Scene>>,
    pub textscript_vm: TextScriptVM,
    pub shutdown: bool,
    key_old: u16,
}

impl SharedGameState {
    pub fn new(ctx: &mut Context) -> GameResult<SharedGameState> {
        let screen_size = graphics::drawable_size(ctx);
        let scale = screen_size.1.div(240.0).floor().max(1.0);
        let canvas_size = (screen_size.0 / scale, screen_size.1 / scale);

        let mut constants = EngineConstants::defaults();
        let mut base_path = "/";

        if filesystem::exists(ctx, "/base/Nicalis.bmp") {
            info!("Cave Story+ (PC) data files detected.");
            constants.apply_csplus_patches();
            base_path = "/base/";
        } else if filesystem::exists(ctx, "/base/lighting.tbl") {
            info!("Cave Story+ (Switch) data files detected.");
            constants.apply_csplus_patches();
            constants.apply_csplus_nx_patches();
            base_path = "/base/";
        } else if filesystem::exists(ctx, "/mrmap.bin") {
            info!("CSE2E data files detected.");
        } else if filesystem::exists(ctx, "/stage.dat") {
            info!("NXEngine-evo data files detected.");
        }

        let font = BMFontRenderer::load(base_path, &constants.font_path, ctx)
            .or_else(|_| BMFontRenderer::load("/", "builtin/builtin_font.fnt", ctx))?;

        Ok(SharedGameState {
            control_flags: ControlFlags(0),
            game_flags: bitvec::bitvec![0; 8000],
            fade_state: FadeState::Hidden,
            game_rng: RNG::new(0),
            effect_rng: RNG::new(Instant::now().elapsed().as_nanos() as i32),
            quake_counter: 0,
            carets: Vec::with_capacity(32),
            key_state: KeyState(0),
            key_trigger: KeyState(0),
            font,
            texture_set: TextureSet::new(base_path),
            base_path: str!(base_path),
            npc_table: NPCTable::new(),
            stages: Vec::with_capacity(96),
            sound_manager: SoundManager::new(ctx)?,
            constants,
            new_npcs: Vec::with_capacity(8),
            scale,
            god_mode: false,
            speed_hack: false,
            screen_size,
            canvas_size,
            next_scene: None,
            textscript_vm: TextScriptVM::new(),
            key_old: 0,
            shutdown: false,
        })
    }

    pub fn reset(&mut self) {
        self.control_flags.0 = 0;
        self.game_flags = bitvec::bitvec![0; 8000];
        self.fade_state = FadeState::Hidden;
        self.game_rng = RNG::new(0);
        self.quake_counter = 0;
        self.carets.clear();
        self.key_state.0 = 0;
        self.key_trigger.0 = 0;
        self.key_old = 0;
        self.new_npcs.clear();
        self.textscript_vm.reset();
        self.textscript_vm.suspend = true;
    }

    pub fn handle_resize(&mut self, ctx: &mut Context) -> GameResult {
        self.screen_size = graphics::drawable_size(ctx);
        self.scale = self.screen_size.1.div(240.0).floor().max(1.0);
        self.canvas_size = (self.screen_size.0 / self.scale, self.screen_size.1 / self.scale);

        graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, self.screen_size.0, self.screen_size.1))?;

        Ok(())
    }

    pub fn update_key_trigger(&mut self) {
        let mut trigger = self.key_state.0 ^ self.key_old;
        trigger &= self.key_state.0;
        self.key_old = self.key_state.0;
        self.key_trigger = KeyState(trigger);
    }

    pub fn tick_carets(&mut self) {
        for caret in self.carets.iter_mut() {
            caret.tick(&self.effect_rng, &self.constants);
        }

        self.carets.retain(|c| !c.is_dead());
    }

    pub fn create_caret(&mut self, x: isize, y: isize, ctype: CaretType, direct: Direction) {
        self.carets.push(Caret::new(x, y, ctype, direct, &self.constants));
    }

    pub fn set_speed_hack(&mut self, toggle: bool) {
        self.speed_hack = toggle;

        if let Err(err) = self.sound_manager.set_speed(if toggle { 2.0 } else { 1.0 }) {
            log::error!("Error while sending a message to sound manager: {}", err);
        }
    }

    pub fn shutdown(&mut self) {
        self.shutdown = true;
    }
}
