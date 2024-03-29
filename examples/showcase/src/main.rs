use leptos::*;
use leptos_material::*;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| {
        let filled_input = create_rw_signal(cx, String::from("Hello World!"));
        view! { cx,
            <style>
                ".flex {
                    display: flex;
                    gap: 8px 8px;
                    flex-wrap: wrap;
                }"
            </style>
            <div style="max-width: 800px; margin-inline: auto">
                <MaterialColors primary=[0x67, 0x50, 0xA4] />
                <MaterialFonts path="public/fonts" />
                <MaterialStyle/>
                <h1>"Components"</h1>
                <h2>"Cards"</h2>
                <div class="flex">
                    <Card>
                        <hgroup>
                            <h3>"Headline"</h3>
                            <p>"Subhead"</p>
                        </hgroup>
                        <p>"Elevated"</p>
                    </Card>
                    <Card kind=CardKind::Filled>
                        <img src="public/example.jpg"/>
                        <p>"Filled"</p>
                    </Card>
                    <Card kind=CardKind::Outlined>
                        <p>"Outlined"</p>
                        <Actions>
                            <Button kind=ButtonKind::Outlined><span class="icon">"home"</span> "Action" </Button>
                            <Button kind=ButtonKind::Filled> "Action" </Button>
                        </Actions>
                    </Card>
                </div>
                <h2>"Buttons"</h2>
                <h3>"Enabled"</h3>
                <div class="flex">
                    <Button kind=ButtonKind::Elevated>"Elevated"</Button>
                    <Button kind=ButtonKind::Filled>"Filled"</Button>
                    <Button kind=ButtonKind::Tonal>"Tonal"</Button>
                    <Button kind=ButtonKind::Outlined>"Outlined"</Button>
                    <Button kind=ButtonKind::Text>"Text"</Button>
                    <Button kind=ButtonKind::Elevated icon="accessible_forward">"Elevated"</Button>
                    <Button kind=ButtonKind::Filled icon="accessible_forward">"Filled"</Button>
                    <Button kind=ButtonKind::Tonal icon="accessible_forward">"Tonal"</Button>
                    <Button kind=ButtonKind::Outlined icon="accessible_forward">"Outlined"</Button>
                    <Button kind=ButtonKind::Text icon="accessible_forward">"Text"</Button>
                </div>
                <h3>"Disabled"</h3>
                <div class="flex">
                    <Button disabled=true kind=ButtonKind::Elevated>"Elevated"</Button>
                    <Button disabled=true kind=ButtonKind::Filled>"Filled"</Button>
                    <Button disabled=true kind=ButtonKind::Tonal>"Tonal"</Button>
                    <Button disabled=true kind=ButtonKind::Outlined>"Outlined"</Button>
                    <Button disabled=true kind=ButtonKind::Text>"Text"</Button>
                    <Button disabled=true kind=ButtonKind::Elevated icon="accessible_forward">"Elevated"</Button>
                    <Button disabled=true kind=ButtonKind::Filled icon="accessible_forward">"Filled"</Button>
                    <Button disabled=true kind=ButtonKind::Tonal icon="accessible_forward">"Tonal"</Button>
                    <Button disabled=true kind=ButtonKind::Outlined icon="accessible_forward">"Outlined"</Button>
                    <Button disabled=true kind=ButtonKind::Text icon="accessible_forward">"Text"</Button>
                </div>
                <h2>"Inputs"</h2>
                <div class="flex">
                    <TextField/>
                    <TextField filled=true value=filled_input/>

                    <TextField label="With Label"/>
                    <TextField label="With Label" filled=true value=filled_input/>
                </div>
            </div>
        }
    })
}
