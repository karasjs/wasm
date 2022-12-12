pub mod refresh_level {
  pub const NONE: usize = 0; //                                         0
  pub const CACHE: usize = 1; //                                        1
  pub const TRANSLATE_X: usize = 2; //                                 10
  pub const TRANSLATE_Y: usize = 4; //                                100
  pub const TRANSLATE_Z: usize = 8; //                               1000
  pub const TRANSLATE: usize = 14; //                                1110
  pub const ROTATE_Z: usize = 16; //                                10000
  pub const SCALE_X: usize = 32; //                                100000
  pub const SCALE_Y: usize = 64; //                               1000000
  pub const SCALE_Z: usize = 128; //                             10000000
  pub const SCALE: usize = 224; //                               11100000
  pub const TRANSFORM: usize = 256; //                          100000000
  pub const TRANSFORM_ALL: usize = 510; //                      111111110
  pub const OPACITY: usize = 512; //                           1000000000
  pub const FILTER: usize = 1024; //                          10000000000
  pub const MIX_BLEND_MODE: usize = 2048; //                 100000000000
  pub const PERSPECTIVE: usize = 4096; //                   1000000000000
  pub const MASK: usize = 8192; //                         10000000000000
  pub const REPAINT: usize = 16384; //                    100000000000000
  pub const REFLOW: usize = 32768; //                    1000000000000000
  pub const REBUILD: usize = 65536; //                  10000000000000000
}
