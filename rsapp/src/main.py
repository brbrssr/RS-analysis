import flet as ft
import date_picker
import time_picker
import trade_pair_input
import interval_picker
import other_parameters
import packer
import lib_handler


def main(page: ft.Page):
    result_date = ft.Text()
    result_time = ft.Text()
    result_trade_pair = ft.Text()
    result_interval = ft.Text()
    result_other_parameters = ft.Text()

    page.title = "RS-analysis"

    page.window.width = 640
    page.window.height = 854
    page.window.resizable = False
    

    def on_date_change(date_picker):
        date = date_picker.get_date()
        result_date.value = f"Date: {date}" if date else "Date not selected"
        page.update()
    datePicker = date_picker.DatePicker(on_change=on_date_change)
    page.add(datePicker.render(), result_date)


    def on_time_change(time_picker):
        time = time_picker.get_time()
        result_time.value = f"Time: {time}" if time else "Time not selected"
        page.update()
    timePicker = time_picker.TimePicker(on_change=on_time_change)
    page.add(timePicker.render(), result_time)


    def on_trade_pair_change(trade_pair_input):
        trade_pair = trade_pair_input.get_trade_pair()
        result_trade_pair.value = f"Trade Pair: {trade_pair}" if trade_pair else "Trade Pair not selected"
        page.update()
    tradePair = trade_pair_input.TradePairInput(on_change=on_trade_pair_change)
    page.add(tradePair.render(), result_trade_pair)


    def on_interval_change(interval_picker):
        interval = interval_picker.get_interval()
        result_interval.value = f"Interval: {interval}" if interval else "Interval not selected"
        page.update()
    intervalPicker = interval_picker.IntervalPicker(on_change=on_interval_change)
    page.add(intervalPicker.render(), result_interval)


    # Callback для дополнительных параметров
    def on_other_parameters_change(other_params):
        values = other_params.get_other_parameters()
        result_other_parameters.value = f"Params: {values}"
        page.update()
    otherParams = other_parameters.OtherParameters(on_change=on_other_parameters_change)
    page.add(otherParams.render(), result_other_parameters)


    # Функция для обработки нажатия кнопки "Analyse"
    def on_analyse_click(e):
        date = datePicker.get_date()
        time = timePicker.get_time()
        trade_pair = tradePair.get_trade_pair()
        interval = intervalPicker.get_interval()
        other_params = otherParams.get_other_parameters()
        # проверка на заполненность полей
        if not all([date, time, trade_pair, interval]):
            page.dialog = ft.AlertDialog(title=ft.Text("Ошибка"), content=ft.Text("Заполните все обязательные поля"))
            page.update()
            return
        # Преобразуем словарь в список в порядке, ожидаемом pack_n_send
        other_parameters_list = [
            other_params.get("Number of iterations", ""),
            other_params.get("Frequency", ""),
            other_params.get("Alpha", ""),
            other_params.get("UB", ""),
            other_params.get("Is Hybrid", ""),
            other_params.get("Max Iterations", ""),
            other_params.get("Number of lags", ""),
            other_params.get("Max Iterations Grid", "")
        ]
        _packer.pack_n_send(date, time, trade_pair, interval, other_parameters_list)

    # Callback для передачи параметров в lib_handler
    def on_packer_button_trigger(parameters):
        lib_handler.run_rs_anal(parameters)

    # Создаем Packer и переопределяем on_click кнопки
    _packer = packer.Packer(on_change=on_packer_button_trigger)
    _packer.packer_button.on_click = on_analyse_click
    page.add(_packer.render())


ft.app(target=main)
# ft.app(target=main, view=ft.WEB_BROWSER)
