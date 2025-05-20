from turtle import window_height, window_width
import flet as ft
import date_picker
import time_picker
import trade_pair_input
import interval_picker


def main(page: ft.Page):
    result_date = ft.Text() # Text fields. Just for debug
    result_time = ft.Text()
    result_trade_pair = ft.Text()
    result_interval = ft.Text()


    page.title = "RS-analysis"

    page.window.width = 640
    page.window.height = 640
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


ft.app(target=main)
