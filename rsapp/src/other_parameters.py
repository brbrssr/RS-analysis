import flet as ft


class OtherParameters:
    # OtherParameters class constructor
    def __init__(self, on_change=None):
        super().__init__()
        self.on_change_callback = on_change

        self.expanded = False


        self.fields = [
            ft.Row([
                ft.TextField(label="Number of iterations", width=280, value="1000"),
                ft.TextField(label="Frequency", width=280, value="7"),
            ]),
            ft.Row([
                ft.TextField(label="Alpha", width=186, value="0.95"),
                ft.TextField(label="UB", width=186, value="0.499"),
                ft.Checkbox(label="Is Hybrid", width=187, value=True),
            ]),
            ft.Row([
                ft.TextField(label="Max Iterations", width=186, value="100"),
                ft.TextField(label="Number of lags", width=186, value="20"),
                ft.TextField(label="Max Iterations Grid", width=187, value="50"),
            ])
        ]

        self.panel_content = ft.Column(self.fields, visible=False)

        self.toggle_button = ft.ElevatedButton("Other Parameters ▼", on_click=self.toggle_panel)

        self.container = ft.Column([
            self.toggle_button,
            self.panel_content
        ])

        self._attach_on_change_handlers()
        # self.content.on_change = self._on_any_change
    

    def _attach_on_change_handlers(self):
        for row in self.fields:
            for control in row.controls:
                if hasattr(control, "on_change"):
                    control.on_change = self._on_any_change
    
    
    def toggle_panel(self, e):
        self.expanded = not self.expanded
        self.panel_content.visible = self.expanded
        self.toggle_button.text = "Other Parameters ▲" if self.expanded else "Other Parameters ▼"
        self.toggle_button.update()
        self.panel_content.update()


    def _on_any_change(self, e):
        if self.on_change_callback:
            self.on_change_callback(self)


    def get_other_parameters(self):
        values = {}
        for row in self.fields:
            for control in row.controls:
                if isinstance(control, ft.TextField):
                    values[control.label] = control.value
                elif isinstance(control, ft.Checkbox):
                    values[control.label] = control.value
        return values


    def render(self):
        return self.container


if __name__ == "__main__":
    def main(page: ft.Page):
        other_params = OtherParameters()
        panel_list = other_params.render()
        page.add(panel_list)

    ft.app(target=main)
