import sys

nums = [*map(int, sys.stdin)]

amount_greater = lambda nums: sum(n > nums[i - 1] for i, n in enumerate(nums) if i)

# Part 1 only
if 0:
    print(amount_greater(nums))

# Part 2 only
if 1:
    windows = [*map(sum, zip(nums, nums[1:], nums[2:]))]
    print(amount_greater(windows))
