use std::{collections::HashMap, error::Error};

use atspi::{State, proxy::accessible::ObjectRefExt};
pub async fn get_active_frame_name() -> Result<String, Box<dyn Error>> {

    let atspi_connection = atspi::AccessibilityConnection::new().await?;
    
    let conn = atspi_connection.connection();

    let apps = atspi_connection
        .root_accessible_on_registry()
        .await?
        .get_children()
        .await?;

    let mut found_active_frame: bool = false;
    let mut active_frame_name: String = String::new();

    for app in apps.iter() {
        let proxy = app.clone().into_accessible_proxy(conn).await?;

        for frame in proxy.get_children().await? {
            let frame = frame.clone().into_accessible_proxy(conn).await?;
            let state = frame.get_state().await?;
            if state.contains(State::Active) {
                active_frame_name = frame.name().await?;
                found_active_frame = true;
            }
        }
    }

    if !found_active_frame {
        return Err("active frame not found".into());
    }

    Ok(active_frame_name)
}

pub async fn get_running_apps() -> Result<String, Box<dyn Error>> {
    let atspi_connection = atspi::AccessibilityConnection::new().await?;
	let root = atspi_connection.root_accessible_on_registry().await?;
    let conn = atspi_connection.connection();

	// we have to use a hashmap to map the id to the natural
	// language name since the get_application method on the
	// accessible proxy for other items in the tree return an id
	// but not the natural language name of the associated app;
	// thus we need this to map the id to the natural language name
	let mut id_to_name = HashMap::new();

	// by getting the names of the children of the root
	// we can get the names of all applications currently running
	for child in root.get_children().await?.iter() {
		let proxy = child.clone().into_accessible_proxy(conn).await?;
		let natural_name = proxy.name().await?;
		let id = proxy
			.get_application()
			.await?
			.name
			.to_string();
		id_to_name.insert(id, natural_name);
	}


    Ok(id_to_name.values().cloned().collect())
}