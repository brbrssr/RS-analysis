import asyncio
import aiohttp
import json
from datetime import datetime, timezone
import os


async def fetch_klines(session: aiohttp.ClientSession, symbol: str, interval: str, start_time: int) -> (int, list):
    """
    Запрашивает до 1500 свечей, начиная с start_time (миллисекунды).
    Возвращает кортеж (start_time, список цен закрытия).
    """
    url = (
        'https://api.binance.com/api/v3/klines'
        f'?symbol={symbol}&interval={interval}'
        f'&startTime={start_time}&limit=1500'
    )
    async with session.get(url) as resp:
        resp.raise_for_status()
        data = await resp.json()
        # price is at index 4, as string
        closes = [float(candle[4]) for candle in data]
        return start_time, closes


async def load_price_series(symbol: str, interval: str, start_ts: int, end_ts: int) -> list:
    """
    Параллельно загружает свечи от start_ts до end_ts с шагом в batch_size * миллисекунд интервала,
    где batch_size = 1500. Возвращает упорядоченный по времени список цен закрытия.
    """
    # вычисляем длину одного интервала в мс
    unit = interval[-1]
    num = int(interval[:-1])
    maps = {'s': 1000, 'm': 60_000, 'h': 3_600_000, 'd': 86_400_000, 'w': 604_800_000}
    if unit not in maps:
        raise ValueError(f"Unsupported interval unit: {unit}")
    step = num * maps[unit] * 1500

    tasks = []
    async with aiohttp.ClientSession() as session:
        t = start_ts
        while t < end_ts:
            tasks.append(fetch_klines(session, symbol, interval, t))
            t += step

        # Собираем результаты
        results = await asyncio.gather(*tasks)
    
    # Сортируем по стартовому времени, чтобы результаты шли хронологически
    results.sort(key=lambda x: x[0])

    # "Выравниваем" вложенные списки в один
    all_closes = []
    for _, closes in results:
        all_closes.extend(closes)

    return all_closes


def get_price_series(symbol: str, interval: str, since: str, output_path: str = 'price_series.json') -> None:
    """
    Основная обёртка. Принимает:
    - symbol: торговая пара, например 'BTCUSDT'
    - interval: Binance-интервал, например '1m', '4h', '1d' и т.п.
    - since: ISO-8601 дата-время начала выборки (RFC3339), например '2025-01-01T00:00:00Z'
    - output_path: куда сохранить JSON с ценами закрытия
    """
    # парсим дату
    try:
        dt = datetime.fromisoformat(since.replace('Z', '+00:00'))
    except ValueError as e:
        raise RuntimeError(f"Неверный формат даты: {e}")

    start_ts = int(dt.timestamp() * 1000)
    end_ts = int(datetime.now(timezone.utc).timestamp() * 1000)

    # запускаем asyncio
    closes = asyncio.run(load_price_series(symbol, interval, start_ts, end_ts))

    # сохраняем в файл
    # если файл существует — очищаем
    if os.path.exists(output_path):
        os.remove(output_path)
    with open(output_path, 'w', encoding='utf-8') as f:
        json.dump(closes, f, ensure_ascii=False, indent=2)

    print(f"Загружено {len(closes)} цен закрытия. Сохранено в '{output_path}'.")


if __name__ == '__main__':
    import argparse

    parser = argparse.ArgumentParser(description='Скачать серию цен закрытия с Binance')
    parser.add_argument('symbol', help='Торговая пара, напр. BTCUSDT')
    parser.add_argument('interval', help='Интервал, напр. 1m, 4h, 1d, …')
    parser.add_argument('since', help='Дата начала в формате ISO, напр. 2025-01-01T00:00:00Z')
    parser.add_argument('--output', '-o', default='price_series.json', help='Путь к JSON-файлу')
    args = parser.parse_args()

    get_price_series(args.symbol, args.interval, args.since, args.output)
