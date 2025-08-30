use atspi::connection::P2P;
use atspi::connection::{AccessibilityConnection, set_session_accessibility};
use atspi::proxy::proxy_ext::ProxyExt;
use atspi::{AtspiError, ObjectRefOwned};
use std::fmt::Display;

#[derive(Clone, Debug, Default)]
struct AccessibleApp {
    name: String,
    tk: String,
    total_refs: usize,
    null_refs: usize,
}

#[derive(Clone, Debug, Default)]
struct AccessibleApps(Vec<AccessibleApp>);

impl AccessibleApps {
    fn new(apps: Vec<AccessibleApp>) -> Self {
        Self(apps)
    }
}

impl Display for AccessibleApps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut name_width = "Application".len();
        let mut tk_width = "Toolkit".len();

        for app in &self.0 {
            if app.name.len() > name_width {
                name_width = app.name.len();
            }
            if app.tk.len() > tk_width {
                tk_width = app.tk.len();
            }
        }

        writeln!(
            f,
            "{:<name_width$} {:<tk_width$} {:<12} {:<10}",
            "Application",
            "Toolkit",
            "Total",
            "Null refs",
            name_width = name_width,
            tk_width = tk_width
        )?;
        writeln!(
            f,
            "{:-<name_width$} {:-<tk_width$} {:-<12} {:-<12}",
            "",
            "",
            "",
            "",
            name_width = name_width,
            tk_width = tk_width
        )?;

        for app in &self.0 {
            writeln!(
                f,
                "{:<name_width$} {:<tk_width$} {:<12} {:<12}",
                app.name,
                app.tk,
                app.total_refs,
                app.null_refs,
                name_width = name_width,
                tk_width = tk_width
            )?;
        }

        Ok(())
    }
}

async fn parse_connected(
    root: ObjectRefOwned,
    conn: &AccessibilityConnection,
) -> Result<AccessibleApp, AtspiError> {
    // Name is the name of the root Accessible
    let accessible = conn.object_as_accessible(&root).await?;
    let name = accessible.name().await?;

    // toolkit is found on the Application interface
    let application_proxy = accessible.proxies().await?.application().await?;
    let tk = application_proxy.toolkit_name().await?;

    let mut total_refs: usize = 0;
    let mut null_refs: usize = 0;

    let mut work = vec![root];

    while let Some(object_ref) = work.pop() {
        total_refs += 1;
        if !object_ref.is_null() {
            let accessible_proxy = conn.object_as_accessible(&object_ref).await?;
            let children = accessible_proxy.get_children().await?;
            work.extend(children);
        } else {
            null_refs += 1;
        }
    }

    Ok(AccessibleApp {
        name,
        tk,
        total_refs,
        null_refs,
    })
}

#[tokio::main]
async fn main() -> Result<(), atspi::AtspiError> {
    let a11y = AccessibilityConnection::new().await?;
    set_session_accessibility(true).await?;

    let root_accessible = a11y.root_accessible_on_registry().await?;
    let children = root_accessible.get_children().await?;

    let mut apps = Vec::with_capacity(children.len());
    for child in children {
        match parse_connected(child, &a11y).await {
            Ok(app) => apps.push(app),
            Err(e) => eprintln!("Error parsing application: {e}"),
        }
    }

    let applications = AccessibleApps::new(apps);
    println!("{applications}");

    Ok(())
}
