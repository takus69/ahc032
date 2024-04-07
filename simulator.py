import numpy as np
from tqdm import tqdm
import pandas as pd
import time
import multiprocessing
import subprocess
import json

def run(i):
    output_str = subprocess.run(f'powershell cat in/{i:04}.txt | .\\target\\debug\\ahc032.exe > out/{i:04}.txt', shell=True, capture_output=True, text=True).stderr
    # print(output_str)
    result = json.loads(output_str.split('\n')[0])
    return result

def main(i):
    start = time.time()
    # print(i, 'start')
    r = run(i)
    t = round(time.time()-start, 4)
    score = r['score']
    data = [i, score, t]
    print('\r', 'end', i, end='')
    # print(i, 'end')
    return data


if __name__ == '__main__':
    start = time.time()
    trial = 150
    result = []
    '''
    for i in tqdm(range(trial)):
        r = run(i)
        t = round(time.time()-start, 4)
        score = r['score']
        data = [i, score, t]
        result.append(data)
    '''
    processes = multiprocessing.cpu_count()
    with multiprocessing.Pool(processes=processes) as pool:
        data = [pool.apply_async(main, (i,)) for i in range(trial)]
        result = [d.get() for d in data]
    print()
    df = pd.DataFrame(result, columns=['i', 'score', 'time'])
    score = np.mean(df['score'])
    print(f'score:', format(int(df['score'].sum()), ','), 'score mean:', format(int(score), ','))
    df.to_csv('result.csv', index=False)
    print(f'end elapsed time: {time.time()-start:.2f}s')
