use bevy::render::{RenderApp, RenderStage};
use bevy_mod_debugdump::schedule_graph;
use que::app;

fn main() {
    let _app = &mut app();
    _app.update();
    let schedule_graph_dot = bevy_mod_debugdump::schedule_graph::schedule_graph_dot(_app);
    std::fs::write("schedule_graph.dot", schedule_graph_dot).unwrap();

    let render_schedule_graph = schedule_graph::schedule_graph_dot_sub_app_styled(
        _app,
        RenderApp,
        &[&RenderStage::Extract],
        &schedule_graph::ScheduleGraphStyle::default(),
    );
    std::fs::write("render_schedule_graph.dot", render_schedule_graph).unwrap();

    // let render_app = _app.get_sub_app(RenderApp).expect("no render app");
    // let render_graph = render_app.world.get_resource::<RenderGraph>().unwrap();
    // let render_graph_dot = bevy_mod_debugdump::render_graph::render_graph_dot(&*render_graph);
    // std::fs::write("render_graph.dot", render_graph_dot).unwrap();
}
