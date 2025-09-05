use std::error::Error;

use atspi::{State, connection::set_session_accessibility, proxy::accessible::ObjectRefExt};
pub async fn get_active_frame_name() -> Result<String, Box<dyn Error>> {
    let atspi = atspi::AccessibilityConnection::new().await?;
    let conn = atspi.connection();
    set_session_accessibility(true).await?;

    let apps = atspi
        .root_accessible_on_registry()
        .await?
        .get_children()
        .await?;

    let mut found_active_frame: bool = false;
    let mut active_frame_name: String = String::new();

    for app in apps.iter() {
        let proxy = app.clone().into_accessible_proxy(conn).await?;
        let state = proxy.get_state().await?;
        assert!(
            !state.contains(State::Active),
            "The top level application should never have active state; only its associated frames should have this state"
        );

        for frame in proxy.get_children().await? {
            let frame = frame.clone().into_accessible_proxy(conn).await?;
            let state = frame.get_state().await?;
            if state.contains(State::Active) {
                active_frame_name = frame.name().await?;
                found_active_frame = true;
            }
        }
    }

    assert!(
        found_active_frame,
        "There must be one active frame at any given time"
    );

    Ok(active_frame_name)
}
