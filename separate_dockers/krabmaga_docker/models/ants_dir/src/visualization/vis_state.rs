use krabmaga::bevy::prelude::Commands;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::fields::sparse_object_grid_2d::SparseGrid2D;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State as StateTrait;
use krabmaga::visualization::agent_render::AgentRender;
use krabmaga::visualization::asset_handle_factory::AssetHandleFactoryResource;
use krabmaga::visualization::fields::number_grid_2d::BatchRender;
use krabmaga::visualization::fields::object_grid_2d::RenderObjectGrid2D;
use krabmaga::visualization::simulation_descriptor::SimulationDescriptor;
use krabmaga::visualization::visualization_state::VisualizationState;

use crate::model::ant::{Ant, AntType};
use crate::model::state::*;
use crate::visualization::ant::AntVis;

#[derive(Clone)]
pub struct VisState;

impl VisualizationState<ModelState> for VisState {
    fn on_init(
        &self,
        commands: &mut Commands,
        sprite_factory: &mut AssetHandleFactoryResource,
        state: &mut ModelState,
        _schedule: &mut Schedule,
        sim: &mut SimulationDescriptor,
    ) {
        state
            .to_food_grid_black
            .render(sprite_factory, commands, sim);
        state.to_food_grid_red.render(sprite_factory, commands, sim);
        state
            .to_home_grid_black
            .render(sprite_factory, commands, sim);
        state.to_home_grid_red.render(sprite_factory, commands, sim);
        state.obstacles_grid.render(sprite_factory, commands, sim);
        state.ants_grid.render(sprite_factory, commands, sim);
    }

    fn get_agent_render(
        &self,
        agent: &Box<dyn Agent>,
        _state: &ModelState,
    ) -> Option<Box<dyn AgentRender>> {
        None
    }

    fn get_agent(
        &self,
        agent_render: &Box<dyn AgentRender>,
        state: &Box<&dyn StateTrait>,
    ) -> Option<Box<dyn Agent>> {
        None
    }

    fn before_render(
        &mut self,
        _state: &mut ModelState,
        _schedule: &Schedule,
        _commands: &mut Commands,
        _sprite_factory: &mut AssetHandleFactoryResource,
    ) {
    }
}

impl VisState {}
