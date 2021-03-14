use crate::model::AppState;

pub struct Delegate;

impl druid::AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        _cmd: &druid::Command,
        _data: &mut AppState,
        _env: &druid::Env,
    ) -> druid::Handled {
        druid::Handled::No
    }
}
