use std::{
  cell::{Ref, RefMut},
  collections::VecDeque,
};

use ambassador::Delegate;
use enum_as_inner::EnumAsInner;
use uuid::Uuid;

use super::{RootContainer, SplitContainer, TabbedContainer};
use crate::{
  common::{
    platform::NativeWindow, Direction, DisplayState, Rect, RectDelta,
    TilingDirection,
  },
  containers::{traits::*, ContainerDto},
  monitors::Monitor,
  user_config::{GapsConfig, UserConfig, WindowRuleConfig},
  windows::{
    traits::*, ActiveDrag, NonTilingWindow, TilingWindow, WindowState,
  },
  workspaces::Workspace,
};

#[derive(Clone, Debug, EnumAsInner, Delegate)]
#[delegate(CommonGetters)]
#[delegate(PositionGetters)]
pub enum Container {
    Root(RootContainer),
    Monitor(Monitor),
    Workspace(Workspace),
    Split(SplitContainer),
    TilingWindow(TilingWindow),
    NonTilingWindow(NonTilingWindow),
    Tabbed(TabbedContainer),
}

impl From<RootContainer> for Container {
    fn from(value: RootContainer) -> Self {
        Container::Root(value)
    }
}

impl From<Monitor> for Container {
    fn from(value: Monitor) -> Self {
        Container::Monitor(value)
    }
}

impl From<Workspace> for Container {
    fn from(value: Workspace) -> Self {
        Container::Workspace(value)
    }
}

impl From<SplitContainer> for Container {
    fn from(value: SplitContainer) -> Self {
        Container::Split(value)
    }
}

impl From<NonTilingWindow> for Container {
    fn from(value: NonTilingWindow) -> Self {
        Container::NonTilingWindow(value)
    }
}

impl From<TilingWindow> for Container {
    fn from(value: TilingWindow) -> Self {
        Container::TilingWindow(value)
    }
}

impl From<TabbedContainer> for Container {
    fn from(value: TabbedContainer) -> Self {
        Container::Tabbed(value)
    }
}

#[derive(Clone, Debug, EnumAsInner, Delegate)]
#[delegate(CommonGetters)]
#[delegate(PositionGetters)]
#[delegate(TilingSizeGetters)]
pub enum TilingContainer {
    Split(SplitContainer),
    TilingWindow(TilingWindow),
    Tabbed(TabbedContainer),
}

impl From<SplitContainer> for TilingContainer {
    fn from(value: SplitContainer) -> Self {
        TilingContainer::Split(value)
    }
}

impl From<TilingWindow> for TilingContainer {
    fn from(value: TilingWindow) -> Self {
        TilingContainer::TilingWindow(value)
    }
}

impl From<TabbedContainer> for TilingContainer {
    fn from(value: TabbedContainer) -> Self {
        TilingContainer::Tabbed(value)
    }
}

impl TryFrom<Container> for TilingContainer {
    type Error = &'static str;

    fn try_from(container: Container) -> Result<Self, Self::Error> {
        match container {
            Container::Split(c) => Ok(TilingContainer::Split(c)),
            Container::TilingWindow(c) => Ok(TilingContainer::TilingWindow(c)),
            Container::Tabbed(c) => Ok(TilingContainer::Tabbed(c)),
            _ => Err("Cannot convert type to `TilingContainer`."),
        }
    }
}

impl PartialEq for TilingContainer {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for TilingContainer {}

#[derive(Clone, Debug, EnumAsInner, Delegate)]
#[delegate(CommonGetters)]
#[delegate(PositionGetters)]
#[delegate(WindowGetters)]
pub enum WindowContainer {
    TilingWindow(TilingWindow),
    NonTilingWindow(NonTilingWindow),
}

impl From<TilingWindow> for WindowContainer {
    fn from(value: TilingWindow) -> Self {
        WindowContainer::TilingWindow(value)
    }
}

impl From<NonTilingWindow> for WindowContainer {
    fn from(value: NonTilingWindow) -> Self {
        WindowContainer::NonTilingWindow(value)
    }
}

impl PartialEq for WindowContainer {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for WindowContainer {}

#[derive(Clone, Debug, EnumAsInner, Delegate)]
#[delegate(CommonGetters)]
#[delegate(PositionGetters)]
#[delegate(TilingDirectionGetters)]
pub enum DirectionContainer {
    Workspace(Workspace),
    Split(SplitContainer),
}

impl From<Workspace> for DirectionContainer {
    fn from(value: Workspace) -> Self {
        DirectionContainer::Workspace(value)
    }
}

impl From<SplitContainer> for DirectionContainer {
    fn from(value: SplitContainer) -> Self {
        DirectionContainer::Split(value)
    }
}

#[macro_export]
macro_rules! impl_container_debug {
    ($type:ty) => {
        impl std::fmt::Debug for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Debug::fmt(
                    &self.to_dto().map_err(|_| std::fmt::Error),
                    f,
                )
            }
        }
    };
}
