#![feature(libc)]
#![feature(extern_types)]
#![feature(asm)]
#![feature(ptr_wrapping_offset_from)]
#![feature(label_break_value)]
#![feature(const_raw_ptr_to_usize_cast)]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(unused_mut)]#![feature(custom_attribute)]



extern crate libc;

#[path = "src/cl_scrn.rs"]
pub mod cl_scrn;
#[path = "src/cl_main.rs"]
pub mod cl_main;
#[path = "src/cl_net_chan.rs"]
pub mod cl_net_chan;
#[path = "src/cl_console.rs"]
pub mod cl_console;
#[path = "src/cl_input.rs"]
pub mod cl_input;
#[path = "src/cl_cin.rs"]
pub mod cl_cin;
#[path = "src/cl_cgame.rs"]
pub mod cl_cgame;
#[path = "src/cl_curl.rs"]
pub mod cl_curl;
#[path = "src/cl_parse.rs"]
pub mod cl_parse;
#[path = "src/cl_ui.rs"]
pub mod cl_ui;
#[path = "src/cl_avi.rs"]
pub mod cl_avi;
#[path = "src/cl_keys.rs"]
pub mod cl_keys;

#[path = "src/cm_load.rs"]
pub mod cm_load;
#[path = "src/cm_trace.rs"]
pub mod cm_trace;
#[path = "src/cm_test.rs"]
pub mod cm_test;
#[path = "src/cm_patch.rs"]
pub mod cm_patch;
#[path = "src/cm_polylib.rs"]
pub mod cm_polylib;

#[path = "src/con_log.rs"]
pub mod con_log;
#[path = "src/con_tty.rs"]
pub mod con_tty;

#[path = "src/sv_world.rs"]
pub mod sv_world;
#[path = "src/sv_ccmds.rs"]
pub mod sv_ccmds;
#[path = "src/sv_snapshot.rs"]
pub mod sv_snapshot;
#[path = "src/sv_net_chan.rs"]
pub mod sv_net_chan;
#[path = "src/sv_main.rs"]
pub mod sv_main;
/*
#[path = "src/sv_init.rs"]
pub mod sv_init;
#[path = "src/sv_bot.rs"]
pub mod sv_bot;
*/

#[path = "src/l_crc.rs"]
pub mod l_crc;
#[path = "src/l_memory.rs"]
pub mod l_memory;
#[path = "src/l_precomp.rs"]
pub mod l_precomp;
#[path = "src/l_libvar.rs"]
pub mod l_libvar;
#[path = "src/l_struct.rs"]
pub mod l_struct;
#[path = "src/l_script.rs"]
pub mod l_script;

#[path = "src/md5.rs"]
pub mod md5;
#[path = "src/cmd.rs"]
pub mod cmd;
#[path = "src/qal.rs"]
pub mod qal;
#[path = "src/snd_adpcm.rs"]
pub mod snd_adpcm;
#[path = "src/cvar.rs"]
pub mod cvar;
#[path = "src/snd_codec_ogg.rs"]
pub mod snd_codec_ogg;
#[path = "src/sys_main.rs"]
pub mod sys_main;
#[path = "src/q_math.rs"]
pub mod q_math;
#[path = "src/snd_mix.rs"]
pub mod snd_mix;
#[path = "src/snd_mem.rs"]
pub mod snd_mem;
#[path = "src/md4.rs"]
pub mod md4;
#[path = "src/sys_unix.rs"]
pub mod sys_unix;
#[path = "src/snd_codec_wav.rs"]
pub mod snd_codec_wav;
#[path = "src/snd_main.rs"]
pub mod snd_main;
#[path = "src/msg.rs"]
pub mod msg;
#[path = "src/snd_dma.rs"]
pub mod snd_dma;
#[path = "src/sdl_input.rs"]
pub mod sdl_input;
#[path = "src/snd_codec.rs"]
pub mod snd_codec;
#[path = "src/libmumblelink.rs"]
pub mod libmumblelink;

#[path = "src/vm_interpreted.rs"]
pub mod vm_interpreted;
#[path = "src/vm_x86.rs"]
pub mod vm_x86;
#[path = "src/vm.rs"]
pub mod vm;

#[path = "src/be_aas_main.rs"]
pub mod be_aas_main;
#[path = "src/be_aas_cluster.rs"]
pub mod be_aas_cluster;
#[path = "src/be_ai_move.rs"]
pub mod be_ai_move;
#[path = "src/be_aas_entity.rs"]
pub mod be_aas_entity;
#[path = "src/be_ai_weap.rs"]
pub mod be_ai_weap;
#[path = "src/be_ai_goal.rs"]
pub mod be_ai_goal;
#[path = "src/be_aas_bspq3.rs"]
pub mod be_aas_bspq3;
#[path = "src/be_aas_reach.rs"]
pub mod be_aas_reach;
#[path = "src/be_aas_move.rs"]
pub mod be_aas_move;
#[path = "src/be_interface.rs"]
pub mod be_interface;
#[path = "src/be_ai_weight.rs"]
pub mod be_ai_weight;
#[path = "src/be_aas_route.rs"]
pub mod be_aas_route;
#[path = "src/be_aas_file.rs"]
pub mod be_aas_file;
#[path = "src/be_aas_sample.rs"]
pub mod be_aas_sample;
#[path = "src/be_ai_char.rs"]
pub mod be_ai_char;
#[path = "src/be_aas_debug.rs"]
pub mod be_aas_debug;
#[path = "src/be_ai_gen.rs"]
pub mod be_ai_gen;
#[path = "src/be_aas_routealt.rs"]
pub mod be_aas_routealt;
#[path = "src/be_ai_chat.rs"]
pub mod be_ai_chat;
#[path = "src/be_ea.rs"]
pub mod be_ea;
#[path = "src/be_aas_optimize.rs"]
pub mod be_aas_optimize;
/*
#[path = "src/sv_game.rs"]
pub mod sv_game;
*/
#[path = "src/snd_codec_opus.rs"]
pub mod snd_codec_opus;
#[path = "src/snd_wavelet.rs"]
pub mod snd_wavelet;
#[path = "src/puff.rs"]
pub mod puff;
#[path = "src/unzip.rs"]
pub mod unzip;
#[path = "src/snd_altivec.rs"]
pub mod snd_altivec;
#[path = "src/sys_autoupdater.rs"]
pub mod sys_autoupdater;
#[path = "src/snd_openal.rs"]
pub mod snd_openal;
#[path = "src/huffman.rs"]
pub mod huffman;
#[path = "src/sdl_snd.rs"]
pub mod sdl_snd;

#[path = "src/net_chan.rs"]
pub mod net_chan;

fn main() { sys_main::main() }

