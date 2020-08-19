use num_traits::clamp;

use crate::game_state::GameState;
use crate::player::Player;

const OFF_X: &[isize; 4] = &[0, 1, 0, 1];
const OFF_Y: &[isize; 4] = &[0, 0, 1, 1];

impl Player {
    fn judge_hit_block(&mut self, x: isize, y: isize, state: &GameState) {
        // left wall
        if (self.y - self.hit.top as isize) < (y * 0x10 + 4) * 0x200
            && self.y + self.hit.bottom as isize > (y * 0x10 - 4) * 0x200
            && (self.x - self.hit.right as isize) < (x * 0x10 + 8) * 0x200
            && (self.x - self.hit.right as isize) > x * 0x10 * 0x200 {
            self.x = ((x * 0x10 + 8) * 0x200) + self.hit.right as isize;

            if self.xm < -0x180 {
                self.xm = -0x180;
            }

            if !state.key_state.left() && self.xm < 0 {
                self.xm = 0;
            }

            self.flags.set_flag_x01(true);
        }

        // right wall
        if (self.y - self.hit.top as isize) < (y * 0x10 + 4) * 0x200
            && self.y + self.hit.bottom as isize > (y * 0x10 - 4) * 0x200
            && (self.x + self.hit.right as isize) > (x * 0x10 - 8) * 0x200
            && (self.x + self.hit.right as isize) < x * 0x10 * 0x200 {
            self.x = ((x * 0x10 - 8) * 0x200) - self.hit.right as isize;

            if self.xm > 0x180 {
                self.xm = 0x180;
            }

            if !state.key_state.right() && self.xm > 0 {
                self.xm = 0;
            }

            self.flags.set_flag_x04(true);
        }

        // ceiling
        if (self.x - self.hit.right as isize) < (x * 0x10 + 5) * 0x200
            && (self.x + self.hit.right as isize) > (x * 0x10 - 5) * 0x200
            && (self.y - self.hit.top as isize) < (y * 0x10 + 8) * 0x200
            && (self.y - self.hit.top as isize) > y * 0x10 * 0x200 {
            self.y = ((y * 0x10 + 8) * 0x200) + self.hit.top as isize;

            if !self.cond.cond_x02() && self.ym < -0x200 {
                // PutLittleStar(); todo
            }

            if self.ym < 0 {
                self.ym = 0;
            }

            self.flags.set_flag_x02(true);
        }

        // floor
        if ((self.x - self.hit.right as isize) < (x * 0x10 + 5) * 0x200)
            && ((self.x + self.hit.right as isize) > (x * 0x10 - 5) * 0x200)
            && ((self.y + self.hit.bottom as isize) > (y * 0x10 - 8) * 0x200)
            && ((self.y + self.hit.bottom as isize) < y * 0x10 * 0x200) {
            self.y = ((y * 0x10 - 8) * 0x200) - self.hit.bottom as isize;

            if self.ym > 0x400 {
                // PlaySoundObject(23, SOUND_MODE_PLAY); todo
            }

            if self.ym > 0 {
                self.ym = 0;
            }

            self.flags.set_flag_x08(true);
        }
    }

    fn judge_hit_triangle_a(&mut self, x: isize, y: isize) {
        if self.x < (x * 0x10 + 8) * 0x200
            && self.x > (x * 0x10 - 8) * 0x200
            && (self.y - self.hit.top as isize) < (y * 0x10 * 0x200) - (self.x - x * 0x10 * 0x200) / 2 + 0x800
            && (self.y + self.hit.bottom as isize) > (y * 0x10 - 8) * 0x200 {
            self.y = (y * 0x10 * 0x200) - ((self.x - x * 0x10 * 0x200) / 2) + 0x800 + self.hit.top as isize;

            if !self.cond.cond_x02() && self.ym < -0x200 {
                // PutLittleStar(); todo
            }

            if self.ym < 0 {
                self.ym = 0;
            }

            self.flags.set_flag_x02(true);
        }
    }

    fn judge_hit_triangle_b(&mut self, x: isize, y: isize) {
        if self.x < (x * 0x10 + 8) * 0x200
            && self.x > (x * 0x10 - 8) * 0x200
            && (self.y - self.hit.top as isize) < (y * 0x10 * 0x200) - (self.x - x * 0x10 * 0x200) / 2 - 0x800
            && (self.y + self.hit.bottom as isize) > (y * 0x10 - 8) * 0x200 {
            self.y = (y * 0x10 * 0x200) - ((self.x - x * 0x10 * 0x200) / 2) - 0x800 + self.hit.top as isize;

            if !self.cond.cond_x02() && self.ym < -0x200 {
                // PutLittleStar(); todo
            }

            if self.ym < 0 {
                self.ym = 0;
            }

            self.flags.set_flag_x02(true);
        }
    }

    fn judge_hit_triangle_c(&mut self, x: isize, y: isize) {
        if self.x < (x * 0x10 + 8) * 0x200
            && self.x > (x * 0x10 - 8) * 0x200
            && (self.y - self.hit.top as isize) < (y * 0x10 * 0x200) + (self.x - x * 0x10 * 0x200) / 2 - 0x800
            && (self.y + self.hit.bottom as isize) > (y * 0x10 - 8) * 0x200 {
            self.y = (y * 0x10 * 0x200) + ((self.x - x * 0x10 * 0x200) / 2) - 0x800 + self.hit.top as isize;

            if !self.cond.cond_x02() && self.ym < -0x200 {
                // PutLittleStar(); todo
            }

            if self.ym < 0 {
                self.ym = 0;
            }

            self.flags.set_flag_x02(true);
        }
    }

    fn judge_hit_triangle_d(&mut self, x: isize, y: isize) {
        if (self.x < (x * 0x10 + 8) * 0x200)
            && (self.x > (x * 0x10 - 8) * 0x200)
            && (self.y - self.hit.top as isize) < (y * 0x10 * 0x200) + (self.x - x * 0x10 * 0x200) / 2 + 0x800
            && (self.y + self.hit.bottom as isize) > (y * 0x10 - 8) * 0x200 {
            self.y = (y * 0x10 * 0x200) + ((self.x - x * 0x10 * 0x200) / 2) + 0x800 + self.hit.top as isize;

            if !self.cond.cond_x02() && self.ym < -0x200 {
                // PutLittleStar(); todo
            }

            if self.ym < 0 {
                self.ym = 0;
            }

            self.flags.set_flag_x02(true);
        }
    }

    fn judge_hit_triangle_e(&mut self, x: isize, y: isize) {
        self.flags.set_flag_x10000(true);

        if (self.x < (x * 0x10 + 8) * 0x200)
            && (self.x > (x * 0x10 - 8) * 0x200)
            && (self.y + self.hit.bottom as isize) > (y * 0x10 * 0x200) + (self.x - x * 0x10 * 0x200) / 2 - 0x800
            && (self.y - self.hit.top as isize) < (y * 0x10 + 8) * 0x200 {
            self.y = (y * 0x10 * 0x200) + ((self.x - x * 0x10 * 0x200) / 2) - 0x800 - self.hit.bottom as isize;

            if self.ym > 0x400 {
                // PlaySoundObject(23, SOUND_MODE_PLAY); todo
            }

            if self.ym > 0 {
                self.ym = 0;
            }

            self.flags.set_flag_x20(true);
            self.flags.set_flag_x08(true);
        }
    }

    fn judge_hit_triangle_f(&mut self, x: isize, y: isize) {
        self.flags.set_flag_x20000(true);

        if (self.x < (x * 0x10 + 8) * 0x200)
            && (self.x > (x * 0x10 - 8) * 0x200)
            && (self.y + self.hit.bottom as isize) > (y * 0x10 * 0x200) + (self.x - x * 0x10 * 0x200) / 2 + 0x800
            && (self.y - self.hit.top as isize) < (y * 0x10 + 8) * 0x200 {
            self.y = (y * 0x10 * 0x200) + ((self.x - x * 0x10 * 0x200) / 2) + 0x800 - self.hit.bottom as isize;

            if self.ym > 0x400 {
                // PlaySoundObject(23, SOUND_MODE_PLAY); todo
            }

            if self.ym > 0 {
                self.ym = 0;
            }

            self.flags.set_flag_x20(true);
            self.flags.set_flag_x08(true);
        }
    }

    fn judge_hit_triangle_g(&mut self, x: isize, y: isize) {
        self.flags.set_flag_x40000(true);

        if (self.x < (x * 0x10 + 8) * 0x200)
            && (self.x > (x * 0x10 - 8) * 0x200)
            && (self.y + self.hit.bottom as isize) > (y * 0x10 * 0x200) - (self.x - x * 0x10 * 0x200) / 2 + 0x800
            && (self.y - self.hit.top as isize) < (y * 0x10 + 8) * 0x200 {
            self.y = (y * 0x10 * 0x200) - ((self.x - x * 0x10 * 0x200) / 2) + 0x800 - self.hit.bottom as isize;

            if self.ym > 0x400 {
                // PlaySoundObject(23, SOUND_MODE_PLAY); todo
            }

            if self.ym > 0 {
                self.ym = 0;
            }

            self.flags.set_flag_x10(true);
            self.flags.set_flag_x08(true);
        }
    }

    fn judge_hit_triangle_h(&mut self, x: isize, y: isize) {
        self.flags.set_flag_x80000(true);

        if (self.x < (x * 0x10 + 8) * 0x200)
            && (self.x > (x * 0x10 - 8) * 0x200)
            && (self.y + self.hit.bottom as isize) > (y * 0x10 * 0x200) - (self.x - x * 0x10 * 0x200) / 2 - 0x800
            && (self.y - self.hit.top as isize) < (y * 0x10 + 8) * 0x200 {
            self.y = (y * 0x10 * 0x200) - ((self.x - x * 0x10 * 0x200) / 2) - 0x800 - self.hit.bottom as isize;

            if self.ym > 0x400 {
                // PlaySoundObject(23, SOUND_MODE_PLAY); todo
            }

            if self.ym > 0 {
                self.ym = 0;
            }

            self.flags.set_flag_x10(true);
            self.flags.set_flag_x08(true);
        }
    }

    fn judge_hit_water(&mut self, x: isize, y: isize) {
        if (self.x - self.hit.right as isize) < (x * 0x10 + 5) * 0x200
            && (self.x + self.hit.right as isize) > (x * 0x10 - 5) * 0x200
            && (self.y - self.hit.top as isize) < (y * 0x10 + 5) * 0x200
            && (self.y + self.hit.bottom as isize) > y * 0x10 * 0x200 {
            self.flags.set_underwater(true);
        }
    }

    pub fn tick_map_collisions(&mut self, state: &GameState) {
        let x = clamp(self.x / 0x10 / 0x200, 0, state.stage.map.width as isize);
        let y = clamp(self.y / 0x10 / 0x200, 0, state.stage.map.height as isize);

        for (ox, oy) in OFF_X.iter().zip(OFF_Y) {
            let attrib = state.stage.map.get_attribute((x + *ox) as usize, (y + *oy) as usize);
            match attrib {
                // Block
                0x02 | 0x60 => {
                    self.judge_hit_water(x + *ox, y + *oy);
                }
                0x05 | 0x41 | 0x43 | 0x46 => {
                    self.judge_hit_block(x + *ox, y + *oy, state);
                }
                0x50 | 0x70 => {
                    self.judge_hit_triangle_a(x + *ox, y + *oy);
                    if attrib & 0x20 != 0 { self.judge_hit_water(x + *ox, y + *oy); }
                }
                0x51 | 0x71 => {
                    self.judge_hit_triangle_b(x + *ox, y + *oy);
                    if attrib & 0x20 != 0 { self.judge_hit_water(x + *ox, y + *oy); }
                }
                0x52 | 0x72 => {
                    self.judge_hit_triangle_c(x + *ox, y + *oy);
                    if attrib & 0x20 != 0 { self.judge_hit_water(x + *ox, y + *oy); }
                }
                0x53 | 0x73 => {
                    self.judge_hit_triangle_d(x + *ox, y + *oy);
                    if attrib & 0x20 != 0 { self.judge_hit_water(x + *ox, y + *oy); }
                }
                0x54 | 0x74 => {
                    self.judge_hit_triangle_e(x + *ox, y + *oy);
                    if attrib & 0x20 != 0 { self.judge_hit_water(x + *ox, y + *oy); }
                }
                0x55 | 0x75 => {
                    self.judge_hit_triangle_f(x + *ox, y + *oy);
                    if attrib & 0x20 != 0 { self.judge_hit_water(x + *ox, y + *oy); }
                }
                0x56 | 0x76 => {
                    self.judge_hit_triangle_g(x + *ox, y + *oy);
                    if attrib & 0x20 != 0 { self.judge_hit_water(x + *ox, y + *oy); }
                }
                0x57 | 0x77 => {
                    self.judge_hit_triangle_h(x + *ox, y + *oy);
                    if attrib & 0x20 != 0 { self.judge_hit_water(x + *ox, y + *oy); }
                }
                0x61 => {
                    self.judge_hit_water(x + *ox, y + *oy);
                    self.judge_hit_block(x + *ox, y + *oy, state);
                }
                _ => {}
            }
        }
    }
}
