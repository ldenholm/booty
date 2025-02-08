# Problem statement:

# building an app - used by many custs. e.g. jira
# avoid customers overloading. rate limiter.
# cust can make x per y.

# customer = [rq1, rq2, rq]
#{customerId: {[rq1, rq2, rq3]} O(1) access. insert at O(1). sort O(n) del O(1)
from collections import defaultdict
import time
import random
print("hello")

class RateLimiter:
    def __init__(self, maxRequests, timeWindow):
        self.maxRequests = maxRequests
        self.timeWindow = timeWindow
        self.customerRequests = defaultdict(list)
        self.customerCredit = defaultdict(list)

    def isRequestAllowed(self, customerId: int) -> bool:
        # we need to get current time.
        # we need some logic to maintain our current request log
        # use pythons list comprehension to do some fancy shenanigans for tracking the request log
        # if number of requests per time is greater than allowed, return false.
        # else we'll return true, append this request to the customers request log.
        current_time = time.time()

        credit = self.customerCredit[customerId]
        
        if credit > 0:
            self.maxRequests += credit

        
        #timestamps = self.customerRequests[customerId]
        timestamps = [timestamp for timestamp in self.customerRequests[customerId] 
                      if current_time - timestamp < self.timeWindow]
        
        self.customerRequests[customerId] = timestamps

        # if self.customerCredit[customerId] > 0:
        #     if len(timestamps) - self.customerCredit[customerId] >= self.maxRequests:
        #         return True

        if len(timestamps) >= self.maxRequests:
            return False
        
        # accrue credit
        credit += len(timestamps) - self.maxRequests
        
        self.customerRequests[customerId].append(current_time)
        return True
        
    
rl = RateLimiter(2, 4)

# how can we test this.
# generate some randomness
for i in range(100):
    # generate random sleeping to exceed the rate limit
    # hardcode a long sleep time to double check the logic works.
    # 0.5 < x < 2
    time.sleep(random.uniform(0.5, 2))
    # print('first', rl.isRequestAllowed(1))
    # print('second', rl.isRequestAllowed(1))
    # print('third', rl.isRequestAllowed(1))
    # print('fourth', rl.isRequestAllowed(1))
    # print('fifth', rl.isRequestAllowed(1))

    print(rl.isRequestAllowed(1))
    print(rl.isRequestAllowed(1))
    print(rl.isRequestAllowed(1))
    print(rl.isRequestAllowed(1))
    print(rl.isRequestAllowed(1))

    # cd test suite
    # hardcode a customer request dictionary that contains more requests than are allowed
    # make sure that fails

    # hardcode a customer request dictionary that contains fewer requests than are allowed
    # make sure it passes

    # support customers by implementing a credit system. extra requests
    # data structure -> 
    
    # if a customer has not made x requests in y time, we grant them ???
    # they can save credit. 2 req per 4 window, if in 4s window

    # each customer has their own rate limiter.
    # the primary map is of a customer, it measures their requests, and
    # it constructs a rate limiter for processing their requests.








    # class CustomerRequestEngine:
    #     def __init__(self):
    #         self.credit # track how many requests they have saved
    #         self.timeWindow # constant time window
    #         self.maxRequests # changes depending on credit, with a constant default value
    #         sel
        
    #     def processRequestWithCredit():
    #         # some logic here to increase their maxRequest per window
    #         pass
    #     def processRequest():
    #         # default processor