//! Shared `/valence` shell: authenticated layout, left nav, and route outlet.
//!
//! [`ValenceLayout`] wraps every page under [`crate::ValenceRoutes`]. Page components
//! live under [`crate::pages`]; server functions under [`crate::server`].

use lepton_shell::AppBarUserMenu;
use leptos::prelude::*;
use leptos_router::components::Outlet;
use orbital::components::{
    Navigation, NavigationBody, NavigationConfig, NavigationLink, NavigationMaterial,
};
use uf_integrations::{
    ShellAppBar, ShellAuthMenu, ShellLeftNav, UnifiedFieldAppBar, UnifiedFieldShellLayout,
};
use uf_product::routes::RequireAuthenticated;

use crate::paths;
use crate::AppMetadata;

#[component]
pub fn ValenceLayout() -> impl IntoView {
    let app_name = AppMetadata::name().to_string();
    let selected_value = RwSignal::new(None::<String>);
    let open_categories = RwSignal::new(Vec::<String>::new());

    view! {
        <div data-testid="valence-app-root">
        <UnifiedFieldShellLayout>
            <ShellAppBar slot>
                <UnifiedFieldAppBar
                    app_name=app_name
                    app_id=AppMetadata::id()
                    homepage_url="/".to_string()
                >
                    <ShellAuthMenu slot:auth_menu>
                        <AppBarUserMenu />
                    </ShellAuthMenu>
                </UnifiedFieldAppBar>
            </ShellAppBar>
            <ShellLeftNav slot>
                <Navigation config=NavigationConfig::new().with_selected_value(selected_value).with_open_categories(open_categories)>
                    <NavigationMaterial slot />
                    <NavigationBody slot>
                        <div id="valence-nav">
                            <NavigationLink path=paths::ROOT value=paths::ROOT icon=icondata::AiDashboardOutlined exact=true test_id="nav-dashboard">"Dashboard"</NavigationLink>
                            <NavigationLink path=paths::SCHEMA value=paths::SCHEMA icon=icondata::AiAppstoreOutlined test_id="nav-schemas">"Schemas"</NavigationLink>
                            <NavigationLink path=paths::TRAITS value=paths::TRAITS icon=icondata::AiExperimentOutlined test_id="nav-traits">"Traits"</NavigationLink>
                            <NavigationLink path=paths::ITERS value=paths::ITERS icon=icondata::AiDatabaseOutlined test_id="nav-iters">"Iters"</NavigationLink>
                            <NavigationLink path="/valence/deletions" value="/valence/deletions" icon=icondata::AiStopOutlined test_id="nav-deletions">"Deletions"</NavigationLink>
                        </div>
                    </NavigationBody>
                </Navigation>
            </ShellLeftNav>
            <RequireAuthenticated>
                <Outlet />
            </RequireAuthenticated>
        </UnifiedFieldShellLayout>
        </div>
    }
}
