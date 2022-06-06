#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

use {
    smash::{
        hash40,
        app::*,
        lib::*
    }
};

#[skyline::hook(replace = sv_animcmd::ATTACK)]
unsafe fn attack_replace(lua_state: u64) {
    // let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    // let fighter_kind = smash::app::utility::get_kind(boma);
    let mut l2c_agent = L2CAgent::new(lua_state);
    let hitbox_params: Vec<L2CValue> = (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
    l2c_agent.clear_lua_stack();
    for i in 0..36 {
        if i == 4 {
            let high_low = sv_math::rand(hash40("fighter"), 8);
            let mut new_angle;
            if high_low == 0 {
                new_angle = sv_math::rand(hash40("fighter"), 120) as u64 + 30 + 180;
            }
            else {
                new_angle = sv_math::rand(hash40("fighter"), 188) as u64;
                if new_angle > 180 {
                    new_angle += 180;
                }
            }
            l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_angle));
        }
        else {
            l2c_agent.push_lua_stack(&mut hitbox_params[i].clone());
        }
    }
    original!()(lua_state);
}

#[skyline::hook(replace = sv_animcmd::ATTACK_ABS)]
unsafe fn attack_abs_replace(lua_state: u64) {
    // let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    // let fighter_kind = smash::app::utility::get_kind(boma);
    let mut l2c_agent = L2CAgent::new(lua_state);
    let hitbox_params: Vec<L2CValue> = (0..15).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
    l2c_agent.clear_lua_stack();
    for i in 0..15 {
        if i == 3 {
            let high_low = sv_math::rand(hash40("fighter"), 8);
            let mut new_angle;
            if high_low == 0 {
                new_angle = sv_math::rand(hash40("fighter"), 120) as u64 + 30 + 180;
            }
            else {
                new_angle = sv_math::rand(hash40("fighter"), 182) as u64;
                if new_angle > 180 {
                    new_angle += 180;
                }
            }
            l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_angle));
        }
        else {
            l2c_agent.push_lua_stack(&mut hitbox_params[i].clone());
        }
    }
    original!()(lua_state);
}

#[skyline::main(name = "random_launch_angles")]
pub fn main() {
    skyline::install_hooks!(
        attack_replace,
        attack_abs_replace
    );
}