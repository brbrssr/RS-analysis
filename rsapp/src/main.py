import flet as ft
import date_picker


def main(page: ft.Page):
    result = ft.Text() # Text field. Just for debug
    
    def on_date_change(date_picker):
        date = date_picker.get_date()
        result.value = f"Date: {date}" if date else "Date not selected"
        page.update()
    
    datePicker = date_picker.DatePicker(on_change = on_date_change)

    page.add(datePicker.render(), result)


ft.app(main)
