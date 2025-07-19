import json

def file_clean(path: str):
    with open(path, 'w', encoding='utf-8') as f:
        json.dump([], f, ensure_ascii=False)