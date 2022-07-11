use crate::State;
use elysium_math::Matrix3x4;
use elysium_sdk::model::{DrawModelState, ModelRender, ModelRenderInfo};

#[inline(never)]
pub unsafe extern "C" fn draw_model(
    this: *const ModelRender,
    context: *mut u8,
    draw_state: *mut DrawModelState,
    info: *const ModelRenderInfo,
    bone_to_world: *const Matrix3x4,
) {
    let state = State::get();
    let hooks = state.hooks.as_ref().unwrap_unchecked();
    let interfaces = state.interfaces.as_ref().unwrap_unchecked();
    let model_render = &interfaces.model_render;

    if let Some(gold) = state.materials.gold {
        model_render.override_material(gold, 0, -1);
        (hooks.draw_model)(this, context, draw_state, info, bone_to_world);
        model_render.reset_material();
    }
}
