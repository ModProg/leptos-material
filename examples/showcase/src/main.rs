use leptos::*;
use leptos_material::*;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| {
        view! { cx,
            <MaterialColors primary=[0x67, 0x50, 0xA4] />
            <MaterialStyle/>
            <h1>"Components"</h1>
            <h2>"Cards"</h2>
            <div style="display:flex">
                <Card elevated=true>
                    "Elevated"
                </Card>
                <Card filled=true>
                    "Filled"
                </Card>
                <Card outlined=true>
                    "Outlined"
                </Card>
            </div>
            <h2>"Buttons"</h2>
            <div style="display:flex">
                <Button kind=ButtonKind::Elevated>"Elevated"</Button>
                <Button kind=ButtonKind::Filled>"Filled"</Button>
                <Button kind=ButtonKind::Tonal>"Tonal"</Button>
                <Button kind=ButtonKind::Outlined>"Outlined"</Button>
                <Button kind=ButtonKind::Text>"Text"</Button>
            </div>
            <h3>"Disabled"</h3>
            <div style="display:flex">
                <Button disabled=true kind=ButtonKind::Elevated>"Elevated"</Button>
                <Button disabled=true kind=ButtonKind::Filled>"Filled"</Button>
                <Button disabled=true kind=ButtonKind::Tonal>"Tonal"</Button>
                <Button disabled=true kind=ButtonKind::Outlined>"Outlined"</Button>
                <Button disabled=true kind=ButtonKind::Text>"Text"</Button>
            </div>
        }
    })
}
