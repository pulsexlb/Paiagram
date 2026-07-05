use bevy_moder_api::*;
use paiagram_sdk::ui::*;

mod_def!("ui_example");

#[system(schedule = Startup)]
fn init() {
    spawn!(ModTab {
        tab_id: "my".into(),
        title: "Test Mod".into()
    });
    spawn!(Container {
        node_id: "root".into(),
        tab_id: "my".into(),
        parent_node_id: None,
        layout: ContainerLayout::Column
    });
    spawn!(Label {
        node_id: "lbl".into(),
        tab_id: "my".into(),
        parent_node_id: Some("root".into()),
        text: "Hello".into()
    });
    spawn!(Button {
        node_id: "btn".into(),
        tab_id: "my".into(),
        parent_node_id: Some("root".into()),
        text: "Click".into(),
        clicked: false
    });
}

#[system(schedule = Update)]
fn update() {
    for (button,) in query_mut!(Button).iter_mut() {
        if button.clicked {
            log_info!("Button was clicked!");
            button.clicked = false;
        }
    }
}
