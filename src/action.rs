use crate::{
  components::home::{AddService, Mode},
  systemd::{UnitId, UnitWithStatus},
};

#[derive(Debug, Clone)]
pub enum Action {
  Quit,
  Resume,
  Suspend,
  Render,
  DebouncedRender,
  SpinnerTick,
  Resize(u16, u16),
  ToggleShowLogger,
  RefreshServices,
  SetServices(Vec<UnitWithStatus>),
  EnterMode(Mode),
  EnterError(String),
  CancelTask,
  ToggleHelp,
  SetUnitFilePath { unit: UnitId, path: Result<String, String> },
  CopyUnitFilePath,
  SetLogs { unit: UnitId, logs: Vec<String> },
  AppendLogLine { unit: UnitId, line: String },
  StartService(UnitId),
  StopService(UnitId),
  RestartService(UnitId),
  ReloadService(UnitId),
  EnableService(UnitId),
  DisableService(UnitId),
  AddService(AddService),
  ScrollUp(u16),
  ScrollDown(u16),
  ScrollToTop,
  ScrollToBottom,
  EditUnitFile { unit: UnitId, path: String },
  Noop,
}
