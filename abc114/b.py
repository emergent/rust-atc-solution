s = str(input()).strip()
nums = [abs(int(s[i:i+3]) - 753) for i in range(len(s)-2)]
print(min(nums))
