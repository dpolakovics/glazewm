use anyhow::Context;

use crate::{
  common::TilingDirection,
  containers::{
    traits::{CommonGetters, TilingDirectionGetters},
    Container, DirectionContainer, TabbedContainer,
  },
  user_config::UserConfig,
  wm_event::WmEvent,
  wm_state::WmState,
};

/// Toggle tabbed layout around the container (window/workspace).
/// If already tabbed, flatten it; otherwise, create a new tabbed container.
pub fn toggle_tabbed_layout(
  container: Container,
  state: &mut WmState,
  config: &UserConfig,
) -> anyhow::Result<()> {
  if let Container::Tabbed(tabbed) = container.clone() {
    flatten_tabbed_container(tabbed.clone())?;
    return Ok(());
  }

  let parent_direction = match container.direction_container() {
    Ok(dc) => dc.tiling_direction(),
    Err(_) => TilingDirection::Horizontal,
  };

  let tabbed = TabbedContainer::new(parent_direction, config.value.gaps.clone());
  wrap_in_tabbed_container(tabbed.clone(), container)?;

  state.emit_event(WmEvent::Custom(format!(
    "Toggled tabbed layout. Container = {}",
    tabbed.id()
  )));

  Ok(())
}

/// Wraps an existing container in a tabbed container by reparenting it.
pub fn wrap_in_tabbed_container(
  tabbed: TabbedContainer,
  container: Container,
) -> anyhow::Result<()> {
  let parent = container.parent();
  let old_id = container.id();

  if let Some(mut parent) = parent {
    parent.remove_child(&container);
  }

  tabbed.add_child(container.clone())?;
  container.set_parent(Some(tabbed.clone().into()));

  if let Some(mut parent) = parent {
    parent.add_child(tabbed.clone().into())?;
    tabbed.set_parent(Some(parent.into()));
  }

  log::debug!("Wrapped container {} in a tabbed container.", old_id);
  Ok(())
}

/// Remove the tabbed container and reparent its children.
pub fn flatten_tabbed_container(
  tabbed: TabbedContainer,
) -> anyhow::Result<()> {
  let parent = tabbed.parent();
  let children = tabbed.children();

  if let Some(mut p) = parent.clone() {
    p.remove_child(&tabbed.into());
  }

  for child in children {
    if let Some(mut p) = parent.clone() {
      p.add_child(child.clone())?;
      child.set_parent(Some(p.into()));
    }
  }

  Ok(())
}
