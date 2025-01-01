use std::{
  cell::{RefCell},
  collections::VecDeque,
  rc::Rc,
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
  common::{Rect, TilingDirection},
  containers::{
    traits::{
      CommonGetters, PositionGetters, TilingDirectionGetters,
      TilingSizeGetters,
    },
    Container, ContainerDto,
  },
  impl_common_getters, impl_container_debug,
  impl_position_getters_as_resizable, impl_tiling_direction_getters,
  impl_tiling_size_getters,
  user_config::GapsConfig,
};

/// A container type for a tabbed layout. Each tab is a child container, but only the active tab is shown.
#[derive(Clone)]
pub struct TabbedContainer(Rc<RefCell<TabbedContainerInner>>);

struct TabbedContainerInner {
  id: Uuid,
  parent: Option<Container>,
  children: VecDeque<Container>,
  child_focus_order: VecDeque<Uuid>,
  tiling_size: f32,
  tiling_direction: TilingDirection,
  gaps_config: GapsConfig,
  active_index: usize,
}

/// DTO for tabbed container, for logging and IPC.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TabbedContainerDto {
  id: Uuid,
  parent_id: Option<Uuid>,
  children: Vec<ContainerDto>,
  child_focus_order: Vec<Uuid>,
  has_focus: bool,
  tiling_size: f32,
  width: i32,
  height: i32,
  x: i32,
  y: i32,
  tiling_direction: TilingDirection,
  active_index: usize,
}

impl TabbedContainer {
  pub fn new(
    tiling_direction: TilingDirection,
    gaps_config: GapsConfig,
  ) -> Self {
    let inner = TabbedContainerInner {
      id: Uuid::new_v4(),
      parent: None,
      children: VecDeque::new(),
      child_focus_order: VecDeque::new(),
      tiling_size: 1.0,
      tiling_direction,
      gaps_config,
      active_index: 0,
    };
    Self(Rc::new(RefCell::new(inner)))
  }

  pub fn to_dto(&self) -> anyhow::Result<ContainerDto> {
    let rect = self.to_rect()?;
    let children = self
      .children()
      .iter()
      .map(|child| child.to_dto())
      .try_collect()?;

    Ok(ContainerDto::Custom(format!(
      "{:?}",
      TabbedContainerDto {
        id: self.id(),
        parent_id: self.parent().map(|p| p.id()),
        children,
        child_focus_order: self.0.borrow().child_focus_order.clone().into(),
        has_focus: self.has_focus(None),
        tiling_size: self.tiling_size(),
        tiling_direction: self.tiling_direction(),
        width: rect.width(),
        height: rect.height(),
        x: rect.x(),
        y: rect.y(),
        active_index: self.0.borrow().active_index,
      }
    )))
  }

  /// Switch to the next tab, wrapping around if needed.
  pub fn next_tab(&self) {
    let mut inner = self.0.borrow_mut();
    if !inner.children.is_empty() {
      inner.active_index =
        (inner.active_index + 1) % inner.children.len();
    }
  }

  /// Switch to the previous tab, wrapping around if needed.
  pub fn prev_tab(&self) {
    let mut inner = self.0.borrow_mut();
    if !inner.children.is_empty() {
      inner.active_index = if inner.active_index == 0 {
        inner.children.len() - 1
      } else {
        inner.active_index - 1
      };
    }
  }
}

impl_container_debug!(TabbedContainer);
impl_common_getters!(TabbedContainer);
impl_tiling_size_getters!(TabbedContainer);
impl_tiling_direction_getters!(TabbedContainer);
impl_position_getters_as_resizable!(TabbedContainer);
