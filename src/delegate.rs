use crate::model::AppState;

pub struct Delegate;

impl druid::AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        cmd: &druid::Command,
        data: &mut AppState,
        _env: &druid::Env,
    ) -> druid::Handled {
        match cmd {
            _ if cmd.is(druid::commands::NEW_FILE) => {
                if data.pixels.is_dirty() {
                    // Ask the user if they'd like to save.
                }

                data.pixels.zero();
                druid::Handled::Yes
            }

            _ => druid::Handled::No,
        }
    }
}
