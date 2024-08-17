# 引用套件
from datetime import datetime

# 產生斐波那契數列的個數
NUMBER = 100

# 產生斐波那契數列
def fibonacci(number, acc, current):
    if number == 0:
        return acc
    else:
        return fibonacci(number - 1, acc + current, acc)

if __name__ == "__main__":
    # 計算執行時間
    old_time = datetime.now()
    print(f"fibonacci number: {fibonacci(NUMBER, 0, 1)}")
    duration = datetime.now() - old_time

    A_SEC_IN_MICROSECONDS = 1_000_000
    print(f"{duration.total_seconds() * A_SEC_IN_MICROSECONDS} 毫秒(µs)")