#!/usr/bin/env python3
# -*- coding:utf-8 -*-


tmp = 0
result = [8]*8

def cal8queen(row : int):
    if row == 8:
        printResult()
        global tmp 
        tmp += 1
        return

    for i in range(0, 8):
        if isOk(row, i):
            result[row] = i
            cal8queen(row+1)

def isOk(row : int, column : int):
    left = column - 1
    right = column + 1
    for up in range(row, -1, -1):
        if result[up] == column:
            return False
        if left >= 0 and result[up] == left:
            return False
        if right < 8 and result[up] == right: return False
        left -= 1
        right += 1

    return True

def printResult():
    for i in result:
        for j in range(0, 8):
            if j == result[i]:
                print(1, end=' ')
            else:
                print(0, end=' ')
        print()
    print()


if __name__ == '__main__':
    cal8queen(0)
    print(tmp)



