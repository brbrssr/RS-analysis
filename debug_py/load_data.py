import asyncio
import aiohttp
import json
from datetime import datetime, timezone
import os


async def fetch_klines(session: aiohttp.ClientSession, symbol: str, interval: str, start_time: int) -> (int, list):
    """
    Requests up to 1,500 candles starting from start_time (milliseconds).
    Returns a tuple (start_time, list of closing prices).
    """
    url = (
        'https://api.binance.com/api/v3/klines'
        f'?symbol={symbol}&interval={interval}'
        f'&startTime={start_time}&limit=1500'
    )
    async with session.get(url) as resp:
        resp.raise_for_status()
        data = await resp.json()

        closes = [float(candle[4]) for candle in data]
        return start_time, closes


async def load_price_series(symbol: str, interval: str, start_ts: int, end_ts: int) -> list:
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

        results = await asyncio.gather(*tasks)
    
    results.sort(key=lambda x: x[0])

    all_closes = []
    for _, closes in results:
        all_closes.extend(closes)

    return all_closes


def get_price_series(symbol: str, interval: str, since: str, output_path: str = 'price_series.json') -> None:
    try:
        dt = datetime.fromisoformat(since.replace('Z', '+00:00'))
    except ValueError as e:
        raise RuntimeError(f"Incorrect data format: {e}")

    start_ts = int(dt.timestamp() * 1000)
    end_ts = int(datetime.now(timezone.utc).timestamp() * 1000)

    closes = asyncio.run(load_price_series(symbol, interval, start_ts, end_ts))

    if os.path.exists(output_path):
        os.remove(output_path)
    with open(output_path, 'w', encoding='utf-8') as f:
        json.dump(closes, f, ensure_ascii=False, indent=2)

    print(f"Uploaded {len(closes)} closing prices in'{output_path}'.")

